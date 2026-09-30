"""Animal cues from real recordings (CC0 / public domain, a few CC-BY) + a synthesised goldfish
(ART-SOUND, Q-210 route 1). Sources are downloaded once into .run/sound-src (not committed),
cut, pitched (rubberband), softened and processed with process.py.

Run: python3 tools/sound/animals.py            (build everything into assets/audio/animals/)
Every cue is approved = false until a human has listened (nobody listened while building).
Each recipe: src, t0, t1 (seconds in the source), st (semitones, tempo kept), lp (low-pass Hz),
tempo (1 = unchanged), fade_out (s), note (what it is / why).
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
from synthlib import rng_for, t_axis, SR  # noqa: E402

CACHE = os.path.join(process.REPO, ".run", "sound-src")
UA = {"User-Agent": "buchstabenzoo-sound/0.1 (riedermagic@gmail.com)"}
CM = "https://upload.wikimedia.org/wikipedia/commons/"
OGA = "https://opengameart.org/sites/default/files/"

# key -> (download url, member inside zip or None, licence, author, origin page, date retrieved)
SOURCES = {
    "elephant": (CM + "4/40/Elephant_voice_-_trumpeting.ogg", None, "CC0", "Tamil Wikipedia user (Own work, 2011)",
                 "https://commons.wikimedia.org/wiki/File:Elephant_voice_-_trumpeting.ogg"),
    "horse": (CM + "d/db/Wiehern.ogg", None, "public-domain", "Hue (Own work, 2006)",
              "https://commons.wikimedia.org/wiki/File:Wiehern.ogg"),
    "foal": (OGA + "baby-animals_0.zip", "Whinny.ogg", "CC0", "Technopeasant (pack, sources CC0 from Freesound)",
             "https://opengameart.org/content/baby-animals-sounds-pack"),
    "panda": (CM + "b/b8/Giant_panda_twittering.ogg", None, "public-domain", "own work, Wikimedia Commons uploader (2009)",
              "https://commons.wikimedia.org/wiki/File:Giant_panda_twittering.ogg"),
    "lion": (CM + "7/7d/Lion_raring-sound1TamilNadu178.ogg", None, "public-domain",
             "Tamil Wikipedia uploader (lion in a zoo)",
             "https://commons.wikimedia.org/wiki/File:Lion_raring-sound1TamilNadu178.ogg"),
    "fennec": (CM + "5/5f/Fennec_Singing.ogg", None, "public-domain", "Eosin-Y (2007)",
               "https://commons.wikimedia.org/wiki/File:Fennec_Singing.ogg"),
    "owl": (CM + "9/91/Steinkauz.OGG", None, "public-domain", "Rabe19 (Own work, 2013, little owl)",
            "https://commons.wikimedia.org/wiki/File:Steinkauz.OGG"),
    "camel": (OGA + "camel.zip", "ogg/camel_01.ogg", "CC0", "AntumDeluge (from a recording by craigsmith)",
              "https://opengameart.org/content/camel-groan"),
    "bat": (OGA + "bat.zip", "ogg/bat_02.ogg", "CC0", "AntumDeluge (from a recording by polymorpheva)",
            "https://opengameart.org/content/bat-screeches"),
    "bat1": (OGA + "bat.zip", "ogg/bat_01.ogg", "CC0", "AntumDeluge (from a recording by polymorpheva)",
             "https://opengameart.org/content/bat-screeches"),
    "bat3": (OGA + "bat.zip", "ogg/bat_03.ogg", "CC0", "AntumDeluge (from a recording by polymorpheva)",
             "https://opengameart.org/content/bat-screeches"),
    "monkey1": (OGA + "monkey-sounds_0.zip", "monkey-1.ogg", "CC0", "AntumDeluge (human imitation of a monkey)",
                "https://opengameart.org/content/monkey-sounds"),
    "monkey2": (OGA + "monkey-sounds_0.zip", "monkey-2.ogg", "CC0", "AntumDeluge (human imitation of a monkey)",
                "https://opengameart.org/content/monkey-sounds"),
    "monkey3": (OGA + "monkey-sounds_0.zip", "monkey-3.ogg", "CC0", "AntumDeluge (human imitation of a monkey)",
                "https://opengameart.org/content/monkey-sounds"),
    "hedgehog": (CM + "3/3d/Hedgehog_O.ogg", None, "CC0", "Ullus (Own work, 2013, eared hedgehog)",
                 "https://commons.wikimedia.org/wiki/File:Hedgehog_O.ogg"),
    "koala": (CM + "b/be/Perception-of-Male-Caller-Identity-in-Koalas-%28Phascolarctos-cinereus%29-"
              "Acoustic-Analysis-and-pone.0020329.s001.ogv", None, "CC-BY",
              "Charlton, Ellis, McKinnon, Brumm, Nilsson, Fitch, PLoS ONE 2011 (CC BY 2.5), Movie S1",
              "https://commons.wikimedia.org/wiki/File:Perception-of-Male-Caller-Identity-in-Koalas-(Phascolarctos-"
              "cinereus)-Acoustic-Analysis-and-pone.0020329.s001.ogv"),
    "kiwi": (CM + "1/18/Male-ni-brown-kiwi.wav", None, "CC-BY", "Department of Conservation (New Zealand), CC BY 4.0",
             "https://commons.wikimedia.org/wiki/File:Male-ni-brown-kiwi.wav"),
    "raccoon": (OGA + "raccoon_chatter.zip", "ogg/raccoon_chatter.ogg", "CC-BY",
                "jnargus (video), cut by AntumDeluge, CC BY 3.0 or later",
                "https://opengameart.org/content/raccoon-chatter"),
    "raccoon_baby": (OGA + "raccoon_chatter.zip", "ogg/raccoon_chatter_baby_01.ogg", "CC-BY",
                     "jnargus (video), cut by AntumDeluge, CC BY 3.0 or later",
                     "https://opengameart.org/content/raccoon-chatter"),
}

# cue -> recipe. st = pitch shift in semitones (rubberband keeps the length).
CUES = {
    # elephant: trumpet 1.46 s; "happy" = short high toot, "refuse" = low soft snort-like stub
    "animal_elephant_call": dict(src="elephant", t0=0.05, t1=1.45, st=3, lp=6000, fade_out=0.25),
    "animal_elephant_happy": dict(src="elephant", t0=0.05, t1=0.65, st=6, lp=6000, fade_out=0.15),
    "animal_elephant_refuse": dict(src="elephant", t0=0.05, t1=0.40, st=-2, lp=3500, fade_out=0.12),
    # zebra: real horse neigh (zebras are equids), foal whinny for the baby
    "animal_zebra_call": dict(src="horse", t0=0.10, t1=1.90, st=2, lp=7000, fade_out=0.3),
    "animal_zebra_happy": dict(src="horse", t0=0.20, t1=0.85, st=5, lp=7000, fade_out=0.15),
    "animal_zebra_refuse": dict(src="horse", t0=1.10, t1=1.45, st=-1, lp=3500, fade_out=0.12),
    "animal_zebra_baby": dict(src="foal", t0=0.0, t1=1.4, st=2, lp=8000, fade_out=0.25),
    # lion: first roar of the recording, softened and pitched up; happy = short rumble, refuse = tiny grumble
    "animal_lion_call": dict(src="lion", t0=0.55, t1=2.25, st=4, lp=2800, fade_out=0.45),
    "animal_lion_happy": dict(src="lion", t0=0.65, t1=1.25, st=6, lp=2400, fade_out=0.3),
    "animal_lion_refuse": dict(src="lion", t0=0.65, t1=1.00, st=3, lp=2000, fade_out=0.15),
    # panda: real "twittering" (friendly by nature)
    "animal_panda_call": dict(src="panda", t0=0.14, t1=2.2, st=1, lp=8000, fade_out=0.25),
    "animal_panda_happy": dict(src="panda", t0=1.0, t1=1.9, st=3, lp=8000, fade_out=0.15),
    "animal_panda_refuse": dict(src="panda", t0=0.14, t1=0.50, st=-1, lp=6000, fade_out=0.12),
    # monkey: human imitations (OpenGameArt, Stendhal), cut / pitched only
    "animal_monkey_call": dict(src="monkey3", t0=0.0, t1=1.3, st=2, lp=9000, fade_out=0.1),
    "animal_monkey_happy": dict(src="monkey1", t0=0.0, t1=1.0, st=3, lp=9000, fade_out=0.1),
    "animal_monkey_refuse": dict(src="monkey2", t0=0.0, t1=0.45, st=-2, lp=6000, fade_out=0.1),
    # hippo: STAND-IN from a camel groan (no free hippo recording found), low, short, soft
    "animal_hippo_call": dict(src="camel", t0=0.15, t1=1.45, st=-2, lp=3000, fade_out=0.4),
    "animal_hippo_happy": dict(src="camel", t0=0.15, t1=0.75, st=1, lp=3000, fade_out=0.25),
    "animal_hippo_refuse": dict(src="camel", t0=0.15, t1=0.5, st=-4, lp=2500, fade_out=0.15),
    # snow fox + fennec: same recording (fennec "singing", yips), different segments and pitch
    "animal_snow_fox_call": dict(src="fennec", t0=12.78, t1=13.95, st=-3, lp=7000, fade_out=0.25),
    "animal_snow_fox_happy": dict(src="fennec", t0=0.74, t1=1.25, st=-1, lp=7000, fade_out=0.12),
    "animal_snow_fox_refuse": dict(src="fennec", t0=9.08, t1=9.48, st=-4, lp=5000, fade_out=0.1),
    "animal_fennec_call": dict(src="fennec", t0=4.52, t1=5.68, st=2, lp=8000, fade_out=0.25),
    "animal_fennec_happy": dict(src="fennec", t0=12.78, t1=13.40, st=4, lp=8000, fade_out=0.15),
    "animal_fennec_refuse": dict(src="fennec", t0=9.08, t1=9.48, st=0, lp=5000, fade_out=0.1),
    # bat: screeches are very high/harsh: low-passed, pitched down a little
    "animal_bat_call": dict(src="bat", t0=1.0, t1=1.75, st=-3, lp=5000, fade_out=0.2),
    "animal_bat_happy": dict(src="bat1", t0=0.1, t1=0.5, st=-1, lp=5000, fade_out=0.12),
    "animal_bat_refuse": dict(src="bat3", t0=0.0, t1=0.25, st=-3, lp=4000, fade_out=0.1),
    # owl: little owl (Steinkauz) calls
    "animal_owl_call": dict(src="owl", t0=8.00, t1=9.80, st=-2, lp=6000, fade_out=0.3),
    "animal_owl_happy": dict(src="owl", t0=1.30, t1=2.05, st=0, lp=6000, fade_out=0.15),
    "animal_owl_refuse": dict(src="owl", t0=2.52, t1=2.72, st=-3, lp=5000, fade_out=0.08),
    # hedgehog: eared hedgehog voice (Hedgehog_O), three short snippets
    "animal_hedgehog_call": dict(src="hedgehog", t0=13.6, t1=14.35, st=3, lp=8000, fade_out=0.15),
    "animal_hedgehog_happy": dict(src="hedgehog", t0=6.70, t1=7.05, st=5, lp=8000, fade_out=0.1),
    "animal_hedgehog_refuse": dict(src="hedgehog", t0=9.72, t1=10.12, st=0, lp=6000, fade_out=0.1),
    # koala: male bellow from a PLoS ONE video (CC BY 2.5), short pieces, pitched up + softened; baby = high
    "animal_koala_call": dict(src="koala", t0=19.50, t1=20.50, st=5, lp=3500, fade_out=0.3),
    "animal_koala_happy": dict(src="koala", t0=20.75, t1=21.45, st=8, lp=4000, fade_out=0.2),
    "animal_koala_refuse": dict(src="koala", t0=22.10, t1=22.50, st=3, lp=3000, fade_out=0.12),
    "animal_koala_baby": dict(src="koala", t0=20.75, t1=21.35, st=12, lp=6000, fade_out=0.2),
    # kiwi: male call whistle (CC BY 4.0, DOC NZ), 2 short pieces
    "animal_kiwi_call": dict(src="kiwi", t0=5.30, t1=6.42, st=2, lp=8000, fade_out=0.2),
    "animal_kiwi_happy": dict(src="kiwi", t0=5.30, t1=5.85, st=5, lp=8000, fade_out=0.12),
    "animal_kiwi_refuse": dict(src="kiwi", t0=6.42, t1=6.72, st=-2, lp=6000, fade_out=0.1),
    # raccoon: chatter (CC BY)
    "animal_raccoon_call": dict(src="raccoon", t0=0.0, t1=0.98, st=3, lp=9000, fade_out=0.12),
    "animal_raccoon_happy": dict(src="raccoon_baby", t0=0.0, t1=0.6, st=2, lp=9000, fade_out=0.1),
    "animal_raccoon_refuse": dict(src="raccoon", t0=0.0, t1=0.3, st=-2, lp=6000, fade_out=0.08),
}


# Cues that are not a recording of the species itself (shown in the manifest and the brief).
STAND_IN = {c: "stand-in: camel groan, no free hippo recording found" for c in
            ("animal_hippo_call", "animal_hippo_happy", "animal_hippo_refuse")}
STAND_IN.update({c: "human imitation of a monkey, not a recording of a monkey" for c in
                 ("animal_monkey_call", "animal_monkey_happy", "animal_monkey_refuse")})
STAND_IN.update({c: "real horse neigh (zebras are equids); no free zebra recording" for c in
                 ("animal_zebra_call", "animal_zebra_happy", "animal_zebra_refuse")})
STAND_IN["animal_zebra_baby"] = "foal whinny from the CC0 baby animals pack"
STAND_IN["animal_koala_baby"] = "derived: koala bellow piece pitched up 12 semitones"
STAND_IN.update({c: "derived from the fennec recording (no free snow fox recording)" for c in
                 ("animal_snow_fox_call", "animal_snow_fox_happy", "animal_snow_fox_refuse")})


def fetch(key):
    url, member, *_ = SOURCES[key]
    os.makedirs(CACHE, exist_ok=True)
    base = url.split("/")[-1].split("?")[0].replace("%28", "(").replace("%29", ")")
    zpath = os.path.join(CACHE, base)
    if not os.path.exists(zpath):
        print("download", url)
        with urllib.request.urlopen(urllib.request.Request(url, headers=UA), timeout=120) as r:
            open(zpath, "wb").write(r.read())
    if member is None:
        return zpath
    out = os.path.join(CACHE, key + "_" + os.path.basename(member))
    if not os.path.exists(out):
        with zipfile.ZipFile(zpath) as z:
            open(out, "wb").write(z.read(member))
    return out


def load(path, t0, t1):
    r = subprocess.run(["ffmpeg", "-v", "error", "-ss", f"{t0}", "-t", f"{t1 - t0}", "-i", path, "-vn", "-ac", "1",
                        "-ar", str(SR), "-f", "f32le", "-"], capture_output=True, check=True)
    return np.frombuffer(r.stdout, "<f4").astype(float)


def pitch(x, st, tempo=1.0):
    if st == 0 and tempo == 1.0:
        return x
    r = subprocess.run(["ffmpeg", "-v", "error", "-f", "f32le", "-ar", str(SR), "-ac", "1", "-i", "-", "-af",
                        f"rubberband=pitch={2 ** (st / 12):.5f}:tempo={tempo:.4f}", "-f", "f32le", "-"],
                       input=x.astype("<f4").tobytes(), capture_output=True, check=True)
    return np.frombuffer(r.stdout, "<f4").astype(float)


def make(cue, rc):
    x = load(fetch(rc["src"]), rc["t0"], rc["t1"])
    x = x - np.mean(x)
    x = signal.sosfilt(signal.butter(2, 120, "high", fs=SR, output="sos"), x)  # rumble / DC
    x = pitch(x, rc.get("st", 0), rc.get("tempo", 1.0))
    x = signal.sosfilt(signal.butter(3, rc.get("lp", 8000), "low", fs=SR, output="sos"), x)
    n = len(x)
    fi, fo = int(0.012 * SR), int(rc.get("fade_out", 0.1) * SR)
    env = np.ones(n)
    env[:fi] = np.linspace(0, 1, fi) ** 2
    fo = min(fo, n - fi)
    env[n - fo:] = np.cos(np.linspace(0, np.pi / 2, fo)) ** 2  # gentle tail
    return x * env


# ---- goldfish: synthesised bubbles (no real recording needed; friendly "blub") ----
def bloop(f0, f1, dur, rng):
    t = t_axis(dur)
    f = f0 * (f1 / f0) ** (t / dur)
    ph = 2 * np.pi * np.cumsum(f) / SR
    env = np.exp(-t / (dur * 0.35)) * np.minimum(1, t / 0.004)
    return np.sin(ph) * env + 0.25 * np.sin(2 * ph) * env ** 2


def goldfish(cue):
    rng = rng_for(cue, 1)
    if cue == "animal_goldfish_call":  # three rising blubs
        parts = [(0.00, 520, 900, 0.12, 1.0), (0.17, 620, 1100, 0.11, 0.8), (0.32, 720, 1300, 0.10, 0.6)]
        total = 0.6
    elif cue == "animal_goldfish_happy":  # quick happy blub-blub-blub + tiny splash
        parts = [(0.00, 600, 1200, 0.07, 1.0), (0.09, 700, 1400, 0.07, 0.9), (0.18, 800, 1600, 0.07, 0.8),
                 (0.27, 900, 1900, 0.09, 0.7)]
        total = 0.5
    else:  # refuse: one low, soft plop
        parts = [(0.00, 420, 240, 0.14, 1.0)]
        total = 0.25
    out = np.zeros(int(total * SR))
    for t0, a, b, d, g in parts:
        s = bloop(a, b, d, rng)
        i = int(t0 * SR)
        out[i:i + len(s)] += g * s[: len(out) - i]
    if cue == "animal_goldfish_happy":
        nz = rng.standard_normal(int(0.12 * SR))
        nz = signal.sosfilt(signal.butter(2, [2500, 6000], "band", fs=SR, output="sos"), nz)
        out[: len(nz)] += 0.15 * nz * np.exp(-t_axis(0.12) / 0.03)
    return out


GOLDFISH = ["animal_goldfish_call", "animal_goldfish_happy", "animal_goldfish_refuse"]


def build(only=None):
    outdir = os.path.join(process.OUT, "animals")
    for cue, rc in CUES.items():
        if only and cue not in only:
            continue
        y = process.normalise(make(cue, rc))
        process.encode(y, os.path.join(outdir, cue + "_1"))
        print(f"{cue}_1: {len(y)/SR:.2f}s {process.lufs(y):.1f} LUFS peak {process.peak_db(y):.1f} dBFS")
    for cue in GOLDFISH:
        if only and cue not in only:
            continue
        y = process.normalise(goldfish(cue))
        process.encode(y, os.path.join(outdir, cue + "_1"))
        print(f"{cue}_1: {len(y)/SR:.2f}s {process.lufs(y):.1f} LUFS peak {process.peak_db(y):.1f} dBFS")


if __name__ == "__main__":
    build(sys.argv[1:] or None)
