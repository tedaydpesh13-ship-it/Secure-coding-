//deepesh:
use std::fs;
use std::path::Path;

pub fn delete_file(filename: &str) -> Result<(), String> {
if !Path::new(filename).exists() {
return Err(format!("'{}' does not exist", filename));
}
fs::remove_file(filename)
.map_err(|e| format!("Failed to delete '{}': {}", filename, e))
}
