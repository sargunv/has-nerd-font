# has-nerd-font

[![crates.io](https://img.shields.io/crates/v/has-nerd-font)](https://crates.io/crates/has-nerd-font)
[![docs.rs](https://img.shields.io/docsrs/has-nerd-font)](https://docs.rs/has-nerd-font)
[![license](https://img.shields.io/crates/l/has-nerd-font)](LICENSE)

A CLI tool and Rust library that attempts to detect whether the current terminal
session can render [Nerd Font](https://www.nerdfonts.com/) glyphs. Useful for
conditionally enabling icons in shell configs and CLI tools.

## CLI Usage

`has-nerd-font` communicates primarily through its exit code. With no flags, it
produces no output.

```bash
if has-nerd-font; then
  alias ls='eza --icons=always'
  export STARSHIP_CONFIG="$HOME/.config/starship/nerd.toml"
fi
```

## CLI Installation

### mise

```bash
# prebuilt binary from GitHub Releases
mise use -g "github:sargunv/has-nerd-font"

# or build from crates.io
mise use -g "cargo:has-nerd-font"
```

### chezmoi

Add to your `.chezmoiexternal.toml`, adjusting the asset name for your platform:

```toml
[".local/bin/has-nerd-font"]
type = "archive-file"
url = {{ gitHubLatestReleaseAssetURL "sargunv/has-nerd-font" "has-nerd-font-x86_64-unknown-linux-gnu.tar.gz" | quote }}
executable = true
path = "has-nerd-font"
```

Available assets:

- `has-nerd-font-x86_64-unknown-linux-gnu.tar.gz`
- `has-nerd-font-aarch64-apple-darwin.tar.gz`

### Prebuilt binaries

Download the latest release from
[GitHub Releases](https://github.com/sargunv/has-nerd-font/releases/latest) and
extract it somewhere on your `PATH`:

```bash
# macOS (Apple Silicon)
curl -fsSL https://github.com/sargunv/has-nerd-font/releases/latest/download/has-nerd-font-aarch64-apple-darwin.tar.gz \
  | tar xz -C ~/.local/bin

# Linux (x86_64)
curl -fsSL https://github.com/sargunv/has-nerd-font/releases/latest/download/has-nerd-font-x86_64-unknown-linux-gnu.tar.gz \
  | tar xz -C ~/.local/bin
```

### cargo install

With a [Rust toolchain](https://rustup.rs/) installed:

```bash
cargo install has-nerd-font
```

## Flags

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
| Alacritty              | Checks the configured font family                   |

### Unrecognized terminals

If the terminal can't be identified at all, the answer is no. Set `NERD_FONT=1`
to override.

Feel free to submit a PR to improve detection for your favorite terminal.

## Library Usage

Add the dependency:

```bash
cargo add has-nerd-font
```

Then call `detect()` with the current environment:

```rust
let env_vars: Vec<(String, String)> = std::env::vars().collect();
let result = has_nerd_font::detect(&env_vars);

if result.detected == Some(true) {
    // enable Nerd Font icons
}
```

See the [API documentation](https://docs.rs/has-nerd-font) for details on
`DetectionResult` and related types.
