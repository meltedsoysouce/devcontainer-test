# Devcontainer Claude Code Development Environment

This repository was created to learn how to set up a development environment using **devcontainer** for **Claude Code**. It includes a minimal Rust project and the necessary configuration files to run the code inside a VS Code devcontainer.

## Features

- Rust project scaffold (`src/main.rs`)
- Devcontainer configuration (`.devcontainer/`)
- Scripts for container management (`mise-tasks/`)
- Example prompt and design documents

## Getting Started

1. Install **mise** (https://mise.jdx.dev) if you haven't already.
2. Run `mise run container-up` to start the devcontainer.
3. Inside the container you can build and run the project:
   ```bash
   cargo build
   cargo run
   ```
4. Use the provided scripts to manage worktrees and stop the container.

## Contributing

Feel free to experiment with the setup, add more examples, or improve the documentation.
