use crate::ui::gui::MyApp;
use std::fs;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let mut should_close = false;
    let mut should_rename = false;
    let mut old_path_clone = None;
    let mut new_path_clone = None;
    let mut new_name_clone = None;

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
                    if ui.button("Rename").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                        should_rename = true;
                    }

                    if ui.button("Cancel").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        should_close = true;
                    }
                });

                if should_rename && !app.rename_input.is_empty() {
                    if let Some(parent) = path.parent() {
                        let new_path = parent.join(&app.rename_input);

                        match fs::rename(path, &new_path) {
                            Ok(_) => {
                                app.error_message = Some(format!("✓ Renamed to {}", app.rename_input));

                                // Store the values to update after the closure
                                old_path_clone = Some(path.clone());
                                new_path_clone = Some(new_path);
                                new_name_clone = Some(app.rename_input.clone());

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

    // Update the renamed item after the window closure
    if let (Some(old_path), Some(new_path), Some(new_name)) = (old_path_clone, new_path_clone, new_name_clone) {
        update_renamed_item(app, &old_path, &new_path, &new_name);
    }

    if should_close {
        app.rename_file_path = None;
        app.rename_input.clear();
        app.rename_just_opened = false;
    }
}

fn update_renamed_item(app: &mut MyApp, old_path: &std::path::Path, new_path: &std::path::Path, new_name: &str) {
    use crate::models::file_items::FileSystemItem;

    // Find and update the item in current_items
    for item in &mut app.current_items {
        match item {
            FileSystemItem::SvgFile { name, path } if path == old_path => {
                *name = new_name.to_string();
                *path = new_path.to_path_buf();
                break;
            }
            FileSystemItem::FontFile { name, path } if path == old_path => {
                *name = new_name.to_string();
                *path = new_path.to_path_buf();
                break;
            }
            FileSystemItem::Directory { name, path } if path == old_path => {
                *name = new_name.to_string();
                *path = new_path.to_path_buf();
                break;
            }
            _ => {}
        }
    }

    // Also update selected_svg if it was the renamed file
    if let Some(selected) = &app.selected_svg {
        if selected == old_path {
            app.selected_svg = Some(new_path.to_path_buf());
        }
    }
}