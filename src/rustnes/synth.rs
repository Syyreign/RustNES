use std::{time::Duration};
use rodio::{Sink, OutputStream, Source};
use std::thread;

use crate::rustnes::waves;

/// The struct that defines all of the synth values
/// TODO make more of these variables private, and add getters and setters
pub(crate) struct Synth{
    pub(crate) track: Track,
    pub(crate) tempo: f32,
    pub(crate) volume: f32,

    pub(crate) beats_per_measure: u32,

    pub(crate) measures: u32,
    pub(crate) max_measures: u32,
}

impl Default for Synth{
    fn default() -> Self {

        Self {  
            track: Track::default(),
            tempo: 180.0,
            volume: 100.0,

            beats_per_measure: 8,

            measures: 1,
            max_measures: 16,
        }
    }
}

impl Synth{

    pub(crate) fn play(&mut self){

        let source = waves::Oscillators::new(&self.track, self.tempo)
            .take_duration(Duration::from_secs_f32(10.0))
            .amplify(self.volume / 100.0);

        thread::spawn(||{

            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
        
            sink.append(source);
            sink.sleep_until_end();
        });
    }

    pub(crate) fn add_measure(&mut self) -> bool{
        if !self.can_add_measure(){
            println!("Synth::add_measure: Can't add any more notes to the track");
            return false;
        }

        self.track.add_columns(8);
        true
    }

    pub(crate) fn remove_measure(&mut self) -> bool{
        if !self.can_remove_measure() { 
            println!("Synth::remove_measure: Can't remove any more notes from the track");
            return false;
        }

        self.track.remove_columns(8);
        true
    }

    fn can_add_measure(&self) -> bool {
        (self.track.get_length() as u32 + self.beats_per_measure) <= (self.beats_per_measure * self.max_measures)
    }

    fn can_remove_measure(&self) -> bool {
        (self.track.get_length() as u32) >= self.beats_per_measure
    }
}

/// The current track of the synth
/// contains the 4 main channels
pub struct Track{
    pub(crate) channels: [Vec<WaveColumn>; 4],
}


// Just initializing all 4 channels in an array  
// Index 0: Pulse one
// Index 1: Pulse two
// Index 2: Triangel
// Index 3: Noise
impl Default for Track{
    fn default() -> Self {
        Self {  
            channels: [vec![WaveColumn::default(); 8], 
            vec![WaveColumn::default(); 8], 
            vec![WaveColumn::default(); 8], 
            vec![WaveColumn::default(); 8]],
        }
    }
}

impl Track{

    /// Gets the amount of notes in the track
    /// This length is based on the len() of the vec in index 0
    /// As all of the channels are the same length, this should be fine
    pub fn get_length(&self) -> usize{
        self.channels[0].len()
    }

    /// Add columns based "amount"
    pub fn add_columns(&mut self, amount: usize){
        for channel in &mut self.channels{
            let new_columns = vec![WaveColumn::default(); amount];
            channel.extend_from_slice(&new_columns);
        }
    }

    /// Remove columns based on "amount"
    /// If any of the channels have a length < amount, return
    pub fn remove_columns(&mut self, amount: usize) {
        if self.channels.iter().any(|channel| channel.len() <= amount){
            println!("Track::remove_columns: Can't remove any more columns");
            return
        }

        for channel in &mut self.channels{
            channel.truncate(channel.len() - amount);
        }

    }
}


/// The column of each note, its bools are represented as a binary number
/// to keep from needing a large array of bools.
#[derive(Clone, Debug)]
pub struct WaveColumn {
    column: u16,
}

impl Default for WaveColumn {
    fn default() -> Self {
        Self { 
            column: 0,
        }
    }
}


impl WaveColumn{

    /// Select the note pressed. ANDs the number to clear everything but the selected,
    /// then XORs the number to create a toggle
    pub(crate) fn select(&mut self, index: u16){
        let b = 1 << index;

        self.column &= b;
        self.column ^= b;
    }

    /// Checks if the current note is selected
    pub(crate) fn is_selected(&self, index: u16) -> bool{
        self.column == 1 << index
    }

    /// Returns the index of the current columns note
    /// If non selected, return -1
    pub(crate) fn get_index(&self) -> i32{
        let index = self.column.trailing_zeros();

        if index == 16 {
            return -1;
        }
        index as i32 + 69
    }
}


///
/// Temporarily moved here
/// Simple rodio sink to play an NES triangle wave
/// 
pub(crate) fn play_nes_triangle_wave(freq: f32){
    // let source = waves::NESTriangleWave::new(freq).take_duration(Duration::from_secs_f32(1.0));

    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // sink.append(source);
    // sink.sleep_until_end();
}

///
/// Temporarily moved here
/// Simple rodio sink to play an NES pulse wave
/// 
pub(crate) fn play_nes_pulse_wave(freq: f32){
    // let source = waves::NESPulseWave::new(freq, 0.5).take_duration(Duration::from_secs_f32(1.0));

    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // sink.append(source);
    // sink.sleep_until_end();
}

///
/// Temporarily moved here
/// Simple rodio sink to play a sine wave
/// 
pub(crate) fn play_nes_noise(){
    // let source = waves::NESNoise::new().take_duration(Duration::from_secs_f32(1.0));

    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // sink.append(source);
    // sink.sleep_until_end();
}