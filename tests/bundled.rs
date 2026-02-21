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
