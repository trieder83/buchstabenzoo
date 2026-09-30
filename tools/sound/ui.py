"""UI cues (ART-SOUND). Run: python3 tools/sound/ui.py"""
import numpy as np
from synthlib import *


def ui_tap(rng, v):
    out = canvas(0.12)
    place(out, marimba(880, 0.12, 0.03), 0, 1.0)
    place(out, tone(1760, 0.08, 0.015), 0, 0.3)
    return out


def ui_refuse(rng, v):
    # soft, friendly "bwoop-boop" going down (no buzzer)
    out = canvas(0.4)
    place(out, marimba(392, 0.25, 0.08), 0.0, 0.8)
    place(out, marimba(294, 0.3, 0.1), 0.14, 0.8)
    return out


def ui_success(rng, v):
    out = canvas(0.9)
    for i, f in enumerate([523, 659, 784, 1047]):
        place(out, bell(f, 0.6, 0.18), 0.1 * i, 0.5)
    return out


CUES = {
    "ui_tap": ("ui", ui_tap, 1),
    "ui_refuse": ("ui", ui_refuse, 1),
    "ui_success": ("ui", ui_success, 1),
}

if __name__ == "__main__":
    import process
    process.build(CUES, __file__)
