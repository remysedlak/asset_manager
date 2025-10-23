use eframe::egui;
pub mod models;
mod file_finder;

use models::graphic_file;

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
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            age: 25,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello from Egui!");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Click me!").clicked() {
                println!("Button clicked!");
            }

            ui.separator();

            ui.label(format!("Hello, {}! You are {} years old.", self.name, self.age));
        });
    }
}