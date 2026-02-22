pub enum EnvDecision {
    OverrideEnabled,
    OverrideDisabled,
    Continue,
}

pub fn detect(vars: &[(String, String)]) -> EnvDecision {
    match crate::var(vars, "NERD_FONT") {
        Some(raw) => parse_nerd_font(raw),
        None => EnvDecision::Continue,
    }
}

fn parse_nerd_font(raw: &str) -> EnvDecision {
    let normalized = raw.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "1" | "true" | "yes" => EnvDecision::OverrideEnabled,
        "0" | "false" | "no" => EnvDecision::OverrideDisabled,
        _ => EnvDecision::Continue,
    }
}
