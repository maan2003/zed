# tau-gui parity TODO

Remaining work to bring `tau-gui` closer to `tau-cli` while preserving GUI/editor-buffer behavior.

## Event handling and rendering

- Add provider turn-stat latency/cumulative-latency parity; basic token/cache stat rendering from `ProviderResponseFinished.usage` is in place.
- Finish deeper delegate parity: basic `ToolDelegateProgress` display updates and delegated-agent learning are in place; remaining work is exact tau-cli activity/tool-summary semantics.
- Finish full tool-call state parity like tau-cli for out-of-order events and tool summaries; background placeholders/results/cancel status accounting is in place.
- Do not support session switching for now; do not spend parity work on `UiSwitchSession`/session-switch UI beyond safe no-op/reset behavior if events are observed.
- Ignore terminal-only events (`TermBell`, `Osc1337SetUserVar`) in tau-gui unless a real GUI-specific notification need appears.

## Commands and interaction

- Finish command completion/parsing parity for `/set`, actions, shell shortcuts, and any remaining slash commands; root slash, `/agent`, `/model`, `/role`, and `@agent` mention completions now use Zed/editor completion primitives.
- Mirror tau-cli command validation and help text more closely for `/agent`, `/model`, `/tree`, `/compact`, `/cancel`, and shell commands; `/role` selection/delete/settings parity is in place.
- Add support for queued prompt recall from the GUI, including a keybinding or command that emits `UiRecallQueuedPrompt`.
- Improve `/agent` listing/switch UX beyond plain text once GUI affordances exist.
- Add action-command invocation support if tau-cli exposes extension actions as slash commands in the current protocol.

## GUI-specific behavior

- Treat `ActionOutput::EditorBuffer` as a real GUI editor buffer/view instead of only transcript text. `ActionOutput` is extension UI-action output: either plain text or an editor buffer payload with title/text/editability.
- Preserve transcript rendering parity while allowing richer GUI affordances for tool details, diffs, and action outputs.
- Keep prompt draft restoration and cursor/scroll behavior polished when prompts are recalled or events rewrite live blocks.

## State tracking

- Track prompt ids, live responses, pending tools, background placeholders, and delegate children with the same lifecycle assumptions as tau-cli.
- Track per-agent activity like tau-cli; per-agent context usage/status restoration is in place.
- Track available models/roles/efforts/verbosities/thinking-summary options for status and deeper completion parity; role/model names and static setting values are now exposed through prompt completions.
- Track extension action schemas for completions and command invocation.

## Verification

- Add focused tests for transcript mutations: live response replacement/removal, tool progress replacement, prompt recall draft replacement, and terminated prompt cleanup.
- Add protocol fixture/replay tests comparing key tau-cli event sequences against tau-gui transcript output where rendering semantics should match.
- Manually exercise against `~/src/tau` daemon for: new agent creation, queued prompts, cancellation, compaction, shell commands, tool background results, delegate progress, and extension restart/error events.
