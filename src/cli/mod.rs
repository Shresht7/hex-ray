// Library
use clap::Parser;

// Modules
mod cmd;

// ----------------------
// COMMAND LINE ARGUMENTS
// ----------------------

#[derive(Parser, Clone)]
#[command(version, about)]
pub struct Args {
    /// Subcommand
    #[clap(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(clap::Subcommand, Clone)]
pub enum Command {
    /// Prints the plain output
    View(cmd::View),
    /// View the output in a tabulated format
    Output(cmd::Output),
    /// Run the Terminal User Interface
    Tui(cmd::View),
}
