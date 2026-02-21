#![cfg(target_os = "macos")]

mod support;

use insta::assert_snapshot;

#[test]
fn terminal_app_default_snapshots_json_and_explain() {
    let home = support::scenario_home("terminal-app-real-default");
    support::install_terminal_app_fixture(&home, "terminal-app-real-default.plist");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!(
        "terminal_app_default_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "terminal_app_default_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn terminal_app_nerd_font_snapshots_json_and_explain() {
    let home = support::scenario_home("terminal-app-real-nerd-font");
    support::install_terminal_app_fixture(&home, "terminal-app-real-nerd-font.plist");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "terminal_app_nerd_font_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "terminal_app_nerd_font_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn terminal_app_malformed_snapshots_json_and_explain() {
    let home = support::scenario_home("terminal-app-malformed");
    support::install_terminal_app_fixture(&home, "terminal-app-malformed.plist");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(5));
    assert_snapshot!(
        "terminal_app_malformed_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "terminal_app_malformed_explain",
        support::stderr_text(&output)
    );
}
