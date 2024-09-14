// -------
// FORMATS
// -------

/// Supported output formats
#[derive(Clone)]
pub enum Format {
    Hex,
    UpperHex,
    Binary,
    Octal,
    Decimal,
}

// Parse Format enum from a string
impl From<String> for Format {
    fn from(format_string: String) -> Self {
        match format_string.as_ref() {
            "hex" => Format::Hex,
            "x" => Format::Hex,
            "HEX" => Format::UpperHex,
            "X" => Format::UpperHex,
            "binary" => Format::Binary,
            "b" => Format::Binary,
            "octal" => Format::Octal,
            "o" => Format::Octal,
            "decimal" => Format::Decimal,
            "d" => Format::Decimal,
            _ => panic!("Invalid output format"),
        }
    }
}

impl Format {
    pub fn format(&self, data: u8) -> String {
        match &self {
            Self::Hex => format!("{:02x}", data),
            Self::UpperHex => format!("{:02X}", data),
            Self::Binary => format!("{:08b}", data),
            Self::Octal => format!("{:03o}", data),
            Self::Decimal => format!("{:03}", data),
        }
    }

    pub fn size(&self) -> usize {
        match &self {
            Format::Hex => 2,
            Format::UpperHex => 2,
            Format::Binary => 8,
            Format::Octal => 3,
            Format::Decimal => 3,
        }
    }
}
