"""
This module defines standard and semitone-based guitar tunings.

It uses the `numpy` library to calculate a sequence of semitone frequencies
based on the standard guitar tuning frequencies (E2, A2, D3, G3, B3, E4).

The module provides:
    - **_TUNING_STANDARD_A4**:
        A tuple containing the standard frequencies for a 6-string guitar.
    - **_TUNING_SEMITONES**:
        A tuple containing frequencies for 12 semitone intervals starting from
        each standard tuning.
    - **AUDIO_TUNING**:
        A dictionary that maps tuning names to their corresponding
        frequency tuples.

Constants
---------
- _TUNING_STANDARD_A4:
    Base frequencies for standard guitar tuning.
- _TUNING_SEMITONES:
    Frequencies for the semitone intervals based on the standard tuning.
- AUDIO_TUNING:
    Dictionary with named tuning sets.
"""

from typing import Final

import numpy as np

"""E2, A2, D3, G3, B3, E4"""
_TUNING_STANDARD_A4: Final[tuple[float, ...]] = (
    82.41,
    110.0,
    146.83,
    196.0,
    246.94,
    329.63,
)

"""
E2, F2, F#2/Gb2, G2, G#2/Ab2, A2, A#2/Bb2, B2, C3< C#3/Db3, D3, D#3/Eb3
A2, A#2/Bb2, B2, C3, C#3/Db3, D3, D#3/Eb3, E3, F3, F#3/Gb3, G3, G#3/Ab3
D3, D#3/Eb3, E3, F3, F#3/Gb3, G3, G#3/Ab3, A3, A#3/Bb3, B3, C4, C#4/Db4
G3, G#3/Ab3, A3, A#3/Bb3, B3, C4, C#4/Db4, D4, D#4/Eb4, E4, F4, F#4/Gb4
B3, C4, C#4/Db4, D4, D#4/Eb4, E4, F4, F#4/Gb4, G4, G#4/Ab4, A4, A#4/Bb4
E4, F4, F#4/Gb4, G4, G#4/Ab4, A4, A#4/Bb4, B4, C5, C#5/Db5, D5, D#5/Eb5
"""
_TUNING_SEMITONES: Final[tuple[float, ...]] = tuple(
    np.round(freq * (2 ** (n / 12)), decimals=2)
    for freq in _TUNING_STANDARD_A4
    for n in range(12)
)

AUDIO_TUNING: Final[dict[str, tuple[float, ...]]] = {
    "tuning_standard": _TUNING_STANDARD_A4,
    "tuning_semitones": _TUNING_SEMITONES,
}
