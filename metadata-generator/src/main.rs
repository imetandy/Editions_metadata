use clap::{Arg, Command, Parser};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::{fs::{self, File}, io::{BufWriter, Write, Error}};

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
    metadata: Option<Option<std::path::PathBuf>>,
}

fn parse_metadata_file () 
    -> Result<Metadata, Error> {
        let args = Cli::parse();
        let metadata_file = File::open(args.metadata.as_ref().unwrap())?;
        let metadata: Metadata = serde_json::from_reader(metadata_file)?;
        println!("Metadata: {:?}", metadata);

        // Serialize it to a JSON string.
        let m = metadata;
        // make a new file with the name as the title
        let file = File::create(&(m.title.to_owned() + "_metadata.json"));

        let mut writer = BufWriter::new(file.unwrap());
        serde_json::to_writer_pretty(&mut writer, &m)?;
        let _ = writer.flush();

        Ok(Metadata {
            date_created: m.date_created,
            title: m.title,
            creator: m.creator,
            description: m.description,
            video_files: m.video_files,
            audio_files: m.audio_files,
        })
}

fn check_for_metadata_file() -> Result<(), Error> {

    let args = Cli::parse();

    if args.metadata != None {
        let metadata_file = File::open(args.metadata.as_ref().unwrap())?;
        let metadata: Metadata = serde_json::from_reader(metadata_file)?;
        println!("Metadata: {:?}", metadata);

        // Serialize it to a JSON string.
        let m = metadata;
        // make a new file with the name as the title
        let file = File::create(&(m.title.to_owned() + "_metadata.json"));

        let mut writer = BufWriter::new(file.unwrap());
        serde_json::to_writer_pretty(&mut writer, &m)?;
        let _ = writer.flush();
        
        
    } else {
        let mut metadata = Metadata {
            date_created: "".to_string(),
            title: "".to_string(),
            creator: "".to_string(),
            description: "".to_string(),
            video_files: Vec::new(),
            audio_files: Vec::new(),
        };

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

        // Add user input to metadata struct
        metadata.date_created = user_input[0].clone().trim().to_string();
        metadata.title = user_input[1].clone().trim().to_string();
        metadata.creator = user_input[2].clone().trim().to_string();
        metadata.description = user_input[3].clone().trim().to_string();

        // Serialize it to a JSON string.
        let m = metadata;
        // make a new file with the name as the title
        let file = File::create(&(m.title.to_owned() + "_metadata.json"));

        let mut writer = BufWriter::new(file.unwrap());
        serde_json::to_writer_pretty(&mut writer, &m)?;
        let _ = writer.flush();


    }
    Ok(())
}

fn gather_files(path: &str, metadata_collector: &mut Metadata) -> Result<(), Error> {
    let directory = fs::read_dir(path).unwrap();

    for entry in directory {
        match entry {
            Ok(entry) => {
                let file_path = entry.path();
                // Convert path to a string
                let file_path_str = file_path.to_string_lossy(); 
                let hash_result = hasher::hash_file(&file_path_str);

                let hash = match hash_result {
                    Ok(hash) => {
                        println!("File hash for {}: {}", file_path_str, hash);
                        hash
                    }
                    Err(e) => {
                        eprintln!("Error hashing file {}: {}", file_path_str, e);
                        continue;
                    }
                };
                let file_metadata = MediaFile {
                    file_name: file_path.file_name().unwrap().to_string_lossy().to_string(),
                    file_hash: hash,
                    file_size: file_path.metadata().unwrap().len(),
                    file_type: file_path.extension().unwrap().to_string_lossy().to_string(),
                    file_path: file_path.to_string_lossy().to_string(),
                };
                metadata_collector.video_files.push(file_metadata);
            }
            Err(e) => eprintln!("Error reading directory entry: {}", e),
        }
    }
    Ok(())
}


fn main() -> Result<(), Error> {

    // get metadata from user
    metadata_builder()?;


    // read in user metadata
    // users can either supply a metadata file, or we can generate one for them with the data they input on cmd

    // logger initialization
//     env_logger::init();
//     info!("starting up");
//     warn!("oops, nothing implemented!");

//     let args = Cli::parse();

//     let path = args.path;

//     let files = fs::read_dir(path.clone()).unwrap();

//     for file in files {
//         println!("Name: {}", file.unwrap().path().display());
//     }

//     let directory = fs::read_dir(path).unwrap();

//     let mut metadata_collector = Metadata {
//         date_created: "".to_string(),
//         title: "".to_string(),
//         creator: "".to_string(),
//         description: "".to_string(),
//         video_files: Vec::new(),
//         audio_files: Vec::new(),
//     };
    
//     for entry in directory {
//         match entry {
//             Ok(entry) => {
//                 let file_path = entry.path();
//                 // Convert path to a string
//                 let file_path_str = file_path.to_string_lossy(); 
//                 let hash_result = hasher::hash_file(&file_path_str);

//                 let hash = match hash_result {
//                     Ok(hash) => {
//                         println!("File hash for {}: {}", file_path_str, hash);
//                         hash
//                     }
//                     Err(e) => {
//                         eprintln!("Error hashing file {}: {}", file_path_str, e);
//                         continue;
//                     }
//                 };
//                 let file_metadata = MediaFile {
//                     file_name: file_path.file_name().unwrap().to_string_lossy().to_string(),
//                     file_hash: hash.to_string(),
//                     file_size: file_path.metadata().unwrap().len(),
//                     file_type: file_path.extension().unwrap().to_string_lossy().to_string(),
//                     file_path: file_path.to_string_lossy().to_string(),
//                 };
//                 metadata_collector.video_files.push(file_metadata);
//             }
//             Err(e) => eprintln!("Error reading directory entry: {}", e),
//         }
//     }



//     let mut user_input = Vec::new();
    
//     let user_date: String = "".to_string();
//     let user_title: String = "".to_string();
//     let user_creator: String = "".to_string();
//     let user_description: String = "".to_string();
    
//     user_input.push(user_date);
//     user_input.push(user_title);
//     user_input.push(user_creator);
//     user_input.push(user_description);



//     for (i, message) in user_input.iter_mut().enumerate() {

    match i {
        0 => {
            println!("Enter the date the artwork was created (YYYY-MM-DD): ");
        }
        1 => {
            println!("Enter the title of the artwork: ");
        }
        2 => {
            println!("Enter the creator of the artwork: ");
        }
        3 => {
            println!("Enter a description of the artwork: ");
        }
        _ => {
               break;
        }
    }

//     match std::io::stdin().read_line(message) {
//         Ok(_) => {

//             println!("You entered: {}", message);
//         }
//         Err(e) => {
//             eprintln!("Error reading user input: {}", e);
//         }
//     }
// }
//     // Add user input to metadata struct
//     metadata_collector.date_created = user_input[0].clone().trim().to_string();
//     metadata_collector.title = user_input[1].clone().trim().to_string();
//     metadata_collector.creator = user_input[2].clone().trim().to_string();
//     metadata_collector.description = user_input[3].clone().trim().to_string();
    
    
//     // Serialize it to a JSON string.
//     let m = metadata_collector;
//     // make a new file with the name as the title
//     let file = File::create(&(m.title.to_owned() + "_metadata.json"));

//     let mut writer = BufWriter::new(file.unwrap());
//     serde_json::to_writer_pretty(&mut writer, &m)?;
//     let _ = writer.flush();



    // TODO: Progrdess bar, will implement later
    // let pb = indicatif::ProgressBar::new(100);
    // for i in 0..100 {
    
    //     pb.println(format!("[+] finished #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");

    Ok(())

}   

