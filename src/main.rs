fn main() {
    // Collect the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Exit if incorrect number of arguments are passed
    if args.len() < 2 {
        eprintln!("Usage: hex-ray <file-path>");
        std::process::exit(1);
    }

    // Get the filepath from the arguments
    let filepath = &args[1];

    println!("{}", filepath);
}
