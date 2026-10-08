# Brief — `logo` (app icon + wordmark suggestions)

Spec: ART-PIPELINE (§7 generation log, APIPE-010), ART-DIRECTION. Style: `art/style/style.md` (comic). Request: user, 2026-10-08. Status: **in-review** (nothing approved — the user decides).

## Purpose

(A) 8 app icon suggestions (1024x1024, full-bleed, no transparency, no baked rounded corners; iOS masks them), (B) 3 wordmark logo suggestions "Letter Zoo" / "Buchstabenzoo", (C) comparison sheet `compare.png` (1024 thumbnail, 180 px, 60 px with iOS mask on a home-screen-like background, next to the current icon). Palette: avoid all-green (the zoo is green): warm yellow, sky blue, coral, zebra black/white. Child-safe, no brands, no trademarked characters.

## Settings

`tools/gen_image.py art/marketing/logo/brief.md --prompt N --out <name>_v1.png ... --aspect 1:1|16:9 --size 1K|2K --ref <refs> --extra "<ref note>"`. Icons: 1:1 at 2K, downscaled to 1024 with PIL. References: `art/animals/<animal>/three_quarter.png`, girl `art/characters/player_girl/three_quarter.png`. Reference note: "The attached images only define the look: our comic style and the characters/animals. Do not copy their layout; compose the new image described above."

## Prompts

First paragraph of each prompt = STYLE block from `art/style/style.md`, verbatim.

### Prompt 1 — `A1_zebra_L`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed flat background colour (no rounded corners, no border, no frame, no drop shadow around the icon, no transparency, no mockup). One single bold centred subject, very simple chunky shapes with thick dark-brown outlines so it stays readable when shrunk to 60 pixels, a small safe margin around the subject. No text except where stated. Background: sunny warm yellow with a subtle lighter sun-burst of flat rays. Subject: the friendly smiling head of a zebra seen from the front, big round happy eyes, bold black-and-white stripes, and across its forehead and cheek one of the big black stripes forms a clear, chunky capital letter L. Simple, symmetric, very bold.
```

### Prompt 2 — `A2_giraffe_L`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed flat background colour (no rounded corners, no border, no frame, no drop shadow around the icon, no transparency, no mockup). One single bold centred subject, very simple chunky shapes with thick dark-brown outlines so it stays readable when shrunk to 60 pixels, a small safe margin around the subject. No text except where stated. Background: bright sky blue with two fluffy flat white comic clouds. Subject: a cheerful giraffe whose long neck goes straight up and whose body is arranged so that the vertical neck and the horizontal back and legs together form a big chunky capital letter L (long vertical neck, short horizontal body at the bottom), orange-yellow coat with brown patches, big smiling eyes, two little horns. Only this one giraffe.
```

### Prompt 3 — `A3_girl_gate`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed flat background colour (no rounded corners, no border, no frame, no drop shadow around the icon, no transparency, no mockup). One single bold centred subject, very simple chunky shapes with thick dark-brown outlines so it stays readable when shrunk to 60 pixels, a small safe margin around the subject. No text except where stated. Background: warm sunset orange-to-coral flat colour (two flat tones). Subject: a big white-and-wood zoo gate arch in the shape of a round arch; in the arch opening the same girl as in the reference image (long straight dark-brown hair, white blue-striped T-shirt) peeks out cheerfully and waves, below her on the path three cartoon paw prints. No text.
```

### Prompt 4 — `A4_lion_book`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed flat background colour (no rounded corners, no border, no frame, no drop shadow around the icon, no transparency, no mockup). One single bold centred subject, very simple chunky shapes with thick dark-brown outlines so it stays readable when shrunk to 60 pixels, a small safe margin around the subject. No text except where stated. Background: deep friendly teal-blue flat colour with a few small white stars. Subject: the friendly smiling face of a young lion with a big golden mane in the comic style of the reference animal, holding up a big open book in front of its chin; the book pages show only two simple coloured shapes (a sun and a leaf) and wavy lines, no letters. Warm gold, orange and cream colours against the blue.
```

### Prompt 5 — `A5_paws_to_Z`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed flat background colour (no rounded corners, no border, no frame, no drop shadow around the icon, no transparency, no mockup). One single bold centred subject, very simple chunky shapes with thick dark-brown outlines so it stays readable when shrunk to 60 pixels, a small safe margin around the subject. No text except where stated. Background: sky blue. Subject: a trail of four big cartoon paw prints (alternating colours: orange and yellow) winding diagonally up toward one huge chunky capital letter Z in the upper centre; every paw print is fully inside the square with a margin to the edge, nothing is cut off, the plain flat blue background continues to all four edges (no ground, no horizon, no blur). The Z is bright red with white zebra-style stripes and thick dark-brown outline, with a small cute zebra foal head peeking from behind it. Simple and bold.
```

### Prompt 6 — `A6_abc_blocks`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed flat background colour (no rounded corners, no border, no frame, no drop shadow around the icon, no transparency, no mockup). One single bold centred subject, very simple chunky shapes with thick dark-brown outlines so it stays readable when shrunk to 60 pixels, a small safe margin around the subject. No text except where stated. Background: warm cream-yellow flat colour. Subject: a stack of three chunky toy alphabet blocks forming a little tower: bottom blocks red 'A' and blue 'B', a yellow 'C' on the second level, and on the very top sits a small cheerful zebra standing on the top block, waving a hoof. Thick dark-brown outlines. The letters must be exactly A, B, C, correctly formed and upright.
```

### Prompt 7 — `A7_magnifier_eyes`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed flat background colour (no rounded corners, no border, no frame, no drop shadow around the icon, no transparency, no mockup). One single bold centred subject, very simple chunky shapes with thick dark-brown outlines so it stays readable when shrunk to 60 pixels, a small safe margin around the subject. No text except where stated. Background: bright grass-free coral pink to orange flat colour (no green). Subject: a big round magnifying glass with a thick wooden-brown handle pointing to the lower right; in the glass lens, a pair of huge friendly curious animal eyes (big white eyes, brown pupils with white sparkle highlights) peek out of a cute round panda-and-fox-like face in black and orange, a question mark shape floats at the top right. No letters.
```

### Prompt 8 — `A8_fence_L`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed flat background colour (no rounded corners, no border, no frame, no drop shadow around the icon, no transparency, no mockup). One single bold centred subject, very simple chunky shapes with thick dark-brown outlines so it stays readable when shrunk to 60 pixels, a small safe margin around the subject. No text except where stated. Background: sunny warm yellow-orange. Subject: a big capital letter L built from chunky wooden zoo fence planks (a vertical fence post stack with a horizontal bottom plank), light warm wood brown with dark-brown outlines, and a cheerful brown monkey with a long curly tail hanging from the top of the L by one arm, waving with the other hand, big expressive eyes, a banana in its other foot. The L is clearly readable as the letter L.
```

### Prompt 9 — `B1_zebra_lettering`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

WORDMARK LOGO on a plain flat light cream background (#FFF6DC), landscape, horizontal composition, the text 'Letter Zoo' in two words on ONE line, very chunky fat rounded comic lettering, warm yellow-orange letter faces with thick dark-brown outline and a hard offset darker shadow, spelled exactly L-e-t-t-e-r Z-o-o. The capital Z is covered in black-and-white zebra stripes, the two letters 'o' in 'Zoo' are big cartoon animal eyes with round pupils and white sparkle highlights, the letter 'tt' has a small green leaf. A little friendly zebra head peeks over the top of the Z. Only this text, nothing else.
```

### Prompt 10 — `B2_emblem_no_text`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

EMBLEM ARTWORK WITHOUT ANY TEXT, landscape, plain flat light cream background (#FFF6DC). A cheerful round comic badge: a big smiling friendly lion face, a panda face and an elephant face peek over the top edge of a large empty rounded wooden banner (a flat wide rounded rectangle plank with dark-brown outline and wood grain absent, plain warm cream colour, completely EMPTY, no writing) that fills the lower half; small paw prints and a few stars around. The banner stays blank so lettering can be added later.
```

### Prompt 11 — `B3_paw_lettering`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

WORDMARK LOGO on a plain flat light cream background (#FFF6DC), landscape, horizontal composition, the single word 'Buchstabenzoo' on ONE line in very chunky rounded bubbly comic lettering, each letter in a different bright colour (red, blue, yellow, orange, purple), thick dark-brown outline and hard offset shadow, spelled exactly B-u-c-h-s-t-a-b-e-n-z-o-o. The two final letters 'o' are drawn as big cartoon paw prints with toe beans, the 'B' has two little round animal ears on top. Only this text, nothing else.
```

### Prompt 12 — `C1_panda_A` (round 2: richer icons, user feedback 2026-10-08)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK in the rich, saturated, high-contrast look of a children's game store cover, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed background (no rounded corners, no border, no white frame, no mockup, no transparency). One clear focal point: a big friendly character cropped tight (face and upper body very large) plus big chunky glossy-free 3D comic ALPHABET BLOCKS / letters with thick dark-brown outlines, in red, blue, yellow and green, strong colour contrast, no tiny details, readable at 60 pixels. The only text allowed is the capital letters A, B and C on the blocks, drawn exactly, upright and correctly. Same characters as in the reference images. Background: sunny sky blue with a few flat white comic clouds and a warm yellow sun glow. Subject: the black-and-white panda from the reference, huge happy face, hugging a giant red letter A with both arms; beside it at the bottom a blue block B and a yellow block C. Confetti stars in yellow and orange.
```

### Prompt 13 — `C2_girl_gold_A` (round 2: richer icons, user feedback 2026-10-08)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK in the rich, saturated, high-contrast look of a children's game store cover, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed background (no rounded corners, no border, no white frame, no mockup, no transparency). One clear focal point: a big friendly character cropped tight (face and upper body very large) plus big chunky glossy-free 3D comic ALPHABET BLOCKS / letters with thick dark-brown outlines, in red, blue, yellow and green, strong colour contrast, no tiny details, readable at 60 pixels. The only text allowed is the capital letters A, B and C on the blocks, drawn exactly, upright and correctly. Same characters as in the reference images. Background: warm sunny orange to yellow flat burst with a sky-blue lower area. Subject: the same girl as in the reference image (long straight dark-brown hair, big brown eyes, white T-shirt with blue stripes), big smiling face and shoulders, proudly holding up a huge shiny golden letter A with thick brown outline in both hands, a stripy zebra peeking cheerfully over her shoulder; small blue B and red C blocks at the bottom corners.
```

### Prompt 14 — `C3_zebra_tower` (round 2: richer icons, user feedback 2026-10-08)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK in the rich, saturated, high-contrast look of a children's game store cover, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed background (no rounded corners, no border, no white frame, no mockup, no transparency). One clear focal point: a big friendly character cropped tight (face and upper body very large) plus big chunky glossy-free 3D comic ALPHABET BLOCKS / letters with thick dark-brown outlines, in red, blue, yellow and green, strong colour contrast, no tiny details, readable at 60 pixels. The only text allowed is the capital letters A, B and C on the blocks, drawn exactly, upright and correctly. Same characters as in the reference images. Background: bright sky blue with yellow confetti stars and tiny colourful streamers. Subject: the big smiling zebra from the reference, head and chest, next to a tower of three chunky alphabet blocks: red A, blue B and yellow C with green outlines accents, the zebra balancing the top block on its nose or leaning on the tower, joyful.
```

### Prompt 15 — `C4_lion_cub_jump` (round 2: richer icons, user feedback 2026-10-08)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK in the rich, saturated, high-contrast look of a children's game store cover, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed background (no rounded corners, no border, no white frame, no mockup, no transparency). One clear focal point: a big friendly character cropped tight (face and upper body very large) plus big chunky glossy-free 3D comic ALPHABET BLOCKS / letters with thick dark-brown outlines, in red, blue, yellow and green, strong colour contrast, no tiny details, readable at 60 pixels. The only text allowed is the capital letters A, B and C on the blocks, drawn exactly, upright and correctly. Same characters as in the reference images. Background: purple-violet to bright pink flat burst with golden stars. Subject: a cute lion cub (same comic style as the reference lion, big eyes, small mane) wearing a tiny golden crown, jumping joyfully in the air with arms out, big letter blocks A (red), B (blue) and C (yellow) flying around it in an arc.
```

### Prompt 16 — `C5_elephant_letters` (round 2: richer icons, user feedback 2026-10-08)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK in the rich, saturated, high-contrast look of a children's game store cover, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed background (no rounded corners, no border, no white frame, no mockup, no transparency). One clear focal point: a big friendly character cropped tight (face and upper body very large) plus big chunky glossy-free 3D comic ALPHABET BLOCKS / letters with thick dark-brown outlines, in red, blue, yellow and green, strong colour contrast, no tiny details, readable at 60 pixels. The only text allowed is the capital letters A, B and C on the blocks, drawn exactly, upright and correctly. Same characters as in the reference images. Background: sunny sky blue with white clouds and a yellow sun. Subject: the friendly big grey elephant from the reference, huge happy face, trunk raised high spraying a fountain of colourful chunky letters A, B and C (red, yellow, green, blue) like water into the air, with sparkles.
```

### Prompt 17 — `C6_night_girl_lantern` (round 2: richer icons, user feedback 2026-10-08)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

APP ICON ARTWORK in the rich, saturated, high-contrast look of a children's game store cover, perfectly square 1:1, the artwork fills the entire square edge to edge with a full-bleed background (no rounded corners, no border, no white frame, no mockup, no transparency). One clear focal point: a big friendly character cropped tight (face and upper body very large) plus big chunky glossy-free 3D comic ALPHABET BLOCKS / letters with thick dark-brown outlines, in red, blue, yellow and green, strong colour contrast, no tiny details, readable at 60 pixels. The only text allowed is the capital letters A, B and C on the blocks, drawn exactly, upright and correctly. Same characters as in the reference images. Background: deep midnight blue to purple flat night sky with big golden stars and a big cream moon (night version of the game). Subject: the same girl as in the reference image (long straight dark-brown hair, big brown eyes, white T-shirt with blue stripes) smiling, holding up a glowing yellow-orange lantern that casts a round warm light, big golden glowing letters A B C floating in the sky above her, and two pairs of friendly big softly glowing yellow-green animal eyes peeking from the dark bushes at the bottom corners (cute, round, never scary).
```

### Negative prompt

```text
random extra letters, garbled or misspelled lettering, captions, watermark, signature, brand names, UI, HUD, buttons, rounded-corner frame, device mockup, border, weapons, cages, horror, scary, creepy, menacing, red eyes, sharp teeth, distorted anatomy, extra legs, extra heads, blurry, low resolution, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

The negative list deliberately does not forbid text, because several prompts need exact letters; check every image for stray or garbled writing.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-08 | A5_paws_to_Z_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 5 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A5_paws_to_Z_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 5 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A3_girl_gate_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 3 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A3_girl_gate_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 3 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A8_fence_L_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 8 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A8_fence_L_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 8 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A1_zebra_L_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 1 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A1_zebra_L_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 1 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A4_lion_book_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 4 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A4_lion_book_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 4 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A2_giraffe_L_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 2 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A2_giraffe_L_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 2 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A7_magnifier_eyes_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 7 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A7_magnifier_eyes_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 7 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A6_abc_blocks_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 6 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A6_abc_blocks_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 6 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A5_paws_to_Z_v3.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 5 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | A5_paws_to_Z_v4.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 5 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | B1_zebra_lettering_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 9 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | B1_zebra_lettering_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 9 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | B3_paw_lettering_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 11 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | B3_paw_lettering_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 11 + negative as 'Avoid' + extra text, ref: three_quarter.png | generated, to review |
| 2026-10-08 | B2_emblem_no_text_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 10 + negative as 'Avoid' + extra text, ref: three_quarter.png, three_quarter.png, three_quarter.png | generated, to review |
| 2026-10-08 | B2_emblem_no_text_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 10 + negative as 'Avoid' + extra text, ref: three_quarter.png, three_quarter.png, three_quarter.png | generated, to review |

Image generation: all 2K 1:1 (icons) / 16:9 (wordmarks) via `tools/gen_image.py`; per icon 2 variants (v1/v2); the log rows above were appended by the tool (parallel runs may interleave them).

## Choices (nothing approved)

| Item | Chosen | Note |
|---|---|---|
| A1 | v1 | cleaner L stripe than v2 |
| A2 | v1 | v2 has a blurred ground smear |
| A3 | v1 | |
| A4 | v1 | |
| A5 | v3, edited | v1-v4 all had blurry smudges instead of paw prints at the bottom (the 'no ground' retry v3/v4 failed the same way; max retries reached). Final = PIL crop of v3 (Z + zebra, paws dropped) on flat blue: `A5_paws_to_Z_final.jpg` |
| A6 | v1 | letters A B C correct |
| A7 | v2 | v1 had baked white rounded corners |
| A8 | v2 | v1 reads as an E |
| B1 | v1 | model lettering correct ("Letter Zoo"); shadow smudge below cropped away |
| B2 | v1 | emblem without text; "Letter Zoo" / "Buchstabenzoo" typeset with PIL in **Fredoka Bold** (SIL OFL 1.1, from npm `@fontsource/fredoka` 5.3.0, licence text kept with the package; not committed) |
| B3 | v2 | model lettering correct ("Buchstabenzoo"); v1 had a smudge |

Build: `FREDOKA=<path to fredoka-latin-700-normal.woff> python3 make_logos.py` (icons `icon_A1..A8.png` 1024, wordmarks) and `python3 make_compare.py` (`compare.png`).
| 2026-10-08 | C5_elephant_letters_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 16 + negative as 'Avoid' + extra text, ref: cover_3_reading.png, three_quarter.png | generated, to review |
| 2026-10-08 | C5_elephant_letters_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 16 + negative as 'Avoid' + extra text, ref: cover_3_reading.png, three_quarter.png | generated, to review |
| 2026-10-08 | C2_girl_gold_A_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 13 + negative as 'Avoid' + extra text, ref: cover_3_reading_square.png, three_quarter.png, three_quarter.png | generated, to review |
| 2026-10-08 | C2_girl_gold_A_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 13 + negative as 'Avoid' + extra text, ref: cover_3_reading_square.png, three_quarter.png, three_quarter.png | generated, to review |
| 2026-10-08 | C1_panda_A_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 12 + negative as 'Avoid' + extra text, ref: cover_3_reading.png, three_quarter.png | generated, to review |
| 2026-10-08 | C1_panda_A_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 12 + negative as 'Avoid' + extra text, ref: cover_3_reading.png, three_quarter.png | generated, to review |
| 2026-10-08 | C6_night_girl_lantern_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 17 + negative as 'Avoid' + extra text, ref: cover_2_night.png, three_quarter.png | generated, to review |
| 2026-10-08 | C6_night_girl_lantern_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 17 + negative as 'Avoid' + extra text, ref: cover_2_night.png, three_quarter.png | generated, to review |
| 2026-10-08 | C3_zebra_tower_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 14 + negative as 'Avoid' + extra text, ref: cover_3_reading.png, three_quarter.png | generated, to review |
| 2026-10-08 | C3_zebra_tower_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 14 + negative as 'Avoid' + extra text, ref: cover_3_reading.png, three_quarter.png | generated, to review |
| 2026-10-08 | C4_lion_cub_jump_v1.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 15 + negative as 'Avoid' + extra text, ref: cover_3_reading.png, three_quarter.png | generated, to review |
| 2026-10-08 | C4_lion_cub_jump_v2.jpg | gemini-3-pro-image (2K, 1:1) | — | prompt 15 + negative as 'Avoid' + extra text, ref: cover_3_reading.png, three_quarter.png | generated, to review |

## Round 2 (C-series, user feedback 2026-10-08: "too boring — see an animal, ABC, colourful, one with the girl")

Prompts 12-17 above (C1 panda + A, C2 girl + golden A + zebra, C3 zebra + ABC tower, C4 crowned lion cub, C5 elephant spraying ABC, C6 night girl with lantern). Refs: `cover_3_reading.png` (C2: `cover_3_reading_square.png`; C6: `cover_2_night.png`), `player_girl/three_quarter.png`, animal `three_quarter.png`. All 2K 1:1, 2 variants each, no retries needed. Choices: C1 v2 (v1 had white corner patches top-left), C2 v1, C3 v1, C4 v1, C5 v2, C6 v2 (clearest A B C). Finals `icon_C1..C6.png` (1024, built by `make_logos.py`); `compare.png` now shows the C-series plus A1/A2 next to the current icon (`make_compare.py`).
