use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FileSystemItem {
    Directory { name: String, path: PathBuf },
    SvgFile { name: String, path: PathBuf },
    FontFile { name: String, path: PathBuf },
}
