use crate::models::file_items::FileSystemItem;
use crate::utils::{file_finder, config::AppConfig};
use crate::ui::{code_editor, toolbar, gallery};
use eframe::egui;
use eframe::glow::Context;
use egui::{CentralPanel, Vec2};
use std::path::{Path, PathBuf};
use arboard::Clipboard;
use std::fs;

pub enum View {
    Gallery,
    Settings,
}

pub struct MyApp {
    pub(crate) vault_path: String,
    pub(crate) current_path: String,
    pub(crate) vault_path_input: String,
    pub(crate) current_items: Vec<FileSystemItem>,
    pub(crate) selected_svg: Option<PathBuf>,
    pub(crate) svg_code: String,
    pub(crate) error_message: Option<String>,
    pub(crate) rename_file_path: Option<PathBuf>,
    pub(crate) rename_input: String,
    pub(crate) rename_just_opened: bool,
    pub(crate) current_view: View,
    clipboard: Clipboard,
}

impl MyApp {
    pub(crate) const THUMBNAIL_SIZE: Vec2 = Vec2::new(80.0, 80.0);

    fn save_config(&self) {
        let config = AppConfig {
            vault_path: self.vault_path.clone(),
        };
        config.save();
    }

    pub(crate) fn refresh_directory(&mut self) {
        match file_finder::scan_directory(&self.current_path) {
            Ok(items) => {
                self.current_items = items;
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Error: {}", e));
            }
        }
    }

    fn navigate_to(&mut self, path: String) {
        self.current_path = path;
        self.refresh_directory();
    }

    fn load_svg(&mut self, path: &PathBuf) {
        match fs::read_to_string(path) {
            Ok(content) => {
                self.svg_code = content;
                self.selected_svg = Some(path.clone());
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to read file: {}", e));
            }
        }
    }

    pub(crate) fn save_svg(&mut self) {
        if let Some(path) = &self.selected_svg {
            match fs::write(path, &self.svg_code) {
                Ok(_) => {
                    self.error_message = Some("✓ Saved!".to_string());
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to save: {}", e));
                }
            }
        }
    }

    pub(crate) fn copy_svg_to_clipboard(&mut self) {
        if let Err(e) = self.clipboard.set_text(self.svg_code.clone()) {
            self.error_message = Some(format!("Failed to copy: {}", e));
        } else {
            self.error_message = Some("✓ Copied to clipboard!".to_string());
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let config = AppConfig::load();
        let vault_path = config.vault_path.clone();
        let current_items = file_finder::scan_directory(&vault_path).unwrap_or_default();

        Self {
            vault_path: vault_path.clone(),
            current_path: vault_path.clone(),
            vault_path_input: vault_path,
            current_items,
            selected_svg: None,
            svg_code: String::new(),
            error_message: None,
            current_view: View::Gallery,
            rename_input: String::new(),       // Add this
            rename_just_opened: false,
            rename_file_path: None,
            clipboard: Clipboard::new().unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        toolbar::render(self, ctx);

        if self.rename_file_path.is_some() {
            crate::ui::popups::rename_file::render(self, ctx);
        }

        // Code editor on the right when SVG is selected
        if let View::Gallery = self.current_view {
            if self.selected_svg.is_some() {
                code_editor::render(self, ctx);
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::Settings => {
                    ui.heading("Settings");
                    ui.separator();
                    ui.add_space(10.0);

                    ui.label("Vault Path:");
                    ui.text_edit_singleline(&mut self.vault_path_input);

                    ui.add_space(10.0);

                    if ui.button("Submit").clicked() {
                        self.vault_path = self.vault_path_input.clone();
                        self.current_path = self.vault_path.clone();
                        self.save_config();
                        self.refresh_directory();
                        self.current_view = View::Gallery;
                    }
                }
                View::Gallery => {
                    let (navigate_to, load_svg) = gallery::render(self, ui);

                    if let Some(new_path) = navigate_to {
                        self.navigate_to(new_path);
                    }

                    if let Some(path) = load_svg {
                        self.load_svg(&path);
                    }
                }
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        self.save_config();
    }
}