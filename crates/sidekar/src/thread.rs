use std::sync::Arc;

use anyhow::Result;
use assistant_tool::ToolWorkingSet;
use chrono::{DateTime, Utc};
use collections::{BTreeMap, HashMap};
use futures::future::Shared;
use futures::{FutureExt as _, StreamExt as _};
use gpui::{App, Context, Entity, EventEmitter, SharedString, Task};
use language_model::{LanguageModelToolResult, LanguageModelToolUseId, Role, StopReason};
use language_models::provider::cloud::{MaxMonthlySpendReachedError, PaymentRequiredError};
use serde::{Deserialize, Serialize};
use util::post_inc;
use uuid::Uuid;

use crate::context::{ContextId, ContextKind, ContextSnapshot};
use crate::sidecar::{AgentSessionChatRequestMinimal, Sidecar};
use crate::thread_store::SavedThread;
use crate::types::{
    self, Position, SymbolEventSubStep, ThinkingForEditRequest, UserContext, VariableInformation,
    VariableType,
};

#[derive(Debug, Clone, Copy)]
pub enum RequestKind {
    Chat,
    Reasoning,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct ThreadId(pub(crate) Arc<str>);

impl ThreadId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().into())
    }
}

impl std::fmt::Display for ThreadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct MessageId(pub(crate) Arc<str>);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string().into())
    }
}

/// A message in a [`Thread`].
#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub role: Role,
    pub text: String,
}

/// A thread of conversation with the LLM.
pub struct Thread {
    id: ThreadId,
    updated_at: DateTime<Utc>,
    summary: Option<SharedString>,
    messages: Vec<Message>,
    context: BTreeMap<ContextId, ContextSnapshot>,
    context_by_message: HashMap<MessageId, Vec<ContextId>>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    tools: Arc<ToolWorkingSet>,
    tool_results_by_message: HashMap<MessageId, Vec<LanguageModelToolResult>>,
    pending_tool_uses_by_id: HashMap<LanguageModelToolUseId, PendingToolUse>,
}

impl Thread {
    pub fn new(tools: Arc<ToolWorkingSet>, _cx: &mut Context<Self>) -> Self {
        Self {
            id: ThreadId::new(),
            updated_at: Utc::now(),
            summary: None,
            messages: Vec::new(),
            context: BTreeMap::default(),
            context_by_message: HashMap::default(),
            completion_count: 0,
            pending_completions: Vec::new(),
            tools,
            tool_results_by_message: HashMap::default(),
            pending_tool_uses_by_id: HashMap::default(),
        }
    }

    pub fn from_saved(
        id: ThreadId,
        saved: SavedThread,
        tools: Arc<ToolWorkingSet>,
        _cx: &mut Context<Self>,
    ) -> Self {
        Self {
            id,
            updated_at: saved.updated_at,
            summary: Some(saved.summary),
            messages: saved
                .messages
                .into_iter()
                .map(|message| Message {
                    id: message.id,
                    role: message.role,
                    text: message.text,
                })
                .collect(),
            context: BTreeMap::default(),
            context_by_message: HashMap::default(),
            completion_count: 0,
            pending_completions: Vec::new(),
            tools,
            tool_results_by_message: HashMap::default(),
            pending_tool_uses_by_id: HashMap::default(),
        }
    }

    pub fn id(&self) -> &ThreadId {
        &self.id
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn touch_updated_at(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn summary(&self) -> Option<SharedString> {
        self.summary.clone()
    }

    pub fn summary_or_default(&self) -> SharedString {
        const DEFAULT: SharedString = SharedString::new_static("New Thread");
        self.summary.clone().unwrap_or(DEFAULT)
    }

    pub fn set_summary(&mut self, summary: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.summary = Some(summary.into());
        cx.emit(ThreadEvent::SummaryChanged);
    }

    pub fn message(&self, id: &MessageId) -> Option<&Message> {
        self.messages.iter().find(|message| &message.id == id)
    }

    pub fn messages(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }

    pub fn is_streaming(&self) -> bool {
        !self.pending_completions.is_empty()
    }

    pub fn tools(&self) -> &Arc<ToolWorkingSet> {
        &self.tools
    }

    pub fn context_for_message(&self, id: &MessageId) -> Option<Vec<ContextSnapshot>> {
        let context = self.context_by_message.get(id)?;
        Some(
            context
                .into_iter()
                .filter_map(|context_id| self.context.get(&context_id))
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    pub fn pending_tool_uses(&self) -> Vec<&PendingToolUse> {
        self.pending_tool_uses_by_id.values().collect()
    }

    pub fn insert_user_message(
        &mut self,
        text: impl Into<String>,
        context: Vec<ContextSnapshot>,
        cx: &mut Context<Self>,
    ) {
        let message_id = self.insert_message(Role::User, text, cx);
        let context_ids = context.iter().map(|context| context.id).collect::<Vec<_>>();
        self.context
            .extend(context.into_iter().map(|context| (context.id, context)));
        self.context_by_message.insert(message_id, context_ids);
    }

    pub(crate) fn next_message_id(&mut self) -> MessageId {
        MessageId::new()
    }

    pub fn insert_message(
        &mut self,
        role: Role,
        text: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> MessageId {
        let id = MessageId::new();
        self.messages.push(Message {
            id: id.clone(),
            role,
            text: text.into(),
        });
        self.touch_updated_at();
        cx.emit(ThreadEvent::MessageAdded(id.clone()));
        id
    }

    /// Returns the representation of this [`Thread`] in a textual form.
    ///
    /// This is the representation we use when attaching a thread as context to another thread.
    pub fn text(&self) -> String {
        let mut text = String::new();

        for message in &self.messages {
            text.push_str(match message.role {
                language_model::Role::User => "User:",
                language_model::Role::Assistant => "Assistant:",
                language_model::Role::System => "System:",
            });
            text.push('\n');

            text.push_str(&message.text);
            text.push('\n');
        }

        text
    }

    pub fn to_completion_request(
        &self,
        request_kind: RequestKind,
        root: String,
        _cx: &App,
    ) -> AgentSessionChatRequestMinimal {
        let last_message = self.messages.last().unwrap();
        assert_eq!(last_message.role, Role::User);
        let context = self
            .context_for_message(&last_message.id)
            .unwrap_or_default();
        let mut user_context = UserContext::default();
        for ctx in context {
            match &ctx.kind {
                ContextKind::File => {}
                ContextKind::Directory => continue,
                ContextKind::FetchedUrl => continue,
                ContextKind::Thread => continue,
            }
            let var = VariableInformation {
                start_position: Position {
                    line: 0,
                    character: 0,
                    byte_offset: 0,
                },
                end_position: Position {
                    line: 1000,
                    character: 0,
                    byte_offset: 10000,
                },
                fs_file_path: ctx.name.to_string(),
                name: ctx.name.to_string(),
                variable_type: VariableType::File,
                content: ctx.text.join("\n").to_string(),
                language: "rust".to_string(), // other languages don't exist
            };
            user_context.variables.push(var);
        }

        AgentSessionChatRequestMinimal {
            session_id: self.id.to_string(),
            exchange_id: last_message.id.0.to_string(),
            query: last_message.text.clone(),
            user_context,
            reasoning: matches!(request_kind, RequestKind::Reasoning),
            root,
        }
    }

    pub fn stream_completion(
        &mut self,
        request: AgentSessionChatRequestMinimal,
        sidecar: Entity<Sidecar>,
        cx: &mut Context<Self>,
    ) {
        let pending_completion_id = post_inc(&mut self.completion_count);
        let stream = sidecar.read(cx).anchored_edit(request);

        let task = cx.spawn(|thread, mut cx| async move {
            let stream_completion = async {
                let mut stream = stream.await?;
                let stop_reason = StopReason::EndTurn;

                while let Some(event) = stream.next().await {
                    let event = event?;

                    thread.update(&mut cx, |thread, cx| {
                        let message_id = MessageId(event.exchange_id.into());
                        let chunk = match event.event {
                            types::UIEvent::SymbolEventSubStep(s) => match s.event {
                                SymbolEventSubStep::Edit(
                                    types::SymbolEventEditRequest::ThinkingForEdit(
                                        ThinkingForEditRequest { delta, .. },
                                    ),
                                ) => delta.unwrap_or_default(),
                                _ => return,
                            },
                            types::UIEvent::ChatEvent(chat_message_event) => {
                                chat_message_event.delta.clone().unwrap_or_default()
                            }
                            _ => return,
                        };
                        if let Some(message) =
                            thread.messages.iter_mut().find(|x| x.id == message_id)
                        {
                            message.text += &chunk;
                            cx.emit(ThreadEvent::StreamedAssistantText(message_id, chunk));
                        } else {
                            thread.messages.push(Message {
                                id: message_id.clone(),
                                role: Role::Assistant,
                                text: chunk,
                            });
                            thread.touch_updated_at();
                            cx.emit(ThreadEvent::MessageAdded(message_id));
                        }

                        thread.touch_updated_at();
                        // doubt: why are we emitting completion
                        cx.emit(ThreadEvent::StreamedCompletion);
                        cx.notify();
                    })?;

                    smol::future::yield_now().await;
                }

                thread.update(&mut cx, |thread, _cx| {
                    thread
                        .pending_completions
                        .retain(|completion| completion.id != pending_completion_id);

                    if thread.summary.is_none() && thread.messages.len() >= 2 {
                        // maan2: disabled for now
                        // thread.summarize(cx);
                    }
                })?;

                anyhow::Ok(stop_reason)
            };

            let result = stream_completion.await;

            thread
                .update(&mut cx, |thread, cx| match result.as_ref() {
                    Ok(stop_reason) => match stop_reason {
                        StopReason::ToolUse => {
                            cx.emit(ThreadEvent::UsePendingTools);
                        }
                        StopReason::EndTurn => {}
                        StopReason::MaxTokens => {}
                    },
                    Err(error) => {
                        if error.is::<PaymentRequiredError>() {
                            cx.emit(ThreadEvent::ShowError(ThreadError::PaymentRequired));
                        } else if error.is::<MaxMonthlySpendReachedError>() {
                            cx.emit(ThreadEvent::ShowError(ThreadError::MaxMonthlySpendReached));
                        } else {
                            let error_message = error
                                .chain()
                                .map(|err| err.to_string())
                                .collect::<Vec<_>>()
                                .join("\n");
                            cx.emit(ThreadEvent::ShowError(ThreadError::Message(
                                SharedString::from(error_message.clone()),
                            )));
                        }

                        thread.cancel_last_completion();
                    }
                })
                .ok();
        });

        self.pending_completions.push(PendingCompletion {
            id: pending_completion_id,
            _task: task,
        });
    }

    pub fn insert_tool_output(
        &mut self,
        assistant_message_id: MessageId,
        tool_use_id: LanguageModelToolUseId,
        output: Task<Result<String>>,
        cx: &mut Context<Self>,
    ) {
        let insert_output_task = cx.spawn(|thread, mut cx| {
            let tool_use_id = tool_use_id.clone();
            async move {
                let output = output.await;
                thread
                    .update(&mut cx, |thread, cx| {
                        // The tool use was requested by an Assistant message,
                        // so we want to attach the tool results to the next
                        // user message.
                        // likely doesn't work
                        let next_user_message = MessageId::new();

                        let tool_results = thread
                            .tool_results_by_message
                            .entry(next_user_message)
                            .or_default();

                        match output {
                            Ok(output) => {
                                tool_results.push(LanguageModelToolResult {
                                    tool_use_id: tool_use_id.to_string(),
                                    content: output,
                                    is_error: false,
                                });

                                cx.emit(ThreadEvent::ToolFinished { tool_use_id });
                            }
                            Err(err) => {
                                tool_results.push(LanguageModelToolResult {
                                    tool_use_id: tool_use_id.to_string(),
                                    content: err.to_string(),
                                    is_error: true,
                                });

                                if let Some(tool_use) =
                                    thread.pending_tool_uses_by_id.get_mut(&tool_use_id)
                                {
                                    tool_use.status = PendingToolUseStatus::Error(err.to_string());
                                }
                            }
                        }
                    })
                    .ok();
            }
        });

        if let Some(tool_use) = self.pending_tool_uses_by_id.get_mut(&tool_use_id) {
            tool_use.status = PendingToolUseStatus::Running {
                _task: insert_output_task.shared(),
            };
        }
    }

    /// Cancels the last pending completion, if there are any pending.
    ///
    /// Returns whether a completion was canceled.
    pub fn cancel_last_completion(&mut self) -> bool {
        if let Some(_last_completion) = self.pending_completions.pop() {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub enum ThreadError {
    PaymentRequired,
    MaxMonthlySpendReached,
    Message(SharedString),
}

#[derive(Debug, Clone)]
pub enum ThreadEvent {
    ShowError(ThreadError),
    StreamedCompletion,
    StreamedAssistantText(MessageId, String),
    MessageAdded(MessageId),
    SummaryChanged,
    UsePendingTools,
    ToolFinished {
        #[allow(unused)]
        tool_use_id: LanguageModelToolUseId,
    },
}

impl EventEmitter<ThreadEvent> for Thread {}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}

#[derive(Debug, Clone)]
pub struct PendingToolUse {
    pub id: LanguageModelToolUseId,
    /// The ID of the Assistant message in which the tool use was requested.
    pub assistant_message_id: MessageId,
    pub name: String,
    pub input: serde_json::Value,
    pub status: PendingToolUseStatus,
}

#[derive(Debug, Clone)]
pub enum PendingToolUseStatus {
    Idle,
    Running { _task: Shared<Task<()>> },
    Error(#[allow(unused)] String),
}

impl PendingToolUseStatus {
    pub fn is_idle(&self) -> bool {
        matches!(self, PendingToolUseStatus::Idle)
    }
}
