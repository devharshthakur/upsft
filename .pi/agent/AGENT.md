## About

`upsft` is macOS-only CLI to update selected dependency tools.

Tooling:

- `pnpm` for JS package manager and dev scripts
- `cargo` for Rust build, run, test, fmt, lint

Goal:

- let user pick which deps to manage
- update chosen deps from one CLI

## When writing code for the projecdt

- Keep code macOS-focused unless project scope changes.
- Use `cliclack` for prompts.
- Prefer small, clear modules.
- Avoid cloning operations
- For adding new crates use `cargo add` command instead of editing `cargo.toml` file.

## When creating Creating commits

- First add all the changes using command:

```bash
git add .
```

- Use conventional commit style commit titles,
- Also try to write commit descriptions only if commits is big enought that title cannot give whole context. In that case make title more general with actual details in commit description.
- If no of diffs or changes are large then first suggest dividing into small commits and ask for approval
