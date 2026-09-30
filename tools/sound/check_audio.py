"""Decodes every audio file of the manifest and checks ASND-001 (decodes), ASND-003 (loudness
-16 LUFS +/- 2, peak <= -1 dBFS, length <= 1.5 s / animal calls <= 3 s).
Prints one line per problem; exit code 1 if any. Run: python3 tools/sound/check_audio.py"""
import glob
import os
import subprocess
import sys

import numpy as np

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import process  # noqa: E402

REPO = process.REPO


def decode(path):
    r = subprocess.run(["ffmpeg", "-v", "error", "-i", path, "-ac", "1", "-ar", str(process.SR),
                        "-f", "f32le", "-"], capture_output=True)
    if r.returncode != 0 or not r.stdout:
        return None
    return np.frombuffer(r.stdout, "<f4").astype(float)


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
