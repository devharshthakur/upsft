<img width="981" height="205" alt="upsft_github" src="https://github.com/user-attachments/assets/dbba10cb-9a89-4831-8c6b-e20df2eae5d1" />

# Contents

1. [Installation](#installation)
2. [Usage](#usage)

# Installation

1. Install `upsft` using Cargo:

```bash
cargo install upsft
```

2. To update `upsft` later, run:

```bash
cargo install --force upsft
```

# Usage

```bash
upsft [OPTIONS] [COMMAND]
```

## Commands

| Command      | Alias | Description                                      |
| ------------ | ----- | ------------------------------------------------ |
| `upsft list` | `ls`  | List all managed dependencies                    |
| `upsft init` | —     | Create a new config file at the default location |

## Options

| Short | Long                   | Description                |
| ----- | ---------------------- | -------------------------- |
| `-c`  | `--config-path <PATH>` | Path to custom config file |
| `-h`  | `--help`               | Print help                 |
| `-V`  | `--version`            | Print version              |

## Config Format

```toml
[deps]
"homebrew" = "brew update"
"npm" = "npm update -g"
"pnpm" = "pnpm update -g"
```
