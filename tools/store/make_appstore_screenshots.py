#!/usr/bin/env python3
"""Makes the App Store screenshots at the EXACT pixel sizes App Store Connect accepts (PLAT-040).

Source: the real renderer screenshots art/marketing/screens/letterzoo_screen_NN_*_{landscape_3840x2160,
portrait_2160x3840}.png (NN = 01..05). Each is scaled DOWN (never up: a source smaller than the
target is skipped with a message) and cover-cropped to the target aspect; if the crop would cut more than
MAX_CROP of the picture, it is padded with the cream background instead (so the HUD stays visible).
The result is opaque RGB PNG (no alpha). Review the crops by eye before uploading.

Usage: python3 tools/store/make_appstore_screenshots.py [--out DIR] [--check]
  --out DIR  default store/appstore/screenshots (gitignored: ~100 MB)
  --check    only print the plan (sizes, crop loss), write nothing
"""
import argparse
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent.parent
SRC = ROOT / "art" / "marketing" / "screens"
CREAM = (0xFF, 0xF3, 0xD6)
MAX_CROP = 0.25
# App Store Connect screenshot sizes (px, portrait w x h). iPhone 6.9" and 6.5" and iPad 13".
# Apple names/sizes change: re-check "Screenshot specifications" in the App Store Connect help.
DEVICES = {
    "iphone-6.9": (1320, 2868),
    "iphone-6.5": (1284, 2778),
    "ipad-13": (2064, 2752),
}
SHOTS = ["01_day_follow", "02_riddle_board", "03_home_celebrate", "04_night_plaza", "05_night_terrarium"]


def source(shot: str, landscape: bool) -> Path:
    o = "landscape_3840x2160" if landscape else "portrait_2160x3840"
    return SRC / f"letterzoo_screen_{shot}_{o}.png"


def fit(img: Image.Image, tw: int, th: int) -> tuple[Image.Image | None, float]:
    """Scale down + cover-crop (or pad) to tw x th. Returns (image, cropped fraction) or (None, 0) if upscaling."""
    sw, sh = img.size
    s = max(tw / sw, th / sh)  # cover scale
    cw, ch = round(sw * s), round(sh * s)
    lost = 1 - min(tw / cw, th / ch)
    if lost <= MAX_CROP:
        if s > 1:
            return None, 0.0
        r = img.resize((cw, ch), Image.LANCZOS)
        x, y = (cw - tw) // 2, (ch - th) // 2
        return r.crop((x, y, x + tw, y + th)).convert("RGB"), lost
    s = min(tw / sw, th / sh)  # contain + pad
    if s > 1:
        return None, 0.0
    r = img.resize((round(sw * s), round(sh * s)), Image.LANCZOS)
    out = Image.new("RGB", (tw, th), CREAM)
    out.paste(r.convert("RGB"), ((tw - r.width) // 2, (th - r.height) // 2))
    return out, 0.0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=str(ROOT / "store" / "appstore" / "screenshots"))
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args()
    out = Path(a.out)
    missing = skipped = made = 0
    for dev, (w, h) in DEVICES.items():
        for landscape in (False, True):
            tw, th = (h, w) if landscape else (w, h)
            for i, shot in enumerate(SHOTS, 1):
                p = source(shot, landscape)
                if not p.exists():
                    print(f"MISSING source {p.name}")
                    missing += 1
                    continue
                img, lost = fit(Image.open(p), tw, th)
                tag = "landscape" if landscape else "portrait"
                if img is None:
                    print(f"SKIP {dev} {tag} {shot}: source smaller than {tw}x{th} (no upscaling)")
                    skipped += 1
                    continue
                print(f"{dev:11} {tag:9} {i:02d} {tw}x{th} crop {lost:4.0%} {'pad' if lost == 0 and img.getpixel((0, 0)) == CREAM else ''}")
                if not a.check:
                    d = out / dev / tag
                    d.mkdir(parents=True, exist_ok=True)
                    img.save(d / f"{i:02d}_{shot}.png", optimize=True)
                made += 1
    print(f"{made} made, {skipped} skipped, {missing} missing")
    return 1 if missing or skipped else 0


if __name__ == "__main__":
    sys.exit(main())
