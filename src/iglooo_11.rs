use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct LogEntry {
    ip: String,
    timestamp: String,
    method: Option<String>,
    status_code: Option<u16>,
}

struct LogStats {
    total_lines: usize,
    valid_entries: usize,
    invalid_entries: usize,
    ip_counts: HashMap<String, usize>,
    status_code_counts: HashMap<u16, usize>,
    method_counts: HashMap<String, usize>,
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 9 {
        return None;
    }

    let ip = parts[0].to_string();

    let timestamp = if let Some(start) = line.find('[') {
        if let Some(end) = line.find(']') {
            line[start + 1..end].to_string()
        } else {
            return None;
        }
    } else {
        return None;
    };

    let quote_start = line.find('"')?;
    let quote_end = line[quote_start + 1..].find('"')?;
    let request = &line[quote_start + 1..quote_start + 1 + quote_end];
    let method = request.split_whitespace().next().map(|s| s.to_string());

    let status_code_str = parts.last().unwrap_or(&"");
    let status_code = status_code_str.parse::<u16>().ok();

    Some(LogEntry {
        ip,
        timestamp,
        method,
        status_code,
    })
}

fn process_log_file(file_path: &str) -> LogStats {
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut stats = LogStats {
        total_lines: 0,
        valid_entries: 0,
        invalid_entries: 0,
        ip_counts: HashMap::new(),
        status_code_counts: HashMap::new(),
        method_counts: HashMap::new(),
    };

    for line in reader.lines() {
        stats.total_lines += 1;

        match line {
            Ok(l) => {
                if let Some(entry) = parse_log_line(&l) {
                    stats.valid_entries += 1;

                    *stats.ip_counts.entry(entry.ip).or_insert(0) += 1;

                    if let Some(code) = entry.status_code {
                        *stats.status_code_counts.entry(code).or_insert(0) += 1;
                    }

                    if let Some(method) = entry.method {
                        *stats.method_counts.entry(method).or_insert(0) += 1;
                    }
                } else {
                    stats.invalid_entries += 1;
                }
            }
            Err(_) => stats.invalid_entries += 1,
        }
    }

    stats
}

fn print_summary(stats: &LogStats) {
    println!("Log Summary:");
    println!("Total lines       : {}", stats.total_lines);
    println!("Valid entries     : {}", stats.valid_entries);
    println!("Invalid entries   : {}", stats.invalid_entries);
    println!("\nTop IP addresses:");
    for (ip, count) in &stats.ip_counts {
        println!("- {}: {}", ip, count);
    }

    println!("\nStatus code counts:");
    for (code, count) in &stats.status_code_counts {
        println!("- {}: {}", code, count);
    }

    println!("\nHTTP methods used:");
    for (method, count) in &stats.method_counts {
        println!("- {}: {}", method, count);
    }
}

fn main() {
    let log_file = "server.log";
    let stats = process_log_file(log_file);
    print_summary(&stats);
}
