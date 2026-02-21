use std::path::Path;

mod config;
mod env;
mod terminal;
mod types;

pub use types::{Confidence, DetectionResult, DetectionSource, Terminal};

enum LayerOutcome<T> {
    Final(DetectionResult),
    Continue(T),
}

pub fn detect(vars: &[(String, String)], cwd: &Path) -> DetectionResult {
    if let LayerOutcome::Final(result) = env_layer(vars) {
        return result;
    }

    let terminal = match terminal_layer(vars) {
        LayerOutcome::Final(result) => return result,
        LayerOutcome::Continue(terminal) => terminal,
    };

    match ssh_gate_layer(vars, terminal) {
        LayerOutcome::Final(result) => result,
        LayerOutcome::Continue(terminal) => config::resolve(terminal, cwd),
    }
}

fn env_layer(vars: &[(String, String)]) -> LayerOutcome<()> {
    match env::detect(vars) {
        Some(result) => LayerOutcome::Final(result),
        None => LayerOutcome::Continue(()),
    }
}

fn terminal_layer(vars: &[(String, String)]) -> LayerOutcome<Terminal> {
    match terminal::detect(vars) {
        terminal::TerminalLayerSignal::Bundled(result) => LayerOutcome::Final(result),
        terminal::TerminalLayerSignal::Identified(terminal) => LayerOutcome::Continue(terminal),
        terminal::TerminalLayerSignal::Unknown => LayerOutcome::Final(DetectionResult {
            detected: None,
            source: DetectionSource::UnknownTerminal,
            terminal: None,
            font: None,
            config_path: None,
            profile: None,
            error_reason: None,
            confidence: Confidence::Certain,
        }),
    }
}

fn ssh_gate_layer(vars: &[(String, String)], terminal: Terminal) -> LayerOutcome<Terminal> {
    if is_remote_session(vars) {
        return LayerOutcome::Final(DetectionResult {
            detected: None,
            source: DetectionSource::RemoteSession,
            terminal: Some(terminal),
            font: None,
            config_path: None,
            profile: None,
            error_reason: None,
            confidence: Confidence::Certain,
        });
    }

    LayerOutcome::Continue(terminal)
}

fn is_remote_session(vars: &[(String, String)]) -> bool {
    vars.iter().any(|(key, value)| {
        matches!(key.as_str(), "SSH_TTY" | "SSH_CONNECTION") && !value.is_empty()
    })
}
