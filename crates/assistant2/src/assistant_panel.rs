use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_context_editor::{
    make_lsp_adapter_delegate, render_remaining_tokens, AssistantPanelDelegate, ConfigurationError,
    ContextEditor, SlashCommandCompletionProvider,
};
use assistant_settings::{AssistantDockPosition, AssistantSettings};
use assistant_slash_command::SlashCommandWorkingSet;
use assistant_tool::ToolWorkingSet;

use client::zed_urls;
use editor::Editor;
use fs::Fs;
use gpui::{
    prelude::*, Action, AnyElement, App, AsyncWindowContext, Corner, Entity, EventEmitter,
    FocusHandle, Focusable, FontWeight, KeyContext, Pixels, Subscription, Task, UpdateGlobal,
    WeakEntity,
};
use language::LanguageRegistry;
use language_model::{LanguageModelProviderTosView, LanguageModelRegistry};
use project::Project;
use prompt_library::{open_prompt_library, PromptLibrary};
use prompt_store::PromptBuilder;
use settings::{update_settings_file, Settings};
use time::UtcOffset;
use ui::{prelude::*, ContextMenu, KeyBinding, PopoverMenu, PopoverMenuHandle, Tab, Tooltip};
use util::ResultExt as _;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use workspace::{Pane, Workspace};
use zed_actions::assistant::{DeployPromptLibrary, ToggleFocus};

use crate::active_thread::ActiveThread;
use crate::assistant_configuration::{AssistantConfiguration, AssistantConfigurationEvent};
use crate::context_store::ContextStore;
use crate::history_store::{HistoryEntry, HistoryStore};
use crate::message_editor::MessageEditor;
use crate::thread::{Thread, ThreadError, ThreadId};
use crate::thread_history::{PastThread, ThreadHistory};
use crate::thread_item::ThreadItem;
use crate::thread_store::ThreadStore;
use crate::{InlineAssistant, NewPromptEditor, NewThread, OpenConfiguration, OpenHistory};

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(|workspace, _: &NewThread, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        panel.update(cx, |panel, cx| panel.new_thread(window, cx));
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                    }
                })
                .register_action(|workspace, _: &OpenHistory, window, cx| {
                    if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                        workspace.focus_panel::<AssistantPanel>(window, cx);
                        panel.update(cx, |panel, cx| panel.open_history(window, cx));
                    }
                });
        },
    )
    .detach();
}

pub struct AssistantPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    configuration_subscription: Option<Subscription>,
    local_timezone: UtcOffset,
    history_store: Entity<HistoryStore>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    pane: Entity<Pane>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakEntity<Workspace>,
        _prompt_builder: Arc<PromptBuilder>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(|mut cx| async move {
            let tools = Arc::new(ToolWorkingSet::default());
            log::info!("[assistant2-debug] initializing ThreadStore");
            let thread_store = workspace.update(&mut cx, |workspace, cx| {
                let project = workspace.project().clone();
                ThreadStore::new(project, tools.clone(), cx)
            })??;
            log::info!("[assistant2-debug] finished initializing ThreadStore");

            workspace.update_in(&mut cx, |workspace, window, cx| {
                cx.new(|cx| Self::new(workspace, thread_store, window, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        thread_store: Entity<ThreadStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let fs = workspace.app_state().fs.clone();
        let project = workspace.project().clone();
        let language_registry = project.read(cx).languages().clone();
        let workspace_handle = workspace.weak_handle();
        let history_store = cx.new(|cx| HistoryStore::new(thread_store.clone(), cx));

        let pane = cx.new(|cx| {
            let mut pane = Pane::new(
                workspace_handle.clone(),
                project.clone(),
                Default::default(),
                None,
                NewThread.boxed_clone(),
                window,
                cx,
            );

            pane.set_can_navigate(true, cx);
            pane.set_should_display_tab_bar(|_, _| true);
            pane.display_nav_history_buttons(None);
            pane.set_render_tab_bar_buttons(cx, |_pane, _window, _cx| (None, None));

            pane
        });

        Self {
            workspace: workspace_handle.clone(),
            project: project.clone(),
            fs: fs.clone(),
            language_registry,
            thread_store,
            configuration_subscription: None,
            local_timezone: UtcOffset::from_whole_seconds(
                chrono::Local::now().offset().local_minus_utc(),
            )
            .unwrap(),
            history_store: history_store.clone(),
            width: None,
            height: None,
            pane,
        }
    }

    pub fn toggle_focus(
        workspace: &mut Workspace,
        _: &ToggleFocus,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled {
            return;
        }

        workspace.toggle_panel_focus::<Self>(window, cx);
    }

    pub(crate) fn local_timezone(&self) -> UtcOffset {
        self.local_timezone
    }

    pub(crate) fn thread_store(&self) -> &Entity<ThreadStore> {
        &self.thread_store
    }

    fn new_thread(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let context_store = cx.new(|_cx| ContextStore::new(self.workspace.clone()));

        let thread = self
            .thread_store
            .update(cx, |this, cx| this.create_thread(context_store.clone(), cx));

        let thread_item = cx.new(|cx| {
            ThreadItem::new(
                thread.clone(),
                self.thread_store.clone(),
                self.workspace.clone(),
                self.language_registry.clone(),
                self.fs.clone(),
                window,
                cx,
            )
        });
        self.pane.update(cx, |pane, cx| {
            pane.add_item(Box::new(thread_item.clone()), true, true, None, window, cx);
        });
        thread_item.update(cx, |thread_item, cx| {
            thread_item.message_editor().focus_handle(cx).focus(window);
        });
    }

    fn deploy_prompt_library(
        &mut self,
        _: &DeployPromptLibrary,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        open_prompt_library(
            self.language_registry.clone(),
            Box::new(PromptLibraryInlineAssist::new(self.workspace.clone())),
            Arc::new(|| {
                Box::new(SlashCommandCompletionProvider::new(
                    Arc::new(SlashCommandWorkingSet::default()),
                    None,
                    None,
                ))
            }),
            cx,
        )
        .detach_and_log_err(cx);
    }

    fn open_history(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.thread_store
            .update(cx, |thread_store, cx| thread_store.reload(cx))
            .detach_and_log_err(cx);

        let history_item_ix = self
            .pane
            .read(cx)
            .items()
            .position(|item| item.downcast::<ThreadHistory>().is_some());

        if let Some(history_item_ix) = history_item_ix {
            self.pane.update(cx, |pane, cx| {
                pane.activate_item(history_item_ix, true, true, window, cx);
            });
        } else {
            let this = cx.entity();
            let history_item =
                cx.new(|cx| ThreadHistory::new(this.downgrade(), self.history_store.clone(), cx));

            self.pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(history_item), true, true, None, window, cx);
            });
        }
    }

    pub(crate) fn open_thread(
        &mut self,
        thread_id: &ThreadId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        // TODO: save context on disk
        let context_store = cx.new(|_cx| ContextStore::new(self.workspace.clone()));

        let open_thread_task = self.thread_store.update(cx, |this, cx| {
            this.open_thread(thread_id, context_store.clone(), cx)
        });

        cx.spawn_in(window, |this, mut cx| async move {
            let thread = open_thread_task.await?;
            this.update_in(&mut cx, |this, window, cx| {
                let thread_item = cx.new(|cx| {
                    ThreadItem::new(
                        thread.clone(),
                        this.thread_store.clone(),
                        this.workspace.clone(),
                        this.language_registry.clone(),
                        this.fs.clone(),
                        window,
                        cx,
                    )
                });

                this.pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(thread_item.clone()), true, true, None, window, cx);
                });

                thread_item.update(cx, |thread_item, cx| {
                    thread_item.message_editor().focus_handle(cx).focus(window);
                });

                anyhow::Ok(())
            })??;
            Ok(())
        })
    }

    pub(crate) fn active_thread(&self, cx: &App) -> Option<Entity<Thread>> {
        self.pane
            .read(cx)
            .active_item()
            .and_then(|item| item.downcast::<ThreadItem>())
            .map(|thread_item| thread_item.read(cx).thread().clone())
    }

    pub(crate) fn delete_thread(&mut self, thread_id: &ThreadId, cx: &mut Context<Self>) {
        self.thread_store
            .update(cx, |this, cx| this.delete_thread(thread_id, cx))
            .detach_and_log_err(cx);
    }

    fn key_context(&self) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("AssistantPanel2");
        key_context
    }
}

impl Focusable for AssistantPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanel2"
    }

    fn position(&self, _window: &Window, cx: &App) -> DockPosition {
        match AssistantSettings::get_global(cx).dock {
            AssistantDockPosition::Left => DockPosition::Left,
            AssistantDockPosition::Bottom => DockPosition::Bottom,
            AssistantDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file::<AssistantSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| {
                let dock = match position {
                    DockPosition::Left => AssistantDockPosition::Left,
                    DockPosition::Bottom => AssistantDockPosition::Bottom,
                    DockPosition::Right => AssistantDockPosition::Right,
                };
                settings.set_dock(dock);
            },
        );
    }

    fn size(&self, window: &Window, cx: &App) -> Pixels {
        let settings = AssistantSettings::get_global(cx);
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        match self.position(window, cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        cx.notify();
    }

    fn set_active(&mut self, _active: bool, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn remote_id() -> Option<proto::PanelId> {
        Some(proto::PanelId::AssistantPanel)
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<IconName> {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled || !settings.button {
            return None;
        }

        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        3
    }

    fn pane(&self) -> Option<Entity<Pane>> {
        Some(self.pane.clone())
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context(self.key_context())
            .justify_between()
            .size_full()
            .on_action(cx.listener(|this, _: &NewThread, window, cx| {
                this.new_thread(window, cx);
            }))
            .child(self.pane.clone().into_any_element())
    }
}

struct PromptLibraryInlineAssist {
    workspace: WeakEntity<Workspace>,
}

impl PromptLibraryInlineAssist {
    pub fn new(workspace: WeakEntity<Workspace>) -> Self {
        Self { workspace }
    }
}

impl prompt_library::InlineAssistDelegate for PromptLibraryInlineAssist {
    fn assist(
        &self,
        prompt_editor: &Entity<Editor>,
        _initial_prompt: Option<String>,
        window: &mut Window,
        cx: &mut Context<PromptLibrary>,
    ) {
        InlineAssistant::update_global(cx, |assistant, cx| {
            assistant.assist(&prompt_editor, self.workspace.clone(), None, window, cx)
        })
    }

    fn focus_assistant_panel(
        &self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        workspace
            .focus_panel::<AssistantPanel>(window, cx)
            .is_some()
    }
}
