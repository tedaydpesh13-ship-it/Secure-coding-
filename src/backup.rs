use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

pub fn backup_file(filename: &str) -> Result<PathBuf, String> {
    let mut src = File::open(filename)
        .map_err(|e| format!("Cannot open source '{}': {}", filename, e))?;

    let backup_path = next_backup_name(filename);
    let mut dst = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&backup_path)
        .map_err(|e| format!("Cannot create backup '{}': {}", backup_path.display(), e))?;

    io::copy(&mut src, &mut dst)
        .map_err(|e| format!("Copy failed: {}", e))?;

    Ok(backup_path)
}

fn next_backup_name(filename: &str) -> PathBuf {
    let base = format!("{}.bak", filename);
    let p = Path::new(&base);
    if !p.exists() {
        return p.to_path_buf();
    }
    for i in 1..=9999 {
        let cand = format!("{}.{}", base, i);
        let cp = Path::new(&cand);
        if !cp.exists() {
            return cp.to_path_buf();
        }
    }
    // Fallback: overwrite .bak if everything else exists (very unlikely)
    p.to_path_buf()
}
