use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

// Library
use crate::utils::format::Format;
use crate::utils::helpers;

#[derive(Debug)]
pub struct Row {
    pub data: Vec<u8>,
    pub offset: usize,
    pub group_size: usize,
    pub bytes_read: usize,
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

    pub fn format_offset(&self) -> Line {
        let res = Format::Octal.format(self.offset as u8);
        if res.len() > 8 {
            let s = format!("{:0>8}", res);
            return Line::from(Span::from(s));
        }

        let mut padding_str = String::from("");
        for _ in 0..(8 - res.len()) {
            padding_str.push_str(&"·");
        }
        let padding = Span::styled(padding_str, Style::default().fg(Color::Gray));

        Line::from(vec![padding, Span::from(res)])
    }

    pub fn format_hex_values(&self, selected: usize) -> String {
        let mut s = String::new();
        // Print the hex values
        for (j, byte) in self.data.iter().take(self.bytes_read).enumerate() {
            // Group values by applying spacing
            if j > 0 && j % self.group_size == 0 {
                s.push_str(" ");
            }
            let mut value = Format::Hex.format(*byte);
            if self.offset + j == selected {
                value = format!("+{}", value)
            }
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
    pub fn format_ascii_representation(&self, selected: usize) -> String {
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
                    let mut char = (*byte as char).to_string();
                    if self.offset + k == selected {
                        char = format!("+{}", char)
                    }
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
