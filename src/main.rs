// Library
use clap::Parser;
use std::path::PathBuf;

// Modules
mod helpers;
mod print;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the file to read (defaults to reading from `stdin` if empty)
    #[clap(aliases =["path", "src"])]
    filepath: Option<PathBuf>,

    /// The byte offset at which to start reading; i.e. skip the given number of bytes
    #[arg(alias = "skip", short, long, default_value_t = 0)]
    offset: i64,
}

fn main() -> std::io::Result<()> {
    // Collect the command-line arguments
    let args = Args::parse();

    // If a `filepath` was passed in the arguments, read the file
    // otherwise, read the input from stdin
    if let Some(filepath) = args.filepath {
        let file = std::fs::File::open(filepath)?;
        print::hexdump(file);
    } else {
        let data = std::io::stdin();
        print::hexdump(data);
    }

    Ok(())
}
