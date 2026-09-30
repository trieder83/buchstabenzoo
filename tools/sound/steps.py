"""Footstep cues (ART-SOUND): step_path, step_grass, step_sand, step_wood, step_water.
4 seeded variations each; soft and cartoon-like. Run: python3 tools/sound/steps.py"""
import numpy as np
from synthlib import *


def step_path(rng, v):
    n = int(0.16 * SR)
    p = 1 + rng.uniform(-0.12, 0.12)
    tap = bandpass(noise(rng, n), 1200 * p, 3500 * p) * expdecay(n, 0.018, 0.001)
    thump = tone(130 * p, 0.16, 0.03) * 0.9
    return tap * 0.5 + thump


def step_grass(rng, v):
    n = int(0.2 * SR)
    p = 1 + rng.uniform(-0.15, 0.15)
    sw = bandpass(noise(rng, n), 2500 * p, 7500 * p) * expdecay(n, 0.05, 0.012)
    body = lowpass(noise(rng, n), 500) * expdecay(n, 0.03, 0.004) * 0.8
    out = sw + body
    for _ in range(3):  # tiny blade crackles
        c = int(rng.uniform(0.02, 0.12) * SR)
        out[c:c + 60] += rng.standard_normal(60) * 0.25 * np.hanning(60)
    return out


def step_sand(rng, v):
    n = int(0.22 * SR)
    p = 1 + rng.uniform(-0.1, 0.1)
    hiss = bandpass(noise(rng, n), 1800 * p, 5000 * p) * expdecay(n, 0.07, 0.02)
    body = lowpass(noise(rng, n), 350 * p) * expdecay(n, 0.04, 0.006)
    out = 0.7 * hiss + 0.9 * body
    for _ in range(14):  # fine grains
        c = int(rng.uniform(0.01, 0.18) * SR)
        out[c:c + 20] += rng.standard_normal(20) * 0.12 * np.hanning(20)
    return out


def step_wood(rng, v):
    p = 1 + rng.uniform(-0.1, 0.1)
    y = tone(190 * p, 0.2, 0.035) + 0.6 * tone(340 * p, 0.2, 0.025) + 0.3 * tone(610 * p, 0.2, 0.015)
    click = bandpass(noise(rng, len(y)), 900, 2500) * expdecay(len(y), 0.006, 0.0005)
    return y + 0.4 * click


def step_water(rng, v):
    out = canvas(0.3)
    n = len(out)
    sp = bandpass(noise(rng, n), 700, 3500) * expdecay(n, 0.06, 0.006)
    out += 0.6 * sp
    p = 1 + rng.uniform(-0.2, 0.2)
    for k in range(rng.integers(2, 4)):  # little bloops
        at = rng.uniform(0.01, 0.13)
        f0 = rng.uniform(500, 800) * p
        b = sweep(f0, f0 * 1.9, 0.07) * expdecay(int(0.07 * SR), 0.03, 0.004)
        place(out, b, at, 0.55)
    return out


CUES = {
    "step_path": ("steps", step_path, 4),
    "step_grass": ("steps", step_grass, 4),
    "step_sand": ("steps", step_sand, 4),
    "step_wood": ("steps", step_wood, 4),
    "step_water": ("steps", step_water, 4),
}

if __name__ == "__main__":
    import process
    process.build(CUES, __file__)
