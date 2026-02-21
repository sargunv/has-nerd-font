use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    pub detected: bool,
}

pub fn detect(_vars: &[(String, String)], _cwd: &Path) -> DetectionResult {
    DetectionResult { detected: false }
}
