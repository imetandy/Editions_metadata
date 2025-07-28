use eframe::{egui, App, Frame};
use rfd::FileDialog;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[path = "../blake3_hash.rs"]
mod blake3_hash;
#[path = "../constants.rs"]
mod constants;
use blake3_hash::hasher;
use constants::get_file_type;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct MediaFile {
    file_name: String,
    file_hash: String,
    file_size: u64,
    file_type: String,
    file_path: String,
}

#[derive(Serialize, Deserialize, Default)]
struct Metadata {
    date_created: String,
    title: String,
    creator: String,
    description: String,
    video_files: Vec<MediaFile>,
    audio_files: Vec<MediaFile>,
}

struct GuiApp {
    folder: Option<PathBuf>,
    date: String,
    title: String,
    creator: String,
    description: String,
    status: String,
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            folder: None,
            date: String::new(),
            title: String::new(),
            creator: String::new(),
            description: String::new(),
            status: String::new(),
        }
    }
}

impl App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Metadata Generator");
            if ui.button("Browse for folder").clicked() {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    self.folder = Some(dir);
                }
            }
            if let Some(folder) = &self.folder {
                ui.label(format!("Folder: {}", folder.display()));
            }

            ui.separator();
            ui.label("Date created");
            ui.text_edit_singleline(&mut self.date);
            ui.label("Title");
            ui.text_edit_singleline(&mut self.title);
            ui.label("Creator");
            ui.text_edit_singleline(&mut self.creator);
            ui.label("Description");
            ui.text_edit_multiline(&mut self.description);

            if ui.button("Generate metadata").clicked() {
                match self.generate() {
                    Ok(p) => self.status = format!("Saved to {}", p.display()),
                    Err(e) => self.status = format!("Error: {}", e),
                }
            }

            ui.separator();
            ui.label(&self.status);
        });
    }

}

impl GuiApp {
    fn generate(&self) -> std::io::Result<PathBuf> {
        let folder = self.folder.clone().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No folder selected"))?;
        let entries = fs::read_dir(&folder)?;

        let mut metadata = Metadata {
            date_created: self.date.clone(),
            title: self.title.clone(),
            creator: self.creator.clone(),
            description: self.description.clone(),
            video_files: Vec::new(),
            audio_files: Vec::new(),
        };

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let path_str = path.to_string_lossy().to_string();
                let hash = hasher::hash_file(&path_str)?;
                let mf = MediaFile {
                    file_name: path.file_name().unwrap().to_string_lossy().to_string(),
                    file_hash: hash,
                    file_size: path.metadata()?.len(),
                    file_type: path.extension().unwrap_or_default().to_string_lossy().to_string(),
                    file_path: path_str,
                };
                
                // Automatically sort files based on their type
                match get_file_type(&path) {
                    "video" => metadata.video_files.push(mf),
                    "audio" => metadata.audio_files.push(mf),
                    _ => metadata.video_files.push(mf), // Default to video
                }
            }
        }

        let file_name = format!("{}_metadata.json", self.title.replace(' ', "_"));
        let output = folder.join(file_name);
        let file = File::create(&output)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &metadata)?;
        writer.flush()?;
        Ok(output)
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = GuiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Metadata Generator", native_options, Box::new(|_cc| Box::new(app)))
}
