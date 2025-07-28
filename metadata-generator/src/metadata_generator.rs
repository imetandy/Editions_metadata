use std::{fs::{self, File}, io::{BufWriter, Write}, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::constants::{get_file_type, should_ignore_file};
use crate::hasher::UnifiedHasher;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaFile {
    pub file_name: String,
    pub file_hash: String,
    pub file_size: u64,
    pub file_type: String,
    pub file_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub date_created: String,
    pub title: String,
    pub creator: String,
    pub description: String,
    pub video_files: Vec<MediaFile>,
    pub audio_files: Vec<MediaFile>,
}

#[allow(dead_code)]
pub enum ProgressCallback {
    Cli(Box<dyn Fn(String) + Send>),
    Gui(Box<dyn Fn(String, f32, f32) + Send>), // message, file_progress, overall_progress
    None,
}

pub struct MetadataGenerator {
    hasher: UnifiedHasher,
    progress_callback: ProgressCallback,
}

impl MetadataGenerator {
    #[allow(dead_code)]
    pub fn new_cli() -> Self {
        Self {
            hasher: UnifiedHasher::new_cli(),
            progress_callback: ProgressCallback::None,
        }
    }

    pub fn new_gui() -> Self {
        Self {
            hasher: UnifiedHasher::new_gui(),
            progress_callback: ProgressCallback::None,
        }
    }

    pub fn with_progress_callback(mut self, callback: ProgressCallback) -> Self {
        self.progress_callback = callback;
        self
    }

    #[allow(dead_code)]
    pub fn generate_metadata(
        &self,
        folder_path: &PathBuf,
        metadata: &Metadata,
    ) -> std::io::Result<PathBuf> {
        let entries: Vec<_> = fs::read_dir(folder_path)?.collect();
        let total_files = entries.len();
        let mut processed_files = 0;

        let mut output_metadata = metadata.clone();
        output_metadata.video_files.clear();
        output_metadata.audio_files.clear();

        for (index, entry) in entries.into_iter().enumerate() {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let path_str = path.to_string_lossy().to_string();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                
                // Skip ignored files
                if should_ignore_file(&file_name) {
                    continue;
                }
                
                // Report progress
                match &self.progress_callback {
                    ProgressCallback::Cli(callback) => {
                        callback(format!("Hashing file: {}", file_name));
                    }
                    ProgressCallback::Gui(callback) => {
                        let overall_progress = index as f32 / total_files as f32;
                        callback(file_name.clone(), 0.0, overall_progress);
                    }
                    ProgressCallback::None => {}
                }

                // Hash the file
                let hash = self.hasher.hash_file(&path_str)?;
                processed_files += 1;

                let mf = MediaFile {
                    file_name: file_name.clone(),
                    file_hash: hash,
                    file_size: path.metadata()?.len(),
                    file_type: path.extension().unwrap_or_default().to_string_lossy().to_string(),
                    file_path: path_str,
                };
                
                // Automatically sort files based on their type
                match get_file_type(&path) {
                    "video" => output_metadata.video_files.push(mf),
                    "audio" => output_metadata.audio_files.push(mf),
                    _ => output_metadata.video_files.push(mf), // Default to video
                }

                // Report completion
                match &self.progress_callback {
                    ProgressCallback::Cli(callback) => {
                        callback(format!("Completed: {} ({}/{})", file_name, processed_files, total_files));
                    }
                    ProgressCallback::Gui(callback) => {
                        let overall_progress = processed_files as f32 / total_files as f32;
                        callback(file_name, 1.0, overall_progress);
                    }
                    ProgressCallback::None => {}
                }
            }
        }

        // Save metadata to file
        let file_name = format!("{}_metadata.json", metadata.title.replace(' ', "_"));
        let output = folder_path.join(file_name);
        let file = File::create(&output)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &output_metadata)?;
        writer.flush()?;

        Ok(output)
    }

    pub fn generate_metadata_async(
        &self,
        folder_path: &PathBuf,
        metadata: &Metadata,
    ) -> std::io::Result<PathBuf> {
        let entries: Vec<_> = fs::read_dir(folder_path)?.collect();
        let total_files = entries.len();
        let mut processed_files = 0;

        let mut output_metadata = metadata.clone();
        output_metadata.video_files.clear();
        output_metadata.audio_files.clear();

        for (index, entry) in entries.into_iter().enumerate() {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let path_str = path.to_string_lossy().to_string();
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                
                // Skip ignored files
                if should_ignore_file(&file_name) {
                    continue;
                }
                
                // Report progress
                match &self.progress_callback {
                    ProgressCallback::Cli(callback) => {
                        callback(format!("Hashing file: {}", file_name));
                    }
                    ProgressCallback::Gui(callback) => {
                        let overall_progress = index as f32 / total_files as f32;
                        callback(file_name.clone(), 0.0, overall_progress);
                    }
                    ProgressCallback::None => {}
                }

                // Start async hashing
                self.hasher.hash_file_async(&path_str);
                
                // Wait for hash to complete while updating progress
                let hash = loop {
                    // Update file progress for GUI
                    if let ProgressCallback::Gui(callback) = &self.progress_callback {
                        let file_progress = self.hasher.get_file_progress();
                        let overall_progress = index as f32 / total_files as f32;
                        callback(file_name.clone(), file_progress, overall_progress);
                    }
                    
                    // Check if hash is complete
                    if let Some(result) = self.hasher.get_result() {
                        match result {
                            Ok(hash) => break hash,
                            Err(e) => return Err(e),
                        }
                    }
                    
                    std::thread::sleep(std::time::Duration::from_millis(50));
                };
                
                processed_files += 1;

                let mf = MediaFile {
                    file_name: file_name.clone(),
                    file_hash: hash,
                    file_size: path.metadata()?.len(),
                    file_type: path.extension().unwrap_or_default().to_string_lossy().to_string(),
                    file_path: path_str,
                };
                
                // Automatically sort files based on their type
                match get_file_type(&path) {
                    "video" => output_metadata.video_files.push(mf),
                    "audio" => output_metadata.audio_files.push(mf),
                    _ => output_metadata.video_files.push(mf), // Default to video
                }

                // Report completion
                match &self.progress_callback {
                    ProgressCallback::Cli(callback) => {
                        callback(format!("Completed: {} ({}/{})", file_name, processed_files, total_files));
                    }
                    ProgressCallback::Gui(callback) => {
                        let overall_progress = processed_files as f32 / total_files as f32;
                        callback(file_name, 1.0, overall_progress);
                    }
                    ProgressCallback::None => {}
                }
            }
        }

        // Save metadata to file
        let file_name = format!("{}_metadata.json", metadata.title.replace(' ', "_"));
        let output = folder_path.join(file_name);
        let file = File::create(&output)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &output_metadata)?;
        writer.flush()?;

        Ok(output)
    }
} 