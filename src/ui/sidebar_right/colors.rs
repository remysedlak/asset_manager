// ui/sidebar_right/color_picker.rs
use crate::models::gui::MyApp;
use crate::utils::svg_parser;
use egui::{Color32, RichText};
use std::fs;

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.add_space(2.0);
        ui.label(RichText::new("Colors").size(16.0).strong());
        ui.add_space(5.0);

        // Clone the path early to avoid borrow issues
        let svg_path_clone = app.selected_svg.clone();

        // Extract colors from the loaded SVG
        if let Some(svg_path) = &svg_path_clone {
            match svg_parser::parse_svg_info(svg_path) {
                Ok(svg_info) => {
                    if svg_info.colors_used.is_empty() {
                        ui.label("No colors found in this SVG");
                    } else {
                        ui.label(format!("Found {} colors:", svg_info.colors_used.len()));
                        ui.add_space(5.0);

                        // Display colors with copy buttons
                        for svg_color in svg_info.colors_used.iter() {
                            let mut color = Color32::from_rgb(svg_color.red, svg_color.green, svg_color.blue);
                            let old_hex = format!("#{:02X}{:02X}{:02X}", svg_color.red, svg_color.green, svg_color.blue);

                            ui.horizontal(|ui| {
                                // Color picker button (clickable!)
                                if ui.color_edit_button_srgba(&mut color).changed() {
                                    // Color was changed - update the SVG file
                                    let new_hex = format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b());
                                    if let Err(e) = replace_color_in_svg(svg_path, &old_hex, &new_hex) {
                                        app.set_error_message(format!("Failed to update color: {}", e));
                                    } else {
                                        app.set_error_message(format!("✅ Updated {} to {}", old_hex, new_hex));
                                        // Reload the SVG code
                                        if let Ok(content) = fs::read_to_string(svg_path) {
                                            app.svg_code = content;
                                        }
                                        // Force image cache to refresh by forgetting the texture
                                        let img_uri = format!("file://{}", svg_path.display());
                                        ui.ctx().forget_image(&img_uri);
                                        // Request repaint to show the updated image
                                        ui.ctx().request_repaint();
                                    }
                                }

                                // Hex value
                                ui.label(RichText::new(&old_hex).size(11.0).monospace());

                                // Copy button
                                if ui.small_button("📋").on_hover_text("Copy hex").clicked() {
                                    ui.ctx().copy_text(old_hex.clone());
                                    app.set_error_message(format!("✅ Copied {}", old_hex));
                                }
                            });

                            ui.add_space(3.0);
                        }

                        ui.add_space(5.0);
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

fn replace_color_in_svg(path: &std::path::Path, old_color: &str, new_color: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;

    // Replace all occurrences of the old color with the new color (case-insensitive)
    let old_lower = old_color.to_lowercase();
    let old_upper = old_color.to_uppercase();

    let mut new_content = content.clone();
    new_content = new_content.replace(&old_lower, new_color);
    new_content = new_content.replace(&old_upper, new_color);
    new_content = new_content.replace(old_color, new_color);

    fs::write(path, new_content)?;

    Ok(())
}