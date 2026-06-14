//! Per-task jj workspaces — the isolated working copies that task agents run in.
//!
//! Each task gets its own jj workspace sharing the project's repository, so its
//! agent can edit and commit without colliding with other tasks. The factory
//! owns *policy* (workspace naming, when to create and destroy, orphan cleanup);
//! this module is the *mechanism*, a thin wrapper over the `jj workspace`
//! subcommands.
//!
//! The calls are async (`smol::process`) because the factory runs as an async
//! extension: a `jj workspace add` writes a fresh working copy and can take a
//! noticeable moment, and blocking the extension's event loop on it would stall
//! every other task and the board.

use std::ffi::OsStr;
use std::io;
use std::path::Path;

/// Adds a jj workspace named `name` at `destination`, sharing the repository
/// rooted at `repo_root`.
///
/// `start_revision` is the revision the new workspace's working copy is created
/// on top of (for the factory, the project's trunk). When `None`, jj uses its
/// default of the current working-copy parent.
pub async fn create_workspace(
    repo_root: &Path,
    name: &str,
    destination: &Path,
    start_revision: Option<&str>,
) -> io::Result<()> {
    let mut arguments: Vec<&OsStr> = vec![
        OsStr::new("workspace"),
        OsStr::new("add"),
        OsStr::new("--name"),
        OsStr::new(name),
    ];
    if let Some(start_revision) = start_revision {
        arguments.push(OsStr::new("--revision"));
        arguments.push(OsStr::new(start_revision));
    }
    arguments.push(destination.as_os_str());
    run_jj(repo_root, &arguments).await
}

/// Forgets the jj workspace named `name`, then removes its `destination`
/// directory.
///
/// `jj workspace forget` only drops the workspace from the repository; the
/// working-copy directory is removed separately. An already-absent directory is
/// treated as success so a re-run after a partial failure still completes.
pub async fn destroy_workspace(repo_root: &Path, name: &str, destination: &Path) -> io::Result<()> {
    run_jj(
        repo_root,
        &[
            OsStr::new("workspace"),
            OsStr::new("forget"),
            OsStr::new(name),
        ],
    )
    .await?;
    match smol::fs::remove_dir_all(destination).await {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        result => result,
    }
}

async fn run_jj(repo_root: &Path, arguments: &[&OsStr]) -> io::Result<()> {
    let output = smol::process::Command::new("jj")
        .arg("--repository")
        .arg(repo_root)
        .args(arguments)
        .output()
        .await?;
    if output.status.success() {
        return Ok(());
    }
    let rendered = arguments
        .iter()
        .map(|argument| argument.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");
    Err(io::Error::other(format!(
        "`jj {rendered}` failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    async fn jj(repo_root: &Path, arguments: &[&str]) -> std::process::Output {
        smol::process::Command::new("jj")
            .arg("--repository")
            .arg(repo_root)
            .args(arguments)
            .output()
            .await
            .expect("run jj")
    }

    async fn init_repo(repo_root: &Path) {
        smol::fs::create_dir_all(repo_root)
            .await
            .expect("create repo dir");
        let status = smol::process::Command::new("jj")
            .arg("git")
            .arg("init")
            .arg(repo_root)
            .status()
            .await
            .expect("jj git init");
        assert!(status.success(), "jj git init failed");
        // Repo-local author so the working-copy commit a workspace creates has
        // one, without depending on (or mutating) the test machine's jj config.
        for (key, value) in [("user.name", "test"), ("user.email", "test@example.com")] {
            let output = jj(repo_root, &["config", "set", "--repo", key, value]).await;
            assert!(output.status.success(), "jj config set {key} failed");
        }
    }

    async fn workspace_listing(repo_root: &Path) -> String {
        let output = jj(repo_root, &["workspace", "list"]).await;
        assert!(output.status.success(), "jj workspace list failed");
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    #[test]
    fn create_then_destroy_roundtrip() {
        smol::block_on(async {
            let temp = tempfile::tempdir().expect("tempdir");
            let repo_root = temp.path().join("repo");
            init_repo(&repo_root).await;

            let destination = temp.path().join("ws-task-1");
            create_workspace(&repo_root, "task-1", &destination, None)
                .await
                .expect("create workspace");
            assert!(destination.is_dir(), "workspace directory was not created");
            assert!(
                workspace_listing(&repo_root).await.contains("task-1"),
                "workspace not registered with the repository"
            );

            destroy_workspace(&repo_root, "task-1", &destination)
                .await
                .expect("destroy workspace");
            assert!(!destination.exists(), "workspace directory was not removed");
            assert!(
                !workspace_listing(&repo_root).await.contains("task-1"),
                "workspace still registered after forget"
            );
        });
    }

    #[test]
    fn destroy_is_idempotent_when_directory_already_gone() {
        smol::block_on(async {
            let temp = tempfile::tempdir().expect("tempdir");
            let repo_root = temp.path().join("repo");
            init_repo(&repo_root).await;

            let destination = temp.path().join("ws-task-2");
            create_workspace(&repo_root, "task-2", &destination, None)
                .await
                .expect("create workspace");
            smol::fs::remove_dir_all(&destination)
                .await
                .expect("pre-remove directory");

            destroy_workspace(&repo_root, "task-2", &destination)
                .await
                .expect("destroy tolerates a missing directory");
        });
    }
}
