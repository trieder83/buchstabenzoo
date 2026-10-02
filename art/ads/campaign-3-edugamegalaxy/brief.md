# Brief — campaign 3 banner (EduGameGalaxy)

Ad banners are separate from the game's comic style (`art/style/style.md` does not apply). Family look of
campaigns 1/2: bold, bright, wide 1024 × 500 banner, space mood of the ABC Smash banner (dark blue/violet
galaxy, warm yellow, light blue). The picture is generated **without text**; the brand text is added
with Pillow (`specs/10-gameplay/ads/campaign-3-edugamegalaxy/make_banner.py`). No real person, no brand look-alikes.

## Prompt 1 — picture without text

```text
Wide banner illustration in bright, friendly anime style (clean line art, large expressive eyes, cel-shaded colours, vivid saturated colours). A cheerful, smiling child of about 8 years, an invented character (not a real person), short dark hair, a light blue and white space-explorer jacket, stands on the right half of the picture and looks at the viewer with a big happy smile. In the raised right hand the child holds a big glowing golden star with a cute smile, in the raised left hand a big glowing light-blue plus sign (a thick, simple "+" shape). The background is a friendly, colourful galaxy: deep navy and violet night sky with a soft purple nebula, many small twinkling stars, a few cute round planets in orange, teal and blue, a small ringed planet, soft glowing light. The left third of the picture is calm and mostly empty dark-blue space with only tiny stars (room for a text overlay added later). Bold, bright, cheerful and safe for children. Composition: wide 2:1 banner, the child with both hands raised in the right half, full upper body visible, the star and plus sign clearly readable. There is absolutely no text, no letters, no numbers and no logo anywhere in the image. Avoid: realistic photo, real person, celebrity, scary, dark horror mood, weapons, violence, sexualised or revealing clothing, watermark, signature, text, letters, numbers, logos, brand characters, copyrighted characters, extra fingers, deformed hands, blurry.
```

## Negative prompt (short; already contained in the prompt as "Avoid")

```text
photo, real person, scary, weapons, violence, revealing clothing, watermark, text, letters, numbers, logos, known brand characters, deformed hands
```

## Generation log

| Date | File | Model | Prompt | Notes | Result |
|---|---|---|---|---|---|
| 2026-10-01 | specs/10-gameplay/ads/campaign-3-edugamegalaxy/resources/v1.jpg (deleted) | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' | rejected: child cut off by a letterbox bar, symbols fine |
| 2026-10-01 | specs/10-gameplay/ads/campaign-3-edugamegalaxy/resources/v2.jpg (deleted) | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' | rejected: hands below the symbols, star not held |
| 2026-10-01 | specs/10-gameplay/ads/campaign-3-edugamegalaxy/resources/v3.jpg (deleted) | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' | rejected: boy, busy planets with faces; good but less calm text area |
| 2026-10-01 | specs/10-gameplay/ads/campaign-3-edugamegalaxy/resources/v4.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' | **picked** (girl, both symbols clear, empty left third) |

Then `make_banner.py` crops v4 to 1024 × 500 and sets the text (Lato Black, Pillow): `specs/10-gameplay/ads/campaign-3-edugamegalaxy/resources/edugamegalaxy-de.png` / `-en.png`.

Note: the generated files were saved to the campaign's `resources/` folder (spec directory); the log paths are relative to the repo root.
