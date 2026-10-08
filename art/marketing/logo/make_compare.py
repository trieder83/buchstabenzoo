#!/usr/bin/env python3
"""compare.png: every icon at 1024 (thumb), 180 and 60 px with the iOS rounded-corner mask on a home-screen-like background, plus the current icons."""
import os
from PIL import Image, ImageDraw, ImageFont
H = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(H, "..", "..", "..")
def p(n): return os.path.join(H, n)
items = [("current: cover crop", os.path.join(R, "art/marketing/covers/cover_3_reading_square.png"))]
old = os.path.join(R, "web/public/icons/icon-512.png")
if os.path.exists(old): items.append(("web icon (ABC+paw)", old))
names = ["A1 zebra L","A2 giraffe L","A3 girl in gate","A4 lion + book","A5 Z + zebra","A6 ABC blocks","A7 magnifier","A8 fence L"]
cn = ["C1 panda + A","C2 girl + gold A","C3 zebra + ABC tower","C4 lion cub jumps","C5 elephant ABC","C6 night girl"]
items = items[:1] + [(n, p(f"icon_C{i+1}.png")) for i, n in enumerate(cn)] + [(names[0], p("icon_A1.png")), (names[1], p("icon_A2.png"))]
def mask(im, s):
    im = im.convert("RGB").resize((s, s), Image.LANCZOS).convert("RGBA")
    m = Image.new("L", (s*4, s*4), 0); ImageDraw.Draw(m).rounded_rectangle((0, 0, s*4-1, s*4-1), int(s*4*0.2237), fill=255)
    im.putalpha(m.resize((s, s), Image.LANCZOS)); return im
try: ft = ImageFont.truetype("/usr/share/fonts/truetype/lato/Lato-Black.ttf", 22)
except Exception: ft = ImageFont.load_default()
cw, rows = 330, 3
n = len(items); cols = (n+1)//2
sheet = Image.new("RGB", (cols*cw, 640), (70, 90, 120))
# gradient wallpaper
d = ImageDraw.Draw(sheet)
for y in range(640): d.line((0, y, cols*cw, y), fill=(60+y//10, 100+y//8, 170-y//10))
for i, (label, f) in enumerate(items):
    im = Image.open(f); r, c = divmod(i, cols); x0, y0 = c*cw+15, r*320+10
    t = mask(im, 150); sheet.paste(t, (x0, y0+30), t)
    a = mask(im, 90); sheet.paste(a, (x0+160, y0+30), a)
    b = mask(im, 60); sheet.paste(b, (x0+160, y0+135), b)
    sq = im.convert("RGB").resize((60, 60), Image.LANCZOS); sheet.paste(sq, (x0+235, y0+135))
    d.text((x0, y0+4), label, font=ft, fill="white")
    d.text((x0, y0+190), "150 (1024 src) | 180 shown 50% | 60 masked | 60 square", font=ImageFont.load_default(), fill="white")
sheet.save(p("compare.png"), optimize=True)
