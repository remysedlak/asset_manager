use std::sync::Arc;
use eframe::epaint::text::FontDefinitions;

pub fn prepare_fonts() -> FontDefinitions {
    // Set up custom fonts
    let mut fonts = egui::FontDefinitions::default();

    // Load your custom font
    fonts.font_data.insert(
        "my_font".to_owned(),
        Arc::from(egui::FontData::from_static(include_bytes!("../../assets/clash.ttf")))
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
    fonts
}