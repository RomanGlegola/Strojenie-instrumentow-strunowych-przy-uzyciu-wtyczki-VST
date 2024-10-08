"""
This module defines audio duration categories for signal processing.

It provides predefined durations in seconds to be used for audio processing
or analysis. The durations are grouped into categories based on length, such
as "short", "medium", and "long".

Constants:
    - AUDIO_DURATION: A dictionary where keys represent the duration
        category names ("short", "five_seven", "medium", "long"),
        and values are tuples containing the corresponding durations in
            float format (in seconds).
"""

from typing import Final

AUDIO_DURATION: Final[dict[str, tuple[float, ...]]] = {
    "short": (5.0,),
    "five_seven": (
        5.0,
        7.0,
    ),
    "medium": (7.0,),
    "long": (10.0,),
}
