"""Small synthesis helpers for the Buchstabenzoo sound cues (ART-SOUND, numpy/scipy only).

All randomness comes from a seeded numpy Generator, so every run gives identical audio.
"""
import numpy as np
from scipy import signal

SR = 44100


def t_axis(dur):
    return np.arange(int(dur * SR)) / SR


def rng_for(cue, variant):
    seed = sum((i + 1) * ord(c) for i, c in enumerate(cue)) * 131 + variant * 7919
    return np.random.default_rng(seed)


def expdecay(n, tau, attack=0.002):
    t = np.arange(n) / SR
    env = np.exp(-t / tau)
    a = max(1, int(attack * SR))
    env[:a] *= np.linspace(0, 1, a)
    return env


def noise(rng, n):
    return rng.standard_normal(n)


def bandpass(x, lo, hi, order=2):
    hi = min(hi, SR / 2 - 100)
    sos = signal.butter(order, [lo, hi], btype="band", fs=SR, output="sos")
    return signal.sosfilt(sos, x)


def lowpass(x, f, order=2):
    return signal.sosfilt(signal.butter(order, f, btype="low", fs=SR, output="sos"), x)


def highpass(x, f, order=2):
    return signal.sosfilt(signal.butter(order, f, btype="high", fs=SR, output="sos"), x)


def sweep(f0, f1, dur, shape="exp"):
    """Sine with gliding frequency."""
    n = int(dur * SR)
    u = np.linspace(0, 1, n)
    f = f0 * (f1 / f0) ** u if shape == "exp" else f0 + (f1 - f0) * u
    return np.sin(2 * np.pi * np.cumsum(f) / SR)


def tone(f, dur, tau, partials=((1, 1.0),), attack=0.003):
    t = t_axis(dur)
    y = sum(a * np.sin(2 * np.pi * f * k * t) for k, a in partials)
    return y * expdecay(len(t), tau, attack)


def marimba(f, dur=0.35, tau=0.09):
    """Soft mallet tone: fundamental plus a quickly dying 4th-harmonic partial."""
    t = t_axis(dur)
    y = np.sin(2 * np.pi * f * t) * np.exp(-t / tau)
    y += 0.35 * np.sin(2 * np.pi * f * 3.9 * t) * np.exp(-t / (tau * 0.25))
    a = int(0.002 * SR)
    y[:a] *= np.linspace(0, 1, a)
    return y


def bell(f, dur=0.8, tau=0.25):
    t = t_axis(dur)
    parts = [(1, 1.0), (2.76, 0.35), (5.4, 0.15)]
    y = sum(a * np.sin(2 * np.pi * f * k * t) * np.exp(-t / (tau / (1 + 0.6 * i)))
            for i, (k, a) in enumerate(parts))
    a = int(0.003 * SR)
    y[:a] *= np.linspace(0, 1, a)
    return y


def place(buf, sig, at, gain=1.0):
    i = int(at * SR)
    end = min(len(buf), i + len(sig))
    if end > i:
        buf[i:end] += gain * sig[: end - i]


def canvas(dur):
    return np.zeros(int(dur * SR))


def soft_clip(x, ceiling=0.85):
    return ceiling * np.tanh(x / ceiling)
