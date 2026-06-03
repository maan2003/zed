# Intentional deviations from tau-cli

`tau-cli` remains the behavioral source of truth for protocol handling, command semantics, event lifecycles, and transcript rendering. The deviations below are intentional GUI/editor adaptations, not parity gaps.

## Editor-backed agent tabs

`tau-gui` presents agents as bottom tabs. Each agent tab owns its own Zed `Editor`, `MultiBuffer`, prompt buffer, transcript state, prompt anchors, subscriptions, and render state.

This intentionally differs from `tau-cli`'s terminal session model. It lets Zed preserve draft text, cursor/selection, scroll position, and editor state naturally per agent tab instead of manually copying those fields during switches.

## Zed editor primitives over terminal primitives

Where `tau-cli` uses terminal input and line-editing behavior, `tau-gui` should use Zed/GPUI editor primitives. This includes prompt editing, completions, selections, cursor state, buffer-backed outputs, and future richer views.

Completions should therefore be implemented through Zed editor completion providers rather than terminal completion UI, while preserving tau-cli's command and candidate semantics.

## Action outputs as GUI buffers

When the protocol sends `ActionOutput::EditorBuffer`, `tau-gui` should treat it as a real GUI editor buffer/view instead of only rendering it as terminal transcript text. Text-only action output can still render into the transcript.

## Status line hides session ids

Unlike `tau-cli`, `tau-gui` intentionally omits the `&session` chip from the status line. The GUI attaches to the harness/session for the current project, does not support session switching yet, and uses the status line space for agent tabs and model/parameter status.

## No session switching for now

`tau-gui` intentionally does not support session switching yet. It attaches to the harness/session for the current project and should avoid building session-switch UI until that becomes a deliberate GUI feature.

## Terminal-only events

Terminal-only events such as `TermBell` and `Osc1337SetUserVar` are intentionally ignored unless a concrete GUI notification or integration need appears.

## GUI affordances may differ from tau-cli rendering

Transcript content, event handling, tool lifecycles, status semantics, and command validation should converge with `tau-cli`. The surrounding presentation may differ when the GUI can provide a more native affordance, such as agent tabs, clickable controls, editor buffers, or richer diff/tool detail views.
