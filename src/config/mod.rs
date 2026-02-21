use std::path::Path;

use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

pub fn resolve(terminal: Terminal, _cwd: &Path) -> DetectionResult {
    DetectionResult {
        detected: None,
        source: DetectionSource::NoResolver,
        terminal: Some(terminal),
        font: None,
        config_path: None,
        profile: None,
        error_reason: None,
        confidence: Confidence::Certain,
    }
}
