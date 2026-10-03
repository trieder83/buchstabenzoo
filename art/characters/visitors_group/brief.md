# Brief — `visitors_group` (size comparison of the five visitors with the player girl)

Spec: ART-CHARACTERS (ACHAR-003 distinct palettes), ART-PIPELINE §3. Status: **concept — in review**. Written 2026-10-03.
`lineup.jpg` is composed by script from the chosen front views of `player_girl` and the five `visitor_*` at their nominal heights (1.20 / 1.08 / 1.22 / 1.60 / 1.68 / 1.75 m) with 0.5 m grid lines — it is the exact size reference. `group_v1.jpg` / `group_v2.jpg` are generated redraws for the review page only (they are not used for modelling).

**Negative prompt:**

```text
T-pose, text, logo, brand, watermark, scenery, gradient background, cropped feet, extra figures, fewer than six figures, thin stripes, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### Prompt 1 — group shot (16:9, 2K, reference `lineup.jpg`)

`--extra`: "The attached image is the exact lineup of these characters: copy them exactly."

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Group lineup of six characters standing side by side in one row on the same baseline, all facing the camera in a relaxed friendly standing pose, in this exact order from left to right with exact relative heights: the player girl (1.20 m, long dark brown hair, white and blue striped T-shirt, jeans), the 5-year-old boy visitor (1.08 m, orange cap, violet T-shirt, small red backpack, plush lion), the 7-year-old girl visitor (1.22 m, two braids with yellow bobbles, magenta-pink dress with turquoise hem), the grandmother (1.60 m, silver-white curly hair, ochre cardigan, camera around the neck), the mother (1.68 m, straw sun hat, coral-red dress, striped tote bag), the father (1.75 m, round glasses, teal polo shirt, orange backpack, beige shorts). Copy every character exactly from the attached lineup image, keep their relative sizes, the five visitors have clearly different main colours.
```

## Generation log

| Date | File | Tool | Seed | Prompt | Result |
|---|---|---|---|---|---|
| 2026-10-03 | group_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: lineup.jpg | generated, to review |
| 2026-10-03 | group_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: lineup.jpg | generated, to review |
