# Tau Tasks (simple)

A trimmed version of [TASK_DESIGN.md](./TASK_DESIGN.md). The full document designs a
review-throughput machine — a speculative fleet driven by one human. This one designs for
the bottleneck we actually have at **N=3–5 agents: legibility**. Today, agents are anonymous
tmux tabs; the real failure is losing track of who is doing what and what is waiting on you
(messaging the wrong tab). Everything whose only job is to raise review throughput is deferred
until we have operated a legible fleet of five and *felt* review throughput bind.

## Principle

Build for the bottleneck we are at (legibility at N≈5). Every piece we keep must earn its place
against that. Defer anything that only relieves review throughput — it relieves a constraint we
have not reached, and we cannot design it well from theory.

## Model

A project has a list of **tasks**. A task is an issue plus a patch — the problem and the work
toward it — bound to a named agent in its own worktree. A task is a *named, legible unit*; that
is the whole win at small N.

```rust
struct Task {
    id: TaskId,
    title: String,
    issue: String,                 // problem statement, markdown
    status: Status,                // carries the data that only exists in that state
    attention: Option<Attention>,  // orthogonal axis — what's waiting on the human, in any status
    created_at: Timestamp,
    updated_at: Timestamp,
}

enum Status {
    Open,                                          // backlog: issue only, no worktree, no agent yet
    Active { worktree: PathBuf, agent: AgentId },  // an agent is working it — both always present
    Done   { agent: AgentId },                     // merged — necessarily had an agent
    Closed { agent: Option<AgentId> },             // abandoned — None if closed straight from Open
}

enum Attention { Decision, Question, Review }
```

- **Status** is a thin lifecycle, *not* the `init → exploration → implementation → done`
  waterfall. One agent runs a task end-to-end in a single context. There is no fresh-context
  handoff and no "the decision record must stand alone" requirement — that justification only
  existed to support the handoff, and the handoff is gone. Each variant *carries the data that
  only exists in that state*, so illegal combinations are unrepresentable: an `Active` task always
  has a worktree and an agent, a merged `Done` necessarily had an agent, and only a task closed
  straight from the backlog (`Closed { agent: None }`) has neither. This is "discipline in the
  machinery" at the type level — the transition methods cannot construct an active-without-a-worktree
  task.
- **Attention** is the load-bearing axis and the best idea carried over: "what is waiting on me,
  and why." It is a legibility primitive — grouping the board by attention is what lets a human
  hold five agents in their head. "Ready for review" is the `Review` attention value (the agent
  raising its hand), not a separate stage. It stays a *flat* field rather than moving into `Status`
  precisely because it is orthogonal: a task is flagged independently of where it sits in its
  lifecycle, and that orthogonality is the whole two-axis point. The rule for what lives in a
  `Status` variant versus a flat field is exactly this — *existentially tied to the state goes in
  the variant; independent of the state stays flat.*
- **Speculation is not a feature here.** "The agent runs ahead between syncs" falls out for free
  from removing the stage gate; we do not build pull-back rates or branch-prediction machinery.

Agents are named and bound to their task's worktree. `SpawnedFrom` edges (an agent-filed task
points back at its origin) are cheap and worth keeping for forensics; `Blocks` / ready-work graph
queries are deferred until needed.

### Decision log

The agent keeps a short running log of what it decided, with one mandatory marker: `[assumed]`
for agent-made choices and `[decided]` for human-made or ratified ones. This is **not**
`DECISIONS.md`-as-handoff-prompt — its job is to be *the thing the human reads at a sync instead
of scrubbing the transcript.* It is deliberately lightweight, and it is the piece most at risk of
being busywork (see Open questions).

## Syncpoints and the backstop

The human and agent meet at **syncpoints**, from three sources:

1. **Human-placed** — written into the issue up front, for the forks the human can foresee
   ("decide the storage layout with me before you commit to an approach").
2. **Agent-raised** — the agent hits a fork it recognizes and sets `attention = Decision` /
   `Question`, or signals done with `attention = Review`.
3. **The backstop** — for the dangerous case: a wrong turn that neither party foresaw and the
   agent did not recognize *as* a decision. Sources 1 and 2 both depend on someone flagging the
   fork; the backstop must not.

> **The board is both the legibility tool and the backstop.** We do not build a forced hard
> interrupt. We make running state legible enough that a *glance* catches drift: each task line
> shows its latest decision-log entry and what the agent is doing, and scanning the board at N≈5
> is the periodic review that replaces the old stage gate. Board-glance answers "is anything
> obviously off / who needs me"; skim the decision log when you want to look closer.

This is a *weaker* backstop than a forced gate — a subtle architectural wrong turn may not surface
in one line — but it is the right cost/coverage trade at small N, and review cost amortizes into
cheap glances instead of expensive cold context reloads. A forced heartbeat sync is a future
option if we observe drift slipping through (see below).

## UI surfaces

Minimal, and built in this order:

1. **Board / rail** — build first; this is what fixes the tmux problem today. Named agents, one
   line per task (status glyph, attention flag, freshness, latest decision-log entry), grouped by
   attention. Magit-style structured text buffer with keyboard navigation. Collapses to a rail
   beside an open task and re-expands on escape.
2. **Task view** — transcript + the decision log + the diff. With stages collapsed there is no
   stage-adaptive layout to build; one screen.
3. **State machine** — mechanizes the bookkeeping: `new` (create task), `start` (make worktree,
   start/name an agent bound to it), `flag`/`unflag` (set/clear attention), `merge` (manual for
   now). One method per transition, each emitting an event on the harness stream. The board renders
   from those events, so the CLI sees the same truth.

## Milestone

> **Run 5 named agents on 5 tasks and never lose track of who is doing what or what is waiting on
> you.**

Operate that for real, then watch what binds *next* — and design the next phase against evidence,
not theory. The working bet is that the next bottleneck will not be review throughput; it will be
something we cannot name from here because we have never driven a legible fleet of five.

## Open questions

- **Does the decision log earn its place?** If at N≈5 we find we never read it — we just glance at
  the board and dive into the transcript when something is off — cut it, and the design gets
  smaller still.

## Future potential optimizations

Deferred, not rejected. Each relieves review throughput or adds autonomy — a constraint we have
not yet felt. The trigger to revisit any of these is *observing the specific pain it addresses*,
not finishing the milestone.

- **Merge queue + merge transform + merge delegation.** The whole review-throughput apparatus:
  the implementor submits to a queue, the human pre-authorizes ("merge when ready"), the queue
  rebases/transforms/checks/lands mechanically. Note the limit found in discussion: delegation
  relieves *latency* (round-trips on nits), not *bandwidth* (cold first-pass review), and harvests
  throughput mostly from the easy tasks that were never the bottleneck. Revisit when human review
  throughput is demonstrably the ceiling.
- **Speculation as machinery.** Pull-back rate, per-label autonomy tuning, branch prediction.
  When we want it back, gate it on **verification-to-generation cost** (cheap, objective oracle =
  safe to speculate — e.g. debugging against a repro), *not* on trivial-vs-hard. Never speculate
  on foundations unless they are invariant under the open decision — a wrong foundational guess
  poisons every branch built on it.
- **Forced heartbeat sync.** A periodic sync the agent cannot opt out of, as a stronger backstop
  than board-glance. Add only if we observe unflagged wrong turns slipping past the board.
- **`dank` as backlog owner.** A standing PM agent that authors/triages the backlog in a split
  view. At small N the human authors their own tasks; "scratch chat" stays just a chat.
- **Structured decision records.** Promote conventions in the decision log once real logs show
  which structure recurs. Markdown until then.
- **`Blocks` graph + ready-work / invalidation queries.** "What can I work on now" and "what does
  this merge endanger" as graph walks. Keep only `SpawnedFrom` until the graph is load-bearing.
- **Multibuffer inbox.** A collated review surface; a pure projection over attention state, worth
  it only if board navigation pain appears at high task counts.
- **Stage splits for clean context.** Re-introduce an explicit explore→implement boundary for a
  specific task *only* when a clean-context handoff is demonstrably worth the coordination tax —
  as an opt-in per task, not the default pipeline.
- Already-listed long-horizon items from the full doc: a dedicated review agent, exploration
  tournaments, cross-task shared-memory, post-merge defect-to-assumption provenance.
