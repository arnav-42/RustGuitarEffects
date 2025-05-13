mod audio_utils;
mod effect;
mod overdrive;
mod distortion;
mod chorus;
mod delay;
mod reverb;

use anyhow::Result;
use clap::Parser;
use effect::{Effect, EffectType};
use std::time::Instant;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    effect: String,
    input: String,
    output: String,
}

fn make_effect(name: &str, sr: u32) -> Result<EffectType> {
    let eff = match name.to_lowercase().as_str() {
        "overdrive" => EffectType::Overdrive(overdrive::Overdrive::new(2.5, 0.8)),
        "distortion" => EffectType::Distortion(distortion::Distortion::new(0.3, 4000.0, sr)),
        "chorus" => EffectType::Chorus(chorus::Chorus::new(sr as usize, 10.0, 1.2)),
        "delay" => EffectType::Delay(delay::Delay::new(sr as usize, 450.0, 0.45)),
        "reverb" => EffectType::Reverb(reverb::Reverb::new(sr as usize)),
        _ => anyhow::bail!("Unknown effect: {}", name),
    };
    Ok(eff)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let audio = audio_utils::AudioFile::read(&args.input)?;
    let mut effect = make_effect(&args.effect, audio.spec.sample_rate)?;

    let start = Instant::now();
    let processed: Vec<f32> = audio.samples.iter()
        .map(|&s| effect.process(s))
        .collect();
    eprintln!("Processed {} samples in {:?}", processed.len(), start.elapsed());

    audio.write(&args.output, &processed)?;
    Ok(())
}