use eframe::{egui, App, Frame};
use rfd::FileDialog;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;


#[path = "../constants.rs"]
mod constants;
#[path = "../hasher.rs"]
mod hasher;
#[path = "../metadata_generator.rs"]
mod metadata_generator;

use metadata_generator::{MetadataGenerator, Metadata, ProgressCallback};

#[derive(Clone)]
enum GenerationState {
    Idle,
    Processing { 
        current_file: String, 
        file_progress: f32,
        overall_progress: f32, 
        total_files: usize, 
        processed_files: usize 
    },
    Complete { output_path: PathBuf },
    Error { message: String },
}

struct GuiApp {
    folder: Option<PathBuf>,
    date: String,
    title: String,
    creator: String,
    description: String,
    status: String,
    generation_state: Arc<Mutex<GenerationState>>,
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
            generation_state: Arc::new(Mutex::new(GenerationState::Idle)),
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

            // Check if we can start generation
            let can_generate = self.folder.is_some() && !self.title.is_empty();
            
            if ui.add_enabled(can_generate, egui::Button::new("Generate metadata")).clicked() {
                self.start_generation();
            }

            ui.separator();
            
            // Show progress based on generation state
            if let Ok(state) = self.generation_state.lock() {
                match &*state {
                    GenerationState::Idle => {
                        ui.label(&self.status);
                    }
                    GenerationState::Processing { current_file, file_progress, overall_progress, total_files, processed_files } => {
                        ui.label(format!("Processing: {}/{} files", processed_files + 1, total_files));
                        ui.label(format!("Current file: {}", current_file));
                        ui.add_space(5.0);
                        
                        ui.label("File progress:");
                        ui.add(egui::ProgressBar::new(*file_progress).show_percentage());
                        
                        ui.add_space(5.0);
                        ui.label("Overall progress:");
                        ui.add(egui::ProgressBar::new(*overall_progress).show_percentage());
                        ui.label(format!("Overall progress: {:.1}%", overall_progress * 100.0));
                    }
                    GenerationState::Complete { output_path } => {
                        ui.label(format!("✅ Saved to: {}", output_path.display()));
                        ui.label("Metadata generation complete!");
                    }
                    GenerationState::Error { message } => {
                        ui.label(format!("❌ Error: {}", message));
                    }
                }
            }

            // Request continuous updates when processing
            if let Ok(state) = self.generation_state.lock() {
                if matches!(&*state, GenerationState::Processing { .. }) {
                    ctx.request_repaint();
                }
            }
        });
    }
}

impl GuiApp {
    fn start_generation(&self) {
        let folder = self.folder.clone();
        let date = self.date.clone();
        let title = self.title.clone();
        let creator = self.creator.clone();
        let description = self.description.clone();
        let generation_state = Arc::clone(&self.generation_state);

        thread::spawn(move || {
            let generation_state_clone = Arc::clone(&generation_state);
            
            // Create metadata
            let metadata = Metadata {
                date_created: date,
                title: title.clone(),
                creator,
                description,
                video_files: Vec::new(),
                audio_files: Vec::new(),
            };

            // Create generator with GUI progress callback
            let generator = MetadataGenerator::new_gui()
                .with_progress_callback(ProgressCallback::Gui(Box::new(move |current_file, file_progress, overall_progress| {
                    if let Ok(mut state) = generation_state_clone.lock() {
                        *state = GenerationState::Processing {
                            current_file,
                            file_progress,
                            overall_progress,
                            total_files: 0, // Will be updated
                            processed_files: (overall_progress * 100.0) as usize,
                        };
                    }
                })));

            match generator.generate_metadata_async(&folder.unwrap(), &metadata) {
                Ok(output_path) => {
                    if let Ok(mut state) = generation_state.lock() {
                        *state = GenerationState::Complete { output_path };
                    }
                }
                Err(e) => {
                    if let Ok(mut state) = generation_state.lock() {
                        *state = GenerationState::Error {
                            message: e.to_string(),
                        };
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = GuiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Metadata Generator", native_options, Box::new(|_cc| Box::new(app)))
}
