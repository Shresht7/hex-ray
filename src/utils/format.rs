// -------
// FORMATS
// -------

/// Supported output formats
#[derive(Clone)]
pub enum Format {
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
    pub fn format(&self, data: u8) -> String {
        match &self {
            Self::Hex => format!("{:02x}", data),
            Self::HexWithPrefix => format!("{:#04x}", data),
            Self::UpperHex => format!("{:02X}", data),
            Self::UpperHexWithPrefix => format!("{:#04X}", data),
            Self::Binary => format!("{:08b}", data),
            Self::BinaryWithPrefix => format!("{:#010b}", data),
            Self::Octal => format!("{:03o}", data),
            Self::OctalWithPrefix => format!("{:#05o}", data),
            Self::Decimal => format!("{:03}", data),
        }
    }

    pub fn size(&self) -> usize {
        match &self {
            Format::Hex => 2,
            Format::HexWithPrefix => 4,
            Format::UpperHex => 2,
            Format::UpperHexWithPrefix => 4,
            Format::Binary => 8,
            Format::BinaryWithPrefix => 10,
            Format::Octal => 3,
            Format::OctalWithPrefix => 5,
            Format::Decimal => 3,
        }
    }
}
