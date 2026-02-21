use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

impl Terminal {
    pub fn is_bundled(&self) -> bool {
        matches!(self, Self::Ghostty | Self::WezTerm)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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
            (DetectionSource::ConfigError, _) => 5,
            (DetectionSource::TerminalConfig, Some(true)) => 0,
            (DetectionSource::TerminalConfig, Some(false)) => 6,
            (DetectionSource::TerminalConfig, None) => 5,
        }
    }

    pub fn to_json_value(&self) -> Value {
        let mut value = serde_json::to_value(self).expect("failed to serialize detection result");
        if let Value::Object(ref mut object) = value {
            let terminal_json = self
                .terminal
                .as_ref()
                .map(Terminal::to_json_contract_value)
                .unwrap_or(Value::Null);
            object.insert("terminal".to_string(), terminal_json);
            object.insert("exit_code".to_string(), Value::from(self.exit_code()));
        }
        value
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
            DetectionSource::ConfigError => format!(
                "failed to read terminal configuration: {}",
                self.error_reason.as_deref().unwrap_or("unknown reason")
            ),
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

impl Terminal {
    fn to_json_contract_value(&self) -> Value {
        match self {
            Self::Unknown(raw) => Value::String(raw.clone()),
            _ => serde_json::to_value(self).expect("failed to serialize terminal"),
        }
    }
}
