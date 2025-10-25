use crate::ui::gui::MyApp;
use crate::egui::RichText;
use super::helpers;

pub fn render(
    app: &mut MyApp,
    ui: &mut egui::Ui,
    navigate_to: &mut Option<String>,
    is_at_root: bool,
    root_path: &str,
    display_path: &str,
) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(30, 29, 25))  // Custom background color
        .inner_margin(egui::Margin::same(5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Back button
                if !is_at_root {
                    if ui.button(RichText::new("⬅").size(20.0)).clicked() {
                        let parent = helpers::get_parent_path(&app.current_path);
                        if parent.starts_with(root_path) {
                            *navigate_to = Some(parent);
                        }
                    }
                }

                // Path display
                ui.label(RichText::from(display_path).size(20.0));

                // Control buttons on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(RichText::new("🔍").size(16.0))
                        .on_hover_text("Search")
                        .clicked()
                    {
                        // TODO: Open search dialog
                    }

                    if ui.button(RichText::new("↕").size(16.0))
                        .on_hover_text("Sort")
                        .clicked()
                    {
                        // TODO: Open sort menu
                    }

                    if ui.button(RichText::new("⚙").size(16.0))
                        .on_hover_text("View Options")
                        .clicked()
                    {
                        // TODO: Open view options
                    }
                });
            });
        });
}

pub fn render_status_messages(app: &mut MyApp, ui: &mut egui::Ui) {
    if let Some(error_time) = app.error_message_time {
        if error_time.elapsed().as_secs() >= 1 {
            app.error_message = None;
            app.error_message_time = None;
        }
    }

    if let Some(error) = &app.error_message {
        let color = if error.starts_with("✅") {
            egui::Color32::GREEN
        } else {
            egui::Color32::RED
        };
        ui.colored_label(color, error);
    }
}