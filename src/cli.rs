use clap::Parser;
use std::path::PathBuf;

/// Recursively slugify file and directory names using dot format
#[derive(Parser, Debug)]
#[command(name = "pathcify")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Slugifies files and directories recursively")]
pub struct Cli {
    /// Path to process
    pub path: PathBuf,

    /// Force lowercase
    #[arg(short, long, default_value_t = false)]
    pub lowercase: bool,
}
