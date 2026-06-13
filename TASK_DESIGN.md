# Tau Agent Factory

This document describes the parallel-agent workflow that tau-gui is built toward after CLI parity. The premise: most of the remaining gains in agentic coding are at the multi-agent level, so tau-gui's job is to let one human drive a fleet of agents through many tasks at once without drowning in coordination.

The design is deliberately small at its core: a task with two independent axes, a rule that agents are responsible for the artifacts they author while humans usually annotate and approve, and a rule that artifact ownership determines where changes are routed. Everything else is a consequence of those three.

## Model

A project has a list of **tasks**. A task is an issue plus a patch — the problem and the work toward solving it.

Tasks move through stages:

```
init → exploration → implementation → done
```

Merge readiness is not a stage. When the implementor judges the work done, it raises the task's **attention** to `review` — "ready" is not a property the task holds but a request for the human, the equivalent of marking a PR ready for review. The human reviews and either requests changes (a `review:` comment in the relevant file's comment syntax, bouncing it back) or **delegates the merge** — "mostly looks good, merge it when you think it's ready" — handing the final call to the agent so the human need not be the last step. A delegated agent finishes any remaining work and submits the task to the merge queue itself; if it becomes unsure it simply declines to use the delegation and defers back to the human. Only the human withdraws a delegation.

Transitions can also go backward (a *pull-back*): implementation drops to exploration when a decision turns out wrong, and a failed merge-queue check bounces a queued task back to active implementation. Merge is the one transition that is never speculative — everything before it is disposable, master is the only architectural state.

Two **independent axes** describe a task:

- **Stage** — where the work is (the pipeline above).
- **Attention** — who it is waiting on, and why (`decision`, `question`, `review`).

These are not the same thing, and conflating them is how issue trackers grow junk states like "waiting-for-info." A task in `implementation` that hits a question for the human has not moved backward; it is still in implementation, just *flagged*. The board groups by either axis on demand.

## Artifacts

Each task owns durable artifacts, kept separate from the agents that produce them. Agents are working memory; artifacts are permanent memory.

- **Issue** — the problem statement. Markdown.
- **Decision record** (`DECISIONS.md`) — exploration's output and the most important object in the system. It is the review object at gates, the fresh-context prompt for the implementor, the assumption ledger for speculation, and the landing pad for pull-backs. Markdown file committed on the task's branch, so it appears in the review diff. Initially records are migrated into permanent docs at merge; later the transform may distinguish permanent architectural records from task-history archives.
- **Acceptance checklist** — exploration's second output: how we will know the task is done. A checklist, not executable specs.
- **Patch** — the work itself, on the task's branch in its own jj worktree.

### Decision record conventions

The record format is **arbitrary markdown to start**. Structure will be promoted later, once real records show which conventions recur; it is likely project- and task-specific. Exactly one convention is mandatory from day one:

- Every decision entry uses a stable minimal marker: `[decided]` for human-made or ratified decisions, or `[assumed]` for agent-speculated decisions awaiting ratification.

This single distinction is what makes speculation auditable. Without it the review gate has nothing to grip.

The record must **stand alone**: an implementor works from it with fresh context and no access to the exploration transcript. Prototypes built during exploration are *evidence cited in the record*, not drafts of the product — the explorer may carry specific fragments forward only with an explicit reason ("this handshake is subtle and verified, reuse it"), and biases toward not carrying anything. If the record cannot stand alone, exploration is not finished; that gap should surface loudly rather than be papered over by handing the implementor the spike.

## Speculative execution

Agents may jump stages on their own. A simple task does not wait at the exploration gate for a human — the agent implements directly. But it still produces the artifacts: a decision record with its self-made choices marked `[assumed]`. The human can **pull the task back** at any point.

This is branch prediction. Agent compute is cheap; the human is the expensive stall. Speculate freely, but:

- **Merge is the commit point** — never speculative. A speculative task's merge gate is *ratify the decisions*, not re-review the code.
- **Reviewing decisions post-implementation is often richer than pre** — each decision is seen next to the diff it produced.
- **Pull-back rate is the misprediction rate.** Tracked per label/area, it tunes how much autonomy a class of task gets — learned from history, not hand-configured.
- A closed speculative patch loses almost nothing: the issue and record survive, and "we tried X, here's why it was rejected" is itself evidence. Treat speculative output like a drive-by contributor's PR — close it without guilt; a rejected implementation *demotes* to exploration evidence rather than being deleted.

### Trivial changes: don't split contexts

The stages are not the overhead — a stage that produces an empty artifact costs nothing. The cost is **splitting work across fresh-context agents**: the handoffs, the re-reading, the separate spawns. You only pay that to buy something specific — a *clean context* (an implementor working from the ratified record instead of being anchored on exploration's mess).

So trivial changes do not need a separate pipeline; they need the *same* pipeline run **without context splits**. One agent runs init → implement in a single context and produces an honestly-empty decision record (there were no decisions). Merge still goes through the human — but for a truly trivial task the human can set `merge_delegated` from the start, so that one agent carries it to the merge queue itself with no round trip. Context-splitting is the single knob: insert a boundary at a gate when clean context is worth the handoff, skip it when it is not.

## Agents

### Ownership follows artifact mutability

Agents are persistent and wakeable. They are kept indefinitely by default and can be woken at any time; keeping their transcript around is cheap and useful for forensics. What changes over time is not whether the agent exists, but whether it is the **active owner** of an artifact.

- **Explorer** — owns the decision record while the task can still change design direction. Ratification advances the task; it does not retire the explorer. If implementation uncovers a re-decision, the same explorer can be woken.
- **Implementor** — owns the patch. Woken, not replaced, to handle conflicts and the human's review comments, because it already has the context. An implementor is an agent bound to the task's worktree (tau already binds agents to shell working directories).

Anything that mutates an artifact routes to its active owner — and *which* agent that is falls out of artifact ownership rather than a special case. A patch-level conflict mutates the patch, so the implementor wakes. A conflict large enough to invalidate a decision mutates the *record*, so the explorer wakes; the implementor makes that call ("this is a re-decision, above my pay grade"). A frozen artifact no longer needs an active owner, but the historical agent can still be woken if its context is useful.

### Standing roles (own no task)

- **dank** — the project-manager agent. Always active, because its artifact is the backlog, which never freezes. Authors tasks from conversation with the human, triages agent-filed issues, composes sessions, sets prefetch priority. dank manages the factory but never works in it — no code, no exploration, no reviews. Its power is being the only agent with the whole map and no stake in any patch. The scratch chat is just talking to dank.

## Merge queue

The **merge queue** is mechanical, not an agent: it serializes tasks, rebases each onto master, applies the merge transform, runs checks against the transformed candidate, and either lands that exact patch (→ done) or bounces it back to active implementation with the failure as feedback (a pull-back triggered by a failed check).

A task reaches the queue only when its owning implementor submits it, and the implementor may do that without the human only if the merge is **delegated**. Delegation is forward-looking trust — "merge it when you think it's ready" — granted over a roughly-current state of the patch and held as a standing authorization. If finishing the work stays within what the human would expect, the implementor proceeds and submits. If something turns up that the human plausibly would not have signed off on — a conflict that shifts the design, a check that forces a real change — it declines to use the delegation and raises `attention = review` to defer to the human; the delegation itself remains in place. The implementor makes that call. Withdrawing a delegation is the human's prerogative alone — an agent can exercise delegated trust but never revoke it.

Invalidation detection — noticing that a merge invalidates another task's *exploration* — is future scope (see non-goals).

The checks are CI: tests and lint. Code review is the human's, done when the task raises `attention = review`.

Merge itself is a **transform**, not a bare branch merge. In order: strip the `review:` / `agent:` comment threads from the code using each file's comment syntax, migrate or archive the decision record and design files, run checks, then merge the checked tree to master and flip the task to done.

### Authorship split

**Humans annotate and approve; agents author.** Direct human edits are allowed when useful, but they become input for the owning agent to digest and take responsibility for. The default path remains annotation, in both directions:

- Feedback is **in-document comments**: the human writes `review:` directly in the record or the code, using the file's native comment syntax; the agent replies in the same file with `agent:` and never deletes the original human comment. This pins the conversation in the file the agent is already editing — so the agent cannot lose it across rewrites — and gets version control and reversibility for free, with no comment-anchoring machinery to build. Files with no safe comment syntax fall back to a record comment that points at the file/range. Resolved threads are migrated into the decision record or task archive at merge; code stays clean.
- **Ratifying an assumed decision** is the same annotation family: the human writes `approved` on the entry in the decision record, and the agent flips its marker from `[assumed]` to `[decided]`. Contesting it instead is a `review:` comment that bounces the work back. This is per-entry ratification of the record — distinct from **delegating the merge** (pre-authorizing the implementor to land the patch when ready), which is a task-level gate (see Merge queue), not an annotation.
- Mid-flight steering (interjecting in an agent's transcript) is a decision too, and must be **distilled by the agent back into the record** — otherwise the highest-quality decisions, the ones important enough to grab the wheel for, are the only ones with no durable trace.

## State and storage

The machine-readable task state is owned by the **factory extension** (a harness extension — a separate process speaking events), not by tau-gui. The CLI and every agent see the same board; clients send commands, the extension validates them through transition methods, and everyone renders from the emitted event stream. The store is an implementation detail behind that boundary.

A project has well under ~10k tasks, so there is no scale problem and no need for a query engine. The whole task model lives **in memory as ordinary Rust structs/enums**, with the dependency graph walked in plain Rust (no recursive SQL), persisted to a **redb** durable log — pure Rust, no FFI, transitions as methods on a state machine.

```rust
struct Task {
    id: TaskId,
    title: String,
    stage: Stage,                 // stored explicitly; stage and artifacts can
                                  // legitimately disagree under speculation
    merge_delegated: bool,        // human-set/cleared only; implementor may submit to the queue or defer back
    attention: Option<Attention>, // why the task wants the human; drives attention-mode grouping
                                  // the implementor signals "done" by raising attention = Review
    priority: i32,
    worktree: Option<PathBuf>,
    implementor: Option<AgentId>, // the assigned implementor, if any
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum Stage { Init, Exploration, Implementation, Done, Closed }

enum Attention { Decision, Question, Review }

struct Dep { other: TaskId, kind: DepKind }
enum DepKind { Blocks, SpawnedFrom }
```

The dependency graph is load-bearing: agent-filed tasks link back to the task that spawned them (`SpawnedFrom`), and `Blocks` edges drive ready-work and invalidation queries. "What can I work on now" and "what does this merge endanger" are both graph walks over the in-memory model.

The implementor signals it is done by raising `attention = Review` — "ready for review" is an attention value, not a stored flag. The merge view is just the attention-grouped board filtered to `Review`. Delegating the merge flips `merge_delegated` and clears the attention — the implementor now holds the merge decision and submits to the queue when ready. When unsure it leaves the merge unsubmitted and re-raises `attention = Review` to defer to the human, without touching `merge_delegated`; only the human sets or clears that flag.

### Transitions

Discipline lives in the **transition machinery**, not in habits or prompts. Running the workflow by hand fails: every transition is several manual acts (make the worktree, start or wake the right agent, give it the right role, hand it the record, remember where everything is), and discipline erodes because each act is skippable. The factory makes each transition a single state-machine method that performs the rest mechanically and emits an event:

- `new` — create a task (init).
- `advance` — move to the next stage: create the worktree if needed, start or wake an agent bound to it with the right role prompt, hand over the ratified record. The approve gesture *is* `advance` out of a gate.
- `pullback` — move to an earlier stage; the rejected patch reclassifies as exploration evidence.
- `flag` / `unflag` — set or clear the attention axis.

The convention stops being something the human maintains and becomes something that happens to them.

### Events

Every accepted command emits an event on the harness stream. The board renders from these; dank and the merge queue trigger on them. The event vocabulary (task created, stage changed, attention changed, merge_delegated changed, implementor assigned, active owner changed, dependency added) is the contract both the extension and tau-gui build against.

## UI surfaces

The surface set is intentionally minimal, and every surface is a projection of files-in-worktrees plus harness events — so the CLI sees the same truth and tau-gui is a better lens, never a separate world. tau-gui's editor-first bet pays off here: the artifacts are all text, so the multi-agent UI is "good ways to read and edit text in worktrees," which is what an editor is.

### Adaptive board / rail

One component at two widths — the board and the agent rail are not two things.

- **Expanded**: full board in the main pane — the home screen and session start. Each task is one line: stage glyph, attention flag, freshness, implementor activity. magit-style structured text buffer with keyboard navigation, not a kanban of cards.
- **Collapsed**: the same list as a rail beside a task view, compressed to glyphs, focused task highlighted in place.

Entering a task collapses board → rail and opens the task view; escape re-expands where you were. It behaves like a fisheye lens over the task list, not two screens with a transition.

Two **grouping modes**, toggled by a key: **by stage** ("where is the work") and **by attention** ("what is waiting on me," leverage-ordered). The merge view lives here — `attention = review` shows exactly what is waiting for your review before you delegate the merge. The mode persists through collapse, so the rail keeps answering the question last asked. The attention mode replaces a dedicated inbox — a collated multibuffer inbox is a pure projection over the same state and a worthwhile optimization only if navigation pain appears at high task counts.

The board pane is also **dank's split view**: dank's chat on one side, the board (dank's artifact) on the other, so the human watches rambling get distilled into structured backlog in real time — the same split-view pattern as the exploration view.

### Stage-adaptive task view

One screen whose layout follows the task's stage, because each stage has a different primary medium:

- **Exploration** — chat-primary. Transcript + editable draft tail on one side (today's tau-gui shell buffer), the live decision record growing on the other. Freeform chat is how shared understanding is built; watching the distillate appear is the earliest possible signal that the agent misunderstood.
- **Implementation** — artifact-primary. Decision record + checklist + diff; the agent runs mostly unattended, transcript demoted to a toggle for steering or forensics. Once the implementor raises `attention = review`, the task shows the review layout: diff and record with the `review:` comment threads inline and the **delegate-merge** action — "merge when ready" — that hands the final call to the implementor.

Reviewing diffs inside a real editor (go-to-definition, references, navigation during review) is a structural advantage no web PR view offers — and since review bandwidth is the ceiling on fleet size, that advantage is the point, not a convenience.

### Scratch chat

A bare chat bound to no task — quick questions, poking at the codebase, and the place where "this should be a task" gets filed. It is just talking to dank, so there is no orphan agent and no handoff when a scratch conversation becomes a task.

## Implementation order

Phases are ordered so design risk is retired in the cheap phases and the expensive GUI work builds against validated conventions. The discipline engine and the minimal board ship together as one vertical slice, with a single milestone: **one task driven through its entire lifecycle with no manual bookkeeping.**

1. **Factory extension** — the in-memory Rust task model over redb, the transition methods (each doing full mechanical setup), and the event vocabulary. Role prompts encode the per-stage conventions; agents receive them automatically.
2. **Board in tau-gui** — the agent rail widened into the task list, driven by extension events; two grouping modes; expand/collapse. Enter opens today's transcript screen, unchanged.
3. **Record panel** — the task's `DECISIONS.md` as a side buffer in the task view (a file in the worktree, followed live). This yields the exploration split almost for free, and the `review:` / `agent:` convention works immediately because it is just text in the file.
4. **Merge queue** — the implementor raising `attention = review`, the human reviewing and delegating the merge (pre-authorizing), the implementor submitting to the queue when ready or deferring back when unsure (only the human withdraws a delegation), and the mechanical rebase-transform-check-land loop (strip comments, migrate or archive record, run checks, land the checked tree).
5. **dank as board owner** — dank authors the backlog in its split view.

This mostly bypasses the CLI-parity backlog rather than depending on it — the factory needs agent display names and a working role/model selector, but not thinking-block rendering, prompt history, or most of the rest. Parity items are pulled in as phases demand them.

## Non-goals (for now)

- A dedicated review agent (automated pre-review / compliance lens diffing the patch against the decision record) — the human reviews for now; add later.
- Merge-queue invalidation detection (noticing a merge invalidates another task's exploration) and automated conflict resolution.
- The multibuffer inbox as a distinct screen (attention-mode board replaces it until proven insufficient).
- Structured/schematized decision records (markdown until conventions are observed).
- Executable acceptance criteria (checklists only).
- Exploration tournaments (N competing records, possibly different models) — future scope.
- A cross-task field-notes / shared-memory layer — orthogonal, part of a later "agent factory."
- Post-merge defect-to-assumption provenance tracking — layers on after the core proves out.
