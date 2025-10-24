use scan_dir::ScanDir;
use crate::models::FileSystemItem;

pub fn scan_directory(path: &str) -> Result<Vec<FileSystemItem>, std::io::Error> {
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

    // Scan files (filter for SVG)
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

    // Optional: sort so directories come first, then alphabetically
    items.sort_by(|a, b| {
        match (a, b) {
            (FileSystemItem::Directory { name: n1, .. }, FileSystemItem::Directory { name: n2, .. }) => n1.cmp(n2),
            (FileSystemItem::SvgFile { name: n1, .. }, FileSystemItem::SvgFile { name: n2, .. }) => n1.cmp(n2),
            (FileSystemItem::Directory { .. }, FileSystemItem::SvgFile { .. }) => std::cmp::Ordering::Less,
            (FileSystemItem::SvgFile { .. }, FileSystemItem::Directory { .. }) => std::cmp::Ordering::Greater,
        }
    });

    Ok(items)
}