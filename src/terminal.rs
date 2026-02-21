use crate::Terminal;

pub enum TerminalDecision {
    Bundled(Terminal),
    Identified(Terminal),
    Unknown,
}

pub fn detect(vars: &[(String, String)]) -> TerminalDecision {
    if let Some(value) = env_value(vars, "TERM_PROGRAM")
        && let Some(terminal) = from_term_program(value)
    {
        return decide(terminal);
    }

    if let Some(value) = env_value(vars, "TERM")
        && let Some(terminal) = from_term(value)
    {
        return decide(terminal);
    }

    if has_nonempty(vars, "WEZTERM_PANE") {
        return TerminalDecision::Bundled(Terminal::WezTerm);
    }

    if has_nonempty(vars, "KITTY_PID") {
        return TerminalDecision::Identified(Terminal::Kitty);
    }

    TerminalDecision::Unknown
}

fn decide(terminal: Terminal) -> TerminalDecision {
    if terminal.is_bundled() {
        TerminalDecision::Bundled(terminal)
    } else {
        TerminalDecision::Identified(terminal)
    }
}

fn env_value<'a>(vars: &'a [(String, String)], key: &str) -> Option<&'a str> {
    vars.iter()
        .rev()
        .find_map(|(k, v)| (k == key).then_some(v.as_str()))
}

fn has_nonempty(vars: &[(String, String)], key: &str) -> bool {
    env_value(vars, key).is_some_and(|value| !value.is_empty())
}

fn from_term_program(value: &str) -> Option<Terminal> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "ghostty" => Some(Terminal::Ghostty),
        "wezterm" => Some(Terminal::WezTerm),
        "apple_terminal" => Some(Terminal::TerminalApp),
        _ => None,
    }
}

fn from_term(value: &str) -> Option<Terminal> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "xterm-ghostty" => Some(Terminal::Ghostty),
        "wezterm" => Some(Terminal::WezTerm),
        _ => None,
    }
}
