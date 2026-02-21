use std::path::PathBuf;

use crate::{
    Confidence, DetectionResult, DetectionSource, Terminal,
    font::{is_nerd_font, normalize_font_name},
    plist::load_root_dictionary,
};

pub fn resolve(vars: &[(String, String)]) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error("HOME is not set".to_string(), None, None),
    };

    let config_path = PathBuf::from(home).join("Library/Preferences/com.googlecode.iterm2.plist");
    resolve_from_plist(config_path)
}

#[cfg(target_os = "macos")]
fn resolve_from_plist(config_path: PathBuf) -> DetectionResult {
    let resolved = (|| -> Result<(Option<String>, String), (String, Option<String>)> {
        let root = load_root_dictionary(&config_path).map_err(|reason| (reason, None))?;

        let default_guid = root
            .get("Default Bookmark Guid")
            .and_then(plist::Value::as_string)
            .filter(|guid| !guid.is_empty())
            .ok_or_else(|| ("missing Default Bookmark Guid".to_string(), None))?;

        let bookmarks = root
            .get("New Bookmarks")
            .and_then(plist::Value::as_array)
            .ok_or_else(|| ("missing New Bookmarks array".to_string(), None))?;

        let bookmark = bookmarks
            .iter()
            .filter_map(plist::Value::as_dictionary)
            .find(|bookmark| {
                bookmark
                    .get("Guid")
                    .and_then(plist::Value::as_string)
                    .is_some_and(|guid| guid == default_guid)
            })
            .ok_or_else(|| (format!("missing bookmark for guid {default_guid}"), None))?;

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
                    "missing Normal Font for default bookmark".to_string(),
                    profile.clone(),
                )
            })?;

        if font.is_empty() {
            return Err((
                "empty Normal Font for default bookmark".to_string(),
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
fn resolve_from_plist(config_path: PathBuf) -> DetectionResult {
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
