use clap::Parser;
use log::{info, warn};
//use serde_json::json;
//use serde::{Deserialize, Serialize};
use std::fs;

pub mod blake3_hash;
use blake3_hash::hasher;

// #[derive(Serialize, Deserialize)]
// struct Metadata {
//     date_created: String,
//     title: String,
//     creator: String,
//     description: String,
//     video_files: Vec<MediaFile>,
//     audio_files: Vec<MediaFile>,
// }

// #[derive(Serialize, Deserialize)]
// struct MediaFile {
//     file_name: String,
//     file_hash: String,
//     file_size: u64,
//     file_type: String,
//     file_path: String,
// }

#[derive(Parser)]
struct Cli {
    #[arg(short = 'p', long = "path")]
    path: std::path::PathBuf,

    #[arg(short = 'm', long = "metadata")]
    metadata: std::path::PathBuf,
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

                match hasher::hash_file(&file_path_str) {
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

