use clap::Parser;
use std::path::PathBuf;

/// Recursively slugify file and directory names using dot format
#[derive(Parser, Debug)]
#[command(name = "pathcify")]
#[command(author = "Ewilan Rivière")]
#[command(version = "0.0.2")]
#[command(
    about = "CLI tool to recursively slugify file and directory names using dots, removing special characters and normalizing names."
)]
pub struct Cli {
    /// Path to process
    pub path: PathBuf,

    /// Force lowercase
    #[arg(short, long, default_value_t = false)]
    pub lowercase: bool,

    /// Print detailed output
    #[arg(short, long)]
    pub verbose: bool,
}
