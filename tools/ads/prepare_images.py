#!/usr/bin/env python3
"""Compresses the campaign source images (specs/10-gameplay/ads/*/resources/) into the
served copies in boards/img/ (GAME-ADS "External content", ADS-013): 1024 px wide WebP,
<= 512 KB. Re-run after changing a source image, then run tools/ads/sign.py again.

    python3 tools/ads/prepare_images.py
"""
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / "specs/10-gameplay/ads"
OUT = ROOT / "boards/img"
MAX_BYTES = 512 * 1024
WIDTH = 1024

JOBS = {
    "c1-a.webp": "campaign-1-mathfighter/resources/feature_graphic_5.png",
    "c1-b.webp": "campaign-1-mathfighter/resources/feature_graphic_2.png",
    "c2-de.webp": "campaign-2-abcsmash/resources/feature-graphic-de-de.png",
    "c2-en.webp": "campaign-2-abcsmash/resources/feature-graphic-en-us.png",
    "c3-de.webp": "campaign-3-edugamegalaxy/resources/edugamegalaxy-de.png",
    "c3-en.webp": "campaign-3-edugamegalaxy/resources/edugamegalaxy-en.png",
}


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    for name, rel in JOBS.items():
        img = Image.open(SRC / rel)
        if img.mode not in ("RGB", "RGBA"):
            img = img.convert("RGBA" if "A" in img.mode else "RGB")
        if img.width > WIDTH:
            img = img.resize((WIDTH, round(img.height * WIDTH / img.width)), Image.LANCZOS)
        for quality in (85, 75, 65, 55, 45):
            dest = OUT / name
            img.save(dest, "WEBP", quality=quality, method=6)
            if dest.stat().st_size <= MAX_BYTES:
                break
        size = dest.stat().st_size
        print(f"{name}: {img.width}x{img.height}, {size / 1024:.0f} KB (q{quality})")
        if size > MAX_BYTES:
            raise SystemExit(f"{name} is larger than {MAX_BYTES} bytes")


if __name__ == "__main__":
    main()
