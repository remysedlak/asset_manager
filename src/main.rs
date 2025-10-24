use crate::models::file_items::FileSystemItem;
use crate::utils::{file_finder, config::AppConfig};
use eframe::egui;
use egui::{CentralPanel, ScrollArea, SidePanel, Vec2};
use egui_extras::install_image_loaders;
use std::path::PathBuf;
use eframe::glow::Context;

pub mod models;
mod utils;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Asset Manager",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

enum View {
    Gallery,
    Settings,
}

struct MyApp {
    vault_path: String,
    vault_path_input: String,
    current_items: Vec<FileSystemItem>,
    selected_svg: Option<PathBuf>,
    error_message: Option<String>,
    current_view: View,
}

impl MyApp {
    const THUMBNAIL_SIZE: Vec2 = Vec2::new(80.0, 80.0);
    const PREVIEW_SIZE: f32 = 300.0;

    fn save_config(&self) {
        let config = AppConfig {
            vault_path: self.vault_path.clone(),
        };
        config.save();
    }

    fn refresh_directory(&mut self) {
        match file_finder::scan_directory(&self.vault_path) {
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
        self.vault_path = path;
        self.refresh_directory();
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let config = AppConfig::load();
        let vault_path = config.vault_path.clone();
        let current_items = file_finder::scan_directory(&vault_path).unwrap_or_default();

        Self {
            vault_path: vault_path.clone(),
            vault_path_input: vault_path,
            current_items,
            selected_svg: None,
            error_message: None,
            current_view: View::Gallery,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ////////////////////
        // TOOL SECTION
        /////////////////////
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
                // Home View
                if ui
                    .add_sized(
                        [40.0, 40.0],
                        egui::Button::new(egui::RichText::new("🏠").size(24.0)),
                    )
                    .on_hover_text("House")
                    .clicked()
                {
                    let config = AppConfig::load();
                    self.navigate_to(config.vault_path);
                    self.current_view = View::Gallery;
                }
            });

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
                        self.save_config();
                        self.refresh_directory();
                        self.current_view = View::Gallery;
                    }
                }
                View::Gallery => {
                    ui.separator();
                    if let Some(error) = &self.error_message {
                        ui.colored_label(egui::Color32::RED, error);
                    }

                    // File browser
                    let mut navigate_to: Option<String> = None;

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
                                                    egui::Image::new(img_uri)
                                                        .fit_to_exact_size(Self::THUMBNAIL_SIZE),
                                                ));

                                                if button.clicked() {
                                                    self.selected_svg = Some(path.clone());
                                                }

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

                    // Preview selected SVG
                    if let Some(svg_path) = &self.selected_svg {
                        ui.separator();
                        ui.label(format!("Selected: {}", svg_path.display()));
                        let img_uri = format!("file://{}", svg_path.display());
                        ui.add(
                            egui::Image::new(img_uri)
                                .max_width(Self::PREVIEW_SIZE)
                                .max_height(Self::PREVIEW_SIZE),
                        );
                    }
                }
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        self.save_config();
    }
}