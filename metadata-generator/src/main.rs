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
    // read in user metadata
    // users can either supply a metadata file, or we can generate one for them with the data they input on cmd

    let mut user_input = Vec::new();
    
    let user_date: String = "".to_string();
    let user_title: String = "".to_string();
    let user_creator: String = "".to_string();
    let user_description: String = "".to_string();
    
    user_input.push(user_date);
    user_input.push(user_title);
    user_input.push(user_creator);
    user_input.push(user_description);



    for (i, message) in user_input.iter_mut().enumerate() {

    match i {
        0 => {
            println!("Enter the date the video was created: ");
        }
        1 => {
            println!("Enter the title of the video: ");
        }
        2 => {
            println!("Enter the creator of the video: ");
        }
        3 => {
            println!("Enter a description of the video: ");
        }
        _ => {
               break;
        }
    }

    match std::io::stdin().read_line(message) {
        Ok(_) => {

            println!("You entered: {}", message);
        }
        Err(e) => {
            eprintln!("Error reading user input: {}", e);
        }
    }
}

    let metadata_collector = Metadata {
        date_created: user_input[0].clone().trim().to_string(),
        title: user_input[1].clone().trim().to_string(),
        creator: user_input[2].clone().trim().to_string(),
        description: user_input[3].clone().trim().to_string(),
        video_files: Vec::new(),
        audio_files: Vec::new(),
    };




    // initialize metadata

    // let metadata = r#"
    // {
    //     "date_created": "2021-09-01",
    //     "title": "My Video",
    //     "creator": "Me",
    //     "description": "A video I made",
    //     "video_files": [
    //         {
    //             "file_name": "video.mp4",
    //             "file_hash": "1234567890abcdef",
    //             "file_size": 1234567890,
    //             "file_type": "video/mp4",
    //             "file_path": "/path/to/video.mp4"
    //         }
    //     ],
    //     "audio_files": [
    //         {
    //             "file_name": "audio.mp3",
    //             "file_hash": "abcdef1234567890",
    //             "file_size": 9876543210,
    //             "file_type": "audio/mp3",
    //             "file_path": "/path/to/audio.mp3"
    //         }
    //     ]
    // }"#;
    //let m: Metadata = serde_json::from_str(metadata)?;
    let m = metadata_collector;
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

