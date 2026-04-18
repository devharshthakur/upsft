## About

`upsft` is macOS-only CLI to update selected dependency tools.

Tooling:

- `pnpm` for JS package manager and dev scripts
- `cargo` for Rust build, run, test, fmt, lint

Goal:

- let user pick which deps to manage
- update chosen deps from one CLI

## Project Structure

- `Cargo.toml` — Rust crate manifest
- `Cargo.lock` — Rust lockfile
- `package.json` — pnpm scripts and JS dev deps
- `pnpm-lock.yaml` — pnpm lockfile
- `src/main.rs` — app entrypoint
- `src/config.rs` — prompt config via `cliclack`
- `src/deps.rs` — dependency enum and labels
- `src/error.rs` — app error type
- `src/util.rs` — command helper utilities
- `README.md` — project readme

## `src/deps.rs`

`src/deps.rs` holds dependency list used by CLI:

- `npm`
- `pnpm`
- `homebrew`
- `vp`
- `fnm`

It defines:

- `Dependency` enum
- `Dependency::ALL`
- display labels via `label()` and `Display`

Use this file for any dep-specific metadata later.

## Dependencies (Rust)

- `cliclack` — CLI prompts
- `serde` + `serde_json` — config serialization
- `thiserror` — error handling

## Notes

- Keep code macOS-focused unless project scope changes.
- Use `cliclack` for prompts.
- Keep dep list in `src/deps.rs`, not scattered across files.
- Prefer small, clear modules.
- Avoid cloning operations
- For adding new crates use `cargo add` command instead of editing `cargo.toml` file.
