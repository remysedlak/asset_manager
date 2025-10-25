use crate::ui::gui::{MyApp};
use crate::models::gui::View;
use egui::{SidePanel, RichText};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    SidePanel::left("my_left_panel")
        .exact_width(54.0)
        .frame(egui::Frame::default()
            .inner_margin(egui::Margin::same(5))
            .fill(ctx.style().visuals.panel_fill))
        .show(ctx, |ui| {
            // Set button corner_radius
            ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(10);
            ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(10);
            ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::same(10);

            ui.vertical_centered(|ui| {
                ui.add_space(8.0);

                // SVG View
                if ui
                    .add_sized(
                        [32.0, 32.0],
                        egui::Button::new(RichText::new("🎨").size(28.0)),
                    )
                    .on_hover_text("View SVGs")
                    .clicked()
                {
                    app.current_view = View::Gallery;
                    app.refresh_directory();
                }
                ui.label(RichText::from("svg").size(12.0));

                ui.add_space(8.0);

                // Font View
                if ui
                    .add_sized(
                        [32.0, 32.0],
                        egui::Button::new(RichText::new("α").size(28.0)),
                    )
                    .on_hover_text("Fonts")
                    .clicked()
                {
                    app.current_view = View::Fonts;
                    app.refresh_directory();
                }
                ui.label(RichText::from("fonts").size(12.0));

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
                ui.label(RichText::from("settings").size(12.0));

                ui.add_space(8.0);

                // Help View
                if ui
                    .add_sized(
                        [32.0, 32.0],
                        egui::Button::new(RichText::new("?").size(28.0)),
                    )
                    .on_hover_text("Help")
                    .clicked()
                {
                    app.current_view = View::Help;
                    app.vault_path_input = app.vault_path.clone();
                }
                ui.label(RichText::from("help").size(12.0));
            });
        });
}