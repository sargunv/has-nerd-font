use std::path::{Path, PathBuf};

use serde::Deserialize;

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
        _ => return config_error("HOME is not set".to_string(), None),
    };

    let project_path = cwd.join(".zed/settings.json");
    let user_path = PathBuf::from(home).join(".config/zed/settings.json");

    let project_settings = read_settings(&project_path);
    let user_settings = read_settings(&user_path);

    // If either file had a parse error, report it
    if let Err(reason) = &project_settings {
        return config_error(reason.clone(), Some(project_path));
    }
    if let Err(reason) = &user_settings {
        return config_error(reason.clone(), Some(user_path));
    }

    let project_settings = project_settings.unwrap();
    let user_settings = user_settings.unwrap();

    // If neither file exists, report no settings file found
    if project_settings.is_none() && user_settings.is_none() {
        return config_error("no settings file found".to_string(), None);
    }

    let font = effective_font(&project_settings, &user_settings);

    match font {
        Some((font_name, from_project)) => {
            let font = normalize_font_name(&font_name);
            let config_path = if from_project {
                project_path
            } else {
                user_path
            };
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
            "no font configured".to_string(),
            if project_settings.is_some() {
                Some(project_path)
            } else {
                Some(user_path)
            },
        ),
    }
}

/// Read and parse a settings file. Returns:
/// - `Ok(Some(settings))` if the file exists and was parsed successfully
/// - `Ok(None)` if the file does not exist
/// - `Err(reason)` if the file exists but is malformed
fn read_settings(path: &Path) -> Result<Option<ZedSettings>, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("failed to read {}: {e}", path.display())),
    };

    match serde_jsonc::from_str::<ZedSettings>(&content) {
        Ok(settings) => Ok(Some(settings)),
        Err(e) => Err(format!("failed to parse {}: {e}", path.display())),
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

fn var<'a>(vars: &'a [(String, String)], key: &str) -> Option<&'a str> {
    vars.iter()
        .find_map(|(k, v)| (k == key).then_some(v.as_str()))
}

fn config_error(reason: String, config_path: Option<PathBuf>) -> DetectionResult {
    DetectionResult {
        detected: None,
        source: DetectionSource::ConfigError,
        terminal: Some(Terminal::Zed),
        font: None,
        config_path,
        profile: None,
        error_reason: Some(reason),
        confidence: Confidence::Certain,
    }
}
