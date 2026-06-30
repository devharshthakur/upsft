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
- **Key crates**: clap (CLI), toml (preserve_order), thiserror (error derive), home (home dir)
- **Test tool**: cargo test (no tests currently exist)
- **Changelog**: git-cliff via `cliff.toml`
- **Lint/format**: cargo clippy, cargo fmt, prettier

## Important paths

| Path                     | Purpose                                                           |
| ------------------------ | ----------------------------------------------------------------- |
| `src/main.rs`            | Binary entrypoint, module declarations                            |
| `src/cli.rs`             | CLI parsing (clap derive), arg dispatch, dep listing, update loop |
| `src/config.rs`          | Config load/init/parse logic, default path resolution             |
| `src/deps.rs`            | `Dependency` struct (name + command)                              |
| `src/exec.rs`            | Sequential `sh -c` runner (spawn + report)                        |
| `src/errors/mod.rs`      | Module declarations                                               |
| `src/errors/config.rs`   | `ConfigError` (thiserror-derive)                                  |
| `Cargo.toml`             | Single-crate manifest, dependencies, lints, release profile       |
| `package.json`           | pnpm scripts wrapping cargo commands                              |
| `cliff.toml`             | git-cliff changelog config                                        |
| `clippy.toml`            | Clippy thresholds                                                 |
| `sample.config.toml`     | Example config for testing                                        |
| `lint-staged.config.cjs` | Pre-commit hooks (prettier on md/json/yaml, fmt+clippy on rs)     |

## Source-of-truth files

- **Manifest**: `Cargo.toml` (crate name, version, deps, edition, release profile)
- **Entrypoint**: `src/main.rs` → `src/cli.rs` (`cli::run()`)
- **CLI schema**: `src/cli.rs` (clap `#[derive(Parser)]` struct)
- **Config schema**: `src/config.rs` (`Config::load`, `config::parse_deps_table`)
- **Error contract**: `src/errors/config.rs` (all `ConfigError` variants)
- **Test config**: `sample.config.toml`
- **Build config**: `Cargo.toml` (release profile with LTO, strip, panic=abort)
- **Changelog config**: `cliff.toml`

## Read first by task

| Task                                        | Read first                                                                                 |
| ------------------------------------------- | ------------------------------------------------------------------------------------------ |
| Add a CLI flag/option                       | `src/cli.rs` (clap struct + dispatch)                                                      |
| Change config format or parsing             | `src/config.rs` (load, init, parse)                                                        |
| Add a dependency field/metadata             | `src/deps.rs` → `src/config.rs` (validation loop)                                          |
| Change command execution behaviour          | `src/exec.rs` (spawn + report)                                                             |
| Add/change error messages or error handling | `src/errors/config.rs` → `src/cli.rs` (error match sites)                                  |
| Add a crate dependency                      | `cargo add <crate>` (per repo convention), then relevant `src/` file                       |
| Fix a bug                                   | `src/cli.rs` (dispatch logic) or `src/config.rs` (parsing) — the two main behavior modules |
| Write/add tests                             | `tests/` (none yet exist; create integration tests directory)                              |
| Build/release changes                       | `Cargo.toml` (release profile), `package.json` (dev scripts)                               |
| Changelog work                              | `cliff.toml`                                                                               |
| Git hooks / lint config                     | `lint-staged.config.cjs`, `.husky/`                                                        |

## Architecture and boundaries

```
main.rs (ExitCode)
  └─ cli.rs  (cli::run — parse, dispatch)
       ├─ config.rs (Config::load / Config::init)
       ├─ deps.rs   (Dependency struct)
       ├─ exec.rs   (sequential sh -c runner)
       └─ errors/   (error types)
            ├─ mod.rs     (module declarations)
            └─ config.rs  (ConfigError)
```

- `cli.rs` is the sole orchestrator: it loads config, dispatches to list/update/init, and formats all user output.
- `config.rs` owns all TOML parsing and filesystem config operations.
- `exec.rs` owns shell spawning — single `sh -c` source for sequential execution.
- `errors/config.rs` contains every error variant; other modules only return `Result<_, ConfigError>`.
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
pnpm lint                   # cargo clippy --all-features -- -D warnings
pnpm lint:fix               # cargo clippy --fix --allow-dirty --all-features

# Format
pnpm format                 # prettier --write . && cargo fmt

# Test
pnpm test                   # cargo test

# CLI test with sample config
pnpm test:cli               # cargo run --release -- --config sample.config.toml

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
- **Shell injection risk** — `src/exec.rs` passes user config values directly to `sh -c` with no sanitization. Users control their own config, but custom config paths from untrusted sources are dangerous.
- **`Cargo.lock` is committed** — binary crate; standard practice.
- **`preserve_order` TOML feature is load-bearing** — removing it silently changes dep execution order.
- **Exit codes**: the CLI returns `ExitCode::SUCCESS` (0) or `ExitCode::FAILURE` (1). Non-zero exit from any dep command makes the whole run fail, even if subsequent deps succeed.

## Unknowns / open questions

- No open unknowns currently; `pnpm test:cli` uses `sample.config.toml`.
