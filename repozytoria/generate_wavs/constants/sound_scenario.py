from typing import Final

from constants.audio_amplitude import AUDIO_AMPLITUDE
from constants.audio_bit_depth import AUDIO_BIT_DEPTH
from constants.audio_dcoffset import AUDIO_DC_OFFSET
from constants.audio_duration import AUDIO_DURATION
from constants.audio_frequencies import TUNING
from constants.audio_noise import AUDIO_NOISE_TYPE, AUDIO_NOISE_VOLUME
from constants.audio_phase_shift import AUDIO_PHASE_SHIFT
from constants.audio_sample_rates import AUDIO_SAMPLE_RATE
from constants.audio_wave_forms import AUDIO_WAVE_FORM

SCENARIOS: Final[dict[str, dict[str, dict[str, tuple]]], ...] = {
    "Smoke": {
        "waveform_type": AUDIO_WAVE_FORM["sine"],
        "sample_rate": AUDIO_SAMPLE_RATE["cd_audio"],
        "bit_depth": AUDIO_BIT_DEPTH["dvd_audio"],
        "duration": AUDIO_DURATION["short"],
        "frequency": TUNING["tuning_standard"],
        "amplitude": AUDIO_AMPLITUDE["standard"],
        "dc_offset": AUDIO_DC_OFFSET["none"],
        "phase_shift": AUDIO_PHASE_SHIFT["none"],
    },
    "Normal": {
        "waveform_type": AUDIO_WAVE_FORM["sine"],
        "sample_rate": AUDIO_SAMPLE_RATE["cd_audio"] + AUDIO_SAMPLE_RATE["dvd_audio"],
        "bit_depth": AUDIO_BIT_DEPTH["dvd_audio"],
        "duration": AUDIO_DURATION["five_seven"],
        "frequency": TUNING["tuning_semitones"],
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
        "phase_shift": AUDIO_PHASE_SHIFT["none"] + AUDIO_PHASE_SHIFT["quarter"],
        "noise_type": AUDIO_NOISE_TYPE["white"] + AUDIO_NOISE_TYPE["pink"],
        "noise_volume": AUDIO_NOISE_VOLUME["low"] + AUDIO_NOISE_VOLUME["mid"],
    },
}
