// Library
use clap::Parser;

// -----------
// TUI COMMAND
// -----------

#[derive(Parser, Clone)]
#[command(version, about)]
pub struct Tui {}

impl Tui {
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("tui");
        Ok(())
    }
}
