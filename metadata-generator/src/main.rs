use clap::Parser;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs::{self, File}, io::{BufWriter, Write}};

pub mod blake3_hash;
use blake3_hash::hasher;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Metadata {
    date_created: String,
    title: String,
    creator: String,
    description: String,
    video_files: Vec<MediaFile>,
    audio_files: Vec<MediaFile>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MediaFile {
    file_name: String,
    file_hash: String,
    file_size: u64,
    file_type: String,
    file_path: String,
}

#[derive(Parser)]
struct Cli {
    #[arg(short = 'p', long = "path")]
    path: std::path::PathBuf,

    #[arg(short = 'm', long = "metadata")]
    metadata: std::path::PathBuf,
}

fn main() -> Result<()> {
    // initialize metadata

    let metadata = r#"
    {
        "date_created": "2021-09-01",
        "title": "My Video",
        "creator": "Me",
        "description": "A video I made",
        "video_files": [
            {
                "file_name": "video.mp4",
                "file_hash": "1234567890abcdef",
                "file_size": 1234567890,
                "file_type": "video/mp4",
                "file_path": "/path/to/video.mp4"
            }
        ],
        "audio_files": [
            {
                "file_name": "audio.mp3",
                "file_hash": "abcdef1234567890",
                "file_size": 9876543210,
                "file_type": "audio/mp3",
                "file_path": "/path/to/audio.mp3"
            }
        ]
    }"#;
    let m: Metadata = serde_json::from_str(metadata)?;

    // Do things just like with any other Rust data structure.
    println!("Data: {}", m.description);

    let file = File::create("metadata.json");

    let mut writer = BufWriter::new(file.unwrap());
    serde_json::to_writer_pretty(&mut writer, &m)?;
    let _ = writer.flush();


    // logger initialization
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    let args = Cli::parse();

    let path = args.path;

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

    Ok(())

}   

