use egui::ScrollArea;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use crate::ui::gui::MyApp;
use crate::models::gui::View;

// For when called with Ui (already inside a panel)
pub fn render_editor(app: &mut MyApp, ui: &mut egui::Ui) {
    // Determine which code to edit and which syntax to use
    let (code_buffer, syntax, is_svg) = if app.selected_svg.is_some() {
        (&mut app.svg_code, Syntax::python(), true)  // Changed to xml() for SVG syntax
    } else {
        (&mut app.code, Syntax::rust(), false)
    };

    // Show file name if editing SVG
    if is_svg {
        if let Some(svg_path) = &app.selected_svg {
            ui.horizontal(|ui| {
                ui.label("📄");
                ui.label(svg_path.file_name().unwrap().to_string_lossy());
            });
            ui.add_space(8.0);
        }
    }

    // Code editor
    ScrollArea::vertical().show(ui, |ui| {
        CodeEditor::default()
            .id_source("code editor")
            .with_rows(20)
            .with_fontsize(14.0)
            .with_theme(ColorTheme::GRUVBOX)
            .with_syntax(syntax)
            .with_numlines(true)
            .show(ui, code_buffer);
    });

    ui.add_space(8.0);

    // Save button (only show when editing SVG)
    if is_svg {
        ui.horizontal(|ui| {
            if ui.button("💾 Save").clicked() {
                app.save_svg();
            }

            if ui.button("✖ Close").clicked() {
                // Go back to Gallery view, keeping the SVG selected
                // so the svg_overview panel stays open
                app.current_view = View::Gallery;
            }
        });
    }
}