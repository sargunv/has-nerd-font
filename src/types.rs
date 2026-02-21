use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectionResult {
    pub detected: Option<bool>,
    pub source: DetectionSource,
    pub terminal: Option<Terminal>,
    pub font: Option<String>,
    pub config_path: Option<PathBuf>,
    pub profile: Option<String>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectionSource {
    EnvVar,
    ExplicitDisable,
    UnknownTerminal,
    RemoteSession,
    NoResolver,
    ConfigError { reason: String },
    BundledTerminal,
    TerminalConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
    Certain,
    Probable,
}

impl DetectionResult {
    pub fn exit_code(&self) -> i32 {
        match (&self.source, self.detected) {
            (DetectionSource::EnvVar, _) => 0,
            (DetectionSource::BundledTerminal, _) => 0,
            (DetectionSource::ExplicitDisable, _) => 1,
            (DetectionSource::UnknownTerminal, _) => 2,
            (DetectionSource::RemoteSession, _) => 3,
            (DetectionSource::NoResolver, _) => 4,
            (DetectionSource::ConfigError { .. }, _) => 5,
            (DetectionSource::TerminalConfig, Some(true)) => 0,
            (DetectionSource::TerminalConfig, Some(false)) => 6,
            (DetectionSource::TerminalConfig, None) => 5,
        }
    }

    pub fn explain(&self) -> String {
        match &self.source {
            DetectionSource::EnvVar => "detected Nerd Font from NERD_FONT override".to_string(),
            DetectionSource::ExplicitDisable => {
                "Nerd Font explicitly disabled by NERD_FONT override".to_string()
            }
            DetectionSource::UnknownTerminal => {
                "cannot determine terminal; terminal is unknown".to_string()
            }
            DetectionSource::RemoteSession => {
                "running in remote session; local terminal config not inspected".to_string()
            }
            DetectionSource::NoResolver => {
                "known terminal has no resolver implemented yet".to_string()
            }
            DetectionSource::ConfigError { reason } => {
                format!("failed to read terminal configuration: {reason}")
            }
            DetectionSource::BundledTerminal => {
                "terminal ships with Nerd Font support by default".to_string()
            }
            DetectionSource::TerminalConfig => {
                if self.detected == Some(true) {
                    "terminal configuration indicates a Nerd Font is active".to_string()
                } else if self.detected == Some(false) {
                    "terminal configuration does not indicate a Nerd Font".to_string()
                } else {
                    "terminal configuration status is unknown".to_string()
                }
            }
        }
    }
}
