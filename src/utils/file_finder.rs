use scan_dir::ScanDir;
use crate::models::FileSystemItem;
use std::fs;

pub enum FileFilter {
    Svg,
    Font,
}

// Helper function to check if a file has valid UTF-8 content
fn is_valid_utf8_file(path: &std::path::PathBuf) -> bool {
    fs::read_to_string(path).is_ok()
}

pub fn scan_directory(path: &str, filter: FileFilter) -> Result<Vec<FileSystemItem>, std::io::Error> {
    let mut items = Vec::new();

    // Scan directories - handle errors gracefully
    if let Err(e) = ScanDir::dirs().read(path, |iter| {
        for (entry, name) in iter {
            items.push(FileSystemItem::Directory {
                name: name.clone(),
                path: entry.path(),
            });
        }
    }) {
        // Return error if we can't read the directory at all
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Failed to scan directory '{}': {}", path, e)
        ));
    }

    // Scan files based on filter
    match filter {
        FileFilter::Svg => {
            // If file scanning fails, just skip it (we already got directories)
            let _ = ScanDir::files().read(path, |iter| {
                for (entry, name) in iter {
                    if name.ends_with(".svg") {
                        // Only include SVG files with valid UTF-8 content
                        if is_valid_utf8_file(&entry.path()) {
                            items.push(FileSystemItem::SvgFile {
                                name: name.clone(),
                                path: entry.path(),
                            });
                        }
                        // Silently skip files with invalid UTF-8
                    }
                }
            });
        }
        FileFilter::Font => {
            // If file scanning fails, just skip it (we already got directories)
            let _ = ScanDir::files().read(path, |iter| {
                for (entry, name) in iter {
                    let name_lower = name.to_lowercase();
                    if name_lower.ends_with(".ttf")
                        || name_lower.ends_with(".otf")
                        || name_lower.ends_with(".woff")
                        || name_lower.ends_with(".woff2") {
                        items.push(FileSystemItem::FontFile {
                            name: name.clone(),
                            path: entry.path(),
                        });
                    }
                }
            });
        }
    }

    // Sort so directories come first, then alphabetically
    items.sort_by(|a, b| {
        match (a, b) {
            (FileSystemItem::Directory { name: n1, .. }, FileSystemItem::Directory { name: n2, .. }) => n1.cmp(n2),
            (FileSystemItem::SvgFile { name: n1, .. }, FileSystemItem::SvgFile { name: n2, .. }) => n1.cmp(n2),
            (FileSystemItem::FontFile { name: n1, .. }, FileSystemItem::FontFile { name: n2, .. }) => n1.cmp(n2),
            (FileSystemItem::Directory { .. }, _) => std::cmp::Ordering::Less,
            (_, FileSystemItem::Directory { .. }) => std::cmp::Ordering::Greater,
            (FileSystemItem::SvgFile { .. }, FileSystemItem::FontFile { .. }) => std::cmp::Ordering::Less,
            (FileSystemItem::FontFile { .. }, FileSystemItem::SvgFile { .. }) => std::cmp::Ordering::Greater,
        }
    });

    Ok(items)
}