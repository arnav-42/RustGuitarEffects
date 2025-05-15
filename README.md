# RustGuitarEffects
Rust implementation of common electric guitar effects
Still a major WIP!

## Demo
**Sample Input:**
- [Clean Riff](https://sndup.net/2jqm6/)

**Sample Output:**
- [Distortion](https://sndup.net/y264c/)
- [Delay](https://sndup.net/m92qh)

Overdrive, Reverb, and Chorus are also available.

## Effects and Theory  
The current `input.wav` was from [here](https://freesound.org/people/aceinet/sounds/417150/).
#### Overdrive  
- **Type:** Soft-clipping waveshaper  
- **Formula:** `y = tanh(pre_gain * x) * level`  
- **Params:**  
    - `pre_gain` (α): saturation intensity  
    - `level`: output gain  

#### Distortion  
- **Type:** Hard-clipping + one-pole low-pass  
- **Formula:**  
  `y_clip = clamp(x, -T, +T)`  
  `y[n]   = y[n-1] + α * (y_clip - y[n-1])`  
- **Params:**  
    - `T`: clipping threshold  
    - `f_c`: filter cutoff frequency  

#### Chorus  
- **Type:** Delay line modulated by LFO  
- **Formula:**  
  `d(n) = D0 + D1 * sin(2π * f_LFO * n / Fs)`  
  `y(n) = 0.5 * x(n) + 0.5 * x(n - d(n))`  
- **Params:**  
    - `f_LFO`: modulation rate  
    - `depth_ms`: modulation depth (ms)  

#### Delay  
- **Type:** Feedback delay line  
- **Formula:**  
  `y(n) = x(n) + g * y(n - N)`  
  `N    = Fs * delay_time`  
- **Params:**  
    - `delay_time` (ms)  
    - `feedback` (0.0–1.0)  

#### Reverb  
- **Type:** Schroeder network (comb + all-pass filters)  
- **Comb Filter Formula:**  
  `y(n)      = buf[idx]`  
  `buf[idx]  = x(n) + g * y(n - N)`  
- **All-Pass Filter Formula:**  
  `y(n)      = -g * x(n) + buf[idx]`  
  `buf[idx]  = x(n) + g * y(n - N)`  
- **Params:**  
    - `mix`: dry/wet balance  

## How to use
- Clone  repo
- Build with `cargo build --release`
- To run an effect (eg Overdrive):
```
./target/release/rust-guitar-effects \
  --effect overdrive \
  samples/input.wav \
  output_overdrive.wav
  ```
- You can listen to the output wav in Reaper, Audacity, etc.
- You can add more samples to the `samples\` directory, and adjust file names accordingly.

## Next Steps
- Integrate real-time I/O with `cpal` or JACK
- Wrap as VST3/AU plugin via `nih-plug` or `vst-rs`
- Add MIDI/automation for live control
- Explore convolution reverb or pitch-shifting extensions


