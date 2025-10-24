use crate::egui::text::CCursorRange;
use crate::egui::text::CCursor;
use crate::ui::gui::MyApp;
use crate::models::file_items::FileSystemItem;
use std::fs;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let mut should_close = false;
    let mut should_rename = false;
    let mut rename_result: Option<(std::path::PathBuf, std::path::PathBuf, String)> = None;

    egui::Window::new("Rename File")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            // Clone the path at the start to avoid borrowing issues
            let path = app.rename_file_path.clone();

            if let Some(path) = path {
                ui.label("Filename:");

                let response = ui.text_edit_singleline(&mut app.rename_input);

                // Auto-focus and position cursor before the extension when window opens
                if app.rename_just_opened {
                    response.request_focus();

                    // Position cursor before the file extension
                    if let Some(extension_pos) = app.rename_input.rfind('.') {
                        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                            let cursor_range = CCursorRange::one(
                                CCursor::new(extension_pos)
                            );
                            state.cursor.set_char_range(Some(cursor_range));
                            state.store(ui.ctx(), response.id);
                        }
                    }

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

                        match fs::rename(&path, &new_path) {
                            Ok(_) => {
                                app.set_error_message(format!("✅ Renamed to {}", app.rename_input));
                                should_close = true;

                                // Store the rename info to update after the window closes
                                rename_result = Some((path.clone(), new_path, app.rename_input.clone()));
                            }
                            Err(e) => {
                                app.set_error_message(format!("Failed to rename: {}", e));
                            }
                        }
                    }
                }
            }
        });

    // Update the specific item after the window closes
    if let Some((old_path, new_path, new_name)) = rename_result {
        // Find and update the item in current_items
        for item in &mut app.current_items {
            match item {
                FileSystemItem::SvgFile { name, path } if *path == old_path => {
                    *name = new_name.clone();
                    *path = new_path.clone();
                    break;
                }
                FileSystemItem::FontFile { name, path } if *path == old_path => {
                    *name = new_name.clone();
                    *path = new_path.clone();
                    break;
                }
                FileSystemItem::Directory { name, path } if *path == old_path => {
                    *name = new_name;
                    *path = new_path.clone();
                    break;
                }
                _ => {}
            }
        }

        // Also update selected_svg if it was the renamed file
        if let Some(selected) = &app.selected_svg {
            if *selected == old_path {
                app.selected_svg = Some(new_path);
            }
        }
    }

    if should_close {
        app.rename_file_path = None;
        app.rename_input.clear();
        app.rename_just_opened = false;
    }
}