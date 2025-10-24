use std::path::PathBuf;
use std::process::Command;

pub fn reveal_in_explorer(path: &PathBuf) {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("explorer")
            .arg("/select,")
            .arg(path)
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open")
            .arg("-R")
            .arg(path)
            .spawn();
    }

    #[cfg(target_os = "linux")]
    {
        // Try to open the parent folder
        if let Some(parent) = path.parent() {
            let _ = Command::new("xdg-open")
                .arg(parent)
                .spawn();
        }
    }
}