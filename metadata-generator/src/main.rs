use clap::Parser;
use std::{fs::File, io::Error};


pub mod constants;
pub mod hasher;
pub mod metadata_generator;
pub mod file_analyzer;

use metadata_generator::{MetadataGenerator, Metadata};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'p', long = "path")]
    path: std::path::PathBuf,

    #[arg(short = 'm', long = "metadata")]
    metadata: Option<std::path::PathBuf>,

    #[arg(short = 'v', long = "verify")]
    verify: bool,

    #[arg(short = 'f', long = "metadata-file")]
    metadata_file: Option<std::path::PathBuf>,
}

fn verify_metadata() -> Result<(), Error> {
    let args = Cli::parse();
    
    let metadata_file = args.metadata_file.ok_or_else(|| {
        Error::new(std::io::ErrorKind::InvalidInput, "Metadata file path is required for verification")
    })?;
    
    println!("Verifying metadata file: {}", metadata_file.display());
    println!("Base folder: {}", args.path.display());
    
    let generator = MetadataGenerator::new_cli()
        .with_progress_callback(metadata_generator::ProgressCallback::Cli(Box::new(|message| {
            println!("{}", message);
        })));

    match generator.verify_metadata_file_with_progress(&metadata_file, &args.path) {
        Ok(report) => {
            println!("\n=== Verification Complete ===");
            println!("Metadata file hash: {}", report.metadata_file_hash);
            println!("Total files: {}", report.total_files);
            println!("Valid files: {}", report.valid_files);
            println!("Invalid files: {}", report.invalid_files);
            
            // Certificate verification
            if let Some(certificate_valid) = report.certificate_valid {
                if certificate_valid {
                    println!("✅ Certificate is valid!");
                } else {
                    println!("❌ Certificate is invalid!");
                }
                if let Some(certificate_hash) = &report.certificate_hash {
                    println!("Certificate hash: {}", certificate_hash);
                }
            } else {
                println!("ℹ️ No certificate found");
            }
            
            if report.overall_valid {
                println!("✅ All files and certificate are valid!");
            } else {
                println!("❌ Some files or certificate are invalid!");
                println!("\nDetailed Results:");
                for result in &report.results {
                    if result.is_valid {
                        println!("✅ {}", result.file_name);
                    } else {
                        println!("❌ {} - {}", result.file_name, result.error.as_deref().unwrap_or("Unknown error"));
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Verification failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

fn check_for_metadata_file() -> Result<(), Error> {
    let args = Cli::parse();

    if let Some(metadata_path) = &args.metadata {
        let metadata_file = File::open(metadata_path)?;
        let mut metadata: Metadata = serde_json::from_reader(metadata_file)?;
        println!("Metadata: {:?}", metadata);

        // Detect certificate of authenticity
        let certificate_of_authenticity = metadata_generator::detect_certificate_of_authenticity(&args.path);
        if certificate_of_authenticity.is_none() {
            println!("Warning: No certificate of authenticity PDF found in 'certificate' folder");
        } else {
            println!("Certificate of authenticity found: {:?}", certificate_of_authenticity);
        }
        metadata.certificate_of_authenticity = certificate_of_authenticity;

        // Use the unified metadata generator to process the folder
        let generator = MetadataGenerator::new_cli()
            .with_progress_callback(metadata_generator::ProgressCallback::Cli(Box::new(|message| {
                println!("{}", message);
            })));

        let output_path = generator.generate_metadata(&args.path, &metadata)?;
        println!("Metadata saved to: {}", output_path.display());
    } else {
        let mut metadata = Metadata {
            artwork_id: "".to_string(),
            artwork_title: "".to_string(),
            artwork_short_title: "".to_string(),
            artwork_creator: "".to_string(),
            year_of_creation: 2024,
            short_description: "".to_string(),
            long_description: "".to_string(),
            edition_number: 1,
            total_editions: 1,
            issue_date: "".to_string(),
            gallery: "".to_string(),
            keywords: Vec::new(),
            medium: Vec::new(),
            certificate_of_authenticity: None,
            certificate_hash: None,
            artwork_files: Vec::new(),
        };

        let mut user_input = Vec::new();
        let user_artwork_id: String = "".to_string();
        let user_artwork_title: String = "".to_string();
        let user_artwork_short_title: String = "".to_string();
        let user_artwork_creator: String = "".to_string();
        let user_year_of_creation: String = "".to_string();
        let user_short_description: String = "".to_string();
        let user_long_description: String = "".to_string();
        let user_edition_number: String = "".to_string();
        let user_total_editions: String = "".to_string();
        let user_issue_date: String = "".to_string();
        let user_gallery: String = "".to_string();
        let user_keywords: String = "".to_string();
        let user_medium: String = "".to_string();
        user_input.push(user_artwork_id);
        user_input.push(user_artwork_title);
        user_input.push(user_artwork_short_title);
        user_input.push(user_artwork_creator);
        user_input.push(user_year_of_creation);
        user_input.push(user_short_description);
        user_input.push(user_long_description);
        user_input.push(user_edition_number);
        user_input.push(user_total_editions);
        user_input.push(user_issue_date);
        user_input.push(user_gallery);
        user_input.push(user_keywords);
        user_input.push(user_medium);

        for (i, message) in user_input.iter_mut().enumerate() {
            match i {
                0 => {
                    println!("Enter the artwork ID: ");
                }
                1 => {
                    println!("Enter the artwork title: ");
                }
                2 => {
                    println!("Enter the artwork short title: ");
                }
                3 => {
                    println!("Enter the artwork creator: ");
                }
                4 => {
                    println!("Enter the year of creation: ");
                }
                5 => {
                    println!("Enter the short description: ");
                }
                6 => {
                    println!("Enter the long description: ");
                }
                7 => {
                    println!("Enter the edition number: ");
                }
                8 => {
                    println!("Enter the total editions: ");
                }
                9 => {
                    println!("Enter the issue date (YYYY-MM-DD): ");
                }
                10 => {
                    println!("Enter the gallery: ");
                }
                11 => {
                    println!("Enter keywords (comma-separated): ");
                }
                12 => {
                    println!("Enter medium (comma-separated): ");
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

        // Parse keywords and medium from comma-separated strings
        let keywords_vec: Vec<String> = user_input[11].split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        let medium_vec: Vec<String> = user_input[12].split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Detect certificate of authenticity
        let certificate_of_authenticity = metadata_generator::detect_certificate_of_authenticity(&args.path);
        if certificate_of_authenticity.is_none() {
            println!("Warning: No certificate of authenticity PDF found in 'certificate' folder");
        } else {
            println!("Certificate of authenticity found: {:?}", certificate_of_authenticity);
        }

        // Parse numeric fields
        let year_of_creation_int = user_input[4].trim().parse::<i32>().unwrap_or(2024);
        let edition_number_int = user_input[7].trim().parse::<i32>().unwrap_or(1);
        let total_editions_int = user_input[8].trim().parse::<i32>().unwrap_or(1);

        // Add user input to metadata struct
        metadata.artwork_id = user_input[0].clone().trim().to_string();
        metadata.artwork_title = user_input[1].clone().trim().to_string();
        metadata.artwork_short_title = user_input[2].clone().trim().to_string();
        metadata.artwork_creator = user_input[3].clone().trim().to_string();
        metadata.year_of_creation = year_of_creation_int;
        metadata.short_description = user_input[5].clone().trim().to_string();
        metadata.long_description = user_input[6].clone().trim().to_string();
        metadata.edition_number = edition_number_int;
        metadata.total_editions = total_editions_int;
        metadata.issue_date = user_input[9].clone().trim().to_string();
        metadata.gallery = user_input[10].clone().trim().to_string();
        metadata.keywords = keywords_vec;
        metadata.medium = medium_vec;
        metadata.certificate_of_authenticity = certificate_of_authenticity;

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
    let args = Cli::parse();
    
    if args.verify {
        verify_metadata()
    } else {
        check_for_metadata_file()
    }
}   

