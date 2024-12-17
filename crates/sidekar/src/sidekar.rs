mod active_thread;
mod assistant_configuration;
mod assistant_model_selector;
mod assistant_panel;
mod buffer_codegen;
mod context;
mod context_picker;
mod context_store;
mod context_strip;
mod inline_assistant;
mod inline_prompt_editor;
mod message_editor;
mod sidecar;
mod terminal_codegen;
mod terminal_inline_assistant;
mod thread;
mod thread_history;
mod thread_store;
mod types;
mod ui;

use std::sync::Arc;

use assistant_settings::AssistantSettings;
use client::Client;
use fs::Fs;
use gpui::{actions, App};
use prompt_library::PromptBuilder;
use settings::Settings as _;

pub use crate::assistant_panel::{AssistantPanel, ConcreteAssistantPanelDelegate};
pub use crate::inline_assistant::InlineAssistant;

actions!(
    sidekar,
    [
        SendRequest,
        NewThread,
        NewPromptEditor,
        ToggleContextPicker,
        ToggleModelSelector,
        RemoveAllContext,
        OpenHistory,
        OpenPromptEditorHistory,
        OpenConfiguration,
        RemoveSelectedThread,
        Chat,
        ChatMode,
        CycleNextInlineAssist,
        CyclePreviousInlineAssist,
        FocusUp,
        FocusDown,
        FocusLeft,
        FocusRight,
        RemoveFocusedContext,
        AcceptSuggestedContext
    ]
);

/// Initializes the `sidekar` crate.
pub fn init(
    fs: Arc<dyn Fs>,
    client: Arc<Client>,
    prompt_builder: Arc<PromptBuilder>,
    cx: &mut App,
) {
    AssistantSettings::register(cx);
    assistant_panel::init(cx);

    inline_assistant::init(
        fs.clone(),
        prompt_builder.clone(),
        client.telemetry().clone(),
        cx,
    );
    terminal_inline_assistant::init(
        fs.clone(),
        prompt_builder.clone(),
        client.telemetry().clone(),
        cx,
    );
}
