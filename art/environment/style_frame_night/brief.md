# Brief — `style_frame_night` (the night look)

Spec: GAME-NIGHT (rules 1, 2, 10 — dusk → night, friendly never scary, night lighting as a renderer mode), ART-PIPELINE §5 (style frame first), ART-DIRECTION. Style: `art/style/style.md` (comic). Reference image: the approved day style frame `art/environment/style_frame/style_frame.png`. Status: **brief** (not generated — Gemini monthly spending cap, HTTP 429, 2026-09-26).

## Purpose

The **same scene as the approved day style frame** (zebra enclosure from the high 55° game camera, girl on the path) **at night**, to pin down the night look **before any night mockup** (`night_1` environment) is made and before the renderer's night mode (GAME-NIGHT §10) is tuned. Once approved it is the style reference for every night image (with the day style frame).

| File | Content |
|---|---|
| `style_frame_night_v1.jpg`, `style_frame_night_v2.jpg` | two candidates |
| `style_frame_night.png` | the approved one (copy), ≤ 2048 px |

## Night look (proposal, to be confirmed by this frame)

- **Two lights only:** cool blue moonlight (ambient + key from the upper left, one hard blue shadow tone) and **warm yellow-orange lantern pools** with hard cartoon falloff (matches the renderer: global light colour + point lights, GAME-NIGHT §10).
- **Never scary** (rule 2): deep *friendly* blue, no black areas, everything readable; outlines and flat colours stay.
- **Player always visible:** soft warm light circle around the girl (she carries a lantern — GAME-NIGHT §5).
- **Stars reflected on water** (rule 1) — no sky in the camera, so stars appear only as reflections; fireflies as extra warm sparkles.
- **Boards readable:** the info board has its own small lamp (§5 "night boards").
- **Glowing eyes:** the zebra's eyes glow softly — the NIGHT-006 eyeshine, tested here on a day animal.

## Style-block note (APIPE-010)

The STYLE block is copied verbatim and therefore says "Bright warm midday sunlight". The scene paragraph states explicitly that the night lighting **replaces** it. If Gemini keeps producing daylight, the fix is a night variant of the STYLE block in `art/style/style.md` (never only here) — decision for the user.

## Image settings

- **16:9, 2K**, `gemini-3-pro-image`, 2 variants, reference = the day style frame, `--extra "The reference image is the approved day version of this exact scene — keep its composition, camera, objects, girl and comic style exactly, only change the lighting to the night described above."`
- `tools/gen_image.py art/environment/style_frame_night/brief.md --prompt 1 --out style_frame_night_v1.png style_frame_night_v2.png --aspect 16:9 --size 2K --ref art/environment/style_frame/style_frame.png --extra "…"`

## Prompt (`style_frame_night.png`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Elevated three-quarter top-down view like a cozy zoo park simulation game, horizontal widescreen composition: a high follow camera looking down at about 55 degrees from about 16 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, the ground fills the image, no horizon, no sky.

NIGHT VERSION of the reference image: exactly the same zebra-enclosure scene, composition and camera as the reference image, but late in a calm, friendly night. This lighting replaces the midday sunlight described above: no sun; soft moonlight from the upper left gives a deep friendly blue night palette — medium deep blue and blue-violet for grass, hedge and trees, soft lighter blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object is still clearly visible and keeps its bold dark outlines and flat colours. Warm yellow-orange lantern light is the second light: four or five friendly old-fashioned lanterns on wooden posts stand along the light sand-beige path, each glowing warm yellow and casting a round, hard-edged pool of warm light on the path; the info board has a small warm lamp on top so its blank face is well lit. In the middle of the image the player character stands on the path, small (about one twelfth of the image height): the same small girl as in the reference (long straight dark-brown hair, white T-shirt with blue stripes, blue jeans), carrying a small glowing lantern; around her a soft round circle of warm light on the ground so she is always clearly visible. The zebra stands near the fence, relaxed and sleepy, its big friendly eyes with a small soft glow; the stone arch shelter has a warm glowing lamp. No sky is visible, but stars are reflected as small white sparkles in a small puddle or in the water trough by the fence, and a few tiny warm yellow fireflies float above the hedge. The mood is cosy, magical and safe like a bedtime story for young children — warm lanterns against soft blue, nothing scary, no deep black shadows. All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers; the only marking allowed is the simple solid black zebra silhouette on the enclosure sign, readable in the lantern light.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, daylight, sunshine, midday sun, pitch black, deep black shadows, unreadable dark areas, horror, scary, spooky, halloween, creepy, menacing shapes, glowing red eyes, monsters, ghosts, fog, mist, thunderstorm, rain, angry or menacing animals, sharp teeth, cages, cage bars, rubbish, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, blurry, low resolution, cropped main subject, sky, horizon, moon in the sky, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (user approves)

- [ ] Same scene, camera and comic style as the day style frame (outlines, flat colours, one shadow tone).
- [ ] Night reads immediately (blue), but **nothing is dark enough to hide anything**; no black areas.
- [ ] Friendly and cosy for 4–6 year olds — nothing scary (NIGHT-009).
- [ ] Lantern pools warm and hard-edged (buildable as point lights with cartoon falloff).
- [ ] The girl is clearly visible in her light circle; the info board is lit and readable.
- [ ] Stars reflected on water; zebra eyes glow softly (eyeshine look for NIGHT-006).
- [ ] On approval: copy to `style_frame_night.png`, set `approved` in `art/catalog.js` and `concept_approved = true` in the manifest.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
