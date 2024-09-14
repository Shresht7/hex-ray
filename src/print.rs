// Library
use crate::cli::Args;
use crate::cli::{Color, Colorable};
use crate::format::Format;
use crate::helpers;

impl Args {
    /// Print out the hex-dump of the given byte data
    pub fn dump<T>(&self, mut data: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::Read,
    {
        // Buffer to store the data
        let mut buffer = vec![0; self.size];

        // The total number of bytes read already
        let mut total_bytes_read = 0;
        // The number of bytes remaining to be read
        let mut bytes_remaining = self.limit.unwrap_or(usize::MAX);

        self.print_top_line();

        while bytes_remaining > 0 {
            // Determine the number of bytes to be read in this iteration
            let bytes_to_read = std::cmp::min(bytes_remaining, self.size);

            let bytes_read = data.read(&mut buffer[0..bytes_to_read])?;
            if bytes_read == 0 {
                break;
            }

            self.print_line(&buffer, bytes_read, self.offset as usize + total_bytes_read);
            total_bytes_read += bytes_read;
            bytes_remaining -= bytes_read;
        }

        self.print_bottom_line();

        Ok(())
    }

    fn print_top_line(&self) {
        let mut line = String::from("┌───────────┬");

        for i in 0..self.size {
            if i > 0 && i % 4 == 0 {
                line.push_str("──");
            }
            line.push_str("───");
        }

        line.push_str("─┬─");

        for i in 0..self.size {
            if i > 0 && i % self.group_size == 0 {
                line.push_str("─");
            }
            line.push_str("─");
        }

        line.push_str("─┐");
        println!("{}", line);
    }

    /// Prints a row in the hexdump table
    fn print_line(&self, buffer: &[u8], bytes_read: usize, total_bytes_read: usize) {
        let offset = self.format_offset(total_bytes_read);
        let hex_values = self.format_hex_values(&buffer, bytes_read);
        let ascii_values = self.format_ascii_representation(&buffer, bytes_read);
        println!("│ {} │  {}  │ {} │", offset, hex_values, ascii_values);
    }

    /// Print the offset column
    fn format_offset(&self, offset: usize) -> String {
        let res = Format::Octal.format(offset as u8);
        if res.len() > 8 {
            return format!("{}", res);
        }

        let mut padding = String::from(" ");
        for _ in 0..(8 - res.len()) {
            padding.push_str(&"·".ansi(Color::Black));
        }

        format!("{}{}", padding, res.ansi(Color::White))
    }

    /// Print the hex-values columns
    fn format_hex_values(&self, chunk: &[u8], bytes_read: usize) -> String {
        let mut s = String::new();
        // Print the hex values
        for (j, byte) in chunk.iter().take(bytes_read).enumerate() {
            // Group values by applying spacing
            if j > 0 && j % self.group_size == 0 {
                s.push_str(" ");
            }
            let value = self.format.format(*byte);
            s.push_str(&value.ansi(Color::White)); // Format each byte as a 2-wide hexadecimal value
            s.push_str(" ");
        }

        // Print spacing if the chunk is less than size bytes
        for _ in bytes_read..chunk.len() {
            s.push_str("   "); // Each missing byte is represented by 3 spaces (two for hex-digits and one space)
        }

        s
    }

    /// Print the ASCII columns
    fn format_ascii_representation(&self, chunk: &[u8], bytes_read: usize) -> String {
        let mut s = String::new();

        // Print the ASCII representation
        for (k, byte) in chunk.iter().enumerate() {
            // Group characters by applying spacing
            if k > 0 && k % self.group_size == 0 {
                s.push_str(" ");
            }

            // If there are still bytes to read, print the ASCII character...
            if k < bytes_read {
                let c = if helpers::is_printable_ascii_character(&byte) {
                    let char = (*byte as char).to_string();
                    format!("{}", char.ansi(Color::White))
                } else {
                    format!("{}", "·".ansi(Color::Black)) // Non-printable ASCII characters are replaced by a dot
                };
                s.push_str(c.as_str());
            } else {
                s.push_str(" "); // Else if there are no more bytes left in this iteration, just print an empty space
            }
        }

        s
    }

    fn print_bottom_line(&self) {
        let mut line = String::from("└───────────┴");

        for i in 0..self.size {
            if i > 0 && i % self.group_size == 0 {
                line.push_str("──");
            }
            line.push_str("───");
        }

        line.push_str("─┴─");

        for i in 0..self.size {
            if i > 0 && i % self.group_size == 0 {
                line.push_str("─");
            }
            line.push_str("─");
        }

        line.push_str("─┘");
        println!("{}", line);
    }
}
