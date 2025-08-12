use std::path::{Component, Path};

/// Validate a user-supplied filename so all operations stay in the working folder.
/// Rules:
/// - not empty, <= 100 chars
/// - relative only (no absolute paths)
/// - no parent components (..), no directories
/// - allowed chars: A-Z a-z 0-9 . _ -
pub fn validate_filename(input: &str) -> Result<(), String> {
    if input.trim().is_empty() {
        return Err("Filename cannot be empty".into());
    }
    if input.chars().count() > 100 {
        return Err("Filename is too long (max 100)".into());
    }
    let p = Path::new(input);
    if p.is_absolute() {
        return Err("Absolute paths are not allowed".into());
    }
    // must be a single path segment (no dirs)
    if p.components().count() != 1 {
        return Err("Directories are not allowed — use a simple file name".into());
    }
    for comp in p.components() {
        match comp {
            Component::Normal(os) => {
                let s = os.to_string_lossy();
                if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_') {
                    return Err("Filename contains invalid characters (allowed: letters, numbers, ., -, _)".into());
                }
                if s == "." || s == ".." {
                    return Err("Reserved names are not allowed".into());
                }
            }
            Component::ParentDir | Component::RootDir | Component::Prefix(_) | Component::CurDir => {
                return Err("Path traversal or special components are not allowed".into());
            }
        }
    }
    Ok(())
}
