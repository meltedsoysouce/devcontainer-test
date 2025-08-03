# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a Rust development environment configured with devcontainer support and managed by mise. The project uses devcontainer-cli for container management and includes git worktree utilities.

## Common Commands

### Building and Running
- **Build the project**: `cargo build` (run inside devcontainer)
- **Run the project**: `cargo run` (run inside devcontainer)
- **Build release version**: `cargo build --release` (run inside devcontainer)

### Container Management (using mise)
- **Start devcontainer**: `mise run container-up`
- **Execute commands in container**: `mise run container-exec <command>`
- **Stop container**: Run the script at `mise-tasks/container-down.sh`

### Git Worktree Operations
- **Create worktree**: `mise run checkout-worktree` (creates at `./.git/worktree/{{BRANCH_NAME}}`)
- **Remove worktree**: `mise run remove-worktree`

## Architecture

This is a minimal Rust project with:
- Single binary crate at src/main.rs
- Devcontainer configuration for isolated development environment
- Rust toolchain installed via devcontainer features
- Claude Code pre-installed in the container environment

## Development Workflow

1. Start the devcontainer: `mise run container-up`
2. Execute Rust commands inside the container: `mise run container-exec cargo <command>`
3. The workspace is mounted at `/workspace` inside the container
4. Git operations can be performed both inside and outside the container

## Important Notes

- Cargo is only available inside the devcontainer
- The devcontainer includes VS Code extensions for ESLint, Prettier, and GitLens
- Network administration capabilities are enabled in the container (NET_ADMIN, NET_RAW)
- The container uses Node.js 20 as the base image with Rust installed as a feature