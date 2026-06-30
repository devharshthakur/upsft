# Contributing to upsft

First off, thanks for taking the time to contribute!

The following is a set of guidelines for contributing to `upsft`. These are
mostly guidelines, not rules. Use your best judgment, and feel free to propose
changes to this document in a pull request.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Issue-Driven Development](#issue-driven-development)
- [Getting Started](#getting-started)
  - [Ask First](#ask-first)
  - [Proposing Something New](#proposing-something-new)
- [Working on an Issue](#working-on-an-issue)
  - [Branch Naming](#branch-naming)
  - [Commit Style](#commit-style)
  - [Code Comments & Documentation](#code-comments--documentation)
- [Pull Requests](#pull-requests)
  - [PR Body](#pr-body)
  - [PR Template](#pr-template)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)

---

## Code of Conduct

This project and everyone participating in it is governed by the
[Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating,
you are expected to uphold this code. Please report unacceptable behavior to
[harshprojects2002@outlook.com](mailto:harshprojects2002@outlook.com).

---

## Issue-Driven Development

**All significant work must start with an issue.** This includes:

- New features or enhancements
- Complex bug fixes
- Refactors that change behaviour or structure
- Any change that touches multiple modules or requires discussion

Trivial fixes — typos in docs, one-line comment corrections — may skip the
issue step, but a PR is still required.

---

## Getting Started

### Ask First

Before working on any existing issue, **comment on the issue to let others know
you're picking it up**. This avoids duplicate effort and allows me to
provide guidance or context before you start coding.

If someone is already assigned to an issue or has commented saying they're
working on it, respect their claim unless they've been inactive for a while.

### Proposing Something New

If you have an idea for a feature, an improvement, or a change that doesn't
have an existing issue:

1. **Open an issue first** — describe the problem you're trying to solve and
   your proposed approach.
2. Wait for my feedback before writing code.
3. Once there's consensus, start working (see [Working on an Issue](#working-on-an-issue)).

This saves everyone time — your approach might need adjustments, or the change
might already be in progress.

---

## Working on an Issue

### Branch Naming

Create a branch off `main` for your work. Use the following naming convention:

```
issue/<issue-number>
```

For example, if you're working on issue #42:

```bash
git checkout -b issue/42
```

If the branch needs a short descriptive suffix for clarity, append it with a
hyphen:

```bash
git checkout -b issue/42-add-validation
```

### Commit Style

- **Small, meaningful commits.** Each commit should represent a single logical
  change. Ask yourself: "If this commit is the only one that gets reviewed,
  does it make sense on its own?"
- **No large commits.** Large, monolithic commits — especially those generated
  by AI agents — will be **rejected without review**. Split your work into
  focused, reviewable pieces.
- **Fewer commits is fine** as long as each one is meaningful. Don't pad your
  history. The goal is clarity, not quantity.
- **Conventional Commits format** should be followed for messages. Examples:
  - `feat: add --parallel flag for concurrent execution`
  - `fix: handle empty config file gracefully`
  - `docs: update README with new CLI flags`
  - `refactor: extract config validation into separate module`

### Code Comments & Documentation

- Add comments to explain **why** something is done a certain way, not just
  **what** is done. The code already says what it does.
- If you're adding or changing a public function, include a doc comment
  (`///` or `//!` in Rust) describing its purpose, parameters, and return
  value.
- Comments help code review go faster. If a reviewer can understand your
  intent without asking, everyone wins.

---

## Pull Requests

1. **Reference the issue** your PR closes. Use one of these keywords in the
   PR description:
   - `Closes #42`
   - `Fixes #42`
   - `Resolves #42`

   If the PR only partially addresses an issue, say so explicitly.

2. **Keep PRs focused.** One PR should address one issue. Avoid bundling
   unrelated changes.

3. **PR body must be written by hand.** Please do not use AI agents to
   generate PR descriptions. A hand-written body helps reviewers understand
   your thinking and the changes you made. Explain:

   - **What** the change does (in plain language)
   - **Why** this approach was taken
   - **How** it works (use code terms where relevant — function names, types,
     modules)
   - **Any caveats** or things to watch out for

4. **Use the PR template** if one is provided (see below).

### PR Template

A [pull request template](.github/PULL_REQUEST_TEMPLATE.md) is available in
the repository. Use it when opening a PR — it includes a checklist to help
you cover all the requirements.

Similarly, use the [issue templates](.github/ISSUE_TEMPLATE/) when reporting
bugs or proposing features.

---

## Development Setup

Prerequisites:

- [Rust](https://rustup.rs) (see `rust-toolchain.toml` for the required
  version)
- [pnpm](https://pnpm.io) (for dev tooling and git hooks)
- macOS (this project is macOS-only by design)

```bash
# Install JS dev tooling (pre-commit hooks, etc.)
pnpm install

# Run during development
pnpm dev

# Lint
pnpm lint

# Format
pnpm format

# Test
pnpm test

# Test with sample config
pnpm test:cli
```

See [`package.json`](package.json) for all available scripts.

---

## Project Structure

```
src/
├── main.rs              # Entrypoint
├── cli.rs               # CLI parsing and dispatch
├── config.rs            # Config load/init/parse
├── deps.rs              # Dependency struct
├── exec.rs              # Sequential sh -c runner
└── errors/
    ├── mod.rs
    └── config.rs        # ConfigError type
```

Key conventions:

- `cli.rs` is the sole orchestrator — reads config, dispatches to list/update/init,
  formats all user output.
- `config.rs` owns all TOML parsing and filesystem config operations.
- `exec.rs` owns shell spawning — single `sh -c` runner.
- Dependencies preserve insertion order from the TOML file.
- Errors flow through `ConfigError` (thiserror).

---

## Code Review

All submissions require review via GitHub pull requests.
I have final say on all changes.

Expect feedback on:

- Correctness — does the code do what it claims?
- Safety — are there edge cases or injection risks?
- Style — does it match the existing codebase conventions?
- Commit hygiene — are commits small and meaningful?

I aim to review PRs within a few days. If you haven't heard back, feel free
to nudge.

---

## Questions?

Open a [discussion](https://github.com/devharshthakur/upsft/discussions) or
ask in the issue you're working on.

Happy contributing!
