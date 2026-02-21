#![cfg(target_os = "macos")]

mod support;

use insta::assert_snapshot;

#[test]
fn terminal_app_nerd_font_snapshots_json_and_explain() {
    let home = support::scenario_home("terminal-app-nerd-font");
    support::write_terminal_app_plist(&home, "Basic", "JetBrainsMono Nerd Font");
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
fn terminal_app_non_nerd_font_snapshots_json_and_explain() {
    let home = support::scenario_home("terminal-app-non-nerd-font");
    support::write_terminal_app_plist(&home, "Basic", "Menlo");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "Apple_Terminal"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!(
        "terminal_app_non_nerd_font_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "terminal_app_non_nerd_font_explain",
        support::stderr_text(&output)
    );
}
