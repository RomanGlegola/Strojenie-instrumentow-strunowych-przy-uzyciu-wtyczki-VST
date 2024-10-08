"""
This module defines constants for audio noise types and their respective
volume levels.

It provides predefined categories of noise (e.g., "white" or "pink") and
corresponding volume settings, allowing for easy configuration and simulation
of various noise conditions in audio processing.

Constants
---------
AUDIO_NOISE_TYPE : dict[str, tuple[str, ...]]
    A dictionary where keys represent noise type categories ("none", "white",
    "pink"), and values are tuples containing corresponding noise type names
    as strings.

AUDIO_NOISE_VOLUME : dict[str, tuple[float, ...]]
    A dictionary where keys represent volume levels for noise ("none", "low",
    "mid", "high", "full"), and values are tuples containing corresponding
    volume levels as float values (ranging from 0.0 to 1.0).
"""

from typing import Final

AUDIO_NOISE_TYPE: Final[dict[str, tuple[str, ...]]] = {
    "none": ("",),
    "white": ("white",),
    "pink": ("pink",),
}

AUDIO_NOISE_VOLUME: Final[dict[str, tuple[float, ...]]] = {
    "none": (0.0,),
    "low": (0.1,),
    "mid": (0.3,),
    "high": (0.5,),
    "full": (1.0,),
}
