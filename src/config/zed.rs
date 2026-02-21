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
    let project_result = find_project_settings::<ZedSettings>(cwd, home_path, ".zed");

    let user_settings = read_json5_settings::<ZedSettings>(&user_path);

    // If either file had a parse error, report it
    if let Err((reason, path)) = &project_result {
        return config_error(Terminal::Zed, reason.clone(), Some(path.clone()));
    }
    if let Err(reason) = &user_settings {
        return config_error(Terminal::Zed, reason.clone(), Some(user_path));
    }

    let (project_settings, project_path) = project_result.unwrap();
    let user_settings = user_settings.unwrap();

    // If neither file exists, report no settings file found
    if project_settings.is_none() && user_settings.is_none() {
        return config_error(Terminal::Zed, "no settings file found".to_string(), None);
    }

    let font = effective_font(&project_settings, &user_settings);

    match font {
        Some((font_name, from_project)) => {
            let font = normalize_font_name(&font_name);
            let config_path = if from_project {
                project_path
            } else {
                Some(user_path)
            };
            DetectionResult {
                detected: Some(is_nerd_font(&font)),
                source: DetectionSource::TerminalConfig,
                terminal: Some(Terminal::Zed),
                font: Some(font),
                config_path,
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
fn effective_font(
    project: &Option<ZedSettings>,
    user: &Option<ZedSettings>,
) -> Option<(String, bool)> {
    // Terminal font: project overrides user
    let terminal_font = project
        .as_ref()
        .and_then(|s| s.terminal.as_ref())
        .and_then(|t| t.font_family.clone());

    let terminal_font_from_user = user
        .as_ref()
        .and_then(|s| s.terminal.as_ref())
        .and_then(|t| t.font_family.clone());

    // Buffer font: project overrides user
    let buffer_font = project.as_ref().and_then(|s| s.buffer_font_family.clone());

    let buffer_font_from_user = user.as_ref().and_then(|s| s.buffer_font_family.clone());

    // Prefer terminal over buffer; within each, prefer project over user
    None.or(terminal_font.map(|f| (f, true)))
        .or(terminal_font_from_user.map(|f| (f, false)))
        .or(buffer_font.map(|f| (f, true)))
        .or(buffer_font_from_user.map(|f| (f, false)))
}
