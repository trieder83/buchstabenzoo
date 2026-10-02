"""Decodes every audio file of the manifest and checks ASND-001 (decodes), ASND-003 (loudness
-16 LUFS +/- 2, peak <= -1 dBFS, length <= 1.5 s / animal calls <= 3 s). Group `ambient` (loops,
ASND-020/021): -22 LUFS +/- 2, 20-40 s, ogg <= 150 KB, spectral peak 3-6 kHz, seamless loop point.
Prints one line per problem; exit code 1 if any. Run: python3 tools/sound/check_audio.py"""
import glob
import os
import subprocess
import sys

import numpy as np
from scipy import signal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import process  # noqa: E402

REPO = process.REPO


def decode(path):
    r = subprocess.run(["ffmpeg", "-v", "error", "-i", path, "-ac", "1", "-ar", str(process.SR),
                        "-f", "f32le", "-"], capture_output=True)
    if r.returncode != 0 or not r.stdout:
        return None
    return np.frombuffer(r.stdout, "<f4").astype(float)


def loop_seam(x):
    """(jump at the loop point / largest normal step, level of +-50 ms around the seam - file level in dB)."""
    step = np.abs(np.diff(x))
    w = int(0.05 * process.SR)
    near = np.concatenate([x[-w:], x[:w]])
    rms = lambda v: np.sqrt(np.mean(v * v))
    return abs(x[0] - x[-1]) / step.max(), 20 * np.log10(rms(near) / rms(x))


def spectral_peak_hz(x):
    f, p = signal.welch(x, process.SR, nperseg=4096)
    return float(f[np.argmax(p)])


def check_ambient(path, x):
    """ASND-020/021: a quiet seamless background loop (not a one-shot cue)."""
    out = []
    dur = len(x) / process.SR
    if not 20.0 <= dur <= 40.0:
        out.append(f"{path}: loop {dur:.1f}s not in 20-40 s")
    l, p = process.lufs(x), process.peak_db(x)
    if not -24.0 <= l <= -20.0:
        out.append(f"{path}: {l:.1f} LUFS not in -22 +/- 2")
    if p > -1.0:
        out.append(f"{path}: peak {p:.1f} dBFS > -1")
    if path.endswith(".ogg") and os.path.getsize(path) > 150_000:
        out.append(f"{path}: {os.path.getsize(path)} bytes > 150 KB")
    if os.path.basename(path).startswith("ambient_crickets"):
        hz = spectral_peak_hz(x)
        if not 3000 <= hz <= 6000:
            out.append(f"{path}: spectral peak {hz:.0f} Hz not in 3-6 kHz")
    jump, lvl = loop_seam(x)
    if jump > 1.0:
        out.append(f"{path}: loop seam click (jump {jump:.2f} x the largest normal step)")
    if abs(lvl) > 3.0:
        out.append(f"{path}: level at the loop seam {lvl:+.1f} dB (> 3 dB)")
    return out


def main():
    problems, n = [], 0
    files = sorted(glob.glob(os.path.join(REPO, "assets", "audio", "*", "*.ogg")) +
                   glob.glob(os.path.join(REPO, "assets", "audio", "*", "*.m4a")))
    for path in files:
        n += 1
        x = decode(path)
        if x is None:
            problems.append(f"{path}: does not decode")
            continue
        if "/ambient/" in path:
            problems += check_ambient(path, x)
            continue
        dur = len(x) / process.SR
        limit = 3.05 if os.path.basename(path).startswith("animal_") else 1.55
        if dur > limit:
            problems.append(f"{path}: {dur:.2f}s > {limit}s")
        l, p = process.lufs(x), process.peak_db(x)
        if not -18.0 <= l <= -14.0:
            problems.append(f"{path}: {l:.1f} LUFS not in -16 +/- 2")
        if p > -1.0:
            problems.append(f"{path}: peak {p:.1f} dBFS > -1")
    stray = [f for f in glob.glob(os.path.join(REPO, "assets", "audio", "*", "*.ogg"))
             if not os.path.exists(f[:-4] + ".m4a")]
    problems += [f"{f}: no .m4a twin" for f in stray]
    print("\n".join(problems) if problems else f"ok: {n} files")
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
