# Buchstabenzoo — canonical art style (comic)

**Single source of truth for every image prompt.** Decided 2026-09-26 (Q-010): the game
looks **comic-like**. Every prompt in `art/**/brief.md` copies the blocks below **verbatim**
(copy-paste, no paraphrasing, no additions inside the block). Test APIPE-010 checks this.

To change the style: edit the blocks here, then replace them in every brief in the same
change (search for the old block text).

Camera for all game scenes: high-angle follow camera (~55° down, isometric-like, no sky) — the zoo view; the close views (first person, look-around) add a comic sky and haze (GAME-CAMERA-VIEWS)
— see GAME-PLAYER and the camera paragraph in each environment brief.

## STYLE — scenes (environment, style frame, mockups)

Always the **first paragraph** of every scene prompt.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.
```

## CHARACTER SHEET STYLE — turnarounds and expression sheets

Always the **first paragraph** of every character/animal sheet prompt.

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.
```

## NEGATIVE — style suffix

Append to the end of **every** negative prompt (after the brief-specific part).

```text
voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Renderer implications (for TECH / ART-DIRECTION)

- Cel shading (2 tones + optional rim) in the WebGL2 shader, not baked into textures.
- Outlines drawn by the renderer (inverted hull or screen-space edge pass), not modelled.
- Flat colour textures / vertex colours; faces as small hand-drawn decal textures.
