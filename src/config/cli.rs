use clap::{Parser, Subcommand};

const AUTHOR: &str = "ysuzuki19";

#[derive(Parser)]
#[command(
    author = AUTHOR,
    version,
)]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Plan diff or error
    Plan,
    /// Check if any changes or errors are detected
    Check,
    /// Apply changes to the file
    Apply,
}
