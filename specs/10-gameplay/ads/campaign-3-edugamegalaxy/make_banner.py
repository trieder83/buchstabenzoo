#!/usr/bin/env python3
"""Builds the campaign-3 banners (1024 x 500) from the text-free picture: crop to 2:1, brand text overlay
with Pillow (the image model never spells text). Output: resources/edugamegalaxy-{de,en}.png

    python3 specs/10-gameplay/ads/campaign-3-edugamegalaxy/make_banner.py
"""
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

HERE = Path(__file__).resolve().parent
RES = HERE / "resources"
SRC = RES / "v4.jpg"  # picked variant, see art/ads/campaign-3-edugamegalaxy/brief.md
FONT = "/usr/share/fonts/truetype/lato/Lato-Black.ttf"
YELLOW, BLUE, WHITE = (255, 214, 102), (130, 200, 255), (255, 255, 255)
S = 2  # supersampling
W, H = 1024 * S, 500 * S

LINES = {
    "de": ("Effizient und mit Spaß lernen", "für den Erfolg im Leben"),
    "en": ("Learn efficiently and with fun", "for success in life"),
}


def text(d, xy, s, size, fill):
    f = ImageFont.truetype(FONT, size * S)
    d.text((xy[0] * S, xy[1] * S), s, font=f, fill=fill, stroke_width=2 * S, stroke_fill=(10, 14, 50))
    return d.textlength(s, font=f) / S


def build(lang: str) -> None:
    img = Image.open(SRC).convert("RGB")
    h = round(img.width / 2.05)  # 1024:500
    top = 40
    img = img.crop((0, top, img.width, top + h)).resize((W, H), Image.LANCZOS)
    d = ImageDraw.Draw(img)
    a, b = LINES[lang]
    text(d, (40, 38), a, 33 if lang == "de" else 31, YELLOW)
    text(d, (40, 82), b, 33 if lang == "de" else 31, WHITE)
    text(d, (36, 250), "EduGame", 94, YELLOW)
    text(d, (36, 345), "Galaxy", 94, BLUE)
    img.resize((1024, 500), Image.LANCZOS).save(RES / f"edugamegalaxy-{lang}.png")


if __name__ == "__main__":
    for lang in LINES:
        build(lang)
