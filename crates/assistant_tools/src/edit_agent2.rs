mod edit_parser;
#[cfg(test)]
mod evals;

use crate::streaming_fuzzy_matcher::StreamingFuzzyMatcher;
use crate::{Template, Templates};
use anyhow::Result;
use assistant_tool::ActionLog;
use collections::HashMap;
pub use edit_parser::EditFormat;
use edit_parser::{EditParser, EditParserEvent, EditParserMetrics};
use futures::{
    Stream, StreamExt,
    channel::mpsc::{self, UnboundedReceiver},
    pin_mut,
    stream::BoxStream,
};
use gpui::{AppContext, AsyncApp, Entity, Task};
use language::{Anchor, Buffer, BufferSnapshot, LineIndent, Point, TextBufferSnapshot};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelRequest, LanguageModelRequestMessage,
    LanguageModelToolChoice, MessageContent, Role,
};
use project::ProjectPath;
use project::{AgentLocation, Project};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, iter, mem, ops::Range, path::PathBuf, pin::Pin, sync::Arc, task::Poll};
use streaming_diff::{CharOperation, StreamingDiff};
use util::debug_panic;
use watch;
use zed_llm_client::CompletionIntent;

#[derive(Serialize)]
struct EditMultiFileXmlPromptTemplate {
    edit_description: String,
}

impl Template for EditMultiFileXmlPromptTemplate {
    const TEMPLATE_NAME: &'static str = "edit_file_prompt_xml_multi.hbs";
}

#[derive(Serialize)]
struct EditMultiFileDiffFencedPromptTemplate {
    edit_description: String,
}

impl Template for EditMultiFileDiffFencedPromptTemplate {
    const TEMPLATE_NAME: &'static str = "edit_file_prompt_diff_fenced_multi.hbs";
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditAgentOutputEvent {
    ResolvingEditRange {
        path: PathBuf,
        range: Range<Anchor>,
    },
    UnresolvedEditRange {
        path: PathBuf,
    },
    AmbiguousEditRange {
        path: PathBuf,
        ranges: Vec<Range<usize>>,
    },
    Edited {
        path: PathBuf,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditAgentOutput {
    pub raw_edits: String,
    pub parser_metrics: EditParserMetrics,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiFileEditOutput {
    pub file_edits: Vec<FileEdit>,
    pub total_parser_metrics: EditParserMetrics,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FileEdit {
    pub path: PathBuf,
    pub output: EditAgentOutput,
}

#[derive(Clone)]
pub struct EditAgent2 {
    model: Arc<dyn LanguageModel>,
    action_log: Entity<ActionLog>,
    project: Entity<Project>,
    templates: Arc<Templates>,
    edit_format: EditFormat,
}

impl EditAgent2 {
    pub fn new(
        model: Arc<dyn LanguageModel>,
        project: Entity<Project>,
        action_log: Entity<ActionLog>,
        templates: Arc<Templates>,
        edit_format: EditFormat,
    ) -> Self {
        EditAgent2 {
            model,
            project,
            action_log,
            templates,
            edit_format,
        }
    }

    pub fn edit(
        &self,
        edit_description: String,
        conversation: &LanguageModelRequest,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<MultiFileEditOutput>>,
        mpsc::UnboundedReceiver<EditAgentOutputEvent>,
    ) {
        let this = self.clone();
        let (events_tx, events_rx) = mpsc::unbounded();
        let conversation = conversation.clone();
        let edit_format = self.edit_format;
        let project = self.project.clone();

        let output = cx.spawn(async move |mut cx| {
            // Create multi-file prompt
            let prompt = this.create_multi_file_prompt(edit_description, edit_format)?;

            let edit_chunks = this
                .request(conversation, CompletionIntent::EditFile, prompt, &mut cx)
                .await?;

            // Parse all chunks to group by file
            let (parser_task, mut events) =
                this.parse_edit_chunks_multi_file(edit_chunks, edit_format, &cx);

            let mut file_events: HashMap<PathBuf, Vec<EditParserEvent>> = HashMap::new();
            let mut current_file: Option<PathBuf> = None;

            // Collect events and group by file
            while let Some(event) = events.next().await {
                match event {
                    Ok(event) => {
                        match &event {
                            EditParserEvent::OldTextChunk { path, .. } => {
                                if let Some(path) = path {
                                    current_file = Some(path.clone());
                                    file_events.entry(path.clone()).or_insert_with(Vec::new);
                                }
                            }
                            _ => {}
                        }

                        // Add event to current file's events
                        if let Some(ref file) = current_file {
                            file_events.get_mut(file).unwrap().push(event);
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            let parser_metrics = parser_task.await?;
            let mut file_edits = Vec::new();
            let mut total_metrics = parser_metrics;

            // Apply edits to each file
            for (path, events) in file_events {
                // Convert PathBuf to ProjectPath
                let project_path = match this.path_to_project_path(&path, &project, &cx).await {
                    Ok(path) => path,
                    Err(_) => continue, // Skip files not in project
                };

                // Open buffer for this file
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                    .await?;

                // Apply edits to this file
                let output = this
                    .apply_edit_chunks_from_events(
                        buffer,
                        path.clone(),
                        futures::stream::iter(events.into_iter().map(Ok)),
                        events_tx.clone(),
                        &mut cx,
                    )
                    .await?;

                total_metrics = total_metrics + output.parser_metrics.clone();
                file_edits.push(FileEdit { path, output });
            }

            Ok(MultiFileEditOutput {
                file_edits,
                total_parser_metrics: total_metrics,
            })
        });

        (output, events_rx)
    }

    fn create_multi_file_prompt(
        &self,
        edit_description: String,
        edit_format: EditFormat,
    ) -> Result<String> {
        match edit_format {
            EditFormat::XmlTags => {
                EditMultiFileXmlPromptTemplate { edit_description }.render(&self.templates)
            }
            EditFormat::DiffFenced => {
                EditMultiFileDiffFencedPromptTemplate { edit_description }.render(&self.templates)
            }
        }
    }

    fn resolve_old_text<T>(
        snapshot: TextBufferSnapshot,
        mut edit_events: T,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<(T, Vec<ResolvedOldText>)>>,
        watch::Receiver<Option<Range<usize>>>,
    )
    where
        T: 'static + Send + Unpin + Stream<Item = Result<EditParserEvent>>,
    {
        let (mut old_range_tx, old_range_rx) = watch::channel(None);
        let task = cx.background_spawn(async move {
            let mut matcher = StreamingFuzzyMatcher::new(snapshot);
            while let Some(edit_event) = edit_events.next().await {
                let EditParserEvent::OldTextChunk {
                    chunk,
                    done,
                    line_hint,
                    path: _,
                } = edit_event?
                else {
                    break;
                };

                old_range_tx.send(matcher.push(&chunk, line_hint)).ok();
                if done {
                    break;
                }
            }

            let matches = matcher.finish();
            let best_match = matcher.select_best_match();

            old_range_tx.send(best_match.clone()).ok();

            let indent = LineIndent::from_iter(
                matcher
                    .query_lines()
                    .first()
                    .unwrap_or(&String::new())
                    .chars(),
            );

            let resolved_old_texts = if let Some(best_match) = best_match {
                vec![ResolvedOldText {
                    range: best_match,
                    indent,
                }]
            } else {
                matches
                    .into_iter()
                    .map(|range| ResolvedOldText { range, indent })
                    .collect::<Vec<_>>()
            };

            Ok((edit_events, resolved_old_texts))
        });

        (task, old_range_rx)
    }

    fn compute_edits<T>(
        snapshot: BufferSnapshot,
        resolved_old_text: ResolvedOldText,
        mut edit_events: T,
        cx: &mut AsyncApp,
    ) -> (
        Task<Result<T>>,
        UnboundedReceiver<(Range<Anchor>, Arc<str>)>,
    )
    where
        T: 'static + Send + Unpin + Stream<Item = Result<EditParserEvent>>,
    {
        let (edits_tx, edits_rx) = mpsc::unbounded();
        let compute_edits = cx.background_spawn(async move {
            let buffer_start_indent = snapshot
                .line_indent_for_row(snapshot.offset_to_point(resolved_old_text.range.start).row);
            let indent_delta = if buffer_start_indent.tabs > 0 {
                IndentDelta::Tabs(
                    buffer_start_indent.tabs as isize - resolved_old_text.indent.tabs as isize,
                )
            } else {
                IndentDelta::Spaces(
                    buffer_start_indent.spaces as isize - resolved_old_text.indent.spaces as isize,
                )
            };

            let old_text = snapshot
                .text_for_range(resolved_old_text.range.clone())
                .collect::<String>();
            let mut diff = StreamingDiff::new(old_text);
            let mut edit_start = resolved_old_text.range.start;
            let mut new_text_chunks =
                Self::reindent_new_text_chunks(indent_delta, &mut edit_events);
            let mut done = false;
            while !done {
                let char_operations = if let Some(new_text_chunk) = new_text_chunks.next().await {
                    diff.push_new(&new_text_chunk?)
                } else {
                    done = true;
                    mem::take(&mut diff).finish()
                };

                for op in char_operations {
                    match op {
                        CharOperation::Insert { text } => {
                            let edit_start = snapshot.anchor_after(edit_start);
                            edits_tx.unbounded_send((edit_start..edit_start, Arc::from(text)))?;
                        }
                        CharOperation::Delete { bytes } => {
                            let edit_end = edit_start + bytes;
                            let edit_range =
                                snapshot.anchor_after(edit_start)..snapshot.anchor_before(edit_end);
                            edit_start = edit_end;
                            edits_tx.unbounded_send((edit_range, Arc::from("")))?;
                        }
                        CharOperation::Keep { bytes } => edit_start += bytes,
                    }
                }
            }

            drop(new_text_chunks);
            anyhow::Ok(edit_events)
        });

        (compute_edits, edits_rx)
    }

    fn reindent_new_text_chunks(
        delta: IndentDelta,
        mut stream: impl Unpin + Stream<Item = Result<EditParserEvent>>,
    ) -> impl Stream<Item = Result<String>> {
        let mut buffer = String::new();
        let mut in_leading_whitespace = true;
        let mut done = false;
        futures::stream::poll_fn(move |cx| {
            while !done {
                let (chunk, is_last_chunk) = match stream.poll_next_unpin(cx) {
                    Poll::Ready(Some(Ok(EditParserEvent::NewTextChunk { chunk, done }))) => {
                        (chunk, done)
                    }
                    Poll::Ready(Some(Err(err))) => return Poll::Ready(Some(Err(err))),
                    Poll::Pending => return Poll::Pending,
                    _ => return Poll::Ready(None),
                };

                buffer.push_str(&chunk);

                let mut indented_new_text = String::new();
                let mut start_ix = 0;
                let mut newlines = buffer.match_indices('\n').peekable();
                loop {
                    let (line_end, is_pending_line) = match newlines.next() {
                        Some((ix, _)) => (ix, false),
                        None => (buffer.len(), true),
                    };
                    let line = &buffer[start_ix..line_end];

                    if in_leading_whitespace {
                        if let Some(non_whitespace_ix) = line.find(|c| delta.character() != c) {
                            // We found a non-whitespace character, adjust
                            // indentation based on the delta.
                            let new_indent_len =
                                cmp::max(0, non_whitespace_ix as isize + delta.len()) as usize;
                            indented_new_text
                                .extend(iter::repeat(delta.character()).take(new_indent_len));
                            indented_new_text.push_str(&line[non_whitespace_ix..]);
                            in_leading_whitespace = false;
                        } else if is_pending_line {
                            // We're still in leading whitespace and this line is incomplete.
                            // Stop processing until we receive more input.
                            break;
                        } else {
                            // This line is entirely whitespace. Push it without indentation.
                            indented_new_text.push_str(line);
                        }
                    } else {
                        indented_new_text.push_str(line);
                    }

                    if is_pending_line {
                        start_ix = line_end;
                        break;
                    } else {
                        in_leading_whitespace = true;
                        indented_new_text.push('\n');
                        start_ix = line_end + 1;
                    }
                }
                buffer.replace_range(..start_ix, "");

                // This was the last chunk, push all the buffered content as-is.
                if is_last_chunk {
                    indented_new_text.push_str(&buffer);
                    buffer.clear();
                    done = true;
                }

                if !indented_new_text.is_empty() {
                    return Poll::Ready(Some(Ok(indented_new_text)));
                }
            }

            Poll::Ready(None)
        })
    }

    fn parse_edit_chunks_multi_file(
        &self,
        chunks: impl 'static + Send + Stream<Item = Result<String, LanguageModelCompletionError>>,
        edit_format: EditFormat,
        cx: &AsyncApp,
    ) -> (
        Task<Result<EditParserMetrics>>,
        UnboundedReceiver<Result<EditParserEvent>>,
    ) {
        let (tx, rx) = mpsc::unbounded();
        let output = cx.background_spawn(async move {
            pin_mut!(chunks);

            let mut parser = EditParser::new(edit_format);
            let mut raw_edits = String::new();
            while let Some(chunk) = chunks.next().await {
                match chunk {
                    Ok(chunk) => {
                        raw_edits.push_str(&chunk);
                        for event in parser.push(&chunk) {
                            tx.unbounded_send(Ok(event))?;
                        }
                    }
                    Err(error) => {
                        tx.unbounded_send(Err(error.into()))?;
                    }
                }
            }
            // Send final events
            for event in parser.push("") {
                tx.unbounded_send(Ok(event))?;
            }
            Ok(parser.finish())
        });
        (output, rx)
    }

    async fn apply_edit_chunks_from_events<T>(
        &self,
        buffer: Entity<Buffer>,
        path: PathBuf,
        events_stream: T,
        output_events: mpsc::UnboundedSender<EditAgentOutputEvent>,
        cx: &mut AsyncApp,
    ) -> Result<EditAgentOutput>
    where
        T: Stream<Item = Result<EditParserEvent>> + Unpin + Send + 'static,
    {
        self.action_log
            .update(cx, |log, cx| log.buffer_read(buffer.clone(), cx))?;

        let mut edit_events = events_stream.peekable();
        while let Some(edit_event) = Pin::new(&mut edit_events).peek().await {
            // Skip events until we're at the start of a new edit.
            let Ok(EditParserEvent::OldTextChunk { .. }) = edit_event else {
                edit_events.next().await.unwrap()?;
                continue;
            };

            let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

            // Resolve the old text in the background
            let (resolve_old_text, mut old_range) =
                Self::resolve_old_text(snapshot.text.clone(), edit_events, cx);
            while old_range.changed().await.is_ok() {
                if let Some(old_range) = old_range.borrow().clone() {
                    let old_range = snapshot.anchor_before(old_range.start)
                        ..snapshot.anchor_before(old_range.end);
                    self.project.update(cx, |project, cx| {
                        project.set_agent_location(
                            Some(AgentLocation {
                                buffer: buffer.downgrade(),
                                position: old_range.end,
                            }),
                            cx,
                        );
                    })?;
                    output_events
                        .unbounded_send(EditAgentOutputEvent::ResolvingEditRange {
                            path: path.clone(),
                            range: old_range,
                        })
                        .ok();
                }
            }

            let (edit_events_, mut resolved_old_text) = resolve_old_text.await?;
            edit_events = edit_events_;

            // If we can't resolve the old text, restart the loop
            let resolved_old_text = match resolved_old_text.len() {
                1 => resolved_old_text.pop().unwrap(),
                0 => {
                    output_events
                        .unbounded_send(EditAgentOutputEvent::UnresolvedEditRange {
                            path: path.clone(),
                        })
                        .ok();
                    continue;
                }
                _ => {
                    let ranges = resolved_old_text
                        .into_iter()
                        .map(|text| {
                            let start_line =
                                (snapshot.offset_to_point(text.range.start).row + 1) as usize;
                            let end_line =
                                (snapshot.offset_to_point(text.range.end).row + 1) as usize;
                            start_line..end_line
                        })
                        .collect();
                    output_events
                        .unbounded_send(EditAgentOutputEvent::AmbiguousEditRange {
                            path: path.clone(),
                            ranges,
                        })
                        .ok();
                    continue;
                }
            };

            // Compute edits in the background and apply them
            let (compute_edits, edits) =
                Self::compute_edits(snapshot, resolved_old_text, edit_events, cx);
            let mut edits = edits.ready_chunks(32);
            while let Some(edits) = edits.next().await {
                if edits.is_empty() {
                    continue;
                }

                cx.update(|cx| {
                    let max_edit_end = buffer.update(cx, |buffer, cx| {
                        buffer.edit(edits.iter().cloned(), None, cx);
                        let max_edit_end = buffer
                            .summaries_for_anchors::<Point, _>(
                                edits.iter().map(|(range, _)| &range.end),
                            )
                            .max()
                            .unwrap();
                        buffer.anchor_before(max_edit_end)
                    });
                    self.action_log
                        .update(cx, |log, cx| log.buffer_edited(buffer.clone(), cx));
                    self.project.update(cx, |project, cx| {
                        project.set_agent_location(
                            Some(AgentLocation {
                                buffer: buffer.downgrade(),
                                position: max_edit_end,
                            }),
                            cx,
                        );
                    });
                })?;
                output_events
                    .unbounded_send(EditAgentOutputEvent::Edited { path: path.clone() })
                    .ok();
            }

            edit_events = compute_edits.await?;
        }

        Ok(EditAgentOutput {
            raw_edits: String::new(),
            parser_metrics: EditParserMetrics::default(),
        })
    }

    async fn path_to_project_path(
        &self,
        path: &PathBuf,
        project: &Entity<Project>,
        cx: &AsyncApp,
    ) -> Result<ProjectPath> {
        // Try to find the path in the project's worktrees
        let worktrees =
            project.read_with(cx, |project, cx| project.worktrees(cx).collect::<Vec<_>>())?;

        for worktree in worktrees {
            let (worktree_id, worktree_root) = worktree.read_with(cx, |worktree, _| {
                (worktree.id(), worktree.abs_path().clone())
            })?;

            // Check if the path starts with any component of the worktree path
            if let Ok(relative_path) =
                path.strip_prefix(worktree_root.file_name().unwrap_or_default())
            {
                return Ok(ProjectPath {
                    worktree_id,
                    path: relative_path.into(),
                });
            }

            // Also try direct relative path
            if worktree_root.join(path).exists() {
                return Ok(ProjectPath {
                    worktree_id,
                    path: path.clone().into(),
                });
            }
        }

        Err(anyhow::anyhow!("Path not found in project: {:?}", path))
    }

    async fn request(
        &self,
        mut conversation: LanguageModelRequest,
        intent: CompletionIntent,
        prompt: String,
        cx: &mut AsyncApp,
    ) -> Result<BoxStream<'static, Result<String, LanguageModelCompletionError>>> {
        let mut messages_iter = conversation.messages.iter_mut();
        if let Some(last_message) = messages_iter.next_back() {
            if last_message.role == Role::Assistant {
                let old_content_len = last_message.content.len();
                last_message
                    .content
                    .retain(|content| !matches!(content, MessageContent::ToolUse(_)));
                let new_content_len = last_message.content.len();

                // We just removed pending tool uses from the content of the
                // last message, so it doesn't make sense to cache it anymore
                // (e.g., the message will look very different on the next
                // request). Thus, we move the flag to the message prior to it,
                // as it will still be a valid prefix of the conversation.
                if old_content_len != new_content_len && last_message.cache {
                    if let Some(prev_message) = messages_iter.next_back() {
                        last_message.cache = false;
                        prev_message.cache = true;
                    }
                }

                if last_message.content.is_empty() {
                    conversation.messages.pop();
                }
            } else {
                debug_panic!(
                    "Last message must be an Assistant tool calling! Got {:?}",
                    last_message.content
                );
            }
        }

        conversation.messages.push(LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text(prompt)],
            cache: false,
        });

        // Include tools in the request so that we can take advantage of
        // caching when ToolChoice::None is supported.
        let mut tool_choice = None;
        let mut tools = Vec::new();
        if !conversation.tools.is_empty()
            && self
                .model
                .supports_tool_choice(LanguageModelToolChoice::None)
        {
            tool_choice = Some(LanguageModelToolChoice::None);
            tools = conversation.tools.clone();
        }

        let request = LanguageModelRequest {
            thread_id: conversation.thread_id,
            prompt_id: conversation.prompt_id,
            intent: Some(intent),
            mode: conversation.mode,
            messages: conversation.messages,
            tool_choice,
            tools,
            stop: Vec::new(),
            temperature: None,
            thinking_allowed: true,
        };

        Ok(self.model.stream_completion_text(request, cx).await?.stream)
    }
}

struct ResolvedOldText {
    range: Range<usize>,
    indent: LineIndent,
}

#[derive(Copy, Clone, Debug)]
enum IndentDelta {
    Spaces(isize),
    Tabs(isize),
}

impl IndentDelta {
    fn character(&self) -> char {
        match self {
            IndentDelta::Spaces(_) => ' ',
            IndentDelta::Tabs(_) => '\t',
        }
    }

    fn len(&self) -> isize {
        match self {
            IndentDelta::Spaces(n) => *n,
            IndentDelta::Tabs(n) => *n,
        }
    }
}
