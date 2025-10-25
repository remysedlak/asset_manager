use crate::utils::font_loader::prepare_fonts;
use eframe::egui;
use egui_extras::install_image_loaders;

mod models;
mod ui;
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
            // Install image loaders
            install_image_loaders(&cc.egui_ctx);

            // Apply fonts to the egui context
            cc.egui_ctx.set_fonts(prepare_fonts());

            Ok(Box::new(ui::gui::MyApp::default()))
        }),
    )
}
