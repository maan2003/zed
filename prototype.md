# Tau GUI prototype

This prototype is a small experiment to find out whether Zed/GPUI editor primitives can support a Tau shell-like GUI. The goal is not to recreate the full Tau CLI or the full Zed app shell. The goal is to validate the core interaction model: a keyboard-friendly, editor-backed Tau session buffer that feels comparable to the CLI for reading, scrolling, selecting, typing, and following live output.

## Scope

Create a minimal `tau_gui` app crate as the entrypoint. This repository currently does not contain the full Zed app entrypoint, only GPUI/editor/library crates and examples, so the Tau GUI should have its own thin app shell.

It is fine to copy/adapt narrowly useful code from upstream Zed or Tau CLI when it reduces initial cost. Avoid importing the whole Zed product shell unless the editor primitives force it.

## Harness connection

The GUI is attach-only for the first prototype.

Startup behavior:

1. Use the GUI process current working directory as the project root.
2. Find an existing Tau harness daemon for that directory.
3. If no daemon exists, hard exit with an error.
4. Connect to the daemon Unix socket.
5. Send `Message::Hello` with `client_name = "tau-gui"` and `client_kind = Ui`.
6. Send `Message::Subscribe` for the same broad event families the CLI uses: `ui.`, `session.`, `provider.`, `tool.`, `extension.`, `harness.`, `shell.`, and probably `term.` if needed.
7. Read frames, peel `Message::LogEvent` into the inner `Event`, and render a simplified transcript.

Use Tau crates as git dependencies from the Tau GitHub repository rather than local path dependencies or copied protocol definitions. It is acceptable to depend on `tau-harness` for runtime-dir discovery at first. If that dependency graph becomes a problem, extract or reimplement only the small `$XDG_RUNTIME_DIR/tau/*/{tau.dir,tau.sock}` discovery later while continuing to use `tau-proto` for the wire protocol.

## Editor-first UI model

Everything user-facing in the main interaction surface should be an editor. There should be no separate prompt input field in the first prototype.

The main buffer is a single editor-backed shell buffer containing both history and the current draft:

```text
[read-only transcript/output]
[streaming assistant/tool/status output inserted here]
[draft_start anchor]
[user editable draft]
```

Core behavior:

- The transcript/output region is read-only.
- The current draft region is editable.
- Track the draft start with a real editor/buffer anchor or mark, not by parsing textual prompt markers.
- Remote output inserts immediately before `draft_start`, so the user’s draft remains at the editable tail and moves down as output streams.
- The draft remains editable while a response is streaming. Tau already handles queued prompts.
- Submitting sends only the draft range, not any visible prompt marker.
- After submit, the submitted draft becomes read-only transcript/history and a fresh editable draft starts at the end.

Initial submit binding can be `Ctrl-Enter`. Plain Enter should keep normal editor behavior while testing whether this can feel like an editor-native shell buffer.

## Rendering target

Use Tau CLI as the broad visual/behavioral reference, but do not try to match it exactly in the first spike.

Initial rendering should be deliberately simple:

- show submitted user prompts with a visible prompt marker/style;
- show assistant response text from provider update/finished events;
- show compact tool/status/shell events as simple distinct transcript lines or blocks;
- keep the buffer scrollable, selectable, and copyable;
- prioritize discovering missing editor primitives over feature completeness.

Prompt markers such as user/Tau markers should preferably be editor decorations, inlays, blocks, or another non-submitted visual primitive. Do not emit Markdown/control markers such as heading prefixes into the buffer merely so rendering can hide or reinterpret them later; if a message should look like a heading/card/role label, keep that as semantic range metadata plus editor decoration. If that is too expensive for the first spike, textual markers may be used temporarily, but the prototype should record that as an editor primitive gap rather than treating text parsing as the design.

## Future requirements

Vim mode is not required for the first prototype, but it is likely important later. The current fork appears to retain `vim_mode_setting` and editor hooks/comments for Vim integration, but not the full Vim mode crate. At some point we may need to revive or recopy the removed Vim layer from upstream Zed and adapt it to the Tau shell buffer model.

## Non-goals for the first prototype

- Starting or supervising the Tau harness daemon.
- A daemon picker or global session browser.
- Full Tau CLI feature parity.
- Slash commands.
- Role/model/settings UI.
- Provider auth UI.
- Shell shortcut UI.
- Project panel, generic file editing, LSP, DAP, git UI, extensions, collaboration, or Zed product shell features.

## Questions the prototype should answer

- Can the Zed editor support a single-buffer shell model with read-only history and editable tail?
- Can remote output stream into the buffer before the draft anchor without disrupting typing, selection, or scroll behavior?
- Are editor anchors/marks sufficient to track draft boundaries robustly?
- What primitives are missing for prompt markers, read-only ranges, append-before-draft behavior, and block-like Tau output?
- How much of the existing `editor` crate can be used without pulling in Zed workspace/project/LSP/product dependencies?
