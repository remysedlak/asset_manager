use crate::models::file_items::FileSystemItem;
use crate::ui::{code_editor, gallery, toolbar, settings};
use crate::utils::{config::AppConfig};
use arboard::Clipboard;
use eframe::egui;
use eframe::glow::Context;
use egui::{CentralPanel, RichText, Vec2};
use std::fs;
use std::path::{PathBuf};
use std::time::Instant;
use crate::utils::file_finder::{scan_directory, FileFilter};
use crate::models::gui::View;

pub struct MyApp {
    pub(crate) grid_reset_counter: usize,
    pub(crate) vault_path: String,
    pub(crate) current_path: String,
    pub(crate) font_path: String,
    pub(crate) vault_path_input: String,
    pub(crate) current_font_input: String,
    pub(crate) current_items: Vec<FileSystemItem>,
    pub(crate) selected_svg: Option<PathBuf>,
    pub(crate) svg_code: String,
    pub(crate) error_message: Option<String>,
    pub(crate) error_message_time: Option<Instant>,
    pub(crate) rename_file_path: Option<PathBuf>,
    pub(crate) rename_input: String,
    pub(crate) rename_just_opened: bool,
    pub(crate) current_view: View,
    pub(crate) delete_file_path: Option<PathBuf>,
    clipboard: Clipboard,
    pub(crate) code: String,
    pub(crate) reset_panel_width: bool,
}

impl MyApp {
    pub(crate) const THUMBNAIL_SIZE: Vec2 = Vec2::new(80.0, 80.0);

    pub(crate) fn save_config(&self) {
        let config = AppConfig {
            vault_path: self.vault_path.clone(),
            font_path: self.font_path.clone()
        };
        config.save();
    }

    pub(crate) fn set_error_message(&mut self, message: String) {
        self.error_message = Some(message);
        self.error_message_time = Some(Instant::now());
    }

    pub fn refresh_directory(&mut self) {
        let (path, filter) = match self.current_view {
            View::Gallery => (&self.vault_path, FileFilter::Svg),
            View::Fonts => (&self.font_path, FileFilter::Font),
            _ => return,
        };

        match scan_directory(path, filter) {
            Ok(items) => {
                self.current_items = items;
                self.current_path = path.clone();
            }
            Err(e) => self.set_error_message(format!("Error scanning directory: {}", e)),
        }
    }

    fn navigate_to(&mut self, path: String) {
        self.current_path = path;

        let filter = match self.current_view {
            View::Gallery => FileFilter::Svg,
            View::Fonts => FileFilter::Font,
            _ => return,
        };

        match scan_directory(&self.current_path, filter) {
            Ok(items) => self.current_items = items,
            Err(e) => self.set_error_message(format!("Error scanning directory: {}", e)),
        }
    }

    fn load_svg(&mut self, path: &PathBuf) {
        match fs::read_to_string(path) {
            Ok(content) => {
                self.svg_code = content;
                self.selected_svg = Some(path.clone());
            }
            Err(e) => {
                self.set_error_message(format!("Failed to read file: {}", e));
            }
        }
    }

    pub(crate) fn save_svg(&mut self) {
        if let Some(path) = &self.selected_svg {
            match fs::write(path, &self.svg_code) {
                Ok(_) => {
                    self.set_error_message("✅ Saved!".to_string());
                }
                Err(e) => {
                    self.set_error_message(format!("Failed to save: {}", e));
                }
            }
        }
    }

    pub(crate) fn copy_svg_to_clipboard(&mut self) {
        if let Err(e) = self.clipboard.set_text(self.svg_code.clone()) {
            self.set_error_message(format!("Failed to copy: {}", e));
        } else {
            self.set_error_message("✅ Copied to clipboard!".to_string());
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let config = AppConfig::load();
        let vault_path = config.vault_path.clone();
        let font_path = config.font_path.clone();
        let current_items = scan_directory(&vault_path, FileFilter::Svg).unwrap_or_default();

        Self {
            grid_reset_counter: 0,
            reset_panel_width: false,
            vault_path: vault_path.clone(),
            current_path: vault_path.clone(),
            font_path: font_path.clone(),
            vault_path_input: vault_path,
            current_items,
            current_font_input: font_path,
            selected_svg: None,
            svg_code: String::new(),
            error_message: None,
            error_message_time: None,
            current_view: View::Gallery,
            rename_input: String::new(),
            rename_just_opened: false,
            delete_file_path: None,
            rename_file_path: None,
            clipboard: Clipboard::new().unwrap(),
            code: String::from("// Start coding here\nfn main() {\n    println!(\"Hello, world!\");\n}"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        toolbar::render(self, ctx);

        if self.rename_file_path.is_some() {
            crate::ui::popups::rename_file::render(self, ctx);
        }

        if self.delete_file_path.is_some() {
            crate::ui::popups::delete_file::render(self, ctx);
        }

        // Code editor on the right when SVG is selected
        if let View::Gallery = self.current_view {
            if self.selected_svg.is_some() {
                crate::ui::svg_overview::render(self, ctx);
            }
        }

        CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(40, 39, 35))// Match your header color
                    .inner_margin(egui::Margin::same(0))
            )
            .show(ctx, |ui| match self.current_view {
                View::Settings => {
                    settings::render(self, ui);
                }
                View::Gallery => {
                    // Render header at top level
                    let root_path = gallery::helpers::get_root_path(self).clone();
                    let current_path = self.current_path.clone();
                    let is_at_root = current_path == root_path;
                    let display_path = gallery::helpers::calculate_display_path(&current_path, &root_path);

                    let mut navigate_to: Option<String> = None;
                    gallery::header::render(self, ui, &mut navigate_to, is_at_root, &root_path, &display_path);
                    gallery::header::render_status_messages(self, ui);

                    // Render gallery content without header
                    let (nav, load_svg) = gallery::render_content(self, ui);

                    // Combine navigation from header and content
                    if let Some(new_path) = navigate_to.or(nav) {
                        self.navigate_to(new_path);
                    }

                    if let Some(path) = load_svg {
                        self.load_svg(&path);
                    }
                }
                View::Fonts => {
                    // Render header at top level
                    let root_path = gallery::helpers::get_root_path(self).clone();
                    let current_path = self.current_path.clone();
                    let is_at_root = current_path == root_path;
                    let display_path = gallery::helpers::calculate_display_path(&current_path, &root_path);

                    let mut navigate_to: Option<String> = None;
                    gallery::header::render(self, ui, &mut navigate_to, is_at_root, &root_path, &display_path);
                    gallery::header::render_status_messages(self, ui);

                    // Render gallery content without header
                    let (nav, _load_font) = gallery::render_content(self, ui);

                    if let Some(new_path) = navigate_to.or(nav) {
                        self.navigate_to(new_path);
                    }
                }
                View::Help => {
                    ui.heading(RichText::from("Help").size(20.0).strong());
                    ui.separator();
                    ui.add_space(10.0);
                    ui.vertical(|ui|{
                        ui.label(RichText::from("Controls:").size(15.0));
                    });
                }
                View::Editor => {
                    // If there's an SVG selected, make sure the code is loaded
                    if self.selected_svg.is_some() && self.svg_code.is_empty() {
                        // This shouldn't happen, but just in case
                        if let Some(path) = &self.selected_svg.clone() {
                            self.load_svg(&path);
                        }
                    }
                    code_editor::render_editor(self, ui);
                }
            });
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        self.save_config();
    }
}