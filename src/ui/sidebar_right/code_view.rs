use crate::models::gui::MyApp;
use egui::{RichText};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

const BUTTON_HEIGHT: f32 = 35.0;

pub fn render(app: &mut MyApp, ui: &mut egui::Ui) {
    render_header(ui);
    render_code_editor(app, ui);
    render_buttons(app, ui);
}

fn render_header(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Code").size(16.0).strong());
    });
    ui.add_space(5.0);
}

fn render_code_editor(app: &mut MyApp, ui: &mut egui::Ui) {
    // Calculate remaining space for editor
    let available_height = ui.available_height() - BUTTON_HEIGHT - 10.0;

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .max_height(available_height)
        .show(ui, |ui| {
            CodeEditor::default()
                .id_source("sidebar_code_editor")
                .with_rows(15)
                .with_fontsize(11.0)
                .with_theme(ColorTheme::GRUVBOX)
                .with_syntax(Syntax::python()) // Use xml() for better SVG syntax
                .with_numlines(true)
                .show(ui, &mut app.svg_code);
        });
}

fn render_buttons(app: &mut MyApp, ui: &mut egui::Ui) {
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Open Editor").clicked() {
                app.current_view = crate::models::gui::View::Editor;
            }

            if ui.button("Copy Code").clicked() {
                app.copy_svg_to_clipboard();
            }
        });
    });
}