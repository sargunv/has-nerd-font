use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

mod hyper;
mod iterm2;
mod terminal_app;
mod vscode;
mod zed;

pub fn resolve(terminal: Terminal, vars: &[(String, String)], cwd: &Path) -> DetectionResult {
    match terminal {
        Terminal::Hyper => hyper::resolve(vars),
        Terminal::ITerm2 => iterm2::resolve(vars),
        Terminal::TerminalApp => terminal_app::resolve(vars),
        Terminal::Vscode => vscode::resolve(vars, cwd),
        Terminal::Zed => zed::resolve(vars, cwd),
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

pub(crate) fn var<'a>(vars: &'a [(String, String)], key: &str) -> Option<&'a str> {
    vars.iter()
        .find_map(|(k, v)| (k == key).then_some(v.as_str()))
}

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

/// Read and parse a JSON5 settings file. Returns:
/// - `Ok(Some(settings))` if the file exists and was parsed successfully
/// - `Ok(None)` if the file does not exist or is inaccessible
/// - `Err(reason)` if the file exists but is malformed
pub(crate) fn read_json5_settings<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => return Ok(None),
        Err(e) => return Err(format!("failed to read {}: {e}", path.display())),
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

/// Walk up from `cwd` to `home` (inclusive) looking for `{subdir}/settings.json`.
/// Returns:
/// - `Ok((Some(settings), Some(path)))` if found and parsed
/// - `Ok((None, None))` if not found in any ancestor
/// - `Err((reason, path))` if found but malformed
pub(crate) fn find_project_settings<T: DeserializeOwned>(
    cwd: &Path,
    home: &Path,
    subdir: &str,
) -> Result<(Option<T>, Option<PathBuf>), (String, PathBuf)> {
    let mut dir = Some(cwd);
    while let Some(current) = dir {
        let candidate = current.join(subdir).join("settings.json");
        match read_json5_settings::<T>(&candidate) {
            Ok(Some(settings)) => return Ok((Some(settings), Some(candidate))),
            Ok(None) => {} // not found here, keep walking
            Err(reason) => return Err((reason, candidate)),
        }
        if current == home {
            break;
        }
        dir = current.parent();
    }
    Ok((None, None))
}
