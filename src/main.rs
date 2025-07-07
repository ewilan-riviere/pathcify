use clap::Parser;
use pathcify::cli::Cli;
use pathcify::walker::process_dir;

fn main() {
    let cli = Cli::parse();
    let full_path = cli.path.canonicalize().unwrap_or_else(|_| {
        eprintln!("Error: Unable to canonicalize path.");
        std::process::exit(1);
    });
    println!("Pathcify on path: {}", full_path.display());

    if !cli.path.exists() {
        eprintln!("Error: path does not exist.");
        std::process::exit(1);
    }

    process_dir(&cli.path, cli.lowercase, cli.verbose);
    println!("Done!");
}
