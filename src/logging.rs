use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Utc;

pub fn log_action(action: &str, filename: &str, outcome: &str) -> io::Result<()> {
    let ts = Utc::now().to_rfc3339();
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logfile.txt")?;
    writeln!(f, "{} | {} | {} | {}", ts, action, filename, outcome)?;
    Ok(())
}
