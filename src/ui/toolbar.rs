use crate::ui::gui::{MyApp, View};
use egui::{SidePanel, RichText};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    SidePanel::left("my_left_panel")
        .max_width(60.0)
        .frame(egui::Frame::default().inner_margin(egui::Margin::same(8.0)))
        .show(ctx, |ui| {
            // SVG View
            if ui
                .add_sized(
                    [40.0, 40.0],
                    egui::Button::new(RichText::new("🎨").size(24.0)),
                )
                .on_hover_text("View SVGs")
                .clicked()
            {
                app.current_view = View::Gallery;
                app.refresh_directory();
            }
            // Settings View
            if ui
                .add_sized(
                    [40.0, 40.0],
                    egui::Button::new(RichText::new("⚙").size(24.0)),
                )
                .on_hover_text("Settings")
                .clicked()
            {
                app.current_view = View::Settings;
                app.vault_path_input = app.vault_path.clone();
            }
        });
}