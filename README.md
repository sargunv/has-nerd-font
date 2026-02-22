# has-nerd-font

A fast, dependency-free CLI tool that detects whether the current terminal
session can render [Nerd Font](https://www.nerdfonts.com/) glyphs. Useful for
conditionally enabling icons in your shell configs.

## Usage

`has-nerd-font` communicates primarily through its exit code. With no flags, it
produces no output

```bash
if has-nerd-font; then
  alias ls='eza --icons=always'
  export STARSHIP_CONFIG="$HOME/.config/starship/nerd.toml"
fi
```

## Installation

TODO

### Flags

```
has-nerd-font [OPTIONS]

OPTIONS:
    --explain    Print a human-readable explanation to stderr
    --json       Print a machine-readable JSON result to stdout
```

`--explain` writes to stderr:

```bash
has-nerd-font --explain
```

```
terminal ships with Nerd Font support by default
```

`--json` outputs a structured result to stdout:

```bash
has-nerd-font --json | jq .
```

```json
{
  "detected": true,
  "source": "bundled_terminal",
  "terminal": "ghostty",
  "font": null,
  "config_path": null,
  "profile": null,
  "confidence": "certain"
}
```

Since they write to different output streams, `--explain` and `--json` can be
used together.

```bash
has-nerd-font --explain --json | jq .
```

## Exit codes

Exit codes follow a detection cascade — lower numbers mean the tool stopped
earlier, higher numbers mean it got further through the detection layers.

| Code | Meaning                     | When                                                         |
| ---- | --------------------------- | ------------------------------------------------------------ |
| `0`  | Nerd Font available         | `NERD_FONT=1`, bundled terminal, or config font matches      |
| `1`  | Explicitly disabled         | `NERD_FONT=0` (or `false`/`no`) is set                       |
| `2`  | Unknown — no terminal info  | Terminal could not be identified                             |
| `3`  | Unknown — remote session    | SSH detected; local config files not reachable               |
| `4`  | Unknown — no resolver       | Terminal identified but no config parser exists for it       |
| `5`  | Unknown — config unreadable | Config file missing, unparseable, or font key absent         |
| `6`  | Not a Nerd Font             | Font extracted from config, doesn't match Nerd Font patterns |

## Supported terminals

### Always yes (bundled Nerd Fonts)

These terminals ship with built-in Nerd Font support. Detection is immediate.

| Terminal  |
| --------- |
| Ghostty   |
| WezTerm   |
| Kitty     |
| OpenCode  |
| Conductor |

### Config file detection

For these terminals, the tool reads config files to determine the active font.
If the font name looks like a Nerd Font, the answer is yes.

| Terminal               | Notes                                               |
| ---------------------- | --------------------------------------------------- |
| iTerm2 (macOS)         | Checks the active profile's font                    |
| Apple Terminal (macOS) | Checks the default profile's font                   |
| VS Code                | Project settings first, falls back to user settings |
| VSCodium               | Project settings first, falls back to user settings |
| Zed                    | Project settings first, falls back to user settings |
| Alacritty              | TODO                                                |

### Unrecognized terminals

If the terminal can't be identified at all, the answer is no. Set `NERD_FONT=1`
to override.

Feel free to submit a PR to improve detection for your favorite terminal.
