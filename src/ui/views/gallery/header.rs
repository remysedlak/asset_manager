use crate::models::gui::MyApp;
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
        .fill(egui::Color32::from_rgb(30, 29, 25))
        .inner_margin(egui::Margin::same(5))
        .show(ui, |ui| {
            // Style buttons to be transparent
            ui.style_mut().visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(10);
            ui.style_mut().visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(10);
            ui.style_mut().visuals.widgets.active.corner_radius = egui::CornerRadius::same(10);

            ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;

            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_premultiplied(255, 255, 255, 20);
            ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgba_premultiplied(255, 255, 255, 30);

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

                // Path display or search bar
                if app.search_active {
                    ui.add(
                        egui::TextEdit::singleline(&mut app.search_query)
                            .hint_text("Search files...")
                            .desired_width(300.0)
                    );

                    if ui.button("x").clicked() {
                        app.search_active = false;
                        app.search_query.clear();
                    }
                } else {
                    ui.label(RichText::from(display_path).size(20.0));
                }

                // Control buttons on the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {

                    if ui.button(RichText::new("⚙").size(16.0))
                        .on_hover_text("View Options")
                        .clicked()
                    {
                        app.gallery_options = !app.gallery_options;
                    }

                    let sort_icon = if app.sort_ascending { "⬆" } else { "⬇" };
                    if ui.button(RichText::new(sort_icon).size(16.0))
                        .on_hover_text(if app.sort_ascending { "Sorted A-Z" } else { "Sorted Z-A" })
                        .clicked()
                    {
                        app.sort_ascending = !app.sort_ascending;
                        helpers::sort_items(&mut app.current_items, app.sort_ascending);
                    }

                    if ui.button(RichText::new("🔍").size(16.0))
                        .on_hover_text("Search")
                        .clicked()
                    {
                        app.search_active = !app.search_active;
                        if !app.search_active {
                            app.search_query.clear();
                        }
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