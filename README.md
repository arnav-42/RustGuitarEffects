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

The current `input.wav` was from [here](https://freesound.org/people/aceinet/sounds/417150/).

To run an effect (eg Overdrive):
```
./target/release/rust-guitar-effects \
  --effect overdrive \
  samples/input.wav \
  output_overdrive.wav
  ```
You can listen to the output wav in Reaper, Audacity, etc. Also, you can add more samples to the `samples\` directory, and adjust file names accordingly.


