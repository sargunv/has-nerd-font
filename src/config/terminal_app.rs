use std::path::PathBuf;

use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

const HOME_KEY: &str = "HOME";
const DEFAULT_WINDOW_SETTINGS_KEY: &str = "Default Window Settings";
const WINDOW_SETTINGS_KEY: &str = "Window Settings";

pub fn resolve(vars: &[(String, String)]) -> DetectionResult {
    let home = match var(vars, HOME_KEY) {
        Some(value) if !value.is_empty() => value,
        _ => return config_error("HOME is not set".to_string(), None, None),
    };

    let config_path = PathBuf::from(home).join("Library/Preferences/com.apple.Terminal.plist");

    resolve_from_plist(vars, config_path)
}

#[cfg(target_os = "macos")]
fn resolve_from_plist(_vars: &[(String, String)], config_path: PathBuf) -> DetectionResult {
    let value = match plist::Value::from_file(&config_path) {
        Ok(value) => value,
        Err(err) => {
            return config_error(
                format!("failed to read plist: {err}"),
                None,
                Some(config_path),
            );
        }
    };

    let root = match value.as_dictionary() {
        Some(root) => root,
        None => {
            return config_error(
                "terminal plist root is not a dictionary".to_string(),
                None,
                Some(config_path),
            );
        }
    };

    let profile = match root
        .get(DEFAULT_WINDOW_SETTINGS_KEY)
        .and_then(plist::Value::as_string)
    {
        Some(profile) if !profile.is_empty() => profile.to_string(),
        _ => {
            return config_error(
                "missing Default Window Settings".to_string(),
                None,
                Some(config_path),
            );
        }
    };

    let font = match resolve_font(root, &profile) {
        Ok(font) => font,
        Err(reason) => return config_error(reason, Some(profile), Some(config_path)),
    };

    DetectionResult {
        detected: Some(is_nerd_font(&font)),
        source: DetectionSource::TerminalConfig,
        terminal: Some(Terminal::TerminalApp),
        font: Some(font),
        config_path: Some(config_path),
        profile: Some(profile),
        error_reason: None,
        confidence: Confidence::Certain,
    }
}

#[cfg(not(target_os = "macos"))]
fn resolve_from_plist(_vars: &[(String, String)], config_path: PathBuf) -> DetectionResult {
    config_error(
        "Terminal.app resolver is only supported on macOS".to_string(),
        None,
        Some(config_path),
    )
}

#[cfg(target_os = "macos")]
fn resolve_font(root: &plist::Dictionary, profile: &str) -> Result<String, String> {
    let settings = root
        .get(WINDOW_SETTINGS_KEY)
        .and_then(plist::Value::as_dictionary)
        .ok_or_else(|| "missing Window Settings dictionary".to_string())?;

    let profile_settings = settings
        .get(profile)
        .and_then(plist::Value::as_dictionary)
        .ok_or_else(|| format!("missing profile settings for {profile}"))?;

    let font = profile_settings
        .get("Font")
        .and_then(font_value_to_string)
        .or_else(|| {
            profile_settings
                .get("Normal Font")
                .and_then(font_value_to_string)
        })
        .ok_or_else(|| format!("missing font descriptor for profile {profile}"))?;

    if font.is_empty() {
        return Err(format!("empty font descriptor for profile {profile}"));
    }

    Ok(font)
}

#[cfg(target_os = "macos")]
fn font_value_to_string(value: &plist::Value) -> Option<String> {
    if let Some(font) = value.as_string() {
        return Some(font.to_string());
    }

    let descriptor = value.as_dictionary()?;
    descriptor
        .get("FontName")
        .and_then(plist::Value::as_string)
        .map(ToString::to_string)
}

fn var<'a>(vars: &'a [(String, String)], key: &str) -> Option<&'a str> {
    vars.iter()
        .find_map(|(k, v)| (k == key).then_some(v.as_str()))
}

fn is_nerd_font(font: &str) -> bool {
    let normalized = font.trim().to_ascii_lowercase();
    if normalized.contains("nerd font") || normalized.contains("nerdfont") {
        return true;
    }

    normalized
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|token| matches!(token, "nf" | "nfm" | "nfp"))
}

fn config_error(
    reason: String,
    profile: Option<String>,
    config_path: Option<PathBuf>,
) -> DetectionResult {
    DetectionResult {
        detected: None,
        source: DetectionSource::ConfigError,
        terminal: Some(Terminal::TerminalApp),
        font: None,
        config_path,
        profile,
        error_reason: Some(reason),
        confidence: Confidence::Certain,
    }
}
