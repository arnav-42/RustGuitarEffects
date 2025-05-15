use anyhow::{Context, Result};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::path::Path;

pub struct AudioFile {
    pub samples: Vec<f32>,
    pub spec: WavSpec,
}

impl AudioFile {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut reader = WavReader::open(path)
            .context("Failed to open WAV file")?;
        
        let spec = reader.spec();
        let mut samples = Vec::new();
        
        // Convert any bit depth to normalized f32 (-1.0 to 1.0)
        match (spec.sample_format, spec.bits_per_sample) {
            (SampleFormat::Float, 32) => {
                // Direct read for 32-bit float
                samples = reader
                    .samples::<f32>()
                    .collect::<Result<Vec<f32>, _>>()
                    .context("Failed to read samples")?;
            },
            (SampleFormat::Int, 16) => {
                // Convert 16-bit int to float
                samples = reader
                    .samples::<i16>()
                    .map(|s| s.map(|s| s as f32 / 32768.0))
                    .collect::<Result<Vec<f32>, _>>()
                    .context("Failed to read samples")?;
            },
            (SampleFormat::Int, 24) => {
                // Convert 24-bit int to float
                samples = reader
                    .samples::<i32>()
                    .map(|s| s.map(|s| s as f32 / 8388608.0))
                    .collect::<Result<Vec<f32>, _>>()
                    .context("Failed to read samples")?;
            },
            (SampleFormat::Int, 32) => {
                // Convert 32-bit int to float
                samples = reader
                    .samples::<i32>()
                    .map(|s| s.map(|s| s as f32 / 2147483648.0))
                    .collect::<Result<Vec<f32>, _>>()
                    .context("Failed to read samples")?;
            },
            _ => {
                anyhow::bail!("Unsupported sample format: {:?} with {} bits per sample", 
                             spec.sample_format, spec.bits_per_sample);
            }
        }
        
        // Handle stereo files: convert to mono by averaging channels
        if spec.channels == 2 {
            let mut mono_samples = Vec::new();
            for chunk in samples.chunks(2) {
                if chunk.len() == 2 {
                    mono_samples.push((chunk[0] + chunk[1]) * 0.5);
                }
            }
            samples = mono_samples;
        }
        
        eprintln!(
            "Loaded WAV: {}Hz, {} channels, {} samples ({:.1}s)",
            spec.sample_rate,
            spec.channels,
            samples.len(),
            samples.len() as f32 / spec.sample_rate as f32
        );
        
        Ok(AudioFile { samples, spec })
    }
    
    pub fn write<P: AsRef<Path>>(&self, path: P, processed_samples: &[f32]) -> Result<()> {
        // Create output spec: always 32-bit float mono
        let output_spec = WavSpec {
            channels: 1,
            sample_rate: self.spec.sample_rate,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        
        let mut writer = WavWriter::create(path, output_spec)
            .context("Failed to create output WAV file")?;
        
        // Write processed samples
        for &sample in processed_samples {
            // Apply basic limiter to prevent clipping
            let limited_sample = sample.max(-1.0).min(1.0);
            writer.write_sample(limited_sample)
                .context("Failed to write sample")?;
        }
        
        writer.finalize()
            .context("Failed to finalize WAV file")?;
        
        eprintln!(
            "Wrote WAV: {}Hz, 1 channel, {} samples ({:.1}s)",
            output_spec.sample_rate,
            processed_samples.len(),
            processed_samples.len() as f32 / output_spec.sample_rate as f32
        );
        
        Ok(())
    }
}
