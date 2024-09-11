use std::usize;

// Library
use crate::helpers;

/// Print out the hex-dump of the given byte data
pub fn hexdump<T>(mut data: T, offset: usize, limit: Option<usize>, size: usize)
where
    T: std::io::Read,
{
    // Buffer to store the data
    let mut buffer = vec![0; size];

    // The total number of bytes read already
    let mut total_bytes_read = 0;
    // The number of bytes remaining to be read
    let mut bytes_remaining = match limit {
        Some(n) => n,
        None => usize::MAX,
    };

    loop {
        // Determine the number of bytes to be read in this iteration
        let bytes_to_read = if bytes_remaining < size {
            bytes_remaining
        } else {
            size
        };

        match data.read(&mut buffer[0..bytes_to_read]) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    print_line(&buffer, size, bytes_read, offset + total_bytes_read);
                    total_bytes_read += bytes_read;
                    bytes_remaining -= bytes_read;
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

/// Prints a row in the hexdump table
fn print_line(buffer: &[u8], size: usize, bytes_read: usize, total_bytes_read: usize) {
    print_offset(total_bytes_read);
    print_hex_values(&buffer, size);
    print_ascii_representation(&buffer, bytes_read);
}

/// Print the offset column
fn print_offset(offset: usize) {
    let res = format!("{}", offset);
    if res.len() > 8 {
        print!("{}", res);
        return;
    }

    let mut padding = String::from(" ");
    for _ in 0..(8 - res.len()) {
        padding.push_str(".");
    }

    print!("{}{} |  ", padding, res);
}

/// Print the hex-values columns
fn print_hex_values(chunk: &[u8], size: usize) {
    // Print the hex values
    for (j, byte) in chunk.iter().enumerate() {
        // Group values by applying spacing
        if j > 0 && j % 4 == 0 {
            print!(" ");
        }
        print!("{:02x} ", byte); // Format each byte as a 2-wide hexadecimal value
    }

    // Print spacing if the chunk is less than size bytes
    if chunk.len() < size {
        for _ in 0..(size - chunk.len()) {
            print!("   ") // Each missing byte is represented by 3 spaces (two for hex-digits and one space)
        }
    }
}

/// Print the ASCII columns
fn print_ascii_representation(chunk: &[u8], bytes_read: usize) {
    print!("  | ");
    // Print the ASCII representation
    for (k, byte) in chunk.iter().enumerate() {
        // Group characters by applying spacing
        if k > 0 && k % 4 == 0 {
            print!(" ");
        }

        // If there are still bytes to read, print the ASCII character...
        if k < bytes_read {
            if helpers::is_printable_ascii_character(&byte) {
                print!("{}", *byte as char);
            } else {
                print!("."); // Non-printable ASCII characters are replaced by a dot
            }
        } else {
            print!(" "); // Else if there are no more bytes left in this iteration, just print an empty space
        }
    }
    println!(" |");
}
