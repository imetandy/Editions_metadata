pub mod hasher {
    use blake3::Hasher;
    use indicatif::{ProgressBar, ProgressStyle};
    use std::{fs::File, sync::{atomic::{AtomicBool, AtomicU64, Ordering}, Arc}, thread, time::Duration};
    use memmap::Mmap;

pub fn hash_file(path: &str) -> std::io::Result<String> {
    // The file
    // let file = Path::new(path);
    let file = File::open(&path)?;
    let file_size = file.metadata()?.len();

    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));

    let mmap = unsafe { Mmap::map(&file)? };

    // Shared atomic variable to track progress
    let bytes_hashed = Arc::new(AtomicU64::new(0));
    let bytes_hashed_clone = Arc::clone(&bytes_hashed);

    // Flag to signal the end of hashing
    let done = Arc::new(AtomicBool::new(false));
    let done_clone = Arc::clone(&done);

    // Start a thread to update the progress bar
    let pb_thread = thread::spawn(move || {
        while !done_clone.load(Ordering::Relaxed) {
            pb.set_position(bytes_hashed_clone.load(Ordering::Relaxed));
            thread::sleep(Duration::from_millis(100));
        }
        pb.finish_with_message("Hashing complete");
    });

    // Use a separate thread for the actual hashing to not block the progress bar updates
    let hash_thread = thread::spawn(move || {
        let mut hasher = Hasher::new();
        let chunk_size = 8192; // You can adjust this size based on your needs

        for chunk in mmap.chunks(chunk_size) {
            hasher.update(chunk);
            bytes_hashed.fetch_add(chunk.len() as u64, Ordering::Relaxed);
        }

        let hash = hasher.finalize();
        done.store(true, Ordering::Relaxed);
        hash.to_hex().to_string()
    });

    let hash_hex = hash_thread.join().unwrap();
    pb_thread.join().unwrap();

    Ok(hash_hex)
}
}