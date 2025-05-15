use crate::effect::Effect;

pub struct Overdrive {
    pre_gain: f32,
    level: f32,
}

impl Overdrive {
    pub fn new(pre_gain: f32, level: f32) -> Self {
        eprintln!("Creating Overdrive effect with pre_gain={}, level={}", pre_gain, level);
        Overdrive { pre_gain, level }
    }
}

impl Effect for Overdrive {
    fn process(&mut self, sample: f32) -> f32 {
        // soft-clipping waveshaper: y = tanh(pre_gain * x) * level
        let result = (self.pre_gain * sample).tanh() * self.level;
        
        static mut COUNTER: u32 = 0;
        unsafe {
            if COUNTER < 10 {
                eprintln!("Overdrive: in={}, out={}", sample, result);
                COUNTER += 1;
            }
        }
        
        result
    }
}
