use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod utils;

#[derive(Parser)]
#[command(name = "repoman")]
#[command(about = "CLI tool to manage organization repositories", version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all repositories from an organization
    List {
        /// GitHub organization or account name
        org: String,
    },
    /// Clone repositories from an organization
    Install {
        /// GitHub organization or account name
        org: String,
        /// Base directory (defaults to ~/Desktop/<org>)
        #[arg(short, long)]
        dir: Option<PathBuf>,
        /// Clone all repositories
        #[arg(short, long)]
        all: bool,
        /// Interactively select repositories to clone
        #[arg(short, long)]
        select: bool,
    },
    /// Show status of locally cloned repositories
    Status {
        /// GitHub organization or account name
        org: String,
        /// Base directory (defaults to ~/Desktop/<org>)
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { org } => {
            commands::list::execute(&org).await?;
        }
        Commands::Install {
            org,
            dir,
            all,
            select,
        } => {
            commands::install::execute(&org, dir, all, select).await?;
        }
        Commands::Status { org, dir } => {
            commands::status::execute(&org, dir).await?;
        }
    }

    Ok(())
}