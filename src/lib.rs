use std::path::Path;

mod types;

pub use types::{Confidence, DetectionResult, DetectionSource, Terminal};

pub fn detect(_vars: &[(String, String)], _cwd: &Path) -> DetectionResult {
    DetectionResult {
        detected: None,
        source: DetectionSource::UnknownTerminal,
        terminal: None,
        font: None,
        config_path: None,
        profile: None,
        error_reason: None,
        confidence: Confidence::Certain,
    }
}
