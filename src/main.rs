use clap::Parser;
use dotify::cli::Cli;
use dotify::walker::process_dir;

fn main() {
    let cli = Cli::parse();

    if !cli.path.exists() {
        eprintln!("Error: path does not exist.");
        std::process::exit(1);
    }

    process_dir(&cli.path, cli.lowercase);
}
