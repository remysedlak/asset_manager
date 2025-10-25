use crate::models::gui::MyApp;
use egui::{RichText, Align};

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) {
    // Track which action to take after rendering
    let mut svg_save_clicked = false;
    let mut font_save_clicked = false;

    // Add padding and center content
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(40, 30))
        .show(ui, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Settings").size(32.0).strong());
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(30.0);

            // SVG Path Section
            svg_save_clicked = render_path_section(
                ui,
                "SVG Vault Path",
                "Choose the folder where your SVG files are stored",
                &mut app.vault_path_input,
            );

            ui.add_space(40.0);

            // Font Path Section
            font_save_clicked = render_path_section(
                ui,
                "Font Vault Path",
                "Choose the folder where your font files are stored",
                &mut app.current_font_input,
            );

            ui.add_space(40.0);
        });

    // Execute actions after rendering
    if svg_save_clicked {
        app.vault_path = app.vault_path_input.clone();
        app.save_config();
        app.current_view = crate::models::gui::View::Gallery;
        app.refresh_directory();
    }

    if font_save_clicked {
        app.font_path = app.current_font_input.clone();
        app.save_config();
    }
}

fn render_path_section(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    path_input: &mut String,
) -> bool {
    let mut save_clicked = false;

    egui::Frame::new()
        .fill(egui::Color32::from_rgb(35, 39, 42))
        .inner_margin(egui::Margin::same(20))
        .corner_radius(8.0)
        .show(ui, |ui| {
            // Section title
            ui.label(RichText::new(title).size(20.0).strong());
            ui.add_space(5.0);

            // Description
            ui.label(
                RichText::new(description)
                    .size(14.0)
                    .color(egui::Color32::from_rgb(150, 150, 150))
            );

            ui.add_space(10.0);

            // Path input
            ui.horizontal(|ui| {
                ui.label(RichText::new("Path:").size(16.0));

                let text_edit = egui::TextEdit::singleline(path_input)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(ui.available_width() - 250.0);
                ui.add(text_edit);
            });

            ui.add_space(5.0);

            // Action buttons on same line
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.add_sized(
                        [100.0, 35.0],
                        egui::Button::new(RichText::new("Save").size(16.0))
                    ).clicked() {
                        save_clicked = true;
                    }

                    if ui.add_sized(
                        [120.0, 35.0],
                        egui::Button::new(RichText::new("📁 Browse").size(16.0))
                    ).clicked() {
                        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                            *path_input = folder.to_string_lossy().to_string();
                        }
                    }
                });
            });
        });

    save_clicked
}