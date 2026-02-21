mod support;

use insta::assert_snapshot;

const VSCODE_ASKPASS: &str = "/app/share/code/code";
const VSCODE_APP_DIR: &str = "Code";

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
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("vscode_default_explain", support::stderr_text(&output));
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
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "vscode_nerd_font_editor_explain",
        support::stderr_text(&output)
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
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "vscode_nerd_font_terminal_explain",
        support::stderr_text(&output)
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
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "vscode_malformed_explain",
        support::stderr_text_normalized(&output, &[])
    );
}

#[test]
fn vscode_project_override_snapshots_json_and_explain() {
    let home = support::scenario_home("vscode-project-override");
    support::install_vscode_fixture(&home, "vscode-default.jsonc", VSCODE_APP_DIR); // user: non-Nerd
    let cwd = tempfile::tempdir().expect("failed to create temp dir");
    support::install_vscode_project_fixture(cwd.path(), "vscode-nerd-font-editor.jsonc"); // project: Nerd
    let home_str = home.to_string_lossy().to_string();
    let cwd_str = cwd
        .path()
        .canonicalize()
        .expect("failed to canonicalize cwd")
        .to_string_lossy()
        .to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &vscode_env(&home_str),
        Some(cwd.path()),
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "vscode_project_override_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            &[(&cwd_str, "<PROJECT_CWD>")]
        )
    );
    assert_snapshot!(
        "vscode_project_override_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn vscode_project_override_subdirectory_snapshots_json_and_explain() {
    let home = support::scenario_home("vscode-project-override-subdir");
    support::install_vscode_fixture(&home, "vscode-default.jsonc", VSCODE_APP_DIR); // user: non-Nerd
    let project_root = tempfile::tempdir().expect("failed to create temp dir");
    support::install_vscode_project_fixture(project_root.path(), "vscode-nerd-font-editor.jsonc"); // project: Nerd
    let subdir = project_root.path().join("src/deeply/nested");
    std::fs::create_dir_all(&subdir).expect("failed to create subdirectory");
    let home_str = home.to_string_lossy().to_string();
    let project_root_str = project_root
        .path()
        .canonicalize()
        .expect("failed to canonicalize project root")
        .to_string_lossy()
        .to_string();

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
            &[(&project_root_str, "<PROJECT_CWD>")]
        )
    );
    assert_snapshot!(
        "vscode_project_override_subdirectory_explain",
        support::stderr_text(&output)
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
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "vscodium_nerd_font_editor_explain",
        support::stderr_text(&output)
    );
}
