"""
This module defines audio DC offset levels used for processing audio signals.

It includes predefined DC offset values that can be applied to audio data
to simulate different levels of signal bias.

Constants:
    - AUDIO_DC_OFFSET: A dictionary where keys represent the offset level
        names ("none", "low", "medium", "high") and values are tuples
        containing the corresponding DC offset values in float format.
"""

from typing import Final

AUDIO_DC_OFFSET: Final[dict[str, tuple[float, ...]]] = {
    "none": (0.0,),
    "low": (0.5,),
    "medium": (0.5,),
    "high": (1.0,),
}
