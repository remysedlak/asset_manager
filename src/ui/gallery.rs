use crate::egui::RichText;
use crate::ui::gui::MyApp;
use crate::models::file_items::FileSystemItem;
use crate::utils::file_actions;
use egui::ScrollArea;
use std::path::PathBuf;
use crate::models::gui::View;

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) -> (Option<String>, Option<PathBuf>) {
    let mut navigate_to: Option<String> = None;
    let mut load_svg: Option<PathBuf> = None;
    let mut pending_error: Option<String> = None;
    let mut pending_edit: Option<PathBuf> = None;
    let mut pending_rename: Option<(PathBuf, String)> = None;
    let mut pending_delete: Option<PathBuf> = None;

    // Determine the root path based on current view
    let root_path = match app.current_view {
        View::Gallery => &app.vault_path,
        View::Fonts => &app.font_path,
        _ => &app.current_path,
    };

    let is_at_root = app.current_path == *root_path;

    // Calculate relative path for display
    let display_path = if let Ok(relative) = std::path::Path::new(&app.current_path)
        .strip_prefix(root_path)
    {
        if relative.as_os_str().is_empty() {
            // At root, show just the folder name
            std::path::Path::new(root_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(root_path)
                .to_string()
        } else {
            // Show relative path from root
            format!(
                "{}/{}",
                std::path::Path::new(root_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(""),
                relative.display()
            )
        }
    } else {
        // Fallback to full path if strip_prefix fails
        app.current_path.clone()
    };

    // Header
    ui.horizontal(|ui| {
        if !is_at_root {
            if ui.button(RichText::new("⬅").size(20.0)).clicked() {
                let parent = get_parent_path(&app.current_path);
                if parent.starts_with(root_path) {
                    navigate_to = Some(parent);
                }
            }
        }
        ui.label(RichText::from(&display_path).size(20.0));
    });

    ui.separator();

    // Status message
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

    // Grid setup - recalculate every frame based on current available width
    let available_width = ui.available_width() - 20.0;
    let item_width = MyApp::THUMBNAIL_SIZE.x + 25.0;
    let num_columns = (available_width / item_width).floor().max(1.0) as usize;

    ScrollArea::vertical()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
            ui.add_space(10.0);

            egui::Frame::new()
                .inner_margin(egui::Margin::symmetric(20, 10))
                .show(ui, |ui| {
                    // Generate unique grid ID based on number of columns
                    // This forces the grid to reset when column count changes
                    let grid_id = format!("file_grid_{}_{}", num_columns, app.grid_reset_counter);

                    egui::Grid::new(grid_id)
                        .num_columns(num_columns)
                        .spacing([20.0, 20.0])
                        .min_col_width(0.0)  // Don't enforce minimum column width
                        .max_col_width(MyApp::THUMBNAIL_SIZE.x + 25.0)  // Set max column width
                        .show(ui, |ui| {
                            for (idx, item) in app.current_items.iter().enumerate() {
                                match item {
                                    FileSystemItem::Directory { name, path } => {
                                        ui.vertical(|ui| {
                                            ui.set_width(MyApp::THUMBNAIL_SIZE.x);
                                            ui.set_height(MyApp::THUMBNAIL_SIZE.y);

                                            let button = ui.add(
                                                egui::Button::new(
                                                    RichText::new("📁").size(MyApp::THUMBNAIL_SIZE.y * 0.6)
                                                )
                                                    .corner_radius(10.0)
                                                    .min_size(MyApp::THUMBNAIL_SIZE)
                                            );

                                            if button.double_clicked() {
                                                navigate_to = Some(path.to_string_lossy().to_string());
                                            }

                                            ui.label(RichText::from(name).size(11.0));
                                        });
                                    }
                                    FileSystemItem::SvgFile { name, path } => {
                                        ui.vertical(|ui| {
                                            let img_uri = format!("file://{}", path.display());
                                            let button = ui.add(egui::Button::new(
                                                egui::Image::new(img_uri)
                                                    .fit_to_exact_size(MyApp::THUMBNAIL_SIZE),
                                            ).corner_radius(10.0));

                                            if button.clicked() {
                                                load_svg = Some(path.clone());
                                                app.reset_panel_width = true;
                                            }

                                            show_context_menu(button, ui, path, name, true,
                                                              &mut pending_edit, &mut pending_rename,
                                                              &mut pending_delete, &mut pending_error);

                                            ui.label(RichText::from(name).size(11.0));
                                        });
                                    }
                                    FileSystemItem::FontFile { name, path } => {
                                        ui.vertical(|ui| {
                                            ui.set_width(MyApp::THUMBNAIL_SIZE.x);
                                            ui.set_height(MyApp::THUMBNAIL_SIZE.y);

                                            let extension = path.extension()
                                                .and_then(|e| e.to_str())
                                                .unwrap_or("")
                                                .to_uppercase();

                                            let button = ui.add(
                                                egui::Button::new(
                                                    RichText::new(format!("🔤\n.{}", extension))
                                                        .size(MyApp::THUMBNAIL_SIZE.y * 0.25)
                                                )
                                                    .corner_radius(10.0)
                                                    .min_size(MyApp::THUMBNAIL_SIZE)
                                            );

                                            show_context_menu(button, ui, path, name, false,
                                                              &mut pending_edit, &mut pending_rename,
                                                              &mut pending_delete, &mut pending_error);

                                            ui.label(RichText::from(name).size(11.0));
                                        });
                                    }
                                }

                                if (idx + 1) % num_columns == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                });
        });

    // Apply pending actions
    if let Some(path) = pending_edit {
        load_svg = Some(path);
        app.current_view = View::Editor;
    }

    if let Some((path, name)) = pending_rename {
        app.rename_file_path = Some(path);
        app.rename_input = name;
        app.rename_just_opened = true;
    }

    if let Some(path) = pending_delete {
        app.delete_file_path = Some(path);
    }

    if let Some(error) = pending_error {
        app.set_error_message(error);
    }

    (navigate_to, load_svg)
}

fn show_context_menu(
    response: egui::Response,
    _ui: &mut egui::Ui,
    path: &PathBuf,
    name: &str,
    is_svg: bool,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
) {
    response.context_menu(|ui| {
        if is_svg && ui.button("Edit").clicked() {
            *pending_edit = Some(path.clone());
            ui.close();
        }

        if ui.button("Copy File").clicked() {
            match file_actions::copy_file_to_clipboard(path) {
                Ok(_) => *pending_error = Some("✅ File copied to clipboard".to_string()),
                Err(e) => *pending_error = Some(format!("Failed to copy file: {}", e)),
            }
            ui.close();
        }

        if ui.button("Rename").clicked() {
            *pending_rename = Some((path.clone(), name.to_string()));
            ui.close();
        }

        if ui.button("File Explorer").clicked() {
            file_actions::reveal_in_explorer(path);
            ui.close();
        }

        if ui.button("Delete").clicked() {
            *pending_delete = Some(path.clone());
            ui.close();
        }
    });
}

fn get_parent_path(current: &str) -> String {
    std::path::Path::new(current)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| current.to_string())
}