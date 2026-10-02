"""Night cricket loop `ambient_crickets` (ART-SOUND "Ambient loops", ASND-020..).

Source: OpenGameArt "Crickets Ambient Noise - loopable" by Ted Kerr, CC0
(https://opengameart.org/content/crickets-ambient-noise-loopable, 11.5 s, stereo mp3). Cleanup:
mono, high-pass 1 kHz (4th order, zero phase), then two passes of the 11.5 s source (the second
one rotated by 37 % and re-sampled by +3 % so the pattern does not repeat exactly) joined with
0.8 s equal-power crossfades, and a wrap-around crossfade of the loop point (seam check:
`seam_db`). Loudness -22 LUFS (ungated K-weighted): a quiet bed far below the -16 LUFS cues.
Output: assets/audio/ambient/ambient_crickets_1.{ogg,m4a}. Run: python3 tools/sound/crickets.py
"""
import os
import subprocess
import sys
import urllib.request

import numpy as np
from scipy import signal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import process  # noqa: E402

SR = process.SR
URL = "https://opengameart.org/sites/default/files/crickets_1.mp3"
SRC = os.path.join(process.REPO, ".run", "sound-src", "crickets", "oga_crickets_1.mp3")
CUES = {"ambient_crickets": ("ambient", None, 1)}  # id -> (group, -, variants); read by register.py
TARGET_LUFS = -22.0
XFADE_S = 0.8


def source():
    if not os.path.exists(SRC):
        os.makedirs(os.path.dirname(SRC), exist_ok=True)
        req = urllib.request.Request(URL, headers={"User-Agent": "buchstabenzoo-sound/1.0"})
        open(SRC, "wb").write(urllib.request.urlopen(req, timeout=60).read())
    r = subprocess.run(["ffmpeg", "-v", "error", "-i", SRC, "-ac", "1", "-ar", str(SR), "-f", "f32le", "-"],
                       capture_output=True, check=True)
    return np.frombuffer(r.stdout, "<f4").astype(float)


def crossfade(a, b, n):
    t = np.linspace(0, np.pi / 2, n)
    return np.concatenate([a[:-n], a[-n:] * np.cos(t) + b[:n] * np.sin(t), b[n:]])


def build():
    x = source()
    x = signal.sosfiltfilt(signal.butter(4, 1000, "highpass", fs=SR, output="sos"), x)
    x -= x.mean()
    a = x
    b = np.roll(signal.resample(x, int(len(x) / 1.03)), int(len(x) * 0.37))
    n = int(XFADE_S * SR)
    seq = crossfade(a, b, n)
    # wrap-around: the loop point is a crossfade of the end into the start
    seq = np.concatenate([seq, a[:n]])  # extra material to blend into the first n samples
    body = seq[:-n].copy()
    t = np.linspace(0, np.pi / 2, n)
    body[:n] = seq[:n] * np.sin(t) + seq[-n:] * np.cos(t)
    y = body
    y *= 10 ** ((TARGET_LUFS - process.lufs(y)) / 20)
    return y


def seam_stats(y):
    """(jump / largest normal step, level around the seam - file level in dB), see check_audio.loop_seam."""
    step = np.abs(np.diff(y))
    w = int(0.05 * SR)
    near = np.concatenate([y[-w:], y[:w]])
    return abs(y[0] - y[-1]) / step.max(), 20 * np.log10(np.sqrt(np.mean(near ** 2)) / np.sqrt(np.mean(y ** 2)))


def encode(y, base):
    """Small files: low-pass 9 kHz (the chirps sit at 3-6 kHz), Vorbis q0, AAC 32 kbit."""
    os.makedirs(os.path.dirname(base), exist_ok=True)
    wav = base + ".tmp.wav"
    process.write_wav(wav, signal.sosfiltfilt(signal.butter(4, 9000, "lowpass", fs=SR, output="sos"), y))
    for args, ext in ((["-c:a", "libvorbis", "-q:a", "0"], "ogg"), (["-c:a", "aac", "-b:a", "32k"], "m4a")):
        subprocess.run(["ffmpeg", "-v", "error", "-y", "-i", wav, "-ac", "1", *args, "-map_metadata", "-1",
                        "-fflags", "+bitexact", f"{base}.{ext}"], check=True)
    os.remove(wav)


def main():
    y = build()
    encode(y, os.path.join(process.OUT, "ambient", "ambient_crickets_1"))
    print(f"ambient_crickets_1: {len(y)/SR:.1f}s {process.lufs(y):.1f} LUFS peak {process.peak_db(y):.1f} dBFS "
          f"seam jump/max-step {seam_stats(y)[0]:.2f}, level at seam {seam_stats(y)[1]:+.2f} dB")


if __name__ == "__main__":
    main()
