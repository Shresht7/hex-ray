// Library
use crate::utils::format::Format;
use crate::utils::helpers;

#[derive(Debug)]
pub struct Row {
    data: Vec<u8>,
    offset: usize,
    group_size: usize,
    bytes_read: usize,
}

impl Row {
    pub fn parse(data: &[u8], offset: usize, group_size: usize, bytes_read: usize) -> Self {
        Self {
            data: data.to_vec(),
            offset,
            group_size,
            bytes_read,
        }
    }

    pub fn to_string(&self) -> String {
        let offset = self.format_offset();
        let hex_values = self.format_hex_values();
        let ascii_values = self.format_ascii_representation();
        format!("│ {} │ {} │ {} │", offset, hex_values, ascii_values)
    }

    fn format_offset(&self) -> String {
        let res = Format::Octal.format(self.offset as u8);
        if res.len() > 8 {
            return format!("{:0>8}", res);
        }

        let mut padding = String::from(" ");
        for _ in 0..(8 - res.len()) {
            padding.push_str(&"·");
        }

        format!("{}{}", padding, res)
    }

    fn format_hex_values(&self) -> String {
        let mut s = String::new();
        // Print the hex values
        for (j, byte) in self.data.iter().take(self.bytes_read).enumerate() {
            // Group values by applying spacing
            if j > 0 && j % self.group_size == 0 {
                s.push_str(" ");
            }
            let value = Format::Hex.format(*byte);
            s.push_str(&value); // Format each byte as a 2-wide hexadecimal value
            s.push_str(" ");
        }

        // Print spacing if the chunk is less than size bytes
        for k in self.bytes_read..self.data.len() {
            // Group values by applying spacing
            if k > 0 && k % self.group_size == 0 {
                s.push_str(" ");
            }
            // Group values by applying spacing
            s.push_str(&" ".repeat(Format::Hex.size() + 1)); // Each missing byte is represented by 3 spaces (two for hex-digits and one space)
        }

        s
    }

    /// Print the ASCII columns
    fn format_ascii_representation(&self) -> String {
        let mut s = String::new();

        // Print the ASCII representation
        for (k, byte) in self.data.iter().enumerate() {
            // Group characters by applying spacing
            if k > 0 && k % self.group_size == 0 {
                s.push_str(" ");
            }
            // If there are still bytes to read, print the ASCII character...
            if k < self.bytes_read {
                let c = if helpers::is_printable_ascii_character(&byte) {
                    let char = (*byte as char).to_string();
                    format!("{}", char)
                } else {
                    format!("{}", "·") // Non-printable ASCII characters are replaced by a dot
                };
                s.push_str(c.as_str());
            } else {
                s.push_str(" "); // Else if there are no more bytes left in this iteration, just print an empty space
            }
        }

        s
    }
}
