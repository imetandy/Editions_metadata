pub mod hasher {
    use blake3::Hasher;
    use std::{io::{self}, path::Path};
    use std::time::Instant;

pub fn hash_file(path: &str) -> io::Result<String> {
    // The file
    let file = Path::new(path);
    // Create a Blake3 hasher
    let mut hasher = Hasher::new();


    // Start measuring time
    let start = Instant::now();
    hasher.update_mmap_rayon(file)?;
    
    // Finalize the hash and return it as a hexadecimal string
    let hash = hasher.finalize();
    // Stop measuring time
    let duration = start.elapsed();
    println!("Hashing took {:?}", duration);
    Ok(hash.to_hex().to_string())
}
}