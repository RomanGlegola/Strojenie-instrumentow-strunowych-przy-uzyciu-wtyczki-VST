"""
This module defines constants for various audio sample rates used in
different media formats.

The sample rates are grouped by common media categories, such as TV/Radio,
CD, DVD, and Blu-ray audio standards. Each sample rate is represented
in Hertz (Hz) and indicates the number of samples per second used in
digital audio recordings.

Constants:
    - _TV_RADIO_VOIP:
        A tuple containing sample rates used for TV, radio, and VoIP audio
            (8000, 16000, 32000 Hz).
    - _CD_AUDIO:
        A tuple with standard sample rates for CD audio (22050, 44100 Hz).
    - _DVD_AUDIO:
        A tuple with standard sample rates for DVD audio (48000, 96000 Hz).
    - _BLU_RAY_AUDIO:
        A tuple with sample rates for Blu-ray audio (192000 Hz).
    - AUDIO_SAMPLE_RATE:
        A dictionary that maps media format names ("tv_audio", "cd_audio",
            "dvd_audio", "blu-ray") to their corresponding sample rate tuples.
"""

from typing import Final

_TV_RADIO_VOIP: Final[tuple[int, ...]] = (
    8000,
    16000,
    32000,
)

_CD_AUDIO: Final[tuple[int, ...]] = (
    22050,
    44100,
)

_DVD_AUDIO: Final[tuple[int, ...]] = (
    48000,
    96000,
)

_BLU_RAY_AUDIO: Final[tuple[int, ...]] = (192000,)

AUDIO_SAMPLE_RATE: Final[dict[str, tuple[int, ...]]] = {
    "tv_audio": _TV_RADIO_VOIP,
    "cd_audio": _CD_AUDIO,
    "dvd_audio": _DVD_AUDIO,
    "blu-ray": _BLU_RAY_AUDIO,
}
