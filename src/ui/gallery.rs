use crate::egui::RichText;
use crate::ui::gui::MyApp;
use crate::models::file_items::FileSystemItem;
use crate::utils::file_actions;
use egui::ScrollArea;
use std::path::PathBuf;

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) -> (Option<String>, Option<PathBuf>) {
    let mut navigate_to: Option<String> = None;
    let mut load_svg: Option<PathBuf> = None;

    // Determine the root path based on current view
    let root_path = match app.current_view {
        crate::ui::gui::View::Gallery => &app.vault_path,
        crate::ui::gui::View::Fonts => &app.font_path,
        _ => &app.current_path,
    };

    // Check if we're at the root directory
    let is_at_root = app.current_path == *root_path;

    // Center the grid content
    ui.horizontal(|ui| {
        // Only show back button if not at root
        if !is_at_root {
            if ui.button(RichText::new("⬅").size(20.0)).clicked() {
                let parent = get_parent_path(&app.current_path);
                // Don't go above the root path
                if parent.starts_with(root_path) {
                    navigate_to = Some(parent);
                }
            }
        }
        ui.label(RichText::from(&app.current_path).size(20.0));
    });

    ui.separator();

    if let Some(error) = &app.error_message {
        let color = if error.starts_with("✓") {
            egui::Color32::GREEN
        } else {
            egui::Color32::RED
        };
        ui.colored_label(color, error);
    }

    // Calculate columns based on available width (account for scrollbar)
    let available_width = ui.available_width() - 20.0;
    let item_width = MyApp::THUMBNAIL_SIZE.x + 25.0;
    let num_columns = (available_width / item_width).floor().max(1.0) as usize;

    ScrollArea::vertical()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
            ui.add_space(10.0);

            egui::Frame::none()
                .inner_margin(egui::Margin::symmetric(20.0, 10.0))
                .show(ui, |ui| {
                    egui::Grid::new("file_grid")
                        .num_columns(num_columns)
                        .spacing([20.0, 20.0])
                        .show(ui, |ui| {
                            for (idx, item) in app.current_items.iter().enumerate() {
                                match item {
                                    FileSystemItem::Directory { name, path } => {
                                        let button_response = ui
                                            .vertical(|ui| {
                                                ui.set_width(MyApp::THUMBNAIL_SIZE.x);
                                                ui.set_height(MyApp::THUMBNAIL_SIZE.y);

                                                let button = ui.button(
                                                    egui::RichText::new("📁")
                                                        .size(MyApp::THUMBNAIL_SIZE.y * 0.6),
                                                );
                                                ui.label(name);
                                                button
                                            })
                                            .inner;

                                        if button_response.double_clicked() {
                                            navigate_to = Some(path.to_string_lossy().to_string());
                                        }
                                    }
                                    FileSystemItem::SvgFile { name, path } => {
                                        ui.vertical(|ui| {
                                            let img_uri = format!("file://{}", path.display());
                                            let button = ui.add(egui::ImageButton::new(
                                                egui::Image::new(img_uri)
                                                    .rounding(10.0)
                                                    .fit_to_exact_size(MyApp::THUMBNAIL_SIZE),
                                            ));

                                            if button.clicked() {
                                                load_svg = Some(path.clone());
                                            }

                                            button.context_menu(|ui| {
                                                if ui.button("Edit").clicked() {
                                                    load_svg = Some(path.clone());
                                                    ui.close_menu();
                                                }
                                                if ui.button("Copy File").clicked() {
                                                    match file_actions::copy_file_to_clipboard(path) {
                                                        Ok(_) => app.error_message = Some("✓ File copied to clipboard".to_string()),
                                                        Err(e) => app.error_message = Some(format!("Failed to copy file: {}", e)),
                                                    }
                                                    ui.close_menu();
                                                }
                                                if ui.button("Rename").clicked() {
                                                    app.rename_file_path = Some(path.clone());
                                                    app.rename_input = name.clone();
                                                    app.rename_just_opened = true;
                                                    ui.close_menu();
                                                }
                                                if ui.button("File Explorer").clicked() {
                                                    file_actions::reveal_in_explorer(path);
                                                    ui.close_menu();
                                                }
                                                if ui.button("Delete").clicked() {
                                                    ui.close_menu();
                                                }
                                            });

                                            ui.label(name);
                                        });
                                    }
                                    FileSystemItem::FontFile { name, path } => {
                                        ui.vertical(|ui| {
                                            ui.set_width(MyApp::THUMBNAIL_SIZE.x);
                                            ui.set_height(MyApp::THUMBNAIL_SIZE.y);

                                            // Font icon with extension
                                            let extension = path.extension()
                                                .and_then(|e| e.to_str())
                                                .unwrap_or("")
                                                .to_uppercase();

                                            let button = ui.button(
                                                egui::RichText::new(format!("🔤\n.{}", extension))
                                                    .size(MyApp::THUMBNAIL_SIZE.y * 0.3)
                                            );

                                            if button.clicked() {
                                                // Could preview font or show details
                                            }

                                            button.context_menu(|ui| {
                                                if ui.button("Copy File").clicked() {
                                                    match file_actions::copy_file_to_clipboard(path) {
                                                        Ok(_) => app.error_message = Some("✓ File copied to clipboard".to_string()),
                                                        Err(e) => app.error_message = Some(format!("Failed to copy file: {}", e)),
                                                    }
                                                    ui.close_menu();
                                                }
                                                if ui.button("Rename").clicked() {
                                                    app.rename_file_path = Some(path.clone());
                                                    app.rename_input = name.clone();
                                                    app.rename_just_opened = true;
                                                    ui.close_menu();
                                                }
                                                if ui.button("File Explorer").clicked() {
                                                    file_actions::reveal_in_explorer(path);
                                                    ui.close_menu();
                                                }
                                                if ui.button("Delete").clicked() {
                                                    ui.close_menu();
                                                }
                                            });

                                            ui.label(name);
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

    (navigate_to, load_svg)
}

fn get_parent_path(current: &str) -> String {
    std::path::Path::new(current)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| current.to_string())
}