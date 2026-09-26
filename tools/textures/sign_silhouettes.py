#!/usr/bin/env python3
"""Enclosure sign silhouettes (ART-ENVIRONMENT behaviour 6, AENV-011).

Draws each animal's comic silhouette procedurally (source of truth, like the Blender
scripts) and writes `assets/textures/signs/silhouette_<animal>.png`: one solid dark shape
(#2B2320) with soft, rounded edges on a transparent background. The renderer lays it as a
decal on the cream `sign_panel` of `enclosure_sign`.

Zebra: side view facing left, modelled on art/animals/zebra/side.png (big head and muzzle,
upright mohawk mane, pointed ear, tufted tail). A few stripes are cut out of the body and
neck so it reads as a zebra (not a horse) even at small size.

All other animals: derived from the approved concept side view `art/animals/<id>/side.png`
(the figure is separated from the flat grey background, holes filled, and fitted into the
panel area, facing left). Hand-drawn masks (like the zebra's) take precedence.

Usage: python3 tools/textures/sign_silhouettes.py [--preview out.png]
"""

from __future__ import annotations

import argparse
import math
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
OUT_DIR = ROOT / "assets/textures/signs"
INK = (0x2B, 0x23, 0x20)
W, H = 384, 256  # output size (aspect of the silhouette area on the 1.30 × 0.72 m panel)
SS = 4  # supersampling for smooth edges


def _s(pts):
    return [(x * SS, y * SS) for x, y in pts]


def _ellipse(d: ImageDraw.ImageDraw, cx, cy, rx, ry, fill=255):
    d.ellipse([(cx - rx) * SS, (cy - ry) * SS, (cx + rx) * SS, (cy + ry) * SS], fill=fill)


def _rot_ellipse(cx, cy, rx, ry, deg, n=48):
    a = math.radians(deg)
    pts = []
    for i in range(n):
        t = 2 * math.pi * i / n
        x, y = rx * math.cos(t), ry * math.sin(t)
        pts.append((cx + x * math.cos(a) - y * math.sin(a), cy + x * math.sin(a) + y * math.cos(a)))
    return pts


def _capsule(d: ImageDraw.ImageDraw, p0, p1, r0, r1, fill=255, n=12):
    """Tapered capsule from p0 (radius r0) to p1 (radius r1)."""
    steps = max(2, int(math.dist(p0, p1) / 2))
    for i in range(steps + 1):
        t = i / steps
        x = p0[0] + (p1[0] - p0[0]) * t
        y = p0[1] + (p1[1] - p0[1]) * t
        _ellipse(d, x, y, r0 + (r1 - r0) * t, r0 + (r1 - r0) * t, fill)


def _bezier(p0, p1, p2, n=24):
    return [
        (
            (1 - t) ** 2 * p0[0] + 2 * (1 - t) * t * p1[0] + t * t * p2[0],
            (1 - t) ** 2 * p0[1] + 2 * (1 - t) * t * p1[1] + t * t * p2[1],
        )
        for t in (i / n for i in range(n + 1))
    ]


def _curved_stripe(d: ImageDraw.ImageDraw, p0, pc, p2, w0, w1, fill=0):
    """Quadratic curve stroke tapering from half-width w0 to w1 (a pointed stripe)."""
    pts = _bezier(p0, pc, p2)
    left, right = [], []
    for i, (x, y) in enumerate(pts):
        a = pts[max(i - 1, 0)]
        b = pts[min(i + 1, len(pts) - 1)]
        dx, dy = b[0] - a[0], b[1] - a[1]
        ln = math.hypot(dx, dy) or 1.0
        nx, ny = -dy / ln, dx / ln
        w = w0 + (w1 - w0) * i / (len(pts) - 1)
        left.append((x + nx * w, y + ny * w))
        right.append((x - nx * w, y - ny * w))
    d.polygon(_s(left + right[::-1]), fill=fill)


def zebra_mask() -> Image.Image:
    m = Image.new("L", (W * SS, H * SS), 0)
    d = ImageDraw.Draw(m)
    ground = 246
    legs = [(166, 150, -3), (192, 150, -1), (272, 146, 3), (298, 146, 5)]
    # body and rump
    _ellipse(d, 232, 140, 88, 48)
    _ellipse(d, 292, 134, 42, 44)
    _ellipse(d, 178, 138, 36, 38)  # chest
    # legs (slightly tapered) with rounded hooves
    for x, top, dx in legs:
        _capsule(d, (x, top), (x + dx, ground - 14), 12, 9.5)
        _ellipse(d, x + dx, ground - 9, 12.5, 9)
    # neck: thick, leaning forward-up from the chest to behind the head
    d.polygon(_s([(150, 150), (214, 104), (158, 36), (112, 64)]), fill=255)
    # head (big, chibi): skull angled down to a round muzzle
    d.polygon(_s(_rot_ellipse(106, 66, 40, 30, -30)), fill=255)
    _capsule(d, (96, 76), (62, 108), 27, 25)
    # ear, pointing up and a bit back
    d.polygon(_s(_rot_ellipse(134, 20, 9, 22, 22)), fill=255)
    # upright mohawk mane from the poll to the withers
    _capsule(d, (126, 30), (206, 92), 12, 8)
    for i in range(8):
        t = i / 7
        x = 128 + (204 - 128) * t
        y = 26 + (88 - 26) * t
        _ellipse(d, x + 7, y - 9, 8.5, 8.5)
    # tail with a tuft
    _capsule(d, (328, 116), (346, 184), 4.5, 3.5)
    d.polygon(_s(_rot_ellipse(348, 198, 10, 20, -10)), fill=255)

    # stripe cut-outs (the cream panel shows through): pointed, curved like the concept
    cut = ImageDraw.Draw(m)
    for x in (200, 228, 256):
        _curved_stripe(cut, (x + 4, 92), (x - 6, 125), (x - 2, 164), 8.0, 1.0)
    _curved_stripe(cut, (286, 92), (300, 118), (322, 128), 7.0, 1.0)  # rump, sweeping back
    _curved_stripe(cut, (282, 110), (294, 136), (318, 150), 6.0, 1.0)
    # neck: across the neck, from the mane side down towards the throat
    for x0, y0, x1, y1 in [(148, 44, 128, 92), (170, 64, 146, 116), (192, 84, 166, 136)]:
        _curved_stripe(cut, (x0 + 4, y0 + 6), ((x0 + x1) / 2 + 6, (y0 + y1) / 2), (x1, y1), 7.0, 1.0)
    # leg bands
    for x, top, dx in legs:
        for y in (198, 216):
            xc = x + dx * (y - top) / (ground - top)
            cut.rectangle([(xc - 16) * SS, y * SS, (xc + 16) * SS, (y + 4.5) * SS], fill=0)
    # eye and nostril
    _ellipse(cut, 110, 60, 5.5, 6.5, fill=0)
    _ellipse(cut, 50, 104, 3.5, 3, fill=0)
    return m


def render(mask: Image.Image) -> Image.Image:
    # soft rounded edges: slight blur, then downsample
    mask = mask.filter(ImageFilter.GaussianBlur(SS * 0.6)).resize((W, H), Image.LANCZOS)
    img = Image.new("RGBA", (W, H), INK + (0,))
    img.putalpha(mask)
    return img


SILHOUETTES = {"zebra": zebra_mask}

# Animals whose silhouette comes from the approved concept side view (ART-ENVIRONMENT 6).
FROM_ART = ["hippo", "panda", "koala", "elephant", "goldfish", "monkey", "giraffe", "lion", "snow_fox"]
PAD = 12  # px margin inside the W×H panel area
THRESH = 90  # colour distance to the background: keeps the dark outline, drops the light ground shadow


def side_view_mask(animal: str) -> Image.Image:
    """Silhouette mask (supersampled W*SS × H*SS) from art/animals/<animal>/side.png."""
    src = Image.open(ROOT / f"art/animals/{animal}/side.png").convert("RGB")
    grey = src.convert("L")
    w, h = src.size
    bg = src.getpixel((3, 3))
    # foreground = pixels that differ from the background colour (incl. the cast shadow is
    # light grey-blue and differs less than the dark outline; THRESH drops it)
    diff = Image.eval(
        Image.merge("RGB", [Image.eval(c, lambda v, b=b: abs(v - b)) for c, b in zip(src.split(), bg)]).convert("L"),
        lambda v: 255 if v > THRESH else 0,
    )
    # fill holes: flood the background from the border, everything not reached is figure
    flood = diff.copy()
    ImageDraw.floodfill(flood, (0, 0), 128)
    fig = flood.point(lambda v: 0 if v == 128 else 255)
    fig = fig.filter(ImageFilter.MaxFilter(3)).filter(ImageFilter.MinFilter(3))
    # drop the ground shadow ellipse: keep only rows above the lowest dark outline pixel
    bbox = fig.getbbox()
    if bbox is None:
        raise SystemExit(f"no figure found in art/animals/{animal}/side.png")
    fig = fig.crop(bbox)
    # fit into the panel area, keep aspect, centred, bottom-aligned feel
    fw, fh = fig.size
    scale = min((W - 2 * PAD) / fw, (H - 2 * PAD) / fh)
    nw, nh = max(1, int(fw * scale)), max(1, int(fh * scale))
    fig = fig.resize((nw * SS, nh * SS), Image.LANCZOS)
    m = Image.new("L", (W * SS, H * SS), 0)
    m.paste(fig, (((W - nw) // 2) * SS, ((H - nh) // 2) * SS))
    return m


for _animal in FROM_ART:
    if (ROOT / f"art/animals/{_animal}/side.png").exists():
        SILHOUETTES.setdefault(_animal, lambda a=_animal: side_view_mask(a))


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--preview", help="also write a preview on the cream panel colour")
    args = ap.parse_args()
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    for animal, fn in SILHOUETTES.items():
        img = render(fn())
        path = OUT_DIR / f"silhouette_{animal}.png"
        img.save(path, optimize=True)
        print(f"wrote {path.relative_to(ROOT)} {img.size}")
        if args.preview:
            bg = Image.new("RGBA", (W, H), (0xFF, 0xF3, 0xD6, 255))
            bg.alpha_composite(img)
            small = bg.resize((W // 6, H // 6), Image.LANCZOS).resize((W // 2, H // 2), Image.NEAREST)
            sheet = Image.new("RGBA", (W + W // 2 + 10, H), (255, 255, 255, 255))
            sheet.paste(bg, (0, 0))
            sheet.paste(small, (W + 10, 0))
            sheet.save(args.preview)


if __name__ == "__main__":
    main()
