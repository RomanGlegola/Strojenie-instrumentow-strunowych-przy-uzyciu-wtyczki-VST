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
