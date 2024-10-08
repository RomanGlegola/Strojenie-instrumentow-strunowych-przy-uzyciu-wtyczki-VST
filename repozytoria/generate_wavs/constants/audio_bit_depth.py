"""
This module defines constants for different audio bit depths based on media
standards and applications.

Bit depth determines the number of bits used to represent each audio sample,
directly influencing the dynamic range and precision of the recorded audio.
Different bit depths are used for various audio formats, ranging from
low-quality telephone audio to high-quality digital audio workstations (DAWs).

Constants
---------
AUDIO_BIT_DEPTH : dict[str, tuple[int, ...]]
    A dictionary where keys represent different application categories
    ("telephone", "broadcasting", "video_tape", "cd_audio", "dvd_audio",
    "daw"), and values are tuples containing the corresponding bit depths
    in integer format.
"""

from typing import Final

AUDIO_BIT_DEPTH: Final[dict[str, tuple[int, ...]]] = {
    "telephone": (8,),
    "broadcasting": (
        10,
        11,
    ),
    "video_tape": (
        12,
        16,
    ),
    "cd_audio": (16,),
    "dvd_audio": (
        16,
        20,
        24,
    ),
    "daw": (
        4,
        8,
        16,
        24,
        32,
        48,
        64,
    ),
}
