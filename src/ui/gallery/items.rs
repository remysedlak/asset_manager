use crate::models::file_items::FileSystemItem;
use crate::ui::gui::MyApp;
use crate::utils::file_actions;
use crate::egui::RichText;
use std::path::PathBuf;

pub fn render(
    item: &FileSystemItem,
    ui: &mut egui::Ui,
    navigate_to: &mut Option<String>,
    load_svg: &mut Option<PathBuf>,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
) {
    match item {
        FileSystemItem::Directory { name, path } => {
            render_directory(ui, name, path, navigate_to);
        }
        FileSystemItem::SvgFile { name, path } => {
            render_svg(
                ui,
                name,
                path,
                load_svg,
                pending_edit,
                pending_rename,
                pending_delete,
                pending_error,
            );
        }
        FileSystemItem::FontFile { name, path } => {
            render_font(
                ui,
                name,
                path,
                pending_edit,
                pending_rename,
                pending_delete,
                pending_error,
            );
        }
    }
}

fn render_directory(
    ui: &mut egui::Ui,
    name: &str,
    path: &PathBuf,
    navigate_to: &mut Option<String>,
) {
    ui.vertical(|ui| {
        ui.set_width(MyApp::THUMBNAIL_SIZE.x);
        ui.set_height(MyApp::THUMBNAIL_SIZE.y);

        let button = ui.add(
            egui::Button::new(RichText::new("📁").size(MyApp::THUMBNAIL_SIZE.y * 0.6))
                .corner_radius(10.0)
                .min_size(MyApp::THUMBNAIL_SIZE),
        );

        if button.double_clicked() {
            *navigate_to = Some(path.to_string_lossy().to_string());
        }

        ui.label(RichText::from(name).size(11.0));
    });
}

fn render_svg(
    ui: &mut egui::Ui,
    name: &str,
    path: &PathBuf,
    load_svg: &mut Option<PathBuf>,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
) {
    ui.vertical(|ui| {
        let img_uri = format!("file://{}", path.display());
        let button = ui.add(
            egui::Button::new(egui::Image::new(img_uri).fit_to_exact_size(MyApp::THUMBNAIL_SIZE))
                .corner_radius(10.0),
        );

        if button.clicked() {
            *load_svg = Some(path.clone());
        }

        show_context_menu(
            button,
            ui,
            path,
            name,
            true,
            pending_edit,
            pending_rename,
            pending_delete,
            pending_error,
        );

        ui.label(RichText::from(name).size(11.0));
    });
}

fn render_font(
    ui: &mut egui::Ui,
    name: &str,
    path: &PathBuf,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
) {
    ui.vertical(|ui| {
        ui.set_width(MyApp::THUMBNAIL_SIZE.x);
        ui.set_height(MyApp::THUMBNAIL_SIZE.y);

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_uppercase();

        let button = ui.add(
            egui::Button::new(
                RichText::new(format!("🔤\n.{}", extension))
                    .size(MyApp::THUMBNAIL_SIZE.y * 0.25),
            )
                .corner_radius(10.0)
                .min_size(MyApp::THUMBNAIL_SIZE),
        );

        show_context_menu(
            button,
            ui,
            path,
            name,
            false,
            pending_edit,
            pending_rename,
            pending_delete,
            pending_error,
        );

        ui.label(RichText::from(name).size(11.0));
    });
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