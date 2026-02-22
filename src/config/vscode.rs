use std::path::Path;

use serde::Deserialize;

use super::{config_error, read_json5_settings, var};
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
        home.join(format!(".config/{app_dir}/User/settings.json"))
    }
}

pub fn resolve(vars: &[(String, String)]) -> DetectionResult {
    let home = match var(vars, "HOME") {
        Some(value) if !value.is_empty() => value,
        _ => return config_error(Terminal::Vscode, "HOME is not set".to_string(), None),
    };

    let app_dir = match resolve_app_dir(vars) {
        Ok(dir) => dir,
        Err(reason) => return config_error(Terminal::Vscode, reason, None),
    };

    let config_path = user_settings_path(Path::new(home), app_dir);

    let settings = match read_json5_settings::<VscodeSettings>(&config_path) {
        Ok(Some(s)) => s,
        Ok(None) => {
            return config_error(Terminal::Vscode, "no settings file found".to_string(), None);
        }
        Err(reason) => return config_error(Terminal::Vscode, reason, Some(config_path)),
    };

    let font_name = settings
        .terminal_font_family
        .as_deref()
        .or(settings.editor_font_family.as_deref());

    match font_name {
        Some(name) => {
            let font = normalize_font_name(name);
            DetectionResult {
                detected: Some(is_nerd_font(&font)),
                source: DetectionSource::TerminalConfig,
                terminal: Some(Terminal::Vscode),
                font: Some(font),
                config_path: Some(config_path),
                profile: None,
                error_reason: None,
                confidence: Confidence::Certain,
            }
        }
        None => config_error(
            Terminal::Vscode,
            "no font configured".to_string(),
            Some(config_path),
        ),
    }
}
