use std::sync::Arc;

use crate::{
    active_thread::ActiveThread,
    message_editor::MessageEditor,
    thread::{Thread, ThreadError, ThreadEvent, ThreadId},
    thread_store::ThreadStore,
};
use assistant_tool::ToolWorkingSet;
use fs::Fs;
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, FontWeight,
    IntoElement, SharedString, Subscription, WeakEntity, Window,
};
use language::LanguageRegistry;
use ui::prelude::*;
use workspace::{
    item::{self, Item},
    Workspace,
};

pub struct ThreadItem {
    thread: Entity<Thread>,
    active_thread: Entity<ActiveThread>,
    message_editor: Entity<MessageEditor>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Clone)]
pub enum ThreadItemEvent {
    ThreadEvent(ThreadEvent),
}

impl EventEmitter<ThreadItemEvent> for ThreadItem {}

impl ThreadItem {
    pub fn new(
        thread: Entity<Thread>,
        thread_store: Entity<ThreadStore>,
        workspace: WeakEntity<Workspace>,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let active_thread = cx.new(|cx| {
            ActiveThread::new(
                thread.clone(),
                thread_store.clone(),
                language_registry,
                window,
                cx,
            )
        });

        let message_editor = cx.new(|cx| {
            MessageEditor::new(
                fs,
                workspace.clone(),
                thread_store.downgrade(),
                thread.clone(),
                window,
                cx,
            )
        });

        let thread = active_thread.read(cx).thread().clone();

        let _subscriptions = vec![cx.subscribe(&thread, |_, _, event, cx| {
            cx.emit(ThreadItemEvent::ThreadEvent(event.clone()))
        })];

        Self {
            thread,
            active_thread,
            message_editor,
            _subscriptions,
        }
    }

    pub fn thread(&self) -> &Entity<Thread> {
        &self.thread
    }

    pub fn active_thread(&self) -> &Entity<ActiveThread> {
        &self.active_thread
    }

    pub fn message_editor(&self) -> &Entity<MessageEditor> {
        &self.message_editor
    }

    fn cancel(
        &mut self,
        _: &editor::actions::Cancel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.thread
            .update(cx, |thread, _| thread.cancel_last_completion());
    }

    fn render_last_error(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let error_message = self
            .active_thread
            .read(cx)
            .last_error()
            .and_then(|e| match e {
                ThreadError::Message(error) => Some(error),
                _ => None,
            })?;

        Some(
            div()
                .absolute()
                .right_3()
                .bottom_12()
                .max_w_96()
                .py_2()
                .px_3()
                .elevation_2(cx)
                .occlude()
                .child(
                    v_flex()
                        .gap_0p5()
                        .child(
                            h_flex()
                                .gap_1p5()
                                .items_center()
                                .child(Icon::new(IconName::XCircle).color(Color::Error))
                                .child(
                                    Label::new("Error interacting with language model")
                                        .weight(FontWeight::MEDIUM),
                                ),
                        )
                        .child(
                            div()
                                .id("error-message")
                                .max_h_32()
                                .overflow_y_scroll()
                                .child(Label::new(error_message.clone())),
                        )
                        .child(h_flex().justify_end().mt_1().child(
                            Button::new("dismiss", "Dismiss").on_click(cx.listener(
                                |this, _, _, cx| {
                                    this.active_thread.update(cx, |this, _cx| {
                                        this.clear_last_error();
                                    });

                                    cx.notify();
                                },
                            )),
                        ))
                        .into_any(),
                )
                .into_any(),
        )
    }
}

impl Item for ThreadItem {
    type Event = ThreadItemEvent;

    fn tab_content_text(&self, _window: &Window, cx: &App) -> Option<SharedString> {
        Some(self.active_thread.read(cx).summary_or_default(cx))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(item::ItemEvent)) {
        match event {
            ThreadItemEvent::ThreadEvent(ThreadEvent::SummaryChanged) => {
                f(item::ItemEvent::UpdateTab)
            }
            _ => {}
        }
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        Some(self.active_thread.read(cx).summary_or_default(cx))
    }

    fn include_in_nav_history() -> bool {
        false
    }
}

impl Render for ThreadItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg_color = cx.theme().colors().editor_background;
        v_flex()
            .size_full()
            .bg(bg_color)
            .on_action(cx.listener(Self::cancel))
            .child(self.active_thread.clone())
            .child(h_flex().child(self.message_editor.clone()))
            .children(self.render_last_error(cx))
    }
}

impl Focusable for ThreadItem {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.message_editor.focus_handle(cx)
    }
}
