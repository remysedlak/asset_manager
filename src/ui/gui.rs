use crate::models::file_items::FileSystemItem;
use crate::utils::{file_finder, config::AppConfig};
use eframe::egui;
use eframe::glow::Context;
use egui::{CentralPanel, ScrollArea, SidePanel, Vec2};
use std::path::{Path, PathBuf};
use arboard::Clipboard;
use ui::code_editor;
use std::fs;
use crate::ui;

pub enum View {
    Gallery,
    Settings,
}

pub struct MyApp {
    vault_path: String,           // Saved home path
    current_path: String,          // Current browsing path
    vault_path_input: String,
    current_items: Vec<FileSystemItem>,
    pub(crate) selected_svg: Option<PathBuf>,
    pub(crate) svg_code: String,              // SVG code being edited
    error_message: Option<String>,
    current_view: View,
    clipboard: Clipboard,
}

impl MyApp {
    pub(crate) const THUMBNAIL_SIZE: Vec2 = Vec2::new(80.0, 80.0);
    const PREVIEW_SIZE: f32 = 300.0;

    fn save_config(&self) {
        let config = AppConfig {
            vault_path: self.vault_path.clone(),
        };
        config.save();
    }

    fn refresh_directory(&mut self) {
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

    fn navigate_back(&mut self) {
        let current = Path::new(&self.current_path);
        if let Some(parent) = current.parent() {
            self.navigate_to(parent.to_string_lossy().to_string());
        }
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
            clipboard: Clipboard::new().unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TOOL SECTION
        SidePanel::left("my_left_panel")
            .max_width(60.0)
            .frame(egui::Frame::default().inner_margin(egui::Margin::same(8.0)))
            .show(ctx, |ui| {
                // SVG View
                if ui
                    .add_sized(
                        [40.0, 40.0],
                        egui::Button::new(egui::RichText::new("🎨").size(24.0)),
                    )
                    .on_hover_text("View SVGs")
                    .clicked()
                {
                    self.current_view = View::Gallery;
                    self.refresh_directory();
                }
                // Settings View
                if ui
                    .add_sized(
                        [40.0, 40.0],
                        egui::Button::new(egui::RichText::new("⚙").size(24.0)),
                    )
                    .on_hover_text("Settings")
                    .clicked()
                {
                    self.current_view = View::Settings;
                    self.vault_path_input = self.vault_path.clone();
                }
            });

        // In the update function, replace the RIGHT PANEL section with:
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
                    ui.horizontal(|ui| {
                        // Back button
                        if ui.button("⬅ Back").clicked() {
                            self.navigate_back();
                        }

                        ui.label(format!("Current: {}", self.current_path));
                    });

                    ui.separator();

                    if let Some(error) = &self.error_message {
                        let color = if error.starts_with("✓") {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        };
                        ui.colored_label(color, error);
                    }

                    // File browser
                    let mut navigate_to: Option<String> = None;
                    let mut load_svg: Option<PathBuf> = None;

                    ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("file_grid")
                            .num_columns(4)
                            .spacing([10.0, 10.0])
                            .show(ui, |ui| {
                                for (idx, item) in self.current_items.iter().enumerate() {
                                    match item {
                                        FileSystemItem::Directory { name, path } => {
                                            let button_response = ui
                                                .vertical(|ui| {
                                                    // Fixed size container for folder icon
                                                    ui.set_width(Self::THUMBNAIL_SIZE.x);
                                                    ui.set_height(Self::THUMBNAIL_SIZE.y);

                                                    let button = ui.button(
                                                        egui::RichText::new("📁")
                                                            .size(Self::THUMBNAIL_SIZE.y * 0.6),
                                                    );
                                                    ui.label(name);
                                                    button
                                                })
                                                .inner;

                                            if button_response.double_clicked() {
                                                navigate_to = Some(path.to_string_lossy().to_string());
                                            }
                                        }
                                        FileSystemItem::SvgFile { name, path } => {
                                            ui.vertical(|ui| {
                                                let img_uri = format!("file://{}", path.display());
                                                let button = ui.add(egui::ImageButton::new(
                                                    egui::Image::new(img_uri).rounding(10.0)
                                                        .fit_to_exact_size(Self::THUMBNAIL_SIZE),
                                                ));

                                                if button.clicked() {
                                                    load_svg = Some(path.clone());
                                                }

                                                // Right click context menu
                                                button.context_menu(|ui| {
                                                    if ui.button("📝 Edit").clicked() {
                                                        load_svg = Some(path.clone());
                                                        ui.close_menu();
                                                    }
                                                });

                                                ui.label(name);
                                            });
                                        }
                                    }

                                    if (idx + 1) % 4 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });
                    });

                    // Handle navigation outside the grid
                    if let Some(new_path) = navigate_to {
                        self.navigate_to(new_path);
                    }

                    // Handle load SVG outside the grid
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