"""
This module defines constants for different audio waveforms.

Waveforms determine the shape of the audio signal and influence the
characteristics of the sound, such as its timbre and harmonic content. These
predefined waveform types are commonly used in audio synthesis and signal
processing.

Constants:
    - AUDIO_WAVE_FORM: A dictionary where keys represent waveform names
        ("sine", "square", "triangle"), and values are tuples containing the
        corresponding waveform type as strings.
"""

from typing import Final

AUDIO_WAVE_FORM: Final[dict[str, tuple[str, ...]]] = {
    "sine": ("sine",),
    "square": ("square",),
    "triangle": ("triangle",),
}
