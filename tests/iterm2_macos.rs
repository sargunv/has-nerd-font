#![cfg(target_os = "macos")]

mod support;

use insta::assert_snapshot;

#[test]
fn iterm2_default_snapshots_json_and_explain() {
    let home = support::scenario_home("iterm2-real-default");
    support::install_iterm2_fixture(&home, "iterm2-real-default.plist");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "iTerm.app"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!(
        "iterm2_default_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("iterm2_default_explain", support::stderr_text(&output));
}

#[test]
fn iterm2_nerd_font_snapshots_json_and_explain() {
    let home = support::scenario_home("iterm2-real-nerd-font");
    support::install_iterm2_fixture(&home, "iterm2-real-nerd-font.plist");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "iTerm.app"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "iterm2_nerd_font_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("iterm2_nerd_font_explain", support::stderr_text(&output));
}

#[test]
fn iterm2_active_profile_snapshots_json_and_explain() {
    let home = support::scenario_home("iterm2-active-profile");
    support::install_iterm2_fixture(&home, "iterm2-multi-profile.plist");
    let home_str = home.to_string_lossy().to_string();

    // ITERM_PROFILE=NerdProfile should pick the nerd font profile,
    // even though the default bookmark uses Monaco (non-nerd font).
    let output = support::run_cli(
        &["--json", "--explain"],
        &[
            ("TERM_PROGRAM", "iTerm.app"),
            ("HOME", &home_str),
            ("ITERM_PROFILE", "NerdProfile"),
        ],
        None,
    );

    assert_eq!(output.status.code(), Some(0));
    assert_snapshot!(
        "iterm2_active_profile_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "iterm2_active_profile_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn iterm2_active_profile_fallback_snapshots_json_and_explain() {
    let home = support::scenario_home("iterm2-active-profile-fallback");
    support::install_iterm2_fixture(&home, "iterm2-multi-profile.plist");
    let home_str = home.to_string_lossy().to_string();

    // Without ITERM_PROFILE, should fall back to default bookmark (Monaco).
    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "iTerm.app"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(6));
    assert_snapshot!(
        "iterm2_active_profile_fallback_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!(
        "iterm2_active_profile_fallback_explain",
        support::stderr_text(&output)
    );
}

#[test]
fn iterm2_malformed_snapshots_json_and_explain() {
    let home = support::scenario_home("iterm2-malformed");
    support::install_iterm2_fixture(&home, "iterm2-malformed.plist");
    let home_str = home.to_string_lossy().to_string();

    let output = support::run_cli(
        &["--json", "--explain"],
        &[("TERM_PROGRAM", "iTerm.app"), ("HOME", &home_str)],
        None,
    );

    assert_eq!(output.status.code(), Some(5));
    assert_snapshot!(
        "iterm2_malformed_json",
        support::stdout_json_snapshot(&output)
    );
    assert_snapshot!("iterm2_malformed_explain", support::stderr_text(&output));
}
