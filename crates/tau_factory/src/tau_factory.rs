//! Tau agent-factory extension.
//!
//! Drives tasks (an issue plus a patch) through their lifecycle, owning a
//! per-task jj workspace and the agent bound to it. Built as a tau harness
//! extension that lives in this repo (alongside the board UI) so its
//! experimental dependencies stay out of the upstream harness. Built
//! incrementally; for now this crate exposes the task model, its durable store,
//! the workspace mechanism, and the harness-extension run loop that drives the
//! `new`/`start` actions.

pub mod extension;
pub mod store;
pub mod workspace;

/// The task model lives in its own crate (`tau_task`) so the board UI can share
/// the exact wire type without pulling in the factory's durable-store
/// dependencies. Re-exported as `task` so in-crate paths stay unchanged.
pub use tau_task as task;
