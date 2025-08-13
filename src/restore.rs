//sahil:
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

pub fn restore_file(filename: &str) -> Result<PathBuf, String> {
    let backup_name = format!("{}.bak", filename);
    let backup_path = Path::new(&backup_name);
    if !backup_path.exists() {
        return Err(format!("Backup '{}' does not exist", backup_path.display()));
    }

    // Read backup
    let mut src = File::open(&backup_path)
        .map_err(|e| format!("Cannot open backup '{}': {}", backup_path.display(), e))?;

    // Write to temp then rename (safer)
    let tmp_name = format!("{}.tmp", filename);
    let tmp_path = Path::new(&tmp_name);
    {
        let mut dst = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(tmp_path)
            .map_err(|e| format!("Cannot create temp file '{}': {}", tmp_path.display(), e))?;
        io::copy(&mut src, &mut dst)
            .map_err(|e| format!("Copy failed: {}", e))?;
    }

    // On Windows, rename fails if target exists; remove then rename for portability
    if Path::new(filename).exists() {
        fs::remove_file(filename)
            .map_err(|e| format!("Failed to remove existing '{}': {}", filename, e))?;
    }
    fs::rename(tmp_path, filename)
        .map_err(|e| format!("Failed to finalize restore to '{}': {}", filename, e))?;

    Ok(backup_path.to_path_buf())
}
