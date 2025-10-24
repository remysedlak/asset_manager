use std::path::{Path, PathBuf};
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

pub fn copy_file_to_clipboard(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        use arboard::SetExtWindows;
        let mut clipboard = Clipboard::new()?;
        clipboard
            .set()
            .file_list([path])
            .wait()
            .map_err(|e| format!("Failed to copy file: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        use arboard::SetExtMacOS;
        let mut clipboard = Clipboard::new()?;
        clipboard
            .set()
            .file_list([path])
            .wait()
            .map_err(|e| format!("Failed to copy file: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        let uri = format!("file://{}\n", path.canonicalize()?.display());
        use std::io::Write;

        let mut child = std::process::Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .arg("-t")
            .arg("text/uri-list")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("xclip not found. Please install it with: sudo apt install xclip: {e}"))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(uri.as_bytes())?;
        }

        child.wait()?;
    }

    Ok(())
}