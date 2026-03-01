#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod config;
mod env;
mod font;
#[cfg(target_os = "macos")]
mod plist;
mod terminal;
mod types;

pub use types::{Confidence, DetectionResult, DetectionSource, Terminal};

/// Look up the last occurrence of `key` in the env var list (last wins).
fn var<'a>(vars: &'a [(String, String)], key: &str) -> Option<&'a str> {
    vars.iter()
        .rev()
        .find_map(|(k, v)| (k == key).then_some(v.as_str()))
}

enum LayerOutcome<T> {
    Final(DetectionResult),
    Continue(T),
}

/// Detect whether the current terminal session can render Nerd Font glyphs.
///
/// Pass the current environment variables as a slice of `(key, value)` pairs.
/// The function inspects environment variables and terminal config files to
/// determine font status.
///
/// # Example
///
/// ```
/// let env_vars: Vec<(String, String)> = std::env::vars().collect();
/// let result = has_nerd_font::detect(&env_vars);
///
/// match result.detected {
///     Some(true) => println!("Nerd Font available"),
///     Some(false) => println!("No Nerd Font detected"),
///     None => println!("Could not determine font status"),
/// }
/// ```
pub fn detect(vars: &[(String, String)]) -> DetectionResult {
    if let LayerOutcome::Final(result) = env_layer(vars) {
        return result;
    }

    let terminal = match terminal_layer(vars) {
        LayerOutcome::Final(result) => return result,
        LayerOutcome::Continue(terminal) => terminal,
    };

    match ssh_gate_layer(vars, terminal) {
        LayerOutcome::Final(result) => result,
        LayerOutcome::Continue(terminal) => config::resolve(terminal, vars),
    }
}

fn env_layer(vars: &[(String, String)]) -> LayerOutcome<()> {
    match env::detect(vars) {
        env::EnvDecision::OverrideEnabled => {
            LayerOutcome::Final(base_result(Some(true), DetectionSource::EnvVar, None))
        }
        env::EnvDecision::OverrideDisabled => LayerOutcome::Final(base_result(
            Some(false),
            DetectionSource::ExplicitDisable,
            None,
        )),
        env::EnvDecision::Continue => LayerOutcome::Continue(()),
    }
}

fn terminal_layer(vars: &[(String, String)]) -> LayerOutcome<Terminal> {
    match terminal::detect(vars) {
        terminal::TerminalDecision::Bundled(terminal) => LayerOutcome::Final(base_result(
            Some(true),
            DetectionSource::BundledTerminal,
            Some(terminal),
        )),
        terminal::TerminalDecision::Identified(terminal) => LayerOutcome::Continue(terminal),
        terminal::TerminalDecision::Unknown => {
            LayerOutcome::Final(base_result(None, DetectionSource::UnknownTerminal, None))
        }
    }
}

fn ssh_gate_layer(vars: &[(String, String)], terminal: Terminal) -> LayerOutcome<Terminal> {
    if is_remote_session(vars) {
        return LayerOutcome::Final(base_result(
            None,
            DetectionSource::RemoteSession,
            Some(terminal),
        ));
    }

    LayerOutcome::Continue(terminal)
}

fn base_result(
    detected: Option<bool>,
    source: DetectionSource,
    terminal: Option<Terminal>,
) -> DetectionResult {
    DetectionResult {
        detected,
        source,
        terminal,
        font: None,
        config_path: None,
        profile: None,
        error_reason: None,
        confidence: Confidence::Certain,
    }
}

fn is_remote_session(vars: &[(String, String)]) -> bool {
    vars.iter().any(|(key, value)| {
        matches!(key.as_str(), "SSH_TTY" | "SSH_CONNECTION") && !value.is_empty()
    })
}
