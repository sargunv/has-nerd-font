use std::path::Path;

use serde::Deserialize;

use super::{config_error, read_json5_settings, var};
use crate::font::{is_nerd_font, normalize_font_name};
use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

#[derive(Deserialize)]
struct ZedSettings {
    #[serde(default)]
    terminal: Option<ZedTerminal>,
    #[serde(default)]
    buffer_font_family: Option<String>,
}

#[derive(Deserialize)]
struct ZedTerminal {
    #[serde(default)]
    font_family: Option<String>,
}

pub fn resolve(vars: &[(String, String)]) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error(Terminal::Zed, "HOME is not set".to_string(), None),
    };

    let config_path = Path::new(home).join(".config/zed/settings.json");

    let settings = match read_json5_settings::<ZedSettings>(&config_path) {
        Ok(Some(s)) => s,
        Ok(None) => return config_error(Terminal::Zed, "no settings file found".to_string(), None),
        Err(reason) => return config_error(Terminal::Zed, reason, Some(config_path)),
    };

    let font_name = settings
        .terminal
        .as_ref()
        .and_then(|t| t.font_family.as_deref())
        .or(settings.buffer_font_family.as_deref());

    match font_name {
        Some(name) => {
            let font = normalize_font_name(name);
            DetectionResult {
                detected: Some(is_nerd_font(&font)),
                source: DetectionSource::TerminalConfig,
                terminal: Some(Terminal::Zed),
                font: Some(font),
                config_path: Some(config_path),
                profile: None,
                error_reason: None,
                confidence: Confidence::Certain,
            }
        }
        None => config_error(
            Terminal::Zed,
            "no font configured".to_string(),
            Some(config_path),
        ),
    }
}
