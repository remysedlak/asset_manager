use std::sync::Arc;
use eframe::egui;
use egui_extras::install_image_loaders;

mod models;
mod utils;
mod ui;

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

            // Set up custom fonts
            let mut fonts = egui::FontDefinitions::default();

            // Load your custom font
            fonts.font_data.insert(
                "my_font".to_owned(),
                Arc::from(egui::FontData::from_static(include_bytes!("../assets/clash.ttf")))
            );

            // Set as primary font for proportional text
            fonts.families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "my_font".to_owned());

            // Optionally set for monospace too
            fonts.families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .insert(0, "my_font".to_owned());

            // Apply fonts to the egui context
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(ui::gui::MyApp::default()))
        }),
    )
}