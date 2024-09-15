// Library
use super::helpers;

// ----------
// ANSI CODES
// ----------

/// ANSI escape codes for text formatting
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 30,
    // Red,
    // Green,
    // Yellow,
    // Blue,
    // Magenta,
    // Cyan,
    White,
    // Default = 39,
    // BgBlack = 40,
    // BgRed,
    // BgGreen,
    // BgYellow,
    // BgBlue,
    // BgMagenta,
    // BgCyan,
    // BgWhite,
    // BgDefault = 49,
    // BrightBlack = 90,
    // BrightRed,
    // BrightGreen,
    // BrightYellow,
    // BrightBlue,
    // BrightMagenta,
    // BrightCyan,
    // BrightWhite,
    // BrightDefault = 99,
    // BgBrightBlack = 100,
    // BgBrightRed,
    // BgBrightGreen,
    // BgBrightYellow,
    // BgBrightBlue,
    // BgBrightMagenta,
    // BgBrightCyan,
    // BgBrightWhite,
    // BgBrightDefault = 109,
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
