"""
This module defines a set of audio configuration scenarios for testing and
analysis.

Each scenario combines multiple audio parameters, such as waveform type,
sample rate, bit depth, duration, frequency, amplitude, DC offset,
phase shift, and noise settings. The scenarios are designed to represent
various audio conditions, ranging from basic to more complex stress testing.

Imported Constants
------------------
- AUDIO_AMPLITUDE : Different amplitude levels for audio signals.
- AUDIO_BIT_DEPTH : Bit depths for various media standards.
- AUDIO_DC_OFFSET : DC offset levels to introduce signal bias.
- AUDIO_DURATION : Predefined duration categories for audio samples.
- AUDIO_TUNING : Standard guitar tuning frequencies and their semitones.
- AUDIO_NOISE_TYPE : Types of audio noise (e.g., "white", "pink").
- AUDIO_NOISE_VOLUME : Volume levels for different types of noise.
- AUDIO_PHASE_SHIFT : Phase shift angles to adjust the signal's phase.
- AUDIO_SAMPLE_RATE : Sample rates for TV, CD, DVD, and Blu-ray audio.
- AUDIO_WAVE_FORM : Types of audio waveforms used for signal generation.

Constants
---------
SCENARIOS : dict[str, dict[str, dict[str, tuple]]]
    A dictionary containing predefined audio scenarios:
    - "Smoke":
        Basic configuration with a single sine waveform and standard
        CD audio sample rate.
    - "Normal":
        Standard configuration with multiple sample rates,
        semitone frequencies, and added noise.
    - "Stress":
        Complex scenario with high-frequency signals, multiple
        phase shifts, and high noise levels.
"""

from typing import Final, Union

from constants.audio_amplitude import AUDIO_AMPLITUDE
from constants.audio_bit_depth import AUDIO_BIT_DEPTH
from constants.audio_dcoffset import AUDIO_DC_OFFSET
from constants.audio_duration import AUDIO_DURATION
from constants.audio_frequencies import AUDIO_TUNING
from constants.audio_noise import AUDIO_NOISE_TYPE, AUDIO_NOISE_VOLUME
from constants.audio_phase_shift import AUDIO_PHASE_SHIFT
from constants.audio_sample_rates import AUDIO_SAMPLE_RATE
from constants.audio_wave_forms import AUDIO_WAVE_FORM

SCENARIOS: Final[dict[str, dict[str, tuple[Union[str, int, float], ...]]]] = {
    "Smoke": {
        "waveform_type": AUDIO_WAVE_FORM["sine"],
        "sample_rate": AUDIO_SAMPLE_RATE["cd_audio"],
        "bit_depth": AUDIO_BIT_DEPTH["dvd_audio"],
        "duration": AUDIO_DURATION["short"],
        "frequency": AUDIO_TUNING["tuning_standard"],
        "amplitude": AUDIO_AMPLITUDE["standard"],
        "dc_offset": AUDIO_DC_OFFSET["none"],
        "phase_shift": AUDIO_PHASE_SHIFT["none"],
    },
    "Normal": {
        "waveform_type": AUDIO_WAVE_FORM["sine"],
        "sample_rate": AUDIO_SAMPLE_RATE["cd_audio"]
        + AUDIO_SAMPLE_RATE["dvd_audio"],
        "bit_depth": AUDIO_BIT_DEPTH["dvd_audio"],
        "duration": AUDIO_DURATION["five_seven"],
        "frequency": AUDIO_TUNING["tuning_semitones"],
        "amplitude": AUDIO_AMPLITUDE["standard"],
        "dc_offset": AUDIO_DC_OFFSET["none"],
        "phase_shift": AUDIO_PHASE_SHIFT["none"],
        "noise_type": AUDIO_NOISE_TYPE["none"] + AUDIO_NOISE_TYPE["pink"],
        "noise_volume": AUDIO_NOISE_VOLUME["none"] + AUDIO_NOISE_VOLUME["low"],
    },
    "Stress": {
        "waveform_type": AUDIO_WAVE_FORM["sine"],
        "sample_rate": AUDIO_SAMPLE_RATE["cd_audio"]
        + AUDIO_SAMPLE_RATE["dvd_audio"]
        + AUDIO_SAMPLE_RATE["blu-ray"],
        "bit_depth": AUDIO_BIT_DEPTH["dvd_audio"],
        "duration": AUDIO_DURATION["long"],
        "frequency": (440.0, 880.0, 1760.0),
        "amplitude": AUDIO_AMPLITUDE["standard"],
        "dc_offset": AUDIO_DC_OFFSET["none"],
        "phase_shift": AUDIO_PHASE_SHIFT["none"]
        + AUDIO_PHASE_SHIFT["quarter"],
        "noise_type": AUDIO_NOISE_TYPE["white"] + AUDIO_NOISE_TYPE["pink"],
        "noise_volume": AUDIO_NOISE_VOLUME["low"] + AUDIO_NOISE_VOLUME["mid"],
    },
}
