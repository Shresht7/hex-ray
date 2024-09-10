// Library
use std::io::Read;

// Modules
mod helpers;
mod print;

fn main() -> std::io::Result<()> {
    // Collect the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Buffer to store the input contents
    let mut buffer = Vec::new();

    // If no arguments are passed, read from stdin...
    // ...otherwise, read from the filepath
    if args.len() < 2 {
        std::io::stdin().read_to_end(&mut buffer)?;
    } else {
        // Get the filepath from the arguments
        let filepath = &args[1];

        // Open the file
        let mut file = std::fs::File::open(filepath)?;

        // Read the file contents
        file.read_to_end(&mut buffer)?;
    }

    // Print the hexdump
    print::hexdump(&buffer);

    Ok(())
}
