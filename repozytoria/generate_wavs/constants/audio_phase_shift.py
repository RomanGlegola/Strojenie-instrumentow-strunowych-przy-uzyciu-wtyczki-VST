"""
This module defines constants for different audio phase shift angles.

The phase shifts are used to adjust the phase of an audio signal, altering
the alignment of the waveform without changing its frequency or amplitude.
This can be useful in various audio processing applications, such as signal
mixing or phase cancellation.

Constants:
    - AUDIO_PHASE_SHIFT:
        A dictionary where keys represent phase shift categories
            ("none", "quarter", "half"), and values are tuples containing the
            corresponding phase shift angles in degrees (as float values).
"""

from typing import Final

AUDIO_PHASE_SHIFT: Final[dict[str, tuple[float, ...]]] = {
    "none": (0.0,),
    "quarter": (45.0,),
    "half": (90.0,),
}
