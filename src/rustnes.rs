use egui::{Color32};

// Moved waves below rustnes to allow mod to play waves, and clear up main
mod waves;

mod synth;

mod filters;

pub(crate) struct RustNES {
    // Test variable for the GUI. Displays currently selected files name
    pub(crate) _picked_path: Option<String>,
    pub(crate) _test_bool: bool,
    pub(crate) synth: synth::Synth,

    pub(crate) unselected_color: Color32,
    pub(crate) selected_color: Color32,

    pub(crate) selected_channel: usize,
    pub(crate) selected_measure: usize,

    pub(crate) channel_symbol: [String;4],
}

impl Default for RustNES {
    fn default() -> Self {
        Self {
            _picked_path: None,
            _test_bool: false,
            synth: synth::Synth::new(8),
            unselected_color: Color32::from_rgb(100, 100, 100),
            selected_color: Color32::from_rgb(80, 200, 80),

            selected_channel: 0,
            selected_measure: 0,

            channel_symbol: ["∏".to_owned(),"∏".to_owned(),"⏶".to_owned(),"♒".to_owned()],
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
                ui.menu_button("File", |ui|{self.file_menu(ui)});
                ui.menu_button("Edit", Self::edit_menu);
    
                #[cfg(debug_assertions)]
                ui.menu_button("Debug", Self::debug_menu);
            });
        });
    }

    /// The File context menu
    /// Contains New, Open File, Save, Export (MIDI/NSF), Import (MIDI/NSF)
    pub(crate) fn file_menu(&mut self, ui: &mut egui::Ui) {
        if ui.button("New").clicked() {

            //Deletes the old track, and creates a new one
            self.synth.new_track();
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
                if ui.button("Stop").clicked(){
                    self.synth.stop();
                }
                ui.add(egui::Slider::new(&mut self.synth.volume, 0.0..=100.0).show_value(false));
            });
        });
    }

    /// Creates the note stepper in the middle of the UI
    /// Uses the columns ui to create a grid of buttons that each correspond
    /// to a specific note. Currently only 8 buttons per screen.
    pub(crate) fn note_stepper(&mut self, ui: &mut egui::Ui){

        let curr_channel = &mut self.synth.track.channels[self.selected_channel];

        ui.columns(self.synth.beats_per_measure as usize, |columns|{ 

            //TODO move this to using slices
            for i in 0 .. self.synth.beats_per_measure as usize{

                if curr_channel.len() < i{
                    println!("RustNES:note_stepper: Index out of bounds returning");
                    return;
                }

                let curr: &mut synth::WaveColumn = &mut curr_channel[i + (self.selected_measure * self.synth.beats_per_measure as usize)];

                // Renders the buttons for the stepper.
                // Starts from 12, as egui starts from the top down
                for j in (0..12).rev(){
                    if columns[i].add(
                    egui::Button::new("").
                    fill(
                        if curr.is_selected(j) {self.selected_color} 
                        else { self.unselected_color}
                    )).is_pointer_button_down_on(){
                        curr.select(j);
                        println!("{}",curr.get_index());
                    };
                }
            }
        });
    }

    /// The channel selector to be able to select which of the 4 main channels are being used.
    /// The default channel is currently PulseOne
    pub(crate) fn channel_selector(&mut self, ui: &mut egui::Ui){
        ui.separator();

        ui.columns(self.synth.max_measures as usize, |columns|{ 

            for i in 0..self.synth.max_measures as usize{

                columns[i].vertical_centered_justified(|centered|{
                    self.add_channel_column(centered, i)
                });
            }

        });
    }

    /// Adds the channel controls and allows selecting specific channels and measure
    fn add_channel_column(&mut self, ui: &mut egui::Ui, measure_index: usize){
        // If the index is less than current measure, then add the channel column
        if measure_index < self.synth.get_measure_count(){
            if ui.button("–").clicked() {
                let remove_amount = self.synth.get_measure_count() - measure_index;

                self.synth.remove_measure(remove_amount);
            }
            
            for j in 0 .. self.synth.track.get_channel_count(){
                // TODO turn this into a function
                if ui.add(
                egui::Button::new(format!("{}",self.channel_symbol[j]))
                .fill(
                    if measure_index == self.selected_measure && j == self.selected_channel {self.selected_color} 
                    else { self.unselected_color}
                )).clicked(){
                    self.selected_channel = j;
                    self.selected_measure = measure_index;
                    println!("{},{}", self.selected_measure, self.selected_channel);
                };
            }
        }

        // If the index is greater than the measure, then show the "+" for the channel
        else{
            if ui.button("+").clicked() {
                let add_amount = (measure_index + 1) - self.synth.get_measure_count();

                 self.synth.add_measure(add_amount);
            }
        }
    }
}