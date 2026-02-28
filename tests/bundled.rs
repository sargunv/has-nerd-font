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
fn kitty_bundled_terminal_snapshots_json_and_explain() {
    let output = support::run_cli(&["--json", "--explain"], &[("TERM_PROGRAM", "kitty")], None);

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!("bundled_kitty_json", support::stdout_json_snapshot(&output));
    assert_snapshot!("bundled_kitty_explain", support::stderr_text(&output));
}

#[test]
fn kitty_bundled_terminal_via_term_snapshots_json_and_explain() {
    let output = support::run_cli(&["--json", "--explain"], &[("TERM", "xterm-kitty")], None);

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "bundled_kitty_term_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("bundled_kitty_term_explain", support::stderr_text(&output));
}

#[test]
fn superset_bundled_terminal_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "Superset")],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "bundled_superset_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("bundled_superset_explain", support::stderr_text(&output));
}

#[test]
fn superset_bundled_terminal_via_env_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("SUPERSET_PANE_ID", "pane-1772261027171-p1p7lqjoe")],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "bundled_superset_env_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "bundled_superset_env_explain",
        support::stderr_text(&output)
    );
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
