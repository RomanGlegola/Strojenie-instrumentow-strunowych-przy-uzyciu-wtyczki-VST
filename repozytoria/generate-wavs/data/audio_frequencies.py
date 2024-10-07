import numpy as np

# E2, A2, D3, G3, B3, E4
_tuning_standard_a4: tuple[float, ...] = (
    82.41,
    110.0,
    146.83,
    196.0,
    246.94,
    329.63,
)

# E2, F2, F#2/Gb2, G2, G#2/Ab2, A2, A#2/Bb2, B2, C3< C#3/Db3, D3, D#3/Eb3
# A2, A#2/Bb2, B2, C3, C#3/Db3, D3, D#3/Eb3, E3, F3, F#3/Gb3, G3, G#3/Ab3
# D3, D#3/Eb3, E3, F3, F#3/Gb3, G3, G#3/Ab3, A3, A#3/Bb3, B3, C4, C#4/Db4
# G3, G#3/Ab3, A3, A#3/Bb3, B3, C4, C#4/Db4, D4, D#4/Eb4, E4, F4, F#4/Gb4
# B3, C4, C#4/Db4, D4, D#4/Eb4, E4, F4, F#4/Gb4, G4, G#4/Ab4, A4, A#4/Bb4
# E4, F4, F#4/Gb4, G4, G#4/Ab4, A4, A#4/Bb4, B4, C5, C#5/Db5, D5, D#5/Eb5
_tuning_semitones: tuple[float, ...] = tuple(
    np.round(freq * (2 ** (n / 12)), 2)
    for freq in _tuning_standard_a4
    for n in range(12)
)

tuning: dict[str, tuple[float, ...]] = {
    "tuning_standard": _tuning_standard_a4,
    "tuning_semitones": _tuning_semitones,
}
