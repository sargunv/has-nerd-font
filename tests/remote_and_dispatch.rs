use std::path::Path;

use has_nerd_font::{DetectionSource, Terminal, detect};

fn vars(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

#[test]
fn known_non_bundled_terminal_with_ssh_is_remote_session() {
    let env = vars(&[
        ("TERM_PROGRAM", "Apple_Terminal"),
        ("SSH_TTY", "/dev/pts/1"),
    ]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::RemoteSession);
    assert_eq!(result.exit_code(), 3);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
}

#[test]
fn known_non_bundled_local_terminal_without_resolver_returns_no_resolver() {
    let env = vars(&[("KITTY_PID", "42")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::NoResolver);
    assert_eq!(result.exit_code(), 4);
    assert_eq!(result.terminal, Some(Terminal::Kitty));
}
