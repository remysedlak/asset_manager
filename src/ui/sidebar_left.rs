use crate::models::gui::View;
use crate::models::gui::MyApp;
use egui::{RichText, SidePanel};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    SidePanel::left("my_left_panel")
        .exact_width(54.0)
        .frame(egui::Frame::default().inner_margin(egui::Margin::same(5)).fill(egui::Color32::from_rgb(30, 29, 25)))
        .show(ctx, |ui| {
            // Set button corner_radius
            ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(10);
            ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(10);
            ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::same(10);

            // Make buttons transparent
            ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;

            // Optional: customize hover and active states
            ui.style_mut().visuals.widgets.hovered.bg_fill =
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 20);
            ui.style_mut().visuals.widgets.active.bg_fill =
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 30);

            // Top buttons
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);

                // SVG View
                if ui
                    .add_sized(
                        [32.0, 32.0],
                        egui::Button::new(RichText::new("🎨").size(22.0)),
                    )
                    .on_hover_text("View SVGs")
                    .clicked()
                {
                    app.current_view = View::Gallery;
                    app.refresh_directory();
                }

                ui.add_space(8.0);

                // Font View
                if ui
                    .add_sized(
                        [32.0, 32.0],
                        egui::Button::new(RichText::new("Aa").size(18.0)),
                    )
                    .on_hover_text("Fonts")
                    .clicked()
                {
                    app.current_view = View::Fonts;
                    app.refresh_directory();
                }
            });

            // Spacer to push bottom buttons down
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(8.0);

                // Settings View
                if ui
                    .add_sized(
                        [32.0, 32.0],
                        egui::Button::new(RichText::new("⚙").size(20.0)),
                    )
                    .on_hover_text("Settings")
                    .clicked()
                {
                    app.current_view = View::Settings;
                    app.vault_path_input = app.vault_path.clone();
                }

                ui.add_space(8.0);
                // Help View
                if ui
                    .add_sized(
                        [32.0, 32.0],
                        egui::Button::new(RichText::new("?").size(18.0)),
                    )
                    .on_hover_text("Help")
                    .clicked()
                {
                    app.current_view = View::Help;
                    app.vault_path_input = app.vault_path.clone();
                }

                ui.add_space(8.0);
            });
        });
}
