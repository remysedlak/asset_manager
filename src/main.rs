use eframe::egui;
use egui::{CentralPanel, ScrollArea, SidePanel, Vec2};
use egui_extras::install_image_loaders;
use crate::utils::file_finder;
use crate::models::file_items::FileSystemItem;
use std::path::PathBuf;

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

struct MyApp {
    vault_path: String,
    current_items: Vec<FileSystemItem>,
    selected_svg: Option<PathBuf>,
    error_message: Option<String>,
}

impl MyApp {
    const THUMBNAIL_SIZE: Vec2 = Vec2::new(80.0, 80.0);
    const PREVIEW_SIZE: f32 = 300.0;

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
        let vault_path = "/home/remy/Pictures/images/svg".to_owned();
        let current_items = file_finder::scan_directory(&vault_path).unwrap_or_default();

        Self {
            vault_path,
            current_items,
            selected_svg: None,
            error_message: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("my_left_panel")
            .max_width(30.0)
            .frame(egui::Frame::default().inner_margin(egui::Margin::same(8.0)))
            .show(ctx, |ui| {
                if ui.button("🎨").on_hover_text("View SVGs").clicked() {
                    self.refresh_directory();
                }
                ui.add_space(4.0);
                if ui.button("⚙").on_hover_text("Settings").clicked() {}
            });

        CentralPanel::default().show(ctx, |ui| {
            // Path input bar
            ui.horizontal(|ui| {
                ui.label("Filepath ");
                let response = ui.text_edit_singleline(&mut self.vault_path);

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.refresh_directory();
                }

                if ui.button("🔄").clicked() {
                    self.refresh_directory();
                }
            });

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
                                    let button_response = ui.vertical(|ui| {
                                        // Fixed size container for folder icon
                                        ui.set_width(Self::THUMBNAIL_SIZE.x);
                                        ui.set_height(Self::THUMBNAIL_SIZE.y);

                                        let button = ui.button(
                                            egui::RichText::new("📁")
                                                .size(Self::THUMBNAIL_SIZE.y * 0.6)
                                        );
                                        ui.label(name);
                                        button
                                    }).inner;

                                    if button_response.double_clicked() {
                                        navigate_to = Some(path.to_string_lossy().to_string());
                                    }
                                }
                                FileSystemItem::SvgFile { name, path } => {
                                    ui.vertical(|ui| {
                                        let img_uri = format!("file://{}", path.display());
                                        let button = ui.add(
                                            egui::ImageButton::new(
                                                egui::Image::new(img_uri)
                                                    .fit_to_exact_size(Self::THUMBNAIL_SIZE)
                                            )
                                        );

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
                        .max_height(Self::PREVIEW_SIZE)
                );
            }
        });
    }
}