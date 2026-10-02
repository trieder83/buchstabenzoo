#!/usr/bin/env python3
"""Draws the Buchstabenzoo app icons (TECH-PLATFORMS "Installable and full screen", PLAT-014).

Deterministic (Pillow, no randomness): a green rounded square with zebra stripes, a cream badge
with a bold "ABC" and a yellow paw dot, dark brown outlines (art/style/style.md palette).
Writes web/public/icons/{icon-192,icon-512,icon-maskable-512,apple-touch-icon,favicon-32}.png.
Usage: python3 tools/make_icons.py
"""
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

OUT = Path(__file__).resolve().parent.parent / "web" / "public" / "icons"
BROWN, CREAM, YELLOW, GREEN, DARK_GREEN = "#3b2314", "#fff3d6", "#ffd65c", "#7cc46a", "#5e8c3d"
FONTS = [
    "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf",
]
S = 1024  # master canvas, downscaled with LANCZOS


def font(size: int) -> ImageFont.FreeTypeFont:
    for f in FONTS:
        if Path(f).exists():
            return ImageFont.truetype(f, size)
    return ImageFont.load_default(size)


def master(maskable: bool) -> Image.Image:
    """Full-bleed art for maskable, a rounded square with transparent corners otherwise."""
    img = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    bg = Image.new("RGBA", (S, S), GREEN)
    d = ImageDraw.Draw(bg)
    # zebra stripes: diagonal dark-green bands
    for x in range(-S, 2 * S, 220):
        d.polygon([(x, S), (x + 90, S), (x + 90 + S, 0), (x + S, 0)], fill=DARK_GREEN)
    # content scale: maskable keeps everything inside the central 80 % circle (safe zone)
    k = 0.78 if maskable else 1.0
    c = S / 2

    def box(x0, y0, x1, y1):
        return [c + (x0 - c) * k, c + (y0 - c) * k, c + (x1 - c) * k, c + (y1 - c) * k]

    w = int(28 * k)
    d.rounded_rectangle(box(120, 190, 904, 610), radius=int(150 * k), fill=CREAM, outline=BROWN, width=w)
    f = font(int(300 * k))
    d.text((c, c - 112 * k), "ABC", font=f, fill=BROWN, anchor="mm", stroke_width=int(6 * k), stroke_fill=BROWN)
    # yellow paw dot (pad + 4 toes) under the badge
    px, py = c, c + 290 * k
    d.ellipse(box(px - 70, py - 40, px + 70, py + 70), fill=YELLOW, outline=BROWN, width=int(18 * k))
    for dx, dy in ((-105, -45), (-38, -95), (38, -95), (105, -45)):
        d.ellipse(
            box(px + dx - 30, py + dy - 30, px + dx + 30, py + dy + 30), fill=YELLOW, outline=BROWN, width=int(14 * k)
        )
    if maskable:
        return bg
    mask = Image.new("L", (S, S), 0)
    ImageDraw.Draw(mask).rounded_rectangle([0, 0, S - 1, S - 1], radius=190, fill=255)
    img.paste(bg, (0, 0), mask)
    ImageDraw.Draw(img).rounded_rectangle([0, 0, S - 1, S - 1], radius=190, outline=BROWN, width=28)
    return img


def save(img: Image.Image, name: str, size: int, flatten: bool = False) -> None:
    out = img.resize((size, size), Image.LANCZOS)
    if flatten:  # iOS fills transparent corners black: use an opaque square
        base = Image.new("RGB", (size, size), GREEN)
        base.paste(out, (0, 0), out)
        out = base
    out.save(OUT / name, optimize=True)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    rounded, full = master(False), master(True)
    save(rounded, "icon-192.png", 192)
    save(rounded, "icon-512.png", 512)
    save(full, "icon-maskable-512.png", 512)
    save(master(False).resize((S, S)), "apple-touch-icon.png", 180, flatten=True)
    save(rounded, "favicon-32.png", 32)


if __name__ == "__main__":
    main()
