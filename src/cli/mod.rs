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
    /// Prints the styled output
    View(cmd::View),
    /// Outputs only the values
    Output(cmd::Output),
    /// View using an interactive Terminal User Interface
    Inspect(cmd::View),
}
