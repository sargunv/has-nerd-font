use std::path::Path;

use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

mod iterm2;
mod terminal_app;
mod zed;

pub fn resolve(terminal: Terminal, vars: &[(String, String)], cwd: &Path) -> DetectionResult {
    match terminal {
        Terminal::ITerm2 => iterm2::resolve(vars),
        Terminal::TerminalApp => terminal_app::resolve(vars),
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
