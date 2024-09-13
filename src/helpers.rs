// -------
// HELPERS
// -------

/// Returns a boolean indicating whether the given byte represents
/// a printable ascii character.
pub fn is_printable_ascii_character(byte: &u8) -> bool {
    byte.is_ascii_graphic() || !byte.is_ascii_whitespace()
}

/// Supported display formats
pub enum Format {
    Hex,
    UpperHex,
    LowerHex,
    Binary,
    Octal,
}

impl Format {
    pub fn format(&self, data: u8) -> String {
        match &self {
            Self::Hex => format!("{:02x}", data),
            Self::UpperHex => format!("{:02X}", data),
            Self::LowerHex => format!("{:02x}", data),
            Self::Binary => format!("{:010b}", data),
            Self::Octal => format!("{:o}", data),
        }
    }
}
