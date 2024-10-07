from data.audio_frequencies import tuning
from data.audio_sample_rates import sample_rate
from data.audio_wave_forms import wave_form
from data.audio_duration import duration
from data.audio_bit_depth import bit_depth
from data.audio_amplitude import amplitude
from data.audio_dcoffset import dc_offset
from data.audio_phase_shift import phase_shift
from data.audio_noise import noise_type, noise_volume


scenarios: dict = {
    "Smoke": {
        "waveform_type": wave_form["sine"],
        "sample_rate": sample_rate["cd_audio"],
        "bit_depth": bit_depth["dvd_audio"],
        "duration": duration["short"],
        "frequency": tuning["tuning_standard"],
        "amplitude": amplitude["standard"],
        "dc_offset": dc_offset["none"],
        "phase_shift": phase_shift["none"],
    },
    "Normal": {
        "waveform_type": wave_form["sine"],
        "sample_rate":
            sample_rate["cd_audio"]
            + sample_rate["dvd_audio"],
        "bit_depth": bit_depth["dvd_audio"],
        "duration": duration["five_seven"],
        "frequency": tuning["tuning_semitones"],
        "amplitude": amplitude["standard"],
        "dc_offset": dc_offset["none"],
        "phase_shift": phase_shift["none"],
        "noise_type": noise_type["none"] + noise_type["pink"],
        "noise_volume": noise_volume["none"] + noise_volume["low"],
    },
    "Stress": {
        "waveform_type": wave_form["sine"],
        "sample_rate":
            sample_rate["cd_audio"]
            + sample_rate["dvd_audio"]
            + sample_rate["blu-ray"],
        "bit_depth": bit_depth["dvd_audio"],
        "duration": duration["long"],
        "frequency": (440.0, 880.0, 1760.0),
        "amplitude": amplitude["standard"],
        "dc_offset": dc_offset["none"],
        "phase_shift": phase_shift["none"] + phase_shift["quarter"],
        "noise_type": noise_type["white"] + noise_type["pink"],
        "noise_volume": noise_volume["low"] + noise_volume["mid"],
    },
}
