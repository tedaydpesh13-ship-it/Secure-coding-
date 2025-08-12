mod backup;
mod restore;
mod delete;
mod validate;
mod logging;

use std::io::{self, Write};

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim().to_string())
}

fn main() {
    // 1) Ask filename
    let filename = match read_line("Please enter your file name: ") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read input: {}", e);
            return;
        }
    };

    if let Err(msg) = validate::validate_filename(&filename) {
        eprintln!("Invalid filename: {}", msg);
        let _ = logging::log_action("input", &filename, &format!("invalid: {}", msg));
        return;
    }

    // 2) Ask command
    let cmd = match read_line("Please enter your command (backup, restore, delete): ") {
        Ok(s) => s.to_lowercase(),
        Err(e) => {
            eprintln!("Failed to read input: {}", e);
            return;
        }
    };

    match cmd.as_str() {
        "backup" => {
            match backup::backup_file(&filename) {
                Ok(backup_path) => {
                    println!("Your backup created: {}", backup_path.display());
                    let _ = logging::log_action("backup", &filename, &format!("ok -> {}", backup_path.display()));
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    let _ = logging::log_action("backup", &filename, &format!("error: {}", err));
                }
            }
        }
        "restore" => {
            match restore::restore_file(&filename) {
                Ok(from_backup) => {
                    println!("File restored from: {}", from_backup.display());
                    let _ = logging::log_action("restore", &filename, &format!("ok <- {}", from_backup.display()));
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    let _ = logging::log_action("restore", &filename, &format!("error: {}", err));
                }
            }
        }
        "delete" => {
            let confirm = read_line(&format!("Are you sure you want to delete {}? (yes/no): ", filename));
            match confirm {
                Ok(ans) if ans == "yes" => match delete::delete_file(&filename) {
                    Ok(()) => {
                        println!("File deleted.");
                        let _ = logging::log_action("delete", &filename, "ok");
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        let _ = logging::log_action("delete", &filename, &format!("error: {}", err));
                    }
                },
                Ok(_) => {
                    println!("Deletion cancelled.");
                    let _ = logging::log_action("delete", &filename, "cancelled");
                }
                Err(e) => {
                    eprintln!("Failed to read input: {}", e);
                    let _ = logging::log_action("delete", &filename, &format!("error reading confirm: {}", e));
                }
            }
        }
        _ => {
            println!("Unknown command");
            let _ = logging::log_action("input", &filename, "unknown command");
        }
    }
}
