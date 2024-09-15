// Library
use std::io::Seek;

// -------
// HELPERS
// -------

/// Returns a boolean indicating whether the given byte represents
/// a printable ascii character.
pub fn is_printable_ascii_character(byte: &u8) -> bool {
    byte.is_ascii_graphic() || !byte.is_ascii_whitespace()
}

/// The string representing the NO_COLOR environment variable
const ENV_NO_COLOR: &str = "NO_COLOR";

/// Returns a boolean to indicate whether ANSI Colors are enabled
pub fn color_enabled() -> bool {
    !std::env::var(ENV_NO_COLOR).is_ok_and(|e| e.to_lowercase() == "true")
}

pub fn get_stdin_reader() -> Result<(Box<dyn std::io::BufRead>, usize), Box<dyn std::error::Error>>
{
    let offset = 0; // Offset is not supported in this mode
    let data = std::io::stdin();
    Ok((Box::new(std::io::BufReader::new(data)), offset))
}

pub fn get_file_reader(
    filepath: &std::path::PathBuf,
    mut offset: i64,
) -> Result<(Box<dyn std::io::BufRead>, usize), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(filepath)?;
    // A positive offset seeks forwards from the start of the file
    if offset >= 0 {
        file.seek(std::io::SeekFrom::Start(offset as u64))?;
    } else if offset < 0 {
        // ... while an negative offset seeks backwards from the end of the file
        let file_size = file.seek(std::io::SeekFrom::End(0))?;
        file.seek(std::io::SeekFrom::End(offset))?;
        offset = file_size as i64 + offset;
    }
    Ok((Box::new(std::io::BufReader::new(file)), offset as usize))
}
