// -------
// FORMATS
// -------

/// Supported output formats
#[derive(Clone, Debug, Default)]
pub enum Format {
    #[default]
    Hex,
    HexWithPrefix,
    UpperHex,
    UpperHexWithPrefix,
    Binary,
    BinaryWithPrefix,
    Octal,
    OctalWithPrefix,
    Decimal,
}

// Parse Format enum from a string
impl From<String> for Format {
    fn from(format_string: String) -> Self {
        match format_string.as_ref() {
            "hex" | "x" | "hexadecimal" => Format::Hex,
            "#hex" | "#x" | "#hexadecimal" => Format::HexWithPrefix,
            "HEX" | "X" | "Hex" | "Hexadecimal" => Format::UpperHex,
            "#HEX" | "#X" | "#Hex" | "#Hexadecimal" => Format::UpperHexWithPrefix,
            "binary" | "b" | "bin" => Format::Binary,
            "#binary" | "#b" | "#bin" => Format::BinaryWithPrefix,
            "octal" | "o" | "oct" => Format::Octal,
            "#octal" | "#o" | "#oct" => Format::OctalWithPrefix,
            "decimal" | "d" | "dec" => Format::Decimal,
            _ => panic!("Invalid output format"),
        }
    }
}

impl Format {
    /// Formats the u8 value
    pub fn format(&self, data: u8) -> String {
        match &self {
            Self::Hex => format!("{:02x}", data),                 // e.g. 3f
            Self::HexWithPrefix => format!("{:#04x}", data),      // e.g. 0x3f
            Self::UpperHex => format!("{:02X}", data),            // e.g. 3F
            Self::UpperHexWithPrefix => format!("{:#04X}", data), // e.g. 0x3F
            Self::Binary => format!("{:08b}", data),              // e.g. 00111111
            Self::BinaryWithPrefix => format!("{:#010b}", data),  // e.g. 0b00111111
            Self::Octal => format!("{:03o}", data),               // e.g. 77
            Self::OctalWithPrefix => format!("{:#05o}", data),    // e.g. 0o77
            Self::Decimal => format!("{:03}", data),              // e.g. 63
        }
    }

    /// Returns the size occupied by the format when representing the value
    pub fn size(&self) -> usize {
        match &self {
            Format::Hex => 2,                // e.g. 3f
            Format::HexWithPrefix => 4,      // e.g. 0x3f
            Format::UpperHex => 2,           // e.g. 3F
            Format::UpperHexWithPrefix => 4, // e.g. 0x3F
            Format::Binary => 8,             // e.g. 00111111
            Format::BinaryWithPrefix => 10,  // e.g. 0b00111111
            Format::Octal => 3,              // e.g. 77
            Format::OctalWithPrefix => 5,    // e.g. 0o77
            Format::Decimal => 3,            // e.g. 63
        }
    }
}
