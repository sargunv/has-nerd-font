use std::path::Path;

use serde::Deserialize;

use super::{config_error, find_project_settings, read_json5_settings, var};
use crate::font::{is_nerd_font, normalize_font_name};
use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

#[derive(Deserialize)]
struct VscodeSettings {
    #[serde(default, rename = "terminal.integrated.fontFamily")]
    terminal_font_family: Option<String>,
    #[serde(default, rename = "editor.fontFamily")]
    editor_font_family: Option<String>,
}

/// Known VS Code forks and their identifying information.
/// To add a new fork (e.g. Cursor), add an entry here.
struct VscodeFork {
    /// Directory name used for user settings (e.g. "Code", "VSCodium").
    app_dir: &'static str,
    /// Case-insensitive substrings to match in `VSCODE_GIT_ASKPASS_NODE`.
    askpass_substrings: &'static [&'static str],
}

const KNOWN_FORKS: &[VscodeFork] = &[
    VscodeFork {
        app_dir: "Code",
        askpass_substrings: &["code"],
    },
    VscodeFork {
        app_dir: "VSCodium",
        askpass_substrings: &["codium"],
    },
];

/// Resolves the app-specific directory name for user settings from env vars.
///
/// Checks `VSCODE_GIT_ASKPASS_NODE` for known fork substrings. Returns an
/// error if the env var is missing or doesn't match any known fork.
fn resolve_app_dir(vars: &[(String, String)]) -> Result<&'static str, String> {
    let askpass = var(vars, "VSCODE_GIT_ASKPASS_NODE")
        .filter(|v| !v.is_empty())
        .ok_or_else(|| "VSCODE_GIT_ASKPASS_NODE is not set".to_string())?;

    let askpass_lower = askpass.to_ascii_lowercase();
    KNOWN_FORKS
        .iter()
        .find(|f| {
            f.askpass_substrings
                .iter()
                .any(|s| askpass_lower.contains(s))
        })
        .map(|f| f.app_dir)
        .ok_or_else(|| format!("unrecognized VSCODE_GIT_ASKPASS_NODE: {askpass}"))
}

/// Returns the platform-specific path to the user settings.json for the given app directory.
fn user_settings_path(home: &Path, app_dir: &str) -> std::path::PathBuf {
    if cfg!(target_os = "macos") {
        home.join(format!(
            "Library/Application Support/{app_dir}/User/settings.json"
        ))
    } else {
        // Linux (and future: Windows would use %APPDATA%/{app_dir}/User/settings.json)
        home.join(format!(".config/{app_dir}/User/settings.json"))
    }
}

pub fn resolve(vars: &[(String, String)], cwd: &Path) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error(Terminal::Vscode, "HOME is not set".to_string(), None),
    };

    let app_dir = match resolve_app_dir(vars) {
        Ok(dir) => dir,
        Err(reason) => return config_error(Terminal::Vscode, reason, None),
    };

    let home_path = Path::new(home);
    let user_path = user_settings_path(home_path, app_dir);

    // Walk up from cwd to $HOME looking for .vscode/settings.json
    let (project_settings, project_path) =
        match find_project_settings::<VscodeSettings>(cwd, home_path, ".vscode") {
            Ok(result) => result,
            Err((reason, path)) => return config_error(Terminal::Vscode, reason, Some(path)),
        };

    let user_settings = match read_json5_settings::<VscodeSettings>(&user_path) {
        Ok(result) => result,
        Err(reason) => return config_error(Terminal::Vscode, reason, Some(user_path)),
    };

    if project_settings.is_none() && user_settings.is_none() {
        return config_error(Terminal::Vscode, "no settings file found".to_string(), None);
    }

    match effective_font(&project_settings, &user_settings) {
        Some((font_name, from_project)) => {
            let font = normalize_font_name(font_name);
            DetectionResult {
                detected: Some(is_nerd_font(&font)),
                source: DetectionSource::TerminalConfig,
                terminal: Some(Terminal::Vscode),
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
            Terminal::Vscode,
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
/// Project settings override user settings; terminal font is preferred over editor font.
/// Returns the font name and whether it came from the project settings (true) or user settings (false).
fn effective_font<'a>(
    project: &'a Option<VscodeSettings>,
    user: &'a Option<VscodeSettings>,
) -> Option<(&'a str, bool)> {
    None.or(project
        .as_ref()
        .and_then(|s| s.terminal_font_family.as_deref())
        .map(|f| (f, true)))
        .or(user
            .as_ref()
            .and_then(|s| s.terminal_font_family.as_deref())
            .map(|f| (f, false)))
        .or(project
            .as_ref()
            .and_then(|s| s.editor_font_family.as_deref())
            .map(|f| (f, true)))
        .or(user
            .as_ref()
            .and_then(|s| s.editor_font_family.as_deref())
            .map(|f| (f, false)))
}
