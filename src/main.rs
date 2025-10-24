use eframe::egui;
use egui_extras::install_image_loaders;

mod models;
mod utils;
mod gui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Asset Manager",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx); // egui image loader functionality
            Ok(Box::new(gui::MyApp::default()))
        }),
    )
}