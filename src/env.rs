pub enum EnvDecision {
    OverrideEnabled,
    OverrideDisabled,
    Continue,
}

pub fn detect(vars: &[(String, String)]) -> EnvDecision {
    let Some(raw) = vars
        .iter()
        .find_map(|(key, value)| (key == "NERD_FONT").then_some(value))
    else {
        return EnvDecision::Continue;
    };

    let normalized = raw.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "1" | "true" | "yes" => EnvDecision::OverrideEnabled,
        "0" | "false" | "no" => EnvDecision::OverrideDisabled,
        _ => EnvDecision::Continue,
    }
}
