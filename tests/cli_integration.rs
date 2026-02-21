use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value as JsonValue;
use tempfile::TempDir;

#[cfg(target_os = "macos")]
use plist::{Dictionary, Value};
#[cfg(target_os = "macos")]
use std::fs;

fn run_cli(args: &[&str], env: &[(&str, &str)]) -> Output {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("has-nerd-font"));
    command.env_clear();
    command.args(args);

    for (key, value) in env {
        command.env(key, value);
    }

    command.output().expect("failed to execute has-nerd-font")
}

fn parse_stdout_json(output: &Output) -> JsonValue {
    serde_json::from_slice::<JsonValue>(&output.stdout).expect("stdout should be valid JSON")
}

fn terminal_plist_path(home: &Path) -> PathBuf {
    home.join("Library/Preferences/com.apple.Terminal.plist")
}

#[cfg(target_os = "macos")]
fn write_terminal_plist(home: &Path, profile: &str, font: &str) {
    let plist_path = terminal_plist_path(home);
    fs::create_dir_all(
        plist_path
            .parent()
            .expect("terminal plist should have a parent directory"),
    )
    .expect("failed to create plist directory");

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
        .expect("failed to write plist fixture");
}

#[test]
fn no_flags_emits_no_output_and_uses_result_exit_code() {
    let output = run_cli(&[], &[("TERM_PROGRAM", "ghostty")]);

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

#[test]
fn json_flag_emits_valid_json_with_exit_code() {
    let output = run_cli(&["--json"], &[("TERM_PROGRAM", "ghostty")]);

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());

    let json = parse_stdout_json(&output);
    assert_eq!(json["exit_code"].as_i64(), Some(0));
}

#[test]
fn explain_flag_writes_explanation_to_stderr() {
    let output = run_cli(&["--explain"], &[("TERM_PROGRAM", "ghostty")]);

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("ships with Nerd Font support"));
}

#[test]
fn json_and_explain_split_stdout_and_stderr() {
    let output = run_cli(&["--json", "--explain"], &[("TERM_PROGRAM", "ghostty")]);

    assert_eq!(output.status.code(), Some(0));

    let json = parse_stdout_json(&output);
    assert_eq!(json["exit_code"].as_i64(), Some(0));

    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
    assert!(stderr.contains("ships with Nerd Font support"));
}

#[test]
fn terminal_app_vertical_path_uses_home_plist_fixture() {
    let home = TempDir::new().expect("temp HOME should be created");

    #[cfg(target_os = "macos")]
    write_terminal_plist(home.path(), "Basic", "JetBrainsMono Nerd Font");

    let home_value = home.path().to_string_lossy().to_string();
    let output = run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_value)],
    );

    #[cfg(target_os = "macos")]
    {
        assert_eq!(output.status.code(), Some(0));
        let json = parse_stdout_json(&output);
        assert_eq!(json["exit_code"].as_i64(), Some(0));
        assert_eq!(json["source"].as_str(), Some("terminal_config"));
        assert_eq!(json["detected"].as_bool(), Some(true));

        let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
        assert!(stderr.contains("terminal configuration indicates a Nerd Font is active"));
    }

    #[cfg(not(target_os = "macos"))]
    {
        assert_eq!(output.status.code(), Some(5));
        let json = parse_stdout_json(&output);
        assert_eq!(json["exit_code"].as_i64(), Some(5));
        assert_eq!(json["source"].as_str(), Some("config_error"));

        let stderr = String::from_utf8(output.stderr).expect("stderr should be utf-8");
        assert!(stderr.contains("failed to read terminal configuration"));
    }
}
