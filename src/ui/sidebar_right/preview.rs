use crate::models::gui::MyApp;
use crate::utils::svg_parser;
use egui::RichText;

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) {
    // Remove the fixed height container
    ui.vertical(|ui| {
        render_header(app, ui);
        render_image(app, ui);
        render_stats(app, ui);
    });
}

fn render_header(app: &MyApp, ui: &mut egui::Ui) {
    if let Some(svg_path) = &app.selected_svg {
        ui.horizontal(|ui| {
            ui.label(RichText::new("📄").size(16.0));

            let file_name = svg_path.file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            // Truncate filename if too long
            let display_name = if file_name.len() > 35 {
                format!("{}...", &file_name[..32])
            } else {
                file_name
            };

            ui.label(RichText::new(display_name).size(13.0));
        });

        ui.add_space(8.0);
    }
}

fn render_image(app: &MyApp, ui: &mut egui::Ui) {
    // Fixed height for image area only
    let image_height = 200.0; // Fixed image preview height

    egui::Frame::new()
        .fill(egui::Color32::from_rgb(30, 30, 30))
        .show(ui, |ui| {
            ui.set_height(image_height);

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if let Some(svg_path) = &app.selected_svg {
                        let img_uri = format!("file://{}", svg_path.display());

                        ui.vertical_centered(|ui| {
                            ui.add(
                                egui::Image::new(img_uri)
                                    .max_width(ui.available_width() - 20.0)
                                    .shrink_to_fit()
                            );
                        });
                    }
                });
        });
}

fn render_stats(app: &MyApp, ui: &mut egui::Ui) {
    ui.add_space(8.0);

    if let Some(svg_path) = &app.selected_svg {
        match svg_parser::parse_svg_info(svg_path) {
            Ok(info) => {
                egui::Frame::new()
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Dimensions
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("📏").size(12.0));
                                if let (Some(w), Some(h)) = (&info.width, &info.height) {
                                    ui.label(RichText::new(format!("{} × {}", w, h)).size(11.0));
                                } else if let Some(vb) = &info.view_box {
                                    ui.label(RichText::new(format!("{} × {}", vb.w, vb.h)).size(11.0));
                                } else {
                                    ui.label(RichText::new("No dimensions").size(11.0).weak());
                                }
                            });

                            ui.add_space(1.0);

                            // Path count
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("🔀").size(12.0));
                                ui.label(RichText::new(format!("{} paths", info.path_count)).size(11.0));
                            });

                            ui.add_space(1.0);

                            // Colors count
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("🎨").size(12.0));
                                ui.label(RichText::new(format!("{} colors", info.colors_used.len())).size(11.0));
                            });

                            ui.add_space(1.0);

                            // Commands
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("⚡").size(12.0));
                                ui.label(RichText::new(format!("{} commands", info.total_path_commands)).size(11.0));
                            });
                        });
                    });
            }
            Err(_) => {
                ui.label(RichText::new("Unable to parse SVG info").size(10.0).weak());
            }
        }
    }

    ui.add_space(8.0); // Small spacing after stats before colors
}