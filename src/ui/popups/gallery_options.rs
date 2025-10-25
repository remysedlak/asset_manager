use egui::RichText;
use crate::models::gui::MyApp;

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    let mut open = true;
    egui::Window::new("Gallery Configuration")
        .resizable(false)
        .collapsible(false)
        .open(&mut open)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {

                ui.add_space(10.0);

                // Thumbnail size control with arrows
                ui.horizontal(|ui| {
                    ui.label("Thumbnail Size:");

                    ui.add_space(10.0);

                    // Down arrow button
                    if ui.button("◀").clicked() && app.thumbnail_size > 1.0 {
                        app.thumbnail_size -= 1.0;
                        app.grid_reset_counter += 1;
                    }

                    // Display the current value
                    ui.label(RichText::from(format!("{}", app.thumbnail_size)).strong().size(16.0));

                    // Up arrow button
                    if ui.button("▶").clicked() && app.thumbnail_size < 20.0 {
                        app.thumbnail_size += 1.0;
                        app.grid_reset_counter += 1;
                    }
                });

                ui.add_space(10.0);
            });
        });
    // Check if window was closed
    if !open {
        app.gallery_options = false;
    }
}