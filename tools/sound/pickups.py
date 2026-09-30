"""Pickup / drop cues (ART-SOUND). Run: python3 tools/sound/pickups.py"""
import numpy as np
from synthlib import *


def rustle(rng, dur, lo=2000, hi=6500, tau=0.05):
    n = int(dur * SR)
    return bandpass(noise(rng, n), lo, hi) * expdecay(n, tau, 0.01)


def pickup_food(rng, v):
    out = canvas(0.3)
    place(out, rustle(rng, 0.12), 0.0, 0.35)
    place(out, sweep(380, 900, 0.1) * expdecay(int(0.1 * SR), 0.06), 0.03, 0.8)
    place(out, marimba(1100, 0.15, 0.04), 0.1, 0.3)
    return out


def drop_food(rng, v):
    out = canvas(0.35)
    place(out, sweep(420, 160, 0.1) * expdecay(int(0.1 * SR), 0.05), 0.0, 0.8)
    place(out, tone(110, 0.2, 0.05), 0.07, 0.9)
    place(out, rustle(rng, 0.15, 1500, 5000, 0.04), 0.06, 0.35)
    return out


def pickup_item(rng, v):
    out = canvas(0.3)
    place(out, marimba(520, 0.2, 0.05), 0.0, 0.7)
    place(out, marimba(780, 0.25, 0.07), 0.08, 0.8)
    return out


def drop_item(rng, v):
    out = canvas(0.32)
    place(out, marimba(700, 0.2, 0.05), 0.0, 0.6)
    place(out, marimba(440, 0.25, 0.07), 0.07, 0.6)
    place(out, tone(140, 0.15, 0.03), 0.12, 0.7)
    return out


def harvest_plant(rng, v):
    out = canvas(0.55)
    n = int(0.25 * SR)
    u = np.linspace(0, 1, n)
    sq = bandpass(noise(rng, n), 1200, 3200) * (0.5 + 0.5 * np.sin(2 * np.pi * 14 * u * 0.25)) ** 2 * u
    place(out, sq, 0.0, 0.4)                       # roots rubbing out of the soil
    place(out, sweep(300, 700, 0.06) * expdecay(int(0.06 * SR), 0.04), 0.25, 0.9)   # pop
    place(out, rustle(rng, 0.2, 2500, 7000, 0.07), 0.27, 0.4)   # leaves
    place(out, marimba(988, 0.2, 0.06), 0.33, 0.3)
    return out


def basket_add(rng, v):
    out = canvas(0.5)
    place(out, rustle(rng, 0.15, 1500, 5000, 0.05), 0.0, 0.3)
    place(out, marimba(660, 0.25, 0.07), 0.05, 0.7)
    place(out, marimba(880, 0.3, 0.09), 0.14, 0.8)
    return out


CUES = {
    "pickup_food": ("pickups", pickup_food, 1),
    "drop_food": ("pickups", drop_food, 1),
    "pickup_item": ("pickups", pickup_item, 1),
    "drop_item": ("pickups", drop_item, 1),
    "harvest_plant": ("pickups", harvest_plant, 1),
    "basket_add": ("pickups", basket_add, 1),
}

if __name__ == "__main__":
    import process
    process.build(CUES, __file__)
