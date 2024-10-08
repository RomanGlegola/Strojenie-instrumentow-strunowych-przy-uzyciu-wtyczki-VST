from typing import Final

AUDIO_DC_OFFSET: Final[dict[str, tuple[float, ...]]] = {
    "none": (0.0,),
    "low": (0.5,),
    "medium": (0.5,),
    "high": (1.0,),
}
