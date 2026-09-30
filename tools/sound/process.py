"""Processing + encoding for all sound cues (ART-SOUND "Processing and format").

Trim silence, fade 5 ms in/out, normalise to -16 LUFS (K-weighted, ungated - short cues are
below the 400 ms gating block), soft-limit so peaks stay <= -1 dBFS, mono, then encode
.ogg (Vorbis) + .m4a (AAC fallback) with ffmpeg into assets/audio/<group>/<cue>_<n>.<ext>.

Library use:  process.build(CUES, __file__)   (CUES: name -> (group, fn(rng, variant), variants))
CLI:          python3 tools/sound/process.py in.wav group cue_name_1   (external / generated audio)
"""
import os
import subprocess
import sys
import tempfile
import wave

import numpy as np
from scipy import signal

SR = 44100
TARGET = -16.0
PEAK_MAX_DB = -1.2
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
OUT = os.path.join(REPO, "assets", "audio")


def _kw_sos(sr):
    # BS.1770 K-weighting (shelf + RLB high-pass), coefficients for 48 kHz, designed via bilinear
    # transform at other rates.
    f0, G, Q = 1681.974450955533, 3.999843853973347, 0.7071752369554196
    K = np.tan(np.pi * f0 / sr)
    Vh = 10 ** (G / 20)
    Vb = Vh ** 0.4996667741545416
    a0 = 1 + K / Q + K * K
    b = [(Vh + Vb * K / Q + K * K) / a0, 2 * (K * K - Vh) / a0, (Vh - Vb * K / Q + K * K) / a0]
    a = [1, 2 * (K * K - 1) / a0, (1 - K / Q + K * K) / a0]
    f1, Q1 = 38.13547087602444, 0.5003270373238773
    K = np.tan(np.pi * f1 / sr)
    a0 = 1 + K / Q1 + K * K
    b2 = [1, -2, 1]
    a2 = [1, 2 * (K * K - 1) / a0, (1 - K / Q1 + K * K) / a0]
    return signal.tf2sos(b, a), signal.tf2sos(b2, a2)


def lufs(x, sr=SR):
    """Ungated integrated loudness of a mono signal (BS.1770 K-weighting)."""
    s1, s2 = _kw_sos(sr)
    y = signal.sosfilt(s2, signal.sosfilt(s1, x))
    ms = np.mean(y * y)
    return -0.691 + 10 * np.log10(ms + 1e-12)


def peak_db(x):
    return 20 * np.log10(np.max(np.abs(x)) + 1e-12)


def trim(x, thresh_db=-50.0):
    pk = np.max(np.abs(x))
    idx = np.where(np.abs(x) > pk * 10 ** (thresh_db / 20))[0]
    return x[idx[0]: idx[-1] + 1]


def fade(x, ms=5):
    n = int(ms / 1000 * SR)
    x = x.copy()
    x[:n] *= np.linspace(0, 1, n)
    x[-n:] *= np.linspace(1, 0, n)
    return x


def normalise(x):
    x = fade(trim(x - np.mean(x)))
    ceiling = 10 ** ((PEAK_MAX_DB - 0.3) / 20)
    g = 10 ** ((TARGET - lufs(x)) / 20)
    y = x * g
    for _ in range(12):
        # soft limiter: tanh only bends the peaks, then re-aim at the target
        y = ceiling * np.tanh(y / ceiling)
        d = TARGET - lufs(y)
        if abs(d) < 0.15:
            break
        y = y * 10 ** (d / 20)
    y = np.clip(y, -ceiling, ceiling)
    return fade(y, 3)


def write_wav(path, x):
    with wave.open(path, "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(SR)
        w.writeframes((np.clip(x, -1, 1) * 32767).astype("<i2").tobytes())


def encode(x, base):
    """Writes base.ogg and base.m4a (mono, 44.1 kHz)."""
    os.makedirs(os.path.dirname(base), exist_ok=True)
    with tempfile.TemporaryDirectory() as td:
        wav = os.path.join(td, "in.wav")
        write_wav(wav, x)
        subprocess.run(["ffmpeg", "-v", "error", "-y", "-i", wav, "-ac", "1", "-c:a", "libvorbis",
                        "-q:a", "2", "-map_metadata", "-1", "-fflags", "+bitexact", base + ".ogg"], check=True)
        subprocess.run(["ffmpeg", "-v", "error", "-y", "-i", wav, "-ac", "1", "-c:a", "aac",
                        "-b:a", "48k", "-map_metadata", "-1", "-fflags", "+bitexact", base + ".m4a"], check=True)


def build(cues, script_path):
    sys.path.insert(0, os.path.dirname(script_path))
    from synthlib import rng_for
    for cue, (group, fn, variants) in cues.items():
        for v in range(1, variants + 1):
            x = normalise(fn(rng_for(cue, v), v))
            encode(x, os.path.join(OUT, group, f"{cue}_{v}"))
            print(f"{cue}_{v}: {len(x)/SR:.2f}s  {lufs(x):.1f} LUFS  peak {peak_db(x):.1f} dBFS")


def read_wav(path):
    with wave.open(path, "rb") as w:
        assert w.getsampwidth() == 2
        d = np.frombuffer(w.readframes(w.getnframes()), "<i2").astype(float) / 32768
        return d.reshape(-1, w.getnchannels()).mean(axis=1), w.getframerate()


if __name__ == "__main__":
    if len(sys.argv) != 4:
        sys.exit(__doc__)
    src, group, name = sys.argv[1:]
    data, sr = read_wav(src)
    if sr != SR:
        data = signal.resample_poly(data, SR, sr)
    y = normalise(data)
    encode(y, os.path.join(OUT, group, name))
    print(f"{name}: {len(y)/SR:.2f}s {lufs(y):.1f} LUFS peak {peak_db(y):.1f} dBFS")
