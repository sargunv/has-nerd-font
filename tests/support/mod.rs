#![allow(dead_code)]

use std::path::Path;
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
    std::str::from_utf8(&output.stdout)
        .expect("stdout should be valid utf-8")
        .to_owned()
}

pub fn stdout_json_snapshot(output: &Output) -> String {
    stdout_json_snapshot_with_extra_normalizations(output, &[])
}

pub fn stdout_json_snapshot_with_extra_normalizations(
    output: &Output,
    extra: &[(&str, &str)],
) -> String {
    let mut json: Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be valid JSON");

    let scenario_root = snapshot_root().to_string_lossy().to_string();

    for key in &["config_path", "error_reason"] {
        if let Some(field) = json.get_mut(*key)
            && let Some(value) = field.as_str()
        {
            let mut normalized = value.replace(&scenario_root, "<SCENARIO_HOME>");
            for (from, to) in extra {
                normalized = normalized.replace(from, to);
            }
            *field = Value::String(normalized);
        }
    }

    format!(
        "{}\n",
        serde_json::to_string_pretty(&json).expect("failed to re-serialize JSON")
    )
}

pub fn stderr_text(output: &Output) -> String {
    stderr_text_normalized(output, &[])
}

pub fn stderr_text_normalized(output: &Output, extra: &[(&str, &str)]) -> String {
    let text = std::str::from_utf8(&output.stderr)
        .expect("stderr should be valid utf-8")
        .to_owned();
    let scenario_root = snapshot_root().to_string_lossy().to_string();
    let mut normalized = text.replace(&scenario_root, "<SCENARIO_HOME>");
    for (from, to) in extra {
        normalized = normalized.replace(from, to);
    }
    normalized
}

fn snapshot_root() -> PathBuf {
    std::env::temp_dir().join("has-nerd-font-snapshots")
}

pub fn scenario_home(name: &str) -> PathBuf {
    let path = snapshot_root().join(name);
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

#[cfg(target_os = "macos")]
pub fn install_iterm2_fixture(home: &Path, fixture_name: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("iterm2")
        .join(fixture_name);
    let plist_path = home.join("Library/Preferences/com.googlecode.iterm2.plist");
    std::fs::create_dir_all(
        plist_path
            .parent()
            .expect("iTerm2 plist should have parent directory"),
    )
    .expect("failed to create iTerm2 plist directory");

    std::fs::copy(&fixture_path, &plist_path).expect("failed to copy iTerm2 plist fixture");
}

pub fn install_vscode_fixture(home: &Path, fixture_name: &str, app_dir: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("vscode")
        .join(fixture_name);
    let settings_path = if cfg!(target_os = "macos") {
        home.join(format!(
            "Library/Application Support/{app_dir}/User/settings.json"
        ))
    } else {
        home.join(format!(".config/{app_dir}/User/settings.json"))
    };
    std::fs::create_dir_all(
        settings_path
            .parent()
            .expect("vscode settings should have parent directory"),
    )
    .expect("failed to create vscode settings directory");
    std::fs::copy(&fixture_path, &settings_path).expect("failed to copy vscode settings fixture");
}

pub fn install_vscode_project_fixture(cwd: &Path, fixture_name: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("vscode")
        .join(fixture_name);
    let settings_path = cwd.join(".vscode/settings.json");
    std::fs::create_dir_all(
        settings_path
            .parent()
            .expect("vscode project settings should have parent directory"),
    )
    .expect("failed to create vscode project settings directory");
    std::fs::copy(&fixture_path, &settings_path)
        .expect("failed to copy vscode project settings fixture");
}

pub fn install_zed_fixture(home: &Path, fixture_name: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("zed")
        .join(fixture_name);
    let settings_path = home.join(".config/zed/settings.json");
    std::fs::create_dir_all(
        settings_path
            .parent()
            .expect("zed settings should have parent directory"),
    )
    .expect("failed to create zed settings directory");
    std::fs::copy(&fixture_path, &settings_path).expect("failed to copy zed settings fixture");
}

pub fn install_hyper_fixture(home: &Path, fixture_name: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("hyper")
        .join(fixture_name);
    let config_path = home.join(".config/Hyper/hyper.json");
    std::fs::create_dir_all(
        config_path
            .parent()
            .expect("hyper config should have parent directory"),
    )
    .expect("failed to create hyper config directory");
    std::fs::copy(&fixture_path, &config_path).expect("failed to copy hyper config fixture");
}

pub fn install_zed_project_fixture(cwd: &Path, fixture_name: &str) {
    let fixture_path = Path::new("tests")
        .join("fixtures")
        .join("zed")
        .join(fixture_name);
    let settings_path = cwd.join(".zed/settings.json");
    std::fs::create_dir_all(
        settings_path
            .parent()
            .expect("zed project settings should have parent directory"),
    )
    .expect("failed to create zed project settings directory");
    std::fs::copy(&fixture_path, &settings_path)
        .expect("failed to copy zed project settings fixture");
}
