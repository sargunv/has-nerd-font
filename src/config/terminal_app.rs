use std::path::PathBuf;

use crate::{
    Confidence, DetectionResult, DetectionSource, Terminal,
    font::is_nerd_font,
    plist::{font_name_from_keyed_archive, load_root_dictionary},
};

pub fn resolve(vars: &[(String, String)]) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error("HOME is not set".to_string(), None, None),
    };

    let config_path = PathBuf::from(home).join("Library/Preferences/com.apple.Terminal.plist");

    resolve_from_plist(config_path)
}

#[cfg(target_os = "macos")]
fn resolve_from_plist(config_path: PathBuf) -> DetectionResult {
    let resolved = (|| -> Result<(String, String), (String, Option<String>)> {
        let root = load_root_dictionary(&config_path).map_err(|reason| (reason, None))?;

        let profile = root
            .get("Default Window Settings")
            .and_then(plist::Value::as_string)
            .filter(|profile| !profile.is_empty())
            .map(ToString::to_string)
            .ok_or_else(|| ("missing Default Window Settings".to_string(), None))?;

        let font =
            resolve_font(&root, &profile).map_err(|reason| (reason, Some(profile.clone())))?;

        Ok((profile, font))
    })();

    let (profile, font) = match resolved {
        Ok(resolved) => resolved,
        Err((reason, profile)) => return config_error(reason, profile, Some(config_path)),
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
fn resolve_from_plist(config_path: PathBuf) -> DetectionResult {
    config_error(
        "Terminal.app resolver is only supported on macOS".to_string(),
        None,
        Some(config_path),
    )
}

#[cfg(target_os = "macos")]
fn resolve_font(root: &plist::Dictionary, profile: &str) -> Result<String, String> {
    let settings = root
        .get("Window Settings")
        .and_then(plist::Value::as_dictionary)
        .ok_or_else(|| "missing Window Settings dictionary".to_string())?;

    let profile_settings = settings
        .get(profile)
        .and_then(plist::Value::as_dictionary)
        .ok_or_else(|| format!("missing profile settings for {profile}"))?;

    let font = profile_settings
        .get("Font")
        .and_then(font_name_from_keyed_archive)
        .or_else(|| {
            profile_settings
                .get("Normal Font")
                .and_then(font_name_from_keyed_archive)
        })
        .ok_or_else(|| format!("missing font descriptor for profile {profile}"))?;

    if font.is_empty() {
        return Err(format!("empty font descriptor for profile {profile}"));
    }

    Ok(font)
}

fn var<'a>(vars: &'a [(String, String)], key: &str) -> Option<&'a str> {
    vars.iter()
        .find_map(|(k, v)| (k == key).then_some(v.as_str()))
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
