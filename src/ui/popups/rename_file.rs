use crate::ui::gui::MyApp;
use std::fs;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let mut should_close = false;
    let mut should_rename = false;

    egui::Window::new("Rename File")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            if let Some(path) = &app.rename_file_path {
                ui.label("Filename:");

                let response = ui.text_edit_singleline(&mut app.rename_input);

                // Auto-focus the text field when window opens
                if app.rename_just_opened {
                    response.request_focus();
                    app.rename_just_opened = false;
                }

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("✓ Rename").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                        should_rename = true;
                    }

                    if ui.button("✖ Cancel").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        should_close = true;
                    }
                });

                if should_rename && !app.rename_input.is_empty() {
                    if let Some(parent) = path.parent() {
                        let new_path = parent.join(&app.rename_input);

                        match fs::rename(path, &new_path) {
                            Ok(_) => {
                                app.error_message = Some(format!("✓ Renamed to {}", app.rename_input));
                                app.refresh_directory();
                                should_close = true;
                            }
                            Err(e) => {
                                app.error_message = Some(format!("Failed to rename: {}", e));
                            }
                        }
                    }
                }
            }
        });

    if should_close {
        app.rename_file_path = None;
        app.rename_input.clear();
        app.rename_just_opened = false;
    }
}