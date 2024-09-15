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
    View(cmd::View),
    Output,
}
