// Library
use crate::helpers;

pub struct Hex {
    offset: usize,
    limit: Option<usize>,
    size: usize,
}

impl Hex {
    /// Instantiate a new Hex
    pub fn new(offset: usize, limit: Option<usize>, size: usize) -> Self {
        Self {
            offset,
            limit,
            size,
        }
    }

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

        loop {
            // Determine the number of bytes to be read in this iteration
            let bytes_to_read = std::cmp::min(bytes_remaining, self.size);

            let bytes_read = data.read(&mut buffer[0..bytes_to_read])?;
            if bytes_read > 0 {
                self.print_line(&buffer, bytes_read, self.offset + total_bytes_read);
                total_bytes_read += bytes_read;
                bytes_remaining -= bytes_read;
            } else {
                break;
            }
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
            if i > 0 && i % 4 == 0 {
                line.push_str("─");
            }
            line.push_str("─");
        }

        line.push_str("─┐");
        println!("{}", line);
    }

    /// Prints a row in the hexdump table
    fn print_line(&self, buffer: &[u8], bytes_read: usize, total_bytes_read: usize) {
        print!("│ ");
        self.print_offset(total_bytes_read);
        print!(" │  ");
        self.print_hex_values(&buffer, bytes_read);
        print!("  │ ");
        self.print_ascii_representation(&buffer, bytes_read);
        println!(" │");
    }

    /// Print the offset column
    fn print_offset(&self, offset: usize) {
        let res = format!("{}", offset);
        if res.len() > 8 {
            print!("{}", res);
            return;
        }

        let mut padding = String::from(" ");
        for _ in 0..(8 - res.len()) {
            padding.push_str("·");
        }

        print!("{}{}", padding, res);
    }

    /// Print the hex-values columns
    fn print_hex_values(&self, chunk: &[u8], bytes_read: usize) {
        // Print the hex values
        for (j, byte) in chunk.iter().take(bytes_read).enumerate() {
            // Group values by applying spacing
            if j > 0 && j % 4 == 0 {
                print!(" ");
            }
            print!("{:02x} ", byte); // Format each byte as a 2-wide hexadecimal value
        }

        // Print spacing if the chunk is less than size bytes
        for _ in bytes_read..chunk.len() {
            print!("   "); // Each missing byte is represented by 3 spaces (two for hex-digits and one space)
        }
    }

    /// Print the ASCII columns
    fn print_ascii_representation(&self, chunk: &[u8], bytes_read: usize) {
        // Print the ASCII representation
        for (k, byte) in chunk.iter().enumerate() {
            // Group characters by applying spacing
            if k > 0 && k % 4 == 0 {
                print!(" ");
            }

            // If there are still bytes to read, print the ASCII character...
            if k < bytes_read {
                if helpers::is_printable_ascii_character(&byte) {
                    print!("{}", *byte as char);
                } else {
                    print!("·"); // Non-printable ASCII characters are replaced by a dot
                }
            } else {
                print!(" "); // Else if there are no more bytes left in this iteration, just print an empty space
            }
        }
    }

    fn print_bottom_line(&self) {
        let mut line = String::from("└───────────┴");

        for i in 0..self.size {
            if i > 0 && i % 4 == 0 {
                line.push_str("──");
            }
            line.push_str("───");
        }

        line.push_str("─┴─");

        for i in 0..self.size {
            if i > 0 && i % 4 == 0 {
                line.push_str("─");
            }
            line.push_str("─");
        }

        line.push_str("─┘");
        println!("{}", line);
    }
}
