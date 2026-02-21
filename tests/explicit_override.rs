mod support;

use insta::assert_snapshot;

#[test]
fn explicit_override_enabled_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("NERD_FONT", "1"), ("TERM_PROGRAM", "Apple_Terminal")],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "override_enabled_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("override_enabled_explain", support::stderr_text(&output));
}

#[test]
fn explicit_override_disabled_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("NERD_FONT", "0"), ("TERM_PROGRAM", "ghostty")],
        None,
    );

    assert_eq!(output.status.code(), Some(1));
    assert_snapshot!(
        "override_disabled_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("override_disabled_explain", support::stderr_text(&output));
}
