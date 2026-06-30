# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.2] - 2026-06-30

### Changed

- `cargo-dist` setup by @devharshthakur
- Add create github release by @devharshthakur
- Merge branch 'main' of https://github.com/devharshthakur/upsft by @devharshthakur

### Documentation

- Update `AGENTS.md` by @devharshthakur
- Update by @devharshthakur
- Update by @github-actions[bot]

### Fixed

- Dist diff error by @devharshthakur

### Miscellaneous Tasks

- Version bump by @devharshthakur
- Version bump by @devharshthakur
- Bump version by @devharshthakur

### New Contributors

- @github-actions[bot] made their first contribution

## [1.1.1] - 2026-06-29

### Fixed

- Version bump by @devharshthakur

## [1.1.0] - 2026-06-29

### Added

- Add Executor trait, ShellExecutor, and runner functions by @devharshthakur
- Running commands concurrently by @devharshthakur
- Render deps in a table format by @devharshthakur

### Changed

- Add `CHANGELOG` update and commit action by @devharshthakur
- Release workflows added by @devharshthakur
- Create a separate command execution function by @devharshthakur
- Removed all implementations of parallel execution by @devharshthakur
- Merge pull request #30 from devharshthakur/cleanups by @devharshthakur in [#30](https://github.com/devharshthakur/upsft/pull/30)
- Exec modulue by @devharshthakur
- Merge branch 'cleanup' into fix/executor-abstraction by @devharshthakur
- Drop lib crate, simplify exec, restructure CLI by @devharshthakur
- Rename OutputSink→OutputHandler, *Sink→*Handler by @devharshthakur
- Moved inline import to module scope by @devharshthakur
- Merge branch 'audit-fixes' into fix/executor-abstraction by @devharshthakur
- Idiomatic cleanups in config validation by @devharshthakur
- Strip tests, move DepMsg/ChannelSink to module level by @devharshthakur
- Remove redundant doc comments from config and deps by @devharshthakur
- Switch update dispatch to executor runners by @devharshthakur
- Merge pull request #21 from devharshthakur/devharshthakur/issue12 by @devharshthakur in [#21](https://github.com/devharshthakur/upsft/pull/21)
- Merge branch 'feat/display-deps' by @devharshthakur

### Documentation

- Removed parallel execution by @devharshthakur
- Update AGENTS.md and README.md for -P flag and refactors by @devharshthakur
- Update AGENTS.md and CHANGELOG for executor refactor by @devharshthakur

### Fixed

- Lowercase error messages per Rust API guidelines by @devharshthakur
- Fix the release script by @devharshthakur

### Miscellaneous Tasks

- Clean up JS dev dependencies and TypeScript config by @devharshthakur
- Remove unused serde and tabled dependencies by @devharshthakur
- Add more options by @devharshthakur
- Separate lint:fix command from lint by @devharshthakur
- Remove unused serde::Deserialize derive from Dependency by @devharshthakur
- Bump deps by @devharshthakur
- Add rust files by @devharshthakur
- Remove old cmd and execute modules, dead UpsftError by @devharshthakur

## [1.0.0] - 2026-06-16

### Added

- Validate deps at parse time by @devharshthakur

### Changed

- Merge pull request #13 from devharshthakur/feat/refactor-modules-unify-errors by @devharshthakur in [#13](https://github.com/devharshthakur/upsft/pull/13)
- Moved config error to error file by @devharshthakur
- Extract library crate from main by @devharshthakur
- Add home crate, unify errors, extract cmd module by @devharshthakur
- Merge branch 'new-task-branch' by @devharshthakur
- Bump version to 1.0.0 by @devharshthakur
- Merge branch 'main' of https://github.com/devharshthakur/upsft by @devharshthakur
- Bump version to 1.0.0 by @devharshthakur
- Merge pull request #11 from devharshthakur/feat/graphify by @devharshthakur in [#11](https://github.com/devharshthakur/upsft/pull/11)

### Documentation

- Update by @devharshthakur
- Version bump by @devharshthakur
- Add changlog scripts by @devharshthakur
- Removed banner by @devharshthakur
- Simplify README with concise tables and config example by @devharshthakur
- Update `AGENT.md` by @devharshthakur

### Fixed

- Preserve config execution order by @devharshthakur

### Miscellaneous Tasks

- Using experimental cli by @devharshthakur
- Add doc comments to Dependency struct and methods by @devharshthakur
- Remove unused dependencies and configure separate clippy config by @devharshthakur
- Add VSCode settings by @devharshthakur
- Configure pnpm workspace by @devharshthakur
- Update tsconfig for restructured project by @devharshthakur
- Migrate prettier config to flat file by @devharshthakur
- Remove deprecated release scripts by @devharshthakur
- Remove graphify plugin and related files by @devharshthakur
- Update JS/TS devDependencies by @devharshthakur
- Setup `graphify` by @devharshthakur
- Update changelog by @devharshthakur

## [0.0.3] - 2026-04-26

### Added

- Created `CHANGELOG.md` by @devharshthakur
- Add config validation for keys and values by @devharshthakur

### Changed

- Merge pull request #10 from devharshthakur/chore/changelog by @devharshthakur in [#10](https://github.com/devharshthakur/upsft/pull/10)
- Update `sample.config.toml` by @devharshthakur
- Merge pull request #9 from devharshthakur/feat/config-validation by @devharshthakur in [#9](https://github.com/devharshthakur/upsft/pull/9)
- Moved `config.toml` for testing by @devharshthakur
- Refactor code depended on `Dependency` struct by @devharshthakur
- Cleanedup `config.rs` by @devharshthakur
- Removed dead code and add comments by @devharshthakur
- `config.rs` by @devharshthakur

### Documentation

- Add issue guidelines by @devharshthakur

### Fixed

- Changelog by @devharshthakur
- Use gh token for git-cliff auth by @devharshthakur
- `changelog` script by @devharshthakur
- `changelog` script by @devharshthakur
- Some patches by @devharshthakur
- Rewrote `Dependecy` struct by @devharshthakur
- Complete `init_config` implementation and add comments by @devharshthakur
- Removed error module by @devharshthakur
- Readme by @devharshthakur

### Miscellaneous Tasks

- Update changelog by @devharshthakur
- Adjust scripts by @devharshthakur
- Version bump by @devharshthakur
- Rename `release.ts` to `changelog.ts` by @devharshthakur
- Update `release.ts` by @devharshthakur
- Add pnpm scripts by @devharshthakur
- Add release automation scripts by @devharshthakur
- Add git-cliff config by @devharshthakur
- Update changelog by @devharshthakur
- Add release build optimizations by @devharshthakur
- Setup a `test:cli` command by @devharshthakur
- Add `rimraf` by @devharshthakur
- Move from `pi` to `opencode` by @devharshthakur
- Version bump by @devharshthakur

## [0.0.2] - 2026-04-23

### Changed

- `Vec` type by @devharshthakur
- Merge pull request #4 from devharshthakur/fix/cli-args by @devharshthakur in [#4](https://github.com/devharshthakur/upsft/pull/4)

### Documentation

- Update readme by @devharshthakur

### Fixed

- Cli args by @devharshthakur

## [0.0.1] - 2026-04-22

### Added

- Init config by @devharshthakur
- Add `list` command support by @devharshthakur
- `config` and `error` and completing `main.rs` by @devharshthakur
- Wire cli into main with update/list flows by @devharshthakur
- Add clap cli arg parsing by @devharshthakur
- Default deps by @devharshthakur
- `error.rs` by @devharshthakur
- Config and deps by @devharshthakur
- `util.rs` by @devharshthakur

### Changed

- Merge pull request #3 from devharshthakur/feat/init-config by @devharshthakur in [#3](https://github.com/devharshthakur/upsft/pull/3)
- Return ExitCode from cli entrypoint by @devharshthakur
- Return config loading errors instead of exiting by @devharshthakur
- Merge pull request #2 from devharshthakur/refactor/codebase by @devharshthakur in [#2](https://github.com/devharshthakur/upsft/pull/2)
- Rewrite `cli.rs` by @devharshthakur
- refactor: move `deps` module to `src/deps.rs` (remove `deps` subdirectory) by @devharshthakur
- Merge pull request #1 from devharshthakur/feat/config by @devharshthakur in [#1](https://github.com/devharshthakur/upsft/pull/1)
- `util` module by @devharshthakur
- Use match statements instead of panic by @devharshthakur
- `deps` module by @devharshthakur
- Shift `agent.md` file by @devharshthakur
- Code clean by @devharshthakur
- Deleted `config` and `error` files by @devharshthakur
- Update util.rs by @devharshthakur
- Initial commit by @devharshthakur

### Documentation

- `readme.md` and `cargo.toml` by @devharshthakur
- Add comments and format `readme.md` by @devharshthakur
- `readme.md` by @devharshthakur
- Add coding conventions by @devharshthakur
- `AGENTS.md` by @devharshthakur

### Fixed

- Error message for empty config file by @devharshthakur
- `lint-stage` config by @devharshthakur
- Execute commands via shell for && support by @devharshthakur
- Avoid clone operation by @devharshthakur
- Clippy config error by @devharshthakur
- `clippy` by @devharshthakur
- `clippy` fix so lint suceeds by @devharshthakur
- Add lock file to vc by @devharshthakur
- Lintstage for rs by @devharshthakur

### Miscellaneous Tasks

- Simplify `lint` script and in `lint-stage` by @devharshthakur
- Add `toml` and tracing dependencies by @devharshthakur
- Update agent documentation with commit guidelines by @devharshthakur
- Add inline comment in Dependency::new by @devharshthakur
- Install `toml` by @devharshthakur
- Separate `clippy` config by @devharshthakur
- `pnpm format && pnpm lint` by @devharshthakur
- Add deps by @devharshthakur
- Fmt by @devharshthakur
- Prettier by @devharshthakur
- Husky by @devharshthakur
- Gitginore by @devharshthakur

### New Contributors

- @devharshthakur made their first contribution

[1.1.0]: https://github.com/devharshthakur/upsft/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/devharshthakur/upsft/compare/v0.0.3...v1.0.0
[0.0.3]: https://github.com/devharshthakur/upsft/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/devharshthakur/upsft/compare/v0.0.1...v0.0.2

<!-- generated by git-cliff -->
