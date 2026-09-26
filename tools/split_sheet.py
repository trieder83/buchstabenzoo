#!/usr/bin/env python3
"""Split a turnaround sheet (views side by side in one row) into single view images.

Cuts in the middle of the empty background gaps between the figures, so ears, tails and
hair are never clipped. Needs Pillow.

Usage:
  tools/split_sheet.py art/animals/zebra/sheet_v1.jpg front side back three_quarter [--flip side]

Writes <name>.png next to the sheet. --flip mirrors the named views horizontally (e.g. when
the generator drew the side profile facing right instead of left).
"""
import argparse, os, sys
from PIL import Image, ImageOps


def figure_runs(img, threshold=25, min_width=100):
    grey = img.convert("L")
    w, h = grey.size
    px = grey.load()
    bg = px[5, 5]
    occupied = [any(abs(px[x, y] - bg) > threshold for y in range(0, h, 3)) for x in range(w)]
    runs, x = [], 0
    while x < w:
        if occupied[x]:
            start = x
            while x < w and occupied[x]:
                x += 1
            runs.append((start, x))
        x += 1
    return [r for r in runs if r[1] - r[0] >= min_width]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("sheet")
    ap.add_argument("names", nargs="+")
    ap.add_argument("--flip", nargs="*", default=[])
    ap.add_argument("--threshold", type=int, default=25)
    a = ap.parse_args()

    img = Image.open(a.sheet).convert("RGB")
    runs = figure_runs(img, a.threshold)
    if len(runs) != len(a.names):
        sys.exit(f"found {len(runs)} figures {runs}, expected {len(a.names)} — try --threshold")
    w, h = img.size
    cuts = [0] + [(runs[i][1] + runs[i + 1][0]) // 2 for i in range(len(runs) - 1)] + [w]
    folder = os.path.dirname(a.sheet)
    for name, left, right in zip(a.names, cuts, cuts[1:]):
        view = img.crop((left, 0, right, h))
        if name in a.flip:
            view = ImageOps.mirror(view)
        out = os.path.join(folder, name + ".png")
        view.save(out)
        print("saved", out, f"(x {left}–{right}{', mirrored' if name in a.flip else ''})")


if __name__ == "__main__":
    main()
