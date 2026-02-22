mod support;

use insta::assert_snapshot;

const VSCODE_ASKPASS: &str = "/app/share/code/code";
const VSCODE_APP_DIR: &str = "Code";

const APP_SUPPORT_NORMALIZATIONS: &[(&str, &str)] = if cfg!(target_os = "macos") {
    &[("Library/Application Support", "<APP_SUPPORT>")]
} else {
    &[(".config", "<APP_SUPPORT>")]
};

fn vscode_env<'a>(home_str: &'a str) -> Vec<(&'a str, &'a str)> {
    vec![
        ("TERM_PROGRAM", "vscode"),
        ("HOME", home_str),
        ("VSCODE_GIT_ASKPASS_NODE", VSCODE_ASKPASS),
    ]
}

#[test]
fn vscode_default_snapshots_json_and_explain() {
    let home = support::scenario_home("vscode-default");
    support::install_vscode_fixture(&home, "vscode-default.jsonc", VSCODE_APP_DIR);
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(&["--json", "--explain"], &vscode_env(&home_str), None);

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!(
        "vscode_default_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            APP_SUPPORT_NORMALIZATIONS
        )
    );
    assert_snapshot!(
        "vscode_default_explain",
        support::stderr_text_normalized(&output, APP_SUPPORT_NORMALIZATIONS)
    );
}

#[test]
fn vscode_nerd_font_editor_snapshots_json_and_explain() {
    let home = support::scenario_home("vscode-nerd-font-editor");
    support::install_vscode_fixture(&home, "vscode-nerd-font-editor.jsonc", VSCODE_APP_DIR);
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(&["--json", "--explain"], &vscode_env(&home_str), None);

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "vscode_nerd_font_editor_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            APP_SUPPORT_NORMALIZATIONS
        )
    );
    assert_snapshot!(
        "vscode_nerd_font_editor_explain",
        support::stderr_text_normalized(&output, APP_SUPPORT_NORMALIZATIONS)
    );
}

#[test]
fn vscode_nerd_font_terminal_snapshots_json_and_explain() {
    let home = support::scenario_home("vscode-nerd-font-terminal");
    support::install_vscode_fixture(&home, "vscode-nerd-font-terminal.jsonc", VSCODE_APP_DIR);
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(&["--json", "--explain"], &vscode_env(&home_str), None);

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "vscode_nerd_font_terminal_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            APP_SUPPORT_NORMALIZATIONS
        )
    );
    assert_snapshot!(
        "vscode_nerd_font_terminal_explain",
        support::stderr_text_normalized(&output, APP_SUPPORT_NORMALIZATIONS)
    );
}

#[test]
fn vscode_malformed_snapshots_json_and_explain() {
    let home = support::scenario_home("vscode-malformed");
    support::install_vscode_fixture(&home, "vscode-malformed.jsonc", VSCODE_APP_DIR);
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(&["--json", "--explain"], &vscode_env(&home_str), None);

    assert_eq!(output.status.code(), Some(5));
    assert_snapshot!(
        "vscode_malformed_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            APP_SUPPORT_NORMALIZATIONS
        )
    );
    assert_snapshot!(
        "vscode_malformed_explain",
        support::stderr_text_normalized(&output, APP_SUPPORT_NORMALIZATIONS)
    );
}

#[test]
fn vscode_project_override_snapshots_json_and_explain() {
    let home = support::scenario_home("vscode-project-override");
    support::install_vscode_fixture(&home, "vscode-default.jsonc", VSCODE_APP_DIR); // user: non-Nerd
    let cwd = home.join("projects/my-project");
    std::fs::create_dir_all(&cwd).expect("failed to create project dir");
    support::install_vscode_project_fixture(&cwd, "vscode-nerd-font-editor.jsonc"); // project: Nerd
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(&["--json", "--explain"], &vscode_env(&home_str), Some(&cwd));

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "vscode_project_override_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            APP_SUPPORT_NORMALIZATIONS
        )
    );
    assert_snapshot!(
        "vscode_project_override_explain",
        support::stderr_text_normalized(&output, APP_SUPPORT_NORMALIZATIONS)
    );
}

#[test]
fn vscode_project_override_subdirectory_snapshots_json_and_explain() {
    let home = support::scenario_home("vscode-project-override-subdir");
    support::install_vscode_fixture(&home, "vscode-default.jsonc", VSCODE_APP_DIR); // user: non-Nerd
    let project_root = home.join("projects/my-project");
    std::fs::create_dir_all(&project_root).expect("failed to create project dir");
    support::install_vscode_project_fixture(&project_root, "vscode-nerd-font-editor.jsonc"); // project: Nerd
    let subdir = project_root.join("src/deeply/nested");
    std::fs::create_dir_all(&subdir).expect("failed to create subdirectory");
    let home_str = home.to_string_lossy().to_string();

    // Run from a subdirectory — should still find .vscode/settings.json at the project root
    let output = support::run_cli(
        &["--json", "--explain"],
        &vscode_env(&home_str),
        Some(&subdir),
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "vscode_project_override_subdirectory_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            APP_SUPPORT_NORMALIZATIONS
        )
    );
    assert_snapshot!(
        "vscode_project_override_subdirectory_explain",
        support::stderr_text_normalized(&output, APP_SUPPORT_NORMALIZATIONS)
    );
}

const VSCODIUM_ASKPASS: &str = "/app/share/codium/codium";
const VSCODIUM_APP_DIR: &str = "VSCodium";

fn vscodium_env<'a>(home_str: &'a str) -> Vec<(&'a str, &'a str)> {
    vec![
        ("TERM_PROGRAM", "vscode"),
        ("HOME", home_str),
        ("VSCODE_GIT_ASKPASS_NODE", VSCODIUM_ASKPASS),
    ]
}

#[test]
fn vscodium_nerd_font_editor_snapshots_json_and_explain() {
    let home = support::scenario_home("vscodium-nerd-font-editor");
    support::install_vscode_fixture(&home, "vscode-nerd-font-editor.jsonc", VSCODIUM_APP_DIR);
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(&["--json", "--explain"], &vscodium_env(&home_str), None);

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "vscodium_nerd_font_editor_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            APP_SUPPORT_NORMALIZATIONS
        )
    );
    assert_snapshot!(
        "vscodium_nerd_font_editor_explain",
        support::stderr_text_normalized(&output, APP_SUPPORT_NORMALIZATIONS)
    );
}
