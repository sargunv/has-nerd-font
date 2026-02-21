use std::path::Path;

use has_nerd_font::{DetectionSource, detect};

fn vars(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

#[test]
fn nerd_font_truthy_tokens_produce_env_override_yes() {
    for token in ["1", "true", "yes", " TRUE ", "  YeS  "] {
        let env = vars(&[("NERD_FONT", token), ("TERM_PROGRAM", "Apple_Terminal")]);

        let result = detect(&env, Path::new("."));

        assert_eq!(
            result.source,
            DetectionSource::EnvVar,
            "token {token:?} should finalize at env layer"
        );
        assert_eq!(result.detected, Some(true));
        assert_eq!(result.exit_code(), 0);
    }
}

#[test]
fn nerd_font_falsy_tokens_produce_explicit_disable() {
    for token in ["0", "false", "no", " FALSE ", "  nO  "] {
        let env = vars(&[("NERD_FONT", token), ("TERM_PROGRAM", "ghostty")]);

        let result = detect(&env, Path::new("."));

        assert_eq!(
            result.source,
            DetectionSource::ExplicitDisable,
            "token {token:?} should finalize at env layer"
        );
        assert_eq!(result.detected, Some(false));
        assert_eq!(result.exit_code(), 1);
    }
}

#[test]
fn unset_nerd_font_continues_to_later_layers() {
    let env = vars(&[("TERM_PROGRAM", "ghostty")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::BundledTerminal);
    assert_eq!(result.detected, Some(true));
}

#[test]
fn unrecognized_nerd_font_value_continues_to_later_layers() {
    let env = vars(&[("NERD_FONT", "maybe"), ("TERM_PROGRAM", "Apple_Terminal")]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::NoResolver);
    assert_eq!(result.detected, None);
}

#[test]
fn duplicate_nerd_font_keys_use_last_value_when_truthy_then_falsy() {
    let env = vars(&[
        ("NERD_FONT", "yes"),
        ("TERM_PROGRAM", "ghostty"),
        ("NERD_FONT", "0"),
    ]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::ExplicitDisable);
    assert_eq!(result.detected, Some(false));
}

#[test]
fn duplicate_nerd_font_keys_use_last_value_when_falsy_then_truthy() {
    let env = vars(&[
        ("NERD_FONT", "0"),
        ("TERM_PROGRAM", "Apple_Terminal"),
        ("NERD_FONT", "true"),
    ]);

    let result = detect(&env, Path::new("."));

    assert_eq!(result.source, DetectionSource::EnvVar);
    assert_eq!(result.detected, Some(true));
}

#[test]
fn empty_or_whitespace_nerd_font_values_continue_to_later_layers() {
    for token in ["", " ", "   ", "\t", "\n"] {
        let env = vars(&[("NERD_FONT", token), ("TERM_PROGRAM", "ghostty")]);

        let result = detect(&env, Path::new("."));

        assert_eq!(
            result.source,
            DetectionSource::BundledTerminal,
            "token {token:?} should continue past env layer"
        );
        assert_eq!(result.detected, Some(true));
    }
}
