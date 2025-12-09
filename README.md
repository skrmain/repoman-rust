# repoman-rust

A CLI tool to manage GitHub organization repositories - now in Rust! 🦀

Originally made with help of `claude`, now rewritten in Rust for better performance and reliability.

## Features

- 📋 **List** all repositories from a GitHub organization or user
- 📦 **Install** (clone) repositories locally with flexible options
- 📊 **Status** check on cloned repositories (git status, ahead/behind tracking)
- 🚀 Fast and efficient with Rust
- 🎨 Beautiful colored terminal output
- 🔄 Handles pagination for large organizations

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- Git installed on your system
- Optional: GitHub Personal Access Token for higher API rate limits

### Build from source

```sh
# Clone the repository
git clone <your-repo-url>
cd repoman

# Install Cargo (One Time Only)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build release binary
cargo build --release

# The binary will be at ./target/release/repoman
# Optionally, install it globally
cargo install --path .
```

## Usage

### List repositories

```sh
repoman list <org-name>

# Example
repoman list rust-lang
```

### Install (clone) repositories

```sh
# Clone all repositories
repoman install <org-name> --all

# Interactively select repositories to clone
repoman install <org-name> --select

# Specify custom directory
repoman install <org-name> --dir ~/projects/my-org --all

# Default behavior: shows list and prompts for confirmation
repoman install <org-name>
```

### Check status

```sh
repoman status <org-name>

# With custom directory
repoman status <org-name> --dir ~/projects/my-org
```

## Configuration

### GitHub Token (Optional)

For higher API rate limits, set your GitHub token:

```sh
export GITHUB_TOKEN=your_github_token_here
```

Or add it to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.)

## Development

```sh
# Run in development mode
cargo run -- list rust-lang
cargo run -- status <org-name>
cargo run -- install <org-name> --select

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Project Structure

```
repoman/
├── src/
│   ├── main.rs           # Entry point and CLI setup
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── list.rs       # List command
│   │   ├── install.rs    # Install command
│   │   └── status.rs     # Status command
│   └── utils/
│       ├── mod.rs
│       ├── github.rs     # GitHub API interactions
│       └── git.rs        # Git operations
├── Cargo.toml
└── README.md
```

## Dependencies

- `clap` - Command-line argument parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client for GitHub API
- `serde` - Serialization/deserialization
- `colored` - Terminal colors
- `indicatif` - Progress indicators
- `dialoguer` - Interactive prompts
- `git2` - Git operations (alternative to shelling out)

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
