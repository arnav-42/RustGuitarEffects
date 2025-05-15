use crate::effect::Effect;

pub struct Delay {
    buffer: Vec<f32>,
    write_pos: usize,
    buffer_size: usize,
    delay_samples: usize,
    feedback: f32,
}

impl Delay {
    pub fn new(sample_rate: usize, delay_time_ms: f32, feedback: f32) -> Self {
        let delay_samples = ((delay_time_ms * sample_rate as f32) / 1000.0).round() as usize;
        
        let buffer_size = delay_samples + 1;
        
        Delay {
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            buffer_size,
            delay_samples,
            feedback: feedback.clamp(0.0, 1.0),
        }
    }
}

impl Effect for Delay {
    fn process(&mut self, sample: f32) -> f32 {
        let read_pos = (self.buffer_size + self.write_pos - self.delay_samples) % self.buffer_size;
        
        let delayed_sample = self.buffer[read_pos];
        
        let output = sample + self.feedback * delayed_sample;
        
        self.buffer[self.write_pos] = output;
        
        self.write_pos = (self.write_pos + 1) % self.buffer_size;
        
        output
    }
    
    fn reset(&mut self) {
        self.buffer = vec![0.0; self.buffer_size];
        self.write_pos = 0;
    }
}
