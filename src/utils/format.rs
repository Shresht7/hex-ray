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
            Self::Octal => format!("{:03o}", data),               // e.g. 077
            Self::OctalWithPrefix => format!("{:#05o}", data),    // e.g. 0o077
            Self::Decimal => format!("{:03}", data),              // e.g. 063
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
            Format::Octal => 3,              // e.g. 077
            Format::OctalWithPrefix => 5,    // e.g. 0o077
            Format::Decimal => 3,            // e.g. 063
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: u8 = 63;

    #[test]
    fn should_format_as_hex() {
        assert_eq!(Format::Hex.format(TEST_INPUT), "3f");
    }

    #[test]
    fn should_format_as_hex_with_prefix() {
        assert_eq!(Format::HexWithPrefix.format(TEST_INPUT), "0x3f");
    }

    #[test]
    fn should_format_as_uppercase_hex() {
        assert_eq!(Format::UpperHex.format(TEST_INPUT), "3F");
    }

    #[test]
    fn should_format_as_uppercase_hex_with_prefix() {
        assert_eq!(Format::UpperHexWithPrefix.format(TEST_INPUT), "0x3F");
    }

    #[test]
    fn should_format_as_binary() {
        assert_eq!(Format::Binary.format(TEST_INPUT), "00111111");
    }

    #[test]
    fn should_format_as_binary_with_prefix() {
        assert_eq!(Format::BinaryWithPrefix.format(TEST_INPUT), "0b00111111");
    }

    #[test]
    fn should_format_as_octal() {
        assert_eq!(Format::Octal.format(TEST_INPUT), "077");
    }

    #[test]
    fn should_format_as_octal_with_prefix() {
        assert_eq!(Format::OctalWithPrefix.format(TEST_INPUT), "0o077");
    }

    #[test]
    fn should_format_as_decimal() {
        assert_eq!(Format::Decimal.format(TEST_INPUT), "063");
    }

    #[test]
    fn should_have_appropriate_size() {
        assert_eq!(Format::Hex.size(), "3f".len());
        assert_eq!(Format::HexWithPrefix.size(), "0x3f".len());
        assert_eq!(Format::UpperHex.size(), "3F".len());
        assert_eq!(Format::UpperHexWithPrefix.size(), "0x3F".len());
        assert_eq!(Format::Binary.size(), "00111111".len());
        assert_eq!(Format::BinaryWithPrefix.size(), "0b00111111".len());
        assert_eq!(Format::Octal.size(), "077".len());
        assert_eq!(Format::OctalWithPrefix.size(), "0o077".len());
        assert_eq!(Format::Decimal.size(), "063".len());
    }
}
