use std::{collections::HashMap, path::Path, sync::Arc};

use axum::{Json, Router};
use futures::{
    channel::{mpsc, oneshot},
    future::{BoxFuture, LocalBoxFuture},
    io::BufReader,
    stream::BoxStream,
    AsyncBufReadExt, FutureExt, SinkExt, StreamExt as _,
};
use gpui::{App, AppContext as _, AsyncApp, Entity, EventEmitter, Task, WeakEntity};
use http_client::{AsyncBody, HttpClient, HttpClientWithUrl, Request};
use itertools::Itertools;
use language::Buffer;
use log::{error, info};
use project::{ProjectItem, ProjectPath};
use rope::Point;
use serde::Deserialize;
use util::ResultExt;
use workspace::Workspace;

use crate::{
    assistant_panel::SidecarPatch,
    thread::{MessageId, ThreadId},
    thread_store::ThreadStore,
    types::{
        self, AgentSessionChatRequest, AnthropicAPIKey, Diagnostic, DocumentSymbol,
        EditedCodeStreamingEvent, EditedCodeStreamingRequest, Entity as LLMEntity,
        GoToDefinitionRequest, LLMClientConfig, LLMProvider, LLMProviderAPIKeys, LLMType,
        LSPDiagnosticsInput, LSPDiagnosticsOutput, OpenFileRequestPartial, OpenFileResponse,
        RepoRef, UIEventWithID, UserContext,
    },
};

pub struct Sidecar {
    client: Arc<HttpClientWithUrl>,
    thread_store: Entity<ThreadStore>,
    workspace: WeakEntity<Workspace>,
    active_edits: HashMap<String, SidecarPatch>,
}

pub fn llm_client_config() -> LLMClientConfig {
    LLMClientConfig {
        slow_model: LLMType::ClaudeSonnet,
        fast_model: LLMType::ClaudeSonnet,
        models: HashMap::from_iter([(
            LLMType::ClaudeSonnet,
            LLMEntity {
                context_length: 200_000, // sidecar just ignores it?
                temperature: 0.0,        // sidecar just ignores it?
                provider: LLMProvider::Anthropic,
            },
        )]),
        providers: vec![LLMProviderAPIKeys::Anthropic(AnthropicAPIKey {
            api_key: std::env::var("ANTHROPIC_API_KEY")
                .expect("please set anthropic api key env variable"),
        })],
    }
}

const UI_SERVER_PORT: u16 = 29159;
type OnMainThreadTask = Box<
    dyn Send
        + Sync
        + 'static
        + FnOnce(WeakEntity<Sidecar>, AsyncApp) -> LocalBoxFuture<'static, ()>,
>;

impl Sidecar {
    pub fn new(
        thread_store: Entity<ThreadStore>,
        workspace: WeakEntity<Workspace>,
        cx: &mut App,
    ) -> Entity<Self> {
        let client = Arc::new(HttpClientWithUrl::new(
            cx.http_client(),
            "http://localhost:19463/api",
            None,
        ));
        let model = cx.new(|_| Sidecar {
            client,
            workspace,
            thread_store,
            active_edits: Default::default(),
        });
        let weak = model.downgrade();
        let (tx, mut rx) = mpsc::unbounded::<OnMainThreadTask>();
        // run tasks sends by tokio thread
        cx.spawn(move |cx| async move {
            while let Some(f) = rx.next().await {
                let weak_model = weak.clone();
                cx.spawn(|cx| f(weak_model, cx)).detach();
            }
        })
        .detach();
        Self::start_server(tx);
        model
    }

    pub fn anchored_edit(
        &self,
        request: AgentSessionChatRequestMinimal,
    ) -> BoxFuture<'static, anyhow::Result<BoxStream<'static, anyhow::Result<UIEventWithID>>>> {
        let client = self.client.clone();
        async move {
            // FIXME: get it correctly
            let request = AgentSessionChatRequest {
                session_id: request.session_id,
                exchange_id: request.exchange_id,
                editor_url: format!("http://127.0.0.1:{port}", port = UI_SERVER_PORT),
                query: request.query,
                user_context: request.user_context,
                repo_ref: RepoRef::new(&request.root),
                root_directory: request.root.clone(),
                project_labels: vec![],
                codebase_search: false,
                reasoning: request.reasoning,
                access_token: String::new(),
                model_configuration: llm_client_config(),
                all_files: vec![],
                open_files: vec![],
                shell: "bash".into(),
            };
            let response = client
                .send(
                    Request::post(client.build_url("/agentic/agent_tool_use"))
                        .header("Content-Type", "application/json")
                        .body(AsyncBody::from(serde_json::to_vec(&request)?))?,
                )
                .await?;
            let body = response.into_body();
            let reader = BufReader::new(body);
            Ok(reader
                .lines()
                .filter_map(|line| async move {
                    match line {
                        Ok(line) => {
                            #[derive(Deserialize)]
                            #[serde(untagged)]
                            #[allow(unused)]
                            enum ResponseType {
                                Event(UIEventWithID),
                                Started { started: bool, session_id: String },
                                Done { done: String, session_id: String },
                            }
                            let line = line.strip_prefix("data:")?;
                            match serde_json::from_str(line) {
                                Ok(ResponseType::Event(response)) => Some(Ok(response)),
                                Ok(ResponseType::Started { .. } | ResponseType::Done { .. }) => {
                                    None
                                }
                                Err(error) => Some(Err(anyhow::format_err!(error))),
                            }
                        }
                        Err(error) => Some(Err(anyhow::format_err!(error))),
                    }
                })
                .boxed())
        }
        .boxed()
    }

    // we need to run axum on tokio runtime on a different thread and
    // send tasks to run on main thread that have access to editor types
    // this server sucks, ideally this changes to jsonrpc over stdio
    fn start_server(tx: mpsc::UnboundedSender<OnMainThreadTask>) {
        let _ = std::thread::Builder::new()
            .name("editor_server".to_owned())
            .spawn(move || {
                let runtime = tokio::runtime::Runtime::new()?;
                runtime.block_on(Self::server(tx))
            });
    }

    async fn server(tx: mpsc::UnboundedSender<OnMainThreadTask>) -> anyhow::Result<()> {
        let tcp = tokio::net::TcpListener::bind(("127.0.0.1", UI_SERVER_PORT)).await?;
        axum::serve(
            tcp,
            Router::new().fallback({
                let mut tx = tx.clone();
                move |r: axum::http::Uri, body: String| async move {
                    let path = r.path().to_owned();
                    let (otx, orx) = oneshot::channel();
                    tx.send(Box::new(move |this: WeakEntity<Self>, cx: AsyncApp| {
                        Box::pin(async move {
                            let body = serde_json::from_str(&body).unwrap_or_default();
                            let res =
                                Self::handle_request(this.upgrade().unwrap(), &path, body, cx)
                                    .await;
                            otx.send(res).unwrap();
                        })
                    }))
                    .await
                    .unwrap();
                    Json(orx.await.unwrap())
                }
            }),
        )
        .await?;
        Ok(())
    }

    pub fn root(&self, cx: &App) -> Arc<Path> {
        let proj = self.workspace.upgrade().unwrap().read(cx).project().clone();
        proj.read(cx)
            .visible_worktrees(cx)
            .next()
            .unwrap()
            .read(cx)
            .abs_path()
    }

    async fn open_buffer(
        this: &Entity<Self>,
        abs_path: &str,
        cx: &AsyncApp,
    ) -> anyhow::Result<Entity<Buffer>> {
        let buf = cx
            .update(|cx| {
                let proj = this
                    .read(cx)
                    .workspace
                    .upgrade()
                    .unwrap()
                    .read(cx)
                    .project()
                    .clone();
                proj.update(cx, |p, cx| {
                    let Some((w, path)) = p.find_worktree(abs_path.as_ref(), cx) else {
                        return Task::ready(Err(anyhow::format_err!("not found: {abs_path}")));
                    };
                    let proj_path = ProjectPath {
                        worktree_id: w.read(cx).id(),
                        path: path.into(),
                    };
                    p.open_buffer(proj_path, cx)
                })
            })?
            .await?;
        Ok(buf)
    }

    async fn lsp(
        this: &Entity<Self>,
        buf: &Entity<Buffer>,
        cx: &AsyncApp,
    ) -> anyhow::Result<Option<(Arc<lsp::LanguageServer>, lsp::TextDocumentIdentifier)>> {
        cx.update(|cx| {
            let proj = this
                .read(cx)
                .workspace
                .upgrade()
                .unwrap()
                .read(cx)
                .project()
                .clone()
                .read(cx);

            let lsp = proj
                .lsp_store()
                .read(cx)
                .as_local()?
                .primary_language_server_for_buffer(buf.read(cx), cx)?
                .1
                .clone();
            Some((
                lsp,
                lsp::TextDocumentIdentifier::new(
                    lsp::Url::from_file_path(
                        proj.absolute_path(&buf.read(cx).project_path(cx).unwrap(), cx)
                            .unwrap(),
                    )
                    .unwrap(),
                ),
            ))
        })
    }

    async fn handle_request(
        this: Entity<Self>,
        path: &str,
        body: serde_json::Value,
        mut cx: AsyncApp,
    ) -> serde_json::Value {
        info!("request: {path}");
        match path {
            "/new_exchange" => {
                let id: ThreadId = serde_json::from_value(body["session_id"].clone()).unwrap();
                let thread = cx
                    .update(|cx| {
                        this.update(cx, |this, cx| {
                            this.thread_store.update(cx, |t, cx| t.open_thread(&id, cx))
                        })
                    })
                    .expect("app is fine")
                    .await
                    .expect("thread load is fine");
                let id = cx
                    .update(|cx| thread.update(cx, |t, _| t.next_message_id()))
                    .expect("app is fine");
                serde_json::json!({
                    "exchange_id": format!("{id}", id = id.0),
                })
            }
            "/recent_edits" => {
                serde_json::json!({
                    "changed_files": [],
                })
            }
            "/file_open" => {
                let request = serde_json::from_value::<OpenFileRequestPartial>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx).await;
                let resp = if let Ok(buf) = buf {
                    let text = cx.update_entity(&buf, |buf, _| buf.text()).unwrap();
                    OpenFileResponse {
                        fs_file_path: request.fs_file_path.clone(),
                        file_contents: text,
                        exists: true,
                        language: "rust".to_string(),
                    }
                } else {
                    OpenFileResponse {
                        fs_file_path: request.fs_file_path.clone(),
                        file_contents: String::new(),
                        exists: false,
                        language: "rust".to_string(),
                    }
                };
                serde_json::to_value(resp).unwrap()
            }
            "/create_file" => {
                // FIXME: actually create files
                serde_json::json!({
                    "done": false,
                    "fs_file_path": "",
                })
            }
            "/get_outline_nodes" => {
                let request = serde_json::from_value::<OpenFileRequestPartial>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();
                let syms = if let Some((lsp, text_document)) =
                    Self::lsp(&this, &buf, &cx).await.unwrap()
                {
                    let response = lsp
                        .request::<lsp::DocumentSymbolRequest>(lsp::DocumentSymbolParams {
                            text_document,
                            work_done_progress_params: Default::default(),
                            partial_result_params: Default::default(),
                        })
                        .await
                        .unwrap();
                    match response {
                        None => vec![],
                        Some(lsp::DocumentSymbolResponse::Flat(_)) => vec![],
                        Some(lsp::DocumentSymbolResponse::Nested(symbols)) => {
                            symbols.into_iter().map(DocumentSymbol::from).collect()
                        }
                    }
                } else {
                    vec![]
                };
                let text = cx.update_entity(&buf, |buf, _| buf.text()).unwrap();
                serde_json::json!({
                    "file_content": text,
                    "language": "rust",
                    "outline_nodes": syms
                })
            }
            "/apply_edits_streamed" => {
                let request = serde_json::from_value::<EditedCodeStreamingRequest>(body).unwrap();
                let buffer = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();
                let thread = ThreadId(request.session_id.into());
                let message = MessageId(request.exchange_id.into());
                cx.update(|cx| {
                    this.update(cx, |this, cx| {
                        let start_point: Point = request.range.start_position.into();
                        let mut end_point: Point = request.range.end_position.into();
                        // sidecar range is inclusive row level
                        end_point.row += 1;
                        let range = buffer.read(cx).anchor_after(start_point)
                            ..buffer.read(cx).anchor_before(end_point);
                        let patch = this
                            .active_edits
                            .entry(request.edit_request_id.clone())
                            .or_insert(SidecarPatch {
                                original: range.clone(),
                                replacement: String::new(),
                                done: false,
                            });
                        patch.original = range;
                        assert_eq!(patch.done, false);
                        match request.event {
                            EditedCodeStreamingEvent::Start => {}
                            EditedCodeStreamingEvent::Delta(d) => {
                                // ignore markdown
                                if !d.contains("```") {
                                    patch.replacement += &d;
                                }
                            }
                            EditedCodeStreamingEvent::End => {
                                patch.done = true;
                                patch.replacement += "\n";
                                cx.emit(SidecarEvent::Patch {
                                    thread,
                                    message,
                                    buffer,
                                    patch: patch.clone(),
                                });
                                assert!(this
                                    .active_edits
                                    .remove(&request.edit_request_id)
                                    .is_some())
                            }
                        }
                    })
                })
                .log_err();
                serde_json::json!({})
            }
            "/go_to_definition" => {
                let request = serde_json::from_value::<GoToDefinitionRequest>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();

                let locations = if let Some((lsp, text_document)) =
                    Self::lsp(&this, &buf, &cx).await.unwrap()
                {
                    let response = lsp
                        .request::<lsp::request::GotoDefinition>(lsp::GotoDefinitionParams {
                            text_document_position_params: lsp::TextDocumentPositionParams::new(
                                text_document,
                                request.position.into(),
                            ),
                            work_done_progress_params: Default::default(),
                            partial_result_params: Default::default(),
                        })
                        .await
                        .unwrap();

                    match response {
                        None => vec![],
                        Some(lsp::GotoDefinitionResponse::Scalar(location)) => vec![location],
                        Some(lsp::GotoDefinitionResponse::Array(locations)) => locations,
                        Some(lsp::GotoDefinitionResponse::Link(links)) => links
                            .into_iter()
                            .map(|link| lsp::Location::new(link.target_uri, link.target_range))
                            .collect(),
                    }
                } else {
                    vec![]
                };
                let loc = locations
                    .into_iter()
                    .map(|x| types::DefinitionLocation {
                        fs_file_path: x.uri.path().to_string(),
                        range: (x.range.start..x.range.end).into(),
                    })
                    .collect_vec();
                serde_json::json!({
                    "definitions": loc,
                })
            }
            "/go_to_implementation" => {
                let request = serde_json::from_value::<GoToDefinitionRequest>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();

                let locations = if let Some((lsp, text_document)) =
                    Self::lsp(&this, &buf, &cx).await.unwrap()
                {
                    let response = lsp
                        .request::<lsp::request::GotoImplementation>(
                            lsp::GotoImplementationParams {
                                text_document_position_params: lsp::TextDocumentPositionParams::new(
                                    text_document,
                                    request.position.into(),
                                ),
                                work_done_progress_params: Default::default(),
                                partial_result_params: Default::default(),
                            },
                        )
                        .await
                        .unwrap();

                    match response {
                        None => vec![],
                        Some(lsp::GotoDefinitionResponse::Scalar(location)) => vec![location],
                        Some(lsp::GotoDefinitionResponse::Array(locations)) => locations,
                        Some(lsp::GotoDefinitionResponse::Link(links)) => links
                            .into_iter()
                            .map(|link| lsp::Location::new(link.target_uri, link.target_range))
                            .collect(),
                    }
                } else {
                    vec![]
                };
                let loc = locations
                    .into_iter()
                    .map(|x| types::DefinitionLocation {
                        fs_file_path: x.uri.path().to_string(),
                        range: (x.range.start..x.range.end).into(),
                    })
                    .collect_vec();
                serde_json::json!({
                    "implementation_locations": loc,
                })
            }
            "/go_to_references" => {
                let request = serde_json::from_value::<GoToDefinitionRequest>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();

                let locations = if let Some((lsp, text_document)) =
                    Self::lsp(&this, &buf, &cx).await.unwrap()
                {
                    let response = lsp
                        .request::<lsp::request::References>(lsp::ReferenceParams {
                            text_document_position: lsp::TextDocumentPositionParams::new(
                                text_document,
                                request.position.into(),
                            ),
                            work_done_progress_params: Default::default(),
                            partial_result_params: Default::default(),
                            context: lsp::ReferenceContext {
                                include_declaration: true,
                            },
                        })
                        .await
                        .unwrap();

                    response.unwrap_or_default()
                } else {
                    vec![]
                };
                let loc = locations
                    .into_iter()
                    .map(|x| types::DefinitionLocation {
                        fs_file_path: x.uri.path().to_string(),
                        range: (x.range.start..x.range.end).into(),
                    })
                    .collect_vec();
                serde_json::json!({
                    "reference_locations": loc,
                })
            }
            "/go_to_type_definition" => {
                let request = serde_json::from_value::<GoToDefinitionRequest>(body).unwrap();
                let buf = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();

                let locations = if let Some((lsp, text_document)) =
                    Self::lsp(&this, &buf, &cx).await.unwrap()
                {
                    let response = lsp
                        .request::<lsp::request::GotoTypeDefinition>(
                            lsp::GotoTypeDefinitionParams {
                                text_document_position_params: lsp::TextDocumentPositionParams::new(
                                    text_document,
                                    request.position.into(),
                                ),
                                work_done_progress_params: Default::default(),
                                partial_result_params: Default::default(),
                            },
                        )
                        .await
                        .unwrap();

                    match response {
                        None => vec![],
                        Some(lsp::GotoDefinitionResponse::Scalar(location)) => vec![location],
                        Some(lsp::GotoDefinitionResponse::Array(locations)) => locations,
                        Some(lsp::GotoDefinitionResponse::Link(links)) => links
                            .into_iter()
                            .map(|link| lsp::Location::new(link.target_uri, link.target_range))
                            .collect(),
                    }
                } else {
                    vec![]
                };
                let loc = locations
                    .into_iter()
                    .map(|x| types::DefinitionLocation {
                        fs_file_path: x.uri.path().to_string(),
                        range: (x.range.start..x.range.end).into(),
                    })
                    .collect_vec();
                serde_json::json!({
                    "definitions": loc,
                })
            }
            "/rip_grep_path" => {
                serde_json::json!({ "rip_grep_path": "rg" })
            }
            "/terminal_output_new" => {
                serde_json::json!({})
            }
            "/diagnostics" => {
                let request = serde_json::from_value::<LSPDiagnosticsInput>(body).unwrap();
                let buffer = Self::open_buffer(&this, &request.fs_file_path, &cx)
                    .await
                    .unwrap();

                let diagnostics = cx
                    .update(|cx| {
                        buffer
                            .read(cx)
                            .snapshot()
                            .diagnostics_in_range::<text::Point, text::Point>(
                                request.range.into(),
                                false,
                            )
                            .map(|entry| Diagnostic {
                                message: entry.diagnostic.message,
                                range: entry.range.into(),
                                quick_fix_labels: None,
                                parameter_hints: None,
                                fs_file_path: request.fs_file_path.clone(),
                            })
                            .collect()
                    })
                    .unwrap();

                serde_json::to_value(LSPDiagnosticsOutput { diagnostics }).unwrap()
            }
            _ => {
                error!("unknown request: {path}");
                serde_json::json!({})
            }
        }
    }
}

pub enum SidecarEvent {
    Patch {
        thread: ThreadId,
        message: MessageId,
        buffer: Entity<Buffer>,
        patch: SidecarPatch,
    },
}

impl EventEmitter<SidecarEvent> for Sidecar {}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentSessionChatRequestMinimal {
    pub session_id: String,
    pub exchange_id: String,
    pub query: String,
    pub user_context: UserContext,
    pub root: String,
    pub reasoning: bool,
}
