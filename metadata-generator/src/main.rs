use clap::Parser;
use std::{fs::File, io::{BufWriter, Write, Error}};


pub mod constants;
pub mod hasher;
pub mod metadata_generator;

use metadata_generator::{MetadataGenerator, Metadata};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'p', long = "path")]
    path: std::path::PathBuf,

    #[arg(short = 'm', long = "metadata")]
    metadata: Option<std::path::PathBuf>,
}



fn check_for_metadata_file() -> Result<(), Error> {
    let args = Cli::parse();

    if let Some(metadata_path) = &args.metadata {
        let metadata_file = File::open(metadata_path)?;
        let metadata: Metadata = serde_json::from_reader(metadata_file)?;
        println!("Metadata: {:?}", metadata);

        // Serialize it to a JSON string.
        let m = metadata;
        // make a new file with the name as the title
        let file = File::create(&(m.title.to_owned() + "_metadata.json"))?;

        let mut writer = BufWriter::new(file);
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

        // Use the unified metadata generator
        let generator = MetadataGenerator::new_cli()
            .with_progress_callback(metadata_generator::ProgressCallback::Cli(Box::new(|message| {
                println!("{}", message);
            })));

        let output_path = generator.generate_metadata(&args.path, &metadata)?;
        println!("Metadata saved to: {}", output_path.display());
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    // get metadata from user
    check_for_metadata_file()?;

    Ok(())
}   

