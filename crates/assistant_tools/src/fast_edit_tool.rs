use crate::{
    Templates,
    edit_agent2::{
        EditAgent2, EditAgentOutput, EditAgentOutputEvent, EditFormat, MultiFileEditOutput,
    },
};
use anyhow::Result;
use assistant_tool::ActionLog;
use collections::HashMap;
use futures::StreamExt;
use gpui::Render;
use gpui::{
    Animation, AnimationExt, App, AsyncApp, Context, Entity, Task, Transformation, WeakEntity,
    percentage, pulsating_between,
};
use language_model::{LanguageModel, LanguageModelRequest};
use project::Project;
use std::{path::PathBuf, sync::Arc, time::Duration};
use ui::{IconName, prelude::*};

/// Performs fast edits on multiple files based on the conversation context
pub async fn perform_fast_edit(
    edit_description: String,
    request: Arc<LanguageModelRequest>,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    model: Arc<dyn LanguageModel>,
    cx: &mut AsyncApp,
) -> Result<FastEditResult> {
    let edit_format = EditFormat::from_model(model.clone())?;
    let edit_agent = EditAgent2::new(
        model,
        project.clone(),
        action_log,
        Templates::new(),
        edit_format,
    );

    // Use edit to handle multiple files
    let (output_task, mut events) = edit_agent.edit(edit_description.clone(), &request, cx);

    let mut all_events = Vec::new();

    // Collect all events
    while let Some(event) = events.next().await {
        all_events.push(event);
    }

    let output = output_task.await?;

    Ok(FastEditResult {
        output,
        events: all_events,
        edit_description,
    })
}

#[derive(Clone)]
pub struct FastEditResult {
    pub output: MultiFileEditOutput,
    pub events: Vec<EditAgentOutputEvent>,
    pub edit_description: String,
}

impl FastEditResult {
    /// Generate a markdown summary of the edits
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("## Fast Edit Results\n\n");
        markdown.push_str(&format!("**Description:** {}\n\n", self.edit_description));

        if self.output.file_edits.is_empty() {
            markdown.push_str("No edits were made.\n");
        } else {
            markdown.push_str(&format!(
                "Edited {} file(s):\n\n",
                self.output.file_edits.len()
            ));

            for file_edit in &self.output.file_edits {
                markdown.push_str(&format!("- `{}`\n", file_edit.path.display()));
            }
        }

        markdown
    }
}

/// UI component for displaying fast edit results
pub struct FastEditView {
    #[allow(dead_code)]
    project: Entity<Project>,
    result: Option<FastEditResult>,
    edited_files: Vec<EditedFile>,
    expanded_files: HashMap<PathBuf, bool>,
}

struct EditedFile {
    path: PathBuf,
    output: EditAgentOutput,
}

impl FastEditView {
    pub fn new(project: Entity<Project>) -> Self {
        Self {
            project,
            result: None,
            edited_files: Vec::new(),
            expanded_files: HashMap::new(),
        }
    }

    pub fn set_result(&mut self, result: FastEditResult, cx: &mut Context<Self>) {
        self.edited_files = result
            .output
            .file_edits
            .iter()
            .map(|file_edit| EditedFile {
                path: file_edit.path.clone(),
                output: file_edit.output.clone(),
            })
            .collect();

        self.result = Some(result);
        cx.notify();
    }

    pub fn handle_event(&mut self, event: EditAgentOutputEvent, cx: &mut Context<Self>) {
        match event {
            EditAgentOutputEvent::Edited { path } => {
                // Find the edited file in our list
                if let Some(_file) = self.edited_files.iter_mut().find(|f| f.path == path) {
                    // Update if needed
                    cx.notify();
                }
            }
            EditAgentOutputEvent::ResolvingEditRange { path, .. } => {
                if !self.edited_files.iter().any(|f| f.path == path) {
                    self.edited_files.push(EditedFile {
                        path,
                        output: EditAgentOutput {
                            raw_edits: String::new(),
                            parser_metrics: Default::default(),
                        },
                    });
                }
                cx.notify();
            }
            _ => {}
        }
    }

    fn toggle_file(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let expanded = self.expanded_files.entry(path).or_insert(false);
        *expanded = !*expanded;
        cx.notify();
    }

    pub fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let is_pending = self.result.is_none();
        let header_color = if is_pending {
            Color::Muted
        } else {
            Color::Default
        };

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::Bolt)
                            .size(IconSize::Small)
                            .color(header_color),
                    )
                    .child(
                        Label::new("Fast Edit")
                            .size(LabelSize::Small)
                            .color(header_color),
                    )
                    .when(is_pending, |el| {
                        el.child(
                            Icon::new(IconName::ArrowCircle)
                                .size(IconSize::XSmall)
                                .color(Color::Muted)
                                .with_animation(
                                    "arrow-circle",
                                    Animation::new(Duration::from_secs(2))
                                        .repeat()
                                        .with_easing(pulsating_between(0.0, 1.0)),
                                    |icon, delta| {
                                        icon.transform(Transformation::rotate(percentage(delta)))
                                    },
                                ),
                        )
                    }),
            )
            .child(if let Some(result) = &self.result {
                v_flex()
                    .gap_1()
                    .child(
                        Label::new(&result.edit_description)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .children(self.edited_files.iter().enumerate().map(|(idx, file)| {
                                let path = file.path.clone();
                                let is_expanded =
                                    self.expanded_files.get(&path).copied().unwrap_or(false);

                                v_flex()
                                    .gap_1()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                IconButton::new(
                                                    SharedString::from(format!("toggle-{}", idx)),
                                                    if is_expanded {
                                                        IconName::ChevronDown
                                                    } else {
                                                        IconName::ChevronRight
                                                    },
                                                )
                                                .icon_size(IconSize::Small)
                                                .on_click({
                                                    let path = path.clone();
                                                    cx.listener(move |this, _, _, cx| {
                                                        this.toggle_file(path.clone(), cx);
                                                    })
                                                }),
                                            )
                                            .child(
                                                Label::new(
                                                    path.file_name()
                                                        .and_then(|name| name.to_str())
                                                        .map(|s| s.to_string())
                                                        .unwrap_or_else(|| {
                                                            "Unknown file".to_string()
                                                        }),
                                                )
                                                .size(LabelSize::Small),
                                            ),
                                    )
                                    .when(is_expanded, |el| {
                                        el.child(
                                            div().ml_4().child(
                                                Label::new(format!(
                                                    "Applied {} edits",
                                                    file.output.parser_metrics.tags
                                                ))
                                                .size(LabelSize::Small)
                                                .color(Color::Success),
                                            ),
                                        )
                                    })
                            })),
                    )
                    .into_any_element()
            } else {
                Label::new("Preparing fast edit...")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .into_any_element()
            })
    }
}

impl Render for FastEditView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_4()
            .gap_3()
            .child(
                h_flex()
                    .gap_2()
                    .child(Icon::new(IconName::Pencil).size(IconSize::Small))
                    .child(Label::new("Fast Edit Results").size(LabelSize::Large)),
            )
            .when_some(self.result.as_ref(), |parent, result| {
                parent
                    .child(
                        Label::new(format!("{} files edited", result.output.file_edits.len()))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .children(result.output.file_edits.iter().map(|file_edit| {
                                h_flex()
                                    .gap_2()
                                    .child(Icon::new(IconName::File).size(IconSize::Small))
                                    .child(
                                        Label::new(file_edit.path.display().to_string())
                                            .size(LabelSize::Small),
                                    )
                            })),
                    )
            })
            .when(self.result.is_none(), |parent| {
                parent.child(
                    div()
                        .h_20()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(
                                    Icon::new(IconName::ArrowCircle)
                                        .size(IconSize::Small)
                                        .with_animation(
                                            "arrow-circle",
                                            Animation::new(Duration::from_secs(2)).repeat(),
                                            |icon, delta| {
                                                icon.transform(Transformation::rotate(percentage(
                                                    delta,
                                                )))
                                            },
                                        ),
                                )
                                .child(Label::new("Running fast edit...").color(Color::Muted)),
                        ),
                )
            })
    }
}

/// Helper function to create a task that performs fast edit and updates a view
pub fn create_fast_edit_task(
    edit_description: String,
    request: Arc<LanguageModelRequest>,
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
    model: Arc<dyn LanguageModel>,
    view: Option<WeakEntity<FastEditView>>,
    cx: &mut App,
) -> Task<Result<FastEditResult>> {
    cx.spawn(async move |mut cx| {
        let result = perform_fast_edit(
            edit_description,
            request,
            project,
            action_log,
            model,
            &mut cx,
        )
        .await?;

        // Update the view if provided
        if let Some(view) = view {
            if let Some(view) = view.upgrade() {
                view.update(cx, |view, cx| {
                    view.set_result(result.clone(), cx);
                })?;
            }
        }

        Ok(result)
    })
}
