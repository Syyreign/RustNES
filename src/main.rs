use rodio::{source::{Source}};

mod rustnes; 

fn main() {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 550.0)),
        min_window_size: Some(egui::vec2(400.0, 550.0)),
        ..Default::default()
    };
    eframe::run_native(
        "RustNES",
        options,
        Box::new(|_cc| Box::new(rustnes::RustNES::default())),
    )
}

/// Egui base.
/// Each of the method calls below are to a specific part of the UI
impl eframe::App for rustnes::RustNES {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            self.navigation_bar(ui);
            
            self.note_stepper(ui);

            self.channel_selector(ui);

            self.control_bar(ui);

        });
    }
}