use std::path::Path;

use has_nerd_font::{DetectionSource, Terminal, detect};

fn vars(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

#[test]
fn env_override_finalizes_before_later_layers() {
    let env = vars(&[
        ("NERD_FONT", "1"),
        ("TERM_PROGRAM", "Apple_Terminal"),
        ("SSH_TTY", "/dev/pts/1"),
    ]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::EnvVar);
    assert_eq!(result.detected, Some(true));
}

#[test]
fn env_override_truthy_tokens_are_trimmed_and_case_insensitive() {
    let env = vars(&[("NERD_FONT", "  YeS  "), ("TERM_PROGRAM", "Apple_Terminal")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::EnvVar);
    assert_eq!(result.detected, Some(true));
}

#[test]
fn env_override_falsy_tokens_short_circuit_before_terminal_layer() {
    let env = vars(&[("NERD_FONT", " FALSE "), ("TERM_PROGRAM", "ghostty")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::ExplicitDisable);
    assert_eq!(result.detected, Some(false));
}

#[test]
fn bundled_terminal_finalizes_before_ssh_gate() {
    let env = vars(&[("TERM_PROGRAM", "ghostty"), ("SSH_TTY", "/dev/pts/1")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::BundledTerminal);
    assert_eq!(result.detected, Some(true));
    assert_eq!(result.terminal, Some(Terminal::Ghostty));
}

#[test]
fn ssh_gate_finalizes_before_config_dispatch() {
    let env = vars(&[
        ("TERM_PROGRAM", "Apple_Terminal"),
        ("SSH_TTY", "/dev/pts/1"),
    ]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::RemoteSession);
    assert_eq!(result.detected, None);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
}
