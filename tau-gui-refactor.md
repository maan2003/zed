# Tau GUI refactor notes

This fork should become a small Tau-focused GUI for the Tau codebase in `~/src/tau`, not a full Zed application and not a generic editor. The goal is to keep the parts that make Zed's editor rendering and GPUI experience valuable, while removing product features, services, and extension surfaces that only make sense for Zed.

The source of truth for Tau behavior is `~/src/tau`: its harness, session model, protocol types, tool events, configuration, and existing terminal UX. This GUI should integrate with those concepts instead of inventing a separate agent runtime or parallel project model.

## Product shape

Tau GUI should feel like an Emacs-ish graphical shell for Tau: keyboard-first, buffer/pane oriented, text-heavy, and able to render rich inline UI where useful. It does not need the full IDE stack unless that stack directly supports the Tau interaction model.

The best analogy is not "Zed but for Tau" and not "a terminal emulator with buttons." It borrows from Emacs in GUI mode: mostly textual, command-driven, and buffer-centric, but with enough native GUI affordances to make inspection, navigation, completion, and rich results pleasant. This analogy is about interaction style, not scope. The initial product should be highly tied to Tau and should not try to become a generic Emacs-like environment, editor, or extension platform.

The GUI should preserve the speed and composability of a TUI while removing terminal constraints where they hurt Tau workflows: cramped panes, poor inline tool rendering, limited mouse inspection, awkward rich output, weak persistent visual state, and poor navigation across long agent sessions.

That means the editor is a rendering/input primitive, not the product. We should keep enough editor machinery to render text buffers, selections, blocks, creases, completions, hovers, inlays, and other local UI affordances. We should avoid keeping systems merely because Zed wires them into the editor today.

## Ideal target

The ideal Tau GUI is a native desktop command environment for running Tau sessions. It should make Tau feel like a focused agent workbench rather than a chat app, an IDE, or a general-purpose programmable editor. The first version should be explicitly Tau-specific: every buffer type, command, completion provider, pane, and rendered block should exist because it helps operate Tau.

A good target experience:

- A main interaction buffer where the user types messages, commands, edits, and follow-up instructions.
- Multiple persistent buffers for sessions, logs, tool outputs, plans, scratch notes, diffs, search results, and help.
- Keyboard-first navigation between buffers, panes, prompts, completions, and history.
- A minibuffer-like command/prompt area for quick actions, command search, path selection, model/session switching, and confirmations.
- Rich inline rendering for tool calls, diffs, diagnostics, approvals, markdown, structured data, and progress.
- Editor blocks or block-like widgets for non-text regions: tool cards, plan steps, command results, file previews, expandable logs, and status panels.
- Completion everywhere it matters: slash commands, paths, symbols/context references, session names, tools, models, prompt snippets, and command names.
- Hovers/popovers for details without losing place: tool metadata, file paths, diagnostics, token/cost info, command docs, and error explanations.
- A small amount of mouse support for inspection, selection, pane resizing, and clicking obvious actions, without making mouse usage the primary workflow.
- No generic IDE obligations: no full debugger, no full LSP product, no extension marketplace, no Zed collaboration, no project panel unless Tau specifically needs it.

The target should be "Emacs GUI mode for Tau": a native, textual, deeply keyboardable Tau workbench with graphical affordances where they pay for themselves. It should not become generic Emacs. It should not become a generic editor. It should not attempt to expose every Zed feature. It should not become a browser-style chat UI.

For the initial version, avoid designing generic abstractions unless Tau immediately needs them. A generic buffer API is less important than excellent Tau session buffers. A generic command system is less important than fast Tau commands. A generic extension story is less important than first-class rendering for Tau tool calls, plans, diffs, shell output, and sub-agent activity.

A successful first app would have only a few core concepts:

- **Buffers:** named, persistent views of Tau-related text or structured output.
- **Windows/panes:** spatial layout for buffers, with simple splitting and focus movement.
- **Minibuffer/command prompt:** a universal place for commands, completions, and transient input.
- **Session model:** one or more Tau conversations connected to Tau's harness/session APIs.
- **Inline result blocks:** rendered tool calls, diffs, command output, errors, and summaries inside buffers.
- **Completion providers:** Tau-native providers, not necessarily LSP-backed providers.

Everything else should justify itself against this target. If a feature is only useful for hypothetical non-Tau apps, it should wait.

## What should stay

### Platform and UI foundation

Keep GPUI and the platform crates needed for a normal cross-platform desktop app:

- `gpui`
- `gpui_platform`
- `gpui_linux`
- `gpui_macos`
- `gpui_windows`
- `gpui_wgpu`
- GPUI support crates such as macros, shared strings, utilities, and tokio integration

Do not make this Linux-only just to delete crates. Cross-platform native GUI is part of the value proposition.

### Text and rendering primitives

Keep the low-level text model and rendering support:

- `text`
- `rope`
- `sum_tree`
- `multi_buffer`
- `language` / `language_core` as long as syntax, highlighting, anchors, buffer snapshots, and query support depend on them
- `theme`, `theme_settings`, `syntax_theme`
- `ui`, `ui_input`, `component`, `icons`, `file_icons` where they support editor/UI rendering

These are the real reusable primitives.

### Editor capabilities worth preserving

Keep editor features that map naturally to Tau UI:

- selections, cursors, movement, scrolling
- soft wrapping and display-map coordinate translation
- editor blocks / custom rendered regions
- creases/folds if they help hide verbose tool output or render summaries
- inlays and inline annotations
- hover popovers
- completions and completion providers
- code/action menus if they can become Tau command/action menus
- split editor/view primitives if they help pane layout

Completions are especially worth keeping. Tau will likely want command completion, path completion, symbol-ish completion, model/tool completion, slash command completion, and inline suggestions.

## What should go

### Zed product shell

Remove crates that implement Zed as a complete application rather than a reusable UI foundation: title bars, sidebars, panels, project panels, settings UI, onboarding, update UI, feedback, command palette integrations, and similar product shell features.

Tau should build its own app shell around Tau concepts instead of dragging Zed's workspace/product structure along forever.

### Collaboration and cloud product features

Remove collaboration, calls, channels, cloud-specific account flows, cloud model provider UI, and sharing infrastructure. Tau does not need Zed collaboration semantics for a local Tau GUI.

AGPL/collab code should stay out unless there is an explicit reason to accept those obligations.

### Generic IDE subsystems unless proven necessary

Treat these as candidates to remove or isolate behind small interfaces:

- LSP orchestration
- DAP/debugger support
- full project/worktree management
- task discovery and runnable extraction
- git UI and blame UI
- extension host / WASM extension API
- language-server installation and language onboarding

Some data types from these systems may be useful, but keeping whole subsystems because one editor method mentions them is the wrong direction. Prefer stubbing or splitting the editor surface so rendering can compile without the full IDE backend.

### Tests, benches, and fixtures from Zed

For this experiment, Zed's editor tests and benches are useful as reference material but should not drive the fork's dependency graph. They pull in large fixtures, language grammars, LSP test harnesses, and property-test dependencies.

If we need tests, write smaller Tau-specific regression tests around the behaviors we actually use.

## Decision philosophy

1. **Keep primitives, remove product.** A crate stays if it is a reusable rendering/input/text primitive. It goes if it is Zed product behavior.
2. **Keep cross-platform support.** Do not delete macOS/Windows/WebGPU platform crates just to reduce count.
3. **Prefer source-level decoupling over dependency hoarding.** If `editor` depends on `project` or `lsp` for a small feature, split or stub that feature instead of keeping the entire subsystem.
4. **Preserve Tau-relevant UX affordances.** Completions, hovers, blocks, inlays, panes, and rich text rendering matter more than IDE features like debugger integration.
5. **Avoid generic editor ambition.** We are not rebuilding Zed or Emacs. We only need the editor pieces that make Tau interaction better.
6. **Cut in verified layers.** After each trim, run at least `nix develop --command cargo check -p editor` until we have a Tau app crate to check instead.
7. **Document intentional stubs.** If a feature is disabled to break a dependency, leave a short comment explaining whether it is permanently out of scope or temporarily waiting for a Tau-native replacement.

## Practical next cuts

The next meaningful work is source-level isolation of `editor` from these heavy systems:

- `project`
- `workspace`
- `lsp`
- `dap`
- `extension`
- `git`
- `db` persistence

Do not remove completions while doing this. Instead, make completion providers local/editor-facing so Tau can provide completions without pulling in Zed's full LSP/project stack.
