use std::time::Duration;
use rodio::{Sink, OutputStream, Source};

use crate::rustnes::waves;

pub(crate) struct Synth{
    pub(crate) sequence_columns: Vec<SequenceColumn>,
    pub(crate) current_position: u32,
}

impl Default for Synth{
    fn default() -> Self {
        Self {  
            sequence_columns: vec![SequenceColumn::default(); 8],
            current_position: 0,
        }
    }
}

impl Synth{

    /// Returns the number of columns in the synth
    pub(crate) fn get_columns_len(&self) -> usize{
        self.sequence_columns.len()
    }

    pub(crate) fn play(&mut self){
        for i in 0..self.get_columns_len(){
            let raw_note = self.sequence_columns[i].get_index();
            if raw_note != -1 {
                let freq = self.get_frequency(raw_note + 69);

                let source = waves::NESTriangleWave::new(freq).take_duration(Duration::from_secs_f32(0.25));

                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();
            
                sink.append(source);
                sink.sleep_until_end();
            }
        }
    }

    fn get_frequency(&self, note: i32) -> f32{
        440.0 * f32::powf(2.0, (note as f32 - 69.0) / 12.0)
    }
}


/// The column of each note, its bools are represented as a binary number
/// to keep from needing a large array of bools.
#[derive(Clone)]
pub(crate) struct SequenceColumn {
    column: u16,
}

impl Default for SequenceColumn {
    fn default() -> Self {
        Self { 
            column: 0,
        }
    }
}


impl SequenceColumn{

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
        index as i32
    }
}


///
/// Temporarily moved here
/// Simple rodio sink to play an NES triangle wave
/// 
pub(crate) fn play_nes_triangle_wave(freq: f32){
    let source = waves::NESTriangleWave::new(freq).take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

///
/// Temporarily moved here
/// Simple rodio sink to play an NES pulse wave
/// 
pub(crate) fn play_nes_pulse_wave(freq: f32){
    let source = waves::NESPulseWave::new(freq, 0.5).take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}

///
/// Temporarily moved here
/// Simple rodio sink to play a sine wave
/// 
pub(crate) fn play_nes_noise(){
    let source = waves::NESNoise::new().take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}