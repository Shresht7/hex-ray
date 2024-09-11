// Library
use crate::helpers;

/// Print out the hex-dump of the given byte data
pub fn hexdump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print_offset(i);
        print_hex_values(chunk);
        print_ascii_representation(chunk);
    }
}

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
