use crate::utils::github;
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn execute(org: &str) -> Result<()> {
    println!("\n{} {}\n", "📋 Repositories for:".blue(), org.bold());

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
        println!("\n{}", "⚠️  No repositories found".yellow());
        return Ok(());
    }

    println!();
    for (i, repo) in repos.iter().enumerate() {
        println!("{}. {}", format!("{}", i + 1).cyan(), repo.name.bold());
        
        if let Some(desc) = &repo.description {
            println!("   {}", desc.bright_black());
        }
        
        let lang = repo.language.as_deref().unwrap_or("No language");
        println!(
            "   {}",
            format!(
                "⭐ {} | 🍴 {} | {}",
                repo.stargazers_count, repo.forks_count, lang
            )
            .bright_black()
        );
        println!();
    }

    Ok(())
}