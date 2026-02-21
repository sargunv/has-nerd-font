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
