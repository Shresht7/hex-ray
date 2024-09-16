// Traits
use clap::Parser;

// Modules
mod cli;
mod utils;

fn main() {
    let args = cli::Args::parse();
    match run(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(args: cli::Args) -> Result<(), Box<dyn std::error::Error>> {
    let ret = match args.cmd {
        Some(cli::Command::View(cmd)) => cmd.execute()?,
        Some(cli::Command::Output(cmd)) => cmd.execute()?,
        Some(cli::Command::Tui(cmd)) => cmd.execute_tui()?,
        _ => {}
    };
    Ok(ret)
}
