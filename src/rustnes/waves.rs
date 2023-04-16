use std::{time::Duration};
use rand::Rng;

use crate::Source;

// TODO make this not use a super
use super::synth::{WaveColumn, Track};
use crate::rustnes::filters;

#[derive(Clone, Debug)]
pub struct Oscillators {
    pulse_one: NESPulseWave,
    pulse_two: NESPulseWave,
    triangle: NESTriangleWave,
    noise: NESNoise,
    num_sample: usize,
    beats_per_second: f32,
    length: usize,

    low_pass_filter: filters::LowPassFilter,
    high_pass_filter1: filters::HighPassFilter,
    high_pass_filter2: filters::HighPassFilter,
}

impl Oscillators {
    /// The frequency of the sine.
    #[inline]
    pub fn new(track: &Track, tempo: f32) -> Oscillators {
        Oscillators {
            pulse_one: NESPulseWave::new(track.channels[0].to_vec()),
            pulse_two: NESPulseWave::new(track.channels[1].to_vec()),
            triangle: NESTriangleWave::new(track.channels[2].to_vec()),
            noise: NESNoise::new(track.channels[3].to_vec()),
            num_sample: 0,
            beats_per_second: tempo / 60.0,
            length: track.get_length(),

            low_pass_filter: filters::LowPassFilter::default(),
            high_pass_filter1: filters::HighPassFilter::default(),
            high_pass_filter2: filters::HighPassFilter::default(),
        }
    }
}

impl Iterator for Oscillators {
    type Item = f32;

    ///
    /// This function imitates the NES triangle wave
    /// This could probably be generalized more
    /// TODO double check this is correct!
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let mut index = self.num_sample as f32 / (48000.0 / self.beats_per_second);
        if index > self.length as f32 -1.0{
            return Some(0.0);
        }

        index = index.min(self.length as f32 - 1.0);

        // This is awful, don't do something like this.
        let p1 = self.pulse_one.next(self.num_sample, index as usize);
        let p2 = self.pulse_two.next(self.num_sample, index as usize);
        let t = self.triangle.next(self.num_sample, index as usize);
        let n = self.noise.next(index as usize);

        // As the NES mixer isn't linear this equation emulated it
        // TODO add dmc (the 0.0 / 22638.0)
        let pulse_out = 95.88 / ((8128.0 / (p1 + p2)) + 100.0);
        let tnd_out = 159.79 / ((1.0 / ((t / 8227.0) + (n / 12241.0) + (0.0 / 22638.0))) + 100.0);

        let mut output = self.high_pass_filter1.filter(pulse_out + tnd_out, 0.996039);
        output = self.high_pass_filter2.filter(output, 0.999835);
        output = self.low_pass_filter.filter(output);

        Some(output)
    }
}

impl Source for Oscillators {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// 
/// Creates a triangle wave using 16 steps. This is a limitation of the NES and 
/// what gives it a unique sound
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESTriangleWave {
    sequence_columns: Vec<WaveColumn>,
    steps: [f32;16],
}

impl NESTriangleWave {
    /// The frequency of the sine.
    #[inline]
    pub fn new(sequence_columns: Vec<WaveColumn>) -> NESTriangleWave {
        NESTriangleWave {
            sequence_columns: sequence_columns,

            // The steps of the triangle wave
            steps: [-1.0, -0.86666, -0.73333, -0.6, -0.46666, -0.33333, -0.2, -0.06666, 0.06666, 0.2, 0.33333, 0.46666, 0.6, 0.73333, 0.86666, 1.0],
        }
    }
}
 
impl NESTriangleWave {

    ///
    /// This function imitates the NES triangle wave
    /// This could probably be generalized more
    /// TODO double check this is correct!
    /// 
    #[inline]
    fn next(&mut self, num_sample: usize, index: usize) -> f32 {

        let col = &self.sequence_columns[index as usize];

        if col.get_index() == -1 {
            return 0.0;
        }

        let freq = get_frequency(col.get_index());

        let freq_ratio = freq / 48000.0;

        // Create a triangle wave, from 0-15 as float values
        let mut x = (((num_sample as f32 * 30.0) * freq_ratio) % 30.0) - 15.0;

        // Round the float values to indexes of an array corresponsing to the stepped triangle wave
        x = x.round();

        self.steps[x as usize]
    }
}

/// 
/// Creates pulse wave
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESPulseWave {
    sequence_columns: Vec<WaveColumn>,
    duty: f32,
}

impl NESPulseWave {
    /// The frequency of the sine.
    /// Duty is time of each pulse. 0.5 is a square wave
    #[inline]
    pub fn new(sequence_columns: Vec<WaveColumn>) -> NESPulseWave {
        NESPulseWave {
            sequence_columns: sequence_columns,
            duty: 0.5,
        }
    }

    #[inline]
    fn next(&mut self, num_sample: usize, index: usize) -> f32 {

        let col = &self.sequence_columns[index as usize];

        if col.get_index() == -1 {
            return 0.0;
        }

        let freq = get_frequency(col.get_index());

        // Messy, should be cleaned up a bit
        // Divide the 48000 into segments of self.freq, then checks if the current sample
        // is less than half of that. If it is then return 0.0 otherwise return 1.0
        if (num_sample as f32 % (48000.0 / freq)) < (48000.0 / freq) * self.duty {
            return 0.0;
        }

        1.0
    }
}

/// 
/// Creates noise using 16 steps. This is a limitation of the NES and 
/// what gives it a unique sound
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESNoise {
    sequence_columns: Vec<WaveColumn>,
    steps: [f32;16],
}

impl NESNoise {
    #[inline]
    pub fn new(sequence_columns: Vec<WaveColumn>) -> NESNoise {
        NESNoise {
            sequence_columns: sequence_columns,
            // The steps that the noise can produce
            steps: [-1.0, -0.86666, -0.73333, -0.6, -0.46666, -0.33333, -0.2, -0.06666, 0.06666, 0.2, 0.33333, 0.46666, 0.6, 0.73333, 0.86666, 1.0],
        }
    }
    
}
 
impl NESNoise {
    ///
    /// This function imitates the NES Noise
    /// 
    #[inline]
    fn next(&mut self, index: usize) -> f32 {
        let col = &self.sequence_columns[index as usize];

        if col.get_index() == -1 {
            return 0.0;
        }

        let x = rand::thread_rng().gen_range(0..self.steps.len());

        self.steps[x]
    }
}

/// Converts the Midi note number into a frequency
fn get_frequency(note: i32) -> f32{
    440.0 * f32::powf(2.0, (note as f32 - 69.0) / 12.0)
}