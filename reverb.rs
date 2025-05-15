use crate::effect::Effect;
use rand::Rng;

// Comb filter implementation
struct CombFilter {
    buffer: Vec<f32>,
    buffer_size: usize,
    index: usize,
    feedback: f32,
}

impl CombFilter {
    fn new(delay_samples: usize, feedback: f32) -> Self {
        CombFilter {
            buffer: vec![0.0; delay_samples],
            buffer_size: delay_samples,
            index: 0,
            feedback,
        }
    }
    
    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.index];
        
        // buf[idx] = x(n) + g * y(n - N)
        self.buffer[self.index] = input + self.feedback * output;
        
        self.index = (self.index + 1) % self.buffer_size;
        
        output
    }
    
    fn reset(&mut self) {
        self.buffer = vec![0.0; self.buffer_size];
        self.index = 0;
    }
}

// all-pass filter implementation
struct AllPassFilter {
    buffer: Vec<f32>,
    buffer_size: usize,
    index: usize,
    feedback: f32,
}

impl AllPassFilter {
    fn new(delay_samples: usize, feedback: f32) -> Self {
        AllPassFilter {
            buffer: vec![0.0; delay_samples],
            buffer_size: delay_samples,
            index: 0,
            feedback,
        }
    }
    
    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.index];
        
        // all-pass filter formula:
        // y(n) = -g * x(n) + buf[idx]
        // buf[idx] = x(n) + g * y(n - N)
        let result = -self.feedback * input + output;
        self.buffer[self.index] = input + self.feedback * output;
        
        self.index = (self.index + 1) % self.buffer_size;
        
        result
    }
    
    fn reset(&mut self) {
        self.buffer = vec![0.0; self.buffer_size];
        self.index = 0;
    }
}

// Schroeder reverb
pub struct Reverb {
    comb_filters: Vec<CombFilter>,
    all_pass_filters: Vec<AllPassFilter>,
    mix: f32,
}

impl Reverb {
    pub fn new(sample_rate: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        // Create 4 comb filters with different delay times
        let comb_filters = vec![
            CombFilter::new((sample_rate as f32 * 0.036) as usize, 0.84),  // ~36ms
            CombFilter::new((sample_rate as f32 * 0.031) as usize, 0.83),  // ~31ms
            CombFilter::new((sample_rate as f32 * 0.041) as usize, 0.82),  // ~41ms
            CombFilter::new((sample_rate as f32 * 0.027) as usize, 0.80),  // ~27ms
        ];
        
        // Create 2 all-pass filters
        let all_pass_filters = vec![
            AllPassFilter::new((sample_rate as f32 * 0.005) as usize, 0.7),  // ~5ms
            AllPassFilter::new((sample_rate as f32 * 0.0017) as usize, 0.7), // ~1.7ms
        ];
        
        Reverb {
            comb_filters,
            all_pass_filters,
            mix: 0.3, // Default mix 30% wet, 70% dry
        }
    }
    
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }
}

impl Effect for Reverb {
    fn process(&mut self, sample: f32) -> f32 {
        // Process through comb filters in parallel and sum
        let mut comb_output = 0.0;
        for comb_filter in &mut self.comb_filters {
            comb_output += comb_filter.process(sample);
        }
        comb_output /= self.comb_filters.len() as f32;
        
        // Process through all-pass filters in series
        let mut wet_output = comb_output;
        for all_pass_filter in &mut self.all_pass_filters {
            wet_output = all_pass_filter.process(wet_output);
        }
        
        // Mix dry and wet signals
        (1.0 - self.mix) * sample + self.mix * wet_output
    }
    
    fn reset(&mut self) {
        for comb_filter in &mut self.comb_filters {
            comb_filter.reset();
        }
        
        for all_pass_filter in &mut self.all_pass_filters {
            all_pass_filter.reset();
        }
    }
}
