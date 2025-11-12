use gix::ThreadSafeRepository;
use gix::bstr::ByteSlice;
use openapi::models::CommitInfo;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MergeConflict {
    pub file: String,
    pub diff: String,
}

#[derive(Debug, Error)]
pub enum GitError {
    #[error("failed to open git repository at {path}: {source}")]
    Open {
        path: PathBuf,
        #[source]
        source: gix::open::Error,
    },
    #[error("failed to resolve revision {rev}: {source}")]
    RevParse {
        rev: String,
        #[source]
        source: gix::revision::spec::parse::single::Error,
    },
    #[error("failed to access commit object: {0}")]
    ObjectAccess(#[from] gix::object::find::existing::Error),
    #[error("git command spawn failed: {0}")]
    CommandSpawn(#[from] std::io::Error),
    #[error("git command exited with error: {stderr}")]
    CommandFailure { stderr: String },
    #[error("repository at {path} is bare or invalid")]
    BareRepository { path: PathBuf },
}

pub fn ensure_non_bare_repo(repo_root: &Path) -> Result<(), GitError> {
    let repo = ThreadSafeRepository::open(repo_root).map_err(|source| GitError::Open {
        path: repo_root.to_path_buf(),
        source,
    })?;
    let repo = repo.to_thread_local();
    if repo.is_bare() {
        return Err(GitError::BareRepository {
            path: repo_root.to_path_buf(),
        });
    }
    Ok(())
}

pub fn merge(repo_root: &Path, revision: &str) -> Result<(), GitError> {
    run_git_command(
        repo_root,
        vec![OsString::from("merge"), OsString::from(revision)],
    )?;
    Ok(())
}

pub fn merge_ff_only(repo_root: &Path, revision: &str) -> Result<(), GitError> {
    run_git_command(
        repo_root,
        vec![
            OsString::from("merge"),
            OsString::from("--ff-only"),
            OsString::from(revision),
        ],
    )?;
    Ok(())
}

pub fn collect_merge_conflicts(repo_root: &Path) -> Result<Vec<MergeConflict>, GitError> {
    let files_output = run_git_command(
        repo_root,
        vec![
            OsString::from("diff"),
            OsString::from("--name-only"),
            OsString::from("--diff-filter=U"),
        ],
    )?;

    let files: Vec<String> = files_output
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let mut conflicts = Vec::with_capacity(files.len());
    for file in files {
        let diff = run_git_command(
            repo_root,
            vec![
                OsString::from("diff"),
                OsString::from("--cc"),
                OsString::from("--"),
                OsString::from(&file),
            ],
        )?;
        conflicts.push(MergeConflict { file, diff });
    }

    Ok(conflicts)
}

pub fn list_worktrees(repo_root: &Path) -> Result<Vec<PathBuf>, GitError> {
    let output = run_git_command(
        repo_root,
        vec![
            OsString::from("worktree"),
            OsString::from("list"),
            OsString::from("--porcelain"),
        ],
    )?;

    let mut paths = Vec::new();
    for block in output.split("\n\n") {
        for line in block.lines() {
            if let Some(rest) = line.strip_prefix("worktree ") {
                paths.push(PathBuf::from(rest.trim()));
                break;
            }
        }
    }

    Ok(paths)
}

pub fn get_commit_info(repo_root: &Path, rev: &str) -> Result<CommitInfo, GitError> {
    let repo = ThreadSafeRepository::open(repo_root).map_err(|source| GitError::Open {
        path: repo_root.to_path_buf(),
        source,
    })?;
    let repo = repo.to_thread_local();
    let id = repo
        .rev_parse_single(rev.as_bytes().as_bstr())
        .map_err(|source| GitError::RevParse {
            rev: rev.to_string(),
            source,
        })?;
    let object = id.object()?;
    let commit = object.into_commit();
    let message = commit
        .message_raw()
        .ok()
        .map(|raw| String::from_utf8_lossy(raw.as_ref()).trim().to_string())
        .unwrap_or_default();
    let is_root_commit = commit.parent_ids().next().is_none();
    let hash = id.to_hex().to_string();
    let diff = generate_diff_stat(repo_root, rev, is_root_commit)?;

    Ok(CommitInfo {
        hash,
        message,
        diff,
    })
}

pub fn get_file_diff(repo_root: &Path, rev: &str, file: &str) -> Result<String, GitError> {
    let repo = ThreadSafeRepository::open(repo_root).map_err(|source| GitError::Open {
        path: repo_root.to_path_buf(),
        source,
    })?;
    let repo = repo.to_thread_local();
    let id = repo
        .rev_parse_single(rev.as_bytes().as_bstr())
        .map_err(|source| GitError::RevParse {
            rev: rev.to_string(),
            source,
        })?;
    let commit = id.object()?.into_commit();
    let is_root_commit = commit.parent_ids().next().is_none();

    let relative_path = normalize_file_argument(repo_root, file);

    let mut args = vec![OsString::from("diff")];
    if is_root_commit {
        args.push(OsString::from("--root"));
        args.push(OsString::from(rev));
    } else {
        args.push(OsString::from(format!("{rev}^!")));
    }
    args.push(OsString::from("--"));
    args.push(relative_path.into_os_string());

    run_git_command(repo_root, args)
}

fn generate_diff_stat(
    repo_root: &Path,
    rev: &str,
    is_root_commit: bool,
) -> Result<String, GitError> {
    let mut args = vec![OsString::from("diff"), OsString::from("--stat")];
    if is_root_commit {
        args.push(OsString::from("--root"));
        args.push(OsString::from(rev));
    } else {
        args.push(OsString::from(format!("{rev}^!")));
    }
    run_git_command(repo_root, args)
}

fn run_git_command(repo_root: &Path, args: Vec<OsString>) -> Result<String, GitError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(GitError::CommandFailure {
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn normalize_file_argument(repo_root: &Path, file: &str) -> PathBuf {
    let cleaned = file.trim_start_matches("./");
    let candidate = Path::new(cleaned);
    if candidate.is_absolute() {
        candidate
            .strip_prefix(repo_root)
            .map(PathBuf::from)
            .unwrap_or_else(|_| candidate.to_path_buf())
    } else {
        PathBuf::from(candidate)
    }
}
