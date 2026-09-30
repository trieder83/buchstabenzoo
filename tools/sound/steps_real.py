"""Footstep cues from real recordings (CC0) and Runway-generated water splashes (ART-SOUND).
Replaces the synthesised step_path / step_grass / step_sand / step_wood; step_water lives
in steps.py (synth) only as long as no generated take is used (see WATER below).

Sources (downloaded once into .run/sound-src/steps, not committed):
  Fantozzi "footsteps" pack (OpenGameArt, CC0, from Freesound): StoneL/R1-3 -> step_path,
      SandL/R1-3 -> step_grass ("sand sounds like grass too", author's note)
  TinyWorlds "Different steps" (OpenGameArt, CC0): wood01-03 -> step_wood, gravel + mud02 -> step_sand
  Runway eleven_text_to_sound_v2 (generated, prompts in art/sound/brief.md): water_a/b -> step_water
Processing per take: cut at the onset, pitch up with a resample (+2..+3.5 semitones = lighter,
smaller, slightly quicker step), low-pass (soft, not crunchy), fade out within max_len (single
footfall 0.15-0.3 s), then process.normalise (-16 LUFS, peak <= -1.2 dBFS), mono, ogg + m4a.
Run: python3 tools/sound/steps_real.py   (downloads on first run)
"""
import io
import os
import subprocess
import sys
import urllib.request
import zipfile

import numpy as np
from scipy import signal

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import process  # noqa: E402

SR = process.SR
CACHE = os.path.join(process.REPO, ".run", "sound-src", "steps")
UA = {"User-Agent": "Mozilla/5.0 (buchstabenzoo-sound riedermagic@gmail.com)"}
OGA = "https://opengameart.org/sites/default/files/"

SOURCES = {
    "fantozzi": (OGA + "Fantozzi-footsteps.7z", "CC0", "Fantozzi (Freesound pack 10338, uploaded to OpenGameArt by qubodup)",
                 "https://opengameart.org/content/fantozzis-footsteps-grasssand-stone"),
    "kdd": (OGA + "%5Bkdd%5DDifferentSteps_0.zip", "CC0", "TinyWorlds",
            "https://opengameart.org/content/different-steps-on-wood-stone-leaves-gravel-and-mud"),
    "runway": (None, "generated", "Runway API, model eleven_text_to_sound_v2 (output usable commercially per Runway terms, Q-250)",
               "https://docs.dev.runwayml.com/api#tag/Start-generating/paths/~1v1~1sound_effect/post"),
}
RUNWAY_DIR = os.path.join(process.REPO, ".run", "sound-src", "runway")
RUNWAY_PROMPTS = {
    "water_a": ("Single soft footstep of a child splashing in shallow water, small gentle splash, dry, close microphone", 0.6),
    "water_b": ("Gentle small splash of a bare child foot stepping in a shallow puddle, single step, cartoon-soft", 0.6),
}


def src_path(key, member):
    if key == "fantozzi":
        return os.path.join(CACHE, "fant", "Fantozzi-footsteps", "flac", member + ".flac")
    if key == "kdd":
        return os.path.join(CACHE, "kdd", member + ".ogg")
    return os.path.join(RUNWAY_DIR, member + ".mp3")


def fetch():
    os.makedirs(CACHE, exist_ok=True)
    f = os.path.join(CACHE, "fantozzi.7z")
    if not os.path.exists(f):
        open(f, "wb").write(urllib.request.urlopen(urllib.request.Request(SOURCES["fantozzi"][0], headers=UA)).read())
        subprocess.run(["7z", "x", "-y", "-o" + os.path.join(CACHE, "fant"), f], check=True, capture_output=True)
    z = os.path.join(CACHE, "kdd.zip")
    if not os.path.exists(z):
        open(z, "wb").write(urllib.request.urlopen(urllib.request.Request(SOURCES["kdd"][0], headers=UA)).read())
        zipfile.ZipFile(z).extractall(os.path.join(CACHE, "kdd"))


def load(path):
    raw = subprocess.run(["ffmpeg", "-v", "error", "-i", path, "-ac", "1", "-ar", str(SR), "-f", "f32le", "-"],
                         capture_output=True, check=True).stdout
    return np.frombuffer(raw, "<f4").astype(float)


# cue -> list of recipes (one per variation): src, member, st (semitones), lp (Hz), hp (Hz), max_len (s),
# t0 (s, optional start), gain_tail (fade-out start fraction)
def R(src, member, st, lp, hp=90, max_len=0.28, t0=0.0, fade_from=0.45):
    return dict(src=src, member=member, st=st, lp=lp, hp=hp, max_len=max_len, t0=t0, fade_from=fade_from)


CUES = {
    "step_path": [R("fantozzi", "Fantozzi-StoneL1", 2.5, 4500), R("fantozzi", "Fantozzi-StoneR1", 3.0, 4200),
                  R("fantozzi", "Fantozzi-StoneL2", 2.0, 4800), R("fantozzi", "Fantozzi-StoneR2", 3.5, 4300),
                  R("fantozzi", "Fantozzi-StoneL3", 2.8, 4500), R("fantozzi", "Fantozzi-StoneR3", 2.2, 4600)],
    "step_grass": [R("fantozzi", "Fantozzi-SandL1", 2.5, 3800, max_len=0.3), R("fantozzi", "Fantozzi-SandR1", 3.0, 3600, max_len=0.3),
                   R("fantozzi", "Fantozzi-SandL2", 2.0, 4000, max_len=0.3), R("fantozzi", "Fantozzi-SandR2", 3.5, 3700, max_len=0.3),
                   R("fantozzi", "Fantozzi-SandL3", 2.8, 3800, max_len=0.3), R("fantozzi", "Fantozzi-SandR3", 2.2, 3900, max_len=0.3)],
    "step_sand": [R("kdd", "gravel", 3.0, 3000, max_len=0.32), R("kdd", "gravel", 4.5, 2800, max_len=0.32),
                  R("kdd", "mud02", 2.0, 3200, max_len=0.26), R("fantozzi", "Fantozzi-SandR2", 1.0, 2200, max_len=0.3)],
    "step_wood": [R("kdd", "wood01", 3.0, 2800, hp=110, max_len=0.25), R("kdd", "wood02", 3.5, 2800, hp=110, max_len=0.25),
                  R("kdd", "wood03", 3.0, 2800, hp=110, max_len=0.28), R("kdd", "wood03", 5.0, 2800, hp=110, max_len=0.25)],
    "step_water": [R("runway", "water_a", 1.0, 5000, max_len=0.35), R("runway", "water_b", 1.0, 5000, max_len=0.35),
                   R("runway", "water_a", 3.0, 5000, max_len=0.35), R("runway", "water_b", 3.0, 5000, max_len=0.35)],
}


def make(rc):
    x = load(src_path(rc["src"], rc["member"]))
    env = np.abs(x)
    start = max(int(rc["t0"] * SR), 0)
    on = start + int(np.argmax(env[start:] > env[start:].max() * 0.12))
    x = x[max(on - int(0.004 * SR), 0):]
    r = 2 ** (rc["st"] / 12)
    x = signal.resample_poly(x, 1000, int(round(1000 * r)))
    x = signal.sosfilt(signal.butter(2, rc["hp"], "hp", fs=SR, output="sos"), x)
    x = signal.sosfilt(signal.butter(2, rc["lp"], "lp", fs=SR, output="sos"), x)
    n = min(len(x), int(rc["max_len"] * SR))
    x = x[:n].copy()
    f0 = int(n * rc["fade_from"])
    x[f0:] *= np.cos(np.linspace(0, np.pi / 2, n - f0)) ** 2  # smooth tail, no long ring
    return x


def build():
    fetch()
    for cue, recipes in CUES.items():
        for i, rc in enumerate(recipes, 1):
            x = process.normalise(make(rc))
            process.encode(x, os.path.join(process.OUT, "steps", f"{cue}_{i}"))
            print(f"{cue}_{i}: {len(x)/SR:.2f}s {process.lufs(x):.1f} LUFS peak {process.peak_db(x):.1f} dBFS")


if __name__ == "__main__":
    build()
