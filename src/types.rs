use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectionResult {
    pub detected: Option<bool>,
    pub source: DetectionSource,
    pub terminal: Option<Terminal>,
    pub font: Option<String>,
    pub config_path: Option<PathBuf>,
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    pub confidence: Confidence,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSource {
    EnvVar,
    ExplicitDisable,
    UnknownTerminal,
    RemoteSession,
    NoResolver,
    ConfigError,
    BundledTerminal,
    TerminalConfig,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Terminal {
    Ghostty,
    WezTerm,
    OpenCode,
    Conductor,
    Kitty,
    Alacritty,
    ITerm2,
    TerminalApp,
    Vscode,
    Zed,
    Hyper,
    Unknown(String),
}

impl Terminal {
    pub fn is_bundled(&self) -> bool {
        matches!(
            self,
            Self::Ghostty | Self::WezTerm | Self::Kitty | Self::OpenCode | Self::Conductor
        )
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Certain,
    Probable,
}
