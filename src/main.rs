// Traits
use clap::Parser;
use std::io::{BufReader, Seek};

// Modules
mod helpers;
mod print;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Path to the file to read (defaults to reading from `stdin` if empty)
    #[clap(aliases = ["path", "src"])]
    filepath: Option<std::path::PathBuf>,

    /// The byte offset at which to start reading; i.e. skip the given number of bytes.
    /// You can specify a positive or negative integer value; A positive integer offset
    /// seeks forward from the start, while a negative offset seeks backwards from the end
    #[arg(alias = "skip", short, long, default_value_t = 0)]
    offset: i64,

    /// The number of bytes to read. The program will stop after reading
    /// the specified number of bytes.
    #[arg(short, long)]
    limit: Option<usize>,

    /// The size of each row
    #[arg(short, long, default_value_t = 16)]
    size: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let args = Args::parse();

    // The byte offset at which to start reading the data
    let mut offset = args.offset as usize;

    // If a `filepath` was passed in the arguments, read the file ...
    if let Some(filepath) = args.filepath {
        let mut file = std::fs::File::open(filepath)?;

        // Apply the offset at which the program starts reading. A positive
        // offset seeks forward from the beginning of the file ...
        if args.offset > 0 {
            file.seek(std::io::SeekFrom::Start(args.offset as u64))?;
        } else if args.offset < 0 {
            // ... while an negative offset seeks backwards from the end of the file
            let file_size = file.seek(std::io::SeekFrom::End(0))?;
            offset = (file_size as i64 + args.offset) as usize;
            file.seek(std::io::SeekFrom::End(args.offset))?;
        }

        let reader = BufReader::new(file);
        print::hexdump(reader, offset, args.limit, args.size)?;
    } else {
        // ... Otherwise, read the input from STDIN
        offset = 0; // Offset is not supported in this mode
        let data = std::io::stdin();
        let reader = BufReader::new(data);
        print::hexdump(reader, offset, args.limit, args.size)?;
    }

    Ok(())
}
