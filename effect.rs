pub trait Effect {
    /// process one mono sample and return  processed sample
    fn process(&mut self, sample: f32) -> f32;

    /// re‑init any stateful buffers (called when reading new file)
    fn reset(&mut self) {}
}

// enumeration for dynamic dispatch in main.rs
pub enum EffectType {
    Overdrive(super::overdrive::Overdrive),
    Distortion(super::distortion::Distortion),
    Chorus(super::chorus::Chorus),
    Delay(super::delay::Delay),
    Reverb(super::reverb::Reverb),
}

impl Effect for EffectType {
    fn process(&mut self, s: f32) -> f32 {
        match self {
            EffectType::Overdrive(e) => e.process(s),
            EffectType::Distortion(e) => e.process(s),
            EffectType::Chorus(e) => e.process(s),
            EffectType::Delay(e) => e.process(s),
            EffectType::Reverb(e) => e.process(s),
        }
    }
    fn reset(&mut self) {
        match self {
            EffectType::Overdrive(e) => e.reset(),
            EffectType::Distortion(e) => e.reset(),
            EffectType::Chorus(e) => e.reset(),
            EffectType::Delay(e) => e.reset(),
            EffectType::Reverb(e) => e.reset(),
        }
    }
}