use std::fs;
use std::path::{Path, PathBuf};

use has_nerd_font::{DetectionSource, Terminal, detect};
use plist::{Dictionary, Value};

fn vars(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

fn make_home_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    path.push(format!("has-nerd-font-{name}-{nanos}"));
    path
}

fn terminal_plist_path(home: &Path) -> PathBuf {
    home.join("Library/Preferences/com.apple.Terminal.plist")
}

fn write_terminal_plist(home: &Path, profile: &str, font: &str) {
    let plist_path = terminal_plist_path(home);
    fs::create_dir_all(
        plist_path
            .parent()
            .expect("terminal plist should have parent directory"),
    )
    .expect("failed to create terminal plist directory");

    let mut profile_settings = Dictionary::new();
    profile_settings.insert("Font".to_string(), Value::String(font.to_string()));

    let mut window_settings = Dictionary::new();
    window_settings.insert(profile.to_string(), Value::Dictionary(profile_settings));

    let mut root = Dictionary::new();
    root.insert(
        "Default Window Settings".to_string(),
        Value::String(profile.to_string()),
    );
    root.insert(
        "Window Settings".to_string(),
        Value::Dictionary(window_settings),
    );

    Value::Dictionary(root)
        .to_file_xml(&plist_path)
        .expect("failed to write terminal plist");
}

#[test]
fn terminal_app_resolver_plist_with_nerd_font_detects_true_from_terminal_config() {
    let home = make_home_path("terminal-app-nf");
    write_terminal_plist(&home, "Basic", "JetBrainsMono Nerd Font");

    let home_str = home.to_string_lossy().to_string();
    let env = vars(&[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_str)]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::TerminalConfig);
    assert_eq!(result.detected, Some(true));
    assert_eq!(result.exit_code(), 0);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
    assert_eq!(result.profile.as_deref(), Some("Basic"));
    assert_eq!(result.font.as_deref(), Some("JetBrainsMono Nerd Font"));
    assert_eq!(result.config_path, Some(terminal_plist_path(&home)));
}

#[test]
fn terminal_app_resolver_plist_with_non_nerd_font_detects_false_from_terminal_config() {
    let home = make_home_path("terminal-app-non-nf");
    write_terminal_plist(&home, "Basic", "Menlo");

    let home_str = home.to_string_lossy().to_string();
    let env = vars(&[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_str)]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::TerminalConfig);
    assert_eq!(result.detected, Some(false));
    assert_eq!(result.exit_code(), 6);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
    assert_eq!(result.profile.as_deref(), Some("Basic"));
    assert_eq!(result.font.as_deref(), Some("Menlo"));
    assert_eq!(result.config_path, Some(terminal_plist_path(&home)));
}

#[test]
fn terminal_app_resolver_missing_plist_returns_config_error() {
    let home = make_home_path("terminal-app-missing");
    fs::create_dir_all(home.join("Library/Preferences"))
        .expect("failed to create terminal preferences directory");

    let home_str = home.to_string_lossy().to_string();
    let env = vars(&[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_str)]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::ConfigError);
    assert_eq!(result.detected, None);
    assert_eq!(result.exit_code(), 5);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
    assert_eq!(result.config_path, Some(terminal_plist_path(&home)));
}

#[test]
fn terminal_app_resolver_malformed_plist_returns_config_error() {
    let home = make_home_path("terminal-app-malformed");
    let plist_path = terminal_plist_path(&home);
    fs::create_dir_all(
        plist_path
            .parent()
            .expect("terminal plist should have parent directory"),
    )
    .expect("failed to create terminal plist directory");
    fs::write(&plist_path, b"not a plist").expect("failed to write malformed plist");

    let home_str = home.to_string_lossy().to_string();
    let env = vars(&[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_str)]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::ConfigError);
    assert_eq!(result.detected, None);
    assert_eq!(result.exit_code(), 5);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
    assert_eq!(result.config_path, Some(plist_path));
}

#[test]
fn terminal_app_resolver_missing_home_returns_config_error() {
    let env = vars(&[("TERM_PROGRAM", "Apple_Terminal")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::ConfigError);
    assert_eq!(result.detected, None);
    assert_eq!(result.exit_code(), 5);
    assert_eq!(result.terminal, Some(Terminal::TerminalApp));
}
