// Library
use clap::Parser;
use std::io::Seek;
use std::path::PathBuf;

// Modules
mod helpers;
mod print;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the file to read (defaults to reading from `stdin` if empty)
    #[clap(aliases = ["path", "src"])]
    filepath: Option<PathBuf>,

    /// The byte offset at which to start reading; i.e. skip the given number of bytes.
    /// You can specify a positive or negative integer value; A positive integer offset
    /// seeks forward from the start, while a negative offset seeks backwards from the end
    #[arg(alias = "skip", short, long, default_value_t = 0)]
    offset: i64,
}

fn main() -> Result<(), std::io::Error> {
    // Collect the command-line arguments
    let args = Args::parse();

    // The byte offset at which to start reading
    let mut offset = args.offset as usize;

    // If a `filepath` was passed in the arguments, read the file
    // otherwise, read the input from stdin
    if let Some(filepath) = args.filepath {
        let mut file = std::fs::File::open(filepath)?;

        // Apply the offset at which the program starts reading
        if args.offset > 0 {
            file.seek(std::io::SeekFrom::Start(args.offset as u64))?;
        } else if args.offset < 0 {
            let file_size = file.seek(std::io::SeekFrom::End(0))?;
            offset = (file_size as i64 + args.offset) as usize;
            file.seek(std::io::SeekFrom::End(args.offset))?;
        }

        print::hexdump(file, offset);
    } else {
        let data = std::io::stdin();
        print::hexdump(data, offset);
    }

    Ok(())
}
