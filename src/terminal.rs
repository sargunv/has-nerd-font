use crate::Terminal;

pub enum TerminalDecision {
    Bundled(Terminal),
    Identified(Terminal),
    Unknown,
}

pub fn detect(vars: &[(String, String)]) -> TerminalDecision {
    if let Some(value) = env_value(vars, "TERM_PROGRAM") {
        let raw = value.trim();

        if let Some(terminal) = from_term_program(raw) {
            return decide(terminal);
        }

        if !raw.is_empty() {
            return TerminalDecision::Identified(Terminal::Unknown(raw.to_string()));
        }
    }

    if let Some(value) = env_value(vars, "TERM")
        && let Some(terminal) = from_term(value)
    {
        return decide(terminal);
    }

    if env_value(vars, "OPENCODE_TERMINAL") == Some("1") {
        return decide(Terminal::OpenCode);
    }

    if env_value(vars, "CONDUCTOR_WORKSPACE_NAME").is_some() {
        return decide(Terminal::Conductor);
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

fn from_term_program(value: &str) -> Option<Terminal> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "ghostty" => Some(Terminal::Ghostty),
        "wezterm" => Some(Terminal::WezTerm),
        "iterm.app" => Some(Terminal::ITerm2),
        "apple_terminal" => Some(Terminal::TerminalApp),
        "vscode" => Some(Terminal::Vscode),
        "zed" => Some(Terminal::Zed),
        "hyper" => Some(Terminal::Hyper),
        _ => None,
    }
}

fn from_term(value: &str) -> Option<Terminal> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "xterm-ghostty" => Some(Terminal::Ghostty),
        "xterm-kitty" => Some(Terminal::Kitty),
        _ => None,
    }
}
