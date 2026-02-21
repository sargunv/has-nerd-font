use has_nerd_font::{Confidence, DetectionResult, DetectionSource, Terminal};

fn sample_result(source: DetectionSource, detected: Option<bool>) -> DetectionResult {
    DetectionResult {
        detected,
        source,
        terminal: Some(Terminal::TerminalApp),
        font: Some("MesloLGS Nerd Font".to_string()),
        config_path: None,
        profile: Some("Default".to_string()),
        confidence: Confidence::Certain,
    }
}

#[test]
fn exit_code_maps_from_source_and_detected() {
    let cases = [
        (sample_result(DetectionSource::EnvVar, Some(true)), 0),
        (
            sample_result(DetectionSource::BundledTerminal, Some(true)),
            0,
        ),
        (
            sample_result(DetectionSource::ExplicitDisable, Some(false)),
            1,
        ),
        (sample_result(DetectionSource::UnknownTerminal, None), 2),
        (sample_result(DetectionSource::RemoteSession, None), 3),
        (sample_result(DetectionSource::NoResolver, None), 4),
        (
            sample_result(
                DetectionSource::ConfigError {
                    reason: "missing plist".to_string(),
                },
                None,
            ),
            5,
        ),
        (
            sample_result(DetectionSource::TerminalConfig, Some(true)),
            0,
        ),
        (
            sample_result(DetectionSource::TerminalConfig, Some(false)),
            6,
        ),
    ];

    for (result, expected) in cases {
        assert_eq!(result.exit_code(), expected);
    }
}

#[test]
fn result_serializes_key_fields_for_json_output() {
    let result = sample_result(DetectionSource::TerminalConfig, Some(false));
    let json = serde_json::to_value(&result).expect("result should serialize");

    assert_eq!(json["detected"], serde_json::Value::Bool(false));
    assert_eq!(
        json["source"],
        serde_json::Value::String("TerminalConfig".to_string())
    );
    assert_eq!(
        json["terminal"],
        serde_json::Value::String("TerminalApp".to_string())
    );
    assert_eq!(
        json["confidence"],
        serde_json::Value::String("Certain".to_string())
    );
    assert_eq!(result.exit_code(), 6);
}

#[test]
fn explain_mentions_key_semantics() {
    let explicit_disable = sample_result(DetectionSource::ExplicitDisable, Some(false));
    let unknown_terminal = sample_result(DetectionSource::UnknownTerminal, None);

    let explicit_text = explicit_disable.explain();
    let unknown_text = unknown_terminal.explain();

    assert!(explicit_text.contains("explicit"));
    assert!(explicit_text.contains("disabled"));
    assert!(unknown_text.contains("unknown"));
    assert!(unknown_text.contains("terminal"));
}
