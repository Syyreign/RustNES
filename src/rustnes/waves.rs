use std::time::Duration;
use rand::Rng;

use crate::Source;

use super::synth::SequenceColumn;

#[derive(Clone, Debug)]
pub struct Oscillators {
    pulse_one: NESPulseWave,
    pulse_two: NESPulseWave,
    num_sample: usize,
    tempo: u32,
}

impl Oscillators {
    /// The frequency of the sine.
    #[inline]
    pub fn new(sequence_column: &Vec<SequenceColumn>) -> Oscillators {
        Oscillators {
            pulse_one: NESPulseWave::new(sequence_column.to_vec()),
            pulse_two: NESPulseWave::new(sequence_column.to_vec()),
            num_sample: 0,
            tempo: 80,
        }
    }

    pub fn start_pulse_one(&mut self, freq: f32, duty: f32, length: u32){
        self.pulse_one.update(freq, duty, length)
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

        // This is awful, don't unwrap something like this.
        let t1 = self.pulse_one.next().unwrap();
        let t2 = self.pulse_two.next().unwrap();
        let sample = (t1 + t2) / 2.0;

        Some(sample)
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
    freq: f32,
    num_sample: usize,
    steps: [f32;16],
}

impl NESTriangleWave {
    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> NESTriangleWave {
        NESTriangleWave {
            freq: freq,
            num_sample: 0,

            // The steps of the triangle wave
            steps: [-1.0, -0.86666, -0.73333, -0.6, -0.46666, -0.33333, -0.2, -0.06666, 0.06666, 0.2, 0.33333, 0.46666, 0.6, 0.73333, 0.86666, 1.0],
        }
    }
}
 
impl Iterator for NESTriangleWave {
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
        let mut x = (((self.num_sample as f32 * 30.0) * freq_ratio) % 30.0) - 15.0;

        // Round the float values to indexes of an array corresponsing to the stepped triangle wave
        x = x.round();

        Some(self.steps[x as usize])
    }
}

/// 
/// Creates pulse wave
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESPulseWave {
    sequence_columns: Vec<SequenceColumn>,
    freq: f32,
    duty: f32,
    num_sample: usize,
}

impl NESPulseWave {
    /// The frequency of the sine.
    /// Duty is time of each pulse. 0.5 is a square wave
    #[inline]
    pub fn new(sequence_column: Vec<SequenceColumn>) -> NESPulseWave {
        NESPulseWave {
            sequence_columns: sequence_column,
            freq: 0.0,
            duty: 0.5,
            num_sample: 0,
        }
    }

    #[inline]
    pub fn update(&mut self, freq: f32, duty: f32, length: u32){
        self.freq = freq;
        self.duty = duty;
    }
}
 
impl Iterator for NESPulseWave {
    type Item = f32;

    ///
    /// This function imitates the NES Pulse wave
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let index = self.num_sample as f32 / (48000.0 / 2.0);
        if index as usize >= self.sequence_columns.len() {
            return Some(0.0);
        }

        let col = &self.sequence_columns[index as usize];

        if col.get_index() == -1 {
            return Some(0.0);
        }

        
        let freq = get_frequency(col.get_index());

        // Messy, should be cleaned up a bit
        // Divide the 48000 into segments of self.freq, then checks if the current sample
        // is less than half of that. If it is then return 0.0 otherwise return 1.0
        if (self.num_sample as f32 % (48000.0 / freq)) < (48000.0 / freq) * self.duty {
            return Some(0.0)
        }

        Some(1.0)
    }
}

/// 
/// Creates noise using 16 steps. This is a limitation of the NES and 
/// what gives it a unique sound
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESNoise {
    steps: [f32;16],
}

impl NESNoise {
    #[inline]
    pub fn new() -> NESNoise {
        NESNoise {
            // The steps that the noise can produce
            steps: [-1.0, -0.86666, -0.73333, -0.6, -0.46666, -0.33333, -0.2, -0.06666, 0.06666, 0.2, 0.33333, 0.46666, 0.6, 0.73333, 0.86666, 1.0],
        }
    }
}
 
impl Iterator for NESNoise {
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

fn get_frequency(note: i32) -> f32{
    440.0 * f32::powf(2.0, (note as f32 - 69.0) / 12.0)
}