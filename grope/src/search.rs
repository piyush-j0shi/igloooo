use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::Result;

#[derive(Debug)]
pub struct Match {
    pub line_number: usize,
    pub line: String,
}

#[derive(Debug)]
pub struct FileResult {
    pub path: PathBuf,
    pub matches: Vec<Match>,
}

pub fn search_pattern(path: &Path, pattern: &str, case_sensitive: bool) -> Result<FileResult> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut matches: Vec<Match> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        if case_sensitive {
            let tobe_matchedstring = pattern.to_lowercase();
            if line.to_lowercase().contains(&tobe_matchedstring) {
                matches.push(Match {
                    line_number: i,
                    line: line,
                });
            }
        } else {
            if line.contains(pattern) {
                matches.push(Match {
                    line_number: i,
                    line: line,
                });
            }
        }
    }

    Ok(FileResult {
        path: path.to_path_buf(),
        matches: matches,
    })
}
