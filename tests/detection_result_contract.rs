use has_nerd_font::{Confidence, DetectionResult, DetectionSource, Terminal};

fn sample_result(source: DetectionSource, detected: Option<bool>) -> DetectionResult {
    DetectionResult {
        detected,
        source,
        terminal: Some(Terminal::TerminalApp),
        font: Some("MesloLGS Nerd Font".to_string()),
        config_path: None,
        profile: Some("Default".to_string()),
        error_reason: None,
        confidence: Confidence::Certain,
    }
}

#[test]
fn detection_result_contract_exit_code_maps_from_source_and_detected() {
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
        (sample_result(DetectionSource::ConfigError, None), 5),
        (
            sample_result(DetectionSource::TerminalConfig, Some(true)),
            0,
        ),
        (
            sample_result(DetectionSource::TerminalConfig, Some(false)),
            6,
        ),
        (sample_result(DetectionSource::TerminalConfig, None), 5),
    ];

    for (result, expected) in cases {
        assert_eq!(result.exit_code(), expected);
    }
}

#[test]
fn detection_result_contract_result_serializes_key_fields_for_json_output() {
    let result = sample_result(DetectionSource::TerminalConfig, Some(false));
    let json = result.to_json_value();

    assert_eq!(json["detected"], serde_json::Value::Bool(false));
    assert_eq!(
        json["source"],
        serde_json::Value::String("terminal_config".to_string())
    );
    assert_eq!(
        json["terminal"],
        serde_json::Value::String("terminal_app".to_string())
    );
    assert_eq!(
        json["confidence"],
        serde_json::Value::String("certain".to_string())
    );
    assert_eq!(json["exit_code"], serde_json::Value::Number(6.into()));
}

#[test]
fn detection_result_contract_config_error_json_keeps_source_string_and_reason_field() {
    let mut result = sample_result(DetectionSource::ConfigError, None);
    result.error_reason = Some("missing plist".to_string());

    let json = result.to_json_value();

    assert_eq!(
        json["source"],
        serde_json::Value::String("config_error".to_string())
    );
    assert_eq!(
        json["error_reason"],
        serde_json::Value::String("missing plist".to_string())
    );
}

#[test]
fn detection_result_contract_unknown_terminal_serializes_as_raw_string() {
    let mut result = sample_result(DetectionSource::NoResolver, None);
    result.terminal = Some(Terminal::Unknown("CoolNewTerm".to_string()));

    let json = result.to_json_value();

    assert_eq!(
        json["terminal"],
        serde_json::Value::String("CoolNewTerm".to_string())
    );
}

#[test]
fn detection_result_contract_missing_terminal_serializes_as_null() {
    let mut result = sample_result(DetectionSource::UnknownTerminal, None);
    result.terminal = None;

    let json = result.to_json_value();

    assert_eq!(json["terminal"], serde_json::Value::Null);
}

#[test]
fn detection_result_contract_explain_mentions_key_semantics() {
    let explicit_disable = sample_result(DetectionSource::ExplicitDisable, Some(false));
    let unknown_terminal = sample_result(DetectionSource::UnknownTerminal, None);

    let explicit_text = explicit_disable.explain();
    let unknown_text = unknown_terminal.explain();

    assert!(explicit_text.contains("explicit"));
    assert!(explicit_text.contains("disabled"));
    assert!(unknown_text.contains("unknown"));
    assert!(unknown_text.contains("terminal"));
}
