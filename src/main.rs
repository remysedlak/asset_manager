use eframe::egui;
use egui::{CentralPanel, SidePanel};

mod file_finder;
pub mod models;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Asset Manager",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    vault_path: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            vault_path: "/home/remy/Pictures/images/svg".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // side panel to access menus
        SidePanel::left("my_left_panel")
            .max_width(30.0)
            .frame(egui::Frame::default().inner_margin(egui::Margin::same(8.0)))
            .show(ctx, |ui| {
                // view my svgs
                if ui.button("🎨").on_hover_text("View SVGs").clicked() {
                    ui.add_space(4.0);
                }

                // user settings
                if ui.button("⚙").on_hover_text("Settings").clicked() {}
            });

        // main content to display app
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Filepath ");
                ui.text_edit_singleline(&mut self.vault_path);
            });
        });
    }
}
