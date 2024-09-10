// Library
use std::io::Read;

// Modules
mod helpers;
mod print;

fn main() -> std::io::Result<()> {
    // Collect the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Exit if incorrect number of arguments are passed
    if args.len() < 2 {
        eprintln!("Usage: hex-ray <file-path>");
        std::process::exit(1);
    }

    // Get the filepath from the arguments
    let filepath = &args[1];

    // Open the file
    let mut file = std::fs::File::open(filepath)?;

    // Read the file contents
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Print the hexdump
    print::hexdump(&buffer);

    Ok(())
}
