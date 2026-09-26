#!/usr/bin/env python3
"""Split a turnaround sheet (views side by side in one row) into single view images.

Cuts in the middle of the empty background gaps between the figures, so ears, tails and
hair are never clipped. Needs Pillow.

Usage:
  tools/split_sheet.py art/animals/zebra/sheet_v1.jpg front side back three_quarter [--flip side]

Writes <name>.png next to the sheet. --flip mirrors the named views horizontally (e.g. when
the generator drew the side profile facing right instead of left).

--equal is for sheets whose figures overlap horizontally (e.g. a koala's big ear above the
neighbour's rump), where the gap search fails: near each equal-width position (±12 % of a
column) it finds a vertical seam that may bend by 1 px per row through the background
(min-cost path, needs numpy), crops each view between its seams and fills the pixels beyond
a seam with the background colour. Check the result if figures really touch.
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


def seam_split(img, n, threshold):
    """Split into n views along bent background seams; returns [(view, left, right)]."""
    import numpy as np
    arr = np.asarray(img).astype(np.int16)
    h, w, _ = arr.shape
    bg = arr[5, 5]
    fg = (np.abs(arr - bg).max(axis=2) > threshold).astype(np.int64)
    span = int(0.12 * w / n)
    big = 10**12
    seams = [np.zeros(h, int)]
    for i in range(1, n):
        x0 = round(i * w / n) - span
        acc = fg[:, x0:x0 + 2 * span] * 1000 + 1
        for y in range(1, h):  # dynamic programming: cheapest path from the top row
            p = acc[y - 1]
            acc[y] += np.minimum(p, np.minimum(np.r_[p[1:], big], np.r_[big, p[:-1]]))
        path = [int(acc[-1].argmin())]
        for y in range(h - 1, 0, -1):
            x = path[-1]
            path.append(min((acc[y - 1][k], k) for k in (x - 1, x, x + 1) if 0 <= k < 2 * span)[1])
        seams.append(np.array(path[::-1]) + x0)
    seams.append(np.full(h, w))
    views = []
    for left, right in zip(seams, seams[1:]):
        l, r = int(left.min()), int(right.max())
        view = arr[:, l:r].copy()
        xs = np.arange(l, r)[None, :]
        view[(xs < left[:, None]) | (xs >= right[:, None])] = bg
        views.append((Image.fromarray(view.astype(np.uint8)), l, r))
    return views


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("sheet")
    ap.add_argument("names", nargs="+")
    ap.add_argument("--flip", nargs="*", default=[])
    ap.add_argument("--threshold", type=int, default=25)
    ap.add_argument("--equal", action="store_true",
                    help="split near equal-width positions along bent seams (overlapping figures)")
    a = ap.parse_args()

    img = Image.open(a.sheet).convert("RGB")
    w, h = img.size
    n = len(a.names)
    if a.equal:
        views = seam_split(img, n, a.threshold)
    else:
        runs = figure_runs(img, a.threshold)
        if len(runs) != n:
            sys.exit(f"found {len(runs)} figures {runs}, expected {n} — try --threshold or --equal")
        cuts = [0] + [(runs[i][1] + runs[i + 1][0]) // 2 for i in range(len(runs) - 1)] + [w]
        views = [(img.crop((l, 0, r, h)), l, r) for l, r in zip(cuts, cuts[1:])]
    folder = os.path.dirname(a.sheet)
    for name, (view, left, right) in zip(a.names, views):
        if name in a.flip:
            view = ImageOps.mirror(view)
        out = os.path.join(folder, name + ".png")
        view.save(out)
        print("saved", out, f"(x {left}–{right}{', mirrored' if name in a.flip else ''})")


if __name__ == "__main__":
    main()
