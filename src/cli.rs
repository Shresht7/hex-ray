// Library
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Path to the file to read (defaults to reading from `stdin` if empty)
    #[clap(aliases = ["path", "src"])]
    pub filepath: Option<std::path::PathBuf>,

    /// The byte offset at which to start reading; i.e. skip the given number of bytes.
    /// You can specify a positive or negative integer value; A positive integer offset
    /// seeks forward from the start, while a negative offset seeks backwards from the end
    #[arg(aliases = ["skip", "seek"], short, long, default_value_t = 0)]
    pub offset: i64,

    /// The number of bytes to read. The program will stop after reading
    /// the specified number of bytes.
    #[arg(short, long)]
    pub limit: Option<usize>,

    /// The size of each row
    #[arg(short, long, default_value_t = 16)]
    pub size: usize,

    /// Chunk the output into groups of this size
    #[arg(alias = "chunk", short, long, default_value_t = 4)]
    pub group_size: usize,
}
