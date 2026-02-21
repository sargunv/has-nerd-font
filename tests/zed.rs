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

#[test]
fn zed_project_override_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-project-override");
    support::install_zed_fixture(&home, "zed-default.jsonc"); // user: non-Nerd
    let cwd = tempfile::tempdir().expect("failed to create temp dir");
    support::install_zed_project_fixture(cwd.path(), "zed-nerd-font-buffer.jsonc"); // project: Nerd
    let home_str = home.to_string_lossy().to_string();
    let cwd_str = cwd
        .path()
        .canonicalize()
        .expect("failed to canonicalize cwd")
        .to_string_lossy()
        .to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        Some(cwd.path()),
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "zed_project_override_json",
        support::stdout_json_snapshot_with_extra_normalizations(
            &output,
            &[(&cwd_str, "<PROJECT_CWD>")]
        )
    );
    assert_snapshot!(
        "zed_project_override_explain",
        support::stderr_text(&output)
    );
}
