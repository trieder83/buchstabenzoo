"""Golf-cart, key-box and lock-panel cues (ART-SOUND "Golf cart sounds", ASND-030..).

All synthesised (numpy/scipy, seeded, licence "own"), friendly and soft; no sample material.
One-shots go through process.build (trim, -16 LUFS, peak <= -1.2 dBFS, mono, .ogg + .m4a).
The two engine loops (`cart_engine_path`, `cart_engine_grass`, group `engine`) are 2.0 s *periodic by
construction* (every partial and every modulation is an integer number of cycles in 2 s, noise is
filtered in the FFT domain = circular), so the loop point is seamless; they are normalised to
-22 LUFS (ungated K-weighted, like the ambient loop) and NOT faded or trimmed.
Run: python3 tools/sound/cart_sounds.py
"""
import os
import sys

import numpy as np

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import process  # noqa: E402
from synthlib import *  # noqa: E402,F403

LOOP_S = 2.0
LOOP_LUFS = -22.0


def fm(f, dur, tau, ratio=1.0, index=1.5, idx_tau=0.05, attack=0.004):
    """Soft FM tone (carrier f, modulator f*ratio, index decaying) - horn / chimes."""
    t = t_axis(dur)
    idx = index * np.exp(-t / idx_tau)
    y = np.sin(2 * np.pi * f * t + idx * np.sin(2 * np.pi * f * ratio * t))
    return y * expdecay(len(t), tau, attack)


def thump(f0=170, f1=60, dur=0.16, tau=0.05):
    return sweep(f0, f1, dur) * expdecay(int(dur * SR), tau, 0.003)


def click(rng, dur=0.02, lo=2000, hi=7000, tau=0.006):
    n = int(dur * SR)
    return bandpass(noise(rng, n), lo, hi) * expdecay(n, tau, 0.0005)


def whirr(f0, f1, dur, level=1.0):
    """Electric start-up / shut-down whine: gliding fundamental + 2nd/3rd partial, soft edges."""
    n = int(dur * SR)
    u = np.linspace(0, 1, n)
    f = f0 + (f1 - f0) * u ** 1.5
    ph = 2 * np.pi * np.cumsum(f) / SR
    y = np.sin(ph) + 0.4 * np.sin(2 * ph) + 0.15 * np.sin(3 * ph)
    env = np.sin(np.pi * np.clip(u, 0, 1)) ** 0.8
    return level * y * env


# ---------------------------------------------------------------- one-shot cues
def cart_board(rng, v):
    out = canvas(0.9)
    place(out, thump(), 0.0, 1.0)                                   # door / seat thump
    place(out, lowpass(noise(rng, int(0.08 * SR)), 900) * expdecay(int(0.08 * SR), 0.03), 0.02, 0.5)
    place(out, click(rng), 0.2, 0.5)                                # key-less "power on" click
    place(out, whirr(160, 520, 0.6, 0.55), 0.24, 1.0)               # electric start whirr
    return out


def cart_get_out(rng, v):
    out = canvas(0.7)
    place(out, whirr(420, 130, 0.32, 0.5), 0.0, 1.0)                # motor winds down
    place(out, click(rng), 0.3, 0.5)
    place(out, thump(150, 55, 0.16, 0.05), 0.36, 0.9)               # door / feet thump
    return out


def cart_horn(rng, v):
    out = canvas(0.62)
    for at in (0.0, 0.26):                                          # friendly "beep-beep"
        n = int(0.17 * SR)
        beep = fm(640, 0.17, 0.12, ratio=1.0, index=1.2, idx_tau=0.04) * np.minimum(1, np.linspace(1, 0, n) * 6)
        beep += 0.3 * np.sin(2 * np.pi * 1280 * t_axis(0.17)) * expdecay(n, 0.05)
        place(out, beep, at, 0.8)
    return out


def cart_bump(rng, v):
    out = canvas(0.3)
    f = (200, 170)[v - 1]
    place(out, sweep(f, f * 0.5, 0.14) * expdecay(int(0.14 * SR), 0.05, 0.002), 0.0, 1.0)   # soft bonk
    place(out, lowpass(noise(rng, int(0.05 * SR)), 1200) * expdecay(int(0.05 * SR), 0.015), 0.0, 0.4)
    place(out, marimba(f * 2.6, 0.15, 0.04), 0.0, 0.25)
    return out


def cart_locked(rng, v):
    out = canvas(0.4)                                               # soft "nope": two dull wooden knocks
    place(out, marimba(330, 0.12, 0.035), 0.0, 0.9)
    place(out, marimba(247, 0.18, 0.05), 0.13, 0.9)
    place(out, click(rng, 0.015, 1500, 4000, 0.004), 0.0, 0.3)
    return out


def cart_park_refuse(rng, v):
    out = canvas(0.34)                                              # "blip-blop" down, no buzzer
    for at, f0, f1 in ((0.0, 720, 600), (0.15, 600, 470)):
        n = int(0.11 * SR)
        place(out, sweep(f0, f1, 0.11) * expdecay(n, 0.05, 0.004), at, 0.9)
    return out


def key_box_open(rng, v):
    out = canvas(0.95)
    place(out, click(rng, 0.03, 2500, 8000, 0.008), 0.0, 1.0)       # metal latch: click ...
    place(out, click(rng, 0.04, 1800, 6000, 0.012), 0.07, 0.8)      # ... clack
    place(out, thump(260, 130, 0.1, 0.03), 0.07, 0.5)               # lid swings open
    place(out, bell(1318, 0.7, 0.22), 0.16, 0.55)                   # soft chime
    place(out, bell(1976, 0.6, 0.16), 0.24, 0.35)
    return out


def key_pickup(rng, v):
    out = canvas(0.75)                                              # key jingle
    at = 0.0
    for f in (2093, 2637, 2349, 3136, 2794, 3520):
        place(out, fm(f, 0.3, 0.07, ratio=2.7, index=0.8, idx_tau=0.03), at, 0.5)
        at += 0.05 + 0.03 * rng.random()
    return out


def lock_wheel_tick(rng, v):
    f = (1700, 1900, 2100)[v - 1]
    out = canvas(0.07)
    place(out, click(rng, 0.02, 1200, 4500, 0.005), 0.0, 0.8)
    place(out, tone(f, 0.05, 0.01), 0.0, 0.4)
    return out


def lock_wrong(rng, v):
    out = canvas(0.45)                                              # gentle "bu-bwoh", rounded, not a buzzer
    for at, f in ((0.0, 230), (0.17, 190)):
        n = int(0.22 * SR)
        t = t_axis(0.22)
        y = lowpass(np.sin(2 * np.pi * f * t) + 0.35 * np.sin(2 * np.pi * 2 * f * t)
                    + 0.15 * np.sin(2 * np.pi * 3 * f * t), 900)
        y *= np.sin(np.pi * np.linspace(0, 1, n)) ** 0.7
        place(out, y, at, 0.9)
    return out


def lock_ok(rng, v):
    out = canvas(1.0)                                               # happy sparkle up
    for i, f in enumerate((784, 988, 1175, 1568)):
        place(out, bell(f, 0.7, 0.2), 0.09 * i, 0.5)
    place(out, fm(3136, 0.4, 0.12, ratio=2.0, index=0.5, idx_tau=0.05), 0.36, 0.25)
    return out


CUES = {
    "cart_board": ("cart", cart_board, 1),
    "cart_get_out": ("cart", cart_get_out, 1),
    "cart_horn": ("cart", cart_horn, 1),
    "cart_bump": ("cart", cart_bump, 2),
    "cart_locked": ("cart", cart_locked, 1),
    "cart_park_refuse": ("cart", cart_park_refuse, 1),
    "key_box_open": ("cart", key_box_open, 1),
    "key_pickup": ("cart", key_pickup, 1),
    "lock_wheel_tick": ("cart", lock_wheel_tick, 3),
    "lock_wrong": ("cart", lock_wrong, 1),
    "lock_ok": ("cart", lock_ok, 1),
}

# ---------------------------------------------------------------- engine loops (periodic, 2.0 s)
LOOPS = {"cart_engine_path": ("engine", None, 1), "cart_engine_grass": ("engine", None, 1)}


def periodic_noise(rng, lo, hi, n):
    """White noise band-limited in the FFT domain: exactly periodic in n samples."""
    spec = np.fft.rfft(rng.standard_normal(n))
    f = np.fft.rfftfreq(n, 1 / SR)
    spec *= ((f >= lo) & (f <= hi)) / np.maximum(1.0, f / 1000.0) ** 0.5
    return np.fft.irfft(spec, n)


def engine(kind):
    rng = np.random.default_rng(4711 if kind == "path" else 4712)
    n = int(LOOP_S * SR)
    t = np.arange(n) / SR
    grass = kind == "grass"
    f0 = 74.0 if grass else 90.0                        # integer Hz -> integer cycles in 2 s
    vib = 1.0 * np.sin(2 * np.pi * 1.0 * t)             # 1 Hz wobble, 1 cycle per... 2 loops, ok
    ph = 2 * np.pi * f0 * t + 0.25 * vib
    hum = (np.sin(ph) + 0.55 * np.sin(2 * ph) + 0.3 * np.sin(3 * ph) + 0.12 * np.sin(4 * ph))
    am = 1 + 0.12 * np.sin(2 * np.pi * 3.0 * t)         # gentle motor flutter
    whine_f = 420.0 if grass else 600.0
    whine = (0.10 if grass else 0.16) * (np.sin(2 * np.pi * whine_f * t)
                                          + 0.4 * np.sin(2 * np.pi * 2 * whine_f * t)) * (1 + 0.3 * np.sin(2 * np.pi * 2.0 * t))
    if grass:   # swishing grass: low-passed noise rolling at 5 Hz
        roll = 0.6 + 0.4 * np.sin(2 * np.pi * 5.0 * t + 1.0)
        tex = 0.55 * periodic_noise(rng, 250, 1800, n) * roll
        tex /= np.std(tex) * 5
    else:       # tyres on a path: fine gravel crackle, 9 Hz roll
        roll = 0.7 + 0.3 * np.sin(2 * np.pi * 9.0 * t)
        grit = periodic_noise(rng, 600, 4500, n)
        tex = grit * roll * (0.5 + 0.5 * (periodic_noise(rng, 20, 400, n) > 0))
        tex /= np.std(tex) * 5
    y = hum * am * 0.7 + whine + tex
    y = lowpass(np.concatenate([y, y, y]), 3500 if grass else 6000)[n:2 * n]   # filter on a tiled copy keeps periodicity
    y -= y.mean()
    # pure-numpy loudness set (K-weighting is linear, so scale by the dB difference)
    y *= 10 ** ((LOOP_LUFS - process.lufs(y)) / 20)
    ceiling = 10 ** (-1.5 / 20)
    assert np.max(np.abs(y)) < ceiling, "loop peak too high"
    return y


def _encode(y, base):
    import subprocess
    wav = base + ".tmp.wav"
    process.write_wav(wav, y)
    for args, ext in ((["-c:a", "libvorbis", "-q:a", "2"], "ogg"), (["-c:a", "aac", "-b:a", "48k"], "m4a")):
        subprocess.run(["ffmpeg", "-v", "error", "-y", "-i", wav, "-ac", "1", *args, "-map_metadata", "-1",
                        "-fflags", "+bitexact", f"{base}.{ext}"], check=True)
    os.remove(wav)


def _decoded_jump(base):
    """Worst seam jump / largest step of the decoded .ogg and .m4a (what the checker sees)."""
    import check_audio
    worst = 0.0
    for ext in ("ogg", "m4a"):
        x = check_audio.decode(f"{base}.{ext}")
        worst = max(worst, check_audio.loop_seam(x)[0])
    return worst


def build_loops():
    for cue, (group, _f, _v) in LOOPS.items():
        y0 = engine(cue.split("_")[-1])
        base = os.path.join(process.OUT, group, f"{cue}_1")
        os.makedirs(os.path.dirname(base), exist_ok=True)
        # the signal is periodic, so the loop may start anywhere: pick the start whose *decoded*
        # seam is smoothest (lossy codecs add a little error at the file edges)
        best = None
        for k in range(0, 20):
            y = np.roll(y0, -k * 2205)
            _encode(y, base)
            j = _decoded_jump(base)
            if best is None or j < best[0]:
                best = (j, k)
            if j <= 0.6:
                break
        if best[1] != k:
            y = np.roll(y0, -best[1] * 2205)
            _encode(y, base)
        step = np.abs(np.diff(y)).max()
        print(f"{cue}_1: {len(y)/SR:.2f}s {process.lufs(y):.1f} LUFS peak {process.peak_db(y):.1f} dBFS "
              f"seam (decoded worst) jump/max-step {_decoded_jump(base):.2f} (start offset {best[1]*50} ms)")


if __name__ == "__main__":
    process.build(CUES, __file__)
    build_loops()
