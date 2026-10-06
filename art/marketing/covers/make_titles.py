#!/usr/bin/env python3
"""Cover masters -> title lockups (EN/DE) and crops. Re-runnable: python3 make_titles.py
Font: Lato Black + thick dark-brown rounded outline (no rounded display font installed)."""
from PIL import Image, ImageDraw, ImageFont
import os
os.chdir(os.path.dirname(os.path.abspath(__file__)))
FONT = "/usr/share/fonts/truetype/lato/Lato-Black.ttf"
TITLES = {"en": "Letter Zoo", "de": "Buchstabenzoo"}
BROWN, YELLOW = (74, 42, 26), (255, 210, 63)

# chosen master, title box (master coords: left, top, max width, max height),
# feature crop (y0 of a full-width 2.048:1 crop; title box in crop coords),
# square crop (x0 of a 1536x1536 crop; title box in crop coords)
COVERS = {
 "cover_1_day": dict(src="cover_1_day_v2.jpg", title=(70, 40, 1450, 250),
   feat_y0=100, feat_title=(70, 30, 1450, 250), sq_x0=880, sq_title=(50, 10, 950, 160)),
 "cover_2_night": dict(src="cover_2_night_v1.jpg", title=(60, 20, 1500, 230),
   feat_y0=60, feat_title=(60, 20, 1500, 230), sq_x0=560, sq_title=(50, 10, 950, 160)),
 "cover_3_reading": dict(src="cover_3_reading_v2.jpg", title=(60, 30, 1500, 250),
   feat_y0=60, feat_title=(60, 30, 1500, 250), sq_x0=1040, sq_title=(50, 10, 950, 160)),
}

def title_layer(text, maxw, maxh):
    size = 400
    while True:
        f = ImageFont.truetype(FONT, size)
        sw = max(4, size // 9)
        l, t, r, b = ImageDraw.Draw(Image.new("L", (1, 1))).textbbox((0, 0), text, font=f, stroke_width=sw)
        if (r - l) <= maxw - 24 and (b - t) <= maxh - 24 or size < 20:
            break
        size -= 4
    off = max(5, size // 14)
    W, H = r - l + off + 2, b - t + off + 2
    lay = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    d = ImageDraw.Draw(lay)
    d.text((-l, -t + off), text, font=f, fill=BROWN, stroke_width=sw, stroke_fill=BROWN)  # hard shadow
    d.text((-l, -t), text, font=f, fill=YELLOW, stroke_width=sw, stroke_fill=BROWN)
    return lay

def put(img, text, box):
    x, y, w, h = box
    lay = title_layer(text, w, h)
    out = img.convert("RGBA")
    out.alpha_composite(lay, (x + (w - lay.width) // 2 if False else x, y))
    return out.convert("RGB")

for name, c in COVERS.items():
    m = Image.open(c["src"]).convert("RGB")
    W, H = m.size
    m.save(f"{name}.png")
    fh = round(W / 2.048)
    feat = m.crop((0, c["feat_y0"], W, c["feat_y0"] + fh)).resize((1024, 500), Image.LANCZOS)
    sq = m.crop((c["sq_x0"], 0, c["sq_x0"] + H, H)).resize((1080, 1080), Image.LANCZOS)
    feat.save(f"{name}_feature.png"); sq.save(f"{name}_square.png")
    for lang, text in TITLES.items():
        put(m, text, c["title"]).save(f"{name}_title_{lang}.png")
        # titles on crops are laid out on the full-size crop, then scaled
        fc = m.crop((0, c["feat_y0"], W, c["feat_y0"] + fh))
        put(fc, text, c["feat_title"]).resize((1024, 500), Image.LANCZOS).save(f"{name}_feature_title_{lang}.png")
        sc = m.crop((c["sq_x0"], 0, c["sq_x0"] + H, H))
        put(sc, text, c["sq_title"]).resize((1080, 1080), Image.LANCZOS).save(f"{name}_square_title_{lang}.png")
