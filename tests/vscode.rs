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
