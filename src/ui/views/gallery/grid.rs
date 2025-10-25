use crate::models::gui::MyApp;
use egui::ScrollArea;
use std::path::PathBuf;
use super::items;

pub fn render(
    app: &MyApp,  // Back to immutable borrow
    ui: &mut egui::Ui,
    navigate_to: &mut Option<String>,
    load_svg: &mut Option<PathBuf>,
    pending_edit: &mut Option<PathBuf>,
    pending_rename: &mut Option<(PathBuf, String)>,
    pending_delete: &mut Option<PathBuf>,
    pending_error: &mut Option<String>,
) {
    let available_width = ui.available_width() - 20.0;
    let item_width = MyApp::THUMBNAIL_SIZE.x + 25.0;
    let num_columns = (available_width / item_width).floor().max(1.0) as usize;

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
                        .max_col_width(MyApp::THUMBNAIL_SIZE.x + 25.0)
                        .show(ui, |ui| {
                            for (idx, item) in app.current_items.iter().enumerate() {
                                items::render(
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