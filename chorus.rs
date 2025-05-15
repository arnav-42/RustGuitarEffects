use crate::effect::Effect;
use std::f32::consts::PI;

pub struct Chorus {
    sample_rate: usize,
    buffer: Vec<f32>,
    write_pos: usize,
    buffer_size: usize,
    lfo_phase: f32,
    lfo_freq: f32,
    depth_samples: f32,
    base_delay_samples: f32,
}

impl Chorus {
    pub fn new(sample_rate: usize, lfo_freq: f32, depth_ms: f32) -> Self {
        let base_delay_ms = 10.0;
        let base_delay_samples = base_delay_ms * (sample_rate as f32) / 1000.0;
        let depth_samples = depth_ms * (sample_rate as f32) / 1000.0;
        
        let buffer_size = (base_delay_samples + depth_samples).ceil() as usize + 2;
        
        Chorus {
            sample_rate,
            buffer: vec![0.0; buffer_size],
            write_pos: 0,
            buffer_size,
            lfo_phase: 0.0,
            lfo_freq,
            depth_samples,
            base_delay_samples,
        }
    }
    
    fn get_sample_at_delay(&self, delay_samples: f32) -> f32 {
        let float_read_pos = self.write_pos as f32 - delay_samples;
        
        let float_read_pos = if float_read_pos < 0.0 {
            self.buffer_size as f32 + float_read_pos
        } else {
            float_read_pos
        };
        
        let read_pos_int = float_read_pos.floor() as usize % self.buffer_size;
        let read_pos_next = (read_pos_int + 1) % self.buffer_size;
        
        let frac = float_read_pos - float_read_pos.floor();
        
        (1.0 - frac) * self.buffer[read_pos_int] + frac * self.buffer[read_pos_next]
    }
}

impl Effect for Chorus {
    fn process(&mut self, sample: f32) -> f32 {
        self.buffer[self.write_pos] = sample;
        
        let lfo_value = (self.lfo_phase * 2.0 * PI).sin();
        let delay_samples = self.base_delay_samples + self.depth_samples * lfo_value;
        
        let delayed_sample = self.get_sample_at_delay(delay_samples);
        
        self.write_pos = (self.write_pos + 1) % self.buffer_size;
        
        self.lfo_phase += self.lfo_freq / self.sample_rate as f32;
        if self.lfo_phase > 1.0 {
            self.lfo_phase -= 1.0;
        }
        
        0.5 * sample + 0.5 * delayed_sample
    }
    
    fn reset(&mut self) {
        self.buffer = vec![0.0; self.buffer_size];
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}
