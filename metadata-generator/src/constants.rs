pub const IGNORE_FILES: &[&str] = &[
    ".DS_Store", "Thumbs.db", ".gitignore", ".gitkeep"
];

pub fn should_ignore_file(file_name: &str) -> bool {
    IGNORE_FILES.contains(&file_name) || 
    file_name.ends_with("_metadata.json") ||
    file_name.ends_with("metadata.json") ||
    file_name == "metadata.json"
} 