## About

`upsft` is a macOS-only Rust CLI that batch-updates user-chosen dependency tools from a single TOML config file. The CLI reads `~/.config/upsft/config.toml` (or a custom path with `-c`), then runs each dependency's shell command via `sh -c` sequentially.

### Key concepts

- **Dependency**: a named entry in config (`[deps]` section), key=name, value=shell command to update it.
- **Config**: TOML file at `~/.config/upsft/config.toml`; sample at `sample.config.toml`.
- **CLI flags**: `--init` (create config), `-l`/`--list` (list deps), `-c`/`--config` (custom path).

## Stack fingerprint

- **Language**: Rust (edition 2024)
- **Build**: cargo
- **JS tooling**: pnpm (husky, lint-staged, prettier) — only for dev tooling and git hooks, not runtime
- **Key crates**: clap (CLI), serde (deserialization), toml (preserve_order), thiserror (error derive), home (home dir)
- **Test tool**: cargo test (no tests currently exist)
- **Changelog**: git-cliff via `cliff.toml`
- **Lint/format**: cargo clippy, cargo fmt, prettier

## Important paths

| Path                     | Purpose                                                           |
| ------------------------ | ----------------------------------------------------------------- |
| `src/main.rs`            | Binary entrypoint (3 lines)                                       |
| `src/lib.rs`             | Crate root, module declarations                                   |
| `src/cli.rs`             | CLI parsing (clap derive), arg dispatch, dep listing, update loop |
| `src/config.rs`          | Config load/init/validate logic, default path resolution          |
| `src/deps.rs`            | `Dependency` struct (name + update_command)                       |
| `src/cmd.rs`             | Shell command execution (`sh -c`)                                 |
| `src/error.rs`           | `UpsftError` enum (thiserror-derive, all error variants)          |
| `Cargo.toml`             | Single-crate manifest, dependencies, lints, release profile       |
| `package.json`           | pnpm scripts wrapping cargo commands                              |
| `pnpm-workspace.yaml`    | pnpm workspace root (allows esbuild native builds)                |
| `cliff.toml`             | git-cliff changelog config                                        |
| `clippy.toml`            | Clippy thresholds                                                 |
| `sample.config.toml`     | Example config for testing                                        |
| `lint-staged.config.cjs` | Pre-commit hooks (prettier on md/json/yaml, fmt+clippy on rs)     |

## Source-of-truth files

- **Manifest**: `Cargo.toml` (crate name, version, deps, edition, release profile)
- **Entrypoint**: `src/main.rs` → `src/cli.rs` (`Cli::run()`)
- **CLI schema**: `src/cli.rs` (clap `#[derive(Parser)]` struct)
- **Config schema**: `src/config.rs` (`Config::load`, `config::validate_config`)
- **Error contract**: `src/error.rs` (all `UpsftError` variants)
- **Test config**: `sample.config.toml`
- **Build config**: `Cargo.toml` (release profile with LTO, strip, panic=abort)
- **Changelog config**: `cliff.toml`

## Read first by task

| Task                                        | Read first                                                                                 |
| ------------------------------------------- | ------------------------------------------------------------------------------------------ |
| Add a CLI flag/option                       | `src/cli.rs` (clap struct + dispatch)                                                      |
| Change config format or parsing             | `src/config.rs` (load, init, validate)                                                     |
| Add a dependency field/metadata             | `src/deps.rs` → `src/config.rs` (validation loop)                                          |
| Change command execution behaviour          | `src/cmd.rs`                                                                               |
| Add/change error messages or error handling | `src/error.rs` → `src/cli.rs` (error match sites)                                          |
| Add a crate dependency                      | `cargo add <crate>` (per repo convention), then relevant `src/` file                       |
| Fix a bug                                   | `src/cli.rs` (dispatch logic) or `src/config.rs` (parsing) — the two main behavior modules |
| Write/add tests                             | `tests/` (none yet exist; create integration tests directory)                              |
| Build/release changes                       | `Cargo.toml` (release profile), `package.json` (dev scripts)                               |
| Changelog work                              | `cliff.toml`                                                                               |
| Git hooks / lint config                     | `lint-staged.config.cjs`, `.husky/`                                                        |

## Architecture and boundaries

```
main.rs (ExitCode)
  └─ cli.rs  (Cli::run — parse, dispatch)
       ├─ config.rs (Config::load / Config::init_config)
       ├─ deps.rs  (Dependency struct)
       ├─ cmd.rs   (execute shell command)
       └─ error.rs (UpsftError — no module touches std::io::Error directly)
```

- All public API is in `lib.rs` via `pub mod`.
- `cli.rs` is the sole orchestrator: it loads config, dispatches to list/update/init, and formats all user output.
- `config.rs` owns all TOML parsing and filesystem config operations.
- `cmd.rs` is a pure function — no side effects beyond spawning `sh -c`.
- `error.rs` contains every error variant; other modules only return `Result<_, UpsftError>`.
- `deps.rs` is a simple data struct, no logic.
- **Invariant**: config deps preserve insertion order (TOML `preserve_order` feature), so commands run in the order the user wrote them.

## Commands

```bash
# Install JS tooling (pre-commit hooks etc.)
pnpm install

# Dev run
pnpm dev                    # cargo run

# Release run
pnpm start                  # cargo run --release

# Build
pnpm build                  # cargo build
pnpm build:release          # cargo build --release

# Lint
pnpm lint                   # cargo clippy --fix --allow-dirty --all-features

# Format
pnpm format                 # prettier --write . && cargo fmt

# Test
pnpm test                   # cargo test

# CLI test with sample config
pnpm test:cli               # cargo run --release -- --config test/test.config.toml

# Changelog
pnpm changelog:unreleased   # git-cliff --unreleased --prepend CHANGELOG.md
pnpm changelog:release      # git-cliff --prepend CHANGELOG.md
```

## Search rules

1. Read `AGENTS.md` first (this file).
2. Open the exact files listed in "Read first by task" before any grep.
3. Use targeted `grep` only within `src/` for known symbols.
4. Avoid repo-wide search unless `AGENTS.md` is stale or you're tracing a cross-module issue.

## Risks and gotchas

- **No tests exist yet** — any behavior change carries regression risk; add tests in `tests/` alongside changes.
- **No `cliclack` prompt library** — the existing AGENTS.md said to use it, but it is not in `Cargo.toml` and no code references it. The CLI uses plain clap args only.
- **macOS-only by design** — `home::home_dir()` works on macOS but behavior on other OSes is untested/unsupported.
- **Shell injection risk** — `cmd.rs` passes user config values directly to `sh -c` with no sanitization. Users control their own config, but custom config paths from untrusted sources are dangerous.
- **`Cargo.lock` is committed** — binary crate; standard practice.
- **`preserve_order` TOML feature is load-bearing** — removing it silently changes dep execution order.
- **Exit codes**: the CLI returns `ExitCode::SUCCESS` (0) or `ExitCode::FAILURE` (1). Non-zero exit from any dep command makes the whole run fail, even if subsequent deps succeed.

## Unknowns / open questions

- No `scripts/` directory despite `package.json` referencing `scripts/` (none found — possibly deleted or never created).
- `pnpm test:cli` references `test/test.config.toml` — that path does not exist in the repo; likely a local-only file.
