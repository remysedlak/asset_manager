
use crate::ui::sidebar_left;
use crate::ui::views::*;
use crate::utils::config::AppConfig;
use arboard::Clipboard;
use eframe::egui;
use eframe::glow::Context;
use egui::{CentralPanel, Vec2};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use crate::utils::file_finder::{scan_directory, FileFilter};
use crate::models::gui::View;
pub(crate) use crate::models::gui::MyApp;
use crate::ui::views::gallery;

impl MyApp {
    pub fn get_thumbnail_size(&self) -> Vec2 {
        Vec2::new(self.thumbnail_size * 10.0, self.thumbnail_size * 10.0)
    }

    pub(crate) fn save_config(&self) {
        let config = AppConfig {
            vault_path: self.vault_path.clone(),
            font_path: self.font_path.clone(),
            thumbnail_size: self.thumbnail_size
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
        let thumbnail_size = config.thumbnail_size;

        // Only scan directory if path is valid, otherwise use empty vec
        let current_items = if config.is_valid() {
            scan_directory(&vault_path, FileFilter::Svg).unwrap_or_default()
        } else {
            Vec::new()
        };



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
            gallery_options: false,
            clipboard: Clipboard::new().unwrap(),
            code: String::from("// Start coding here\nfn main() {\n    println!(\"Hello, world!\");\n}"),
            thumbnail_size,
            search_active: false,
            search_query: String::new(),
            sort_ascending: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // hot-keys for app ui
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::Comma) {
                self.current_view = View::Settings;
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {

                if  self.current_view == View::Editor{
                    self.current_view = View::Gallery;
                }

                if self.gallery_options && self.search_active {
                    self.gallery_options = false;
                }

                else if self.gallery_options && !self.search_active {
                    self.gallery_options = false;
                }
                else if !self.gallery_options && self.search_active {
                    self.search_active = false;
                }
            }


            if i.consume_key(egui::Modifiers::CTRL, egui::Key::H) {
                self.current_view = View::Help;
            }
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::G) {
                self.current_view = View::Gallery;
            }
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::F) {
                self.current_view = View::Fonts;
            }
        });
        if self.current_view != View::Editor {
            // render the left sidebar
            sidebar_left::render(self, ctx);
        }

        if self.rename_file_path.is_some() {
            crate::ui::popups::rename_file::render(self, ctx);
        }

        if self.delete_file_path.is_some() {
            crate::ui::popups::delete_file::render(self, ctx);
        }

        if self.gallery_options{
            crate::ui::popups::gallery_options::render(self, ctx);
        }



        // Code editor on the right when SVG is selected
        if let View::Gallery = self.current_view {
            if self.selected_svg.is_some() {
                crate::ui::sidebar_right::render(self, ctx);
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
                    help::render(self, ui);
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