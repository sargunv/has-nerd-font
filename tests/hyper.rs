mod support;

use insta::assert_snapshot;

#[test]
fn hyper_default_snapshots_json_and_explain() {
    let home = support::scenario_home("hyper-default");
    support::install_hyper_fixture(&home, "hyper-default.json");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "hyper"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!("hyper_default_json", support::stdout_json_snapshot(&output));
    assert_snapshot!("hyper_default_explain", support::stderr_text(&output));
}

#[test]
fn hyper_nerd_font_snapshots_json_and_explain() {
    let home = support::scenario_home("hyper-nerd-font");
    support::install_hyper_fixture(&home, "hyper-nerd-font.json");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "hyper"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "hyper_nerd_font_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("hyper_nerd_font_explain", support::stderr_text(&output));
}

#[test]
fn hyper_malformed_snapshots_json_and_explain() {
    let home = support::scenario_home("hyper-malformed");
    support::install_hyper_fixture(&home, "hyper-malformed.json");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "hyper"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(5));
    assert_snapshot!(
        "hyper_malformed_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "hyper_malformed_explain",
        support::stderr_text_normalized(&output, &[])
    );
}
