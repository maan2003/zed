use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context as _, Result, anyhow};
use editor::{
    Editor, EditorMode, Inlay, RowExt, SelectionEffects, SizingBehavior, scroll::Autoscroll,
};
use gpui::{
    App, Context, Entity, Focusable as _, FontStyle, FontWeight, HighlightStyle, Hsla, KeyBinding,
    Rgba, StyledText, Subscription, Task, TextStyle, Window, WindowOptions, actions, div,
    prelude::*, px,
};
use language::{Buffer, BufferEvent, Capability, Point};
use multi_buffer::{MultiBuffer, PathKey};
use project::InlayId;
use settings::SettingsStore;
use tau_proto::{
    CborValue, ContentPart, ContextItem, ContextRole, Event, Frame, Message, ModelParams,
    PromptMessageClass, PromptOriginator, ToolCallItem, UiPromptSubmitted,
};
use text::ToOffset as _;
use theme::ActiveTheme as _;

mod activity_state;
mod agent_state;
mod cli_theme;
mod commands;
mod socket_client;
mod status_line;
mod tool_render;
mod transcript;
use activity_state::MainToolActivity;
use agent_state::{AgentContextUsage, AgentState};
use commands::parse_role_setting_update;
use socket_client::{SocketEvent, Writer};
#[cfg(test)]
use transcript::buffer_range_starts_with;
use transcript::{InsertedTranscript, Transcript};

actions!(tau_gui, [SubmitPrompt]);

fn main() {
    if let Err(error) = run() {
        eprintln!("tau-gui: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let attach_target = attach_target_for_current_dir()?;

    gpui_platform::application()
        .with_assets(assets::Assets)
        .run(move |cx: &mut App| {
            if let Err(error) = init_app(cx) {
                eprintln!("tau-gui: {error:#}");
                cx.quit();
                return;
            }

            eprintln!("tau-gui: binding ctrl-enter to tau_gui::SubmitPrompt in TauGui > Editor");
            cx.bind_keys([KeyBinding::new(
                "ctrl-enter",
                SubmitPrompt,
                Some("TauGui > Editor"),
            )]);
            cx.activate(true);

            let attach_target = attach_target.clone();
            if let Err(error) = cx.open_window(WindowOptions::default(), move |window, cx| {
                cx.new(|cx| TauGui::new(attach_target.clone(), window, cx))
            }) {
                eprintln!("tau-gui: failed to open window: {error:#}");
                cx.quit();
            }
        });

    Ok(())
}

#[derive(Clone)]
struct AttachTarget {
    socket_path: PathBuf,
    session_id: tau_proto::SessionId,
}

fn attach_target_for_current_dir() -> Result<AttachTarget> {
    let project_root = std::env::current_dir().context("failed to read current directory")?;
    let daemon_dir = tau_harness::runtime_dir::find_harness_for_dir(&project_root)
        .ok_or_else(|| anyhow!("no running Tau harness for {}", project_root.display()))?;
    let session_id = tau_harness::runtime_dir::read_session_id(&daemon_dir)
        .ok_or_else(|| anyhow!("running Tau harness did not publish a session id"))?
        .into();

    Ok(AttachTarget {
        socket_path: tau_harness::runtime_dir::socket_path(&daemon_dir),
        session_id,
    })
}

fn init_app(cx: &mut App) -> Result<()> {
    assets::Assets.load_fonts(cx)?;
    let settings_path = tau_gui_settings_path()?;
    let user_settings = load_or_create_tau_gui_settings(&settings_path)?;
    let mut store = SettingsStore::new(cx, settings::default_settings().as_ref());
    store
        .set_user_settings(&user_settings, cx)
        .result()
        .with_context(|| format!("failed to load settings from {}", settings_path.display()))?;
    cx.set_global(store);
    theme_settings::init(theme::LoadThemes::All(Box::new(assets::Assets)), cx);
    release_channel::init(semver::Version::new(0, 1, 0), cx);
    editor::init(cx);
    command_palette::init(cx);
    search::init(cx);
    vim::init(cx);
    let default_key_bindings =
        settings::KeymapFile::load_asset_allow_partial_failure(settings::DEFAULT_KEYMAP_PATH, cx)
            .context("failed to load default keymap")?;
    eprintln!(
        "tau-gui: loaded {} default key bindings from {}",
        default_key_bindings.len(),
        settings::DEFAULT_KEYMAP_PATH
    );
    cx.bind_keys(default_key_bindings);
    let vim_key_bindings =
        settings::KeymapFile::load_asset_allow_partial_failure(settings::VIM_KEYMAP_PATH, cx)
            .context("failed to load vim keymap")?;
    eprintln!(
        "tau-gui: loaded {} vim key bindings from {}",
        vim_key_bindings.len(),
        settings::VIM_KEYMAP_PATH
    );
    cx.bind_keys(vim_key_bindings);
    Ok(())
}

const PROMPT_PLACEHOLDER_INLAY_ID: usize = 0;
const DEFAULT_TAU_GUI_SETTINGS: &str = r#"// Tau GUI user settings. Values here override bundled defaults.
{}
"#;

fn tau_gui_settings_path() -> Result<PathBuf> {
    let config_dir = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(config_home) => PathBuf::from(config_home),
        None => std::env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(".config"))
            .ok_or_else(|| anyhow!("neither XDG_CONFIG_HOME nor HOME is set"))?,
    };

    Ok(config_dir.join("tau-gui").join("settings.json"))
}

fn load_or_create_tau_gui_settings(path: &Path) -> Result<String> {
    match fs::read_to_string(path) {
        Ok(settings) => Ok(settings),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("failed to create settings directory {}", parent.display())
                })?;
            }
            fs::write(path, DEFAULT_TAU_GUI_SETTINGS).with_context(|| {
                format!("failed to write default settings to {}", path.display())
            })?;
            Ok(DEFAULT_TAU_GUI_SETTINGS.to_owned())
        }
        Err(error) => {
            Err(error).with_context(|| format!("failed to read settings from {}", path.display()))
        }
    }
}

fn selection_outside_prompt(
    selection_offset: usize,
    prompt_start: usize,
    draft_end: usize,
) -> bool {
    selection_offset < prompt_start || selection_offset > draft_end
}

#[cfg(test)]
fn buffer_text_ends_with(buffer: &Buffer, end: usize, character: char) -> bool {
    if end == 0 {
        return false;
    }

    buffer
        .text_for_range(0..end)
        .collect::<String>()
        .ends_with(character)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TranscriptStyle {
    UserPrompt,
    UserPromptQueued,
    AgentResponse,
    ToolProgress,
    SystemInfo,
    SystemImportant,
    SystemDisconnect,
}

impl TranscriptStyle {
    fn style_name(self) -> &'static str {
        match self {
            Self::UserPrompt => tau_themes::names::USER_PROMPT,
            Self::UserPromptQueued => tau_themes::names::USER_PROMPT_QUEUED,
            Self::AgentResponse => tau_themes::names::AGENT_RESPONSE,
            Self::ToolProgress => tau_themes::names::PROGRESS_INDICATOR,
            Self::SystemInfo => tau_themes::names::SYSTEM_INFO,
            Self::SystemImportant => tau_themes::names::SYSTEM_INFO_IMPORTANT,
            Self::SystemDisconnect => tau_themes::names::SYSTEM_DISCONNECT,
        }
    }
}
struct TauGui {
    editor: Entity<Editor>,
    prompt_buffer: Entity<Buffer>,
    multi_buffer: Entity<MultiBuffer>,
    transcript: Transcript,
    prompt_end: text::Anchor,
    draft_end: text::Anchor,
    writer: Option<Writer>,
    rx: mpsc::Receiver<SocketEvent>,
    _poll_task: Task<()>,
    _subscriptions: Vec<Subscription>,
    session_id: tau_proto::SessionId,
    streamed_responses: HashMap<String, String>,
    live_response_ranges: HashMap<String, InsertedTranscript>,
    live_compaction_ranges: HashMap<String, InsertedTranscript>,
    cli_theme: tau_themes::Theme,
    pending_tool_calls: HashMap<String, InsertedTranscript>,
    current_model: Option<tau_proto::ModelId>,
    current_role: Option<String>,
    baseline_params: Option<ModelParams>,
    current_params: ModelParams,
    current_context_percent: Option<u8>,
    current_context_input_tokens: Option<u64>,
    current_context_window: Option<u64>,
    main_tool_activity: MainToolActivity,
    previous_provider_usage: Option<tau_proto::ProviderTokenUsage>,
    follow_tail: bool,
    agents: AgentState,
}

impl TauGui {
    fn new(attach_target: AttachTarget, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let transcript_buffer = cx.new(|cx| {
            let mut buffer = Buffer::local("", cx);
            buffer.set_capability(Capability::Read, cx);
            buffer
        });
        let prompt_buffer = cx.new(|cx| Buffer::local("", cx));
        let prompt_start = prompt_buffer.read(cx).anchor_before(0);
        let prompt_end = prompt_start;
        let draft_end = prompt_buffer.read(cx).anchor_after(0);
        let multi_buffer = cx.new(|cx| {
            let mut multi_buffer = MultiBuffer::without_headers(Capability::ReadWrite);
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(0),
                transcript_buffer.clone(),
                [Point::zero()..transcript_buffer.read(cx).max_point()],
                0,
                cx,
            );
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(1),
                prompt_buffer.clone(),
                [Point::zero()..prompt_buffer.read(cx).max_point()],
                0,
                cx,
            );
            multi_buffer
        });
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: true,
                    show_active_line_background: false,
                    sizing_behavior: SizingBehavior::ExcludeOverscrollMargin,
                },
                multi_buffer.clone(),
                None,
                window,
                cx,
            );
            editor.set_show_gutter(false, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_vertical_scrollbar(false, cx);
            editor.set_show_horizontal_scrollbar(false, cx);
            editor.set_offset_content(false, cx);
            editor.set_mouse_click_selection_enabled(false, cx);
            editor.set_soft_wrap_mode(language::language_settings::SoftWrap::EditorWidth, cx);
            editor.disable_header_for_buffer(transcript_buffer.read(cx).remote_id(), cx);
            editor.disable_header_for_buffer(prompt_buffer.read(cx).remote_id(), cx);
            editor.disable_expand_excerpt_buttons(cx);
            editor
        });
        let this = cx.entity().downgrade();
        let prompt_buffer_subscription = cx.subscribe(&prompt_buffer, |this, _, event, cx| {
            if matches!(event, BufferEvent::Edited { .. }) {
                this.update_prompt_inlay(cx);
            }
        });
        let submit_subscription = editor.update(cx, |editor, _cx| {
            let this = this.clone();
            editor.register_action(move |_: &SubmitPrompt, window, cx| {
                eprintln!("tau-gui: SubmitPrompt action dispatched to editor");
                if let Err(error) = this.update(cx, |this, cx| this.submit_prompt(window, cx)) {
                    eprintln!("tau-gui: failed to submit prompt: {error:#}");
                }
            })
        });
        editor.update(cx, |editor, cx| {
            let this = this.clone();
            editor.set_prepare_for_insert(
                Some(
                    move |editor: &mut Editor, window: &mut Window, cx: &mut Context<Editor>| {
                        use multi_buffer::ToOffset as _;

                        let selection_offset = editor
                            .selections
                            .newest_anchor()
                            .head()
                            .to_offset(editor.display_snapshot(cx).buffer_snapshot())
                            .0;
                        let target_anchor = match this.update(cx, |this, cx| {
                            this.prompt_insert_anchor_for_selection(selection_offset, cx)
                        }) {
                            Ok(target_anchor) => target_anchor,
                            Err(error) => {
                                eprintln!(
                                    "tau-gui: failed to prepare prompt for insert: {error:#}"
                                );
                                None
                            }
                        };
                        if let Some(target_anchor) = target_anchor {
                            editor.change_selections(
                                SelectionEffects::no_scroll(),
                                window,
                                cx,
                                |selections| {
                                    selections.select_anchor_ranges([target_anchor..target_anchor]);
                                },
                            );
                        }
                    },
                ),
                cx,
            );
        });
        window.focus(&editor.focus_handle(cx), cx);

        let (tx, rx) = mpsc::channel();
        let writer = match socket_client::spawn(attach_target.socket_path.clone(), tx) {
            Ok(writer) => Some(writer),
            Err(error) => {
                eprintln!("tau-gui: failed to connect to Tau harness: {error:#}");
                None
            }
        };

        let poll_task = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;
                if this
                    .update_in(cx, |this, window, cx| this.drain_socket_events(window, cx))
                    .is_err()
                {
                    break;
                }
            }
        });
        let transcript =
            Transcript::new(transcript_buffer, editor.clone(), multi_buffer.clone(), cx);
        let mut this = Self {
            editor,
            prompt_buffer,
            multi_buffer,
            transcript,
            prompt_end,
            draft_end,
            writer,
            rx,
            _poll_task: poll_task,
            _subscriptions: vec![submit_subscription, prompt_buffer_subscription],
            session_id: attach_target.session_id,
            streamed_responses: HashMap::default(),
            live_response_ranges: HashMap::default(),
            live_compaction_ranges: HashMap::default(),
            cli_theme: cli_theme::select_theme(tau_config::settings::CliTheme::Dark),
            pending_tool_calls: HashMap::default(),
            current_model: None,
            current_role: None,
            baseline_params: None,
            current_params: ModelParams::default(),
            current_context_percent: None,
            current_context_input_tokens: None,
            current_context_window: None,
            main_tool_activity: MainToolActivity::default(),
            previous_provider_usage: None,
            follow_tail: true,
            agents: AgentState::default(),
        };
        this.update_prompt_inlay(cx);
        this.update_status_line(cx);
        this.insert_before_draft_styled(
            "Tau GUI attached. Type a prompt and press Ctrl-Enter.\n\n",
            TranscriptStyle::SystemInfo,
            cx,
        );
        this.move_cursor_to_prompt_end(window, cx);
        this.scroll_to_tail(window, cx);
        this
    }

    fn drain_socket_events(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.refresh_follow_tail(cx);
        let should_follow_tail = self.follow_tail;
        while let Ok(event) = self.rx.try_recv() {
            match event {
                SocketEvent::Frame(frame) => self.handle_frame(frame, cx),
                SocketEvent::Disconnected(reason) => {
                    self.insert_before_draft_styled(
                        &format!("\n[disconnected: {reason}]\n"),
                        TranscriptStyle::SystemDisconnect,
                        cx,
                    );
                }
            }
        }
        if should_follow_tail {
            self.follow_tail = true;
            self.scroll_to_tail(window, cx);
        }
    }

    fn handle_frame(&mut self, frame: Frame, cx: &mut Context<Self>) {
        let (_log_id, frame) = frame.peel_log();
        match frame {
            Frame::Event(event) => self.handle_event(event, cx),
            Frame::Message(Message::Disconnect(disconnect)) => {
                self.insert_before_draft_styled(
                    &format!(
                        "\n[daemon disconnected: {}]\n",
                        disconnect.reason.unwrap_or_else(|| "no reason".to_owned())
                    ),
                    TranscriptStyle::SystemDisconnect,
                    cx,
                );
            }
            Frame::Message(_) => {}
        }
    }

    fn handle_event(&mut self, event: Event, cx: &mut Context<Self>) {
        let previous_agent_id = self.agents.current_agent_id_owned();
        self.learn_agent_metadata(&event);
        if self.agents.current_agent_id() != previous_agent_id.as_deref() {
            self.apply_selected_agent_context_usage();
            self.update_status_line(cx);
            self.update_prompt_inlay(cx);
        }
        match event {
            Event::UiPromptSubmitted(_) => {}
            Event::AgentPromptSubmitted(prompt)
                if prompt.originator.is_user() && !prompt.message_class.is_internal() =>
            {
                self.insert_before_draft_styled(
                    &format!("> {}\n", prompt.text),
                    TranscriptStyle::UserPrompt,
                    cx,
                );
            }
            Event::AgentPromptQueued(queued) if !queued.message_class.is_internal() => {
                self.insert_before_draft_styled(
                    &format!("> {} (queued)\n", queued.text),
                    TranscriptStyle::UserPromptQueued,
                    cx,
                );
            }
            Event::AgentUserMessageInjected(injected) if !injected.message_class.is_internal() => {
                self.insert_before_draft_styled(
                    &format!("> {} (injected)\n", injected.text),
                    TranscriptStyle::UserPromptQueued,
                    cx,
                );
            }
            Event::AgentMessageSent(message) => {
                self.insert_before_draft_styled(
                    &format!(
                        "{}:\n{}\n",
                        agent_message_sent_summary(&message),
                        message.message
                    ),
                    TranscriptStyle::SystemInfo,
                    cx,
                );
            }
            Event::AgentMessageReceived(message) => {
                self.insert_before_draft_styled(
                    &format!(
                        "Message from {} to {}:\n{}\n",
                        message.sender_id, message.recipient_id, message.message
                    ),
                    TranscriptStyle::SystemInfo,
                    cx,
                );
            }
            Event::ProviderResponseUpdated(update) if update.originator.is_user() => {
                let key = update.agent_prompt_id.to_string();
                self.update_live_compaction(
                    key.as_str(),
                    provider_update_compaction_status(&update),
                    cx,
                );
                let text = assistant_text_from_update(&update.items).unwrap_or_default();
                self.streamed_responses.insert(key.clone(), text.clone());
                self.upsert_live_response(key, text.as_str(), cx);
            }
            Event::ProviderResponseFinished(finished) if finished.originator.is_user() => {
                let key = finished.agent_prompt_id.to_string();
                self.remove_live_compaction(key.as_str(), cx);
                if let Some(text) = assistant_text(&finished.output_items) {
                    match self.streamed_responses.remove(&key) {
                        Some(_) | None => {
                            if !text.is_empty() {
                                self.finalize_live_response(key.as_str(), &text, cx);
                            }
                        }
                    }
                } else {
                    self.remove_live_response(key.as_str(), cx);
                }
                if finished.output_items.is_empty() {
                    let text = finished
                        .error
                        .as_deref()
                        .unwrap_or("(provider returned an empty response)");
                    self.insert_before_draft_styled(
                        &format!("{text}\n"),
                        TranscriptStyle::SystemImportant,
                        cx,
                    );
                    self.render_turn_stats(&finished, cx);
                    self.ensure_transcript_gap(cx);
                    return;
                }
                if let Some(error) = &finished.error {
                    self.insert_before_draft_styled(
                        &format!("[provider error: {error}]\n"),
                        TranscriptStyle::SystemImportant,
                        cx,
                    );
                }
                for item in &finished.output_items {
                    if matches!(item, ContextItem::Compaction(_)) {
                        let block = tool_render::render_compaction_block(
                            &self.cli_theme,
                            compaction_success_status(
                                finished.compaction_original_input_tokens,
                                finished.compaction_compacted_input_tokens,
                            ),
                            tool_render::CompactionStatus::Success,
                        );
                        self.insert_before_draft_block(block, cx);
                    }
                }
                let tool_calls = tool_calls_from_output_items(&finished.output_items);
                if !tool_calls.is_empty() {
                    self.main_tool_activity
                        .add_requested_tools(tool_calls.len());
                    self.update_status_line(cx);
                }
                for call in tool_calls {
                    let block = render_tool_call_block(&self.cli_theme, &call);
                    if let Some(inserted) = self.insert_before_draft_block(block, cx) {
                        self.pending_tool_calls
                            .insert(call.call_id.to_string(), inserted);
                    }
                }
                self.render_turn_stats(&finished, cx);
                self.ensure_transcript_gap(cx);
            }
            Event::AgentPromptRecalled(recalled) => {
                self.agents.select(recalled.agent_id.to_string());
                self.replace_draft_text(&recalled.text, cx);
                self.insert_before_draft_styled(
                    "> recalled queued prompt for editing\n",
                    TranscriptStyle::SystemInfo,
                    cx,
                );
            }
            Event::AgentPromptSteered(steered) if !steered.message_class.is_internal() => {
                self.insert_before_draft_styled(
                    &format!("> {} (steered)\n", steered.text),
                    TranscriptStyle::UserPromptQueued,
                    cx,
                );
            }
            Event::AgentCompactionTriggered(triggered) if triggered.originator.is_user() => {
                let block = tool_render::render_compaction_block(
                    &self.cli_theme,
                    format!("requested #{}", triggered.agent_id),
                    tool_render::CompactionStatus::Progress,
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::AgentPromptTerminated(terminated) if terminated.originator.is_user() => {
                let key = terminated.agent_prompt_id.to_string();
                self.remove_live_response(key.as_str(), cx);
                self.remove_live_compaction(key.as_str(), cx);
                self.insert_before_draft_styled(
                    &format!(
                        "[prompt {}: {key}]\n",
                        agent_prompt_termination_reason(terminated.reason)
                    ),
                    TranscriptStyle::SystemInfo,
                    cx,
                );
            }
            Event::ToolDelegateProgress(progress) => {
                if let Some(agent_id) = &progress.agent_id {
                    self.agents.mark_live(agent_id.clone());
                }
                let display = progress
                    .display
                    .clone()
                    .unwrap_or_else(|| tau_proto::ToolUseState {
                        args: progress.task_name.clone(),
                        status: tau_proto::ToolUseStatus::InProgress,
                        status_text: tau_proto::PROGRESS_INDICATOR_TEXT.to_owned(),
                        ..Default::default()
                    });
                self.upsert_delegate_display(
                    progress.call_id.as_str(),
                    &display,
                    progress.agent_id.as_deref(),
                    progress.role.as_deref(),
                    cx,
                );
                self.update_status_line(cx);
            }
            Event::ToolProgress(progress) => {
                if let Some(display) = progress.display.as_ref() {
                    self.upsert_tool_display(
                        progress.call_id.as_str(),
                        &progress.tool_name,
                        display,
                        cx,
                    );
                } else {
                    let text = tau_harness::format_tool_progress(&progress);
                    if !text.is_empty() {
                        self.insert_before_draft_styled(
                            &format!("{text}\n"),
                            TranscriptStyle::ToolProgress,
                            cx,
                        );
                    }
                }
            }
            Event::ToolResult(result) if result.originator.is_user() => {
                if result.kind == tau_proto::ToolResultKind::BackgroundPlaceholder {
                    self.record_main_tool_backgrounded(result.call_id.as_str());
                } else {
                    let block = render_tool_result_block(&self.cli_theme, &result);
                    self.finish_tool_call(result.call_id.as_str(), block, cx);
                    self.record_main_tool_completed(result.call_id.as_str());
                }
                self.update_status_line(cx);
            }
            Event::ProviderToolResult(result)
                if result.originator.is_user()
                    || self
                        .pending_tool_calls
                        .contains_key(result.call_id.as_str()) =>
            {
                if result.kind == tau_proto::ToolResultKind::BackgroundPlaceholder {
                    self.record_main_tool_backgrounded(result.call_id.as_str());
                } else {
                    let block = render_tool_result_parts_block(
                        &self.cli_theme,
                        &result.tool_name,
                        &result.result,
                        result.display.as_ref(),
                    );
                    self.finish_tool_call(result.call_id.as_str(), block, cx);
                    self.record_main_tool_completed(result.call_id.as_str());
                }
                self.update_status_line(cx);
            }
            Event::ToolBackgroundResult(result)
                if result.originator.is_user()
                    || self
                        .pending_tool_calls
                        .contains_key(result.call_id.as_str())
                    || self
                        .main_tool_activity
                        .is_backgrounded(result.call_id.as_str()) =>
            {
                let block = render_tool_result_parts_block(
                    &self.cli_theme,
                    &result.tool_name,
                    &result.result,
                    result.display.as_ref(),
                );
                self.finish_tool_call(result.call_id.as_str(), block, cx);
                self.record_main_tool_completed(result.call_id.as_str());
                self.update_status_line(cx);
            }
            Event::ToolError(error) if error.originator.is_user() => {
                let block = render_tool_error_block(&self.cli_theme, &error);
                self.finish_tool_call(error.call_id.as_str(), block, cx);
                self.record_main_tool_completed(error.call_id.as_str());
                self.update_status_line(cx);
            }
            Event::ProviderToolError(error)
                if error.originator.is_user()
                    || self.pending_tool_calls.contains_key(error.call_id.as_str()) =>
            {
                let block = render_tool_error_parts_block(
                    &self.cli_theme,
                    &error.tool_name,
                    &error.message,
                    error.display.as_ref(),
                );
                self.finish_tool_call(error.call_id.as_str(), block, cx);
                self.record_main_tool_completed(error.call_id.as_str());
                self.update_status_line(cx);
            }
            Event::ToolBackgroundError(error)
                if error.originator.is_user()
                    || self.pending_tool_calls.contains_key(error.call_id.as_str())
                    || self
                        .main_tool_activity
                        .is_backgrounded(error.call_id.as_str()) =>
            {
                let block = render_tool_error_parts_block(
                    &self.cli_theme,
                    &error.tool_name,
                    &error.message,
                    error.display.as_ref(),
                );
                self.finish_tool_call(error.call_id.as_str(), block, cx);
                self.record_main_tool_completed(error.call_id.as_str());
                self.update_status_line(cx);
            }
            Event::ToolRejected(rejected) if rejected.originator.is_user() => {
                let block = render_tool_error_parts_block(
                    &self.cli_theme,
                    &rejected.tool_name,
                    &rejected.message,
                    None,
                );
                self.finish_tool_call(rejected.call_id.as_str(), block, cx);
                self.record_main_tool_completed(rejected.call_id.as_str());
                self.update_status_line(cx);
            }
            Event::ToolCancelled(cancelled) => {
                if self
                    .pending_tool_calls
                    .contains_key(cancelled.call_id.as_str())
                    || self
                        .main_tool_activity
                        .is_backgrounded(cancelled.call_id.as_str())
                {
                    let block = render_tool_error_parts_block(
                        &self.cli_theme,
                        &cancelled.tool_name,
                        "cancelled",
                        None,
                    );
                    self.finish_tool_call(cancelled.call_id.as_str(), block, cx);
                    self.record_main_tool_completed(cancelled.call_id.as_str());
                    self.update_status_line(cx);
                }
            }
            Event::UiShellCommand(command) => {
                let block = tool_render::render_shell_block(
                    &self.cli_theme,
                    &command.command,
                    "",
                    Some("running"),
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::ShellCommandProgress(progress) => {
                if !progress.chunk.is_empty() {
                    self.insert_before_draft_styled(
                        &progress.chunk,
                        TranscriptStyle::ToolProgress,
                        cx,
                    );
                }
            }
            Event::ShellCommandFinished(finished) => {
                let status = if finished.cancelled {
                    "cancelled".to_owned()
                } else {
                    format!("[{}]", finished.exit_code.unwrap_or(-1))
                };
                let block = tool_render::render_shell_block(
                    &self.cli_theme,
                    &finished.command,
                    &finished.output,
                    Some(status.as_str()),
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::ActionResult(result) => {
                let text = match result.output {
                    tau_proto::ActionOutput::Text { text } => text,
                    tau_proto::ActionOutput::EditorBuffer { title, text, .. } => {
                        format!("{title}\n{text}")
                    }
                };
                let block = tool_render::render_action_output_block(&self.cli_theme, &text);
                self.insert_before_draft_block(block, cx);
            }
            Event::ActionError(error) => {
                let block = tool_render::render_action_error_block(
                    &self.cli_theme,
                    &error.action_id,
                    &error.message,
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::ExtensionStarting(starting) => {
                let status = starting.pid.map_or_else(
                    || "starting".to_owned(),
                    |pid| format!("starting pid {pid}"),
                );
                let block = tool_render::extension_status_block(
                    &self.cli_theme,
                    &starting.extension_name,
                    &status,
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::ExtensionReady(ready) => {
                let status = ready
                    .pid
                    .map_or_else(|| "ready".to_owned(), |pid| format!("ready pid {pid}"));
                let block = tool_render::extension_status_block(
                    &self.cli_theme,
                    &ready.extension_name,
                    &status,
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::ExtensionExited(exited) => {
                let status = match (exited.exit_code, exited.signal) {
                    (Some(code), _) => format!("exited {code}"),
                    (_, Some(signal)) => format!("signal {signal}"),
                    (None, None) => "exited".to_owned(),
                };
                let block = tool_render::extension_status_block(
                    &self.cli_theme,
                    &exited.extension_name,
                    &status,
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::ExtensionRestarting(restarting) => {
                let status = restarting.reason.as_ref().map_or_else(
                    || format!("restarting #{}", restarting.attempt),
                    |reason| format!("restarting #{}: {reason}", restarting.attempt),
                );
                let block = tool_render::extension_status_block(
                    &self.cli_theme,
                    &restarting.extension_name,
                    &status,
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::ExtAgentsMdAvailable(agents_md) => {
                let block = tool_render::system_loaded_block(
                    &self.cli_theme,
                    &agents_md.file_path,
                    &agents_md.content,
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::ExtensionContextReady(ready) => {
                let block =
                    tool_render::agent_context_ready_block(&self.cli_theme, &ready.agent_id);
                self.insert_before_draft_block(block, cx);
            }
            Event::HarnessSessionDir(session_dir) => {
                let block = tool_render::session_status_block(
                    &self.cli_theme,
                    &session_dir.path,
                    "/",
                    session_dir.status.as_str(),
                );
                self.insert_before_draft_block(block, cx);
            }
            Event::HarnessUiDir(ui_dir) => {
                let block = tool_render::ui_dir_block(&self.cli_theme, &ui_dir.path);
                self.insert_before_draft_block(block, cx);
            }
            Event::HarnessInfo(info) => {
                let block = tool_render::render_harness_info(&self.cli_theme, &info);
                self.insert_before_draft_block(block, cx);
            }
            Event::HarnessRoleSelected(selected) => {
                self.current_model = selected.model.clone();
                self.current_role = Some(selected.role);
                self.baseline_params = selected.baseline_params;
                self.current_params = selected.model_params;
                self.current_context_window = selected.context_window;
                self.update_status_line(cx);
                self.update_prompt_inlay(cx);
            }
            Event::HarnessContextUsageChanged(changed) => {
                self.current_context_input_tokens = changed.input_tokens;
                self.current_context_percent = changed.percent_used;
                self.update_status_line(cx);
            }
            Event::HarnessAgentContextUsageChanged(changed) => {
                let agent_id = changed.agent_id.to_string();
                self.agents.record_context_usage(
                    agent_id.clone(),
                    AgentContextUsage {
                        input_tokens: changed.input_tokens,
                        percent_used: changed.percent_used,
                        context_window: changed.context_window,
                    },
                );
                if self.agents.current_agent_id() == Some(agent_id.as_str()) {
                    self.apply_selected_agent_context_usage();
                    self.update_status_line(cx);
                }
            }
            Event::SessionStarted(started) => {
                self.session_id = started.session_id;
                self.main_tool_activity.reset();
                self.previous_provider_usage = None;
                self.agents.clear_context_usage();
                self.update_status_line(cx);
            }
            _ => {}
        }
    }

    fn learn_agent_metadata(&mut self, event: &Event) {
        match event {
            Event::AgentStarted(started) => self.agents.remember(started.agent_id.to_string()),
            Event::SessionAgentLoaded(loaded) => self.agents.remember(loaded.agent_id.to_string()),
            Event::SessionAgentUnloaded(unloaded) => {
                self.agents.unload(unloaded.agent_id.as_str());
            }
            Event::UiPromptSubmitted(prompt) if prompt.originator.is_user() => {
                self.agents.select(prompt.agent_id.to_string());
            }
            Event::AgentPromptSubmitted(prompt)
                if prompt.originator.is_user() && !prompt.message_class.is_internal() =>
            {
                self.agents.select(prompt.agent_id.to_string());
            }
            Event::AgentPromptQueued(queued) if !queued.message_class.is_internal() => {
                self.agents.select(queued.agent_id.to_string());
            }
            Event::AgentUserMessageInjected(injected) if !injected.message_class.is_internal() => {
                self.agents.remember(injected.agent_id.to_string());
            }
            Event::AgentMessageSent(message) => {
                self.agents.remember(message.sender_id.to_string());
                if let Some(agent_id) = agent_message_sent_recipient_agent_id(message) {
                    self.agents.remember(agent_id.to_owned());
                }
            }
            Event::AgentMessageReceived(message) => {
                self.agents.remember(message.sender_id.to_string());
                self.agents.remember(message.recipient_id.to_string());
            }
            Event::ToolDelegateProgress(progress) => {
                if let Some(agent_id) = &progress.agent_id {
                    self.agents.mark_live(agent_id.clone());
                }
            }
            Event::AgentPromptCreated(created) if created.originator.is_user() => {
                self.agents.select(created.agent_id.to_string());
            }
            Event::ProviderResponseFinished(finished) if finished.originator.is_user() => {
                self.agents.select(finished.agent_id.to_string());
            }
            _ => {}
        }
    }

    fn apply_selected_agent_context_usage(&mut self) {
        if let Some(usage) = self.agents.selected_context_usage() {
            self.current_context_input_tokens = usage.input_tokens;
            self.current_context_percent = usage.percent_used;
            self.current_context_window = usage.context_window;
        } else {
            self.current_context_input_tokens = None;
            self.current_context_percent = None;
        }
    }

    fn selected_agent_is_active(&self) -> bool {
        self.agents.selected_is_active()
    }

    fn selected_agent_proto_id(&self) -> Option<tau_proto::AgentId> {
        self.agents.current_agent_id_owned().map(Into::into)
    }

    fn send_event(&mut self, event: Event, cx: &mut Context<Self>) -> bool {
        let Some(writer) = &self.writer else {
            eprintln!("tau-gui: command ignored because socket writer is unavailable");
            return false;
        };
        let frame = Frame::Event(event);
        if let Err(error) = socket_client::send_frame(writer, &frame) {
            eprintln!("tau-gui: send failed: {error:#}");
            self.insert_before_draft_styled(
                &format!("\n[send failed: {error}]\n"),
                TranscriptStyle::SystemDisconnect,
                cx,
            );
            return false;
        }
        true
    }

    fn send_command_event(&mut self, event: Event, cx: &mut Context<Self>) -> bool {
        self.send_event(event, cx);
        true
    }

    fn handle_prompt_command(&mut self, text: &str, cx: &mut Context<Self>) -> bool {
        if text == "/cancel" {
            return self.send_command_event(
                Event::UiCancelPrompt(tau_proto::UiCancelPrompt {
                    session_id: self.session_id.clone(),
                    target_agent_id: self.selected_agent_proto_id(),
                    agent_prompt_id: None,
                }),
                cx,
            );
        }
        if text == "/tree" {
            return self.send_command_event(
                Event::UiTreeRequest(tau_proto::UiTreeRequest {
                    session_id: self.session_id.clone(),
                    target_agent_id: self.selected_agent_proto_id(),
                }),
                cx,
            );
        }
        if let Some(node_id) = text.strip_prefix("/tree ") {
            let Ok(node_id) = node_id.trim().parse::<u64>() else {
                self.insert_before_draft_styled(
                    "/tree <id>: id must be a non-negative integer\n",
                    TranscriptStyle::SystemInfo,
                    cx,
                );
                return true;
            };
            return self.send_command_event(
                Event::UiNavigateTree(tau_proto::UiNavigateTree {
                    session_id: self.session_id.clone(),
                    target_agent_id: self.selected_agent_proto_id(),
                    node_id,
                }),
                cx,
            );
        }
        if text == "/compact" {
            return self.send_command_event(
                Event::UiCompactRequest(tau_proto::UiCompactRequest {
                    session_id: self.session_id.clone(),
                    target_agent_id: self.selected_agent_proto_id(),
                }),
                cx,
            );
        }
        if text.starts_with("/compact ") {
            self.insert_before_draft_styled(
                "/compact forces a compaction pass and takes no arguments\n",
                TranscriptStyle::SystemInfo,
                cx,
            );
            return true;
        }
        if text == "/new" {
            self.clear_selected_agent(cx);
            return true;
        }
        if text == "/agent" || text.starts_with("/agent ") {
            self.handle_agent_command(text, cx);
            return true;
        }
        if let Some(role) = text.strip_prefix("/model ") {
            let role = role.trim();
            if !role.is_empty() {
                return self.select_role(role, cx);
            }
            return true;
        }
        if text == "/model" {
            self.insert_before_draft_styled("/model <role>\n", TranscriptStyle::SystemInfo, cx);
            return true;
        }
        if text == "/role" || text.starts_with("/role ") {
            self.handle_role_command(text, cx);
            return true;
        }
        if let Some(command) = text.strip_prefix("!!") {
            return self.send_shell_command(command, false, cx);
        }
        if let Some(command) = text.strip_prefix('!') {
            return self.send_shell_command(command, true, cx);
        }
        false
    }

    fn handle_role_command(&mut self, text: &str, cx: &mut Context<Self>) {
        let rest = text.strip_prefix("/role").unwrap_or("").trim();
        let mut parts = rest.split_whitespace();
        let role = parts.next();
        let command = parts.next();
        let value = parts.next();
        let extra = parts.next();
        let Some(role) = role else {
            self.insert_before_draft_styled(
                "/role <role> [delete|model|effort|verbosity|thinking-summary|service-tier|compaction-threshold|tools|enable-tools|disable-tools] [value]\n",
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        };
        let Some(command) = command else {
            self.select_role(role, cx);
            return;
        };
        if command == "delete" {
            if value.is_some() {
                self.insert_before_draft_styled(
                    "/role <role> delete takes no value\n",
                    TranscriptStyle::SystemInfo,
                    cx,
                );
                return;
            }
            self.send_command_event(
                Event::UiRoleUpdate(tau_proto::UiRoleUpdate {
                    role: role.to_owned(),
                    action: tau_proto::UiRoleUpdateAction::Delete,
                }),
                cx,
            );
            return;
        }
        let Some(value) = value else {
            self.insert_before_draft_styled(
                "/role <role> <setting> <value>\n",
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        };
        if extra.is_some() {
            self.insert_before_draft_styled(
                "/role: too many arguments\n",
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        }
        let action = match parse_role_setting_update(command, value) {
            Ok(action) => action,
            Err(error) => {
                self.insert_before_draft_styled(
                    &format!("/role: {error}\n"),
                    TranscriptStyle::SystemInfo,
                    cx,
                );
                return;
            }
        };
        self.send_command_event(
            Event::UiRoleUpdate(tau_proto::UiRoleUpdate {
                role: role.to_owned(),
                action,
            }),
            cx,
        );
    }

    fn select_role(&mut self, role: &str, cx: &mut Context<Self>) -> bool {
        self.send_command_event(
            Event::UiRoleSelect(tau_proto::UiRoleSelect {
                role: role.to_owned(),
            }),
            cx,
        )
    }

    fn handle_agent_command(&mut self, text: &str, cx: &mut Context<Self>) {
        let rest = text.strip_prefix("/agent").unwrap_or("").trim();
        if rest.is_empty() {
            let current = self.agents.current_agent_id().unwrap_or("none");
            let known_agents = self.agents.known_agents_sorted();
            let active_count = self.agents.active_count();
            self.insert_before_draft_styled(
                &format!(
                    "/agent <new|switch|suspend|resume> [agent_id]; current: {current}; active: {active_count}; known: {}\n",
                    known_agents.join(", ")
                ),
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        }

        let mut parts = rest.split_whitespace();
        let Some(subcommand) = parts.next() else {
            return;
        };
        let target = parts.next();
        if parts.next().is_some() {
            self.insert_before_draft_styled(
                "/agent: too many arguments (use /agent <new|switch|suspend|resume> [agent_id])\n",
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        }
        match subcommand {
            "new" => {
                if target.is_some() {
                    self.insert_before_draft_styled(
                        "/agent new\n",
                        TranscriptStyle::SystemInfo,
                        cx,
                    );
                } else {
                    self.clear_selected_agent(cx);
                }
            }
            "switch" => self.switch_agent(target, cx),
            "suspend" => self.suspend_agent(target, cx),
            "resume" => self.resume_agent(target, cx),
            _ => self.insert_before_draft_styled(
                "/agent <new|switch|suspend|resume> [agent_id]; use /agent switch <agent_id>\n",
                TranscriptStyle::SystemInfo,
                cx,
            ),
        }
    }

    fn clear_selected_agent(&mut self, cx: &mut Context<Self>) {
        self.agents.clear_current_agent();
        self.update_status_line(cx);
        self.update_prompt_inlay(cx);
    }

    fn switch_agent(&mut self, target: Option<&str>, cx: &mut Context<Self>) {
        let Some(agent_id) = target
            .map(str::trim)
            .filter(|agent_id| !agent_id.is_empty())
        else {
            self.insert_before_draft_styled(
                "/agent switch <agent_id|none>\n",
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        };
        if agent_id == "none" {
            self.clear_selected_agent(cx);
            return;
        }
        if !self.agents.known(agent_id) {
            self.insert_before_draft_styled(
                &format!("unknown agent: {agent_id}\n"),
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        }
        if self.agents.suspended(agent_id) {
            self.insert_before_draft_styled(
                &format!("agent is suspended: {agent_id} (use /agent resume {agent_id})\n"),
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        }
        self.agents.select(agent_id.to_owned());
        self.apply_selected_agent_context_usage();
        self.update_status_line(cx);
        self.update_prompt_inlay(cx);
    }

    fn suspend_agent(&mut self, target: Option<&str>, cx: &mut Context<Self>) {
        let target = target
            .map(str::trim)
            .filter(|target| !target.is_empty())
            .map(ToOwned::to_owned)
            .or_else(|| self.agents.current_agent_id_owned());
        let Some(agent_id) = target else {
            self.insert_before_draft_styled(
                "/agent suspend <agent_id>\n",
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        };
        if !self.agents.known(&agent_id) {
            self.insert_before_draft_styled(
                &format!("unknown agent: {agent_id}\n"),
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        }
        self.agents.suspend(agent_id);
        self.update_status_line(cx);
    }

    fn resume_agent(&mut self, target: Option<&str>, cx: &mut Context<Self>) {
        let target = target
            .map(str::trim)
            .filter(|target| !target.is_empty())
            .map(ToOwned::to_owned)
            .or_else(|| {
                self.agents
                    .current_agent_id_owned()
                    .filter(|agent_id| self.agents.suspended(agent_id))
            });
        let Some(agent_id) = target else {
            self.insert_before_draft_styled(
                "/agent resume <agent_id>\n",
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        };
        if !self.agents.known(&agent_id) {
            self.insert_before_draft_styled(
                &format!("unknown agent: {agent_id}\n"),
                TranscriptStyle::SystemInfo,
                cx,
            );
            return;
        }
        self.agents.resume(agent_id);
        self.apply_selected_agent_context_usage();
        self.update_status_line(cx);
        self.update_prompt_inlay(cx);
    }

    fn send_shell_command(
        &mut self,
        command: &str,
        include_in_context: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        let command = command.trim();
        if command.is_empty() {
            return true;
        }
        let command_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| format!("ui-sh-{}", duration.as_nanos()))
            .unwrap_or_else(|_| "ui-sh-0".to_owned());
        self.send_command_event(
            Event::UiShellCommand(tau_proto::UiShellCommand {
                session_id: self.session_id.clone(),
                command_id: command_id.into(),
                command: command.to_owned(),
                include_in_context,
                target_agent_id: self.selected_agent_proto_id(),
            }),
            cx,
        )
    }

    fn clear_prompt_draft(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.prompt_buffer.update(cx, |buffer, cx| {
            let start = self.prompt_end.to_offset(buffer);
            let end = self.draft_end.to_offset(buffer);
            buffer.edit([(start..end, "")], None, cx);
            self.prompt_end = buffer.anchor_before(start);
            self.draft_end = buffer.anchor_after(start);
        });
        self.move_cursor_to_prompt_end(window, cx);
        self.follow_tail = true;
        self.scroll_to_tail(window, cx);
        cx.notify();
    }

    fn submit_prompt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        eprintln!("tau-gui: submit_prompt called");
        let buffer = self.prompt_buffer.read(cx);
        let prompt_end = self.prompt_end.to_offset(buffer);
        let draft_end = self.draft_end.to_offset(buffer);
        let text = buffer
            .text_for_range(prompt_end..draft_end)
            .collect::<String>();
        let text = text.trim().to_owned();
        if text.is_empty() {
            eprintln!("tau-gui: submit ignored because draft is empty");
            return;
        }
        eprintln!("tau-gui: submitting prompt with {} bytes", text.len());

        if self.handle_prompt_command(&text, cx) {
            self.clear_prompt_draft(window, cx);
            return;
        }

        if !self.selected_agent_is_active() {
            self.insert_before_draft_styled(
                "selected agent is suspended; choose a different agent or start a new one\n",
                TranscriptStyle::SystemImportant,
                cx,
            );
            return;
        }

        let event = if let Some(agent_id) = self.agents.current_agent_id_owned() {
            Event::UiPromptSubmitted(UiPromptSubmitted {
                session_id: self.session_id.clone(),
                text: text.clone(),
                agent_id: agent_id.into(),
                message_class: PromptMessageClass::User,
                originator: PromptOriginator::User,
                ctx_id: None,
            })
        } else {
            Event::UiCreateAgent(tau_proto::UiCreateAgent {
                session_id: self.session_id.clone(),
                role: self
                    .current_role
                    .clone()
                    .unwrap_or_else(|| "engineer".to_owned()),
                cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
                initial_prompt: Some(text.clone()),
                message_class: PromptMessageClass::User,
                originator: PromptOriginator::User,
                ctx_id: None,
            })
        };
        if !self.send_event(event, cx) {
            return;
        }
        eprintln!("tau-gui: prompt frame sent");

        self.clear_prompt_draft(window, cx);
    }

    fn move_cursor_to_prompt_end(&self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(anchor) = self.anchor_in_excerpt(self.draft_end, cx) else {
            eprintln!("tau-gui: failed to map draft end anchor into editor excerpt");
            return;
        };
        self.select_anchor(anchor, window, cx);
    }

    fn prompt_insert_anchor_for_selection(
        &self,
        selection_offset: usize,
        cx: &mut Context<Self>,
    ) -> Option<multi_buffer::Anchor> {
        use multi_buffer::ToOffset as _;

        let snapshot = self.multi_buffer.read(cx).snapshot(cx);
        let prompt_start = snapshot.anchor_in_excerpt(self.prompt_end)?;
        let draft_end = snapshot.anchor_in_excerpt(self.draft_end)?;
        let prompt_start = prompt_start.to_offset(&snapshot);
        let draft_end_offset = draft_end.to_offset(&snapshot);
        selection_outside_prompt(selection_offset, prompt_start.0, draft_end_offset.0)
            .then_some(draft_end)
    }

    fn select_anchor(
        &self,
        anchor: multi_buffer::Anchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_anchor_ranges([anchor..anchor]);
            });
        });
    }

    fn update_prompt_inlay(&self, cx: &mut Context<Self>) {
        let to_insert = if self.draft_is_empty(cx) {
            let Some(anchor) = self.anchor_in_excerpt(self.draft_end, cx) else {
                eprintln!("tau-gui: failed to map prompt placeholder anchor into editor excerpt");
                return;
            };
            vec![Inlay::custom(
                PROMPT_PLACEHOLDER_INLAY_ID,
                anchor,
                self.prompt_placeholder_text(),
            )]
        } else {
            Vec::new()
        };
        self.editor.update(cx, |editor, cx| {
            editor.splice_inlays(
                &[InlayId::Custom(PROMPT_PLACEHOLDER_INLAY_ID)],
                to_insert,
                cx,
            );
        });
    }

    fn prompt_placeholder_text(&self) -> String {
        if let Some(agent_id) = self.agents.current_agent_id() {
            return format!("Write a message to {agent_id}…");
        }

        format!(
            "Start new {} agent…",
            self.current_role.as_deref().unwrap_or("Tau")
        )
    }

    fn draft_is_empty(&self, cx: &mut Context<Self>) -> bool {
        let buffer = self.prompt_buffer.read(cx);
        self.prompt_end.to_offset(buffer) == self.draft_end.to_offset(buffer)
    }

    fn anchor_in_excerpt(
        &self,
        anchor: text::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<multi_buffer::Anchor> {
        self.multi_buffer
            .read(cx)
            .snapshot(cx)
            .anchor_in_excerpt(anchor)
    }

    fn refresh_follow_tail(&mut self, cx: &mut Context<Self>) {
        self.follow_tail = self.is_tail_visible(cx);
    }

    fn is_tail_visible(&self, cx: &mut Context<Self>) -> bool {
        self.editor.update(cx, |editor, cx| {
            let Some(visible_lines) = editor.visible_line_count() else {
                return true;
            };
            let snapshot = editor.display_snapshot(cx);
            let scroll_top = editor.scroll_position(cx).y;
            let max_scroll_top = (snapshot.max_point().row().as_f64() - visible_lines + 1.).max(0.);
            scroll_top >= max_scroll_top - 1.
        })
    }

    fn scroll_to_tail(&self, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(anchor) = self.anchor_in_excerpt(self.draft_end, cx) else {
            return;
        };
        self.editor.update(cx, |editor, cx| {
            editor.request_autoscroll(Autoscroll::bottom().for_anchor(anchor), cx);
        });
    }

    fn insert_before_draft_styled(
        &mut self,
        text: &str,
        style: TranscriptStyle,
        cx: &mut Context<Self>,
    ) {
        let style = self.highlight_style(style, cx);
        self.insert_before_draft_highlighted(text, style, cx);
    }

    fn insert_before_draft_highlighted(
        &mut self,
        text: &str,
        style: HighlightStyle,
        cx: &mut Context<Self>,
    ) -> Option<InsertedTranscript> {
        self.insert_before_draft_spans([(text, style)], cx)
    }

    fn upsert_live_response(&mut self, key: String, text: &str, cx: &mut Context<Self>) {
        let block = tool_render::streaming_block(
            &self.cli_theme,
            tau_themes::names::AGENT_RESPONSE,
            text.to_owned(),
        );
        let spans = block_spans(&block, cx)
            .into_iter()
            .map(|(text, style)| (text.to_owned(), style))
            .collect::<Vec<_>>();
        if let Some(inserted) = self.live_response_ranges.remove(&key) {
            self.remove_transcript_highlights(inserted.highlight_keys);
            if let Some(inserted) = self.replace_transcript_range_with_spans(
                inserted.range,
                spans.iter().map(|(text, style)| (text.as_str(), *style)),
                cx,
            ) {
                self.live_response_ranges.insert(key, inserted);
            }
        } else if let Some(inserted) = self.insert_before_draft_spans(
            spans.iter().map(|(text, style)| (text.as_str(), *style)),
            cx,
        ) {
            self.live_response_ranges.insert(key, inserted);
        }
    }

    fn finalize_live_response(&mut self, key: &str, text: &str, cx: &mut Context<Self>) {
        let style = self.highlight_style(TranscriptStyle::AgentResponse, cx);
        if let Some(inserted) = self.live_response_ranges.remove(key) {
            self.remove_transcript_highlights(inserted.highlight_keys);
            self.replace_transcript_range_with_spans(inserted.range, [(text, style)], cx);
        } else {
            self.insert_before_draft_styled(text, TranscriptStyle::AgentResponse, cx);
        }
    }

    fn update_live_compaction(
        &mut self,
        key: &str,
        status: Option<(tool_render::CompactionStatus, String)>,
        cx: &mut Context<Self>,
    ) {
        let Some((status, text)) = status else {
            self.remove_live_compaction(key, cx);
            return;
        };
        let block = tool_render::render_compaction_block(&self.cli_theme, text, status);
        if let Some(inserted) = self.live_compaction_ranges.remove(key) {
            if let Some(inserted) = self.replace_transcript_block(inserted, block, cx) {
                self.live_compaction_ranges.insert(key.to_owned(), inserted);
            }
        } else if let Some(inserted) = self.insert_before_draft_block(block, cx) {
            self.live_compaction_ranges.insert(key.to_owned(), inserted);
        }
    }

    fn remove_live_compaction(&mut self, key: &str, cx: &mut Context<Self>) {
        if let Some(inserted) = self.live_compaction_ranges.remove(key) {
            self.remove_transcript_highlights(inserted.highlight_keys);
            self.remove_transcript_range(inserted.range, cx);
        }
    }

    fn remove_live_response(&mut self, key: &str, cx: &mut Context<Self>) {
        self.streamed_responses.remove(key);
        if let Some(inserted) = self.live_response_ranges.remove(key) {
            self.remove_transcript_highlights(inserted.highlight_keys);
            self.remove_transcript_range(inserted.range, cx);
        }
    }

    fn upsert_delegate_display(
        &mut self,
        call_id: &str,
        display: &tau_proto::ToolUseState,
        agent_id: Option<&str>,
        role: Option<&str>,
        cx: &mut Context<Self>,
    ) {
        let display = tool_render::render_delegate_display(display, agent_id, role);
        let block = tool_render::render_tool_block(&self.cli_theme, &display);
        if let Some(inserted) = self.pending_tool_calls.remove(call_id) {
            if let Some(inserted) = self.replace_transcript_block(inserted, block, cx) {
                self.pending_tool_calls.insert(call_id.to_owned(), inserted);
            }
        } else if let Some(inserted) = self.insert_before_draft_block(block, cx) {
            self.pending_tool_calls.insert(call_id.to_owned(), inserted);
        }
    }

    fn upsert_tool_display(
        &mut self,
        call_id: &str,
        tool_name: &str,
        display: &tau_proto::ToolUseState,
        cx: &mut Context<Self>,
    ) {
        let display = tool_render::render_tool_use_state(tool_name, display);
        let block = tool_render::render_tool_block(&self.cli_theme, &display);
        if let Some(inserted) = self.pending_tool_calls.remove(call_id) {
            if let Some(inserted) = self.replace_transcript_block(inserted, block, cx) {
                self.pending_tool_calls.insert(call_id.to_owned(), inserted);
            }
        } else if let Some(inserted) = self.insert_before_draft_block(block, cx) {
            self.pending_tool_calls.insert(call_id.to_owned(), inserted);
        }
    }

    fn replace_draft_text(&mut self, text: &str, cx: &mut Context<Self>) {
        self.prompt_buffer.update(cx, |buffer, cx| {
            let start = self.prompt_end.to_offset(buffer);
            let end = self.draft_end.to_offset(buffer);
            buffer.edit([(start..end, text)], None, cx);
            self.prompt_end = buffer.anchor_before(start);
            self.draft_end = buffer.anchor_after(start + text.len());
        });
        self.update_prompt_inlay(cx);
        cx.notify();
    }

    fn remove_transcript_range(
        &mut self,
        range: std::ops::Range<text::Anchor>,
        cx: &mut Context<Self>,
    ) {
        self.transcript.remove_range(range, cx);
        cx.notify();
    }

    fn remove_transcript_highlights(&mut self, highlight_keys: Vec<usize>) {
        self.transcript.remove_highlights(highlight_keys);
    }

    fn render_turn_stats(
        &mut self,
        finished: &tau_proto::ProviderResponseFinished,
        cx: &mut Context<Self>,
    ) {
        let Some(usage) = finished.usage.as_ref() else {
            return;
        };
        let block = tool_render::render_turn_stats_block(
            &self.cli_theme,
            usage,
            self.previous_provider_usage.as_ref(),
            None,
            None,
        );
        self.insert_before_draft_block(block, cx);
        self.previous_provider_usage = Some(usage.clone());
    }

    fn finish_tool_call(
        &mut self,
        call_id: &str,
        block: tau_cli_term::StyledBlock,
        cx: &mut Context<Self>,
    ) {
        if let Some(inserted) = self.pending_tool_calls.remove(call_id) {
            self.replace_transcript_block(inserted, block, cx);
        } else {
            self.insert_before_draft_block(block, cx);
        }
    }

    fn insert_before_draft_block(
        &mut self,
        block: tau_cli_term::StyledBlock,
        cx: &mut Context<Self>,
    ) -> Option<InsertedTranscript> {
        let mut spans = Vec::new();
        if self.transcript_trailing_newlines(cx) == 0 {
            spans.push(("\n", HighlightStyle::default()));
        }
        spans.extend(block_spans(&block, cx));
        if !spans.last().is_some_and(|(text, _)| text.ends_with('\n')) {
            spans.push(("\n", HighlightStyle::default()));
        }
        self.insert_before_draft_spans(spans, cx)
    }

    fn replace_transcript_block(
        &mut self,
        inserted: InsertedTranscript,
        block: tau_cli_term::StyledBlock,
        cx: &mut Context<Self>,
    ) -> Option<InsertedTranscript> {
        self.remove_transcript_highlights(inserted.highlight_keys);

        let mut spans = Vec::new();
        let starts_with_newline = self.transcript.range_starts_with(&inserted.range, '\n', cx);
        if starts_with_newline {
            spans.push(("\n", HighlightStyle::default()));
        }
        spans.extend(block_spans(&block, cx));
        if !spans.last().is_some_and(|(text, _)| text.ends_with('\n')) {
            spans.push(("\n", HighlightStyle::default()));
        }
        self.replace_transcript_range_with_spans(inserted.range, spans, cx)
    }

    fn ensure_transcript_gap(&mut self, cx: &mut Context<Self>) {
        if self.transcript_trailing_newlines(cx) == 0 {
            self.insert_before_draft_highlighted("\n", HighlightStyle::default(), cx);
        }
    }

    fn transcript_trailing_newlines(&self, cx: &mut Context<Self>) -> usize {
        self.transcript.trailing_newlines(cx)
    }

    fn insert_before_draft_spans<'a>(
        &mut self,
        spans: impl IntoIterator<Item = (&'a str, HighlightStyle)>,
        cx: &mut Context<Self>,
    ) -> Option<InsertedTranscript> {
        let inserted = self.transcript.insert_spans(spans, cx);
        cx.notify();
        inserted
    }

    fn replace_transcript_range_with_spans<'a>(
        &mut self,
        range: std::ops::Range<text::Anchor>,
        spans: impl IntoIterator<Item = (&'a str, HighlightStyle)>,
        cx: &mut Context<Self>,
    ) -> Option<InsertedTranscript> {
        let inserted = self.transcript.replace_range_with_spans(range, spans, cx);
        cx.notify();
        inserted
    }

    fn update_status_line(&mut self, cx: &mut Context<Self>) {
        cx.notify();
    }

    fn status_line_spans(
        &self,
        cx: &App,
    ) -> (Vec<(String, HighlightStyle)>, Vec<(String, HighlightStyle)>) {
        (
            self.status_chip_spans(self.status_left_chips(), cx),
            self.status_chip_spans(self.status_right_chips(), cx),
        )
    }

    fn status_chip_spans(
        &self,
        chips: Vec<status_line::Chip>,
        cx: &App,
    ) -> Vec<(String, HighlightStyle)> {
        let separator_style = self.highlight_style_for_name(tau_themes::names::MODEL_STATUS, cx);
        let mut spans = Vec::new();
        for (index, chip) in chips.into_iter().enumerate() {
            if index > 0 {
                spans.push((" ".to_owned(), separator_style));
            }
            spans.push((
                chip.text,
                self.highlight_style_for_name(chip.style_name, cx),
            ));
        }
        spans
    }

    fn highlight_style_for_name(&self, style_name: &'static str, cx: &App) -> HighlightStyle {
        let theme_style = self
            .cli_theme
            .resolve_style(&tau_themes::StyleName::new(style_name));
        HighlightStyle {
            color: theme_style.fg.map(|color| tau_color_to_hsla(color, cx)),
            background_color: theme_style.bg.map(|color| tau_color_to_hsla(color, cx)),
            font_weight: theme_style.bold.then_some(FontWeight::BOLD),
            font_style: theme_style.italic.then_some(FontStyle::Italic),
            underline: None,
            strikethrough: None,
            fade_out: None,
        }
    }

    fn record_main_tool_backgrounded(&mut self, call_id: &str) {
        self.main_tool_activity.record_backgrounded(call_id);
    }

    fn record_main_tool_completed(&mut self, call_id: &str) {
        self.main_tool_activity.record_completed(call_id);
    }

    fn status_left_chips(&self) -> Vec<status_line::Chip> {
        status_line::left_chips(
            &self.session_id,
            self.agents.current_agent_id(),
            self.current_role.as_deref(),
            self.current_model.as_ref(),
            self.baseline_params,
            self.current_params,
        )
    }

    fn status_right_chips(&self) -> Vec<status_line::Chip> {
        status_line::right_chips(
            self.main_tools_status_chip(),
            self.agents.active_count(),
            self.context_status_chip(),
        )
    }

    fn main_tools_status_chip(&self) -> Option<String> {
        self.main_tool_activity.status_chip()
    }

    fn context_status_chip(&self) -> Option<String> {
        match (
            self.current_context_percent,
            self.current_context_input_tokens,
            self.current_context_window,
        ) {
            (_, Some(input), Some(window)) => Some(format!(
                "{}/{}",
                tool_render::format_token_count(input),
                tool_render::format_token_count(window)
            )),
            (Some(percent), _, Some(window)) => Some(format!(
                "{percent}%/{}",
                tool_render::format_token_count(window)
            )),
            (Some(percent), _, None) => Some(format!("{percent}%")),
            (None, Some(input), None) => Some(tool_render::format_token_count(input)),
            (None, None, Some(window)) => {
                Some(format!("?/{}", tool_render::format_token_count(window)))
            }
            (None, None, None) => None,
        }
    }

    fn highlight_style(&self, style: TranscriptStyle, cx: &App) -> HighlightStyle {
        let theme_style = self
            .cli_theme
            .resolve_style(&tau_themes::StyleName::new(style.style_name()));
        HighlightStyle {
            color: theme_style.fg.map(|color| tau_color_to_hsla(color, cx)),
            background_color: theme_style.bg.map(|color| tau_color_to_hsla(color, cx)),
            font_weight: theme_style.bold.then_some(FontWeight::BOLD),
            font_style: theme_style.italic.then_some(FontStyle::Italic),
            underline: None,
            strikethrough: None,
            fade_out: None,
        }
    }
}

impl Render for TauGui {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (status_left, status_right) = self.status_line_spans(cx);
        let text_style = self
            .editor
            .update(cx, |editor, cx| editor.style(cx).text.clone());
        let status_left = styled_status_text(status_left, &text_style);
        let status_right = styled_status_text(status_right, &text_style);

        div()
            .id("tau-gui")
            .size_full()
            .flex()
            .flex_col()
            .p(px(2.))
            .bg(cx.theme().colors().editor_background)
            .key_context("TauGui")
            .child(
                div()
                    .id("tau-gui-editor")
                    .w_full()
                    .flex_grow(1.0)
                    .overflow_hidden()
                    .child(self.editor.clone()),
            )
            .child(
                div()
                    .id("tau-gui-status")
                    .w_full()
                    .flex_none()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .font_family(text_style.font_family.clone())
                    .text_size(text_style.font_size)
                    .line_height(text_style.line_height)
                    .text_color(text_style.color)
                    .child(
                        div()
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .truncate()
                            .child(status_left),
                    )
                    .child(
                        div()
                            .flex_none()
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .truncate()
                            .child(status_right),
                    ),
            )
    }
}

fn styled_status_text(
    spans: Vec<(String, HighlightStyle)>,
    default_style: &TextStyle,
) -> StyledText {
    let mut text = String::new();
    let mut highlights = Vec::new();
    for (span_text, style) in spans {
        let start = text.len();
        text.push_str(&span_text);
        let end = text.len();
        if start != end {
            highlights.push((start..end, style));
        }
    }
    StyledText::new(text).with_default_highlights(default_style, highlights)
}

fn tool_calls_from_output_items(output_items: &[ContextItem]) -> Vec<ToolCallItem> {
    output_items
        .iter()
        .filter_map(|item| match item {
            ContextItem::ToolCall(call) => Some(call.clone()),
            _ => None,
        })
        .collect()
}

fn render_tool_call_block(
    theme: &tau_themes::Theme,
    call: &ToolCallItem,
) -> tau_cli_term::StyledBlock {
    let display_payload = tool_display_from_call(call);
    let display = tool_render::render_tool_use_state(call.name.as_str(), &display_payload);
    tool_render::render_tool_block(theme, &display)
}

fn render_tool_result_block(
    theme: &tau_themes::Theme,
    result: &tau_proto::ToolResult,
) -> tau_cli_term::StyledBlock {
    let display = result
        .display
        .as_ref()
        .map(|display| tool_render::render_tool_use_state(&result.tool_name, display))
        .unwrap_or_else(|| {
            tool_render::render_tool_use_state(
                &result.tool_name,
                &tool_render::synthesize_fallback_display(&result.tool_name, None),
            )
        });
    let diff = result
        .display
        .as_ref()
        .and_then(|display| match &display.payload {
            Some(tau_proto::ToolUsePayload::Diff(summary)) => Some(summary.clone()),
            _ => None,
        })
        .or_else(|| tool_render::extract_diff(&result.result));
    match diff.as_ref() {
        Some(diff) => tool_render::render_diff_tool_block(theme, &display, diff, true),
        None => tool_render::render_tool_block(theme, &display),
    }
}

fn render_tool_result_parts_block(
    theme: &tau_themes::Theme,
    tool_name: &str,
    result: &CborValue,
    display: Option<&tau_proto::ToolUseState>,
) -> tau_cli_term::StyledBlock {
    let display = display
        .map(|display| tool_render::render_tool_use_state(tool_name, display))
        .unwrap_or_else(|| {
            tool_render::render_tool_use_state(
                tool_name,
                &tool_render::synthesize_fallback_display(tool_name, None),
            )
        });
    let diff = display
        .payload
        .as_ref()
        .and_then(|payload| match payload {
            tau_proto::ToolUsePayload::Diff(summary) => Some(summary.clone()),
            _ => None,
        })
        .or_else(|| tool_render::extract_diff(result));
    match diff.as_ref() {
        Some(diff) => tool_render::render_diff_tool_block(theme, &display, diff, true),
        None => tool_render::render_tool_block(theme, &display),
    }
}

fn render_tool_error_parts_block(
    theme: &tau_themes::Theme,
    tool_name: &str,
    message: &str,
    display: Option<&tau_proto::ToolUseState>,
) -> tau_cli_term::StyledBlock {
    let display = display
        .map(|display| tool_render::render_tool_use_state(tool_name, display))
        .unwrap_or_else(|| {
            tool_render::render_tool_use_state(
                tool_name,
                &tool_render::synthesize_fallback_display(tool_name, Some(message)),
            )
        });
    tool_render::render_tool_block(theme, &display)
}

fn render_tool_error_block(
    theme: &tau_themes::Theme,
    error: &tau_proto::ToolError,
) -> tau_cli_term::StyledBlock {
    let display = error
        .display
        .as_ref()
        .map(|display| tool_render::render_tool_use_state(&error.tool_name, display))
        .unwrap_or_else(|| {
            tool_render::render_tool_use_state(
                &error.tool_name,
                &tool_render::synthesize_fallback_display(&error.tool_name, Some(&error.message)),
            )
        });
    tool_render::render_tool_block(theme, &display)
}

fn tool_display_from_call(call: &ToolCallItem) -> tau_proto::ToolUseState {
    let args = match call.name.as_str() {
        "read" | "write" | "edit" | "ls" => cbor_text_field(&call.arguments, "path"),
        "grep" | "glob" => cbor_text_field(&call.arguments, "pattern"),
        "shell" => cbor_text_field(&call.arguments, "command"),
        "delegate" => cbor_text_field(&call.arguments, "task_name"),
        _ => cbor_text_field(&call.arguments, "path")
            .or_else(|| cbor_text_field(&call.arguments, "pattern"))
            .or_else(|| cbor_text_field(&call.arguments, "query")),
    }
    .unwrap_or_default();
    tau_proto::ToolUseState {
        args,
        status: tau_proto::ToolUseStatus::InProgress,
        status_text: tau_proto::PROGRESS_INDICATOR_TEXT.to_owned(),
        ..Default::default()
    }
}

fn cbor_text_field(arguments: &CborValue, key: &str) -> Option<String> {
    let CborValue::Map(entries) = arguments else {
        return None;
    };
    entries
        .iter()
        .find_map(|(entry_key, value)| match (entry_key, value) {
            (CborValue::Text(entry_key), CborValue::Text(value)) if entry_key == key => {
                Some(value.clone())
            }
            _ => None,
        })
}

fn agent_message_sent_summary(message: &tau_proto::AgentMessageSent) -> String {
    format!(
        "Message from {} to {}",
        message.sender_id,
        agent_message_sent_recipient_label(message)
    )
}

fn agent_message_sent_recipient_label(message: &tau_proto::AgentMessageSent) -> &str {
    match &message.recipient {
        tau_proto::AgentMessageRecipient::Agent { agent_id } => agent_id.as_str(),
        tau_proto::AgentMessageRecipient::User => "user",
    }
}

fn agent_message_sent_recipient_agent_id(message: &tau_proto::AgentMessageSent) -> Option<&str> {
    match &message.recipient {
        tau_proto::AgentMessageRecipient::Agent { agent_id } => Some(agent_id.as_str()),
        tau_proto::AgentMessageRecipient::User => None,
    }
}

fn provider_update_compaction_status(
    update: &tau_proto::ProviderResponseUpdated,
) -> Option<(tool_render::CompactionStatus, String)> {
    if update.items.iter().any(|item| {
        matches!(
            item,
            tau_proto::ProviderResponseItem::Completed(ContextItem::Compaction(_))
        )
    }) {
        return Some((
            tool_render::CompactionStatus::Success,
            compaction_success_status(
                update.compaction_original_input_tokens,
                update.compaction_compacted_input_tokens,
            ),
        ));
    }

    update.items.iter().find_map(|item| match item {
        tau_proto::ProviderResponseItem::InProgress(
            tau_proto::InProgressOutputItem::Compaction { .. },
        ) => Some((
            tool_render::CompactionStatus::Progress,
            compaction_progress_status(update.compaction_original_input_tokens),
        )),
        _ => None,
    })
}

fn compaction_token_chip(tokens: u64) -> String {
    format!("#{}", tool_render::format_token_count(tokens))
}

fn compaction_progress_status(original_input_tokens: Option<u64>) -> String {
    match original_input_tokens {
        Some(tokens) => format!("{} compacting", compaction_token_chip(tokens)),
        None => "compacting".to_owned(),
    }
}

fn compaction_success_status(
    original_input_tokens: Option<u64>,
    compacted_input_tokens: Option<u64>,
) -> String {
    match (original_input_tokens, compacted_input_tokens) {
        (Some(original), Some(compacted)) => format!(
            "{} → {} ok",
            compaction_token_chip(original),
            compaction_token_chip(compacted)
        ),
        (Some(original), None) => format!("{} ok", compaction_token_chip(original)),
        (None, Some(compacted)) => format!("ok: {}", compaction_token_chip(compacted)),
        (None, None) => "ok".to_owned(),
    }
}

fn agent_prompt_termination_reason(
    reason: tau_proto::AgentPromptTerminationReason,
) -> &'static str {
    match reason {
        tau_proto::AgentPromptTerminationReason::Stale => "stale",
        tau_proto::AgentPromptTerminationReason::Canceled => "cancelled",
    }
}

fn assistant_text_from_update(items: &[tau_proto::ProviderResponseItem]) -> Option<String> {
    let text = items
        .iter()
        .filter_map(|item| match item {
            tau_proto::ProviderResponseItem::Completed(ContextItem::Message(message))
                if message.role == ContextRole::Assistant =>
            {
                Some(message.content.iter().map(content_text).collect::<String>())
            }
            tau_proto::ProviderResponseItem::InProgress(
                tau_proto::InProgressOutputItem::Message { text, .. },
            ) => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    (!text.is_empty()).then_some(text)
}

fn assistant_text(items: &[ContextItem]) -> Option<String> {
    let text = items
        .iter()
        .filter_map(|item| match item {
            ContextItem::Message(message) if message.role == ContextRole::Assistant => {
                Some(message.content.iter().map(content_text).collect::<String>())
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    (!text.is_empty()).then_some(text)
}

fn content_text(part: &ContentPart) -> String {
    match part {
        ContentPart::Text { text } => text.clone(),
    }
}

fn block_spans<'a>(
    block: &'a tau_cli_term::StyledBlock,
    cx: &App,
) -> Vec<(&'a str, HighlightStyle)> {
    block
        .content
        .spans()
        .iter()
        .map(|span| {
            (
                span.text.as_str(),
                terminal_style_to_highlight(span.style, cx),
            )
        })
        .collect()
}

fn terminal_style_to_highlight(style: tau_cli_term::Style, cx: &App) -> HighlightStyle {
    HighlightStyle {
        color: style.fg.map(|color| terminal_color_to_hsla(color, cx)),
        background_color: style.bg.map(|color| terminal_color_to_hsla(color, cx)),
        font_weight: style.bold.then_some(FontWeight::BOLD),
        font_style: style.italic.then_some(FontStyle::Italic),
        underline: None,
        strikethrough: None,
        fade_out: None,
    }
}

fn terminal_color_to_hsla(color: tau_cli_term::Color, cx: &App) -> Hsla {
    match color {
        tau_cli_term::Color::Reset => cx.theme().colors().terminal_foreground,
        tau_cli_term::Color::Black => cx.theme().colors().terminal_ansi_black,
        tau_cli_term::Color::DarkGrey => cx.theme().colors().terminal_ansi_bright_black,
        tau_cli_term::Color::Red => cx.theme().colors().terminal_ansi_bright_red,
        tau_cli_term::Color::DarkRed => cx.theme().colors().terminal_ansi_red,
        tau_cli_term::Color::Green => cx.theme().colors().terminal_ansi_bright_green,
        tau_cli_term::Color::DarkGreen => cx.theme().colors().terminal_ansi_green,
        tau_cli_term::Color::Yellow => cx.theme().colors().terminal_ansi_bright_yellow,
        tau_cli_term::Color::DarkYellow => cx.theme().colors().terminal_ansi_yellow,
        tau_cli_term::Color::Blue => cx.theme().colors().terminal_ansi_bright_blue,
        tau_cli_term::Color::DarkBlue => cx.theme().colors().terminal_ansi_blue,
        tau_cli_term::Color::Magenta => cx.theme().colors().terminal_ansi_bright_magenta,
        tau_cli_term::Color::DarkMagenta => cx.theme().colors().terminal_ansi_magenta,
        tau_cli_term::Color::Cyan => cx.theme().colors().terminal_ansi_bright_cyan,
        tau_cli_term::Color::DarkCyan => cx.theme().colors().terminal_ansi_cyan,
        tau_cli_term::Color::White => cx.theme().colors().terminal_ansi_bright_white,
        tau_cli_term::Color::Grey => cx.theme().colors().terminal_ansi_white,
        tau_cli_term::Color::Rgb { r, g, b } => Rgba {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: 1.,
        }
        .into(),
        tau_cli_term::Color::AnsiValue(value) => ansi_color_to_hsla(value, cx),
    }
}

fn ansi_color_to_hsla(value: u8, cx: &App) -> Hsla {
    if value < 16 {
        return ansi_basic_color_to_hsla(value, cx);
    }
    ansi_color_to_rgba(value).into()
}

fn ansi_basic_color_to_hsla(value: u8, cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match value {
        0 => colors.terminal_ansi_black,
        1 => colors.terminal_ansi_red,
        2 => colors.terminal_ansi_green,
        3 => colors.terminal_ansi_yellow,
        4 => colors.terminal_ansi_blue,
        5 => colors.terminal_ansi_magenta,
        6 => colors.terminal_ansi_cyan,
        7 => colors.terminal_ansi_white,
        8 => colors.terminal_ansi_bright_black,
        9 => colors.terminal_ansi_bright_red,
        10 => colors.terminal_ansi_bright_green,
        11 => colors.terminal_ansi_bright_yellow,
        12 => colors.terminal_ansi_bright_blue,
        13 => colors.terminal_ansi_bright_magenta,
        14 => colors.terminal_ansi_bright_cyan,
        _ => colors.terminal_ansi_bright_white,
    }
}

fn ansi_color_to_rgba(value: u8) -> Rgba {
    if (16..=231).contains(&value) {
        let value = value - 16;
        let r = value / 36;
        let g = (value % 36) / 6;
        let b = value % 6;
        return Rgba {
            r: cube_component(r),
            g: cube_component(g),
            b: cube_component(b),
            a: 1.,
        };
    }
    let level = 8 + (value.saturating_sub(232) as u16).min(23) * 10;
    let channel = level as f32 / 255.;
    Rgba {
        r: channel,
        g: channel,
        b: channel,
        a: 1.,
    }
}

fn cube_component(value: u8) -> f32 {
    if value == 0 {
        0.
    } else {
        (55 + value as u16 * 40) as f32 / 255.
    }
}

fn tau_color_to_hsla(color: tau_themes::Color, cx: &App) -> Hsla {
    let colors = cx.theme().colors();
    match color {
        tau_themes::Color::Black => colors.terminal_ansi_black,
        tau_themes::Color::DarkRed => colors.terminal_ansi_red,
        tau_themes::Color::DarkGreen => colors.terminal_ansi_green,
        tau_themes::Color::DarkYellow => colors.terminal_ansi_yellow,
        tau_themes::Color::DarkBlue => colors.terminal_ansi_blue,
        tau_themes::Color::DarkMagenta => colors.terminal_ansi_magenta,
        tau_themes::Color::DarkCyan => colors.terminal_ansi_cyan,
        tau_themes::Color::DarkGrey => colors.terminal_ansi_bright_black,
        tau_themes::Color::Red => colors.terminal_ansi_bright_red,
        tau_themes::Color::Green => colors.terminal_ansi_bright_green,
        tau_themes::Color::Yellow => colors.terminal_ansi_bright_yellow,
        tau_themes::Color::Blue => colors.terminal_ansi_bright_blue,
        tau_themes::Color::Magenta => colors.terminal_ansi_bright_magenta,
        tau_themes::Color::Cyan => colors.terminal_ansi_bright_cyan,
        tau_themes::Color::White => colors.terminal_ansi_bright_white,
        tau_themes::Color::Grey => colors.terminal_ansi_white,
        tau_themes::Color::Rgb { r, g, b } => Rgba {
            r: r as f32 / 255.,
            g: g as f32 / 255.,
            b: b as f32 / 255.,
            a: 1.,
        }
        .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer_text(buffer: &Buffer) -> String {
        buffer.text_for_range(0..buffer.len()).collect()
    }

    #[test]
    fn prompt_insert_action_only_moves_when_selection_is_outside_draft() {
        assert!(selection_outside_prompt(4, 5, 10));
        assert!(!selection_outside_prompt(5, 5, 10));
        assert!(!selection_outside_prompt(8, 5, 10));
        assert!(!selection_outside_prompt(10, 5, 10));
        assert!(selection_outside_prompt(11, 5, 10));
    }

    #[gpui::test]
    fn anchor_before_stays_before_insertions_at_same_offset(cx: &mut App) {
        let buffer = cx.new(|cx| Buffer::local("ab", cx));
        buffer.update(cx, |buffer, cx| {
            let left = buffer.anchor_before(1);
            let right = buffer.anchor_after(1);

            buffer.edit([(1..1, "X")], None, cx);

            assert_eq!(buffer_text(buffer), "aXb");
            assert_eq!(left.to_offset(buffer), 1);
            assert_eq!(right.to_offset(buffer), 2);
        });
    }

    #[gpui::test]
    fn transcript_newline_checks_handle_multibyte_characters(cx: &mut App) {
        let buffer = cx.new(|cx| Buffer::local("hello 🌍", cx));

        buffer.update(cx, |buffer, _cx| {
            assert!(!buffer_text_ends_with(buffer, buffer.len(), '\n'));
            assert!(buffer_text_ends_with(buffer, buffer.len(), '🌍'));
            assert!(!buffer_range_starts_with(buffer, 6..buffer.len(), '\n'));
            assert!(buffer_range_starts_with(buffer, 6..buffer.len(), '🌍'));
        });
    }

    #[gpui::test]
    fn split_buffers_compose_into_writable_prompt_with_read_only_transcript(cx: &mut App) {
        let transcript_buffer = cx.new(|cx| {
            let mut buffer = Buffer::local("", cx);
            buffer.set_capability(Capability::Read, cx);
            buffer
        });
        let prompt_buffer = cx.new(|cx| Buffer::local("", cx));
        let multi_buffer = cx.new(|cx| {
            let mut multi_buffer = MultiBuffer::without_headers(Capability::ReadWrite);
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(0),
                transcript_buffer.clone(),
                [Point::zero()..transcript_buffer.read(cx).max_point()],
                0,
                cx,
            );
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(1),
                prompt_buffer.clone(),
                [Point::zero()..prompt_buffer.read(cx).max_point()],
                0,
                cx,
            );
            multi_buffer
        });

        transcript_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "hello")], None, cx);
        });
        prompt_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "draft")], None, cx);
        });

        let snapshot = multi_buffer.read(cx).snapshot(cx);
        assert_eq!(snapshot.text(), "hello\ndraft");
        assert_eq!(buffer_text(prompt_buffer.read(cx)), "draft");
        assert_eq!(transcript_buffer.read(cx).capability(), Capability::Read);
        assert_eq!(prompt_buffer.read(cx).capability(), Capability::ReadWrite);
    }

    #[gpui::test]
    fn prompt_autoscroll_anchor_tracks_prompt_end_after_transcript_edits(cx: &mut App) {
        use multi_buffer::ToOffset as _;

        let transcript_buffer = cx.new(|cx| {
            let mut buffer = Buffer::local("", cx);
            buffer.set_capability(Capability::Read, cx);
            buffer
        });
        let prompt_buffer = cx.new(|cx| Buffer::local("", cx));
        let draft_end = prompt_buffer.read(cx).anchor_after(0);
        let multi_buffer = cx.new(|cx| {
            let mut multi_buffer = MultiBuffer::without_headers(Capability::ReadWrite);
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(0),
                transcript_buffer.clone(),
                [Point::zero()..transcript_buffer.read(cx).max_point()],
                0,
                cx,
            );
            multi_buffer.set_excerpts_for_path(
                PathKey::sorted(1),
                prompt_buffer.clone(),
                [Point::zero()..prompt_buffer.read(cx).max_point()],
                0,
                cx,
            );
            multi_buffer
        });

        transcript_buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "transcript\n")], None, cx);
        });
        prompt_buffer.update(cx, |buffer, cx| {
            let end = draft_end.to_offset(buffer);
            buffer.edit([(end..end, "draft")], None, cx);
        });

        let snapshot = multi_buffer.read(cx).snapshot(cx);
        let draft_anchor = snapshot
            .anchor_in_excerpt(draft_end)
            .expect("draft end should be present in multibuffer");
        assert_eq!(draft_anchor.to_offset(&snapshot), snapshot.len());
    }
}
