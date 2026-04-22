<img width="981" height="205" alt="upsft_github" src="https://github.com/user-attachments/assets/dbba10cb-9a89-4831-8c6b-e20df2eae5d1" />

# About

It is a simple cli tool, which you can use to update multiple dependencies.

# Usage

1. Setup the config file `config.toml` you can create it using `init` command

```bash
upsft init
```

> This creates a `config.toml` file in `~/.config/upsft/config.toml` which is default location for configs in a mac

2. Now fill the config toml see the below example

```toml
[deps]
"homebrew" = "brew update"
"npm" = "npm update -g"
"pnpm" = "pnpm update -g"
```

3. Now save the file and run upsft from terminal from home directory, it will start executing update commands

```bash
upsft
```
