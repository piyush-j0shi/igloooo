use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct SearchConfig {
    /// The text pattern to search for
    pub pattern: String,

    /// Files or directories to search
    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    /// Perform a case-insensitive search
    #[arg(short = 'i', long)]
    pub case_insensitive: bool,

    /// Search directories recursively
    #[arg(short = 'r', long)]
    pub recursive: bool,

    /// Show line numbers
    #[arg(long, default_value_t = true)]
    pub show_line_numbers: bool,
}

pub fn parse_args() -> Result<SearchConfig> {
    Ok(SearchConfig::parse())
}
