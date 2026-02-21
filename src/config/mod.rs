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
        Terminal::Unknown(_) => no_resolver(terminal),
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
