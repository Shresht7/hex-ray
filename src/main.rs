// Traits
use clap::Parser;
use std::io::{BufReader, Seek};

// Modules
mod cli;
mod format;
mod helpers;
mod print;

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
        Some(cli::Command::View(mut cmd)) => {
            cmd.init();

            let reader = match &cmd.filepath.clone() {
                // If a `filepath` was passed in the arguments, read the file ...
                Some(filepath) => get_file_reader(filepath, &mut cmd),
                // otherwise, read the input from stdin.
                None => get_stdin_reader(&mut cmd),
            }?;

            cmd.dump(reader)?
        }
        _ => {}
    };
    Ok(ret)
}

fn get_stdin_reader(
    args: &mut cli::View,
) -> Result<Box<dyn std::io::BufRead>, Box<dyn std::error::Error>> {
    args.offset = 0; // Offset is not supported in this mode
    let data = std::io::stdin();
    Ok(Box::new(BufReader::new(data)))
}

fn get_file_reader(
    filepath: &std::path::PathBuf,
    args: &mut cli::View,
) -> Result<Box<dyn std::io::BufRead>, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(filepath)?;
    // A positive offset seeks forwards from the start of the file
    if args.offset >= 0 {
        file.seek(std::io::SeekFrom::Start(args.offset as u64))?;
    } else if args.offset < 0 {
        // ... while an negative offset seeks backwards from the end of the file
        let file_size = file.seek(std::io::SeekFrom::End(0))?;
        file.seek(std::io::SeekFrom::End(args.offset))?;
        args.offset = file_size as i64 + args.offset;
    }
    Ok(Box::new(BufReader::new(file)))
}
