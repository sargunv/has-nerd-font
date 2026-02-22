mod support;

use insta::assert_snapshot;

#[test]
fn zed_default_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-default");
    support::install_zed_fixture(&home, "zed-default.jsonc");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!("zed_default_json", support::stdout_json_snapshot(&output));
    assert_snapshot!("zed_default_explain", support::stderr_text(&output));
}

#[test]
fn zed_nerd_font_buffer_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-nerd-font-buffer");
    support::install_zed_fixture(&home, "zed-nerd-font-buffer.jsonc");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "zed_nerd_font_buffer_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "zed_nerd_font_buffer_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn zed_nerd_font_terminal_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-nerd-font-terminal");
    support::install_zed_fixture(&home, "zed-nerd-font-terminal.jsonc");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "zed_nerd_font_terminal_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "zed_nerd_font_terminal_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn zed_malformed_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-malformed");
    support::install_zed_fixture(&home, "zed-malformed.jsonc");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(5));
    assert_snapshot!("zed_malformed_json", support::stdout_json_snapshot(&output));
    assert_snapshot!(
        "zed_malformed_explain",
        support::stderr_text_normalized(&output, &[])
    );
}
