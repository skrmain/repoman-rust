use anyhow::{Context, Result};
use git2::{Repository, StatusOptions};
use std::path::Path;
use std::process::Command;

pub fn clone_repo(repo_url: &str, dest_path: &Path) -> Result<()> {
    // Using git command for better output handling
    let status = Command::new("git")
        .args(["clone", repo_url, dest_path.to_str().unwrap()])
        .status()
        .context("Failed to execute git clone")?;

    if !status.success() {
        anyhow::bail!("Git clone failed");
    }

    Ok(())
}

#[derive(Debug)]
pub struct GitStatus {
    pub has_changes: bool,
    pub ahead: usize,
    pub behind: usize,
    pub branch: String,
}

pub fn get_git_status(repo_path: &Path) -> Result<GitStatus> {
    let repo = match Repository::open(repo_path) {
        Ok(r) => r,
        Err(_) => {
            return Ok(GitStatus {
                has_changes: false,
                ahead: 0,
                behind: 0,
                branch: "unknown".to_string(),
            });
        }
    };

    // Get branch name
    let branch = match repo.head() {
        Ok(head) => {
            if let Some(name) = head.shorthand() {
                name.to_string()
            } else {
                "unknown".to_string()
            }
        }
        Err(_) => "unknown".to_string(),
    };

    // Check for uncommitted changes
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;
    let has_changes = !statuses.is_empty();

    // Check ahead/behind
    let (ahead, behind) = match get_ahead_behind(&repo) {
        Ok(counts) => counts,
        Err(_) => (0, 0),
    };

    Ok(GitStatus {
        has_changes,
        ahead,
        behind,
        branch,
    })
}

fn get_ahead_behind(repo: &Repository) -> Result<(usize, usize)> {
    let head = repo.head()?;
    let head_oid = head.target().context("No target for HEAD")?;

    let branch = repo.head()?.shorthand().context("No branch name")?.to_string();
    
    // Try to get upstream
    let upstream_name = format!("refs/remotes/origin/{}", branch);
    let upstream = match repo.find_reference(&upstream_name) {
        Ok(r) => r,
        Err(_) => return Ok((0, 0)), // No upstream
    };

    let upstream_oid = upstream.target().context("No target for upstream")?;

    let (ahead, behind) = repo.graph_ahead_behind(head_oid, upstream_oid)?;

    Ok((ahead, behind))
}