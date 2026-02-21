use std::path::Path;

use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

mod terminal_app;

pub fn resolve(terminal: Terminal, vars: &[(String, String)], _cwd: &Path) -> DetectionResult {
    match terminal {
        Terminal::TerminalApp => terminal_app::resolve(vars),
        Terminal::Kitty
        | Terminal::Alacritty
        | Terminal::ITerm2
        | Terminal::Vscode
        | Terminal::Zed
        | Terminal::Hyper => no_resolver(terminal),
        Terminal::Ghostty | Terminal::WezTerm => no_resolver(terminal),
        Terminal::Unknown(name) => {
            config_error(format!("no resolver for unknown terminal: {name}"))
        }
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

fn config_error(reason: String) -> DetectionResult {
    DetectionResult {
        detected: None,
        source: DetectionSource::ConfigError,
        terminal: None,
        font: None,
        config_path: None,
        profile: None,
        error_reason: Some(reason),
        confidence: Confidence::Certain,
    }
}
