use crate::utils::{git, github};
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub async fn execute(org: &str, dir: Option<PathBuf>) -> Result<()> {
    let base_dir = dir.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join("Desktop").join(org)
    });

    println!("\n{} {}", "📊 Status for:".blue(), org.bold());
    println!(
        "{} {}\n",
        "📁 Directory:".bright_black(),
        base_dir.display().to_string().bright_black()
    );

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Fetching repository information...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Fetch remote repos
    let remote_repos = github::fetch_repos(org).await?;
    spinner.set_message("Scanning local directories...");

    // Get local repos
    let local_dirs = if base_dir.exists() {
        std::fs::read_dir(&base_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if entry.file_type().ok()?.is_dir() {
                    Some(entry.file_name().to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else {
        spinner.finish_with_message("Base directory doesn't exist yet".yellow().to_string());
        Vec::new()
    };

    if base_dir.exists() {
        spinner.finish_with_message("Analysis complete");
    }

    // Create maps and sets
    let remote_repo_map: HashMap<String, &github::Repository> =
        remote_repos.iter().map(|r| (r.name.clone(), r)).collect();
    let local_set: HashSet<String> = local_dirs.iter().cloned().collect();

    println!("\n{}\n", "📦 Repository Status:".cyan());

    // Separate cloned and not cloned
    let mut cloned_repos = Vec::new();
    let mut not_cloned_repos = Vec::new();

    for repo in &remote_repos {
        if local_set.contains(&repo.name) {
            cloned_repos.push(&repo.name);
        } else {
            not_cloned_repos.push(&repo.name);
        }
    }

    // Show cloned repos with git status
    if !cloned_repos.is_empty() {
        println!("{}", "✓ Cloned:".green().bold());
        for repo_name in &cloned_repos {
            let repo_path = base_dir.join(repo_name);
            let git_status = git::get_git_status(&repo_path)?;

            let status_text = if git_status.has_changes {
                "[uncommitted changes]".yellow().to_string()
            } else if git_status.ahead > 0 {
                format!("[{} commits ahead]", git_status.ahead).yellow().to_string()
            } else if git_status.behind > 0 {
                format!("[{} commits behind]", git_status.behind).blue().to_string()
            } else {
                "[clean]".bright_black().to_string()
            };

            println!("  • {} {}", repo_name, status_text);
        }
        println!();
    }

    // Show not cloned repos
    if !not_cloned_repos.is_empty() {
        println!("{}", "✗ Not Cloned:".bright_black().bold());
        for name in &not_cloned_repos {
            println!("  {}", format!("• {}", name).bright_black());
        }
        println!();
    }

    // Show local-only repos (not in remote)
    let local_only_repos: Vec<String> = local_dirs
        .iter()
        .filter(|d| !remote_repo_map.contains_key(*d))
        .cloned()
        .collect();

    if !local_only_repos.is_empty() {
        println!("{}", "⚠️  Local Only (not in remote):".yellow().bold());
        for name in &local_only_repos {
            println!("  {}", format!("• {}", name).yellow());
        }
        println!();
    }

    // Summary
    println!("{}", "Summary:".cyan());
    println!(
        "  Remote: {} | Cloned: {} | Not Cloned: {}",
        remote_repos.len().to_string().bold(),
        cloned_repos.len().to_string().green().bold(),
        not_cloned_repos.len().to_string().bright_black().bold()
    );
    if !local_only_repos.is_empty() {
        println!(
            "  Local Only: {}",
            local_only_repos.len().to_string().yellow().bold()
        );
    }
    println!();

    Ok(())
}