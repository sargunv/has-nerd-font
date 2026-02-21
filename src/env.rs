use crate::{Confidence, DetectionResult, DetectionSource};

pub fn detect(vars: &[(String, String)]) -> Option<DetectionResult> {
    if vars
        .iter()
        .any(|(key, value)| key == "NERD_FONT" && value.trim() == "1")
    {
        return Some(DetectionResult {
            detected: Some(true),
            source: DetectionSource::EnvVar,
            terminal: None,
            font: None,
            config_path: None,
            profile: None,
            error_reason: None,
            confidence: Confidence::Certain,
        });
    }

    None
}
