use gix::ThreadSafeRepository;
use gix::bstr::ByteSlice;
use openapi::models::CommitInfo;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MergeConflict {
    pub file: String,
    pub diff: String,
}

#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub header: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub old_path: Option<String>,
    pub status_code: String,
    pub additions: u32,
    pub deletions: u32,
    pub hunks: Option<Vec<DiffHunk>>,
}

#[derive(Debug, Clone)]
pub struct WorktreeStatus {
    pub id: String,
    pub path: PathBuf,
    pub branch: String,
    pub upstream: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub is_dirty: bool,
    pub files: Vec<FileChange>,
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
    #[error("failed to parse git output: {0}")]
    Parse(String),
    #[error("worktree not found: {0}")]
    NotFound(String),
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

pub fn pull_ff_only(repo_root: &Path, remote: &str, branch: &str) -> Result<(), GitError> {
    run_git_command(
        repo_root,
        vec![
            OsString::from("pull"),
            OsString::from("--ff-only"),
            OsString::from(remote),
            OsString::from(branch),
        ],
    )?;
    Ok(())
}

pub fn fast_forward_to_staging(worktree_path: &Path) -> Result<(), GitError> {
    merge_ff_only(worktree_path, "staging")
}

pub fn stage_all(repo_root: &Path) -> Result<(), GitError> {
    run_git_command(
        repo_root,
        vec![OsString::from("add"), OsString::from("--all")],
    )?;
    Ok(())
}

pub fn commit(repo_root: &Path, message: &str) -> Result<(), GitError> {
    run_git_command(
        repo_root,
        vec![
            OsString::from("commit"),
            OsString::from("-m"),
            OsString::from(message),
        ],
    )?;
    Ok(())
}

pub fn is_dirty(repo_root: &Path) -> Result<bool, GitError> {
    let status = collect_status_for_path("staging".to_string(), repo_root.to_path_buf(), false)?;
    Ok(status.is_dirty)
}

pub fn stash_all(repo_root: &Path, message: &str) -> Result<(), GitError> {
    run_git_command(
        repo_root,
        vec![
            OsString::from("stash"),
            OsString::from("push"),
            OsString::from("-u"),
            OsString::from("-m"),
            OsString::from(message),
        ],
    )?;
    Ok(())
}

pub fn abort_merge(repo_root: &Path) -> Result<(), GitError> {
    run_git_command(
        repo_root,
        vec![OsString::from("merge"), OsString::from("--abort")],
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

pub fn create_worker_worktree(
    staging_repo: &Path,
    destination: &Path,
    worker_id: i64,
) -> Result<(), GitError> {
    let branch = format!("ws{worker_id}");
    run_git_command(
        staging_repo,
        vec![
            OsString::from("worktree"),
            OsString::from("add"),
            OsString::from("-B"),
            OsString::from(&branch),
            destination.as_os_str().to_os_string(),
        ],
    )?;
    Ok(())
}

pub fn collect_all_worktree_statuses(
    project_root: &Path,
    include_hunks: bool,
) -> Result<Vec<WorktreeStatus>, GitError> {
    let staging_dir = project_root.join("staging");
    let worktree_paths = list_worktrees(&staging_dir)?;
    let mut statuses = Vec::with_capacity(worktree_paths.len());
    for path in worktree_paths {
        let id = derive_worktree_id(&path);
        statuses.push(collect_status_for_path(id, path, include_hunks)?);
    }
    statuses.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(statuses)
}

pub fn fast_forward_all_worktrees(project_root: &Path) -> Result<(), GitError> {
    let staging_dir = project_root.join("staging");
    let worktree_paths = list_worktrees(&staging_dir)?;
    for path in worktree_paths {
        let id = derive_worktree_id(&path);
        if id == "staging" {
            continue;
        }
        // Try to fast-forward worker branches to staging; skip on failure so others continue.
        if let Err(err) = merge_ff_only(&path, "staging") {
            return Err(err);
        }
    }
    Ok(())
}

pub fn collect_worktree_status(
    project_root: &Path,
    worktree_id: &str,
    include_hunks: bool,
) -> Result<WorktreeStatus, GitError> {
    let staging_dir = project_root.join("staging");
    let worktree_paths = list_worktrees(&staging_dir)?;
    for path in worktree_paths {
        let id = derive_worktree_id(&path);
        if id == worktree_id {
            return collect_status_for_path(id, path, include_hunks);
        }
    }
    Err(GitError::NotFound(worktree_id.to_string()))
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

fn collect_status_for_path(
    id: String,
    path: PathBuf,
    include_hunks: bool,
) -> Result<WorktreeStatus, GitError> {
    let ParsedStatus { branch, entries } = parse_porcelain_status(&path)?;
    let tracked_numstat = collect_tracked_numstat(&path)?;
    let tracked_hunks = if include_hunks {
        collect_tracked_hunks(&path)?
    } else {
        HashMap::new()
    };

    let mut files = Vec::with_capacity(entries.len());
    for entry in entries {
        let (additions, deletions) = if entry.status_code == "??" {
            collect_untracked_numstat(&path, &entry.path)?
        } else {
            *tracked_numstat.get(&entry.path).unwrap_or(&(0, 0))
        };

        let hunks = if include_hunks {
            if entry.status_code == "??" {
                let untracked_hunks = collect_untracked_hunks(&path, &entry.path)?;
                if untracked_hunks.is_empty() {
                    None
                } else {
                    Some(untracked_hunks)
                }
            } else {
                tracked_hunks.get(&entry.path).cloned()
            }
        } else {
            None
        };

        files.push(FileChange {
            path: entry.path,
            old_path: entry.old_path,
            status_code: entry.status_code,
            additions,
            deletions,
            hunks,
        });
    }

    Ok(WorktreeStatus {
        id,
        path,
        branch: branch.branch,
        upstream: branch.upstream,
        ahead: branch.ahead,
        behind: branch.behind,
        is_dirty: !files.is_empty(),
        files,
    })
}

#[derive(Debug)]
struct ParsedStatus {
    branch: BranchInfo,
    entries: Vec<StatusEntry>,
}

#[derive(Debug)]
struct BranchInfo {
    branch: String,
    upstream: Option<String>,
    ahead: u32,
    behind: u32,
}

#[derive(Debug)]
struct StatusEntry {
    status_code: String,
    path: String,
    old_path: Option<String>,
}

fn parse_porcelain_status(repo_root: &Path) -> Result<ParsedStatus, GitError> {
    let output = run_git_command_bytes(
        repo_root,
        vec![
            OsString::from("status"),
            OsString::from("--porcelain=v2"),
            OsString::from("-b"),
            OsString::from("-z"),
        ],
    )?;
    let mut idx = 0;
    let mut branch = BranchInfo {
        branch: "HEAD".to_string(),
        upstream: None,
        ahead: 0,
        behind: 0,
    };
    let mut entries = Vec::new();

    while idx < output.len() {
        let end = output[idx..]
            .iter()
            .position(|b| *b == 0)
            .map(|pos| idx + pos)
            .ok_or_else(|| GitError::Parse("unterminated porcelain record".to_string()))?;
        if end == idx {
            idx += 1;
            continue;
        }
        let record = &output[idx..end];
        idx = end + 1;
        if record.is_empty() {
            continue;
        }
        let line = String::from_utf8_lossy(record);
        match record[0] as char {
            '#' => apply_branch_line(&line, &mut branch),
            '1' => {
                if let Some(entry) = parse_status_line(&line, 9) {
                    entries.push(entry);
                }
            }
            '2' => {
                let mut entry = parse_status_line(&line, 10)
                    .ok_or_else(|| GitError::Parse("invalid rename status line".to_string()))?;
                let old_end = output[idx..]
                    .iter()
                    .position(|b| *b == 0)
                    .map(|pos| idx + pos)
                    .ok_or_else(|| GitError::Parse("missing old path for rename".to_string()))?;
                let old_path = String::from_utf8_lossy(&output[idx..old_end]).to_string();
                entry.old_path = Some(old_path);
                idx = old_end + 1;
                entries.push(entry);
            }
            'u' => {
                if let Some(entry) = parse_status_line(&line, 9) {
                    entries.push(entry);
                }
            }
            '?' => entries.push(StatusEntry {
                status_code: "??".to_string(),
                path: line[2..].to_string(),
                old_path: None,
            }),
            '!' => continue,
            other => {
                return Err(GitError::Parse(format!(
                    "unknown porcelain entry prefix: {other}"
                )));
            }
        }
    }

    Ok(ParsedStatus { branch, entries })
}

fn apply_branch_line(line: &str, branch: &mut BranchInfo) {
    let content = line.trim_start_matches("# ").trim();
    if let Some(rest) = content.strip_prefix("branch.head ") {
        branch.branch = rest.to_string();
    } else if let Some(rest) = content.strip_prefix("branch.upstream ") {
        branch.upstream = Some(rest.to_string());
    } else if let Some(rest) = content.strip_prefix("branch.ab ") {
        let mut parts = rest.split_whitespace();
        if let Some(ahead_part) = parts.next() {
            branch.ahead = ahead_part.trim_start_matches('+').parse().unwrap_or(0);
        }
        if let Some(behind_part) = parts.next() {
            branch.behind = behind_part.trim_start_matches('-').parse().unwrap_or(0);
        }
    }
}

fn parse_status_line(line: &str, field_count: usize) -> Option<StatusEntry> {
    let mut parts = line.splitn(field_count, ' ');
    let _tag = parts.next()?;
    let status_code = parts.next()?.to_string();
    // Skip remaining metadata fields.
    for _ in 0..(field_count - 3) {
        if parts.next().is_none() {
            return None;
        }
    }
    let path = parts.next()?.to_string();
    Some(StatusEntry {
        status_code,
        path,
        old_path: None,
    })
}

fn collect_tracked_numstat(repo_root: &Path) -> Result<HashMap<String, (u32, u32)>, GitError> {
    let output = run_git_command(
        repo_root,
        vec![
            OsString::from("diff"),
            OsString::from("--numstat"),
            OsString::from("HEAD"),
        ],
    )?;
    let mut map = HashMap::new();
    for line in output.lines() {
        let mut parts = line.split('\t');
        let additions_raw = parts.next().unwrap_or("0");
        let deletions_raw = parts.next().unwrap_or("0");
        let path_raw = parts.next().unwrap_or_default();
        let normalized_path = normalize_numstat_path(path_raw);
        let additions = parse_numstat_value(additions_raw);
        let deletions = parse_numstat_value(deletions_raw);
        map.insert(normalized_path, (additions, deletions));
    }
    Ok(map)
}

fn collect_tracked_hunks(repo_root: &Path) -> Result<HashMap<String, Vec<DiffHunk>>, GitError> {
    let output = run_git_command(
        repo_root,
        vec![
            OsString::from("diff"),
            OsString::from("--unified=3"),
            OsString::from("HEAD"),
        ],
    )?;
    parse_diff_output(&output)
}

fn parse_diff_output(diff: &str) -> Result<HashMap<String, Vec<DiffHunk>>, GitError> {
    let mut map: HashMap<String, Vec<DiffHunk>> = HashMap::new();
    let mut current_file: Option<String> = None;
    let mut current_hunk_header: Option<String> = None;
    let mut current_lines: Vec<String> = Vec::new();
    let mut old_path: Option<String> = None;

    for line in diff.lines() {
        if line.starts_with("diff --git ") {
            finalize_hunk(
                &mut map,
                &current_file,
                &mut current_hunk_header,
                &mut current_lines,
            );
            current_file = None;
            old_path = None;
            continue;
        }
        if line.starts_with("--- ") {
            old_path = parse_diff_path(line);
            continue;
        }
        if line.starts_with("+++ ") {
            let path = parse_diff_path(line);
            current_file = path.or_else(|| old_path.clone());
            continue;
        }
        if line.starts_with("@@") {
            finalize_hunk(
                &mut map,
                &current_file,
                &mut current_hunk_header,
                &mut current_lines,
            );
            current_hunk_header = Some(line.to_string());
            continue;
        }
        if current_hunk_header.is_some() {
            current_lines.push(line.to_string());
        }
    }

    finalize_hunk(
        &mut map,
        &current_file,
        &mut current_hunk_header,
        &mut current_lines,
    );
    Ok(map)
}

fn finalize_hunk(
    map: &mut HashMap<String, Vec<DiffHunk>>,
    current_file: &Option<String>,
    current_hunk_header: &mut Option<String>,
    current_lines: &mut Vec<String>,
) {
    if let (Some(file), Some(header)) = (current_file, current_hunk_header.take()) {
        map.entry(file.clone()).or_default().push(DiffHunk {
            header,
            lines: std::mem::take(current_lines),
        });
    } else {
        current_lines.clear();
    }
}

fn parse_diff_path(line: &str) -> Option<String> {
    let path = line
        .get(4..)?
        .trim()
        .trim_end_matches('\t')
        .trim()
        .to_string();
    if path == "/dev/null" {
        return None;
    }
    if path.starts_with("a/") || path.starts_with("b/") {
        return Some(path[2..].to_string());
    }
    Some(path)
}

fn collect_untracked_numstat(repo_root: &Path, path: &str) -> Result<(u32, u32), GitError> {
    let target = repo_root.join(path);
    if !target.exists() || target.is_dir() {
        return Ok((0, 0));
    }
    let output = run_git_command_allowing(
        repo_root,
        vec![
            OsString::from("diff"),
            OsString::from("--numstat"),
            OsString::from("--no-index"),
            OsString::from("--"),
            OsString::from("/dev/null"),
            target.into_os_string(),
        ],
        &[1],
    )?;
    if output.trim().is_empty() {
        return Ok((0, 0));
    }
    let mut parts = output.lines().next().unwrap_or("").split('\t');
    let additions = parse_numstat_value(parts.next().unwrap_or("0"));
    let deletions = parse_numstat_value(parts.next().unwrap_or("0"));
    Ok((additions, deletions))
}

fn collect_untracked_hunks(repo_root: &Path, path: &str) -> Result<Vec<DiffHunk>, GitError> {
    let target = repo_root.join(path);
    if !target.exists() || target.is_dir() {
        return Ok(Vec::new());
    }
    let diff = run_git_command_allowing(
        repo_root,
        vec![
            OsString::from("diff"),
            OsString::from("--unified=3"),
            OsString::from("--no-index"),
            OsString::from("--"),
            OsString::from("/dev/null"),
            target.into_os_string(),
        ],
        &[1],
    )?;
    let mut hunks = Vec::new();
    let mut current_header: Option<String> = None;
    let mut lines = Vec::new();
    for line in diff.lines() {
        if line.starts_with("@@") {
            if let Some(header) = current_header.replace(line.to_string()) {
                hunks.push(DiffHunk {
                    header,
                    lines: std::mem::take(&mut lines),
                });
            } else {
                current_header = Some(line.to_string());
            }
            lines.clear();
            continue;
        }
        if current_header.is_some() {
            lines.push(line.to_string());
        }
    }
    if let Some(header) = current_header {
        hunks.push(DiffHunk { header, lines });
    }
    Ok(hunks)
}

fn run_git_command_bytes(repo_root: &Path, args: Vec<OsString>) -> Result<Vec<u8>, GitError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(&args)
        .output()?;
    if !output.status.success() {
        return Err(GitError::CommandFailure {
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }
    Ok(output.stdout)
}

fn run_git_command_allowing(
    repo_root: &Path,
    args: Vec<OsString>,
    allowed: &[i32],
) -> Result<String, GitError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(&args)
        .output()?;
    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        if !allowed.contains(&code) {
            return Err(GitError::CommandFailure {
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            });
        }
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn normalize_numstat_path(raw: &str) -> String {
    if raw.contains('{') && raw.contains("=>") && raw.contains('}') {
        normalize_brace_rename(raw)
    } else if let Some(idx) = raw.rfind("=>") {
        raw[idx + 2..].trim().to_string()
    } else {
        raw.to_string()
    }
}

fn normalize_brace_rename(raw: &str) -> String {
    let mut result = String::new();
    let mut chars = raw.chars();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut new_segment = String::new();
            let mut in_new = false;
            while let Some(inner) = chars.next() {
                if inner == '}' {
                    break;
                }
                if !in_new {
                    if inner == '=' {
                        if matches!(chars.next(), Some('>')) {
                            if matches!(chars.clone().next(), Some(' ')) {
                                let _ = chars.next();
                            }
                            in_new = true;
                        }
                    }
                } else {
                    new_segment.push(inner);
                }
            }
            result.push_str(new_segment.trim());
        } else {
            result.push(ch);
        }
    }
    result
}

fn parse_numstat_value(raw: &str) -> u32 {
    if raw == "-" {
        0
    } else {
        raw.parse().unwrap_or(0)
    }
}

fn derive_worktree_id(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            if name == "staging" || name.starts_with("ws") {
                name.to_string()
            } else {
                name.to_string()
            }
        })
        .unwrap_or_else(|| path.display().to_string())
}
