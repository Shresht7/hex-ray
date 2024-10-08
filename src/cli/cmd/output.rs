// Library
use crate::utils::format::Format;
use crate::utils::helpers;
use clap::Parser;

// --------------
// OUTPUT COMMAND
// --------------

#[derive(Parser, Clone)]
#[command(version, about)]
pub struct Output {
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

    /// The output display format.
    ///
    /// This can be one of the following: hex (x), HEX (X), binary (b), octal (o), decimal (d).
    ///
    /// To output with the corresponding prefixes prepend a `#` to the format (e.g. `#hex` or `#x`)
    #[arg(short, long, default_value = "hex")]
    pub format: Format,

    /// Character to separate the output
    #[arg(short, long, default_value_t = String::from(" "))]
    pub separator: String,
}

impl Output {
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        let (reader, _) = helpers::get_reader_and_offset(self.filepath.as_ref(), self.offset)?;
        Ok(self.dump(reader)?)
    }

    fn dump<T>(&self, mut data: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: std::io::BufRead,
    {
        // Buffer to store the data
        let mut buffer = vec![0; 16];
        // The number of bytes remaining to be read
        let mut bytes_remaining = self.limit.unwrap_or(usize::MAX);

        while bytes_remaining > 0 {
            // Determine the number of bytes to be read in this iteration
            let bytes_to_read = std::cmp::min(bytes_remaining, 16);

            let bytes_read = data.read(&mut buffer[0..bytes_to_read])?;
            if bytes_read == 0 {
                break;
            }

            buffer.iter().take(bytes_read).for_each(|b| {
                let s = self.format.format(*b);
                print!("{}{}", s, self.separator)
            });

            bytes_remaining -= bytes_read;
        }
        Ok(())
    }
}
