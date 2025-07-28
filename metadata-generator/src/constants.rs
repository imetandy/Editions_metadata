use std::path::Path;

pub const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "avi", "mov", "wmv", "flv", "webm", "mkv", "m4v", "3gp", "ogv"
];

pub const AUDIO_EXTENSIONS: &[&str] = &[
    "wav", "mp3", "aac", "flac", "ogg", "wma", "m4a", "opus", "aiff", "alac"
];

pub const IGNORE_FILES: &[&str] = &[
    ".DS_Store", "Thumbs.db", ".gitignore", ".gitkeep"
];

pub fn get_file_type(file_path: &Path) -> &'static str {
    if let Some(extension) = file_path.extension() {
        let ext_str = extension.to_str().unwrap_or("").to_lowercase();
        
        if VIDEO_EXTENSIONS.contains(&ext_str.as_str()) {
            "video"
        } else if AUDIO_EXTENSIONS.contains(&ext_str.as_str()) {
            "audio"
        } else {
            "video" // Default to video for unknown extensions
        }
    } else {
        "video" // Default to video if no extension
    }
}

pub fn should_ignore_file(file_name: &str) -> bool {
    IGNORE_FILES.contains(&file_name)
} 