# Brief — `covers` (store / website cover images)

Spec: ART-PIPELINE (§7 generation log, APIPE-010), ART-DIRECTION. Style: `art/style/style.md` (comic). Request: user, 2026-10-06. Status: **in-review** (generated 2026-10-06; nothing approved — the user decides).

## Purpose

Three 16:9 cover images ("key art") for the store and website listings of **Letter Zoo** (German title **Buchstabenzoo**): (1) day zoo, (2) night zoo with lantern and the terrarium house, (3) the reading idea. Each: 2K, no text baked in by the model (model text is unreliable), title added afterwards with PIL (separate files, EN and DE), plus a 1024x500 feature-graphic crop and a 1080x1080 square crop. Child-safe, no brands, no real people, no logos, no weapons. The reference images pin the real look (girl, animals, night palette).

## Image settings

- `tools/gen_image.py art/marketing/covers/brief.md --prompt N --out cover_N_<name>_v1.png cover_N_<name>_v2.png --aspect 16:9 --size 2K --ref <refs> --extra "<ref note>"` (2 variants per cover, `gemini-3-pro-image`).
- References: 1 day = `art/environment/style_frame/style_frame.png`, `art/characters/player_girl/three_quarter.png`, `art/animals/{zebra,hippo,panda,elephant}/three_quarter.png`. 2 night = `art/environment/style_frame_night/style_frame_night.png`, `art/environment/env_terrarium_house/overview_v2.jpg`, `art/characters/player_girl/three_quarter.png`. 3 reading = `art/environment/style_frame/style_frame.png`, girl, zebra, panda.
- Reference note (`--extra`): "The attached images only define the look: our comic style, the girl, the animals and the zoo props. Do not copy their layout; compose the new cover scene described above."

## Prompts

First paragraph of every prompt = STYLE block from `art/style/style.md`, verbatim. Cover 2 states that its night lighting replaces the midday sunlight in the block (same approach as `style_frame_night`).

### Prompt 1 — `cover_1_day`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

COVER KEY ART, horizontal widescreen 16:9 composition for a children's game store listing, a little lower and closer than the in-game camera (about 30 degrees above the ground) so characters are big and readable, but still the exact same comic look, characters and zoo props as the reference images. Leave the upper-left third of the image calmer (grass, bushes, a simple sky of flat light blue with two fluffy white comic clouds) as quiet space for a title added later.

A bright, cheerful DAY AT THE ZOO. In the lower centre-right stands the same girl as in the reference image (about 8 years old, long straight dark-brown hair, big brown eyes, white T-shirt with blue stripes, blue jeans with rolled cuffs, brown shoes), smiling, one hand pointing at a large wooden RIDDLE BOARD on two posts beside her, the board's face is a plain cream panel with only a few abstract wavy lines and a big simple question mark shape (no letters, no words, no numbers). Around her on the sand-beige path and the grass friendly animals gather curiously and playfully, all in the comic style of the reference images: a stripy zebra, a round happy hippo, a black-and-white panda and a big grey elephant with a little trunk wave, all looking toward the girl or the board with big expressive eyes. On the left a wooden-fenced EMPTY ENCLOSURE with an open gate, a stone arch shelter and clearly visible cartoon PAW PRINTS and hoof prints leading from the enclosure across the path (the animals have escaped!), bushes and round trees, a wooden bench. Mood: adventurous, funny, sunny, safe, full of joy. All signs are blank without any letters or numbers.
```

### Prompt 2 — `cover_2_night`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

COVER KEY ART, horizontal widescreen 16:9 composition for a children's game store listing, a little lower and closer than the in-game camera (about 30 degrees above the ground) so characters are big and readable, same comic look, girl and zoo props as the reference images. Leave the upper-left third calmer (deep blue flat night sky with a few friendly white stars and a big round cream moon, or soft blue hedges) as quiet space for a title added later.

NIGHT VERSION: this lighting replaces the midday sunlight described above: no sun; soft moonlight from the upper left gives a deep friendly blue night palette with one hard-edged darker blue shadow tone, never pitch black, everything clearly visible with bold dark outlines and flat colours. Warm yellow-orange lantern light is the second light. In the lower centre stands the same girl as in the reference image (about 8 years old, long straight dark-brown hair, big brown eyes, white T-shirt with blue stripes, blue jeans with rolled cuffs, brown shoes), smiling and curious, holding up a small glowing old-fashioned LANTERN that casts a round hard-edged pool of warm yellow light around her. On the right the cosy wooden TERRARIUM HOUSE: a stepped half-disc building with a mossy green roof, a round arched door glowing warm yellow, and glass terrarium windows with warm light inside; in the warm window light and on the path peek out friendly small animals: a green SNAKE curled up smiling, a colourful CHAMELEON on a branch with a curly tail and a little green FROG sitting on a leaf, all with big cute eyes. In the dark bushes and between the trees on the left a few pairs of big friendly softly glowing yellow-green animal EYES shine (cute, round, with a friendly look, never red, never menacing). A few lantern posts glow along the path and tiny warm yellow fireflies and small white stars sparkle in the air. The mood is a cosy, magical bedtime adventure, safe and friendly for four-year-olds, nothing scary. All signs are blank without any letters or numbers.
```

### Prompt 3 — `cover_3_reading`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

COVER KEY ART, horizontal widescreen 16:9 composition for a children's game store listing about LEARNING TO READ, a little lower and closer than the in-game camera so everything is big and readable, same comic look, girl and animals as the reference images. Leave the upper-left corner calmer (a flat light-blue sky with two fluffy white comic clouds) as quiet space for a title added later.

A sunny meadow at the zoo, bright and happy. In the lower centre the same girl as in the reference image (about 8 years old, long straight dark-brown hair, big brown eyes, white T-shirt with blue stripes, blue jeans with rolled cuffs, brown shoes) sits or kneels on the grass, smiling, holding a big OPEN BOOK whose two pages show only large colourful abstract picture-like shapes and wavy line rows (no real writing, no readable letters). Around and above her float and stand giant, chunky, rounded, colourful toy ALPHABET BLOCKS and bubble-shaped letter forms in red, yellow, blue, green and orange with bold dark-brown outlines, as 3D comic props; they should look like simple capital letter shapes such as A, B and Z but are purely decorative, drawn large, few and clean (it is fine if they are simple shapes). Next to her a wooden RIDDLE SIGN on posts with a plain cream face showing only a big simple lightbulb or question mark shape. Friendly animals PEEK curiously into the picture from the edges and from behind the signs and bushes: a stripy zebra with its head tilted, a panda holding a letter block, a round hippo, a little elephant with its trunk raised, a monkey hanging from a branch and a koala, all with big expressive eyes and happy smiles, like they are reading along. Soft colourful comic sparkles and tiny stars float in the air, a few round trees and a wooden fence in the background. Mood: curious, proud, warm, playful. All signs are blank without real words, letters or numbers except the decorative chunky letter blocks.
```

### Negative prompts

All prompts (the negative list is appended to the prompt as "Avoid ..." by the tool; the tool uses the first negative block, i.e. this one):

```text
text, words, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, weapons, cages, cage bars, rubbish, crowds, clutter, horror, scary, spooky, creepy, menacing shapes, glowing red eyes, monsters, angry or menacing animals, sharp teeth, distorted anatomy, extra legs, extra heads, extra fingers, fisheye distortion, blurry, low resolution, cropped main subject, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

For cover 2 the extra text additionally says: "Night scene: no daylight, no sunshine, no pitch black areas, nothing scary."

## Title lockup (PIL)

No rounded display font is installed (`fc-list`: only DejaVu, Lato, Liberation, FreeSans, Kacst, Tlwg ...). Used **Lato Black** (`/usr/share/fonts/truetype/lato/Lato-Black.ttf`) with a thick dark-brown (#4a2a1a) outline (rounded stroke joins from PIL), warm yellow fill with a hard offset shadow, to match the comic look. Script: `make_titles.py` in this folder (re-runnable). Titles: "Letter Zoo" (EN) and "Buchstabenzoo" (DE), placed in the calm upper-left area; for the feature graphic and square crops the title is re-laid-out inside the crop.

## Crops

`make_titles.py` crops from the chosen 16:9 master: feature graphic 1024x500 (aspect 2.048, crop box centred on the main subject, scaled), square 1080x1080 (crop box with horizontal centre on the subject). Crop centres are set per cover in the script.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-06 | cover_3_reading_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: style_frame.png, three_quarter.png, three_quarter.png, three_quarter.png | alternative — alphabet blocks and floating letters, 6 animals; some block letters malformed (mirrored/odd "C"), a green bar shape beside the zebra |
| 2026-10-06 | cover_3_reading_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: style_frame.png, three_quarter.png, three_quarter.png, three_quarter.png | **chosen** — cleaner letters (A B Z C D), open book, riddle sign with question mark, panda holding an A block, monkey, koala, elephant, hippo, zebra; sky top-left for the title |
| 2026-10-06 | cover_2_night_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame_night.png, overview_v2.jpg, three_quarter.png | **chosen** — lantern pool, terrarium house with snake, chameleon and frog in lit windows, glowing friendly eyes in the bushes, moon and stars; the house roof sign shows a snake pictogram; house is a simple box shape (not the stepped half-disc), the sign board is blank |
| 2026-10-06 | cover_2_night_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame_night.png, overview_v2.jpg, three_quarter.png | alternative — girl larger, chameleon + frog windows, snake only on the roof emblem; moon cut off at the corner, less title room |
| 2026-10-06 | cover_1_day_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, three_quarter.png, three_quarter.png, three_quarter.png, three_quarter.png, three_quarter.png | rejected — the model baked the words "RIDDLE BOARD" onto the board (text must not be in the image); otherwise good |
| 2026-10-06 | cover_1_day_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, three_quarter.png, three_quarter.png, three_quarter.png, three_quarter.png, three_quarter.png | **chosen** — clean (no text), girl pointing at the riddle board, zebra, hippo, panda, elephant, empty enclosure with open gate, paw and hoof prints; calm sky top-left for the title |

Generated size 2752x1536 (the 2K setting of the model). No image was edited after generation; masters of the chosen variants are copied to `cover_N_<name>.png`.

## Outputs per cover (N = 1_day, 2_night, 3_reading)

| File | Content |
|---|---|
| `cover_N_<name>.png` | chosen master, 2752x1536, no text |
| `cover_N_<name>_title_en.png` / `_de.png` | master + title "Letter Zoo" / "Buchstabenzoo" |
| `cover_N_<name>_feature.png` | 1024x500 feature graphic (no title) |
| `cover_N_<name>_feature_title_en.png` / `_de.png` | same with title |
| `cover_N_<name>_square.png` | 1080x1080 (no title) |
| `cover_N_<name>_square_title_en.png` / `_de.png` | same with title |

## Recommendation

1. Day: **v2** (v1 has baked text). 2. Night: **v1** (all three terrarium animals visible, moon, eyes). 3. Reading: **v2**.

## Known issues / notes

- No rounded display font installed; Lato Black with a thick outline is a close comic look but not a custom lockup. A designer's logo should replace it for the final store listing.
- Night square crop cuts the chameleon window (the house sits at the right edge); feature crop keeps everything.
- The day, night and reading covers contain decorative or pictogram shapes only; the reading cover has chunky decorative letters (A B C D Z) on purpose.
- Gemini rendered words once despite the prompt; always check for baked text.
- Nothing is approved; set status in `art/catalog.js` only after the user's decision.
