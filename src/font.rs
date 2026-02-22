use std::sync::LazyLock;

use regex::Regex;

/// Matches font names containing "Nerd Font" / "NerdFont", or the abbreviated
/// suffix convention "NF"/"NFM"/"NFP" followed by a hyphen (e.g.
/// "JetBrainsMonoNFM-Regular").
static NERD_FONT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Nerd\s?Font|NF[MP]?-").unwrap());

pub fn normalize_font_name(font: &str) -> String {
    font.trim().to_string()
}

pub fn is_nerd_font(font: &str) -> bool {
    NERD_FONT_RE.is_match(font.trim())
}
