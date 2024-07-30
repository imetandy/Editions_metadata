pub mod hasher {
    use blake3::Hasher;
    use std::{fs::File, io::{self, BufReader, Read}};

pub fn hash_file(path: &str) -> io::Result<String> {
    // Open the file
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Create a Blake3 hasher
    let mut hasher = Hasher::new();

    // Read the file in chunks and feed it to the hasher
    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // End of file
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Finalize the hash and return it as a hexadecimal string
    let hash = hasher.finalize();
    Ok(hash.to_hex().to_string())
}
}