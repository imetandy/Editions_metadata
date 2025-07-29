use std::path::Path;
use anyhow::Result;

pub struct FileMetadata {
    pub format: String,
}

pub fn analyze_file(file_path: &Path) -> Result<FileMetadata> {
    let format = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_uppercase();

    Ok(FileMetadata {
        format,
    })
} 