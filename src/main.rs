// Library
use std::io::Read;

// Modules
mod helpers;

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
    print_hexdump(&buffer);

    Ok(())
}

fn print_hexdump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        // Print the offset
        print!("{:08x}   ", i * 16);

        // Print the hex values
        for (j, byte) in chunk.iter().enumerate() {
            if j > 0 && j % 4 == 0 {
                print!(" ");
            }
            print!("{:02x} ", byte); // Format each byte as a 2-wide hexadecimal value
        }

        // Print spacing if the chunk is less than 16 bytes
        if chunk.len() < 16 {
            for _ in 0..(16 - chunk.len()) {
                print!("   ") // Each missing byte is represented by 3 spaces (two for hex-digits and one space)
            }
        }

        // Print the ASCII representation
        print!("  | ");
        for (k, byte) in chunk.iter().enumerate() {
            if k > 0 && k % 4 == 0 {
                print!(" ");
            }

            if helpers::is_printable_ascii_character(byte) {
                print!("{}", *byte as char);
            } else {
                print!("."); // Non-printable ASCII characters are replaced by a dot
            }
        }
        println!(" | ");
    }
}
