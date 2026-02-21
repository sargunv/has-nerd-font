mod support;

use insta::assert_snapshot;

#[test]
fn unmapped_terminal_program_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "CoolNewTerm")],
        None,
    );

    assert_eq!(output.status.code(), Some(4));
    assert_snapshot!(
        "no_resolver_unknown_terminal_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "no_resolver_unknown_terminal_explain",
        support::stderr_text(&output)
    );
}
