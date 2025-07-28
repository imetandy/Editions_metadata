use blake3::Hasher;
use std::{fs::File, sync::{atomic::{AtomicBool, AtomicU64, Ordering}, Arc}, thread};
use memmap::Mmap;
use indicatif::{ProgressBar, ProgressStyle};

#[allow(dead_code)]
pub enum ProgressReporter {
    Cli(ProgressBar),
    Gui(Arc<AtomicU64>, Arc<AtomicU64>), // file_progress, file_total_size
    None,
}

pub struct UnifiedHasher {
    progress_reporter: ProgressReporter,
    current_file: Arc<std::sync::Mutex<String>>,
    is_complete: Arc<AtomicBool>,
    hash_result: Arc<std::sync::Mutex<Option<Result<String, std::io::Error>>>>,
}

impl UnifiedHasher {
    #[allow(dead_code)]
    pub fn new_cli() -> Self {
        Self {
            progress_reporter: ProgressReporter::None,
            current_file: Arc::new(std::sync::Mutex::new(String::new())),
            is_complete: Arc::new(AtomicBool::new(false)),
            hash_result: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    pub fn new_gui() -> Self {
        Self {
            progress_reporter: ProgressReporter::Gui(
                Arc::new(AtomicU64::new(0)),
                Arc::new(AtomicU64::new(0))
            ),
            current_file: Arc::new(std::sync::Mutex::new(String::new())),
            is_complete: Arc::new(AtomicBool::new(false)),
            hash_result: Arc::new(std::sync::Mutex::new(None)),
        }
    }

    #[allow(dead_code)]
    pub fn hash_file(&self, path: &str) -> std::io::Result<String> {
        let file = File::open(path)?;
        let file_size = file.metadata()?.len();

        // Update current file name
        if let Ok(mut current_file) = self.current_file.lock() {
            *current_file = std::path::Path::new(path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
        }

        // Setup progress reporting based on type
        match &self.progress_reporter {
            ProgressReporter::Cli(_) => {
                // CLI progress bar setup
                let pb = ProgressBar::new(file_size);
                pb.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                    .progress_chars("#>-"));
                
                let mmap = unsafe { Mmap::map(&file)? };
                let mut hasher = Hasher::new();
                let chunk_size = 8192;

                for chunk in mmap.chunks(chunk_size) {
                    hasher.update(chunk);
                    pb.inc(chunk.len() as u64);
                }

                pb.finish_with_message("Hashing complete");
                let hash = hasher.finalize();
                Ok(hash.to_hex().to_string())
            }
            ProgressReporter::Gui(file_progress, file_total_size) => {
                // GUI progress tracking
                file_total_size.store(file_size, Ordering::Relaxed);
                file_progress.store(0, Ordering::Relaxed);

                let mmap = unsafe { Mmap::map(&file)? };
                let mut hasher = Hasher::new();
                let chunk_size = 8192;

                for chunk in mmap.chunks(chunk_size) {
                    hasher.update(chunk);
                    file_progress.fetch_add(chunk.len() as u64, Ordering::Relaxed);
                }

                let hash = hasher.finalize();
                Ok(hash.to_hex().to_string())
            }
            ProgressReporter::None => {
                // No progress reporting (fallback)
                let mmap = unsafe { Mmap::map(&file)? };
                let mut hasher = Hasher::new();
                let chunk_size = 8192;

                for chunk in mmap.chunks(chunk_size) {
                    hasher.update(chunk);
                }

                let hash = hasher.finalize();
                Ok(hash.to_hex().to_string())
            }
        }
    }

    pub fn hash_file_async(&self, path: &str) {
        let path = path.to_string();
        let current_file_clone = Arc::clone(&self.current_file);
        let is_complete_clone = Arc::clone(&self.is_complete);
        let hash_result_clone = Arc::clone(&self.hash_result);

        // Clone progress reporter for async use
        let progress_reporter = match &self.progress_reporter {
            ProgressReporter::Gui(file_progress, file_total_size) => {
                ProgressReporter::Gui(
                    Arc::clone(file_progress),
                    Arc::clone(file_total_size)
                )
            }
            _ => ProgressReporter::None,
        };

        // Reset state for new file
        if let ProgressReporter::Gui(file_progress, _) = &self.progress_reporter {
            file_progress.store(0, Ordering::Relaxed);
        }
        self.is_complete.store(false, Ordering::Relaxed);
        if let Ok(mut result) = self.hash_result.lock() {
            *result = None;
        }

        thread::spawn(move || {
            // Update current file name
            if let Ok(mut current_file) = current_file_clone.lock() {
                *current_file = std::path::Path::new(&path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
            }

            // Open file and get size
            let file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    if let Ok(mut result) = hash_result_clone.lock() {
                        *result = Some(Err(e));
                    }
                    is_complete_clone.store(true, Ordering::Relaxed);
                    return;
                }
            };

            let file_size = match file.metadata() {
                Ok(m) => m.len(),
                Err(e) => {
                    if let Ok(mut result) = hash_result_clone.lock() {
                        *result = Some(Err(e));
                    }
                    is_complete_clone.store(true, Ordering::Relaxed);
                    return;
                }
            };

            // Setup progress tracking
            match &progress_reporter {
                ProgressReporter::Gui(_file_progress, file_total_size) => {
                    file_total_size.store(file_size, Ordering::Relaxed);
                }
                _ => {}
            }

            // Memory map the file
            let mmap = match unsafe { Mmap::map(&file) } {
                Ok(m) => m,
                Err(e) => {
                    if let Ok(mut result) = hash_result_clone.lock() {
                        *result = Some(Err(e));
                    }
                    is_complete_clone.store(true, Ordering::Relaxed);
                    return;
                }
            };

            // Hash the file
            let mut hasher = Hasher::new();
            let chunk_size = 8192;

            for chunk in mmap.chunks(chunk_size) {
                hasher.update(chunk);
                
                // Update progress
                match &progress_reporter {
                    ProgressReporter::Gui(file_progress, _) => {
                        file_progress.fetch_add(chunk.len() as u64, Ordering::Relaxed);
                    }
                    _ => {}
                }
            }

            let hash = hasher.finalize();
            let hash_hex = hash.to_hex().to_string();

            // Store result and mark as complete
            if let Ok(mut result) = hash_result_clone.lock() {
                *result = Some(Ok(hash_hex));
            }
            is_complete_clone.store(true, Ordering::Relaxed);
        });
    }

    pub fn get_file_progress(&self) -> f32 {
        match &self.progress_reporter {
            ProgressReporter::Gui(file_progress, file_total_size) => {
                let file_total = file_total_size.load(Ordering::Relaxed);
                if file_total == 0 {
                    0.0
                } else {
                    file_progress.load(Ordering::Relaxed) as f32 / file_total as f32
                }
            }
            _ => 0.0,
        }
    }

    pub fn get_result(&self) -> Option<Result<String, std::io::Error>> {
        if let Ok(result) = self.hash_result.lock() {
            match &*result {
                Some(Ok(hash)) => Some(Ok(hash.clone())),
                Some(Err(e)) => Some(Err(std::io::Error::new(e.kind(), e.to_string()))),
                None => None,
            }
        } else {
            None
        }
    }
} 