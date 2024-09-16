// Library
use crate::utils::{
    ansi::{Color, Colorable},
    format::Format,
    helpers,
};
use clap::Parser;

// ------------
// VIEW COMMAND
// ------------

#[derive(Parser, Clone, Debug, Default)]
#[command(version, about)]
pub struct View {
    /// Path to the file to read (defaults to reading from `stdin` if empty)
    #[clap(aliases = ["path", "src"])]
    pub filepath: Option<std::path::PathBuf>,

    /// The byte offset at which to start reading; i.e. skip the given number of bytes.
    ///
    /// You can specify a positive or negative integer value; A positive integer offset
    /// seeks forward from the start, while a negative offset seeks backwards from the end
    #[arg(aliases = ["skip", "seek"], short, long, default_value_t = 0)]
    pub offset: i64,

    /// The number of bytes to read.
    ///
    /// The program will stop after reading the specified number of bytes.
    #[arg(short, long)]
    pub limit: Option<usize>,

    /// The size of each row
    #[arg(short, long, default_value_t = 16)]
    pub size: usize,

    /// The output display format.
    ///
    /// This can be one of the following: hex (x), HEX (X), binary (b), octal (o), decimal (d).
    ///
    /// To output with the corresponding prefixes prepend a `#` to the format (e.g. `#hex` or `#x`)
    #[arg(short, long, default_value = "hex")]
    pub format: Format,

    /// Chunk the output into groups of this size
    #[arg(alias = "chunk", short, long, default_value_t = 4)]
    pub group_size: usize,

    /// Disable ANSI colors
    #[arg(short, long)]
    pub no_color: bool,

    /// Simple Output
    #[arg(alias = "plain", short = 'p', long)]
    pub simple: bool,
}

impl View {
    /// Perform initialization setup
    pub fn init(&self) -> &Self {
        // Disable ANSI colors by setting the `NO_COLOR` env variable
        if self.no_color || self.simple {
            std::env::set_var("NO_COLOR", "true");
        }
        self
    }

    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        self.init();
        let (reader, offset) = helpers::get_reader_and_offset(self.filepath.as_ref(), self.offset)?;
        Ok(self.dump(reader, offset)?)
    }

    /// Print out the hex-dump of the given byte data
    fn dump<T>(&self, mut data: T, offset: usize) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::Read,
    {
        // Buffer to store the data
        let mut buffer = vec![0; self.size];

        // The total number of bytes read already
        let mut total_bytes_read = 0;
        // The number of bytes remaining to be read
        let mut bytes_remaining = self.limit.unwrap_or(usize::MAX);

        self.print_file_name();
        self.print_top_line();

        while bytes_remaining > 0 {
            // Determine the number of bytes to be read in this iteration
            let bytes_to_read = std::cmp::min(bytes_remaining, self.size);

            let bytes_read = data.read(&mut buffer[0..bytes_to_read])?;
            if bytes_read == 0 {
                break;
            }

            self.print_line(&buffer, bytes_read, offset + total_bytes_read);
            total_bytes_read += bytes_read;
            bytes_remaining -= bytes_read;
        }

        self.print_bottom_line();
        self.print_total(total_bytes_read);

        Ok(())
    }

    fn print_file_name(&self) {
        if !self.simple {
            if let Some(filepath) = &self.filepath {
                println!("Source: {}", filepath.to_string_lossy())
            } else {
                println!("Source: STDIN");
            }
        }
    }

    fn print_top_line(&self) {
        if self.simple {
            return;
        }

        let mut line = String::from("┌─");
        line.push_str(&"─".repeat(8 + 2));
        line.push_str("┬─");

        for i in 0..self.size {
            if i > 0 && i % self.group_size == 0 {
                line.push_str("─");
            }
            line.push_str(&"─".repeat(self.format.size() + 1));
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
    pub fn print_line(&self, buffer: &[u8], bytes_read: usize, total_bytes_read: usize) {
        let offset = self.format_offset(total_bytes_read);
        let hex_values = self.format_hex_values(&buffer, bytes_read);
        let ascii_values = self.format_ascii_representation(&buffer, bytes_read);
        if self.simple {
            println!("{}:  {}  | {}", offset, hex_values, ascii_values);
        } else {
            println!("│ {} │ {} │ {} │", offset, hex_values, ascii_values);
        }
    }

    /// Print the offset column
    fn format_offset(&self, offset: usize) -> String {
        let res = Format::Octal.format(offset as u8);
        if res.len() > 8 || self.simple {
            return format!("{:0>8}", res);
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
        for k in bytes_read..chunk.len() {
            // Group values by applying spacing
            if k > 0 && k % self.group_size == 0 {
                s.push_str(" ");
            }

            s.push_str(&" ".repeat(self.format.size() + 1)); // Each missing byte is represented by 3 spaces (two for hex-digits and one space)
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
        if self.simple {
            return;
        }

        let mut line = String::from("└─");
        line.push_str(&"─".repeat(8 + 2));
        line.push_str("┴─");

        for i in 0..self.size {
            if i > 0 && i % self.group_size == 0 {
                line.push_str("─");
            }
            line.push_str(&"─".repeat(self.format.size() + 1));
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

    fn print_total(&self, n: usize) {
        if self.simple {
            return;
        }
        println!("Read {} bytes\n", n);
    }
}
