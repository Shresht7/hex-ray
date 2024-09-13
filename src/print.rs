// Library
use crate::helpers;

/// Print out the hex-dump of the given byte data
pub fn hexdump<T>(
    mut data: T,
    offset: usize,
    limit: Option<usize>,
    size: usize,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: std::io::Read,
{
    // Buffer to store the data
    let mut buffer = vec![0; size];

    // The total number of bytes read already
    let mut total_bytes_read = 0;
    // The number of bytes remaining to be read
    let mut bytes_remaining = limit.unwrap_or(usize::MAX);

    print_top_line(size);

    loop {
        // Determine the number of bytes to be read in this iteration
        let bytes_to_read = std::cmp::min(bytes_remaining, size);

        let bytes_read = data.read(&mut buffer[0..bytes_to_read])?;
        if bytes_read > 0 {
            print_line(&buffer, bytes_read, offset + total_bytes_read);
            total_bytes_read += bytes_read;
            bytes_remaining -= bytes_read;
        } else {
            break;
        }
    }

    print_bottom_line(size);

    Ok(())
}

// -------
// HELPERS
// -------

fn print_top_line(size: usize) {
    let mut line = String::from("┌───────────┬");

    for i in 0..size {
        if i > 0 && i % 4 == 0 {
            line.push_str("──");
        }
        line.push_str("───");
    }

    line.push_str("─┬─");

    for i in 0..size {
        if i > 0 && i % 4 == 0 {
            line.push_str("─");
        }
        line.push_str("─");
    }

    line.push_str("─┐");
    println!("{}", line);
}

/// Prints a row in the hexdump table
fn print_line(buffer: &[u8], bytes_read: usize, total_bytes_read: usize) {
    print!("│ ");
    print_offset(total_bytes_read);
    print!(" │  ");
    print_hex_values(&buffer, bytes_read);
    print!("  │ ");
    print_ascii_representation(&buffer, bytes_read);
    println!(" │");
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
        padding.push_str("·");
    }

    print!("{}{}", padding, res);
}

/// Print the hex-values columns
fn print_hex_values(chunk: &[u8], bytes_read: usize) {
    // Print the hex values
    for (j, byte) in chunk.iter().take(bytes_read).enumerate() {
        // Group values by applying spacing
        if j > 0 && j % 4 == 0 {
            print!(" ");
        }
        print!("{:02x} ", byte); // Format each byte as a 2-wide hexadecimal value
    }

    // Print spacing if the chunk is less than size bytes
    for _ in bytes_read..chunk.len() {
        print!("   "); // Each missing byte is represented by 3 spaces (two for hex-digits and one space)
    }
}

/// Print the ASCII columns
fn print_ascii_representation(chunk: &[u8], bytes_read: usize) {
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
                print!("·"); // Non-printable ASCII characters are replaced by a dot
            }
        } else {
            print!(" "); // Else if there are no more bytes left in this iteration, just print an empty space
        }
    }
}

fn print_bottom_line(size: usize) {
    let mut line = String::from("└───────────┴");

    for i in 0..size {
        if i > 0 && i % 4 == 0 {
            line.push_str("──");
        }
        line.push_str("───");
    }

    line.push_str("─┴─");

    for i in 0..size {
        if i > 0 && i % 4 == 0 {
            line.push_str("─");
        }
        line.push_str("─");
    }

    line.push_str("─┘");
    println!("{}", line);
}
