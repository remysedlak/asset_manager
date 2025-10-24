use crate::ui::gui::MyApp;
use egui::{ScrollArea, SidePanel, RichText, Align};

pub fn render(app: &mut MyApp, ctx: &egui::Context) {
    SidePanel::right("side_panel")
        .min_width(300.0)
        .resizable(true)
        .frame(
            egui::Frame::default()
                .inner_margin(egui::Margin::same(16.0))
                .fill(egui::Color32::from_rgb(47, 49, 54))
        )
        .show(ctx, |ui| {
            // Header with title and close button
            ui.horizontal(|ui| {
                ui.heading(RichText::new("Editor").size(18.0));
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(RichText::new("―").size(14.0)).clicked() {
                        app.selected_svg = None;
                        app.svg_code.clear();
                    }
                });
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // File info
            if let Some(svg_path) = &app.selected_svg {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📄").size(14.0));
                    ui.label(
                        RichText::new(svg_path.file_name().unwrap().to_string_lossy())
                            .color(egui::Color32::from_rgb(180, 180, 180))
                    );
                });
                ui.add_space(8.0);
            }

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button(RichText::new("Copy").size(14.0))
                    .on_hover_text("Copy SVG code to clipboard")
                    .clicked()
                {
                    app.copy_svg_to_clipboard();
                }
                if ui.button(RichText::new("Save").size(14.0))
                    .on_hover_text("Save changes to file")
                    .clicked()
                {
                    app.save_svg();
                }
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            // Code editor section
            ui.label(RichText::new("Code:").strong().size(14.0));
            ui.add_space(4.0);

            let code_height = ui.available_height() * 0.3;

            egui::Frame::none()
                .fill(egui::Color32::from_rgb(32, 34, 37))
                .inner_margin(egui::Margin::same(8.0))
                .rounding(6.0)
                .show(ui, |ui| {
                    ScrollArea::vertical()
                        .id_source("code_editor_scroll")  // Add unique ID
                        .max_height(code_height)
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut app.svg_code)
                                    .font(egui::TextStyle::Monospace)
                                    .code_editor()
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(15)
                            );
                        });
                });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            // Preview section
            ui.label(RichText::new("Preview:").strong().size(14.0));
            ui.add_space(4.0);

            egui::Frame::none()
                .fill(egui::Color32::from_rgb(35, 39, 42))
                .inner_margin(egui::Margin::same(16.0))
                .rounding(6.0)
                .show(ui, |ui| {
                    ScrollArea::both()
                        .id_source("preview_scroll")  // Add unique ID
                        .show(ui, |ui| {
                            if let Some(svg_path) = &app.selected_svg {
                                let img_uri = format!("file://{}", svg_path.display());
                                let available_size = ui.available_size();

                                ui.centered_and_justified(|ui| {
                                    ui.add(
                                        egui::Image::new(img_uri)
                                            .max_width(available_size.x - 32.0)
                                            .max_height(available_size.y - 32.0)
                                            .shrink_to_fit()
                                    );
                                });
                            }
                        });
                });
        });
}