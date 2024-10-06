
_tv_radio_voip: tuple[int, ...] = (8000, 16000, 32000, )

_cd_audio: tuple[int, ...] = (22050, 44100, )

_dvd_audio: tuple[int, ...] = (48000, 96000, )

_blu_ray_audio: tuple[int, ...] = (192000, )

sample_rate: dict[str, tuple[int, ...]] = {
    "tv_audio": _tv_radio_voip,
    "cd_audio": _cd_audio,
    "dvd_audio": _dvd_audio,
    "blu-ray": _blu_ray_audio,
}
