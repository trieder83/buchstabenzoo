"""Door / gate cues (ART-SOUND). Run: python3 tools/sound/doors.py"""
import numpy as np
from synthlib import *


def creak(rng, dur, f0, f1, amp=1.0):
    n = int(dur * SR)
    u = np.linspace(0, 1, n)
    f = f0 + (f1 - f0) * u + 12 * np.sin(2 * np.pi * 7 * u * dur)
    ph = 2 * np.pi * np.cumsum(f) / SR
    y = sum(np.sin(k * ph) / k for k in (1, 2, 3, 4))
    y = bandpass(y, 250, 2200)
    env = np.sin(np.pi * u) ** 0.7
    return amp * y * env


def knock(f, rng, dur=0.14):
    y = tone(f, dur, 0.03) + 0.4 * tone(f * 1.8, dur, 0.02)
    return y + 0.3 * bandpass(noise(rng, len(y)), 800, 2500) * expdecay(len(y), 0.008, 0.0005)


def clink(rng, f=1300, dur=0.3):
    parts = ((1, 1.0), (1.53, 0.6), (2.31, 0.4), (3.1, 0.2))
    return tone(f, dur, 0.06, parts, 0.001)


def door_wood_open(rng, v):
    out = canvas(0.85)
    place(out, knock(160, rng), 0.0, 0.8)     # latch
    place(out, creak(rng, 0.6, 300, 470), 0.1, 0.35)
    return out


def door_wood_close(rng, v):
    out = canvas(0.7)
    place(out, creak(rng, 0.4, 470, 300), 0.0, 0.3)
    place(out, knock(120, rng, 0.2), 0.38, 1.0)
    place(out, knock(200, rng, 0.1), 0.42, 0.4)
    return out


def gate_open(rng, v):
    out = canvas(0.75)
    place(out, clink(rng, 1250), 0.0, 0.8)
    place(out, creak(rng, 0.4, 700, 950), 0.15, 0.25)
    place(out, clink(rng, 1700, 0.15), 0.5, 0.3)
    return out


def gate_close(rng, v):
    out = canvas(0.7)
    place(out, creak(rng, 0.3, 950, 700), 0.0, 0.25)
    place(out, knock(140, rng, 0.15), 0.28, 0.7)
    place(out, clink(rng, 1150), 0.3, 0.9)
    place(out, clink(rng, 1500, 0.2), 0.42, 0.4)
    return out


def glass_door(rng, v):
    out = canvas(1.1)
    n = int(0.5 * SR)
    u = np.linspace(0, 1, n)
    sw = bandpass(noise(rng, n), 1500, 6000) * np.sin(np.pi * u) ** 1.5
    place(out, lowpass(sw, 5000), 0.0, 0.35)
    place(out, bell(1568, 0.6, 0.15), 0.45, 0.5)   # friendly ding-dong
    place(out, bell(1175, 0.6, 0.2), 0.62, 0.5)
    return out


def moon_door(rng, v):
    out = canvas(1.4)
    notes = [523, 659, 784, 1047, 1319]
    for i, f in enumerate(notes):
        place(out, bell(f, 0.9, 0.2), 0.08 * i, 0.4)
    sh = highpass(noise(rng, len(out)), 4000) * np.exp(-np.arange(len(out)) / SR / 0.3)
    out += 0.05 * sh
    return out


CUES = {
    "door_wood_open": ("doors", door_wood_open, 1),
    "door_wood_close": ("doors", door_wood_close, 1),
    "gate_open": ("doors", gate_open, 1),
    "gate_close": ("doors", gate_close, 1),
    "glass_door": ("doors", glass_door, 1),
    "moon_door": ("doors", moon_door, 1),
}

if __name__ == "__main__":
    import process
    process.build(CUES, __file__)
