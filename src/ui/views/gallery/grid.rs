use crate::models::gui::MyApp;
use crate::models::gui::View;
use crate::models::file_items::FileSystemItem;
use crate::utils::file_finder::{scan_directory_recursive, FileFilter};
use egui::ScrollArea;
use std::path::PathBuf;
use super::items;

pub fn render(
    app: &MyApp,
    ui: &mut egui::Ui,
    navigate_to: &mut Option<String>,
    load_svg: &mut Option<PathBuf>,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
) {
    let thumbnail_size = app.get_thumbnail_size();
    let available_width = ui.available_width() - 20.0;
    let item_width = thumbnail_size.x + 25.0;
    let num_columns = (available_width / item_width).floor().max(1.0) as usize;

    // Filter items based on search query
    let filtered_items: Vec<&FileSystemItem>;
    let search_results: Vec<FileSystemItem>;

    if app.search_active && !app.search_query.is_empty() {
        let query = app.search_query.to_lowercase();

        // Get the filter type and root path based on current view
        let (filter, root_path) = match app.current_view {
            View::Gallery => (FileFilter::Svg, &app.vault_path),
            View::Fonts => (FileFilter::Font, &app.font_path),
            _ => {
                // Fallback to current items for other views
                filtered_items = app.current_items.iter().collect();
                return render_grid(ui, &filtered_items, thumbnail_size, num_columns, app,
                                   navigate_to, load_svg, pending_edit, pending_rename,
                                   pending_delete, pending_error);
            }
        };

        // Do recursive search when search is active
        search_results = scan_directory_recursive(root_path, filter).unwrap_or_default();

        // Filter by query
        filtered_items = search_results.iter()
            .filter(|item| {
                let name = match item {
                    FileSystemItem::SvgFile { name, .. } => name,
                    FileSystemItem::FontFile { name, .. } => name,
                    FileSystemItem::Directory { name, .. } => name,
                };
                name.to_lowercase().contains(&query)
            })
            .collect();
    } else {
        // No search active, use current items
        filtered_items = app.current_items.iter().collect();
    }

    render_grid(ui, &filtered_items, thumbnail_size, num_columns, app,
                navigate_to, load_svg, pending_edit, pending_rename,
                pending_delete, pending_error);
}

fn render_grid(
    ui: &mut egui::Ui,
    filtered_items: &[&FileSystemItem],
    thumbnail_size: egui::Vec2,
    num_columns: usize,
    app: &MyApp,
    navigate_to: &mut Option<String>,
    load_svg: &mut Option<PathBuf>,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
) {
    ScrollArea::vertical()
        .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
        .show(ui, |ui| {
            ui.add_space(10.0);

            egui::Frame::new()
                .inner_margin(egui::Margin::symmetric(20, 10))
                .show(ui, |ui| {
                    let grid_id = format!("file_grid_{}_{}", num_columns, app.grid_reset_counter);

                    egui::Grid::new(grid_id)
                        .num_columns(num_columns)
                        .spacing([20.0, 20.0])
                        .min_col_width(0.0)
                        .max_col_width(thumbnail_size.x + 25.0)
                        .show(ui, |ui| {
                            for (idx, item) in filtered_items.iter().enumerate() {
                                items::render(
                                    app,
                                    item,
                                    ui,
                                    navigate_to,
                                    load_svg,
                                    pending_edit,
                                    pending_rename,
                                    pending_delete,
                                    pending_error,
                                );

                                if (idx + 1) % num_columns == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                });
        });
}