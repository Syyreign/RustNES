use std::{time::Duration};
use rand::Rng;

use crate::Source;

// TODO make this not use a super
use super::synth::{WaveColumn, Track};
use crate::rustnes::filters;

// The period table of the NES
const PERIODS: &'static [u32] = &[
    2033,1919,1811,1709,1613,1523,1437,1356,1280,1208,
    1140,1076,1016,959, 905, 854, 806, 761, 718, 678,
    640, 604, 570, 538, 507, 479, 452, 427, 403, 380, 
    359, 338, 319, 301, 284, 268, 253, 239, 225, 213,
    201, 189, 179, 169, 159, 150, 142, 134, 126, 119,
    112, 106, 100, 94,  89,  84,  79,  75,  70,  66,
    63,  59,  56,  52,  49,  47,  44,  41,  39,  37,
    35,  33,  31,  29,  27,  26,  24,  23,  21,  20,
];

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
    /// This function imitates the Oscillators of the NES
    /// The channels are mixed by hand to achieve NES like sound
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let mut index = self.num_sample as f32 / (48000.0 / self.beats_per_second);
        if index > self.length as f32{
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

        // Pass the raw mixed sound into two high pass filters, and one
        // low pass filter
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

        // The frequency of the triangle wave on the NES is 1 octave lower hence the / 2.0
        let freq = get_frequency(col.get_index()) / 2.0;

        let freq_ratio = freq / 48000.0;

        // Create a triangle wave, from 0-15 as float values
        let mut x = ((((num_sample as f32 * 30.0) * freq_ratio) % 30.0) - 15.0).abs();

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
/// returns an int cast to a float, as the NES didnt have a FPU
fn get_frequency(note: i32) -> f32{
    if note < 0 || note >= PERIODS.len() as i32 {
        return 0.0;
    }
    
    // 17789773 is the NES CPU clock rate for NTSC
    (1789773 / (16 * (PERIODS[note as usize] + 1))) as f32
}

/// 
/// Creates a triangle wave using 16 steps. This is a limitation of the NES and 
/// what gives it a unique sound
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESTriangleWaveNote {
    freq: f32,
    num_sample: usize,
    steps: [f32;16],
}

impl NESTriangleWaveNote {
    /// The frequency of the sine.
    #[inline]
    pub fn new(index: i32) -> NESTriangleWaveNote {

        let mut freq = 0.0;
        if index >= 0 {
            freq = get_frequency(index);
        }

        NESTriangleWaveNote {
            freq: freq,
            num_sample: 0,

            // The steps of the triangle wave
            steps: [-1.0, -0.86666, -0.73333, -0.6, -0.46666, -0.33333, -0.2, -0.06666, 0.06666, 0.2, 0.33333, 0.46666, 0.6, 0.73333, 0.86666, 1.0],
        }
    }
}
 
impl Iterator for NESTriangleWaveNote {
    type Item = f32;

    ///
    /// This function imitates the NES triangle wave
    /// This could probably be generalized more
    /// TODO double check this is correct!
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let freq_ratio = self.freq / 48000.0;

        // Create a triangle wave, from 0-15 as float values
        let mut x = ((((self.num_sample as f32 * 30.0) * freq_ratio) % 30.0) - 15.0).abs();

        // Round the float values to indexes of an array corresponsing to the stepped triangle wave
        x = x.round();

        Some(self.steps[x as usize])
    }
}

impl Source for NESTriangleWaveNote {
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
/// Creates pulse wave
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESPulseWaveNote {
    freq: f32,
    duty: f32,
    num_sample: usize,
}

impl NESPulseWaveNote {
    /// The frequency of the sine.
    /// Duty is time of each pulse. 0.5 is a square wave
    #[inline]
    pub fn new(index: i32, duty: f32) -> NESPulseWaveNote {

        let mut freq = 0.0;
        if index >= 0 {
            freq = get_frequency(index);
        }

        NESPulseWaveNote {
            freq: freq,
            duty: duty,
            num_sample: 0,
        }
    }
}
 
impl Iterator for NESPulseWaveNote {
    type Item = f32;

    ///
    /// This function imitates the NES Pulse wave
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        // Messy, should be cleaned up a bit
        // Divide the 48000 into segments of self.freq, then checks if the current sample
        // is less than half of that. If it is then return 0.0 otherwise return 1.0
        if (self.num_sample as f32 % (48000.0 / self.freq)) < (48000.0 / self.freq) * self.duty {
            return Some(0.0)
        }

        Some(1.0)
    }
}

impl Source for NESPulseWaveNote {
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
/// Creates noise using 16 steps. This is a limitation of the NES and 
/// what gives it a unique sound
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESNoiseNote {
    steps: [f32;16],
}

impl NESNoiseNote {
    #[inline]
    pub fn new() -> NESNoiseNote {
        NESNoiseNote {
            // The steps that the noise can produce
            steps: [-1.0, -0.86666, -0.73333, -0.6, -0.46666, -0.33333, -0.2, -0.06666, 0.06666, 0.2, 0.33333, 0.46666, 0.6, 0.73333, 0.86666, 1.0],
        }
    }
}
 
impl Iterator for NESNoiseNote {
    type Item = f32;

    ///
    /// This function imitates the NES Noise
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        let x = rand::thread_rng().gen_range(0..self.steps.len());

        Some(self.steps[x])
    }
}

impl Source for NESNoiseNote {
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