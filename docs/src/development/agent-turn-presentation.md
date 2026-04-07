# Agent Turn Presentation

## Goal

Agent threads are becoming tool-dense. A single user request may produce many tool cards, terminal runs, diffs, and intermediate assistant outputs before the final assistant response appears.

This hurts readability in two ways:

- The visible relationship between the user's request and the assistant's final answer gets pushed apart.
- Long tool-heavy turns force the reader to scroll back through internal work just to recover conversational context.

The goal of this concept is to preserve context without hiding useful work entirely.

## Decision

Treat the user turn, not the individual tool call, as the primary reading unit.

Within a turn:

- The user message remains visible.
- The last assistant text remains visible.
- Everything between them is treated as the turn's "work" region.

The work region is presented differently depending on recency:

- Previous turns collapse their work region.
- The latest turn keeps its work region visible, but constrained to a fixed height.

This keeps the conversation legible while still preserving a place for internal work to appear.

## Collapsed Historical Turns

For older turns, the work region is replaced by a compact summary row placed between the user message and the last assistant text.

The current simplified summary is:

- `n tools`

This summary row also serves as the expand affordance for the hidden work.

The intended reading order is:

1. User message
2. Work summary
3. Final assistant text

This preserves the narrative shape of the turn:

- what the user asked
- that the agent did hidden work
- what the agent concluded

## Latest Turn

For the most recent turn, the work region is not collapsed. Instead, it is shown in a fixed-height preview between the user message and the last assistant text.

This gives the reader two things at once:

- enough live visibility into what the agent is doing
- a stable amount of vertical space so the final assistant text does not get pushed far away

The latest turn should continue to benefit from existing thread-level follow behavior while generation is active.

## Why This Shape

This concept is intentionally turn-oriented rather than tool-oriented.

The problem is not that any single tool card is too large. The problem is that a turn made of many internal steps expands the thread so much that the conversational context is lost.

Making individual tool cards smaller would reduce local height, but would not restore the relationship between:

- the user request
- the hidden work
- the final assistant response

Treating the whole turn as the presentation unit addresses that directly.

## Tradeoffs

### Benefits

- Preserves conversational context in long, tool-heavy threads.
- Keeps the user request and final assistant answer visually near each other.
- Reduces vertical noise from intermediate work without removing it entirely.
- Gives the latest turn a live preview without letting it dominate the viewport.

### Costs

- Intermediate assistant messages become secondary to the final assistant text.
- Older turns become less transparent at a glance unless expanded.
- The UI shifts from exposing internal execution steps as first-class visible items toward presenting them as supporting detail inside a turn.

These tradeoffs are acceptable because the primary reading task is understanding the conversation, not auditing every intermediate action by default.

## Non-Goals

This concept does not aim to:

- summarize tool results semantically
- preserve expansion state across reloads
- expose every per-tool metric in the collapsed summary
- change the meaning of the final assistant response

It is purely a presentation change intended to improve readability and context retention.
