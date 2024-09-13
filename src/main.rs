// Traits
use clap::Parser;
use std::io::{BufReader, Seek};

// Modules
mod cli;
mod helpers;
mod print;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse(); // Parse the command-line arguments
    run(args) // Run the command-line application with the given arguments
}

fn run(args: cli::Args) -> Result<(), Box<dyn std::error::Error>> {
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
        let hex = print::Hex::new(offset, args.limit, args.size, args.group_size);
        hex.dump(reader)?;
    } else {
        // ... Otherwise, read the input from STDIN
        offset = 0; // Offset is not supported in this mode
        let data = std::io::stdin();
        let reader = BufReader::new(data);
        let hex = print::Hex::new(offset, args.limit, args.size, args.group_size);
        hex.dump(reader)?;
    }

    Ok(())
}
