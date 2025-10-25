use crate::models::gui::MyApp;
use egui::{RichText};

pub fn render(_app: &mut MyApp, ui: &mut egui::Ui) {

    // Add padding and center content
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(40, 30))
        .show(ui, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Help").size(32.0).strong());
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(30.0);

            egui::Frame::new()
                .fill(egui::Color32::from_rgb(35, 39, 42))
                .inner_margin(egui::Margin::same(20))
                .corner_radius(8.0)
                .show(ui, |ui| {
                    // Section title
                    ui.heading(RichText::new("Keyboard Shortcuts").size(18.0));
                    ui.add_space(10.0);

                    egui::Grid::new("hotkeys_grid")
                        .num_columns(2)
                        .spacing([40.0, 8.0])
                        .striped(true)
                        .show(ui, |ui| {
                            // Header row
                            ui.label(RichText::new("Action").strong());
                            ui.label(RichText::new("Shortcut").strong());
                            ui.end_row();

                            // Hotkey rows
                            ui.label("Graphic Gallery");
                            ui.label(RichText::new("Ctrl + G").monospace());
                            ui.end_row();

                            ui.label("Font Library");
                            ui.label(RichText::new("Ctrl + F").monospace());
                            ui.end_row();

                            ui.label("Help");
                            ui.label(RichText::new("Ctrl + H").monospace());
                            ui.end_row();

                            ui.label("Settings");
                            ui.label(RichText::new("Ctrl + ,").monospace());
                            ui.end_row();
                        });
                });
        });
}