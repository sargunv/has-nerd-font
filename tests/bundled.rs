mod support;

use insta::assert_snapshot;

#[test]
fn ghostty_bundled_terminal_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "ghostty")],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "bundled_ghostty_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("bundled_ghostty_explain", support::stderr_text(&output));
}

#[test]
fn opencode_bundled_terminal_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("OPENCODE_TERMINAL", "1")],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "bundled_opencode_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("bundled_opencode_explain", support::stderr_text(&output));
}

#[test]
fn conductor_bundled_terminal_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("CONDUCTOR_WORKSPACE_NAME", "my-workspace")],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "bundled_conductor_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("bundled_conductor_explain", support::stderr_text(&output));
}
