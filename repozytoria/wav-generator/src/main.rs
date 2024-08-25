use std::{f32::consts::PI};
use rand::Rng;
use hound::{WavWriter, WavSpec, SampleFormat};
use clap::{Arg, Command};


pub struct WavGenerator {
    /// Represents a WAV audio generator with various sound processing and manipulation capabilities.
    ///
    /// This struct provides a comprehensive set of parameters for generating WAV audio signals and applying various
    /// audio effects and filters. It allows for the creation of different types of waveforms, application of 
    /// filters, and adding effects such as reverb and noise. The generator can handle multiple audio channels and 
    /// supports customization of channel volumes.
    ///
    /// # Fields
    ///
    /// - `waveform_type` (Option<String>): 
    ///   - **Description:** Specifies the type of waveform to generate, such as "sine", "square", "triangle", etc.
    ///   - **Type:** `Option<String>`
    ///
    /// - `sample_rate`:
    ///   - **Description:** The number of samples per second (Hz) for the audio signal. Higher sample rates result in better audio quality.
    ///   - **Type:** `u32`
    ///
    /// - `bit_depth`:
    ///   - **Description:** The number of bits used to represent each audio sample. Common values are 16 (CD quality) or 24 (high quality).
    ///   - **Type:** `u16`
    ///
    /// - `duration`:
    ///   - **Description:** The total duration of the generated audio signal in seconds.
    ///   - **Type:** `f32`
    ///
    /// - `frequency`:
    ///   - **Description:** The fundamental frequency of the waveform in Hz. This affects the pitch of the generated sound.
    ///   - **Type:** `f32`
    ///
    /// - `amplitude`:
    ///   - **Description:** The maximum amplitude of the waveform. It controls the loudness of the generated sound.
    ///   - **Type:** `f32`
    ///
    /// - `dc_offset`:
    ///   - **Description:** An offset value added to the waveform to shift it vertically. This can be used to correct for DC bias in the signal.
    ///   - **Type:** `f32`
    ///
    /// - `phase_shift`:
    ///   - **Description:** A phase shift applied to the waveform. It shifts the waveform in time and can be used for phase modulation.
    ///   - **Type:** `f32`
    ///
    /// - `harmonics` (Option<Vec<(f32, f32)>>):
    ///   - **Description:** An optional list of harmonics to add to the waveform. Each harmonic is defined by a frequency and amplitude.
    ///   - **Type:** `Option<Vec<(f32, f32)>>`
    ///
    /// - `noise_type` (Option<String>):
    ///   - **Description:** Specifies the type of noise to add to the waveform, such as "white" or "pink".
    ///   - **Type:** `Option<String>`
    ///
    /// - `noise_volume`:
    ///   - **Description:** The volume of the added noise. Higher values increase the intensity of the noise.
    ///   - **Type:** `f32`
    ///
    /// - `low_pass_filter` (Option<f32>):
    ///   - **Description:** The cutoff frequency of a low-pass filter to apply to the waveform. Frequencies above this value are attenuated.
    ///   - **Type:** `Option<f32>`
    ///
    /// - `high_pass_filter` (Option<f32>):
    ///   - **Description:** The cutoff frequency of a high-pass filter to apply to the waveform. Frequencies below this value are attenuated.
    ///   - **Type:** `Option<f32>`
    ///
    /// - `fade_in`:
    ///   - **Description:** Duration of the fade-in effect at the beginning of the waveform, in seconds. The signal gradually increases from silence to full amplitude.
    ///   - **Type:** `f32`
    ///
    /// - `fade_out`:
    ///   - **Description:** Duration of the fade-out effect at the end of the waveform, in seconds. The signal gradually decreases from full amplitude to silence.
    ///   - **Type:** `f32`
    ///
    /// - `reverb_amount`:
    ///   - **Description:** The amount of reverb to apply to the waveform. Higher values increase the reverberation effect, making the sound more spacious.
    ///   - **Type:** `f32`
    ///
    /// - `num_channels`:
    ///   - **Description:** The number of audio channels (e.g., 1 for mono, 2 for stereo). Determines how many separate audio streams will be generated.
    ///   - **Type:** `u16`
    ///
    /// - `channel_volumes`:
    ///   - **Description:** A list of volume adjustments for each audio channel. The length of this list should match `num_channels`. Each value specifies the volume for the corresponding channel.
    ///   - **Type:** `Vec<f32>`
    pub waveform_type:    Option<String>,
    pub sample_rate:      u32,
    pub bit_depth:        u16,
    pub duration:         f32,
    pub frequency:        f32,
    pub amplitude:        f32,
    pub dc_offset:        f32,
    pub phase_shift:      f32,
    pub harmonics:        Option<Vec<(f32, f32)>>,
    pub noise_type:       Option<String>,
    pub noise_volume:     f32,
    pub low_pass_filter:  Option<f32>,
    pub high_pass_filter: Option<f32>,
    pub fade_in:          f32,
    pub fade_out:         f32,
    pub reverb_amount:    f32,
    pub num_channels:     u16,
    pub channel_volumes:  Vec<f32>,
}

impl WavGenerator {
    /// Generates an audio waveform based on the specified parameters.
    ///
    /// This function creates an audio waveform as a vector of samples and generates a filename suffix
    /// based on the waveform's properties. The waveform is generated according to the specified type
    /// (sine, square, triangle, or sawtooth) and includes various modifications such as harmonics, noise,
    /// and filters if specified.
    ///
    /// # Returns
    /// - `(Vec<f32>, String)`: A tuple containing:
    ///   - `Vec<f32>`: A vector of floating-point samples representing the generated waveform.
    ///   - `String`: A filename suffix describing the waveform parameters, useful for naming the output file.
    ///
    /// # Parameters
    /// - `&self`: A reference to the `WavGenerator` instance which contains the configuration for generating the waveform.
    ///
    /// # Waveform Generation Logic
    /// 1. **Sample Count Calculation:**
    ///    - `samples_count` (usize): The total number of samples to generate, calculated based on the sample rate and duration.
    /// 2. **Sample Generation Loop:**
    ///    - **Time Calculation:** `t` (f32) represents the current time point in seconds for the sample.
    ///    - **Initial Sample Calculation:** The sample is initially computed as a sine wave using the given amplitude, frequency, and phase shift.
    ///    - **Waveform Type Adjustment:**
    ///      - `"square"`: Converts the sine wave to a square wave by setting the sample to the amplitude or negative amplitude.
    ///      - `"triangle"`: Converts the sine wave to a triangle wave using the arcsine function.
    ///      - `"sawtooth"`: Converts the sine wave to a sawtooth wave by linearly increasing the sample value.
    ///
    /// # Filename Suffix Generation
    /// The function also generates a filename suffix based on the waveform type, frequency, amplitude, sample rate, bit depth, duration,
    pub fn generate_waveform(&self) -> (Vec<f32>, String) {
        let samples_count = (self.sample_rate as f32 * self.duration) as usize;
        let mut waveform = Vec::with_capacity(samples_count);

        for i in 0..samples_count {
            let t = i as f32 / self.sample_rate as f32;
            let mut sample = self.amplitude * (2.0 * PI * self.frequency * t + self.phase_shift).sin();

            if let Some(ref waveform_type) = self.waveform_type {
                match waveform_type.as_str() {
                    "square" => sample = if sample > 0.0 { self.amplitude } else { -self.amplitude },
                    "triangle" => sample = self.amplitude * (2.0 / PI) * (PI * self.frequency * t).asin(),
                    "sawtooth" => sample = 2.0 * self.amplitude * (self.frequency * t - (0.5 + (self.frequency * t).floor())),
                    _ => (),
                }
            }

            waveform.push(sample);
        }

        let waveform_type_str = self.waveform_type.as_deref().unwrap_or("sine");

        let mut filename_suffix = format!(
            "{}_freq{}Hz_amp{:.2}dB_samp{:.0}Hz{}bit_{:.2}s",
            waveform_type_str,
            self.frequency,
            20.0 * self.amplitude.log10(),
            self.sample_rate,
            self.bit_depth,
            self.duration
        );

        if self.dc_offset != 0.0 {
            self.apply_dc_offset(&mut waveform);
            filename_suffix.push_str(&format!("_{:.2}dco", self.dc_offset));
        }

        if let Some(ref harmonics) = self.harmonics {
            self.add_harmonics(&mut waveform);
            filename_suffix.push_str("_harmonics");
            for &(harmonic_freq, harmonic_amp) in harmonics.iter() {
                filename_suffix.push_str(&format!("-{}Hz_{:.2}", harmonic_freq, harmonic_amp));
            }
        }

        if let Some(ref noise_type) = self.noise_type {
            self.add_noise(&mut waveform);
            filename_suffix.push_str(&format!("_noise-{}", noise_type));
            if self.noise_volume != 0.0 {
                filename_suffix.push_str(&format!("_volume-{:.2}", self.noise_volume));
            }
        }

        if let Some(low_pass_filter) = self.low_pass_filter {
            self.apply_low_pass_filter(&mut waveform);
            filename_suffix.push_str(&format!("_{}Hz-lp", low_pass_filter));
        }

        if let Some(high_pass_filter) = self.high_pass_filter {
            self.apply_high_pass_filter(&mut waveform);
            filename_suffix.push_str(&format!("_{}Hz-hp", high_pass_filter));
        }

        if self.fade_in > 0.0 {
            self.apply_fades(&mut waveform);
            filename_suffix.push_str(&format!("_fade_in-{:.2}s", self.fade_in));
        }

        if self.fade_out > 0.0 {
            self.apply_fades(&mut waveform);
            filename_suffix.push_str(&format!("_fade_out-{:.2}s", self.fade_out));
        }

        if self.reverb_amount > 0.0 {
            self.apply_reverb(&mut waveform);
            filename_suffix.push_str(&format!("_reverb-{:.2}", self.reverb_amount));
        }

        self.normalize_channels(&mut waveform);

        (waveform, filename_suffix)
    }

    /// Saves the generated waveform to a WAV file with the specified filename.
    ///
    /// This function generates the waveform based on the current parameters of the `WavGenerator` instance,
    /// and then writes it to a WAV file. The filename provided is used as the base name for the output file,
    /// with additional details appended based on the waveform properties.
    ///
    /// # Parameters
    /// - `filename: &str`: The base name for the output WAV file. The function appends a suffix to this base name
    ///   based on the waveform properties (e.g., waveform type, frequency, amplitude).
    ///
    /// # Returns
    /// - `Result<(), Box<dyn std::error::Error>>`: A `Result` type where:
    ///   - `Ok(())` indicates that the file was successfully saved.
    ///   - `Err(Box<dyn std::error::Error>)` contains an error message if there was a problem creating or writing to the file.
    ///
    /// # Operation
    /// 1. **Generate Waveform:**
    ///    - The function calls `generate_waveform()` to create the audio data and derive the filename suffix based on the waveform properties.
    ///
    /// 2. **Construct Final Filename:**
    ///    - The base filename provided by the user is combined with the generated suffix to form the full output filename (with `.wav` extension).
    ///
    /// 3. **Define WAV Specifications:**
    ///    - A `WavSpec` object is created to specify the number of channels, sample rate, bit depth, and sample format for the WAV file.
    ///
    /// 4. **Create WAV Writer:**
    ///    - A `WavWriter` is instantiated with the final filename and specifications, enabling writing samples to the WAV file.
    ///
    /// 5. **Write Samples:**
    ///    - The waveform samples are converted from floating-point values to 16-bit integers and written to the WAV file using the `WavWriter`.
    ///
    /// 6. **Handle Errors:**
    ///    - If any errors occur during file creation or writing, they are returned as an `Err` variant of the `Result`.
    ///
    pub fn save_to_wav(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (waveform, filename_suffix) = self.generate_waveform();
        let final_filename = format!("{}{}.wav", filename, filename_suffix);

        let spec = WavSpec {
            channels: self.num_channels,
            sample_rate: self.sample_rate,
            bits_per_sample: self.bit_depth,
            sample_format: SampleFormat::Int,
        };

        let mut writer = WavWriter::create(final_filename, spec)?;

        for sample in waveform {
            let sample_int = (sample * i16::MAX as f32) as i16;
            writer.write_sample(sample_int)?;
        }

        Ok(())
    }

    /// Applies a DC offset to the waveform samples.
    ///
    /// This function modifies the waveform by adding a constant DC offset value to each sample. 
    /// This can be used to shift the waveform vertically in the amplitude domain, which is useful 
    /// for correcting or adjusting the baseline level of the signal.
    ///
    /// # Parameters
    /// - `waveform: &mut [f32]`: A mutable slice of floating-point samples representing the waveform.
    ///   Each sample in this slice will be increased by the value of `self.dc_offset`.
    ///
    /// # Operation
    /// The function iterates over each sample in the `waveform` slice and adds the `dc_offset` value
    /// to it. This operation is performed in-place, meaning the original waveform is directly modified.
    pub fn apply_dc_offset(&self, waveform: &mut [f32]) {
        for sample in waveform.iter_mut() {
            *sample += self.dc_offset;
        }
    }

    /// Applies a phase shift to the waveform.
    ///
    /// This function adjusts the phase of the waveform by the specified phase shift.
    /// 
    /// # Arguments
    /// - `waveform` (&mut [f32]): A mutable slice of the waveform samples.
    ///
    /// **Note**: The phase shift is already applied in `generate_waveform`, but this method can be used if further adjustments are needed.
    pub fn apply_phase_shift(&self, _waveform: &mut [f32]) {
    }

    /// Adds harmonic components to the waveform.
    ///
    /// This function enhances the waveform by adding harmonic frequencies to it. Each harmonic is defined
    /// by its frequency and amplitude. Harmonics are integer multiples of the fundamental frequency and
    /// contribute to the richness and complexity of the sound.
    ///
    /// # Parameters
    /// - `waveform: &mut [f32]`: A mutable slice of floating-point samples representing the waveform. 
    ///   The function modifies this waveform by adding harmonic components to each sample.
    ///
    /// # Operation
    /// If `self.harmonics` is `Some`, the function iterates over each harmonic tuple in the list. For each 
    /// harmonic, it calculates its contribution to the waveform by adding a sine wave of the harmonic frequency 
    /// scaled by the harmonic amplitude. This is done for each sample in the `waveform` slice, and the waveform 
    /// is modified in-place.
    pub fn add_harmonics(&self, waveform: &mut [f32]) {
        if let Some(ref harmonics) = self.harmonics {
            for &(harmonic_freq, harmonic_amp) in harmonics.iter() {
                for i in 0..waveform.len() {
                    let t = i as f32 / self.sample_rate as f32;
                    waveform[i] += harmonic_amp * (2.0 * PI * harmonic_freq * t + self.phase_shift).sin();
                }
            }
        }
    }

    /// Adds noise to the waveform.
    ///
    /// This function introduces noise to the waveform based on the specified `noise_type`. It modifies each 
    /// sample in the waveform by adding a noise component, which can be of various types:
    /// 
    /// - **White Noise**: Random noise with equal intensity at different frequencies.
    /// - **Pink Noise**: Noise with frequency characteristics that decrease in power as frequency increases.
    ///
    /// The amount of noise added is controlled by `self.noise_volume`, which scales the noise contribution.
    ///
    /// # Parameters
    /// - `waveform: &mut [f32]`: A mutable slice of floating-point samples representing the waveform. 
    ///   The function will modify this waveform by adding noise to each sample.
    ///
    /// # Operation
    /// The function checks the `noise_type` specified in `self.noise_type`. Based on the type:
    /// - For `"white"` noise, it generates random values in the range [-1.0, 1.0] and scales them by `self.noise_volume`.
    /// - For `"pink"` noise, it generates noise using a pink noise algorithm and scales it by `self.noise_volume`.
    /// - If the `noise_type` is not recognized, no noise is added.
    /// 
    /// The noise is added to each sample in the `waveform` slice, modifying it in-place.
    pub fn add_noise(&self, waveform: &mut [f32]) {
        if let Some(ref noise_type) = self.noise_type {
            let mut rng = rand::thread_rng();
            for sample in waveform.iter_mut() {
                let noise_sample = match noise_type.as_str() {
                    "white" => rng.gen_range(-1.0..1.0) * self.noise_volume,
                    "pink" => self.generate_pink_noise() * self.noise_volume,
                    _ => 0.0,
                };
                *sample += noise_sample;
            }
        }
    }

    /// Generates a pink noise sample using a Voss-McCartney algorithm.
    ///
    /// This function uses a pseudo-random number generator to produce a sample of pink noise. Pink noise 
    /// is a type of noise where each octave has equal energy, resulting in a frequency spectrum that decreases 
    /// in power as frequency increases. This is different from white noise, which has equal intensity across all frequencies.
    ///
    /// The implementation uses a static array of random values and a specific algorithm to generate pink noise. 
    /// The `unsafe` block is used to manage the static mutable state required for generating noise.
    ///
    /// # Returns
    /// - `f32`: A single sample of pink noise as a floating-point value. This value is in the range of [-1.0, 1.0].
    ///
    /// # Operation
    /// - Initializes random values if they haven't been set before (`INDEX == 0`).
    /// - Uses a Voss-McCartney algorithm to generate pink noise. This involves updating an array of random values 
    ///   and computing the average value to produce the final noise sample.
    /// - Updates static variables (`INDEX` and `LAST_OUTPUT`) to maintain state across function calls.
    ///
    /// # Safety
    /// - The function uses `unsafe` blocks to manage static mutable state. Ensure that no other parts of the code 
    ///   are concurrently accessing or modifying these static variables to avoid undefined behavior.
    /// - Static mutable variables (`RANDOM_VALUES`, `INDEX`, `LAST_OUTPUT`) are shared across function calls, so 
    ///   careful consideration is required to maintain thread safety and avoid potential issues.
    fn generate_pink_noise(&self) -> f32 {
        const NUM_RANDOM_STREAMS: usize = 16;
        static mut RANDOM_VALUES: [f32; NUM_RANDOM_STREAMS] = [0.0; NUM_RANDOM_STREAMS];
        static mut INDEX: usize = 0;
        static mut LAST_OUTPUT: f32 = 0.0;
        
        unsafe {  // Inicjalizuj wartości losowe
            if INDEX == 0 {
                for i in 0..NUM_RANDOM_STREAMS {
                    RANDOM_VALUES[i] = rand::random::<f32>() * 2.0 - 1.0;
                }
            }
    
            let mut sum = 0.0;
            let mut j = INDEX;
            
            for i in 0..NUM_RANDOM_STREAMS {
                if (j & 1) != 0 {
                    RANDOM_VALUES[i] = rand::random::<f32>() * 2.0 - 1.0;
                }
                sum += RANDOM_VALUES[i];
                j >>= 1;
            }
    
            INDEX += 1;
            LAST_OUTPUT = sum / NUM_RANDOM_STREAMS as f32;

            LAST_OUTPUT
        }
    }

    /// Applies a low-pass filter to the waveform.
    ///
    /// A low-pass filter allows frequencies below the cutoff frequency to pass through while attenuating
    /// higher frequencies. This filter is implemented using a simple first-order RC (Resistor-Capacitor) filter,
    /// which is suitable for basic audio processing applications.
    ///
    /// # Parameters
    /// - `&self`: A reference to the `WavGenerator` instance.
    /// - `waveform: &mut [f32]`: A mutable reference to an array of `f32` values representing the audio waveform samples. 
    ///   The filter is applied in place, directly modifying the waveform data.
    ///
    /// # Filter Logic
    /// - `cutoff_freq` (f32): The cutoff frequency of the low-pass filter, specified in Hertz (Hz). Frequencies above
    ///   this value are attenuated.
    /// - `dt` (f32): The time between samples, calculated as the inverse of the sample rate.
    /// - `rc` (f32): The time constant of the RC filter, calculated as the inverse of `2 * PI * cutoff_freq`.
    /// - `alpha` (f32): The smoothing factor for the filter, calculated as `dt / (rc + dt)`.
    ///
    /// # Operation
    /// The filter iterates over each sample in the waveform. For each sample, it applies the following formula:
    /// 
    /// `sample[i] = alpha * (sample[i] + previous_sample)`, where `previous_sample` is the last processed sample.
    /// 
    /// This formula blends the current sample with the previous one to attenuate high frequencies.
    ///
    /// # Safety
    /// This function assumes that the waveform has at least one sample. The first sample is used as the initial
    /// `previous_sample` and is not modified.
    pub fn apply_low_pass_filter(&self, waveform: &mut [f32]) {
        if let Some(cutoff_freq) = self.low_pass_filter {
            let dt = 1.0 / self.sample_rate as f32;
            let rc = 1.0 / (2.0 * PI * cutoff_freq);
            let alpha = dt / (rc + dt);
    
            let mut previous_sample = waveform[0];
    
            for (i, sample) in waveform.iter_mut().enumerate() {
                if i == 0 {
                    continue;
                }
                *sample = alpha * (*sample + previous_sample);
                previous_sample = *sample;
            }
        }
    }

    /// Applies a high-pass filter to the waveform.
    ///
    /// A high-pass filter allows frequencies above the cutoff frequency to pass through while attenuating
    /// lower frequencies. This is achieved using a simple first-order RC (Resistor-Capacitor) filter,
    /// which is often used for basic audio signal processing.
    ///
    /// # Parameters
    /// - `&self`: A reference to the `WavGenerator` instance.
    /// - `waveform: &mut [f32]`: A mutable reference to an array of `f32` values representing the audio waveform samples.
    ///   The filter modifies the waveform data in place.
    ///
    /// # Filter Logic
    /// - `cutoff_freq` (f32): The cutoff frequency of the high-pass filter, specified in Hertz (Hz). Frequencies below
    ///   this value are attenuated.
    /// - `dt` (f32): The time interval between samples, calculated as the inverse of the sample rate.
    /// - `rc` (f32): The time constant of the RC filter, calculated as the inverse of `2 * PI * cutoff_freq`.
    /// - `alpha` (f32): The smoothing factor for the filter, calculated as `rc / (rc + dt)`.
    ///
    /// # Operation
    /// The filter iterates over each sample in the waveform. For each sample, it applies the following formula:
    ///
    /// `waveform[i] = alpha * (waveform[i - 1] + current_sample - previous_sample)`, 
    /// where `current_sample` is the current sample value, `previous_sample` is the last processed sample, 
    /// and `waveform[i - 1]` is the previously filtered sample.
    ///
    /// This operation effectively attenuates the low frequencies, allowing the higher frequencies to pass.
    ///
    /// # Safety
    /// This function assumes that the waveform has at least one sample. The first sample is used as the initial
    /// `previous_sample` and is set directly in the filtered waveform.
    pub fn apply_high_pass_filter(&self, waveform: &mut [f32]) {
        if let Some(cutoff_freq) = self.high_pass_filter {
            let dt = 1.0 / self.sample_rate as f32;
            let rc = 1.0 / (2.0 * PI * cutoff_freq);
            let alpha = rc / (rc + dt);
    
            let mut previous_sample = waveform[0];
    
            for i in 0..waveform.len() {
                let current_sample = waveform[i];
                if i == 0 {
                    waveform[i] = current_sample;
                } else {
                    waveform[i] = alpha * (waveform[i - 1] + current_sample - previous_sample);
                }
                previous_sample = current_sample;
            }
        }
    }

    /// Applies fade-in and fade-out effects to the waveform.
    ///
    /// This function adjusts the amplitude of the waveform over time to create fade-in and fade-out effects.
    /// A fade-in gradually increases the amplitude from zero to full volume over a specified duration at
    /// the beginning of the waveform. Conversely, a fade-out gradually decreases the amplitude from full
    /// volume to zero over a specified duration at the end of the waveform.
    ///
    /// # Parameters
    /// - `&self`: A reference to the `WavGenerator` instance.
    /// - `waveform: &mut [f32]`: A mutable reference to an array of `f32` values representing the audio waveform samples.
    ///   The function modifies the waveform data in place by applying the fade effects.
    ///
    /// # Fade Logic
    /// - `fade_in_samples` (usize): The number of samples over which the fade-in effect is applied, calculated as
    ///   `self.sample_rate * self.fade_in`.
    /// - `fade_out_samples` (usize): The number of samples over which the fade-out effect is applied, calculated as
    ///   `self.sample_rate * self.fade_out`.
    /// - `total_samples` (usize): The total number of samples in the waveform, derived from the length of the `waveform` array.
    ///
    /// # Operation
    /// 1. **Fade-in:**
    ///    - The function iterates over the first `fade_in_samples` samples of the waveform.
    ///    - For each sample `i`, a scaling factor is computed as `i / fade_in_samples`.
    ///    - The sample's amplitude is then multiplied by this factor, gradually increasing it from `0.0` to `1.0`.
    ///
    /// 2. **Fade-out:**
    ///    - The function iterates over the last `fade_out_samples` samples of the waveform.
    ///    - For each sample `i`, an inverse scaling factor is computed as `1.0 - (i / fade_out_samples)`.
    ///    - The sample's amplitude is then multiplied by this factor, gradually decreasing it from `1.0` to `0.0`.
    pub fn apply_fades(&self, waveform: &mut [f32]) {
        let fade_in_samples = (self.sample_rate as f32 * self.fade_in) as usize;
        let fade_out_samples = (self.sample_rate as f32 * self.fade_out) as usize;
        let total_samples = waveform.len();

        for i in 0..fade_in_samples {  // Apply fade-in
            let factor = i as f32 / fade_in_samples as f32;
            waveform[i] *= factor;
        }

        for i in 0..fade_out_samples {  // Apply fade-out
            let index = total_samples - fade_out_samples + i;
            let factor = i as f32 / fade_out_samples as f32;
            waveform[index] *= 1.0 - factor;
        }
    }

    /// Applies a simple reverb effect to the waveform.
    ///
    /// This function simulates a reverb effect by adding a delayed and decayed version of the waveform to itself.
    /// Reverb gives the impression of space and depth in audio by mimicking the reflections of sound waves in an environment.
    ///
    /// # Parameters
    /// - `&self`: A reference to the `WavGenerator` instance.
    /// - `waveform: &mut [f32]`: A mutable reference to an array of `f32` values representing the audio waveform samples.
    ///   The function modifies this array in place to apply the reverb effect.
    ///
    /// # Reverb Logic
    /// - `delay_samples` (usize): The number of samples by which the waveform is delayed to create the echo effect.
    ///   This is calculated as `self.sample_rate * 0.1`, corresponding to a delay of 100 milliseconds.
    /// - `decay_factor` (f32): The factor by which the delayed waveform is attenuated before being added back to the original waveform.
    ///   This is set by `self.reverb_amount`.
    ///
    /// # Operation
    /// 1. **Delay Calculation:**
    ///    - The function calculates the delay in samples based on the sample rate and a fixed delay time of 0.1 seconds (100 milliseconds).
    ///
    /// 2. **Apply Reverb:**
    ///    - The function iterates over the waveform starting from the `delay_samples` index.
    ///    - For each sample `i`, the function adds a scaled (by `decay_factor`) version of the sample at `i - delay_samples` to the current sample.
    ///    - This process simulates the echoing effect characteristic of reverb.
    pub fn apply_reverb(&self, waveform: &mut [f32]) {
        let delay_samples = (self.sample_rate as f32 * 0.1) as usize;
        let decay_factor = self.reverb_amount;

        for i in delay_samples..waveform.len() {
            waveform[i] += waveform[i - delay_samples] * decay_factor;
        }
    }

    /// Normalizes the amplitude of the audio waveform to the maximum possible value.
    ///
    /// This function adjusts the amplitude of the audio waveform so that the highest peak reaches the maximum value of 1.0.
    /// Normalization is useful for maximizing the signal strength without distortion, ensuring that the waveform uses the full dynamic range.
    ///
    /// # Parameters
    /// - `&self`: A reference to the `WavGenerator` instance.
    /// - `waveform: &mut [f32]`: A mutable reference to an array of `f32` values representing the audio waveform samples.
    ///   The function modifies this array in place to apply normalization.
    ///
    /// # Normalization Logic
    /// - `max_amplitude` (f32): The maximum absolute amplitude found in the waveform. This value is used to calculate the normalization factor.
    /// - `normalization_factor` (f32): The factor by which each sample is multiplied to scale the waveform such that the highest peak equals 1.0.
    ///   This is calculated as `1.0 / max_amplitude`.
    ///
    /// # Operation
    /// 1. **Find Maximum Amplitude:**
    ///    - The function iterates through the waveform to find the maximum amplitude (`max_amplitude`).
    ///    - This value represents the highest peak in the waveform and is used to determine how much the entire waveform needs to be scaled.
    ///
    /// 2. **Calculate Normalization Factor:**
    ///    - The normalization factor is computed as `1.0 / max_amplitude`. This ensures that the highest peak in the waveform reaches 1.0 after scaling.
    ///
    /// 3. **Apply Normalization:**
    ///    - The function iterates through the waveform again, multiplying each sample by the normalization factor to scale the waveform.
    ///
    /// # Panics
    /// - The function does not currently handle the edge case of an empty waveform, which would cause the normalization logic to panic.
    pub fn normalize_channels(&self, waveform: &mut [f32]) {
        let max_amplitude = waveform.iter().cloned().fold(0.0, f32::max);
        let normalization_factor = 1.0 / max_amplitude;

        for sample in waveform.iter_mut() {
            *sample *= normalization_factor;
        }
    }
}

fn parse_harmonics(s: &str) -> Vec<(f32, f32)> {
    s.split(';')
        .filter_map(|pair| {
            let mut parts = pair.split(',');
            let freq = parts.next()?.parse().ok()?;
            let amp = parts.next()?.parse().ok()?;
            Some((freq, amp))
        })
        .collect()
}

fn parse_f32_vec(s: &str) -> Vec<f32> {
    s.split(',')
        .filter_map(|num| num.parse::<f32>().ok())
        .collect()
}

fn main() {
    let matches = Command::new("WavGenerator")
        .version("1.0")
        .author("Author Name")
        .about("Generates WAV files with customizable parameters")
        .arg(Arg::new("waveform_type")
            .long("waveform_type")
            .value_name("TYPE")
            .help("Type of waveform (e.g., sine, square, etc.)")
            .default_value("sine"))
        .arg(Arg::new("sample_rate")
            .long("sample_rate")
            .value_name("RATE")
            .help("Sample rate of the audio")
            .default_value("44100"))
        .arg(Arg::new("bit_depth")
            .long("bit_depth")
            .value_name("DEPTH")
            .help("Bit depth of the audio")
            .default_value("16"))
        .arg(Arg::new("duration")
            .long("duration")
            .value_name("SECONDS")
            .help("Duration of the audio in seconds")
            .default_value("5.0"))
        .arg(Arg::new("frequency")
            .long("frequency")
            .value_name("FREQUENCY")
            .help("Frequency of the waveform")
            .default_value("440.0"))
        .arg(Arg::new("amplitude")
            .long("amplitude")
            .value_name("AMPLITUDE")
            .help("Amplitude of the waveform")
            .default_value("0.5"))
        .arg(Arg::new("dc_offset")
            .long("dc_offset")
            .value_name("DC_OFFSET")
            .help("DC offset of the waveform")
            .default_value("0.0"))
        .arg(Arg::new("phase_shift")
            .long("phase_shift")
            .value_name("PHASE_SHIFT")
            .help("Phase shift of the waveform")
            .default_value("0.0"))
        .arg(Arg::new("harmonics")
            .long("harmonics")
            .value_name("HARMONICS")
            .help("Harmonics in the form freq,amp;freq,amp")
            .default_value(""))
        .arg(Arg::new("noise_type")
            .long("noise_type")
            .value_name("NOISE")
            .help("Type of noise (e.g., white, pink)")
            .default_value(""))
        .arg(Arg::new("noise_volume")
            .long("noise_volume")
            .value_name("VOLUME")
            .help("Volume of the noise")
            .default_value("0.0"))
        .arg(Arg::new("low_pass_filter")
            .long("low_pass_filter")
            .value_name("LOW_PASS")
            .help("Low-pass filter cutoff frequency")
            .default_value("0.0"))
        .arg(Arg::new("high_pass_filter")
            .long("high_pass_filter")
            .value_name("HIGH_PASS")
            .help("High-pass filter cutoff frequency")
            .default_value("0.0"))
        .arg(Arg::new("fade_in")
            .long("fade_in")
            .value_name("FADE_IN")
            .help("Fade-in duration in seconds")
            .default_value("0.0"))
        .arg(Arg::new("fade_out")
            .long("fade_out")
            .value_name("FADE_OUT")
            .help("Fade-out duration in seconds")
            .default_value("0.0"))
        .arg(Arg::new("reverb_amount")
            .long("reverb_amount")
            .value_name("REVERB")
            .help("Amount of reverb")
            .default_value("0.0"))
        .arg(Arg::new("num_channels")
            .long("num_channels")
            .value_name("CHANNELS")
            .help("Number of audio channels")
            .default_value("1"))
        .arg(Arg::new("channel_volumes")
            .long("channel_volumes")
            .value_name("VOLUMES")
            .help("Comma-separated list of volumes for each channel")
            .default_value("1.0"))
        .arg(Arg::new("output_filename")
            .long("output_filename")
            .value_name("FILENAME")
            .help("Name of the output file")
            .default_value("output"))
        .get_matches();

    // Read argument values
    let waveform_type = matches.get_one::<String>("waveform_type").unwrap();
    let sample_rate: u32 = matches.get_one::<String>("sample_rate").unwrap().parse().expect("Invalid sample rate");
    let bit_depth: u16 = matches.get_one::<String>("bit_depth").unwrap().parse().expect("Invalid bit depth");
    let duration: f32 = matches.get_one::<String>("duration").unwrap().parse().expect("Invalid duration");
    let frequency: f32 = matches.get_one::<String>("frequency").unwrap().parse().expect("Invalid frequency");
    let amplitude: f32 = matches.get_one::<String>("amplitude").unwrap().parse().expect("Invalid amplitude");
    let dc_offset: f32 = matches.get_one::<String>("dc_offset").unwrap().parse().expect("Invalid DC offset");
    let phase_shift: f32 = matches.get_one::<String>("phase_shift").unwrap().parse().expect("Invalid phase shift");
    let harmonics = parse_harmonics(matches.get_one::<String>("harmonics").unwrap());
    let noise_type = matches.get_one::<String>("noise_type").unwrap();
    let noise_volume: f32 = matches.get_one::<String>("noise_volume").unwrap().parse().expect("Invalid noise volume");
    let low_pass_filter: f32 = matches.get_one::<String>("low_pass_filter").unwrap().parse().expect("Invalid low-pass filter value");
    let high_pass_filter: f32 = matches.get_one::<String>("high_pass_filter").unwrap().parse().expect("Invalid high-pass filter value");
    let fade_in: f32 = matches.get_one::<String>("fade_in").unwrap().parse().expect("Invalid fade-in duration");
    let fade_out: f32 = matches.get_one::<String>("fade_out").unwrap().parse().expect("Invalid fade-out duration");
    let reverb_amount: f32 = matches.get_one::<String>("reverb_amount").unwrap().parse().expect("Invalid reverb amount");
    let num_channels: u16 = matches.get_one::<String>("num_channels").unwrap().parse().expect("Invalid number of channels");
    let channel_volumes = parse_f32_vec(matches.get_one::<String>("channel_volumes").unwrap());
    let output_filename = matches.get_one::<String>("output_filename").unwrap();

    // Ensure that channel_volumes length matches num_channels
    if channel_volumes.len() != num_channels as usize {
        panic!("Number of channel volumes must match the number of channels");
    }

    let generator = WavGenerator {
        waveform_type:    Some(waveform_type.to_string()),
        sample_rate,
        bit_depth,
        duration,
        frequency,
        amplitude,
        dc_offset,
        phase_shift,
        harmonics:        Some(harmonics),
        noise_type:       if noise_type.is_empty() { None } else { Some(noise_type.to_string()) },
        noise_volume,
        low_pass_filter:  if low_pass_filter > 0.0 { Some(low_pass_filter) } else { None },
        high_pass_filter: if high_pass_filter > 0.0 { Some(high_pass_filter) } else { None },
        fade_in,
        fade_out,
        reverb_amount,
        num_channels,
        channel_volumes,
    };

    generator.save_to_wav(output_filename).expect("Failed to save WAV file");
}