// Library
use crate::{format, helpers};
use clap::Parser;

// ----------------------
// COMMAND LINE ARGUMENTS
// ----------------------

#[derive(Parser, Clone)]
#[command(version, about)]
pub struct Args {
    /// Path to the file to read (defaults to reading from `stdin` if empty)
    #[clap(aliases = ["path", "src"])]
    pub filepath: Option<std::path::PathBuf>,

    /// The byte offset at which to start reading; i.e. skip the given number of bytes.
    ///
    /// You can specify a positive or negative integer value; A positive integer offset
    /// seeks forward from the start, while a negative offset seeks backwards from the end
    #[arg(aliases = ["skip", "seek"], short, long, default_value_t = 0)]
    pub offset: i64,

    /// The number of bytes to read.
    ///
    /// The program will stop after reading the specified number of bytes.
    #[arg(short, long)]
    pub limit: Option<usize>,

    /// The size of each row
    #[arg(short, long, default_value_t = 16)]
    pub size: usize,

    /// The output display format.
    ///
    /// This can be one of the following: [hex, x] [HEX, X] [binary, b] [octal, o] [decimal, d]
    #[arg(short, long, default_value = "hex")]
    pub format: format::Format,

    /// Chunk the output into groups of this size
    #[arg(alias = "chunk", short, long, default_value_t = 4)]
    pub group_size: usize,

    /// Disable ANSI colors
    #[arg(short, long)]
    pub no_color: bool,
}

// ----------
// ANSI CODES
// ----------

/// ANSI escape codes for text formatting
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default = 39,
    BgBlack = 40,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    BgDefault = 49,
    BrightBlack = 90,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    BrightDefault = 99,
    BgBrightBlack = 100,
    BgBrightRed,
    BgBrightGreen,
    BgBrightYellow,
    BgBrightBlue,
    BgBrightMagenta,
    BgBrightCyan,
    BgBrightWhite,
    BgBrightDefault = 109,
}

pub trait Colorable {
    fn ansi(&self, color: Color) -> String;
}

impl Colorable for &'static str {
    fn ansi(&self, code: Color) -> String {
        if helpers::color_enabled() {
            format!("\u{001b}[{}m{}\u{001b}[0m", code as u8, &self)
        } else {
            self.to_string()
        }
    }
}

impl Colorable for String {
    fn ansi(&self, code: Color) -> String {
        if helpers::color_enabled() {
            format!("\u{001b}[{}m{}\u{001b}[0m", code as u8, &self)
        } else {
            self.to_string()
        }
    }
}
