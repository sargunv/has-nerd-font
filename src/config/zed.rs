use std::path::Path;

use serde::Deserialize;

use super::{config_error, find_project_settings, read_json5_settings, var};
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

pub fn resolve(vars: &[(String, String)], cwd: &Path) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error(Terminal::Zed, "HOME is not set".to_string(), None),
    };

    let home_path = Path::new(home);
    let user_path = home_path.join(".config/zed/settings.json");

    // Walk up from cwd to $HOME looking for .zed/settings.json
    let (project_settings, project_path) =
        match find_project_settings::<ZedSettings>(cwd, home_path, ".zed") {
            Ok(result) => result,
            Err((reason, path)) => return config_error(Terminal::Zed, reason, Some(path)),
        };

    let user_settings = match read_json5_settings::<ZedSettings>(&user_path) {
        Ok(result) => result,
        Err(reason) => return config_error(Terminal::Zed, reason, Some(user_path)),
    };

    if project_settings.is_none() && user_settings.is_none() {
        return config_error(Terminal::Zed, "no settings file found".to_string(), None);
    }

    match effective_font(&project_settings, &user_settings) {
        Some((font_name, from_project)) => {
            let font = normalize_font_name(font_name);
            DetectionResult {
                detected: Some(is_nerd_font(&font)),
                source: DetectionSource::TerminalConfig,
                terminal: Some(Terminal::Zed),
                font: Some(font),
                config_path: if from_project {
                    project_path
                } else {
                    Some(user_path)
                },
                profile: None,
                error_reason: None,
                confidence: Confidence::Certain,
            }
        }
        None => config_error(
            Terminal::Zed,
            "no font configured".to_string(),
            if project_settings.is_some() {
                project_path
            } else {
                Some(user_path)
            },
        ),
    }
}

/// Resolve the effective font from project and user settings.
/// Project settings override user settings; terminal font is preferred over buffer font.
/// Returns the font name and whether it came from the project settings (true) or user settings (false).
fn effective_font<'a>(
    project: &'a Option<ZedSettings>,
    user: &'a Option<ZedSettings>,
) -> Option<(&'a str, bool)> {
    None.or(project
        .as_ref()
        .and_then(|s| s.terminal.as_ref())
        .and_then(|t| t.font_family.as_deref())
        .map(|f| (f, true)))
        .or(user
            .as_ref()
            .and_then(|s| s.terminal.as_ref())
            .and_then(|t| t.font_family.as_deref())
            .map(|f| (f, false)))
        .or(project
            .as_ref()
            .and_then(|s| s.buffer_font_family.as_deref())
            .map(|f| (f, true)))
        .or(user
            .as_ref()
            .and_then(|s| s.buffer_font_family.as_deref())
            .map(|f| (f, false)))
}
