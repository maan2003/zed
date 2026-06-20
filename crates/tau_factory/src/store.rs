//! Durable task store, backed by redb.
//!
//! This is the factory's source of truth and the reason it can be restarted
//! without losing track of the fleet: every mutation is committed to a redb
//! transaction before it returns, so a crash at any point recovers to the last
//! completed transition. The factory also emits task events to the harness for
//! the board UI, but recovery reads from *here*, not by replaying that stream —
//! one writer, one durable home, no divergence.
//!
//! State is materialized (one row per task), not event-sourced: the harness
//! already persists the event stream we publish, so an internal log would only
//! duplicate it, and a five-task fleet recovers far more simply by loading rows
//! than by folding a log.

use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::Path;

use chrono::Utc;
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition, TypeName};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::task::{Project, ProjectId, Status, Task, TaskId};

/// Tasks keyed by their id; the value is the [`Task`] itself, stored as CBOR.
/// The key is the raw `u64` (redb's built-in integer key, which sorts
/// numerically) rather than `TaskId`, since `TaskId` lives in another crate and
/// the orphan rule forbids implementing redb's traits on it here.
const TASKS: TableDefinition<u64, Cbor<Task>> = TableDefinition::new("tasks");
const PROJECTS: TableDefinition<u64, Cbor<Project>> = TableDefinition::new("projects");
/// Small key/value table for store-wide counters.
const META: TableDefinition<&str, u64> = TableDefinition::new("meta");
/// The next task id to hand out, under [`META`].
const NEXT_ID: &str = "next_task_id";
const NEXT_PROJECT_ID: &str = "next_project_id";

/// redb value adapter that stores any serde type as CBOR, so tables can be
/// typed over our structs (`Cbor<Task>`) instead of opaque byte slices.
///
/// `from_bytes` must be infallible per redb's `Value` contract, so a decode
/// failure can only panic. That is acceptable here because it means a row we
/// wrote ourselves no longer parses — on-disk corruption, an unrecoverable
/// invariant break, not a normal error path.
#[derive(Debug)]
struct Cbor<T>(PhantomData<T>);

impl<T> redb::Value for Cbor<T>
where
    T: Debug + Serialize + DeserializeOwned,
{
    type SelfType<'a>
        = T
    where
        Self: 'a;
    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> T
    where
        Self: 'a,
    {
        ciborium::from_reader(data).expect("task store row is corrupt: CBOR decode failed")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a T) -> Vec<u8>
    where
        Self: 'b,
    {
        let mut bytes = Vec::new();
        ciborium::into_writer(value, &mut bytes).expect("CBOR encode of task value failed");
        bytes
    }

    fn type_name() -> TypeName {
        TypeName::new(std::any::type_name::<T>())
    }
}

pub struct TaskStore {
    database: Database,
}

impl TaskStore {
    /// Open the store at `path`, creating the database file if it does not yet
    /// exist.
    ///
    /// Both tables are created here (opening a table in a write transaction is
    /// what creates it), so every later read can assume they exist — there is no
    /// "the store has never been written to" state for callers to handle.
    ///
    /// A database failure here, like everywhere in this module, panics: the
    /// factory cannot do its job without its store, so a failed store operation
    /// is unrecoverable rather than something callers should try to handle.
    pub fn open(path: &Path) -> Self {
        let database = Database::create(path)
            .unwrap_or_else(|error| panic!("opening task store at {}: {error}", path.display()));
        let write = database
            .begin_write()
            .expect("begin table-creation transaction");
        write.open_table(TASKS).expect("create tasks table");
        write.open_table(PROJECTS).expect("create projects table");
        write.open_table(META).expect("create meta table");
        write.commit().expect("commit table creation");
        Self { database }
    }

    /// Create a fresh `Open` task, allocating the next id. The id allocation and
    /// the row insert share one transaction, so a crash can never hand the same
    /// id out twice or leave a gap with no task.
    pub fn create(&mut self, title: String, issue: String) -> Task {
        let now = Utc::now();
        let write = self
            .database
            .begin_write()
            .expect("begin write transaction");
        let task = {
            let mut meta = write.open_table(META).expect("open meta table");
            let id = meta
                .get(NEXT_ID)
                .expect("read next id")
                .map(|value| value.value())
                .unwrap_or(1);
            meta.insert(NEXT_ID, id + 1).expect("write next id");

            let task = Task {
                id: TaskId(id),
                title,
                issue,
                status: Status::Open,
                attention: None,
                created_at: now,
                updated_at: now,
            };
            let mut tasks = write.open_table(TASKS).expect("open tasks table");
            tasks.insert(task.id.0, &task).expect("insert task");
            task
        };
        write.commit().expect("commit task creation");
        task
    }

    pub fn create_project(&mut self, name: String, path: std::path::PathBuf) -> Project {
        let now = Utc::now();
        let write = self
            .database
            .begin_write()
            .expect("begin write transaction");
        let project = {
            let mut meta = write.open_table(META).expect("open meta table");
            let id = meta
                .get(NEXT_PROJECT_ID)
                .expect("read next project id")
                .map(|value| value.value())
                .unwrap_or(1);
            meta.insert(NEXT_PROJECT_ID, id + 1)
                .expect("write next project id");

            let project = Project {
                id: ProjectId(id),
                name,
                path,
                created_at: now,
                updated_at: now,
            };
            let mut projects = write.open_table(PROJECTS).expect("open projects table");
            projects
                .insert(project.id.0, &project)
                .expect("insert project");
            project
        };
        write.commit().expect("commit project creation");
        project
    }

    /// Read a single task, or `None` if no task has that id.
    pub fn get(&self, id: TaskId) -> Option<Task> {
        let read = self.database.begin_read().expect("begin read transaction");
        let tasks = read.open_table(TASKS).expect("open tasks table");
        tasks
            .get(id.0)
            .expect("read task")
            .map(|value| value.value())
    }

    /// All tasks, in id order, as a lazy point-in-time snapshot. The iterator
    /// owns its read transaction (redb keeps it alive until the iterator drops)
    /// and borrows nothing from the store — `use<>` makes that explicit — so it
    /// can outlive the call and the store can be mutated while it is held. Used
    /// to rebuild the board on startup.
    pub fn list(&self) -> impl Iterator<Item = Task> + use<> {
        let read = self.database.begin_read().expect("begin read transaction");
        let tasks = read.open_table(TASKS).expect("open tasks table");
        tasks
            .range::<u64>(..)
            .expect("range over tasks")
            .map(|entry| {
                let (_id, value) = entry.expect("read task row");
                value.value()
            })
    }

    pub fn list_projects(&self) -> impl Iterator<Item = Project> + use<> {
        let read = self.database.begin_read().expect("begin read transaction");
        let projects = read.open_table(PROJECTS).expect("open projects table");
        projects
            .range::<u64>(..)
            .expect("range over projects")
            .map(|entry| {
                let (_id, value) = entry.expect("read project row");
                value.value()
            })
    }

    /// Persist `task`, inserting it or overwriting the row with the same id —
    /// like `HashMap::insert`, but durable: the write is committed before this
    /// returns. Mutate a task by `get`-ing it, applying a transition
    /// ([`Task::start`] and friends), setting `updated_at`, and putting it back.
    ///
    /// Taking `&mut self` makes the store's single-writer invariant a
    /// compile-time guarantee, so the `get`-mutate-`put` round trip cannot race
    /// another writer.
    pub fn put(&mut self, task: Task) {
        let write = self
            .database
            .begin_write()
            .expect("begin write transaction");
        {
            let mut tasks = write.open_table(TASKS).expect("open tasks table");
            tasks.insert(task.id.0, &task).expect("write task");
        }
        write.commit().expect("commit task write");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Attention;
    use std::path::PathBuf;
    use tau_proto::AgentId;

    fn store() -> (tempfile::TempDir, TaskStore) {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = TaskStore::open(&dir.path().join("factory.redb"));
        (dir, store)
    }

    #[test]
    fn ids_are_monotonic_and_tasks_start_open() {
        let (_dir, mut store) = store();
        let first = store.create("a".into(), "issue a".into());
        let second = store.create("b".into(), "issue b".into());
        assert_eq!(first.id, TaskId(1));
        assert_eq!(second.id, TaskId(2));
        assert_eq!(first.status, Status::Open);
    }

    #[test]
    fn get_on_empty_store_is_none() {
        let (_dir, store) = store();
        assert!(store.get(TaskId(1)).is_none());
        assert_eq!(store.list().count(), 0);
    }

    #[test]
    fn list_yields_tasks_in_id_order() {
        let (_dir, mut store) = store();
        store.create("a".into(), "issue a".into());
        store.create("b".into(), "issue b".into());
        let ids: Vec<TaskId> = store.list().map(|task| task.id).collect();
        assert_eq!(ids, vec![TaskId(1), TaskId(2)]);
    }

    #[test]
    fn projects_are_persisted_in_id_order() {
        let (_dir, mut store) = store();
        store.create_project("alpha".into(), PathBuf::from("/tmp/alpha"));
        store.create_project("beta".into(), PathBuf::from("/tmp/beta"));

        let projects = store.list_projects().collect::<Vec<_>>();
        assert_eq!(projects[0].id, ProjectId(1));
        assert_eq!(projects[0].name, "alpha");
        assert_eq!(projects[1].id, ProjectId(2));
        assert_eq!(projects[1].path, PathBuf::from("/tmp/beta"));
    }

    #[test]
    fn put_persists_a_mutated_task() {
        let (_dir, mut store) = store();
        let created = store.create("a".into(), "issue a".into());
        let agent = AgentId::parse("impl-1").unwrap();

        // The realistic mutation flow: read, apply a transition, put back.
        let mut task = store.get(created.id).expect("task exists");
        task.start(PathBuf::from("/tmp/ws"), agent.clone()).unwrap();
        store.put(task);

        assert_eq!(
            store.get(created.id).expect("task exists").status,
            Status::Active {
                worktree: PathBuf::from("/tmp/ws"),
                agent,
            }
        );
    }

    #[test]
    fn put_overwrites_rather_than_appending() {
        let (_dir, mut store) = store();
        let mut task = store.create("a".into(), "issue a".into());
        task.title = "renamed".into();
        store.put(task);

        let all: Vec<Task> = store.list().collect();
        assert_eq!(all.len(), 1, "put must overwrite the existing row");
        assert_eq!(all[0].title, "renamed");
    }

    #[test]
    fn tasks_and_id_counter_survive_reopen() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("factory.redb");

        let created_id;
        {
            let mut store = TaskStore::open(&path);
            let mut task = store.create("Fix crash".into(), "stack overflow".into());
            created_id = task.id;
            task.flag(Attention::Review);
            store.put(task);
        }

        // Reopen the same file: the task, its flag, and the id counter persist.
        let mut store = TaskStore::open(&path);
        let recovered = store.get(created_id).expect("task survived reopen");
        assert_eq!(recovered.title, "Fix crash");
        assert_eq!(recovered.attention, Some(Attention::Review));

        let next = store.create("Second".into(), "another".into());
        assert_eq!(next.id, TaskId(2), "id counter must not reset on reopen");
    }
}
