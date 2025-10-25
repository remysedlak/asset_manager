use std::path::PathBuf;
use std::time::Instant;
use arboard::Clipboard;
use crate::models::FileSystemItem;

#[derive(PartialEq)]
pub enum View {
    Gallery,
    Settings,
    Fonts,
    Help,
    Editor,
}

pub struct MyApp {
    pub(crate) grid_reset_counter: usize,
    pub(crate) vault_path: String,
    pub(crate) current_path: String,
    pub(crate) font_path: String,
    pub(crate) vault_path_input: String,
    pub(crate) current_font_input: String,
    pub(crate) current_items: Vec<FileSystemItem>,
    pub(crate) selected_svg: Option<PathBuf>,
    pub(crate) svg_code: String,
    pub(crate) error_message: Option<String>,
    pub(crate) error_message_time: Option<Instant>,
    pub(crate) rename_file_path: Option<PathBuf>,
    pub(crate) rename_input: String,
    pub(crate) rename_just_opened: bool,
    pub(crate) current_view: View,
    pub(crate) delete_file_path: Option<PathBuf>,
    pub(crate) clipboard: Clipboard,
    pub(crate) code: String,
    pub(crate) reset_panel_width: bool,
    pub(crate) gallery_options: bool,
    pub(crate) thumbnail_size: f32,

    pub(crate) search_active: bool,
    pub(crate) search_query: String,

    pub(crate) sort_ascending: bool,
}