use crate::{Confidence, DetectionResult, DetectionSource, Terminal};

pub enum TerminalLayerSignal {
    Bundled(DetectionResult),
    Identified(Terminal),
    Unknown,
}

pub fn detect(vars: &[(String, String)]) -> TerminalLayerSignal {
    if vars
        .iter()
        .any(|(key, value)| key == "TERM_PROGRAM" && value.eq_ignore_ascii_case("ghostty"))
    {
        return TerminalLayerSignal::Bundled(DetectionResult {
            detected: Some(true),
            source: DetectionSource::BundledTerminal,
            terminal: Some(Terminal::Ghostty),
            font: None,
            config_path: None,
            profile: None,
            error_reason: None,
            confidence: Confidence::Certain,
        });
    }

    if vars
        .iter()
        .any(|(key, value)| key == "TERM_PROGRAM" && value == "Apple_Terminal")
    {
        return TerminalLayerSignal::Identified(Terminal::TerminalApp);
    }

    TerminalLayerSignal::Unknown
}
