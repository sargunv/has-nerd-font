use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use crate::{Confidence, DetectionResult, DetectionSource, Terminal, var};

mod alacritty;
mod iterm2;
mod terminal_app;
mod vscode;
mod zed;

pub fn resolve(terminal: Terminal, vars: &[(String, String)]) -> DetectionResult {
    match terminal {
        Terminal::Alacritty => alacritty::resolve(vars),
        Terminal::ITerm2 => iterm2::resolve(vars),
        Terminal::TerminalApp => terminal_app::resolve(vars),
        Terminal::Vscode => vscode::resolve(vars),
        Terminal::Zed => zed::resolve(vars),
        _ => no_resolver(terminal),
    }
}

fn no_resolver(terminal: Terminal) -> DetectionResult {
    DetectionResult {
        detected: None,
        source: DetectionSource::NoResolver,
        terminal: Some(terminal),
        font: None,
        config_path: None,
        profile: None,
        error_reason: None,
        confidence: Confidence::Certain,
    }
}

// --- Shared helpers ---

pub(crate) fn config_error(
    terminal: Terminal,
    reason: String,
    config_path: Option<PathBuf>,
) -> DetectionResult {
    DetectionResult {
        detected: None,
        source: DetectionSource::ConfigError,
        terminal: Some(terminal),
        font: None,
        config_path,
        profile: None,
        error_reason: Some(reason),
        confidence: Confidence::Certain,
    }
}

/// Read a settings file from disk. Returns:
/// - `Ok(Some(content))` if the file exists and was read successfully
/// - `Ok(None)` if the file does not exist or is inaccessible
/// - `Err(reason)` if the file exists but could not be read
fn read_settings_file(path: &Path) -> Result<Option<String>, String> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(Some(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(None),
        Err(e) => Err(format!("failed to read {}: {e}", path.display())),
    }
}

/// Read and parse a JSON5 settings file. Returns:
/// - `Ok(Some(settings))` if the file exists and was parsed successfully
/// - `Ok(None)` if the file does not exist or is inaccessible
/// - `Err(reason)` if the file exists but is malformed
pub(crate) fn read_json5_settings<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, String> {
    let content = match read_settings_file(path)? {
        Some(content) => content,
        None => return Ok(None),
    };

    match serde_json5::from_str::<T>(&content) {
        Ok(settings) => Ok(Some(settings)),
        Err(e) => {
            let location = match &e {
                serde_json5::Error::Message { location, .. } => location.as_ref(),
            };
            match location {
                Some(loc) => Err(format!(
                    "failed to parse {} at line {} column {}",
                    path.display(),
                    loc.line,
                    loc.column
                )),
                None => Err(format!("failed to parse {}: {e}", path.display())),
            }
        }
    }
}

/// Read and parse a TOML settings file. Returns:
/// - `Ok(Some(settings))` if the file exists and was parsed successfully
/// - `Ok(None)` if the file does not exist or is inaccessible
/// - `Err(reason)` if the file exists but is malformed
pub(crate) fn read_toml_settings<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, String> {
    let content = match read_settings_file(path)? {
        Some(content) => content,
        None => return Ok(None),
    };

    match toml::from_str::<T>(&content) {
        Ok(settings) => Ok(Some(settings)),
        Err(e) => Err(format!("failed to parse {}: {e}", path.display())),
    }
}
