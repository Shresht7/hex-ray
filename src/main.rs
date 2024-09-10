// Library
use clap::Parser;
use std::{
    io::{Read, Seek},
    path::PathBuf,
};

// Modules
mod helpers;
mod print;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the file to read (defaults to reading from `stdin` if empty)
    #[clap(aliases =["path", "src"])]
    filepath: Option<PathBuf>,

    /// The byte offset at which to start reading; i.e. skip the given number of bytes. (default: 0)
    #[arg(alias = "skip", short, long, default_value_t = 0)]
    offset: i64,
}

fn main() -> std::io::Result<()> {
    // Collect the command-line arguments
    let args = Args::parse();

    // Buffer to store the input contents
    let mut buffer = Vec::new();

    // If a `filepath` was passed in the arguments, read the file
    // otherwise, read the input from stdin
    if let Some(filepath) = args.filepath {
        let mut file = std::fs::File::open(filepath)?;

        // If an offset is provided, seek to the given position
        if args.offset > 0 {
            file.seek(std::io::SeekFrom::Start(args.offset as u64))?;
        }

        file.read_to_end(&mut buffer)?;
    } else {
        std::io::stdin().read_to_end(&mut buffer)?;
    }

    // Print the hexdump
    print::hexdump(&buffer);

    Ok(())
}
