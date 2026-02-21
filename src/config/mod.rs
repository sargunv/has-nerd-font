use std::path::Path;

use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

pub fn resolve(terminal: Terminal, cwd: &Path) -> DetectionResult {
    match terminal {
        Terminal::TerminalApp => resolve_terminal_app(cwd),
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

fn resolve_terminal_app(_cwd: &Path) -> DetectionResult {
    DetectionResult {
        detected: None,
        source: DetectionSource::ConfigError,
        terminal: Some(Terminal::TerminalApp),
        font: None,
        config_path: None,
        profile: None,
        error_reason: Some("terminal_app resolver not implemented yet".to_string()),
        confidence: Confidence::Certain,
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
