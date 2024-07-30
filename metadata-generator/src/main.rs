use clap::Parser;
use log::{info, warn};

use std::fs::{self, File};
use std::io;
use blake3::Hasher;
use std::io::{BufReader, Read};


#[derive(Parser)]
struct Cli {
    #[arg(short = 'p', long = "path")]
    path: std::path::PathBuf,

    #[arg(short = 'm', long = "metadata")]
    metadata: std::path::PathBuf,
}


fn hash_file(path: &str) -> io::Result<String> {
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


fn main() {
    // logger initialization
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    let args = Cli::parse();

    let path = args.path;

    let metadata = std::fs::read_to_string(&args.metadata).expect("could not read file");
    println!("metadata: {}", metadata);

    let files = fs::read_dir(path.clone()).unwrap();

    for file in files {
        println!("Name: {}", file.unwrap().path().display());
    }

    let directory = fs::read_dir(path).unwrap();

    for entry in directory {
        match entry {
            Ok(entry) => {
                let file_path = entry.path();
                let file_path_str = file_path.to_string_lossy(); // Convert path to a string

                match hash_file(&file_path_str) {
                    Ok(hash) => println!("File hash for {}: {}", file_path_str, hash),
                    Err(e) => eprintln!("Error hashing file {}: {}", file_path_str, e),
                }
            }
            Err(e) => eprintln!("Error reading directory entry: {}", e),
        }
    }
    // TODO: Progrdess bar, will implement later
    // let pb = indicatif::ProgressBar::new(100);
    // for i in 0..100 {
    
    //     pb.println(format!("[+] finished #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

}

