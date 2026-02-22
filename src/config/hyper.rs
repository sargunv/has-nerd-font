use std::path::Path;

use serde::Deserialize;

use super::{config_error, var};
use crate::font::{is_nerd_font, normalize_font_name};
use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

#[derive(Deserialize)]
struct HyperJson {
    #[serde(default)]
    config: Option<HyperConfig>,
}

#[derive(Deserialize)]
struct HyperConfig {
    #[serde(default, rename = "fontFamily")]
    font_family: Option<String>,
}

pub fn resolve(vars: &[(String, String)]) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error(Terminal::Hyper, "HOME is not set".to_string(), None),
    };

    let home_path = Path::new(home);

    // Hyper 4 uses XDG_CONFIG_HOME, falling back to ~/.config
    let config_dir = match var(vars, "XDG_CONFIG_HOME") {
        Some(value) if !value.is_empty() => Path::new(value).join("Hyper"),
        _ => home_path.join(".config/Hyper"),
    };
    let config_path = config_dir.join("hyper.json");

    let content = match std::fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return config_error(
                Terminal::Hyper,
                "no config file found".to_string(),
                Some(config_path),
            );
        }
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return config_error(
                Terminal::Hyper,
                format!("permission denied reading {}", config_path.display()),
                Some(config_path),
            );
        }
        Err(e) => {
            return config_error(
                Terminal::Hyper,
                format!("failed to read {}: {e}", config_path.display()),
                Some(config_path),
            );
        }
    };

    let hyper: HyperJson = match serde_json::from_str(&content) {
        Ok(parsed) => parsed,
        Err(e) => {
            return config_error(
                Terminal::Hyper,
                format!(
                    "failed to parse {} at line {} column {}",
                    config_path.display(),
                    e.line(),
                    e.column()
                ),
                Some(config_path),
            );
        }
    };

    let font = hyper.config.as_ref().and_then(|c| c.font_family.as_deref());

    match font {
        Some(font_name) => {
            let font = normalize_font_name(font_name);
            DetectionResult {
                detected: Some(is_nerd_font(&font)),
                source: DetectionSource::TerminalConfig,
                terminal: Some(Terminal::Hyper),
                font: Some(font),
                config_path: Some(config_path),
                profile: None,
                error_reason: None,
                confidence: Confidence::Certain,
            }
        }
        None => config_error(
            Terminal::Hyper,
            "no font configured".to_string(),
            Some(config_path),
        ),
    }
}
