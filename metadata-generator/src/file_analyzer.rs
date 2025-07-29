use std::path::Path;
use anyhow::Result;
use image::GenericImageView;
use ffmpeg_next as ffmpeg;
use crate::constants::get_file_type;

pub struct FileMetadata {
    pub resolution: Option<String>,
    pub duration: Option<String>,
    pub format: String,
}

pub fn analyze_file(file_path: &Path) -> Result<FileMetadata> {
    let file_type = get_file_type(file_path);
    let format = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_uppercase();

    match file_type {
        "video" => analyze_video_file(file_path, &format),
        "audio" => analyze_audio_file(file_path, &format),
        "image" => analyze_image_file(file_path),
        _ => Ok(FileMetadata {
            resolution: None,
            duration: None,
            format,
        }),
    }
}

fn analyze_video_file(file_path: &Path, format: &str) -> Result<FileMetadata> {
    // Initialize ffmpeg
    ffmpeg::init()?;
    
    let mut resolution = None;
    let mut duration = None;
    
    // Open the input file
    if let Ok(input) = ffmpeg::format::input(file_path) {
        // Get the best video stream
        if let Some(stream) = input.streams().best(ffmpeg::media::Type::Video) {
            let context = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
            
            // Extract resolution
            if let Ok(decoder) = context.decoder().video() {
                let width = decoder.width();
                let height = decoder.height();
                resolution = Some(format!("{}x{}", width, height));
            }
            
            // Extract duration
            let duration_ts = stream.duration();
            if duration_ts != ffmpeg::ffi::AV_NOPTS_VALUE {
                let time_base = stream.time_base();
                let duration_secs = duration_ts as f64 * time_base.numerator() as f64 / time_base.denominator() as f64;
                
                let minutes = (duration_secs / 60.0) as u64;
                let seconds = (duration_secs % 60.0) as u64;
                duration = Some(format!("{}:{:02}", minutes, seconds));
            }
        }
    }
    
    Ok(FileMetadata {
        resolution,
        duration,
        format: format.to_uppercase(),
    })
}

fn analyze_audio_file(file_path: &Path, format: &str) -> Result<FileMetadata> {
    // Initialize ffmpeg
    ffmpeg::init()?;
    
    let mut duration = None;
    
    // Open the input file
    if let Ok(input) = ffmpeg::format::input(file_path) {
        // Get the best audio stream
        if let Some(stream) = input.streams().best(ffmpeg::media::Type::Audio) {
            // Extract duration
            let duration_ts = stream.duration();
            if duration_ts != ffmpeg::ffi::AV_NOPTS_VALUE {
                let time_base = stream.time_base();
                let duration_secs = duration_ts as f64 * time_base.numerator() as f64 / time_base.denominator() as f64;
                
                let minutes = (duration_secs / 60.0) as u64;
                let seconds = (duration_secs % 60.0) as u64;
                duration = Some(format!("{}:{:02}", minutes, seconds));
            }
        }
    }
    
    Ok(FileMetadata {
        resolution: None, // Audio files don't have resolution
        duration,
        format: format.to_uppercase(),
    })
}

pub fn analyze_image_file(file_path: &Path) -> Result<FileMetadata> {
    // Try to analyze image files using the image crate
    if let Ok(img) = image::open(file_path) {
        let dimensions = img.dimensions();
        let resolution = Some(format!("{}x{}", dimensions.0, dimensions.1));
        
        let format = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_uppercase();

        return Ok(FileMetadata {
            resolution,
            duration: None, // Images don't have duration
            format,
        });
    }

    // Fallback for image files
    let format = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_uppercase();

    Ok(FileMetadata {
        resolution: None,
        duration: None,
        format,
    })
} 