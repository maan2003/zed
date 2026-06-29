# Rho GUI

`rho-gui` is the terminal GUI for Rho. It keeps the fast, keyboard-first shell
that came from Tau, but the product boundary is Rho: Rho protocol, Rho daemon,
Rho agent state, and Rho UI semantics.

## Philosophy

- Preserve the good shell. The editor, prompt flow, transcript styling, keyboard
  feel, and tool presentation should remain as polished as Tau's UI unless Rho
  has a better reason to differ.
- Move semantics upstream. The daemon/protocol should project agent state into
  UI-shaped semantic data before it reaches the GUI. The GUI should not infer
  meaning from raw provider replay, full context blocks, or tool output blobs.
- Optimize for bandwidth and UX, not "simple clients." A smart GUI is fine, but
  the wire format should carry compact semantic deltas that let the GUI render a
  good experience without receiving unnecessary data.
- Do not render data just because it exists. Tool results, provider internals,
  call ids, schemas, and full outputs are usually not user-facing transcript
  content. Send and show summaries/previews only when they improve UX.
- Prefer Tau-compatible presentation for now. While Rho's own UX language is
  still forming, Tau's transcript conventions are the baseline: user prompts,
  assistant text, tool lines, progress chips, and restrained system notices.

## Boundaries

### `rho-daemon`

The daemon owns runtime state and long-lived agent behavior:

- agent creation, loading, cancellation, and execution;
- subscriptions to changing agent state;
- projection from internal Rho state into UI protocol frames;
- deciding what information is safe/useful to send to clients.

The daemon should not send full internal state just because it is available. It
should send the smallest semantic UI state that supports good rendering.

### `rho-ui-proto`

The UI protocol is the semantic boundary between daemon and GUI.

It should contain UI concepts such as:

- transcript blocks;
- streaming items;
- tool invocations;
- tool status;
- bounded previews or summaries;
- compact diffs for in-place updates and append-only streaming.

It should avoid exposing:

- full provider responses;
- complete tool outputs by default;
- tool schemas;
- implementation-only ids as user-visible text;
- raw context details that force every client to rediscover UI semantics.

Tool calls are modeled as a single evolving `UiTool`, not separate "call" and
"result" transcript blocks. A tool result normally updates the existing tool's
status. If Rho later needs richer display, the protocol should add bounded,
semantic fields such as preview, diff summary, counters, or diagnostics rather
than shipping arbitrary full output.

Because semantic blocks can change in place, transcript diffs are not purely
append-only. The protocol must support:

- appending new blocks;
- updating existing blocks;
- truncating/resetting when history changes;
- streaming text/item diffs for high-frequency provider output.

### `rho-gui`

The GUI owns presentation:

- GPUI/editor integration;
- prompt editing and keyboard behavior;
- transcript rendering;
- theme/style mapping;
- status line and navigation;
- converting semantic protocol blocks into styled terminal spans.

The GUI should not:

- parse full tool output to discover status;
- render call ids as user-facing labels;
- fetch or infer provider internals;
- receive large data just to throw it away.

It may be sophisticated. "Simple client" is not the goal. The goal is a
responsive UI fed by a bandwidth-conscious semantic protocol.

## Current migration stance

`rho-gui` still reuses Tau UI infrastructure and crates in places. That is a
temporary implementation detail, not a product boundary. When choosing between
renaming everything and preserving working behavior, prefer preserving behavior.
Cut seams from the backend/protocol outward:

1. Rho daemon and protocol own state and semantic projection.
2. Rho GUI renders from Rho protocol frames.
3. Tau-specific compatibility code is removed or renamed as the corresponding
   Rho concept becomes real.

Visible branding should be Rho. Internal Tau names may remain temporarily when
they are transitional dependencies or copied shell infrastructure.
