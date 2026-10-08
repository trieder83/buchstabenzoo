#!/usr/bin/env python3
"""Builds the logo review files from the generated masters (re-runnable). Fredoka (OFL, @fontsource/fredoka via npm) is used for B2."""
import os, sys
from PIL import Image, ImageDraw, ImageFont
H = os.path.dirname(os.path.abspath(__file__))
FONT = os.environ.get("FREDOKA", "")
def p(n): return os.path.join(H, n)
icons = {"A1":"A1_zebra_L_v1.jpg","A2":"A2_giraffe_L_v1.jpg","A3":"A3_girl_gate_v1.jpg","A4":"A4_lion_book_v1.jpg",
         "A5":"A5_paws_to_Z_final.jpg","A6":"A6_abc_blocks_v1.jpg","A7":"A7_magnifier_eyes_v2.jpg","A8":"A8_fence_L_v2.jpg",
         "C1":"C1_panda_A_v2.jpg","C2":"C2_girl_gold_A_v1.jpg","C3":"C3_zebra_tower_v1.jpg","C4":"C4_lion_cub_jump_v1.jpg","C5":"C5_elephant_letters_v2.jpg","C6":"C6_night_girl_lantern_v2.jpg"}
for k, f in icons.items():
    im = Image.open(p(f)).convert("RGB").resize((1024, 1024), Image.LANCZOS)
    im.save(p(f"icon_{k}.png"), optimize=True)
    if os.path.getsize(p(f"icon_{k}.png")) > 1_400_000:
        im.quantize(256, dither=Image.NONE).save(p(f"icon_{k}.png"), optimize=True)
def crop(f, box, out):
    im = Image.open(p(f)).convert("RGB"); w, h = im.size
    im.crop((int(box[0]*w), int(box[1]*h), int(box[2]*w), int(box[3]*h))).save(p(out), optimize=True)
crop("B1_zebra_lettering_v1.jpg", (0.02, 0.05, 0.98, 0.76), "wordmark_B1_letter_zoo.png")
crop("B3_paw_lettering_v2.jpg", (0.04, 0.78-0.0, 0.98, 0.99) if False else (0.03, 0.74, 0.97, 0.95), "wordmark_B3_buchstabenzoo.png")
# B2: emblem + typeset text
em = Image.open(p("B2_emblem_no_text_v1.jpg")).convert("RGB"); W, Hh = em.size
f = 2752 / W
for lang, txt in (("en", "Letter Zoo"), ("de", "Buchstabenzoo")):
    im = em.copy(); d = ImageDraw.Draw(im)
    bx0, by0, bx1, by1 = [int(v*W/800) for v in (140, 235, 665, 395)]  # banner box
    size = 400
    while True:
        ft = ImageFont.truetype(FONT, size)
        tw = d.textlength(txt, font=ft)
        if tw <= (bx1-bx0)*0.88: break
        size -= 6
    x = (bx0+bx1)//2 - int(tw)//2; y = (by0+by1)//2 - size//2 - int(size*0.05)
    d.text((x+size//18, y+size//18), txt, font=ft, fill="#c98a4a", stroke_width=size//10, stroke_fill="#c98a4a")
    d.text((x, y), txt, font=ft, fill="#ffbf2e", stroke_width=size//10, stroke_fill="#4a2a1a")
    im.crop((int(0.03*W), int(0.30*Hh), int(0.97*W), int(0.96*Hh))).save(p(f"wordmark_B2_emblem_{lang}.png"), optimize=True)

for n in ("wordmark_B1_letter_zoo.png","wordmark_B2_emblem_en.png","wordmark_B2_emblem_de.png","wordmark_B3_buchstabenzoo.png"):
    im = Image.open(p(n)); im = im.resize((1600, int(im.height*1600/im.width)), Image.LANCZOS); im.save(p(n), optimize=True)
