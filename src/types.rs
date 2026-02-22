use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The result of a Nerd Font detection attempt.
///
/// Check [`detected`](Self::detected) for the primary result, and inspect other
/// fields for details about how the decision was made.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectionResult {
    /// Whether a Nerd Font was detected.
    ///
    /// - `Some(true)` — a Nerd Font is available
    /// - `Some(false)` — the font was identified but is not a Nerd Font
    /// - `None` — detection was inconclusive (e.g. unknown terminal, SSH session,
    ///   missing config)
    pub detected: Option<bool>,

    /// How the detection result was determined.
    pub source: DetectionSource,

    /// The terminal emulator that was identified, if any.
    pub terminal: Option<Terminal>,

    /// The font name extracted from the terminal's configuration, if available.
    pub font: Option<String>,

    /// The config file path that was read, if any.
    pub config_path: Option<PathBuf>,

    /// The terminal profile that was inspected, if applicable (e.g. iTerm2 profiles).
    pub profile: Option<String>,

    /// A human-readable error message when detection failed due to a config error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,

    /// How confident the detection result is.
    pub confidence: Confidence,
}

/// How the detection result was determined.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSource {
    /// The `NERD_FONT` environment variable was set to a truthy value.
    EnvVar,
    /// The `NERD_FONT` environment variable was set to a falsy value.
    ExplicitDisable,
    /// The terminal emulator could not be identified.
    UnknownTerminal,
    /// An SSH session was detected; local config files are not accessible.
    RemoteSession,
    /// The terminal was identified but has no config parser implemented.
    NoResolver,
    /// The terminal's config file could not be read or parsed.
    ConfigError,
    /// The terminal ships with built-in Nerd Font support.
    BundledTerminal,
    /// The font was read from the terminal's configuration file.
    TerminalConfig,
}

/// A recognized terminal emulator.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Terminal {
    /// [Ghostty](https://ghostty.org/) — bundles Nerd Font support.
    Ghostty,
    /// [WezTerm](https://wezterm.org/) — bundles Nerd Font support.
    WezTerm,
    /// [OpenCode](https://opencode.ai/) — bundles Nerd Font support.
    OpenCode,
    /// Conductor — bundles Nerd Font support.
    Conductor,
    /// [Kitty](https://sw.kovidgoyal.net/kitty/) — bundles Nerd Font support.
    Kitty,
    /// [Alacritty](https://alacritty.org/) — detected via config file.
    Alacritty,
    /// [iTerm2](https://iterm2.com/) (macOS) — detected via config file.
    ITerm2,
    /// Apple Terminal (macOS) — detected via config file.
    TerminalApp,
    /// [VS Code](https://code.visualstudio.com/) or [VSCodium](https://vscodium.com/) — detected via settings.
    Vscode,
    /// [Zed](https://zed.dev/) — detected via settings.
    Zed,
    /// [Hyper](https://hyper.is/) — identified but no config parser implemented.
    Hyper,
    /// A terminal that was not recognized. Contains the raw identifier string.
    Unknown(String),
}

impl Terminal {
    /// Returns `true` if this terminal ships with built-in Nerd Font support.
    pub fn is_bundled(&self) -> bool {
        matches!(
            self,
            Self::Ghostty | Self::WezTerm | Self::Kitty | Self::OpenCode | Self::Conductor
        )
    }
}

/// How confident the detection result is.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    /// The result is definitive (e.g. env var override, bundled terminal, or
    /// exact font name match).
    Certain,
    /// The result is a best guess (e.g. font name matched heuristically).
    Probable,
}
