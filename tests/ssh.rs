mod support;

use insta::assert_snapshot;

#[test]
fn ssh_session_snapshots_json_and_explain() {
    let output = support::run_cli(
        &["--json", "--explain"],
        &[
            ("TERM_PROGRAM", "Apple_Terminal"),
            ("SSH_TTY", "/dev/pts/1"),
        ],
        None,
    );

    assert_eq!(output.status.code(), Some(3));
    assert_snapshot!(
        "ssh_remote_session_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("ssh_remote_session_explain", support::stderr_text(&output));
}
