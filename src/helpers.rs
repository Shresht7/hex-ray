// -------
// HELPERS
// -------

/// Returns a boolean indicating whether the given byte represents
/// a printable ascii character.
pub fn is_printable_ascii_character(byte: &u8) -> bool {
    byte.is_ascii_graphic() || !byte.is_ascii_whitespace()
}

/// The string representing the NO_COLOR environment variable
const ENV_NO_COLOR: &str = "NO_COLOR";

/// Returns a boolean to indicate whether ANSI Colors are enabled
pub fn color_enabled() -> bool {
    !std::env::var(ENV_NO_COLOR).is_ok_and(|e| e.to_lowercase() == "true")
}
