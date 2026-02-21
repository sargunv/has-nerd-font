use crate::Terminal;

pub enum TerminalDecision {
    Bundled(Terminal),
    Identified(Terminal),
    Unknown,
}

pub fn detect(vars: &[(String, String)]) -> TerminalDecision {
    if vars
        .iter()
        .any(|(key, value)| key == "TERM_PROGRAM" && value.eq_ignore_ascii_case("ghostty"))
    {
        return TerminalDecision::Bundled(Terminal::Ghostty);
    }

    if vars
        .iter()
        .any(|(key, value)| key == "TERM_PROGRAM" && value == "Apple_Terminal")
    {
        return TerminalDecision::Identified(Terminal::TerminalApp);
    }

    TerminalDecision::Unknown
}
