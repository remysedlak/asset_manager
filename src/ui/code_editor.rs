use crate::ui::gui::MyApp;
use egui::{ScrollArea, SidePanel};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    SidePanel::right("side_panel")
        .min_width(400.0)
        .frame(
            egui::Frame::default()
                .inner_margin(egui::Margin::same(8.0))
                .fill(egui::Color32::from_rgb(54, 57, 63))
        )
        .show(ctx, |ui| {
            ui.heading("SVG Editor");
            ui.separator();

            if let Some(svg_path) = &app.selected_svg {
                ui.label(format!("File: {}", svg_path.file_name().unwrap().to_string_lossy()));
            }

            ui.horizontal(|ui| {
                if ui.button("📋 Copy").clicked() {
                    app.copy_svg_to_clipboard();
                }
                if ui.button("💾 Save").clicked() {
                    app.save_svg();
                }
            });

            ui.separator();

            ui.label("Code:");
            ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut app.svg_code)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                    );
                });

            ui.separator();

            ui.label("Preview:");
            if let Some(svg_path) = &app.selected_svg {
                let img_uri = format!("file://{}", svg_path.display());
                ui.add(
                    egui::Image::new(img_uri)
                        .fit_to_exact_size(MyApp::THUMBNAIL_SIZE * 2.0)
                );
            }
        });
}