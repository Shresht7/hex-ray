// -------
// HELPERS
// -------

/// Returns a boolean indicating whether the given byte represents
/// a printable ascii character.
pub fn is_printable_ascii_character(byte: &u8) -> bool {
    byte.is_ascii_graphic() || !byte.is_ascii_whitespace()
}
