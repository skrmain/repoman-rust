use crate::utils::{git, github};
use anyhow::Result;
use colored::*;
use dialoguer::{Confirm, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

pub async fn execute(org: &str, dir: Option<PathBuf>, all: bool, select: bool) -> Result<()> {
    let base_dir = dir.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join("Desktop").join(org)
    });

    println!(
        "\n{} {}\n",
        "📦 Managing repositories for:".blue(),
        org.bold()
    );
    println!(
        "{} {}\n",
        "📁 Base directory:".bright_black(),
        base_dir.display().to_string().bright_black()
    );

    // Fetch repositories
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Fetching repositories...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let repos = github::fetch_repos(org).await?;
    spinner.finish_with_message(format!("Found {} repositories", repos.len().to_string().bold()));

    if repos.is_empty() {
        println!("\n{}", "⚠️  No repositories found for this organization".yellow());
        return Ok(());
    }

    let repos_to_clone = if all {
        repos
    } else if select {
        // Interactive selection
        let items: Vec<String> = repos
            .iter()
            .map(|r| {
                format!(
                    "{} - {}",
                    r.name,
                    r.description.as_deref().unwrap_or("No description")
                )
            })
            .collect();

        let selection = Select::new()
            .with_prompt("Select a repository to clone")
            .items(&items)
            .interact()?;

        vec![repos[selection].clone()]
    } else {
        // Default: show list and ask to clone all
        println!("{}", "Available repositories:".cyan());
        for (i, repo) in repos.iter().enumerate() {
            let desc = repo.description.as_deref().unwrap_or("No description");
            println!(
                "  {}. {} {}",
                i + 1,
                repo.name.bold(),
                format!("- {}", desc).bright_black()
            );
        }

        let should_clone_all = Confirm::new()
            .with_prompt("\nClone all repositories?")
            .default(false)
            .interact()?;

        if should_clone_all {
            repos
        } else {
            println!(
                "\n{} Use --select to choose specific repos or --all to clone all",
                "💡".yellow()
            );
            return Ok(());
        }
    };

    // Create base directory
    std::fs::create_dir_all(&base_dir)?;

    // Clone repositories
    println!(
        "\n{} Cloning {} repository(ies)...\n",
        "🚀".cyan(),
        repos_to_clone.len()
    );

    for repo in repos_to_clone {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        spinner.set_message(format!("Cloning {}...", repo.name));
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));

        let dest_path = base_dir.join(&repo.name);
        
        match git::clone_repo(&repo.clone_url, &dest_path) {
            Ok(_) => {
                spinner.finish_with_message(format!("{} Cloned {}", "✓".green(), repo.name));
            }
            Err(e) => {
                spinner.finish_with_message(format!("{} Failed to clone {}", "✗".red(), repo.name));
                eprintln!("  {}: {}", "Error".red(), e);
            }
        }
    }

    println!(
        "\n{} Done! Repositories are in: {}",
        "✅".green(),
        base_dir.display()
    );

    Ok(())
}