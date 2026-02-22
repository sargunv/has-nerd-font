use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::{config_error, read_toml_settings, var};
use crate::font::{is_nerd_font, normalize_font_name};
use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

#[derive(Deserialize)]
struct AlacrittyConfig {
    #[serde(default)]
    font: Option<AlacrittyFont>,
}

#[derive(Deserialize)]
struct AlacrittyFont {
    #[serde(default)]
    normal: Option<AlacrittyFontNormal>,
}

#[derive(Deserialize)]
struct AlacrittyFontNormal {
    #[serde(default)]
    family: Option<String>,
}

pub fn resolve(vars: &[(String, String)]) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error(Terminal::Alacritty, "HOME is not set".to_string(), None),
    };

    let home_path = Path::new(home);
    let xdg_config_home = var(vars, "XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(Path::new)
        .filter(|p| p.is_absolute());

    let default_config = home_path.join(".config");
    let effective_config_home = xdg_config_home.unwrap_or(&default_config);

    let mut candidates = vec![
        effective_config_home.join("alacritty/alacritty.toml"),
        effective_config_home.join("alacritty.toml"),
    ];
    if xdg_config_home.is_some() {
        // Only add $HOME/.config fallbacks when XDG_CONFIG_HOME is explicitly
        // set, since otherwise they duplicate the paths above.
        candidates.push(home_path.join(".config/alacritty/alacritty.toml"));
    }
    candidates.push(home_path.join(".alacritty.toml"));
    candidates.push(PathBuf::from("/etc/alacritty/alacritty.toml"));

    // Try each candidate in order; use the first file that exists.
    for candidate in &candidates {
        match read_toml_settings::<AlacrittyConfig>(candidate) {
            Ok(Some(config)) => return resolve_from_config(config, candidate),
            Ok(None) => continue, // file not found, try next
            Err(reason) => {
                return config_error(Terminal::Alacritty, reason, Some(candidate.clone()));
            }
        }
    }

    config_error(
        Terminal::Alacritty,
        "no config file found".to_string(),
        None,
    )
}

fn resolve_from_config(config: AlacrittyConfig, config_path: &Path) -> DetectionResult {
    let font_family = config.font.and_then(|f| f.normal).and_then(|n| n.family);

    match font_family {
        Some(family) => {
            let font = normalize_font_name(&family);
            DetectionResult {
                detected: Some(is_nerd_font(&font)),
                source: DetectionSource::TerminalConfig,
                terminal: Some(Terminal::Alacritty),
                font: Some(font),
                config_path: Some(config_path.to_path_buf()),
                profile: None,
                error_reason: None,
                confidence: Confidence::Certain,
            }
        }
        None => config_error(
            Terminal::Alacritty,
            "no font configured".to_string(),
            Some(config_path.to_path_buf()),
        ),
    }
}
