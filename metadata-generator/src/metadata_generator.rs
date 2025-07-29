use std::{fs::{self, File}, io::{BufWriter, Write}, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::constants::should_ignore_file;
use crate::hasher::UnifiedHasher;
use crate::file_analyzer::analyze_file;
use memmap;
use blake3;

/// Detects certificate of authenticity PDF files in a certificate folder
/// Returns the relative path to the first PDF file found, or None if no PDF files exist
pub fn detect_certificate_of_authenticity(folder_path: &PathBuf) -> Option<String> {
    let certificate_folder = folder_path.join("certificate");
    
    if !certificate_folder.exists() || !certificate_folder.is_dir() {
        return None;
    }
    
    // Look for any PDF file in the certificate folder
    if let Ok(entries) = fs::read_dir(&certificate_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension.to_string_lossy().to_lowercase() == "pdf" {
                            // Return relative path to the PDF file
                            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                            return Some(format!("./certificate/{}", file_name));
                        }
                    }
                }
            }
        }
    }
    
    None
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtworkFile {
    pub path: String,
    pub file_name: String,
    pub file_hash: String,
    pub file_size: u64,
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub artwork_id: String,
    pub artwork_title: String,
    pub artwork_short_title: String,
    pub artwork_creator: String,
    pub year_of_creation: i32,
    pub short_description: String,
    pub long_description: String,
    pub edition_number: i32,
    pub total_editions: i32,
    pub issue_date: String,
    pub gallery: String,
    pub keywords: Vec<String>,
    pub medium: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_of_authenticity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash: Option<String>,
    pub artwork_files: Vec<ArtworkFile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerificationResult {
    pub file_name: String,
    pub expected_hash: String,
    pub actual_hash: String,
    pub is_valid: bool,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerificationReport {
    pub metadata_file_hash: String,
    pub total_files: usize,
    pub valid_files: usize,
    pub invalid_files: usize,
    pub results: Vec<VerificationResult>,
    pub metadata_file_valid: bool,
    pub certificate_valid: Option<bool>,
    pub certificate_hash: Option<String>,
    pub overall_valid: bool,
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
        output_metadata.artwork_files.clear();

        for (index, entry) in entries.into_iter().enumerate() {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let relative_path = format!("./{}", file_name);
                
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

                // Hash the file (use absolute path for hashing)
                let absolute_path = path.to_string_lossy().to_string();
                let hash = self.hasher.hash_file(&absolute_path)?;
                processed_files += 1;

                // Analyze file to extract metadata
                let file_metadata = analyze_file(&path).unwrap_or_else(|_| {
                    // Fallback if analysis fails
                    crate::file_analyzer::FileMetadata {
                        format: path.extension().unwrap_or_default().to_string_lossy().to_string().to_uppercase(),
                    }
                });

                let af = ArtworkFile {
                    path: relative_path,
                    file_name: file_name.clone(),
                    file_hash: hash,
                    file_size: path.metadata()?.len(),
                    format: file_metadata.format,
                };
                
                output_metadata.artwork_files.push(af);

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

        // Hash certificate if it exists
        if let Some(certificate_path) = &output_metadata.certificate_of_authenticity {
            let certificate_full_path = folder_path.join(certificate_path.trim_start_matches("./"));
            if certificate_full_path.exists() {
                match self.hasher.hash_file(&certificate_full_path.to_string_lossy()) {
                    Ok(certificate_hash) => {
                        output_metadata.certificate_hash = Some(certificate_hash);
                        
                        // Report certificate hashing
                        match &self.progress_callback {
                            ProgressCallback::Cli(callback) => {
                                callback(format!("Certificate hashed: {}", certificate_path));
                            }
                            ProgressCallback::Gui(callback) => {
                                callback("Certificate".to_string(), 1.0, 1.0);
                            }
                            ProgressCallback::None => {}
                        }
                    }
                    Err(e) => {
                        // Log error but continue
                        match &self.progress_callback {
                            ProgressCallback::Cli(callback) => {
                                callback(format!("Warning: Could not hash certificate: {}", e));
                            }
                            ProgressCallback::Gui(callback) => {
                                callback("Certificate error".to_string(), 1.0, 1.0);
                            }
                            ProgressCallback::None => {}
                        }
                    }
                }
            }
        }

        // Save metadata to file
        let file_name = format!("{}_metadata.json", metadata.artwork_title.replace(' ', "_"));
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
        output_metadata.artwork_files.clear();

        for (index, entry) in entries.into_iter().enumerate() {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                let relative_path = format!("./{}", file_name);
                
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

                // Start async hashing (use absolute path for hashing)
                let absolute_path = path.to_string_lossy().to_string();
                self.hasher.hash_file_async(&absolute_path);
                
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

                // Analyze file to extract metadata
                let file_metadata = analyze_file(&path).unwrap_or_else(|_| {
                    // Fallback if analysis fails
                    crate::file_analyzer::FileMetadata {
                        format: path.extension().unwrap_or_default().to_string_lossy().to_string().to_uppercase(),
                    }
                });

                let af = ArtworkFile {
                    path: relative_path,
                    file_name: file_name.clone(),
                    file_hash: hash,
                    file_size: path.metadata()?.len(),
                    format: file_metadata.format,
                };
                
                output_metadata.artwork_files.push(af);

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

        // Hash certificate if it exists
        if let Some(certificate_path) = &output_metadata.certificate_of_authenticity {
            let certificate_full_path = folder_path.join(certificate_path.trim_start_matches("./"));
            if certificate_full_path.exists() {
                match self.hasher.hash_file(&certificate_full_path.to_string_lossy()) {
                    Ok(certificate_hash) => {
                        output_metadata.certificate_hash = Some(certificate_hash);
                        
                        // Report certificate hashing
                        match &self.progress_callback {
                            ProgressCallback::Cli(callback) => {
                                callback(format!("Certificate hashed: {}", certificate_path));
                            }
                            ProgressCallback::Gui(callback) => {
                                callback("Certificate".to_string(), 1.0, 1.0);
                            }
                            ProgressCallback::None => {}
                        }
                    }
                    Err(e) => {
                        // Log error but continue
                        match &self.progress_callback {
                            ProgressCallback::Cli(callback) => {
                                callback(format!("Warning: Could not hash certificate: {}", e));
                            }
                            ProgressCallback::Gui(callback) => {
                                callback("Certificate error".to_string(), 1.0, 1.0);
                            }
                            ProgressCallback::None => {}
                        }
                    }
                }
            }
        }

        // Save metadata to file
        let file_name = format!("{}_metadata.json", metadata.artwork_title.replace(' ', "_"));
        let output = folder_path.join(file_name);
        let file = File::create(&output)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &output_metadata)?;
        writer.flush()?;

        Ok(output)
    }

    /// Fingerprints the metadata file itself using BLAKE3
    pub fn fingerprint_metadata_file(&self, metadata_path: &PathBuf) -> std::io::Result<String> {
        let file = File::open(metadata_path)?;
        let mmap = unsafe { memmap::Mmap::map(&file)? };
        let mut hasher = blake3::Hasher::new();
        hasher.update(&mmap);
        let hash = hasher.finalize();
        Ok(hash.to_hex().to_string())
    }

    /// Verifies all files in a metadata file against their recorded hashes
    pub fn verify_metadata_file(
        &self,
        metadata_path: &PathBuf,
        base_folder: &PathBuf,
    ) -> std::io::Result<VerificationReport> {
        // Read and parse the metadata file
        let metadata_content = fs::read_to_string(metadata_path)?;
        let metadata: Metadata = serde_json::from_str(&metadata_content)?;

        // Fingerprint the metadata file itself
        let metadata_file_hash = self.fingerprint_metadata_file(metadata_path)?;

        let mut results = Vec::new();
        let mut valid_files = 0;
        let mut invalid_files = 0;

        // Verify each file
        for artwork_file in &metadata.artwork_files {
            let file_path = base_folder.join(&artwork_file.file_name);
            
            let verification_result = if file_path.exists() {
                // Hash the actual file
                match self.hasher.hash_file(&file_path.to_string_lossy()) {
                    Ok(actual_hash) => {
                        let is_valid = actual_hash == artwork_file.file_hash;
                        if is_valid {
                            valid_files += 1;
                        } else {
                            invalid_files += 1;
                        }
                        
                        VerificationResult {
                            file_name: artwork_file.file_name.clone(),
                            expected_hash: artwork_file.file_hash.clone(),
                            actual_hash,
                            is_valid,
                            error: None,
                        }
                    }
                    Err(e) => {
                        invalid_files += 1;
                        VerificationResult {
                            file_name: artwork_file.file_name.clone(),
                            expected_hash: artwork_file.file_hash.clone(),
                            actual_hash: String::new(),
                            is_valid: false,
                            error: Some(e.to_string()),
                        }
                    }
                }
            } else {
                invalid_files += 1;
                VerificationResult {
                    file_name: artwork_file.file_name.clone(),
                    expected_hash: artwork_file.file_hash.clone(),
                    actual_hash: String::new(),
                    is_valid: false,
                    error: Some("File not found".to_string()),
                }
            };
            
            results.push(verification_result);
        }

        let total_files = results.len();
        
        // Verify certificate if it exists
        let mut certificate_valid = None;
        let mut certificate_hash = None;
        
        if let Some(certificate_path) = &metadata.certificate_of_authenticity {
            let certificate_full_path = base_folder.join(certificate_path.trim_start_matches("./"));
            if certificate_full_path.exists() {
                match self.hasher.hash_file(&certificate_full_path.to_string_lossy()) {
                    Ok(actual_certificate_hash) => {
                        certificate_hash = Some(actual_certificate_hash.clone());
                        if let Some(expected_certificate_hash) = &metadata.certificate_hash {
                            certificate_valid = Some(actual_certificate_hash == *expected_certificate_hash);
                        } else {
                            certificate_valid = Some(false); // No expected hash recorded
                        }
                    }
                    Err(_) => {
                        certificate_valid = Some(false); // Could not hash certificate
                    }
                }
            } else {
                certificate_valid = Some(false); // Certificate file not found
            }
        }
        
        // Overall validity includes certificate validity
        let overall_valid = invalid_files == 0 && certificate_valid.unwrap_or(true);

        Ok(VerificationReport {
            metadata_file_hash,
            total_files,
            valid_files,
            invalid_files,
            results,
            metadata_file_valid: true, // We successfully read it, so it's valid
            certificate_valid,
            certificate_hash,
            overall_valid,
        })
    }

    /// Verifies metadata file with progress reporting
    pub fn verify_metadata_file_with_progress(
        &self,
        metadata_path: &PathBuf,
        base_folder: &PathBuf,
    ) -> std::io::Result<VerificationReport> {
        // Read and parse the metadata file
        let metadata_content = fs::read_to_string(metadata_path)?;
        let metadata: Metadata = serde_json::from_str(&metadata_content)?;

        // Fingerprint the metadata file itself
        let metadata_file_hash = self.fingerprint_metadata_file(metadata_path)?;

        let mut results = Vec::new();
        let mut valid_files = 0;
        let mut invalid_files = 0;
        let total_files = metadata.artwork_files.len();

        // Verify each file with progress reporting
        for (index, artwork_file) in metadata.artwork_files.iter().enumerate() {
            let file_path = base_folder.join(&artwork_file.file_name);
            
            // Report progress
            match &self.progress_callback {
                ProgressCallback::Cli(callback) => {
                    callback(format!("Verifying file: {} ({}/{})", artwork_file.file_name, index + 1, total_files));
                }
                ProgressCallback::Gui(callback) => {
                    let overall_progress = index as f32 / total_files as f32;
                    callback(artwork_file.file_name.clone(), 0.0, overall_progress);
                }
                ProgressCallback::None => {}
            }
            
            let verification_result = if file_path.exists() {
                // Hash the actual file
                match self.hasher.hash_file(&file_path.to_string_lossy()) {
                    Ok(actual_hash) => {
                        let is_valid = actual_hash == artwork_file.file_hash;
                        if is_valid {
                            valid_files += 1;
                        } else {
                            invalid_files += 1;
                        }
                        
                        // Report completion
                        match &self.progress_callback {
                            ProgressCallback::Cli(callback) => {
                                callback(format!("Verified: {} - {}", 
                                    artwork_file.file_name, 
                                    if is_valid { "VALID" } else { "INVALID" }
                                ));
                            }
                            ProgressCallback::Gui(callback) => {
                                let overall_progress = (index + 1) as f32 / total_files as f32;
                                callback(artwork_file.file_name.clone(), 1.0, overall_progress);
                            }
                            ProgressCallback::None => {}
                        }
                        
                        VerificationResult {
                            file_name: artwork_file.file_name.clone(),
                            expected_hash: artwork_file.file_hash.clone(),
                            actual_hash,
                            is_valid,
                            error: None,
                        }
                    }
                    Err(e) => {
                        invalid_files += 1;
                        
                        // Report error
                        match &self.progress_callback {
                            ProgressCallback::Cli(callback) => {
                                callback(format!("Error verifying: {} - {}", artwork_file.file_name, e));
                            }
                            ProgressCallback::Gui(callback) => {
                                let overall_progress = (index + 1) as f32 / total_files as f32;
                                callback(artwork_file.file_name.clone(), 1.0, overall_progress);
                            }
                            ProgressCallback::None => {}
                        }
                        
                        VerificationResult {
                            file_name: artwork_file.file_name.clone(),
                            expected_hash: artwork_file.file_hash.clone(),
                            actual_hash: String::new(),
                            is_valid: false,
                            error: Some(e.to_string()),
                        }
                    }
                }
            } else {
                invalid_files += 1;
                
                // Report missing file
                match &self.progress_callback {
                    ProgressCallback::Cli(callback) => {
                        callback(format!("Missing file: {}", artwork_file.file_name));
                    }
                    ProgressCallback::Gui(callback) => {
                        let overall_progress = (index + 1) as f32 / total_files as f32;
                        callback(artwork_file.file_name.clone(), 1.0, overall_progress);
                    }
                    ProgressCallback::None => {}
                }
                
                VerificationResult {
                    file_name: artwork_file.file_name.clone(),
                    expected_hash: artwork_file.file_hash.clone(),
                    actual_hash: String::new(),
                    is_valid: false,
                    error: Some("File not found".to_string()),
                }
            };
            
            results.push(verification_result);
        }

        // Verify certificate if it exists
        let mut certificate_valid = None;
        let mut certificate_hash = None;
        
        if let Some(certificate_path) = &metadata.certificate_of_authenticity {
            let certificate_full_path = base_folder.join(certificate_path.trim_start_matches("./"));
            
            // Report certificate verification
            match &self.progress_callback {
                ProgressCallback::Cli(callback) => {
                    callback(format!("Verifying certificate: {}", certificate_path));
                }
                ProgressCallback::Gui(callback) => {
                    callback("Certificate".to_string(), 0.0, 1.0);
                }
                ProgressCallback::None => {}
            }
            
            if certificate_full_path.exists() {
                match self.hasher.hash_file(&certificate_full_path.to_string_lossy()) {
                    Ok(actual_certificate_hash) => {
                        certificate_hash = Some(actual_certificate_hash.clone());
                        if let Some(expected_certificate_hash) = &metadata.certificate_hash {
                            certificate_valid = Some(actual_certificate_hash == *expected_certificate_hash);
                            
                            // Report certificate result
                            match &self.progress_callback {
                                ProgressCallback::Cli(callback) => {
                                    callback(format!("Certificate verified: {} - {}", 
                                        certificate_path, 
                                        if certificate_valid.unwrap() { "VALID" } else { "INVALID" }
                                    ));
                                }
                                ProgressCallback::Gui(callback) => {
                                    callback("Certificate".to_string(), 1.0, 1.0);
                                }
                                ProgressCallback::None => {}
                            }
                        } else {
                            certificate_valid = Some(false); // No expected hash recorded
                            
                            // Report missing expected hash
                            match &self.progress_callback {
                                ProgressCallback::Cli(callback) => {
                                    callback(format!("Certificate error: No expected hash recorded for {}", certificate_path));
                                }
                                ProgressCallback::Gui(callback) => {
                                    callback("Certificate".to_string(), 1.0, 1.0);
                                }
                                ProgressCallback::None => {}
                            }
                        }
                    }
                    Err(e) => {
                        certificate_valid = Some(false); // Could not hash certificate
                        
                        // Report certificate error
                        match &self.progress_callback {
                            ProgressCallback::Cli(callback) => {
                                callback(format!("Certificate error: Could not hash {} - {}", certificate_path, e));
                            }
                            ProgressCallback::Gui(callback) => {
                                callback("Certificate".to_string(), 1.0, 1.0);
                            }
                            ProgressCallback::None => {}
                        }
                    }
                }
            } else {
                certificate_valid = Some(false); // Certificate file not found
                
                // Report missing certificate
                match &self.progress_callback {
                    ProgressCallback::Cli(callback) => {
                        callback(format!("Certificate not found: {}", certificate_path));
                    }
                    ProgressCallback::Gui(callback) => {
                        callback("Certificate".to_string(), 1.0, 1.0);
                    }
                    ProgressCallback::None => {}
                }
            }
        }
        
        // Overall validity includes certificate validity
        let overall_valid = invalid_files == 0 && certificate_valid.unwrap_or(true);

        Ok(VerificationReport {
            metadata_file_hash,
            total_files,
            valid_files,
            invalid_files,
            results,
            metadata_file_valid: true, // We successfully read it, so it's valid
            certificate_valid,
            certificate_hash,
            overall_valid,
        })
    }
} 