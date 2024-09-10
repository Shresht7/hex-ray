// Library
use clap::Parser;
use std::{io::Read, path::PathBuf};

// Modules
mod helpers;
mod print;

#[derive(Parser)]
#[command(version)]
struct Args {
    // Path to the file to read
    filepath: Option<PathBuf>,
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
        file.read_to_end(&mut buffer)?;
    } else {
        std::io::stdin().read_to_end(&mut buffer)?;
    }

    // Print the hexdump
    print::hexdump(&buffer);

    Ok(())
}
