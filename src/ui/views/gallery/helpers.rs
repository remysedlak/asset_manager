use crate::models::gui::MyApp;
use crate::models::gui::View;
use std::path::PathBuf;

pub fn get_root_path(app: &MyApp) -> &String {
    match app.current_view {
        View::Gallery => &app.vault_path,
        View::Fonts => &app.font_path,
        _ => &app.current_path,
    }
}

pub fn calculate_display_path(current_path: &str, root_path: &str) -> String {
    if let Ok(relative) = std::path::Path::new(current_path).strip_prefix(root_path) {
        if relative.as_os_str().is_empty() {
            std::path::Path::new(root_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(root_path)
                .to_string()
        } else {
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
        current_path.to_string()
    }
}

pub fn get_parent_path(current: &str) -> String {
    std::path::Path::new(current)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| current.to_string())
}

pub fn apply_pending_actions(
    app: &mut MyApp,
    load_svg: &mut Option<PathBuf>,
    pending_edit: Option<PathBuf>,
    pending_rename: Option<(PathBuf, String)>,
    pending_delete: Option<PathBuf>,
    pending_error: Option<String>,
) {
    if let Some(path) = pending_edit {
        *load_svg = Some(path);
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

    if load_svg.is_some() {
        app.reset_panel_width = true;
    }
}