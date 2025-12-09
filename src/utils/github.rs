use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub clone_url: String,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
}

pub async fn fetch_repos(org: &str) -> Result<Vec<Repository>> {
    let client = Client::new();
    let mut repos = Vec::new();
    let mut page = 1;
    let per_page = 100;

    loop {
        let url = format!(
            "https://api.github.com/users/{}/repos?per_page={}&page={}&sort=updated",
            org, per_page, page
        );

        let mut request = client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "repoman-cli");

        // Add token if available
        if let Ok(token) = env::var("GITHUB_TOKEN") {
            request = request.header("Authorization", format!("token {}", token));
        }

        let response = request
            .send()
            .await
            .context("Failed to send request to GitHub API")?;

        if response.status() == 404 {
            anyhow::bail!("Organization or user '{}' not found", org);
        }

        if !response.status().is_success() {
            anyhow::bail!(
                "GitHub API error: {} {}",
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            );
        }

        let data: Vec<Repository> = response
            .json()
            .await
            .context("Failed to parse GitHub API response")?;

        if data.is_empty() {
            break;
        }

        let data_len = data.len();
        repos.extend(data);

        if data_len < per_page {
            break;
        }

        page += 1;
    }

    Ok(repos)
}