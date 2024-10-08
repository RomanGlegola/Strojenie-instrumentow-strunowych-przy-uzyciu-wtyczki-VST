"""
This module defines constants for different audio amplitude levels.

The amplitudes represent the scaling factors applied to audio signals to
control their volume. These values can be used to set or adjust the perceived
loudness of the audio.

Constants:
    - AUDIO_AMPLITUDE:
        A dictionary where keys represent amplitude categories
            ("low", "standard", "high"), and values are tuples containing the
            corresponding amplitude scaling factors as float values.
"""

from typing import Final

AUDIO_AMPLITUDE: Final[dict[str, tuple[float, ...]]] = {
    "low": (0.5,),
    "standard": (0.5,),
    "high": (1.0,),
}
