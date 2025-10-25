use crate::models::gui::MyApp;
use crate::models::gui::View;
use crate::models::file_items::FileSystemItem;
use crate::utils::file_finder::{scan_directory_recursive, FileFilter};
use egui::ScrollArea;
use std::path::PathBuf;
use super::items;

pub fn render(
    app: &mut MyApp,
    ui: &mut egui::Ui,
    navigate_to: &mut Option<String>,
    load_svg: &mut Option<PathBuf>,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
) {
    // Extract all needed data upfront to avoid borrow issues
    let thumbnail_size = app.get_thumbnail_size();
    let scrollbar_margin = 10.0;
    let available_width = ui.available_width() - scrollbar_margin - 20.0;
    let item_width = thumbnail_size.x + 25.0;
    let num_columns = (available_width / item_width).floor().max(1.0) as usize;

    let search_active = app.search_active;
    let search_query = app.search_query.clone();
    let current_view = &app.current_view;
    let vault_path = app.vault_path.clone();
    let font_path = app.font_path.clone();
    let grid_reset_counter = app.grid_reset_counter;

    let filtered_items: Vec<FileSystemItem>;

    if search_active && !search_query.is_empty() {
        let query = search_query.to_lowercase();

        let (filter, root_path) = match current_view {
            View::Gallery => (FileFilter::Svg, vault_path),
            View::Fonts => (FileFilter::Font, font_path),
            _ => {
                let items_clone: Vec<FileSystemItem> = app.current_items.clone();
                return render_grid(ui, &items_clone, thumbnail_size, num_columns, grid_reset_counter,
                                   navigate_to, load_svg, pending_edit, pending_rename,
                                   pending_delete, pending_error, app);
            }
        };

        let search_results = scan_directory_recursive(&root_path, filter).unwrap_or_default();

        filtered_items = search_results.into_iter()
            .filter(|item| {
                let name = match item {
                    FileSystemItem::SvgFile { name, .. } => name,
                    FileSystemItem::FontFile { name, .. } => name,
                    FileSystemItem::Directory { name, .. } => name,
                };
                name.to_lowercase().contains(&query)
            })
            .collect();

        render_grid(ui, &filtered_items, thumbnail_size, num_columns, grid_reset_counter,
                    navigate_to, load_svg, pending_edit, pending_rename,
                    pending_delete, pending_error, app);
    } else {
        let items_clone: Vec<FileSystemItem> = app.current_items.clone();
        render_grid(ui, &items_clone, thumbnail_size, num_columns, grid_reset_counter,
                    navigate_to, load_svg, pending_edit, pending_rename,
                    pending_delete, pending_error, app);
    }
}

fn render_grid(
    ui: &mut egui::Ui,
    filtered_items: &[FileSystemItem],
    thumbnail_size: egui::Vec2,
    num_columns: usize,
    grid_reset_counter: usize,
    navigate_to: &mut Option<String>,
    load_svg: &mut Option<PathBuf>,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
    app: &mut MyApp,  // Move to end
) {
    let mut pending_show_sidebar = false;

    egui::Frame::none()
        .inner_margin(egui::Margin {
            left: 0,
            right: 10,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            ScrollArea::vertical()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    ui.add_space(10.0);

                    egui::Frame::new()
                        .inner_margin(egui::Margin::symmetric(20, 10))
                        .show(ui, |ui| {
                            let grid_id = format!("file_grid_{}_{}", num_columns, grid_reset_counter);

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
                                            &mut pending_show_sidebar,
                                        );

                                        if (idx + 1) % num_columns == 0 {
                                            ui.end_row();
                                        }
                                    }
                                });
                        });
                });
        });

    // Apply the sidebar flag after rendering
    if pending_show_sidebar {
        app.show_sidebar_right = true;
    }
}