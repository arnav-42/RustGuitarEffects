use crate::effect::Effect;

pub struct Distortion {
    threshold: f32,
    previous_sample: f32,
    alpha: f32,
}

impl Distortion {
    pub fn new(threshold: f32, cutoff_freq: f32, sample_rate: u32) -> Self {
        // calculating alpha for one-pole low-pass filter
        // alpha = 2pi * f_c / Fs
        let alpha = 2.0 * std::f32::consts::PI * cutoff_freq / sample_rate as f32;
        
        eprintln!("Creating Distortion effect with threshold={}, cutoff={}Hz, sr={}Hz, alpha={}", 
                 threshold, cutoff_freq, sample_rate, alpha);
        
        Distortion {
            threshold,
            previous_sample: 0.0,
            alpha,
        }
    }
    
    fn clip(&self, sample: f32) -> f32 {
        sample.max(-self.threshold).min(self.threshold)
    }
}

impl Effect for Distortion {
    fn process(&mut self, sample: f32) -> f32 {
        // applying hard clipping
        let clipped = self.clip(sample);
        
        // one-pole low-pass filter
        // y[n] = y[n-1] + α * (y_clip - y[n-1])
        self.previous_sample = self.previous_sample + self.alpha * (clipped - self.previous_sample);
        
        static mut COUNTER: u32 = 0;
        unsafe {
            if COUNTER < 10 {
                eprintln!("Distortion: in={}, clipped={}, out={}", sample, clipped, self.previous_sample);
                COUNTER += 1;
            }
        }
        
        self.previous_sample
    }
    
    fn reset(&mut self) {
        self.previous_sample = 0.0;
    }
}
