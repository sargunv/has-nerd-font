use std::path::PathBuf;

use crate::{Confidence, DetectionResult, DetectionSource, Terminal};
#[cfg(target_os = "macos")]
use crate::{
    font::{is_nerd_font, normalize_font_name},
    plist::load_root_dictionary,
};

pub fn resolve(vars: &[(String, String)]) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error("HOME is not set".to_string(), None, None),
    };

    let iterm_profile = var(vars, "ITERM_PROFILE").map(ToString::to_string);

    let config_path = PathBuf::from(home).join("Library/Preferences/com.googlecode.iterm2.plist");
    resolve_from_plist(config_path, iterm_profile)
}

#[cfg(target_os = "macos")]
fn resolve_from_plist(config_path: PathBuf, iterm_profile: Option<String>) -> DetectionResult {
    let resolved = (|| -> Result<(Option<String>, String), (String, Option<String>)> {
        let root = load_root_dictionary(&config_path).map_err(|reason| (reason, None))?;

        let bookmarks = root
            .get("New Bookmarks")
            .and_then(plist::Value::as_array)
            .ok_or_else(|| ("missing New Bookmarks array".to_string(), None))?;

        // If ITERM_PROFILE is set, find the bookmark by profile name.
        // Otherwise, fall back to the default bookmark GUID.
        let bookmark = if let Some(ref profile_name) = iterm_profile {
            bookmarks
                .iter()
                .filter_map(plist::Value::as_dictionary)
                .find(|bookmark| {
                    bookmark
                        .get("Name")
                        .and_then(plist::Value::as_string)
                        .is_some_and(|name| name == profile_name)
                })
                .ok_or_else(|| {
                    (
                        format!("missing bookmark for profile {profile_name}"),
                        iterm_profile.clone(),
                    )
                })?
        } else {
            let default_guid = root
                .get("Default Bookmark Guid")
                .and_then(plist::Value::as_string)
                .filter(|guid| !guid.is_empty())
                .ok_or_else(|| ("missing Default Bookmark Guid".to_string(), None))?;

            bookmarks
                .iter()
                .filter_map(plist::Value::as_dictionary)
                .find(|bookmark| {
                    bookmark
                        .get("Guid")
                        .and_then(plist::Value::as_string)
                        .is_some_and(|guid| guid == default_guid)
                })
                .ok_or_else(|| (format!("missing bookmark for guid {default_guid}"), None))?
        };

        let profile = bookmark
            .get("Name")
            .and_then(plist::Value::as_string)
            .filter(|name| !name.is_empty())
            .map(ToString::to_string);

        let font = bookmark
            .get("Normal Font")
            .and_then(plist::Value::as_string)
            .map(normalize_font_name)
            .ok_or_else(|| {
                (
                    format!(
                        "missing Normal Font for {}",
                        profile.as_deref().unwrap_or("bookmark")
                    ),
                    profile.clone(),
                )
            })?;

        if font.is_empty() {
            return Err((
                format!(
                    "empty Normal Font for {}",
                    profile.as_deref().unwrap_or("bookmark")
                ),
                profile,
            ));
        }

        Ok((profile, font))
    })();

    let (profile, font) = match resolved {
        Ok(resolved) => resolved,
        Err((reason, profile)) => return config_error(reason, profile, Some(config_path)),
    };

    DetectionResult {
        detected: Some(is_nerd_font(&font)),
        source: DetectionSource::TerminalConfig,
        terminal: Some(Terminal::ITerm2),
        font: Some(font),
        config_path: Some(config_path),
        profile,
        error_reason: None,
        confidence: Confidence::Certain,
    }
}

#[cfg(not(target_os = "macos"))]
fn resolve_from_plist(config_path: PathBuf, _iterm_profile: Option<String>) -> DetectionResult {
    config_error(
        "iTerm2 resolver is only supported on macOS".to_string(),
        None,
        Some(config_path),
    )
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
        terminal: Some(Terminal::ITerm2),
        font: None,
        config_path,
        profile,
        error_reason: Some(reason),
        confidence: Confidence::Certain,
    }
}
