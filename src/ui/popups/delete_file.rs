use crate::models::gui::MyApp;
use std::fs;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let mut should_close = false;
    let mut should_delete = false;

    egui::Window::new("Confirm Deletion")
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            // Clone the path at the start to avoid borrowing issues
            let path = app.delete_file_path.clone();

            if let Some(path) = path {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);

                    ui.label(
                        egui::RichText::new("⚠ Are you sure?")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(255, 200, 0))
                    );

                    ui.add_space(10.0);

                    ui.label("This will permanently delete:");

                    ui.add_space(5.0);

                    if let Some(filename) = path.file_name() {
                        ui.label(
                            egui::RichText::new(filename.to_string_lossy())
                                .strong()
                                .color(egui::Color32::WHITE)
                        );
                    }

                    ui.add_space(15.0);
                });

                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("Cancel").size(14.0)).clicked()
                        || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        should_close = true;
                    }

                    ui.add_space(10.0);

                    if ui.button(
                        egui::RichText::new("Delete")
                            .size(14.0)
                            .color(egui::Color32::from_rgb(255, 100, 100))
                    ).clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        should_delete = true;
                    }
                });
            }
        });

    // Clone path again for the deletion logic
    let path_to_delete = app.delete_file_path.clone();

    if should_delete {
        if let Some(path) = path_to_delete {
            match fs::remove_file(&path) {
                Ok(_) => {
                    app.set_error_message("✅ File deleted successfully".to_string());

                    // Remove the item from current_items
                    app.current_items.retain(|item| {
                        use crate::models::file_items::FileSystemItem;
                        match item {
                            FileSystemItem::SvgFile { path: p, .. } => p != &path,
                            FileSystemItem::FontFile { path: p, .. } => p != &path,
                            FileSystemItem::Directory { path: p, .. } => p != &path,
                        }
                    });

                    // Clear selected_svg if it was deleted
                    if let Some(selected) = &app.selected_svg {
                        if selected == &path {
                            app.selected_svg = None;
                            app.svg_code.clear();
                        }
                    }

                    // Force grid to reset
                    app.grid_reset_counter = app.grid_reset_counter.wrapping_add(1);
                }
                Err(e) => {
                    app.set_error_message(format!("Failed to delete: {}", e));
                }
            }
        }
        should_close = true;
    }

    if should_close {
        app.delete_file_path = None;
    }
}