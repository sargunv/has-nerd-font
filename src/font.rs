use std::sync::LazyLock;

use regex::Regex;

/// Matches font names containing "Nerd Font" / "NerdFont", or the abbreviated
/// suffix convention "NF"/"NFM"/"NFP" preceded or followed by a space or hyphen
/// (e.g. "JetBrainsMonoNFM-Regular", "JetBrainsMono NFM Regular",
/// "MonaspiceNe NF").
static NERD_FONT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Nerd\s?Font|NF[MP]?[\s-]|[\s-]NF[MP]?").unwrap());

pub fn normalize_font_name(font: &str) -> String {
    font.trim().to_string()
}

pub fn is_nerd_font(font: &str) -> bool {
    NERD_FONT_RE.is_match(font.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Full (pretty) family names: "Nerd Font" ---

    #[test]
    fn family_name_nerd_font() {
        assert!(is_nerd_font("JetBrainsMono Nerd Font"));
    }

    #[test]
    fn family_name_nerd_font_mono() {
        assert!(is_nerd_font("JetBrainsMono Nerd Font Mono"));
    }

    #[test]
    fn family_name_nerd_font_propo() {
        assert!(is_nerd_font("JetBrainsMono Nerd Font Propo"));
    }

    #[test]
    fn family_name_nerd_font_with_style() {
        assert!(is_nerd_font("FiraCode Nerd Font Bold"));
    }

    #[test]
    fn family_name_nerd_font_no_space() {
        assert!(is_nerd_font("FiraCode NerdFont"));
    }

    #[test]
    fn family_name_symbols_nerd_font() {
        assert!(is_nerd_font("Symbols Nerd Font"));
    }

    #[test]
    fn family_name_symbols_nerd_font_mono() {
        assert!(is_nerd_font("Symbols Nerd Font Mono"));
    }

    #[test]
    fn family_name_liga_sfmono_nerd_font() {
        assert!(is_nerd_font("Liga SFMono Nerd Font"));
    }

    // --- Full (pretty) names with abbreviated NF/NFM/NFP and spaces ---

    #[test]
    fn full_name_nf_with_style() {
        assert!(is_nerd_font("JetBrainsMono NF Regular"));
    }

    #[test]
    fn full_name_nfm_with_style() {
        assert!(is_nerd_font("JetBrainsMono NFM Bold"));
    }

    #[test]
    fn full_name_nfp_with_style() {
        assert!(is_nerd_font("JetBrainsMono NFP Italic"));
    }

    #[test]
    fn full_name_nf_end_of_string() {
        assert!(is_nerd_font("MonaspiceNe NF"));
    }

    #[test]
    fn full_name_nfm_end_of_string() {
        assert!(is_nerd_font("NotoMono NFM"));
    }

    #[test]
    fn full_name_nfp_end_of_string() {
        assert!(is_nerd_font("ZedMono NFP"));
    }

    #[test]
    fn full_name_nf_complex_style() {
        assert!(is_nerd_font("NotoSans NF Cond ExtBd Italic"));
    }

    #[test]
    fn full_name_nf_extended() {
        assert!(is_nerd_font("ZedMono NF Extd Bold Italic"));
    }

    // --- PostScript names with hyphen ---

    #[test]
    fn postscript_nf_hyphen() {
        assert!(is_nerd_font("JetBrainsMonoNF-Regular"));
    }

    #[test]
    fn postscript_nfm_hyphen() {
        assert!(is_nerd_font("JetBrainsMonoNFM-Regular"));
    }

    #[test]
    fn postscript_nfp_hyphen() {
        assert!(is_nerd_font("JetBrainsMonoNFP-Bold"));
    }

    #[test]
    fn postscript_nf_hyphen_italic() {
        assert!(is_nerd_font("HackNF-BoldItalic"));
    }

    #[test]
    fn postscript_nfm_hyphen_condensed() {
        assert!(is_nerd_font("NotoSansMNFM-CondBold"));
    }

    #[test]
    fn postscript_nfp_hyphen_complex() {
        assert!(is_nerd_font("ZedMonoNFP-ExtdBoldItalic"));
    }

    // --- Non-nerd fonts (should NOT match) ---

    #[test]
    fn non_nerd_arial() {
        assert!(!is_nerd_font("Arial"));
    }

    #[test]
    fn non_nerd_menlo() {
        assert!(!is_nerd_font("Menlo"));
    }

    #[test]
    fn non_nerd_helvetica_neue() {
        assert!(!is_nerd_font("Helvetica Neue"));
    }

    #[test]
    fn non_nerd_sf_pro() {
        assert!(!is_nerd_font("SF Pro"));
    }

    #[test]
    fn non_nerd_monaco() {
        assert!(!is_nerd_font("Monaco"));
    }

    #[test]
    fn non_nerd_courier_new() {
        assert!(!is_nerd_font("Courier New"));
    }

    #[test]
    fn non_nerd_sf_mono() {
        assert!(!is_nerd_font("SF Mono"));
    }

    #[test]
    fn non_nerd_fira_code() {
        assert!(!is_nerd_font("Fira Code"));
    }

    #[test]
    fn non_nerd_jetbrains_mono() {
        assert!(!is_nerd_font("JetBrains Mono"));
    }

    // --- Edge cases: whitespace handling ---

    #[test]
    fn leading_trailing_whitespace() {
        assert!(is_nerd_font("  JetBrainsMono Nerd Font  "));
    }
}
