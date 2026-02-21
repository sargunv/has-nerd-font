pub fn normalize_font_name(font: &str) -> String {
    font.trim().to_string()
}

pub fn is_nerd_font(font: &str) -> bool {
    let normalized = normalize_font_name(font);
    if normalized.contains("Nerd Font") || normalized.contains("NerdFont") {
        return true;
    }

    normalized
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|token| {
            ["NF", "NFM", "NFP"]
                .iter()
                .any(|suffix| token.ends_with(suffix))
        })
}
