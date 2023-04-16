#[derive(Clone, Debug)]
pub struct LowPassFilter{
    prev_out: f32,
}

impl Default for LowPassFilter{
    fn default() -> Self{
        LowPassFilter{
            prev_out: 0.0,
        }
    }
}

impl LowPassFilter{
    pub fn filter(&mut self, input: f32) -> f32{
        let output = (input - self.prev_out) * 0.815686;
        self.prev_out = output;
        return output;
    }
}

#[derive(Clone, Debug)]
pub struct HighPassFilter{
    prev_out: f32,
    prev_in: f32,
}

impl Default for HighPassFilter{
    fn default() -> Self{
        HighPassFilter{
            prev_out: 0.0,
            prev_in: 0.0,
        }
    }
}

impl HighPassFilter{
    pub fn filter(&mut self, input: f32, k: f32) -> f32{
        let output = ((self.prev_out) * k) + input - self.prev_in;
        self.prev_in = input;
        self.prev_out = output;
        return output;
    }
}