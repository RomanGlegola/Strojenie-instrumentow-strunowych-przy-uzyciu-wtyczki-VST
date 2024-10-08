from typing import Final

noise_type: Final[dict[str, tuple[str, ...]]] = {
    "none": ("",),
    "white": ("white",),
    "pink": ("pink",),
}

noise_volume: Final[dict[str, tuple[float, ...]]] = {
    "none": (0.0,),
    "low": (0.1,),
    "mid": (0.3,),
    "high": (0.5,),
    "full": (1.0,),
}
