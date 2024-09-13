// Traits
use clap::Parser;
use std::io::{BufReader, Seek};

// Modules
mod cli;
mod helpers;
mod print;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = cli::Args::parse(); // Parse the command-line arguments
    run(&mut args) // Run the command-line application with the given arguments
}

fn run(args: &mut cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    // If a `filepath` was passed in the arguments, read the file ...
    if let Some(filepath) = &args.filepath {
        let mut file = std::fs::File::open(filepath)?;

        // Apply the offset at which the program starts reading. A positive
        // offset seeks forward from the beginning of the file ...
        if args.offset > 0 {
            file.seek(std::io::SeekFrom::Start(args.offset as u64))?;
        } else if args.offset < 0 {
            // ... while an negative offset seeks backwards from the end of the file
            let file_size = file.seek(std::io::SeekFrom::End(0))?;
            file.seek(std::io::SeekFrom::End(args.offset))?;
            args.offset = file_size as i64 + args.offset;
        }

        let reader = BufReader::new(file);
        args.dump(reader)?;
    } else {
        // ... Otherwise, read the input from STDIN
        args.offset = 0; // Offset is not supported in this mode
        let data = std::io::stdin();
        let reader = BufReader::new(data);
        args.dump(reader)?;
    }

    Ok(())
}
