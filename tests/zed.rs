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
fn zed_project_override_subdirectory_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-project-override-subdir");
    support::install_zed_fixture(&home, "zed-default.jsonc"); // user: non-Nerd
    let project_root = home.join("projects/my-project");
    std::fs::create_dir_all(&project_root).expect("failed to create project dir");
    support::install_zed_project_fixture(&project_root, "zed-nerd-font-buffer.jsonc"); // project: Nerd
    let subdir = project_root.join("src/deeply/nested");
    std::fs::create_dir_all(&subdir).expect("failed to create subdirectory");
    let home_str = home.to_string_lossy().to_string();

    // Run from a subdirectory — should still find .zed/settings.json at the project root
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        Some(&subdir),
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "zed_project_override_subdirectory_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "zed_project_override_subdirectory_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn zed_project_override_snapshots_json_and_explain() {
    let home = support::scenario_home("zed-project-override");
    support::install_zed_fixture(&home, "zed-default.jsonc"); // user: non-Nerd
    let cwd = home.join("projects/my-project");
    std::fs::create_dir_all(&cwd).expect("failed to create project dir");
    support::install_zed_project_fixture(&cwd, "zed-nerd-font-buffer.jsonc"); // project: Nerd
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "zed"), ("HOME", &home_str)],
        Some(&cwd),
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "zed_project_override_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "zed_project_override_explain",
        support::stderr_text(&output)
    );
}
