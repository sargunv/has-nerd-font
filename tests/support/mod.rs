#![allow(dead_code)]

use std::path::Path;
#[cfg(target_os = "macos")]
use std::path::PathBuf;
use std::process::{Command, Output};

use serde_json::Value;

pub fn run_cli(args: &[&str], env: &[(&str, &str)], cwd: Option<&Path>) -> Output {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("has-nerd-font"));
    command.env_clear();
    command.args(args);

    for (key, value) in env {
        command.env(key, value);
    }

    if let Some(path) = cwd {
        command.current_dir(path);
    }

    command.output().expect("failed to execute has-nerd-font")
}

pub fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout should be valid utf-8")
}

pub fn stdout_json_snapshot(output: &Output) -> String {
    let mut json: Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be valid JSON");

    if let Some(config_path) = json.get_mut("config_path")
        && let Some(path) = config_path.as_str()
    {
        let scenario_root = std::env::temp_dir()
            .join("has-nerd-font-snapshots")
            .to_string_lossy()
            .to_string();

        let normalized = path.replace(&scenario_root, "<SCENARIO_HOME>");
        *config_path = Value::String(normalized);
    }

    format!(
        "{}\n",
        serde_json::to_string_pretty(&json).expect("failed to re-serialize JSON")
    )
}

pub fn stderr_text(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr should be valid utf-8")
}

#[cfg(target_os = "macos")]
pub fn scenario_home(name: &str) -> PathBuf {
    let path = std::env::temp_dir()
        .join("has-nerd-font-snapshots")
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).expect("failed to create scenario home");
    path
}

#[cfg(target_os = "macos")]
pub fn install_terminal_app_fixture(home: &Path, fixture_name: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("terminal_app")
        .join(fixture_name);
    let plist_path = home.join("Library/Preferences/com.apple.Terminal.plist");
    std::fs::create_dir_all(
        plist_path
            .parent()
            .expect("terminal plist should have parent directory"),
    )
    .expect("failed to create terminal plist directory");

    std::fs::copy(&fixture_path, &plist_path).expect("failed to copy terminal plist fixture");
}
