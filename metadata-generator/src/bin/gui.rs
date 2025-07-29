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
#[path = "../file_analyzer.rs"]
mod file_analyzer;

use metadata_generator::{MetadataGenerator, Metadata, ProgressCallback, VerificationReport};

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

#[derive(Clone)]
enum VerificationState {
    Idle,
    Processing { 
        current_file: String, 
        file_progress: f32,
        overall_progress: f32, 
        total_files: usize, 
        processed_files: usize 
    },
    Complete { report: VerificationReport },
    Error { message: String },
}

#[derive(PartialEq)]
enum Tab {
    Generate,
    Verify,
}

struct GuiApp {
    // Tab management
    selected_tab: Tab,
    
    // Generate tab fields
    folder: Option<PathBuf>,
    artwork_id: String,
    artwork_title: String,
    artwork_short_title: String,
    artwork_creator: String,
    year_of_creation: String,
    short_description: String,
    long_description: String,
    edition_number: String,
    total_editions: String,
    issue_date: String,
    gallery: String,
    keywords: String,
    medium: String,
    status: String,
    certificate_warning: String,
    generation_state: Arc<Mutex<GenerationState>>,
    
    // Verify tab fields
    metadata_file: Option<PathBuf>,
    base_folder: Option<PathBuf>,
    verification_state: Arc<Mutex<VerificationState>>,
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            selected_tab: Tab::Generate,
            folder: None,
            artwork_id: String::new(),
            artwork_title: String::new(),
            artwork_short_title: String::new(),
            artwork_creator: String::new(),
            year_of_creation: String::new(),
            short_description: String::new(),
            long_description: String::new(),
            edition_number: String::new(),
            total_editions: String::new(),
            issue_date: String::new(),
            gallery: String::new(),
            keywords: String::new(),
            medium: String::new(),
            status: String::new(),
            certificate_warning: String::new(),
            generation_state: Arc::new(Mutex::new(GenerationState::Idle)),
            metadata_file: None,
            base_folder: None,
            verification_state: Arc::new(Mutex::new(VerificationState::Idle)),
        }
    }
}

impl App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Metadata Generator");
            
            // Tab selection
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, Tab::Generate, "Generate");
                ui.selectable_value(&mut self.selected_tab, Tab::Verify, "Verify");
            });
            
            ui.separator();
            
            match self.selected_tab {
                Tab::Generate => self.render_generate_tab(ui),
                Tab::Verify => self.render_verify_tab(ui),
            }
            
            // Request continuous updates when processing
            if let Ok(state) = self.generation_state.lock() {
                if matches!(&*state, GenerationState::Processing { .. }) {
                    ctx.request_repaint();
                }
            }
            if let Ok(state) = self.verification_state.lock() {
                if matches!(&*state, VerificationState::Processing { .. }) {
                    ctx.request_repaint();
                }
            }
        });
    }
}

impl GuiApp {
    fn render_generate_tab(&mut self, ui: &mut egui::Ui) {
        // Wrap the entire content in a scrollable area
        egui::ScrollArea::vertical().max_height(f32::INFINITY).show(ui, |ui| {
            if ui.button("Browse for folder").clicked() {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    self.folder = Some(dir.clone());
                    
                    // Check for certificate of authenticity
                    let certificate_path = metadata_generator::detect_certificate_of_authenticity(&dir);
                    if certificate_path.is_none() {
                        self.certificate_warning = "Warning: No certificate of authenticity PDF found in 'certificate' folder".to_string();
                    } else {
                        self.certificate_warning = "Certificate of authenticity found".to_string();
                    }
                }
            }
            if let Some(folder) = &self.folder {
                ui.label(format!("Folder: {}", folder.display()));
            }

            ui.separator();
            ui.label("Artwork ID");
            ui.text_edit_singleline(&mut self.artwork_id);
            ui.label("Artwork Title");
            ui.text_edit_singleline(&mut self.artwork_title);
            ui.label("Artwork Short Title");
            ui.text_edit_singleline(&mut self.artwork_short_title);
            ui.label("Artwork Creator");
            ui.text_edit_singleline(&mut self.artwork_creator);
            ui.label("Year of Creation");
            ui.text_edit_singleline(&mut self.year_of_creation);
            ui.label("Short Description");
            ui.text_edit_singleline(&mut self.short_description);
            ui.label("Long Description");
            ui.text_edit_multiline(&mut self.long_description);
            ui.label("Edition Number");
            ui.text_edit_singleline(&mut self.edition_number);
            ui.label("Total Editions");
            ui.text_edit_singleline(&mut self.total_editions);
            ui.label("Issue Date (YYYY-MM-DD)");
            ui.text_edit_singleline(&mut self.issue_date);
            ui.label("Gallery");
            ui.text_edit_singleline(&mut self.gallery);
            ui.label("Keywords (comma-separated)");
            ui.text_edit_singleline(&mut self.keywords);
            ui.label("Medium (comma-separated)");
            ui.text_edit_singleline(&mut self.medium);
            
            // Display certificate warning
            if !self.certificate_warning.is_empty() {
                if self.certificate_warning.starts_with("Warning:") {
                    ui.colored_label(egui::Color32::from_rgb(255, 165, 0), &self.certificate_warning);
                } else {
                    ui.colored_label(egui::Color32::from_rgb(0, 255, 0), &self.certificate_warning);
                }
            }

            // Check if we can start generation
            let can_generate = self.folder.is_some() && !self.artwork_title.is_empty();
            
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
        });
    }

    fn render_verify_tab(&mut self, ui: &mut egui::Ui) {
        // Wrap the entire content in a scrollable area
        egui::ScrollArea::vertical().max_height(f32::INFINITY).show(ui, |ui| {
            ui.label("Verify Metadata and Files");
            ui.add_space(10.0);
            
            // File selection
            ui.label("Select metadata file:");
            if ui.button("Browse for metadata file").clicked() {
                if let Some(file) = FileDialog::new()
                    .add_filter("JSON files", &["json"])
                    .pick_file() {
                    self.metadata_file = Some(file.clone());
                    
                    // Try to auto-detect base folder (same directory as metadata file)
                    if let Some(parent) = file.parent() {
                        self.base_folder = Some(parent.to_path_buf());
                    }
                }
            }
            if let Some(metadata_file) = &self.metadata_file {
                ui.label(format!("Metadata file: {}", metadata_file.display()));
            }
            
            ui.add_space(10.0);
            
            // Base folder selection
            ui.label("Select base folder (where the artwork files are located):");
            if ui.button("Browse for base folder").clicked() {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    self.base_folder = Some(dir);
                }
            }
            if let Some(base_folder) = &self.base_folder {
                ui.label(format!("Base folder: {}", base_folder.display()));
            }
            
            ui.add_space(10.0);
            
            // Check if we can start verification
            let can_verify = self.metadata_file.is_some() && self.base_folder.is_some();
            
            if ui.add_enabled(can_verify, egui::Button::new("Verify files")).clicked() {
                self.start_verification();
            }
            
            ui.separator();
            
            // Show verification progress and results
            if let Ok(state) = self.verification_state.lock() {
                match &*state {
                    VerificationState::Idle => {
                        ui.label("Select a metadata file and base folder to start verification.");
                    }
                    VerificationState::Processing { current_file, file_progress, overall_progress, total_files, processed_files } => {
                        ui.label(format!("Verifying: {}/{} files", processed_files + 1, total_files));
                        ui.label(format!("Current file: {}", current_file));
                        ui.add_space(5.0);
                        
                        ui.label("File progress:");
                        ui.add(egui::ProgressBar::new(*file_progress).show_percentage());
                        
                        ui.add_space(5.0);
                        ui.label("Overall progress:");
                        ui.add(egui::ProgressBar::new(*overall_progress).show_percentage());
                        ui.label(format!("Overall progress: {:.1}%", overall_progress * 100.0));
                    }
                    VerificationState::Complete { report } => {
                        ui.label("✅ Verification complete!");
                        ui.add_space(10.0);
                        
                        // Summary
                        ui.label(format!("Metadata file hash: {}", report.metadata_file_hash));
                        ui.label(format!("Total files: {}", report.total_files));
                        ui.label(format!("Valid files: {}", report.valid_files));
                        ui.label(format!("Invalid files: {}", report.invalid_files));
                        
                        // Certificate verification
                        if let Some(certificate_valid) = report.certificate_valid {
                            if certificate_valid {
                                ui.colored_label(egui::Color32::from_rgb(0, 255, 0), "✅ Certificate is valid!");
                            } else {
                                ui.colored_label(egui::Color32::from_rgb(255, 0, 0), "❌ Certificate is invalid!");
                            }
                            if let Some(certificate_hash) = &report.certificate_hash {
                                ui.label(format!("Certificate hash: {}", certificate_hash));
                            }
                        } else {
                            ui.label("ℹ️ No certificate found");
                        }
                        
                        if report.overall_valid {
                            ui.colored_label(egui::Color32::from_rgb(0, 255, 0), "✅ All files and certificate are valid!");
                        } else {
                            ui.colored_label(egui::Color32::from_rgb(255, 0, 0), "❌ Some files or certificate are invalid!");
                        }
                        
                        ui.add_space(10.0);
                        
                        // Detailed results
                        ui.label("Detailed Results:");
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for result in &report.results {
                                ui.horizontal(|ui| {
                                    if result.is_valid {
                                        ui.label("✅");
                                    } else {
                                        ui.label("❌");
                                    }
                                    ui.label(&result.file_name);
                                    if !result.is_valid {
                                        if let Some(error) = &result.error {
                                            ui.colored_label(egui::Color32::from_rgb(255, 0, 0), error);
                                        }
                                    }
                                });
                            }
                        });
                    }
                    VerificationState::Error { message } => {
                        ui.label(format!("❌ Error: {}", message));
                    }
                }
            }
        });
    }

    fn start_generation(&self) {
        let folder = self.folder.clone();
        let artwork_id = self.artwork_id.clone();
        let artwork_title = self.artwork_title.clone();
        let artwork_short_title = self.artwork_short_title.clone();
        let artwork_creator = self.artwork_creator.clone();
        let year_of_creation = self.year_of_creation.clone();
        let short_description = self.short_description.clone();
        let long_description = self.long_description.clone();
        let edition_number = self.edition_number.clone();
        let total_editions = self.total_editions.clone();
        let issue_date = self.issue_date.clone();
        let gallery = self.gallery.clone();
        let keywords = self.keywords.clone();
        let medium = self.medium.clone();
        let generation_state = Arc::clone(&self.generation_state);

        thread::spawn(move || {
            let generation_state_clone = Arc::clone(&generation_state);
            
            // Parse keywords and medium from comma-separated strings
            let keywords_vec: Vec<String> = keywords.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            
            let medium_vec: Vec<String> = medium.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            // Parse numeric fields
            let year_of_creation_int = year_of_creation.parse::<i32>().unwrap_or(2024);
            let edition_number_int = edition_number.parse::<i32>().unwrap_or(1);
            let total_editions_int = total_editions.parse::<i32>().unwrap_or(1);

            // Detect certificate of authenticity
            let certificate_of_authenticity = if let Some(folder) = &folder {
                metadata_generator::detect_certificate_of_authenticity(folder)
            } else {
                None
            };

            // Create metadata
            let metadata = Metadata {
                artwork_id,
                artwork_title: artwork_title.clone(),
                artwork_short_title,
                artwork_creator,
                year_of_creation: year_of_creation_int,
                short_description,
                long_description,
                edition_number: edition_number_int,
                total_editions: total_editions_int,
                issue_date,
                gallery,
                keywords: keywords_vec,
                medium: medium_vec,
                certificate_of_authenticity,
                certificate_hash: None,
                artwork_files: Vec::new(),
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

    fn start_verification(&self) {
        let metadata_file = self.metadata_file.clone();
        let base_folder = self.base_folder.clone();
        let verification_state = Arc::clone(&self.verification_state);

        thread::spawn(move || {
            let verification_state_clone = Arc::clone(&verification_state);
            
            // Create generator with GUI progress callback
            let generator = MetadataGenerator::new_gui()
                .with_progress_callback(ProgressCallback::Gui(Box::new(move |current_file, file_progress, overall_progress| {
                    if let Ok(mut state) = verification_state_clone.lock() {
                        *state = VerificationState::Processing {
                            current_file,
                            file_progress,
                            overall_progress,
                            total_files: 0, // Will be updated
                            processed_files: (overall_progress * 100.0) as usize,
                        };
                    }
                })));

            match generator.verify_metadata_file_with_progress(&metadata_file.unwrap(), &base_folder.unwrap()) {
                Ok(report) => {
                    if let Ok(mut state) = verification_state.lock() {
                        *state = VerificationState::Complete { report };
                    }
                }
                Err(e) => {
                    if let Ok(mut state) = verification_state.lock() {
                        *state = VerificationState::Error {
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
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_min_inner_size([600.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native("Metadata Generator", native_options, Box::new(|_cc| Box::new(app)))
}
