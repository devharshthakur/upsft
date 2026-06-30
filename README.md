# upsft

**macOS-only** CLI that runs cli update commands in batch from a single TOML config file. Use it to run update commands of various globally installed clis you have such as `brew`,`npm`,`pnpm`,`vp` etc

- [About](#about)
- [Installation](#installation)
- [Usage](#usage)
- [Config](#config)
- [Exit codes](#exit-codes)
- [License](#license)

---

## About

`upsft` reads a TOML config listing your tools (Homebrew, rustup, pnpm, etc.) and their update commands, then runs them **sequentially** via `sh -c`. No more hunting down each tool's update incantation.

## Why

I know you will tell me about bash scripts. I did that too. It felt verbose. I am not found of shell scripts for such things, i like them as in project scripts. ClI for me was a cleaner approach, feature addible. There is a config file where you can control the cli behaviour.

## Installation

| Method                                   | Command                                 |
| ---------------------------------------- | --------------------------------------- |
| [Homebrew](https://brew.sh)              | `brew install devharshthakur/tap/upsft` |
| [Cargo](https://doc.rust-lang.org/cargo) | `cargo install upsft`                   |

> [!IMPORTANT]
> `upsft` is **macOS-only**. It relies on `home::home_dir()` and is tested only on macOS.

## Usage

```bash
upsft [OPTIONS]
```

Run with no flags to execute all dependency updates in order.

| Flag                     | Description                                            |
| ------------------------ | ------------------------------------------------------ |
| `-l`, `--list`           | List all managed dependencies and their commands       |
| `--init`                 | Create default config at `~/.config/upsft/config.toml` |
| `-c`, `--config` \<path> | Use a custom config file path                          |
| `-h`, `--help`           | Print help                                             |
| `-V`, `--version`        | Print version                                          |

> [!NOTE]
> `--init` and `--list` are mutually exclusive. `-l`/`--list` displays the number of dependencies followed by one `name = "command"` line each.

## Config

Default location: `~/.config/upsft/config.toml`

Override with `-c <path>`. Use `--init` to scaffold a blank config at the default path (or the custom path given with `-c`).

> [!TIP]
> A ready-to-edit sample is at [`sample.config.toml`](./sample.config.toml).

### Example

```toml
[deps]
brew = "brew update && brew upgrade && brew cleanup"
rustup = "rustup update"
pnpm = "pnpm self-update"
```

### Schema

| Section  | Required | Description                             |
| -------- | -------- | --------------------------------------- |
| `[deps]` | Yes      | Table of `name = "shell command"` pairs |

### Validation rules

| Rule            | Details                                           |
| --------------- | ------------------------------------------------- |
| Dep name        | Non-empty, alphanumeric ASCII + `_` / `.` / `-`   |
| Dep command     | Non-empty string                                  |
| `[deps]` table  | Required — config fails if missing                |
| Execution order | Preserves insertion order (TOML `preserve_order`) |
| Shell           | Each value runs via `sh -c` with no sanitization  |

> [!WARNING]
> Dep commands are passed directly to `sh -c` with **no sanitization**. Only use config files you control. Never point `-c` at untrusted input — this is a shell injection risk by design.

## Exit codes

| Code | Meaning                              |
| ---- | ------------------------------------ |
| 0    | All deps updated successfully        |
| 1    | Config error or a dep command failed |

> [!NOTE]
> A non-zero exit from any single dep command fails the whole run. Subsequent deps are still executed.

## License

MIT — see [`LICENSE`](./LICENSE).
