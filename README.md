# upsft

**macOS-only** CLI tool to batch-update your project dependencies from a single TOML config file.

[Installation](#installation) · [Usage](#usage) · [Config](#config) · [Exit codes](#exit-codes) · [License](#license)

---

## Installation

```bash
cargo install upsft
```

To update later:

```bash
cargo install --force upsft
```

## Usage

```bash
upsft [OPTIONS]
```

| Flag              | Description                                            |
| ----------------- | ------------------------------------------------------ |
| `-l`, `--list`    | List all managed dependencies                          |
| `--init`          | Create default config at `~/.config/upsft/config.toml` |
| `-c`, `--config`  | Path to custom config file                             |
| `-h`, `--help`    | Print help                                             |
| `-V`, `--version` | Print version                                          |

Run `upsft` with no flags to execute all dependency updates sequentially.

## Config

Place a TOML file at `~/.config/upsft/config.toml` (or point to one with `-c`).

Use `--init` to scaffold a blank config at the default path.

### Example

```toml
[deps]
brew = "brew update && brew upgrade"
rustup = "rustup update"
pnpm = "pnpm self-update"
```

Each key is a dependency name, each value is the shell command to update it.
Commands run **in the order they appear** in the config file (insertion order is preserved).

A sample config is available at [`sample.config.toml`](./sample.config.toml).

### Validation rules

- Dep names must be non-empty and contain only alphanumeric ASCII, `_`, `.`, or `-`
- Each value must be a non-empty string
- The `[deps]` table itself is required

## Exit codes

| Code | Constant  | When                                       |
| ---- | --------- | ------------------------------------------ |
| `0`  | `SUCCESS` | All deps updated successfully              |
| `1`  | `FAILURE` | Any dep failed (subsequent deps still run) |

## License

MIT — see [`LICENSE`](./LICENSE).
