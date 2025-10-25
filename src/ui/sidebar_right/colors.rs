// ui/sidebar_right/color_picker.rs
use crate::models::gui::MyApp;
use crate::utils::svg_parser;
use egui::{Color32, RichText};

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.add_space(2.0);
        ui.label(RichText::new("Colors").size(16.0).strong());
        ui.add_space(5.0);

        // Extract colors from the loaded SVG
        if let Some(svg_path) = &app.selected_svg {
            match svg_parser::parse_svg_info(svg_path) {
                Ok(svg_info) => {
                    if svg_info.colors_used.is_empty() {
                        ui.label("No colors found in this SVG");
                    } else {
                        ui.label(format!("Found {} colors:", svg_info.colors_used.len()));
                        ui.add_space(5.0);

                        // Display colors in a grid
                        egui::Grid::new("color_grid")
                            .num_columns(4)
                            .spacing([4.0, 8.0])
                            .show(ui, |ui| {
                                for (idx, svg_color) in svg_info.colors_used.iter().enumerate() {
                                    let color = Color32::from_rgb(svg_color.red, svg_color.green, svg_color.blue);
                                    let hex = format!("#{:02X}{:02X}{:02X}", svg_color.red, svg_color.green, svg_color.blue);

                                    // Color swatch button
                                    let button = ui.add(
                                        egui::Button::new("")
                                            .fill(color)
                                            .min_size(egui::vec2(20.0, 20.0))
                                            .corner_radius(20)
                                    );

                                    if button.clicked() {
                                        ui.ctx().copy_text(hex.clone());  // ← Fixed line
                                        app.set_error_message(format!("✅ Copied {}", hex));
                                    }

                                    button.on_hover_text(hex);

                                    // New row every 4 colors
                                    if (idx + 1) % 4 == 0 {
                                        ui.end_row();
                                    }
                                }
                            });

                        ui.add_space(10.0);

                        // Show hex values below
                        ui.label(RichText::new("Click a color to copy its hex value").size(10.0).italics());
                    }
                }
                Err(e) => {
                    ui.label(format!("Error parsing SVG: {}", e));
                }
            }
        } else {
            ui.label("No SVG selected");
        }

        ui.add_space(10.0);
    });
}