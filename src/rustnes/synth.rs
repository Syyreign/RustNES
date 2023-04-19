use std::path::PathBuf;
use std::{time::Duration};
use rodio::{Sink, OutputStream, Source};
use std::thread;

use bincode;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;

// Atomics are used to stop the rodio play thread
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::rustnes::waves;

/// The struct that defines all of the synth values
/// TODO make more of these variables private, and add getters and setters
pub struct Synth{
    //pub inital_size: usize,
    
    pub track: Track,
    pub tempo: f32,
    pub volume: f32,

    pub notes_per_measure: u32,
    pub measures_per_page: u32,
    pub max_pages: u32,

    pub rows_per_column: u32,

    stop_thread: Arc<AtomicBool>,
}

impl Default for Synth{
    fn default() -> Self {

        // The inital size of the window/measure
        // TODO make this more editable/ getters setters
        Synth::new(8, 4, 4)
    }
}

impl Synth{
    pub fn new(max_pages: u32, notes_per_measure: u32, measures_per_page: u32) -> Self{
        Self { 
            //inital_size: initial_size as usize,
            track: Track::new(max_pages as usize * notes_per_measure as usize * 4),

            // The tempo is set to 960, as a note is technically only 1/16
            tempo: 960.0,
            volume: 100.0,

            notes_per_measure: notes_per_measure,
            measures_per_page: measures_per_page as u32,
            max_pages: 4,

            rows_per_column: 24,

            stop_thread: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn play(&mut self){

        // Stop any threads that are playing
        self.stop();

        // The length that the sound should play for
        let length = self.track.get_length() as f32 * self.get_beats_per_second();

        let source = waves::Oscillators::new(&self.track, self.tempo)
            .take_duration(Duration::from_secs_f32(length))
            .amplify(self.volume / 100.0);

        // Make sure that the current thread can play
        self.stop_thread.store(false, Ordering::Relaxed);

        let stop_thread = self.stop_thread.clone();

        // Spawn a new thread with an atomic to allow it to be stopped
        // A thread is needed otherwise the main thread will need to stopped
        // for the sink to play
        thread::spawn(move ||{

            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            sink.append(source);
        
            // Loop until the sink is empty, or the thread is told to stop
            loop{
                if sink.empty() || stop_thread.load(Ordering::Relaxed){
                    break;
                }
            }
            stop_thread.store(false, Ordering::Relaxed);
        });
    }

    pub fn stop(&mut self){
        self.stop_thread.store(true, Ordering::Relaxed);
    }

    pub fn new_track(&mut self){
        self.track = Track::new(self.get_notes_per_page() as usize);
    }

    pub fn add_page(&mut self, amount: usize) -> bool{
        self.max_pages += amount as u32;
        self.track.add_columns(amount * self.get_notes_per_page() as usize);
        true
    }

    pub fn remove_page(&mut self, amount: usize) -> bool{
        if !self.can_remove_measure(amount) { 
            println!("Synth::remove_measure: Can't remove any more notes from the track");
            return false;
        }

        self.max_pages -= amount as u32;
        self.track.remove_columns(amount * self.get_notes_per_page() as usize);
        true
    }

    pub fn can_remove_measure(&self, amount: usize) -> bool {
        (self.track.get_length() as u32) > amount as u32 * self.get_notes_per_page()
    }

    pub fn get_beats_per_second(&self) -> f32 {
        self.tempo / 60.0
    }

    pub fn get_channel_column(&mut self, column_index: usize, selected_channel: usize) -> Option<&mut WaveColumn>{
        if selected_channel >= self.track.channels.len() {
            println!("Synth::get_channel_column: selected_channel {} out of bounds", selected_channel);
            return None;
        }
        let current_channel = &mut self.track.channels[selected_channel];

        if column_index >= current_channel.len(){
            println!("Synth::get_channel_column: column_index {} out of bounds", column_index);
            return None;
        }

        Some(&mut current_channel[column_index])
    }

    pub fn save_track(&self, path: PathBuf) -> std::io::Result<()> {
        let encoded_track: Vec<u8> = bincode::serialize(&self.track).unwrap();
        let mut file = File::create(path)?;
        file.write_all(&encoded_track)
    }

    pub fn load_track(&mut self, path: PathBuf) -> std::io::Result<()> {
        let mut file = File::open(path)?;

        let mut encoded_track = Vec::<u8>::new();
        file.read_to_end(&mut encoded_track)?;

        let decoded_track: Track = bincode::deserialize(&encoded_track[..]).unwrap();

        self.track = decoded_track;
        
        // TODO make this not just send an Ok
        Ok(())
    }

    pub fn get_notes_per_page(&self) -> u32{
        self.notes_per_measure * self.measures_per_page
    }
}

/// The current track of the synth
/// contains the 4 main channels
#[derive(Serialize, Deserialize, Debug)]
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
        Track::new(8)
    }
}

impl Track{

    pub fn new(initial_size: usize) -> Self{
        Self {  
            channels: [vec![WaveColumn::default(); initial_size], 
            vec![WaveColumn::default(); initial_size], 
            vec![WaveColumn::default(); initial_size], 
            vec![WaveColumn::default(); initial_size]],
        }
    }

    /// Gets the amount of notes in the track
    /// This length is based on the len() of the vec in index 0
    /// As all of the channels are the same length, this should be fine
    pub fn get_length(&self) -> usize{
        self.channels[0].len()
    }

    /// Gets the current number of the channels
    /// Default will be 2 pulse, 1 triangle, and 1 noise. 4 in total
    pub fn get_channel_count(&self) -> usize{
        self.channels.len()
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

        self.channels.iter_mut().for_each(|channel|{
            channel.truncate(channel.len() - amount);
        });

    }
}


/// The column of each note, its bools are represented as a binary number
/// to keep from needing a large array of bools.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WaveColumn {
    column: u32,
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
    pub(crate) fn select(&mut self, index: u32){
        let b = 1 << index;

        self.column &= b;
        self.column ^= b;
    }

    /// Checks if the current note is selected
    pub(crate) fn is_selected(&self, index: u32) -> bool{
        self.column == 1 << index
    }

    /// Returns the index of the current columns note
    /// If non selected, return -1
    pub(crate) fn get_index(&self) -> i32{
        let index = self.column.trailing_zeros();

        if index == 32 {
            return -1;
        }

        // + 36 so that the base note frequency is 440 rather than 24
        index as i32 + 36
    }
}


///
/// Temporarily moved here
/// Simple rodio sink to play an NES triangle wave
/// 
pub(crate) fn play_nes_triangle_wave(_freq: f32){
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
pub(crate) fn play_nes_pulse_wave(_freq: f32){
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