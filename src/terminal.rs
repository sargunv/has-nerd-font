use crate::Terminal;

pub enum TerminalDecision {
    Bundled(Terminal),
    Identified(Terminal),
    Unknown,
}

pub fn detect(vars: &[(String, String)]) -> TerminalDecision {
    let var = |key| crate::var(vars, key);

    // Check TERM_PROGRAM for known terminals
    if let Some(value) = var("TERM_PROGRAM")
        && let Some(terminal) = from_term_program(value.trim())
    {
        return decide(terminal);
    }

    // Check TERM for known terminals
    if let Some(value) = var("TERM")
        && let Some(terminal) = from_term(value)
    {
        return decide(terminal);
    }

    // Check terminal-specific env vars
    if var("OPENCODE_TERMINAL") == Some("1") {
        return decide(Terminal::OpenCode);
    }

    if var("CONDUCTOR_WORKSPACE_NAME").is_some_and(|v| !v.is_empty()) {
        return decide(Terminal::Conductor);
    }

    if var("ALACRITTY_LOG").is_some_and(|v| !v.is_empty()) {
        return decide(Terminal::Alacritty);
    }

    // Fall back to Unknown if TERM_PROGRAM was set but unrecognized
    if let Some(value) = var("TERM_PROGRAM") {
        let raw = value.trim();
        if !raw.is_empty() {
            return TerminalDecision::Identified(Terminal::Unknown(raw.to_string()));
        }
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

fn from_term_program(value: &str) -> Option<Terminal> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "ghostty" => Some(Terminal::Ghostty),
        "wezterm" => Some(Terminal::WezTerm),
        "kitty" => Some(Terminal::Kitty),
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
