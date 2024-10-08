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
