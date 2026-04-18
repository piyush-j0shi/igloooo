pub mod cli;
pub mod search;
pub mod walker;
use rayon::prelude::*;

use crate::search::{search_pattern, FileResult};
use anyhow::Result;
use colored::Colorize;

fn main() -> Result<()> {
    let args = cli::parse_args()?;
    let pattern = &args.pattern;
    let pathes = walker::list_directories(&args.paths[0])?;

    let results: Vec<FileResult> = pathes
        .par_iter()
        .map(|path| search_pattern(path, pattern, args.case_insensitive))
        .collect::<Result<Vec<_>>>()?;

    for result in results {
        if !result.matches.is_empty() {
            println!("\n{}", result.path.display().to_string().green().bold());
            for item in &result.matches {
                let highlighted = item.line.replace(pattern, &pattern.red().to_string());
                println!(
                    "  {}: {}",
                    item.line_number.to_string().yellow(),
                    highlighted
                );
            }
        }
    }

    Ok(())
}
