use scan_dir::ScanDir;
use crate::models::FileSystemItem;

pub enum FileFilter {
    Svg,
    Font,
}

pub fn scan_directory(path: &str, filter: FileFilter) -> Result<Vec<FileSystemItem>, std::io::Error> {
    let mut items = Vec::new();

    // Scan directories
    ScanDir::dirs().read(path, |iter| {
        for (entry, name) in iter {
            items.push(FileSystemItem::Directory {
                name: name.clone(),
                path: entry.path(),
            });
        }
    }).unwrap();

    // Scan files based on filter
    match filter {
        FileFilter::Svg => {
            ScanDir::files().read(path, |iter| {
                for (entry, name) in iter {
                    if name.ends_with(".svg") {
                        items.push(FileSystemItem::SvgFile {
                            name: name.clone(),
                            path: entry.path(),
                        });
                    }
                }
            }).unwrap();
        }
        FileFilter::Font => {
            ScanDir::files().read(path, |iter| {
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
            }).unwrap();
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