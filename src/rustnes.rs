use egui::{Color32};

// Moved waves below rustnes to allow mod to play waves, and clear up main
mod waves;

mod synth;

pub(crate) struct RustNES {
    // Test variable for the GUI. Displays currently selected files name
    pub(crate) _picked_path: Option<String>,
    pub(crate) _test_bool: bool,
    pub(crate) volume: f32,
    pub(crate) synth: synth::Synth,
    pub(crate) unselected_color: Color32,
    pub(crate) selected_color: Color32,
}

impl Default for RustNES {
    fn default() -> Self {
        Self {
            _picked_path: None,
            _test_bool: false,
            volume: 100.0,
            synth: synth::Synth::default(),
            unselected_color: Color32::from_rgb(100, 100, 100),
            selected_color: Color32::from_rgb(80, 200, 80),
        }
    }
}

impl RustNES{  
    pub(crate) fn navigation_bar(&mut self, ui: &mut egui::Ui){

        egui::TopBottomPanel::top("navigation_menu")
        .resizable(false)
        .min_height(25.0)
        .show_inside(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.menu_button("File", Self::file_menu);
                ui.menu_button("Edit", Self::edit_menu);
    
                #[cfg(debug_assertions)]
                ui.menu_button("Debug", Self::debug_menu);
            });
        });
    }

    /// The File context menu
    /// Contains New, Open File, Save, Export (MIDI/NSF), Import (MIDI/NSF)
    pub(crate) fn file_menu(ui: &mut egui::Ui) {
        if ui.button("New").clicked() {
            println!("TODO! clear current work");
            ui.close_menu();
        }

        if ui.button("Open File...").clicked() {
            // rfd is used to access files
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                Some(path.display().to_string());
            }
            ui.close_menu();
        }

        if ui.button("Save").clicked() {
            println!("TODO! save current work");
            ui.close_menu();
        }

        ui.menu_button("Export", |ui| {
            if ui.button("MIDI").clicked() {
                println!("TODO! export work as MIDI");
                ui.close_menu();
            }
            if ui.button("NSF").clicked() {
                println!("TODO! export work as NSF");
                ui.close_menu();
            }
        });

        ui.menu_button("Import", |ui| {
            if ui.button("MIDI").clicked() {
                println!("TODO! import MIDI");
                ui.close_menu();
            }
            if ui.button("NSF").clicked() {
                println!("TODO! import NSF");
                ui.close_menu();
            }
        });

    }

    /// The edit context menu
    pub(crate) fn edit_menu(ui: &mut egui::Ui) {
        if ui.button("STUB").clicked() {
            println!("TODO!");
            ui.close_menu();
        }
    }

    /// The debug contect menu.
    /// This menu should only be visible in debug mode
    pub(crate) fn debug_menu(ui: &mut egui::Ui) {
        if ui.button("Play NES Triangle").clicked(){
            synth::play_nes_triangle_wave(440.0);
        }

        if ui.button("Play NES Pulse").clicked(){
            synth::play_nes_pulse_wave(440.0);
        }

        if ui.button("Play NES Noise").clicked(){
            synth::play_nes_noise();
        }
    }

    ///
    /// The botton control bar with play, pause, and volume
    /// 
    pub(crate) fn control_bar(&mut self, ui: &mut egui::Ui){
        egui::TopBottomPanel::bottom("control_menu")
        .resizable(false)
        .min_height(25.0)
        .show_inside(ui, |ui| {
            ui.horizontal_centered(|ui| {
                if ui.button("Play").clicked(){
                    self.synth.play();
                }
                if ui.button("Pause").clicked(){
                    println!("TODO!"); 
                }
                ui.add(egui::Slider::new(&mut self.volume, 0.0..=100.0).show_value(false));
            });
        });
    }

    pub(crate) fn note_stepper(&mut self, ui: &mut egui::Ui){

        ui.columns(8, |columns|{ 

            for i in 0..self.synth.get_columns_len(){

                let curr: &mut synth::SequenceColumn = &mut self.synth.sequence_columns[i];

                for j in (0..12).rev(){
                    if columns[i].add(
                    egui::Button::new("").
                    fill(
                        if curr.is_selected(j) {self.selected_color} 
                        else { self.unselected_color}
                    )).clicked(){
                        curr.select(j);
                        println!("{}",curr.get_index());
                    };
                }
            }
        });

    }
}