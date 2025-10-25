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
            
            // Color picker section - extracts colors from SVG
            colors::render(app, ui);

            ui.separator();

            // Flexible code editor section
            code_view::render(app, ui);
        });
}
