## About

`upsft` is a macOS-only CLI tool to update selected dependency tools.

### Tooling

- `pnpm` for JavaScript package manager and dev scripts
- `cargo` for Rust build, run, test, fmt, lint

### Goals

- Let users pick which dependencies to manage
- Update chosen dependencies from a single CLI

## Code Style Guidelines

- Keep code macOS-focused unless project scope changes
- Use `cliclack` for command-line prompts
- Prefer small, clear modules
- Avoid cloning operations
- Use `cargo add` command instead of editing `Cargo.toml` directly when adding new crates

## Commit Guidelines

- Use conventional commit style for commit titles
- Write commit descriptions only when the commit is large enough that the title cannot provide full context (in that case, make the title more general with details in the description)
- For large numbers of diffs or changes, first suggest dividing into smaller commits and ask for approval before proceeding
