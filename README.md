# upsft

macOS CLI tool to batch-update your project dependencies from a single config file.

[Installation](#installation) · [Usage](#usage) · [Config](#config)

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

| Flag               | Description                                            |
| ------------------ | ------------------------------------------------------ |
| `-l`, `--list`     | List all managed dependencies                          |
| `-P`, `--parallel` | Run update commands in parallel                        |
| `--init`           | Create default config at `~/.config/upsft/config.toml` |
| `-c`, `--config`   | Path to custom config file                             |
| `-h`, `--help`     | Print help                                             |
| `-V`, `--version`  | Print version                                          |

## Config

Place a TOML file at `~/.config/upsft/config.toml` (or point to one with `-c`).

```toml
[deps]
brew = "brew update && brew upgrade"
rustup = "rustup update"
pnpm = "pnpm self-update"
```

Each key is a dependency name, each value is the shell command to update it. Commands run sequentially by default; use `-P` for parallel execution.
