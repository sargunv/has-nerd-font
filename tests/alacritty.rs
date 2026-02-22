mod support;

use insta::assert_snapshot;

#[test]
fn alacritty_nerd_font_snapshots_json_and_explain() {
    let home = support::scenario_home("alacritty-nerd-font");
    support::install_alacritty_fixture(&home, "alacritty-nerd-font.toml");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("ALACRITTY_LOG", "/tmp/fake.log"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "alacritty_nerd_font_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("alacritty_nerd_font_explain", support::stderr_text(&output));
}

#[test]
fn alacritty_non_nerd_font_snapshots_json_and_explain() {
    let home = support::scenario_home("alacritty-non-nerd-font");
    support::install_alacritty_fixture(&home, "alacritty-non-nerd-font.toml");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("ALACRITTY_LOG", "/tmp/fake.log"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!(
        "alacritty_non_nerd_font_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "alacritty_non_nerd_font_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn alacritty_default_snapshots_json_and_explain() {
    let home = support::scenario_home("alacritty-default");
    support::install_alacritty_fixture(&home, "alacritty-default.toml");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("ALACRITTY_LOG", "/tmp/fake.log"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(5));
    assert_snapshot!(
        "alacritty_default_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("alacritty_default_explain", support::stderr_text(&output));
}

#[test]
fn alacritty_malformed_snapshots_json_and_explain() {
    let home = support::scenario_home("alacritty-malformed");
    support::install_alacritty_fixture(&home, "alacritty-malformed.toml");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("ALACRITTY_LOG", "/tmp/fake.log"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(5));
    assert_snapshot!(
        "alacritty_malformed_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("alacritty_malformed_explain", support::stderr_text(&output));
}
