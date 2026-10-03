# Brief — `visitor_adult_2`

Spec: ART-CHARACTERS (visitor types), ART-RIG (skeleton `biped`, visitor clips), GAME-ECON (visitors, Q-300/Q-307), ART-PIPELINE §3. Style: **comic** — `art/style/style.md`.
Status: **concept — in review, `concept_approved` not set** (no `assets/manifest.toml` entry until the user approves, like the other unapproved assets). Written 2026-10-03.
Family/group mix of the five visitors: mother (`visitor_adult_1`), father (`visitor_adult_2`), grandmother (`visitor_adult_3`), girl (`visitor_child_1`), boy (`visitor_child_2`) — group shot `art/characters/visitors_group/`.

## Character

- **Who:** father, ~38, carries a small backpack, glasses; height 1.75 m, head-to-body 1 : 5.5.
- **Look:** a friendly 38-year-old man with dark brown skin, short black hair, round black-framed glasses, a teal (#2A7F8E) polo shirt with a white collar, beige knee-length cargo shorts, white sneakers with grey soles, and a small orange (#F28C28) backpack with two shoulder straps on his back, kind dark eyes behind the glasses, warm broad smile, slightly broad shoulders.
- **Distinct from the player:** no white/blue striped shirt, no jeans, no stripes at all; visitors may wear hats and bags (players never do, ACHAR-006). Main colour differs from the other four visitors and from the player (ACHAR-003).
- **Silhouette from the 55° camera at ~14 m:** the orange backpack on the back and the teal shoulders read from above.
- **Pose for all views:** A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width.

## Colours (map to the shared palette at modelling time)

| Part | Hex |
|---|---|
| Polo (main) | `#2A7F8E` |
| Collar | `#FFFFFF` |
| Shorts | `#D9C6A0` |
| Backpack | `#F28C28` |
| Skin / shade | `#8A5A3C / #6E452D` |
| Hair | `#1E1612` |
| Glasses | `#2B1B12` |
| Sneakers | `#F4F4F0` |

## Image-generation prompts

Sheet: **21:9, 2K**, reference `art/characters/player_girl/sheet_v1.jpg` for layout, line weight, eye style and background only. `--extra`: "The attached image is the approved turnaround sheet of the player character of this game: copy its layout, comic style, line weight, eye style, proportions logic and plain grey background exactly, but draw the different character described here (not the girl from the attached image)."

**Negative prompt:**

```text
T-pose, dynamic pose, text, logo, brand, watermark, scenery, gradient background, cropped feet, extra fingers, thin stripes, stripes on clothes, extra figures, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### Prompt 1 — turnaround sheet (`sheet_v1.jpg`, `sheet_v2.jpg`)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround with four views side by side in one row, same scale and same baseline, full body from head to toe in each: front, exact left side profile (faces the left edge), back (no face visible), three-quarter front (turned 45 degrees to the left). A friendly 38-year-old man with dark brown skin, short black hair, round black-framed glasses, a teal (#2A7F8E) polo shirt with a white collar, beige knee-length cargo shorts, white sneakers with grey soles, and a small orange (#F28C28) backpack with two shoulder straps on his back, kind dark eyes behind the glasses, warm broad smile, slightly broad shoulders. Large round head, chunky rounded body, 1 : 5.5 head-to-body proportion (1.75 m tall, a grown-up among children's-book proportions), round mitten-like hands with a thumb. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width in every view, friendly neutral smile. Clothes and accessories are identical in all four views. ADULT BODY PROPORTIONS, clearly NOT a child: the head is only about one fifth of the total body height, a smaller head with a mature adult face, narrow adult shoulders and torso, long adult legs making up about half of the body height, a full head taller than a child.
```

### Prompt 2 — in-game size view (`ingame_v1.jpg`)

Reference: the chosen sheet. 16:9, 1K. `--extra`: "The attached image is the character sheet of this character: draw exactly this character."

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game size view of the same character: the same father, ~38, carries a small backpack, glasses standing on light sand-beige zoo path next to fresh green grass, seen from a high camera about 55 degrees down (isometric-like, the same view as the game), the character small in the frame (about 12 percent of the image height) so that silhouette and colours must read at a glance; the orange backpack on the back and the teal shoulders read from above. Three copies side by side in the image: front-facing, turned away from the camera, and walking to the right, all the same small size, no other people, no animals, no buildings, flat colours with one hard shadow tone and bold dark-brown outlines, bright warm midday sunlight from the upper left.
```

## Rig and animation notes (for modelling after approval)

- Rig: the shared `biped` skeleton of ART-RIG, own rest pose scaled to 1.75 m (hip/shoulder heights scaled from the player's by height / 1.20 m; adults get a longer neck-to-hip ratio, ART-RIG §2.4 "own rest pose if proportions differ"). Same bone names and hierarchy as the player (ACHAR-001).
- Clips (visitor set, ART-RIG §4.4): `idle`, `talk`, `point`; GAME-ECON adds `walk`, `wave`, `buy` (hold out coin / take a cone), optional `eat_ice`. All follow ART-RIG naming, 30 fps, root motion off.
- Walk speed: documented in m/s per character (adults ~1.2, children ~1.0 m/s) so foot sliding matches.
- Face: decal atlas, same 8 expressions (ART-RIG §6), drawn per character.
- Props as separate rigid parts on sockets: `socket_head_top` (hat / cap), `socket_back` (backpack / tote), `socket_hand_r/l`; balloon / cone / plush are prop models, not part of the body mesh.
- Budget: ≤ 2 156 tris body (= `player_girl`), ≤ 2 600 tris including hat/bag props; ≤ 20 joints; body atlas + face atlas, no extra textures.

## Generation log

| Date | File | Tool | Seed | Prompt | Result |
|---|---|---|---|---|---|
| 2026-10-03 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | generated, to review |
| 2026-10-03 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | generated, to review |
| 2026-10-03 | sheet_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | generated, to review |
| 2026-10-03 | sheet_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | generated, to review |
| 2026-10-03 | ingame_v1.jpg | gemini-3-pro-image (1K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: sheet_v4.jpg | generated, to review |
