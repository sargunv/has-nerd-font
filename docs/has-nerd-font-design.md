# `has-nerd-font` — Technical Specification

A fast, dependency-free CLI tool that detects whether the current terminal
session can render Nerd Font glyphs. Analogous to `supports-color` or
`supports-hyperlinks` in the Node/Rust ecosystems.

## Current implementation status

This v1 vertical slice currently implements only the `Terminal.app` config
resolver in Layer 3. The other resolver designs in this document (Kitty,
Alacritty, iTerm2, VSCode, Zed, Hyper, etc.) remain planned and intentionally
return `no_resolver` in the current code.

## Usage

```bash
# Boolean check (exit code only, no stdout)
if has-nerd-font; then
  alias ls='eza --icons=always'
  export STARSHIP_CONFIG="$HOME/.config/starship/nerd.toml"
fi

# Human-readable explanation (for debugging/setup scripts)
$ has-nerd-font --explain
nerd font detected: terminal "ghostty" bundles Nerd Font Symbols

$ has-nerd-font --explain
no nerd font detected: resolved terminal "iterm2", parsed font "Monaco" from
  profile "Default" — not a Nerd Font family

$ has-nerd-font --explain
unable to detect: remote session (SSH_TTY set), cannot read terminal config

# Machine-readable output
$ has-nerd-font --json
{
  "detected": true,
  "source": "bundled_terminal",
  "terminal": "ghostty",
  "confidence": "certain"
}
```

## Exit Codes

Exit codes are ordered by detection cascade — the code tells you where the tool
stopped and why. Lower numbers mean the tool bailed earlier; higher numbers mean
it got further.

| Code | Layer | Meaning                                  | When                                                                                                   |
| ---- | ----- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `0`  | 1     | **Yes** — Nerd Font available            | `NERD_FONT=1`, bundled terminal, or config font matches NF pattern                                     |
| `1`  | 1     | **No** — explicitly disabled             | `NERD_FONT=0` (or `false`/`no`) is set                                                                 |
| `2`  | 1     | **Unknown** — no terminal info           | `TERM_PROGRAM` unset, `TERM` is generic, no known env vars — nothing to go on                          |
| `3`  | 2     | **Unknown** — remote session             | `SSH_TTY` or `SSH_CONNECTION` is set; config files are on the local machine, not reachable             |
| `4`  | 3     | **Unknown** — no resolver for terminal   | Terminal identified but no config parser exists for it (e.g., `TERM_PROGRAM=SomeNewTerminal`)          |
| `5`  | 3     | **Unknown** — config unreadable          | Terminal identified, resolver exists, but config file is missing, unparseable, or font key not present |
| `6`  | 3     | **No** — config resolved, font is not NF | Font name extracted from config, doesn't match NF patterns                                             |

Exit 0 is always definitive yes. Exits 1 and 6 are confident negatives. Exits
2–5 are "don't know."

Future layers (e.g., system font enumeration) would extend from code 7 onward,
preserving cascade order.

## Detection Cascade

Checks run in order. First match wins.

### Layer 1: Environment & Terminal Identification

**Step 1a: Explicit override**

Check `NERD_FONT` env var.

```
NERD_FONT=1  → exit 0 (source: "env_var")
NERD_FONT=0  → exit 1 (source: "env_var", explicitly disabled)
unset        → continue
```

This is the escape hatch. Users or org setup scripts set it in their terminal's
env config. Any truthy/falsy value works: `1`/`0`, `true`/`false`, `yes`/`no`.

**Step 1b: Terminal identification**

Identify the terminal emulator. Used both for "bundles NF" short-circuit and to
dispatch to the correct config parser in Layer 3.

**Detection logic** (checked in order):

1. `TERM_PROGRAM` env var (most terminals set this)
2. `TERM` env var for terminals that use unique values (`xterm-ghostty`,
   `xterm-kitty`, `alacritty`)
3. Terminal-specific env vars as fallback confirmation:
   - `GHOSTTY_RESOURCES_DIR` → ghostty
   - `KITTY_PID` → kitty
   - `WEZTERM_PANE` → wezterm
   - `ITERM_SESSION_ID` → iterm2
   - `VSCODE_*` (any var starting with `VSCODE_`) → vscode
   - `ZED_TERM` → zed

**Terminal registry:**

| Terminal     | `TERM_PROGRAM`   | `TERM`          | Specific env var        | Bundles NF       |
| ------------ | ---------------- | --------------- | ----------------------- | ---------------- |
| Ghostty      | `ghostty`        | `xterm-ghostty` | `GHOSTTY_RESOURCES_DIR` | **yes**          |
| WezTerm      | `WezTerm`        | —               | `WEZTERM_PANE`          | **yes**          |
| Kitty        | —                | `xterm-kitty`   | `KITTY_PID`             | no               |
| Alacritty    | —                | `alacritty`     | —                       | no               |
| iTerm2       | `iTerm.app`      | —               | `ITERM_SESSION_ID`      | no               |
| Terminal.app | `Apple_Terminal` | —               | —                       | no               |
| VSCode       | `vscode`         | —               | `VSCODE_*`              | no               |
| Zed          | `zed`            | —               | `ZED_TERM`              | no               |
| Hyper        | `Hyper`          | —               | —                       | no               |
| tmux         | `tmux`           | `tmux-256color` | `TMUX`                  | **pass-through** |
| screen       | `screen`         | `screen*`       | `STY`                   | **pass-through** |
| Zellij       | —                | —               | `ZELLIJ`                | **pass-through** |

**Multiplexer handling**: tmux, screen, and Zellij are pass-throughs — they
don't control fonts. When detected, the tool should look for the _outer_
terminal. Strategies:

- **tmux**: Parse `tmux display-message -p '#{client_termtype}'` or check the
  parent process tree. `TERM_PROGRAM` from the outer terminal often leaks
  through.
- **screen/Zellij**: Check parent process or fall through to config parsing.

If a multiplexer is detected but the outer terminal can't be identified, treat
it as "unknown terminal" (exit 2).

**Step 1c: Bundled-NF short-circuit**

If the identified terminal is Ghostty or WezTerm → exit 0 immediately (source:
`"bundled_terminal"`). No further checks needed.

If terminal identification fails entirely → exit 2 (source:
`"unknown_terminal"`).

### Layer 2: Remote Session Check

If `SSH_TTY` or `SSH_CONNECTION` is set, this is a remote session. Config files
for the terminal live on the local machine, not on this host — config parsing
cannot work. Exit 3.

Note: Layer 1 still runs before this check. A Ghostty session over SSH is
detectable via `TERM=xterm-ghostty` and exits 0 at Layer 1. Layer 2 only gates
Layer 3 (config parsing).

### Layer 3: Config File Inspection

For terminals that don't bundle NF, parse their config to extract the font
family name.

**Important**: This layer only runs for local sessions where the terminal's
config files are expected to be on the same machine.

#### Config file locations

All paths respect `XDG_CONFIG_HOME` where applicable (default: `~/.config`).

| Terminal     | Config path(s)                                      | Format                        |
| ------------ | --------------------------------------------------- | ----------------------------- |
| Kitty        | `$XDG_CONFIG_HOME/kitty/kitty.conf`                 | key-value (`font_family ...`) |
| Alacritty    | `$XDG_CONFIG_HOME/alacritty/alacritty.toml`, `.yml` | TOML or YAML                  |
| iTerm2       | `~/Library/Preferences/com.googlecode.iterm2.plist` | plist (binary or XML)         |
| Terminal.app | `~/Library/Preferences/com.apple.Terminal.plist`    | plist                         |
| VSCode       | see below                                           | JSON (JSONC)                  |
| Zed          | see below                                           | JSON (JSONC)                  |
| Hyper        | `~/.hyper.js`                                       | JS (best-effort regex)        |

#### VSCode config resolution

VSCode has a layered settings model. Check in order (first defined font wins):

1. **Workspace settings**: `$CWD/.vscode/settings.json`
2. **User settings**:
   - macOS: `~/Library/Application Support/Code/User/settings.json`
   - Linux: `$XDG_CONFIG_HOME/Code/User/settings.json`

Extract the value of `"terminal.integrated.fontFamily"`. If the value is a CSS
font stack (comma-separated), check each family in order — the first Nerd Font
family wins (matching browser/terminal fallback behavior).

#### Zed config resolution

Zed's settings model is similar to VSCode. Check in order (first defined font
wins):

1. **Local project settings**: `$CWD/.zed/settings.json`
2. **User settings**:
   - macOS: `~/.config/zed/settings.json`
   - Linux: `$XDG_CONFIG_HOME/zed/settings.json`

Zed has a dedicated terminal font setting at `terminal.font_family`. If unset,
the terminal inherits `buffer_font_family`. Check both:

1. `terminal.font_family` — if present, this is the terminal font
2. `buffer_font_family` — fallback if terminal font is not explicitly set

If neither is set, Zed uses its bundled `.ZedMono` (Lilex), which is not a Nerd
Font.

#### Profile handling

Several terminals support multiple named profiles with different fonts. The
fundamental problem: **we often can't determine which profile is active from
within the shell.**

**Strategy: any-match with reduced confidence.**

- If the terminal has profiles (iTerm2, Terminal.app, Windows Terminal),
  enumerate _all_ profiles
- If _any_ profile uses a Nerd Font → exit 0, but set confidence to `"probable"`
  instead of `"certain"`
- If _no_ profile uses a Nerd Font → exit 1 (confident negative — no profile
  could render NF)
- If only the _default_ profile can be identified, check that one and report
  accordingly

**Terminal-specific profile behavior:**

| Terminal     | Profile detection                                       | Approach                                                                                                          |
| ------------ | ------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| iTerm2       | Binary plist, multiple profiles under `"New Bookmarks"` | Check `"Normal Font"` field in each profile. The font name is stored as a string like `"MesloLGS-NF-Regular 12"`. |
| Terminal.app | Plist with named settings sets                          | Read `"Default Window Settings"` for the default profile name. Check its font. Can also enumerate all profiles.   |
| Kitty        | No profiles in config (single font_family)              | Simple single-value extraction                                                                                    |
| Alacritty    | No profiles                                             | Simple single-value extraction                                                                                    |
| VSCode       | No terminal profiles with font overrides                | Single value in settings                                                                                          |
| Zed          | No profiles                                             | `terminal.font_family` → `buffer_font_family` fallback chain                                                      |

#### Font name matching

A font name is considered a Nerd Font if it matches any of these patterns
(case-insensitive):

```
*Nerd Font*        — e.g., "JetBrainsMono Nerd Font"
*Nerd Font Mono*   — e.g., "FiraCode Nerd Font Mono"  
*Nerd Font Propo*  — e.g., "Hack Nerd Font Propo"
* NF              — e.g., "CaskaydiaCove NF" (Windows naming)
* NFM             — e.g., "CaskaydiaCove NFM"
* NFP             — e.g., "CaskaydiaCove NFP"
```

Matching must handle:

- Trailing whitespace / quotes from config parsing
- CSS font stacks: `"'JetBrainsMono Nerd Font', monospace"` — split on comma,
  trim, strip quotes, check each
- macOS plist font descriptors: `"MesloLGS-NF-Regular 12"` — strip size suffix,
  match on family

### Layer 3 outcomes

| Situation                                                         | Exit                       |
| ----------------------------------------------------------------- | -------------------------- |
| Font extracted, matches NF pattern                                | 0                          |
| Font extracted, doesn't match NF                                  | 6                          |
| No resolver for identified terminal                               | 4                          |
| Config file missing                                               | 5                          |
| Config file exists but font key not found (using defaults)        | 5                          |
| Config file exists but deserialization failed (TOML, JSON, plist) | 5                          |
| Multiple profiles, at least one NF                                | 0 (confidence: `probable`) |
| Multiple profiles, none NF                                        | 6                          |

## CLI Interface

```
has-nerd-font [OPTIONS]

OPTIONS:
    --explain       Print human-readable detection result to stderr
    --json          Print JSON detection result to stdout

EXIT CODES:
    0  Nerd Font detected
    1  No — explicitly disabled (NERD_FONT=0)
    2  Unknown — no terminal info
    3  Unknown — remote session
    4  Unknown — no resolver for terminal
    5  Unknown — config unreadable
    6  No — config resolved, font is not NF
```

When neither `--explain` nor `--json` is passed, the tool produces **no output**
— it communicates solely via exit code, like `test(1)`.

`--explain` writes to **stderr** so it can be used alongside exit code checks
without capturing stdout:

```bash
if has-nerd-font --explain; then
  echo "nerd fonts available"
fi
# stderr shows: "nerd font detected: terminal "ghostty" bundles Nerd Font Symbols"
# stdout is clean for the if-check
```

### JSON output schema

```json
{
  "detected": true | false | null,
  "source": "env_var" | "explicit_disable" | "bundled_terminal" | "terminal_config" | "unknown_terminal" | "remote_session" | "no_resolver" | "config_error",
  "terminal": "ghostty" | "kitty" | "vscode" | "zed" | ... | null,
  "font": "JetBrainsMono Nerd Font" | null,
  "config_path": "/Users/x/.config/kitty/kitty.conf" | null,
  "profile": "Default" | null,
  "confidence": "certain" | "probable",
  "exit_code": 0
}
```

`detected` is `true` (exit 0), `false` (exit 1/6), or `null` (exit 2/3/4/5 —
couldn't determine).

## Project Structure

```
has-nerd-font/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point, arg parsing, output formatting
│   ├── lib.rs               # Public API: detect() → DetectionResult
│   ├── env.rs               # Layer 0: NERD_FONT env var check
│   ├── terminal.rs          # Layer 1: terminal identification
│   ├── config/
│   │   ├── mod.rs           # Config dispatcher + font name matching
│   │   ├── kitty.rs
│   │   ├── alacritty.rs
│   │   ├── iterm2.rs        # plist parsing
│   │   ├── terminal_app.rs  # plist parsing
│   │   ├── vscode.rs        # JSON w/ cwd-aware workspace resolution
│   │   ├── zed.rs           # JSON w/ cwd-aware project resolution
│   │   └── hyper.rs         # Best-effort JS regex
│   └── types.rs             # DetectionResult, Terminal, Confidence, etc.
```

## Core API

```rust
pub fn detect(
    vars: &HashMap<String, String>,
    cwd: &Path,
) -> DetectionResult
```

`vars` is the environment variable map (production:
`std::env::vars().collect()`, tests: hand-built). `cwd` is the working directory
for VSCode/Zed workspace settings resolution (production:
`std::env::current_dir()`, tests: a tempdir path).

Resolvers derive paths internally:

```rust
let home = vars.get("HOME")?;
let config_home = vars.get("XDG_CONFIG_HOME")
    .unwrap_or(&format!("{home}/.config"));
```

```rust
pub struct DetectionResult {
    pub detected: Option<bool>,     // None = unknown
    pub source: DetectionSource,
    pub terminal: Option<Terminal>,
    pub font: Option<String>,
    pub config_path: Option<PathBuf>,
    pub profile: Option<String>,
    pub confidence: Confidence,
}

pub enum DetectionSource {
    EnvVar,              // exit 0
    ExplicitDisable,     // exit 1
    UnknownTerminal,     // exit 2
    RemoteSession,       // exit 3
    NoResolver,          // exit 4
    ConfigError { reason: String }, // exit 5
    BundledTerminal,     // exit 0
    TerminalConfig,      // exit 0 or 6
}

pub enum Terminal {
    Ghostty,
    WezTerm,
    Kitty,
    Alacritty,
    ITerm2,
    TerminalApp,
    Vscode,
    Zed,
    Hyper,
    Unknown(String),  // stores raw TERM_PROGRAM value
}

pub enum Confidence {
    Certain,   // single font resolved, or env var explicit
    Probable,  // multi-profile with at least one NF match
}

impl DetectionResult {
    pub fn exit_code(&self) -> i32 { ... }
    pub fn explain(&self) -> String { ... }
}
```

## Dependencies

| Crate                  | Purpose                                                   | Notes                                                       |
| ---------------------- | --------------------------------------------------------- | ----------------------------------------------------------- |
| `clap` (derive)        | CLI arg parsing, `--help`, `--version`, shell completions | Handles `--cwd <PATH>`, `--explain`, `--json` flags cleanly |
| `serde` + `serde_json` | `--json` output, VSCode/Zed settings parsing              | Also needed for any JSONC config files                      |
| `plist`                | macOS iTerm2/Terminal.app plist reading                   | Only compiled on macOS (`#[cfg(target_os = "macos")]`)      |
| `toml`                 | Alacritty config parsing                                  | Small, no-std-compatible                                    |

Avoid: `regex` (NF name matching is simple `contains` / `ends_with` on
lowercased strings), `glob` (same reason), `font-kit` (system font enumeration
is out of scope).

## Performance Target

**< 5ms** total wall time for the common case (env var hit or bundled terminal).
**< 20ms** for config file parsing paths. This is shell-init-hot-path code.

Config files should be read with a single `fs::read_to_string` and scanned
linearly. No AST parsing of TOML/YAML — use line-by-line extraction for simple
formats (Ghostty, Kitty). Only use a real parser for formats where line
extraction is unreliable (TOML sections in Alacritty, JSON in VSCode, plist in
iTerm2).

## Testing Strategy

All detection logic is tested through integration tests. The two arguments to
`detect()` are the primary test seam — tests pass a hand-built `HashMap` for env
vars and a `tempfile::TempDir` path for cwd, avoiding any interaction with the
real process environment or filesystem.

Tests construct a tempdir, set `HOME` in the vars map to point at it, and lay
out config files underneath. Every resolver derives paths from the vars map, so
tests run in parallel without races.

**Config file tests**: Create realistic config files (Kitty, Alacritty TOML,
VSCode/Zed JSON, iTerm2/Terminal.app plist) under the tempdir. All paths resolve
from `env.home` or `env.config_home`, including macOS `~/Library/Preferences/`
plists — just `$HOME/Library/Preferences/...` under the tempdir. Test both
positive cases (NF font configured) and negative cases (non-NF font, missing
font key, missing config file, malformed config).

**Terminal identification tests**: Set `TERM_PROGRAM`, `TERM`, and
terminal-specific vars in `env.vars`. Verify correct terminal detection,
bundled-NF short-circuit, and multiplexer pass-through behavior.

**SSH tests**: Set `SSH_TTY` or `SSH_CONNECTION` in `env.vars` alongside a
terminal identity. Verify Layer 2 gates config parsing and produces exit 3.

**Exit code tests**: A small set of end-to-end tests using `assert_cmd` that
invoke the real binary with controlled env vars and verify exit codes and
`--json` output.

### Dev Dependencies

| Crate        | Purpose                                             |
| ------------ | --------------------------------------------------- |
| `tempfile`   | Temp dirs for config file layout                    |
| `assert_cmd` | End-to-end binary exit code and output verification |

## Publishing

Publish as both a library crate and a binary.

The library exposes `has_nerd_font::detect(&Environment) -> DetectionResult` so
that other Rust tools (starship, eza, etc.) can link against it directly without
shelling out. The binary is a thin wrapper that builds an `Environment` from the
real process environment and formats the result.

```toml
# Cargo.toml
[lib]
name = "has_nerd_font"

[[bin]]
name = "has-nerd-font"
```

The library is the primary artifact. The binary is a convenience for shell
scripts and dotfiles.

### Distribution

- **crates.io**: Publish the crate so Rust tools can depend on it and users can
  `cargo install has-nerd-font`
- **Homebrew**: Provide a formula for `brew install has-nerd-font` (primary
  distribution for macOS engineering orgs)
- **GitHub releases**: Pre-built binaries for macOS (aarch64, x86_64) and Linux
  (x86_64, aarch64)

Things explicitly out of scope for v1:

- **System font enumeration**: Checking whether any Nerd Font is installed on
  the system (via `font-kit`, `fc-list`, CoreText). This is a useful signal but
  doesn't confirm the terminal is using it, and adds a heavyweight dependency.
  Could be a future Layer 4 with exit codes extending from 7 onward.
- **Glyph probing via cursor position reporting**: Unreliable for single-width
  PUA characters and requires terminal interaction (not suitable for non-TTY).
- **`TERM_FEATURES` / capability negotiation**: If a standard emerges for
  terminals to advertise font features, support it.
- **Windows support**: Windows Terminal, PowerShell, cmd.exe config detection.
  Different config paths and formats.
- **Library crate**: Expose `has_nerd_font::detect()` as a library for other
  Rust tools (starship, eza, etc.) to link against.
