use std::usize;

// Library
use crate::helpers;

/// Print out the hex-dump of the given byte data
pub fn hexdump<T>(mut data: T)
where
    T: std::io::Read,
{
    // Buffer to store the data
    let mut buffer = vec![0; 16];

    // The number of bytes read already
    let mut bytes_read = 0;
    // The number of bytes remaining to be read
    let mut bytes_remaining = usize::MAX;

    loop {
        // Determine the number of bytes to be read in this iteration

        match data.read(&mut buffer[0..16]) {
            Ok(n) => {
                if n > 0 {
                    print_offset(bytes_read);
                    print_hex_values(&buffer);
                    print_ascii_representation(&buffer);
                    bytes_read += n;
                    bytes_remaining -= n;
                } else {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

// -------
// HELPERS
// -------

/// Print the offset column
fn print_offset(i: usize) {
    print!("{:08x}   ", i * 16);
}

/// Print the hex-values columns
fn print_hex_values(chunk: &[u8]) {
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
}

/// Print the ASCII columns
fn print_ascii_representation(chunk: &[u8]) {
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
