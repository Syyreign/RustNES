use egui::{Color32, Widget, Sense};

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
    pub(crate) highlight_color: Color32,

    pub(crate) row_highlight_interval: u32,

    pub(crate) selected_channel: usize,
    pub(crate) selected_page: usize,

    pub(crate) channel_symbol: [String;4],

    pressed: bool,
}

impl Default for RustNES {
    fn default() -> Self {
        Self {
            _picked_path: None,
            _test_bool: false,
            synth: synth::Synth::new(8, 4, 8),
            unselected_color: Color32::from_rgb(100, 100, 100),
            selected_color: Color32::from_rgb(80, 200, 80),
            highlight_color: Color32::from_rgb(50, 80, 50),

            row_highlight_interval: 8,

            selected_channel: 0,
            selected_page: 0,

            channel_symbol: ["∏".to_owned(),"∏".to_owned(),"⏶".to_owned(),"♒".to_owned()],

            pressed: false,
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

            self.selected_page = 0;

            //Deletes the old track, and creates a new one
            self.synth.new_track();
            ui.close_menu();
        }

        if ui.button("Open").clicked() {
            // rfd is used to access files
            if let Some(path) = rfd::FileDialog::new()
                // Only look for rsf files
                .add_filter("", &["rsf"])
                .pick_file() {

                // TODO handle result
                self.synth.load_track(path).ok();
            }
            ui.close_menu();
        }

        if ui.button("Save As").clicked() {

            if let Some(path) = rfd::FileDialog::new()
            // Only look for rsf files
            .add_filter("", &["rsf"])
            .save_file() {

                // TODO handle result
                self.synth.save_track(path).ok();
            }
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
        ui.columns(self.synth.measures_per_page as usize, |columns|{

            for current_column_index in 0 .. self.synth.measures_per_page{

                if current_column_index as usize >= columns.len(){
                    println!("RustNES::note_stepper: current_page_index {} out of bounds", current_column_index);
                    continue;
                }

                self.column_stepper(&mut columns[current_column_index as usize], current_column_index);
            }
        });
    }

    /// The current column being rendered
    /// Spacing is set to 0,0 so that the notes sit right next to each other
    fn column_stepper(&mut self, ui: &mut egui::Ui, current_column_index: u32){

        let first_measure_index = self.selected_page as u32 * self.synth.get_notes_per_page();
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        for row_index in (0 .. self.synth.rows_per_column).rev(){
            ui.columns(4, |horizontal|{
                for j in 0 .. self.synth.notes_per_measure{

                    let column_index = first_measure_index + (current_column_index * self.synth.notes_per_measure) + j;
                    //horizontal[j as usize].small_button("");
                    self.column_button(&mut horizontal[j as usize], column_index, row_index)
                }
            });
        }
    }

    /// The current button being rendered to the column
    fn column_button(&mut self, ui: &mut egui::Ui, column_index: u32, row_index: u32){
        let option_curr = self.synth.get_channel_column(column_index as usize, self.selected_channel);

        match option_curr {
            None => {
                println!("RustNES::column_button: option is none");
                return;
            },
            Some(curr) =>{
    
                let button = egui::Button::new("")
                    // TODO move this into a function
                    .fill(
                        if curr.is_selected(row_index) {self.selected_color}
                        else if row_index % self.row_highlight_interval == 0 {self.highlight_color}
                        else { self.unselected_color}
                    )
                    .small()
                    .sense(Sense{ click: true, drag: true, focusable: false });
                let response = button.ui(ui);
            
                // TODO Fix this mess
                // If the button is pressed, then select the current note
                if response.hovered() && !curr.is_selected(row_index) && response.ctx.input().pointer.any_down() && !self.pressed{
                    curr.select(row_index);
                    println!("{} {} selected", column_index, row_index);
                }
            
                // On a drag, select multiple notes
                else if response.hovered() && response.ctx.input().pointer.any_pressed(){
                    curr.select(row_index);
                    println!("{} {} selected", column_index, row_index);
                    self.pressed = true;
                }
            
                // To stop the notes from turning on and off, use a flag
                // Gross
                if response.ctx.input().pointer.any_released() {
                    self.pressed = false;
                }   
            },
        }
            
    }

    /// The channel selector to be able to select which of the 4 main channels are being used.
    /// The default channel is currently PulseOne
    pub(crate) fn channel_selector(&mut self, ui: &mut egui::Ui){
        ui.separator();

        egui::ScrollArea::horizontal().show(ui, |ui| {
            egui::Grid::new("channel_grid").show(ui, |ui| {
                for i in 0..self.synth.max_pages as usize{
                    ui.vertical_centered_justified(|vertical|{
                        self.subtract_channel_columns(vertical, i);
                    });
                }
                self.add_channel_columns(ui);
            });
        });
    }

    /// Adds the channel controls and allows selecting specific channels and measure
    fn subtract_channel_columns(&mut self, ui: &mut egui::Ui, measure_index: usize){
        // If the index is less than current measure, then add the channel column
            if ui.button("–").clicked() {
                let remove_amount = self.synth.max_pages as usize - measure_index;
                if self.synth.remove_page(remove_amount) {
                    self.selected_page = self.synth.max_pages as usize - 1;
                }
            }
            
            for j in 0 .. self.synth.track.get_channel_count(){
                // TODO turn this into a function
                if ui.add(
                egui::Button::new(format!("{}",self.channel_symbol[j]))
                .fill(
                    if measure_index == self.selected_page && j == self.selected_channel {self.selected_color} 
                    else { self.unselected_color}
                ))
                .clicked(){
                    self.selected_channel = j;
                    self.selected_page = measure_index;
                    println!("{},{}", self.selected_page, self.selected_channel);
                };
            }
    }

    fn add_channel_columns(&mut self, ui: &mut egui::Ui,){
        ui.group(|ui|{
            if ui.button("+").clicked() {
                self.synth.add_page(1);
            }
        });
    }
}