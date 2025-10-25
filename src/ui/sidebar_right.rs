use crate::models::gui::View;
use crate::models::gui::MyApp;
use crate::utils::svg_parser;
use egui::{Align, RichText, ScrollArea, SidePanel};
use std::fs;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    // Reset panel width if flagged
    let is_resetting = app.reset_panel_width;
    if app.reset_panel_width {
        ctx.memory_mut(|mem| {
            mem.data.insert_persisted(egui::Id::new("side_panel"), 200.0f32);
        });
        app.reset_panel_width = false;
    }

    SidePanel::right("side_panel")
        .default_width(100.0)
        .min_width(0.0)  // Allow complete collapse
        .max_width(400.0)
        .resizable(true)
        .frame(
            egui::Frame::default()
                .inner_margin(egui::Margin::same(16))
                .fill(egui::Color32::from_rgb(30, 29, 25)),
        )
        .show(ctx, |ui| {
            // Only show content if panel is wide enough OR if we're resetting
            if ui.available_width() > 20.0 || is_resetting {
                ScrollArea::vertical()
                    .id_salt("side_panel_scroll")
                    .show(ui, |ui| {
                        // Header with title and close button
                        ui.horizontal(|ui| {
                            ui.heading(RichText::new("Overview").size(18.0).strong());
                            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                                // close sidebar
                                if ui.button(RichText::new("―").size(14.0)).clicked() {
                                    app.selected_svg = None;
                                    app.svg_code.clear();
                                }
                            });
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // File info
                        if let Some(svg_path) = &app.selected_svg {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("📄").size(14.0));
                                ui.label(
                                    RichText::new(svg_path.file_name().unwrap().to_string_lossy())
                                        .color(egui::Color32::from_rgb(180, 180, 180)),
                                );
                            });
                            ui.add_space(8.0);
                        }

                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // SVG Info Section
                        if let Some(svg_path) = &app.selected_svg {
                            if let Ok(svg_info) = svg_parser::parse_svg_info(svg_path) {
                                ui.label(RichText::new("SVG Info:").strong().size(14.0));
                                ui.add_space(4.0);

                                egui::Frame::new()
                                    .fill(egui::Color32::from_rgb(35, 39, 42))
                                    .inner_margin(egui::Margin::same(12))
                                    .corner_radius(6.0)
                                    .show(ui, |ui| {
                                        ui.spacing_mut().item_spacing.y = 6.0;

                                        // Dimensions
                                        if let (Some(width), Some(height)) =
                                            (&svg_info.width, &svg_info.height)
                                        {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    RichText::new("Size:")
                                                        .color(egui::Color32::from_rgb(150, 150, 150)),
                                                );
                                                ui.label(
                                                    RichText::new(format!("{} × {}", width, height))
                                                        .color(egui::Color32::from_rgb(200, 200, 200)),
                                                );
                                            });
                                        }

                                        // ViewBox
                                        if let Some(view_box) = &svg_info.view_box {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    RichText::new("ViewBox:")
                                                        .color(egui::Color32::from_rgb(150, 150, 150)),
                                                );
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{} {} {} {}",
                                                        view_box.x, view_box.y, view_box.w, view_box.h
                                                    ))
                                                        .color(egui::Color32::from_rgb(200, 200, 200)),
                                                );
                                            });
                                        }

                                        // Path count
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new("Paths:")
                                                    .color(egui::Color32::from_rgb(150, 150, 150)),
                                            );
                                            ui.label(
                                                RichText::new(format!("{}", svg_info.path_count))
                                                    .color(egui::Color32::from_rgb(200, 200, 200)),
                                            );
                                        });

                                        // Path commands
                                        if svg_info.total_path_commands > 0 {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    RichText::new("Commands:")
                                                        .color(egui::Color32::from_rgb(150, 150, 150)),
                                                );
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{}",
                                                        svg_info.total_path_commands
                                                    ))
                                                        .color(egui::Color32::from_rgb(200, 200, 200)),
                                                );
                                            });
                                        }

                                        // Colors with color picker
                                        if !svg_info.colors_used.is_empty() {
                                            ui.add_space(4.0);

                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    RichText::new("🎨 Colors:")
                                                        .color(egui::Color32::from_rgb(150, 150, 150)),
                                                );
                                            });

                                            ui.add_space(4.0);

                                            egui::Grid::new("color_grid")
                                                .spacing([6.0, 6.0])
                                                .show(ui, |ui| {
                                                    for (idx, color) in svg_info.colors_used.iter().take(12).enumerate() {
                                                        let mut color32 = egui::Color32::from_rgb(
                                                            color.red,
                                                            color.green,
                                                            color.blue,
                                                        );

                                                        let original_color = color.clone();

                                                        // Add color picker button
                                                        if egui::color_picker::color_edit_button_srgba(
                                                            ui,
                                                            &mut color32,
                                                            egui::color_picker::Alpha::Opaque
                                                        ).changed() {
                                                            // Color changed - update the SVG file
                                                            if let Some(svg_path) = &app.selected_svg.clone() {
                                                                // First, forget the cached image
                                                                let img_uri = format!("file://{}", svg_path.display());
                                                                ctx.forget_image(&img_uri);

                                                                if let Ok(svg_content) = fs::read_to_string(svg_path) {
                                                                    // Generate all possible representations of the old color
                                                                    let old_hex_lower = format!(
                                                                        "#{:02x}{:02x}{:02x}",
                                                                        original_color.red,
                                                                        original_color.green,
                                                                        original_color.blue
                                                                    );
                                                                    let old_hex_upper = old_hex_lower.to_uppercase();
                                                                    let old_rgb = format!(
                                                                        "rgb({},{},{})",
                                                                        original_color.red,
                                                                        original_color.green,
                                                                        original_color.blue
                                                                    );
                                                                    let old_rgb_spaces = format!(
                                                                        "rgb({}, {}, {})",
                                                                        original_color.red,
                                                                        original_color.green,
                                                                        original_color.blue
                                                                    );

                                                                    // Generate new color in hex format
                                                                    let new_color_hex = format!(
                                                                        "#{:02x}{:02x}{:02x}",
                                                                        color32.r(),
                                                                        color32.g(),
                                                                        color32.b()
                                                                    );

                                                                    // Replace all instances of old color with new color
                                                                    let updated_content = svg_content
                                                                        .replace(&old_hex_lower, &new_color_hex)
                                                                        .replace(&old_hex_upper, &new_color_hex)
                                                                        .replace(&old_rgb, &new_color_hex)
                                                                        .replace(&old_rgb_spaces, &new_color_hex);

                                                                    // Save the updated SVG
                                                                    if let Err(e) = fs::write(svg_path, &updated_content) {
                                                                        app.set_error_message(format!("Failed to save SVG: {}", e));
                                                                    } else {
                                                                        // Update the svg_code in memory
                                                                        app.svg_code = updated_content;

                                                                        // Show success message
                                                                        app.set_error_message("✅ Color updated!".to_string());

                                                                        // Request repaint to refresh preview
                                                                        ctx.request_repaint();
                                                                    }
                                                                }
                                                            }
                                                        }

                                                        if (idx + 1) % 4 == 0 {
                                                            ui.end_row();
                                                        }
                                                    }
                                                });

                                            if svg_info.colors_used.len() > 12 {
                                                ui.label(
                                                    RichText::new(format!(
                                                        "+ {} more",
                                                        svg_info.colors_used.len() - 12
                                                    ))
                                                        .size(11.0)
                                                        .color(egui::Color32::from_rgb(120, 120, 120)),
                                                );
                                            }
                                        }
                                    });

                                ui.add_space(12.0);
                                ui.separator();
                                ui.add_space(8.0);
                            }
                        }
                        // Preview section
                        ui.label(RichText::new("Preview:").strong().size(14.0));
                        ui.add_space(4.0);

                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(35, 39, 42))
                            .inner_margin(egui::Margin::same(16))
                            .corner_radius(6.0)
                            .show(ui, |ui| {
                                ScrollArea::both()
                                    .id_salt("preview_scroll")
                                    .max_height(200.0)
                                    .show(ui, |ui| {
                                        if let Some(svg_path) = &app.selected_svg {
                                            let img_uri = format!("file://{}", svg_path.display());
                                            ui.add(
                                                egui::Image::new(img_uri)
                                                    .max_width(200.0)
                                                    .shrink_to_fit()
                                            );
                                        }
                                    });
                            });


                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Code editor section
                        ui.label(RichText::new("Code:").strong().size(14.0));
                        ui.add_space(4.0);

                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(32, 34, 37))
                            .inner_margin(egui::Margin::same(8))
                            .corner_radius(6.0)
                            .show(ui, |ui| {
                                ScrollArea::vertical()
                                    .id_salt("code_editor_scroll")
                                    .max_height(80.0)
                                    .show(ui, |ui| {
                                        ui.add(
                                            egui::TextEdit::multiline(&mut app.svg_code)
                                                .font(egui::TextStyle::Monospace)
                                                .code_editor()
                                                .desired_rows(15),
                                        );
                                    });
                            });

                        // Action buttons
                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                                if ui
                                    .button(RichText::new("Edit").size(14.0))
                                    .on_hover_text("Open in full editor")
                                    .clicked()
                                {
                                    app.current_view = View::Editor;
                                }
                                if ui
                                    .button(RichText::new("Copy").size(14.0))
                                    .on_hover_text("Copy SVG code to clipboard")
                                    .clicked()
                                {
                                    app.copy_svg_to_clipboard();
                                }
                            });
                        });


                    });
            } else {
                app.selected_svg = None;
                app.svg_code.clear();
            }
        });
}