use std::path::Path;

use has_nerd_font::{DetectionSource, Terminal, detect};

fn vars(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

#[test]
fn term_program_ghostty_short_circuits_as_bundled() {
    let env = vars(&[("TERM_PROGRAM", "ghostty")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::BundledTerminal);
    assert_eq!(result.detected, Some(true));
    assert_eq!(result.terminal, Some(Terminal::Ghostty));
}

#[test]
fn term_xterm_ghostty_short_circuits_as_bundled() {
    let env = vars(&[("TERM", "xterm-ghostty")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::BundledTerminal);
    assert_eq!(result.detected, Some(true));
    assert_eq!(result.terminal, Some(Terminal::Ghostty));
}

#[test]
fn term_program_wezterm_short_circuits_as_bundled() {
    let env = vars(&[("TERM_PROGRAM", "WezTerm")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::BundledTerminal);
    assert_eq!(result.detected, Some(true));
    assert_eq!(result.terminal, Some(Terminal::WezTerm));
}

#[test]
fn wezterm_pane_without_term_signals_is_unknown_terminal() {
    let env = vars(&[("WEZTERM_PANE", "1")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::UnknownTerminal);
    assert_eq!(result.detected, None);
    assert_eq!(result.exit_code(), 2);
}

#[test]
fn unknown_signals_returns_unknown_terminal() {
    let env = vars(&[("TERM", "xterm-256color")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::UnknownTerminal);
    assert_eq!(result.detected, None);
    assert_eq!(result.exit_code(), 2);
}

#[test]
fn unmapped_term_program_routes_to_no_resolver_with_unknown_terminal_variant() {
    let env = vars(&[("TERM_PROGRAM", "CoolNewTerm")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::NoResolver);
    assert_eq!(result.detected, None);
    assert_eq!(result.exit_code(), 4);
    assert_eq!(
        result.terminal,
        Some(Terminal::Unknown("CoolNewTerm".to_string()))
    );
}

#[test]
fn apple_terminal_identifies_and_continues_to_no_resolver() {
    let env = vars(&[("TERM_PROGRAM", "Apple_Terminal")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::ConfigError);
    assert_eq!(result.detected, None);
    assert_eq!(result.exit_code(), 5);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
}

#[test]
fn term_program_takes_precedence_over_term() {
    let env = vars(&[
        ("TERM_PROGRAM", "Apple_Terminal"),
        ("TERM", "xterm-ghostty"),
    ]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::ConfigError);
    assert_eq!(result.detected, None);
    assert_eq!(result.exit_code(), 5);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
}
