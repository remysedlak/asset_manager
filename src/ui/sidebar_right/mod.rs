// ui/sidebar_right/mod.rs
mod preview;
mod code_view;
mod colors;

use crate::models::gui::MyApp;
use egui::SidePanel;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    SidePanel::right("right_panel")
        .resizable(true)
        .default_width(300.0)
        .min_width(200.0)
        .max_width(800.0)
        .show(ctx, |ui| {
            // Fixed-height preview section
            preview::render(app, ui);

            ui.separator();

            // Calculate available height for colors
            // Reserve space for code editor (minimum 200px)
            let min_code_height = 200.0;
            let available_for_colors = (ui.available_height() - min_code_height - 20.0).max(100.0);

            // Color picker section with calculated max height
            egui::ScrollArea::vertical()
                .id_salt("colors_scroll")  // ← Add unique ID
                .max_height(available_for_colors)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    colors::render(app, ui);
                });

            ui.separator();

            // Flexible code editor section
            code_view::render(app, ui);
        });
}