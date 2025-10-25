pub mod header;
pub mod grid;
pub mod items;
pub mod helpers;

use crate::ui::gui::MyApp;
use std::path::PathBuf;

// Main render function - includes header
pub fn render(app: &mut MyApp, ui: &mut egui::Ui) -> (Option<String>, Option<PathBuf>) {
    let mut navigate_to: Option<String> = None;
    let mut load_svg: Option<PathBuf> = None;
    let mut pending_error: Option<String> = None;
    let mut pending_edit: Option<PathBuf> = None;
    let mut pending_rename: Option<(PathBuf, String)> = None;
    let mut pending_delete: Option<PathBuf> = None;

    // Clone everything we need BEFORE any borrows
    let root_path = helpers::get_root_path(app).clone();
    let current_path = app.current_path.clone();
    let is_at_root = current_path == root_path;
    let display_path = helpers::calculate_display_path(&current_path, &root_path);

    // Render header with navigation and controls
    header::render(app, ui, &mut navigate_to, is_at_root, &root_path, &display_path);

    // Render status messages
    header::render_status_messages(app, ui);

    // Render the file grid
    grid::render(
        app,
        ui,
        &mut navigate_to,
        &mut load_svg,
        &mut pending_edit,
        &mut pending_rename,
        &mut pending_delete,
        &mut pending_error,
    );

    // Apply pending actions
    helpers::apply_pending_actions(
        app,
        &mut load_svg,
        pending_edit,
        pending_rename,
        pending_delete,
        pending_error,
    );

    (navigate_to, load_svg)
}

// Render only content (without header) - for when header is rendered separately
pub fn render_content(app: &mut MyApp, ui: &mut egui::Ui) -> (Option<String>, Option<PathBuf>) {
    let mut navigate_to: Option<String> = None;
    let mut load_svg: Option<PathBuf> = None;
    let mut pending_error: Option<String> = None;
    let mut pending_edit: Option<PathBuf> = None;
    let mut pending_rename: Option<(PathBuf, String)> = None;
    let mut pending_delete: Option<PathBuf> = None;

    // Render the file grid only (no header)
    grid::render(
        app,
        ui,
        &mut navigate_to,
        &mut load_svg,
        &mut pending_edit,
        &mut pending_rename,
        &mut pending_delete,
        &mut pending_error,
    );

    // Apply pending actions
    helpers::apply_pending_actions(
        app,
        &mut load_svg,
        pending_edit,
        pending_rename,
        pending_delete,
        pending_error,
    );

    (navigate_to, load_svg)
}