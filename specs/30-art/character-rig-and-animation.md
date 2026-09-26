---
id: ART-RIG
title: Character rig and animation — technical contract
aspect: art
module: character-rig-and-animation
status: draft
depends_on: [ART-PIPELINE, GAME-PLAYER, TECH-ARCH]
test_prefix: RIG
updated: 2026-09-26
---

# Character rig and animation — technical contract

## Goal

One shared humanoid skeleton, one animation set and one set of export rules for all human
characters (`player_girl`, `player_boy`, `visitor_*`, `pirate`), so that animations are
reusable, the Rust/WebGL2 renderer can skin them on the GPU within the uniform budget of
mid-range phones, and gameplay can rely on fixed clip names, lengths, events and speeds.

This spec is the contract between the character pipeline (Blender → `.glb`) and the code
(`zoo-assets` loads, `zoo-core` chooses clips and fires events, `zoo-render` samples, blends
and skins). The list of characters and their look lives in ART-CHARACTERS.

## Behaviour

### 1. Coordinate system and units

1. 1 unit = 1 m, Y-up, the character faces **+Z** in glTF (Blender: faces −Y before
   export) — at yaw 0 that is level south; axes per GAME-LAYOUT "Coordinate spaces"
   (Q-056), never mirrored. The character's right side is at **−X**. Suffix `_l`/`_r` always means the
   character's own left/right.
2. The origin is on the ground between the feet (ART-PIPELINE §9). All transforms are
   applied on the mesh before skinning; the armature object has identity transform.

### 2. Skeleton (`human` rig)

1. All human characters use the same **20 joints** with exactly these names and this
   hierarchy (snake_case, glTF node names):

   ```
   root                         ground, between the feet; never animated
   └─ hips                      pelvis, top of the legs
      ├─ spine                  lower torso
      │  └─ chest               upper torso, carries arms and neck
      │     ├─ neck
      │     │  └─ head          whole head incl. hair and face decal
      │     ├─ shoulder_l
      │     │  └─ upper_arm_l
      │     │     └─ lower_arm_l
      │     │        └─ hand_l
      │     └─ shoulder_r
      │        └─ upper_arm_r
      │           └─ lower_arm_r
      │              └─ hand_r
      ├─ upper_leg_l
      │  └─ lower_leg_l
      │     └─ foot_l
      └─ upper_leg_r
         └─ lower_leg_r
            └─ foot_r
   ```

2. **Joint budget:** 20 joints (target ≤ 24, hard max 32). At 1 `mat4` per joint this uses
   80 of the ≥ 256 vertex uniform vectors WebGL2 guarantees, leaving room for the rest of
   the character shader. No extra bones (no fingers, toes, hair, eyelids, jaw).
3. **Rest (bind) pose = A-pose**, identical to the turnaround sheets: standing straight,
   legs straight at hip width, feet parallel pointing +Z, arms straight and rotated
   **45° down from horizontal**, palms facing down/inwards, head looking along +Z.
4. **Reference joint positions for the player characters** (rest pose, metres; x is the
   character's left for `_l`, mirrored for `_r`). `player_girl` and `player_boy` use
   exactly these values so their clips are interchangeable without retargeting:

   | Joint | x | y | z | Notes |
   |---|---|---|---|---|
   | `root` | 0 | 0 | 0 | ground |
   | `hips` | 0 | 0.50 | 0 | top of the legs |
   | `spine` | 0 | 0.56 | 0 | |
   | `chest` | 0 | 0.70 | 0 | |
   | `neck` | 0 | 0.84 | 0 | top of torso |
   | `head` | 0 | 0.86 | 0 | base of the head; head 0.34 m high → top of skull 1.20 m |
   | `shoulder_l` | 0.10 | 0.80 | 0 | |
   | `upper_arm_l` | 0.19 | 0.80 | 0 | shoulder joint |
   | `lower_arm_l` | 0.19 + 0.17·cos45° | 0.80 − 0.17·sin45° | 0 | elbow (upper arm 0.17 m) |
   | `hand_l` | 0.19 + 0.34·cos45° | 0.80 − 0.34·sin45° | 0 | wrist (lower arm 0.17 m) |
   | `upper_leg_l` | 0.075 | 0.48 | 0 | hip joint |
   | `lower_leg_l` | 0.075 | 0.27 | 0 | knee |
   | `foot_l` | 0.075 | 0.07 | 0 | ankle |

   Character height (top of skull) is 1.20 m; hair may add ≤ 0.04 m. Head-to-body ratio
   ≈ 1 : 3.5 (see ART-CHARACTERS for the look). The art style is comic (Q-010 answered,
   `art/style/style.md`); the joint layout does not depend on it.
5. **Bone orientation:** all characters are built from one template armature
   (`assets/blender/characters/_rig_human.blend`), so every joint has the same local axes
   (bone roll) in every character. Only joint *positions* may differ between characters of
   different proportions (e.g. adult visitors, Q-027).
6. **Attachment sockets** are plain glTF nodes (Blender: empties with bone parent), **not
   joints** — they cost no skinning uniforms. Each socket's local +Y points up and +Z
   forward when the character is in rest pose. An attached item is placed with its own
   origin at the socket (item origin = grip point).

   | Socket | Parent joint | Rest position (m) | Used for |
   |---|---|---|---|
   | `socket_hand_r` | `hand_r` | centre of the right fist | small items held in one hand (key, single food item) |
   | `socket_hand_l` | `hand_l` | centre of the left fist | mirror of `socket_hand_r`, reserved |
   | `socket_carry` | `chest` | (0, 0.62, 0.20) | items/animals carried with both hands in front of the belly (food bundle, monkey baby) — Q-025 |
   | `socket_head_top` | `head` | (0, 1.20, 0) | hats and accessories on visitors |

### 3. Mesh and skinning

1. Each character `.glb` contains exactly **one skin** and **one skinned mesh** with two
   primitives: `body` and `face` (see §6). No unskinned extra meshes except items that the
   game attaches at runtime (those are separate assets).
2. **≤ 4 influences per vertex** (`JOINTS_0`/`WEIGHTS_0` only, no `JOINTS_1`). Weights
   are normalised (sum 1 ± 0.001). `JOINTS_0` is stored as `UNSIGNED_BYTE`.
3. **Smooth skinning** for the rounded comic body: elbows, knees, shoulders, hips, spine
   and neck blend over 2–3 joints so they bend without gaps or collapsing volume. Rigid
   (single joint, weight 1.0) parts: every vertex of the head, hair and `face` primitive
   is weighted to `head` only (the face decal must never stretch); hands to `hand_*`;
   shoes to `foot_*`.
4. No morph targets (blend shapes). No scale in the bind pose.
5. **Outlines are not modelled.** The comic ink outline is drawn by the renderer
   (inverted hull or screen-space edge pass — TECH decides, Q-050). The mesh therefore has
   **smooth (averaged) normals** without hard splits, so cel shading bands and a
   normal-extruded hull stay continuous, and no open edges except where the outline
   should show (e.g. sleeve and trouser openings are closed or doubled). No outline
   geometry, no back-face "shell" meshes, no drawn lines in the textures except the face
   decal.

### 4. Animation clips

1. **Authoring:** 30 fps, linear interpolation, sampled every frame on export. Frame counts
   below are the clip length in frames; the clip duration is `frames / 30` s. Looping clips
   have their last frame equal to their first (seamless loop).
2. **Root motion off:** the `root` joint is never animated; the game moves and rotates the
   character. Clips animate **rotation** on all joints except `root`, plus **translation
   on `hips` only** (bob, crouch). No scale channels.
3. **Player clip set** (`player_girl` and `player_boy`, bit-identical clips):

   | Clip | Frames | Duration | Loop | Events (frame) | Crossfade in | Default expression | Purpose |
   |---|---|---|---|---|---|---|---|
   | `idle` | 90 | 3.00 s | yes | — | 0.20 s | `neutral` | Standing, breathing, small weight shift. |
   | `walk` | 24 | 0.80 s | yes | `footstep_l` (0), `footstep_r` (12) | 0.15 s | `neutral` | Walk cycle, authored for **1.4 m/s** (Q-024). |
   | `run` | 16 | 0.53 s | yes | `footstep_l` (0), `footstep_r` (8) | 0.15 s | `happy` | Run cycle with flight phase, authored for **3.0 m/s** (Q-024). |
   | `pick_up` | 24 | 0.80 s | no | `pick_up_grab` (12) | 0.10 s | `neutral` | Bend and grab with both hands; at the event frame the hands are at ~0.40 m height, 0.35 m in front of the origin. |
   | `give` | 24 | 0.80 s | no | `give_release` (14) | 0.10 s | `happy` | Hold the carried item forward to an animal (~0.50 m high, 0.40 m in front) and let go. |
   | `talk` | 60 | 2.00 s | yes | — | 0.20 s | `talk` ↔ `neutral` alternating | Talking to a visitor: small hand gestures, head nods. |
   | `cheer` | 45 | 1.50 s | no | `cheer_peak` (15) | 0.10 s | `laugh` | Success: jump with both arms up (animal happy, quest step done). |
   | `wave` | 40 | 1.33 s | no | — | 0.15 s | `happy` | Greeting; character-choice preview when tapped (GAME-PLAYER §1). |
   | `carry` | 30 | 1.00 s | yes | — | 0.15 s | — | **Upper-body layer**: both arms hold an item at `socket_carry`. See §4.6. |

4. **Visitor and pirate clip set** (shared hierarchy, own rest pose if proportions differ):

   | Clip | Frames | Duration | Loop | Events (frame) | Crossfade in | Default expression |
   |---|---|---|---|---|---|---|
   | `idle` | 90 | 3.00 s | yes | — | 0.20 s | `neutral` |
   | `talk` | 60 | 2.00 s | yes | — | 0.20 s | `talk` ↔ `neutral` |
   | `point` | 45 | 1.50 s | no | `point_hold` (18) | 0.15 s | `happy` |

   `point` extends the right arm towards local +Z; the game rotates the visitor to face the
   hinted direction before playing it. Whether visitors also walk is open (Q-027).
5. **Transitions:** switching clips crossfades linearly over the clip's "crossfade in"
   duration (per-joint rotation nlerp/slerp, hips translation lerp). A one-shot clip
   (`pick_up`, `give`, `cheer`, `wave`, `point`) plays once, holds its last frame for 0
   frames and then returns to the current locomotion clip (`idle`/`walk`/`run`) with a
   0.15 s crossfade. A new request for the same one-shot while it plays restarts it.
6. **Carry layer:** while the player carries something, `carry` is blended **on top of**
   the locomotion clip for the masked joints `shoulder_*`, `upper_arm_*`, `lower_arm_*`,
   `hand_*` (weight 1 for those joints, 0 for all others), fading in/out over 0.15 s.
   `pick_up` and `give` override the layer while they play.
7. **Locomotion speed matching:** `walk` and `run` play at rate
   `actual_speed / authored_speed`, clamped to [0.8, 1.4] (raised from 1.25 for the path speed
   of 1.93 m/s, GAME-PLAYER §6). Between the walk and run
   speed ranges the game switches `walk` → `run` when the speed rises above 2.4 m/s and
   `run` → `walk` when it falls below 2.0 m/s (hysteresis). Below 0.1 m/s the character
   is in `idle`.
8. **Events** are not stored in the `.glb`. They are defined in the table above and
   mirrored in the data file `assets/models/characters/human_anims.toml` (clip name →
   frames, loop, crossfade, events, default expression, authored speed). `zoo-core` fires
   events from clip time (deterministic, testable without a renderer): an event fires once
   when playback crosses its frame; looping clips fire it again every cycle.

### 5. Clip ownership and budgets

1. Per character `.glb`: ≤ 10 clips, only the clips listed for it (ART-CHARACTERS and
   `assets/manifest.toml` must match this spec). Triangles ≤ 3 000 (target ≤ 2 500 — the
   rounded comic shapes need more than boxes, but the character is small on screen under
   the high camera, Q-049); textures: body flat-colour atlas ≤ 256 × 256 plus face decal
   atlas ≤ 256 × 128; `.glb` file size ≤ 400 KB including textures.
2. Player clips are authored once on `player_girl` and copied unchanged to `player_boy`
   (identical rest pose, §2.4).

### 6. Facial expressions

1. Faces are **hand-drawn comic face decals (texture swap), not bones**. The `face`
   primitive is a thin patch that follows the curved front of the head, slightly offset
   from it (≈ 2 mm, no z-fighting), whose UVs point to cell 0 (`neutral`) of the face
   atlas. It contains eyes, brows and mouth drawn with ink lines; skin, cheeks and nose
   shape belong to the body mesh. The renderer selects an expression by adding a per-draw
   UV offset and draws the patch with alpha test (cut-out) after the head.
2. Face decal atlas: 8 cells of 64 × 64 px in a 4 × 2 grid (256 × 128 px), linear
   filtering with mipmaps, ≥ 4 px transparent padding around each drawing so mip levels
   do not bleed, in this order: `neutral`, `blink`, `happy`, `laugh`, `talk`, `surprised`,
   `thinking`, `sad`. Big comic eyes, bold readable shapes; drawn in the style of
   `art/style/style.md`. Texture approach for the body: Q-026.
3. Under the high game camera (Q-049) the face is only a few pixels large; expressions
   matter in close-ups (character choice screen, dialogue, Q-051) and must stay readable
   there. Gameplay readability comes from silhouette, pose and colour (ART-CHARACTERS).
4. Each clip has a default expression (§4.3); gameplay may override it (e.g. `surprised`
   when a hint is found, `sad` never for the player on wrong food — only gentle
   `thinking`, cf. Q-014).
5. **Blink:** while the expression is `neutral` or `happy`, the game shows `blink` for
   0.12 s at random intervals of 3–6 s from the seeded RNG.
6. `talk` ↔ `neutral` alternation during `talk` clips switches every 0.15 s.

### 7. Export settings (Blender glTF exporter)

| Setting | Value |
|---|---|
| Format | glTF Binary (`.glb`) |
| Include | the character's armature, mesh and socket empties only |
| Transform | +Y up |
| Mesh | apply modifiers; UVs; smooth (averaged) normals, no auto-smooth splits; no vertex colours; no tangents |
| Materials | export; exactly `body` (flat-colour atlas) and `face` (face decal atlas, alpha), PNG embedded; unlit/flat base colour only — cel shading and outline come from the renderer |
| Compression | none (no Draco, no meshopt) |
| Skinning | on, "only deform bones", ≤ 4 influences, no rest-pose bake of animations |
| Shape keys | off |
| Animation | mode "Actions", one action per clip, action name = clip name, sample every frame, "always sample" on, "optimise animation size" on, force-export no scale channels |
| Custom properties | off (events come from `human_anims.toml`) |

The exported file is `assets/models/characters/<asset_id>.glb`; the source is
`assets/blender/characters/<asset_id>.blend` (ART-PIPELINE).

### 8. Reference implementation (model v1)

- Scripts: `tools/blender/characters/human_rig.py` (skeleton, weights, clips, export) and
  `tools/blender/characters/player_girl.py` (look). The template armature is written to
  `assets/blender/characters/_rig_human.blend` on every run (§2.5); all bones have roll 0.
- Clips are generated procedurally: legs by two-bone IK with ground-locked feet
  (heel-roll / flat / toe-roll), so the `walk`/`run` planted-foot drift is 0 mm at
  the authored speed (RIG-010); every frame is keyed with linear interpolation.
- Body texture: 64 × 64 atlas of 8 × 8 flat cells (8 px), per character, sampled
  NEAREST (Q-026 option a). Face atlas: 256 × 128, 4 × 2 cells of 64 px, LINEAR +
  mipmaps, alpha MASK (cutoff 0.5).
- `face` patch: the head's own vertex grid (azimuth ± 67.5°, elevation −54° … +36°)
  pushed 2 mm out along the ellipsoid normal, rigid on `head`.
- Checker: `python3 tools/blender/check_character.py` covers RIG-001…008, 010, 011, 018,
  021, 022 on the exported `.glb`.

## Acceptance criteria

- Both player `.glb` files load in `zoo-assets`, have the 20-joint `human` skeleton with
  identical rest pose, contain exactly the player clip set, and skin correctly in the
  game with GPU skinning at 60 fps on a mid-range phone.
- `human_anims.toml` matches the tables in this spec, and all events fire at the listed
  frames in `zoo-core` tests.
- Walking and running show no visible foot sliding at the authored speeds.
- Expression cells switch without visible filtering seams or bleeding from neighbour cells.
- With the renderer's cel shading and outline, elbows and knees bend without gaps or
  broken outlines.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| RIG-001 | Given each human character `.glb`, then its skin has exactly the 20 joint names of §2.1 with the listed parent of each joint. | asset |
| RIG-002 | Given each human character `.glb`, then its joint count is ≤ 24 (hard fail > 32). | asset |
| RIG-003 | Given each human character `.glb`, then every vertex has ≤ 4 non-zero weights, weights sum to 1 ± 0.001, every joint index is < joint count, no `JOINTS_1`/morph targets exist, and every vertex of the `face` primitive is weighted 1.0 to `head`. | asset |
| RIG-004 | Given `player_girl.glb` and `player_boy.glb`, then every joint's rest local translation and rotation are equal within 0.001 m / 0.1°, and match the table in §2.4 within 0.005 m. | asset |
| RIG-005 | Given each human character `.glb`, then its bind pose is the A-pose: `upper_arm_l`→`hand_l` direction is 45° ± 3° below horizontal, mirrored for `_r`; legs vertical ± 2°. | asset |
| RIG-006 | Given each clip, then `root` has no animation channel, only `hips` has translation channels, and no clip has scale channels. | asset |
| RIG-007 | Given each human character `.glb`, then its clip names equal the manifest `animations` list for that asset and each clip's duration is `frames/30` ± 1/30 s per the tables in §4. | asset |
| RIG-008 | Given each looping clip, then the pose at the first and last frame differ by ≤ 0.5° per joint and ≤ 0.002 m on `hips`. | asset |
| RIG-009 | Given `human_anims.toml`, then it lists exactly the clips of §4.3/§4.4 with the same frames, loop flag, crossfade, events and default expression; every event frame is < the clip's frame count. | unit |
| RIG-010 | Given `walk` playing for one cycle while the character moves at 1.4 m/s (and `run` at 3.0 m/s), then during each foot's ground-contact frames (foot joint lowest), the foot's world-space horizontal drift is ≤ 0.03 m. | asset |
| RIG-011 | Given each human character `.glb`, then nodes `socket_hand_r`, `socket_hand_l`, `socket_carry`, `socket_head_top` exist, are children of the joints in §2.6, and are not in the skin's joint list. | asset |
| RIG-012 | Given `walk` played from t = 0 for 0.85 s, then `zoo-core` emits `footstep_l` at 0.0 s and 0.8 s and `footstep_r` at 0.4 s, each exactly once per crossing. | unit |
| RIG-013 | Given `pick_up` requested once, then `pick_up_grab` fires once at 0.40 s and after 0.80 s the active clip is the current locomotion clip. | unit |
| RIG-014 | Given `idle` active, when `walk` is requested, then at 0.075 s the sampled pose of each joint is the 50 % blend of both clips (± 0.5°), and at 0.15 s it equals `walk`. | unit |
| RIG-015 | Given the carry layer active over `walk`, then masked arm joints equal the `carry` pose and all other joints equal the `walk` pose (± 0.1°). | unit |
| RIG-016 | Given a player speed of 1.2 m/s, then `walk` playback rate is 0.857; at 1.93 m/s it is 1.38; at 2.2 m/s it is clamped to 1.4; at 2.5 m/s the active clip is `run`, decelerating to 2.2 m/s keeps `run`, and at 1.9 m/s it is `walk` again. | unit |
| RIG-017 | Given seed S and 60 s of `idle`, then the blink times are identical across runs, every interval is within 3–6 s and each blink lasts 0.12 s. | unit |
| RIG-018 | Given each human character `.glb`, then it has one skinned mesh with primitives using materials `body` and `face`, the face UVs lie inside cell 0 of the 256 × 128 face atlas, the body texture is ≤ 256 × 256, triangle count ≤ 3 000 and file size ≤ 400 KB. | asset |
| RIG-019 | Given the in-game character viewer, when each clip plays on each character, with cel shading and outline on, then no limb visibly passes through the body, elbows/knees/shoulders bend without gaps, collapsing or broken outlines, and expressions match §4.3 (review checklist). | manual |
| RIG-020 | Given the turnaround sheets and the exported `.glb` rendered from front and side in rest pose, then the silhouettes match (proportions within ~5 %). | manual |
| RIG-021 | Given each human character `.glb` in rest pose, then the centroid of the `face` primitive has z > 0 and y > 0.86 (character faces +Z), `upper_arm_r` has x < 0, and the armature node has identity transform. | asset |
| RIG-022 | Given each human character `.glb`, then the `body` primitive has no duplicated positions with differing normals (smooth normals, §3.5) and no outline/shell mesh exists. | asset |

## Open questions

- Q-009 Who creates the turnaround images (affects when modelling can start).
- Q-010 Art style — answered 2026-09-26: comic style (`art/style/style.md`); smooth skinning (§3.3), outlines by renderer (§3.5).
- Q-024 Walking and running speed of the player (blocking — clips are authored to it).
- Q-025 How the player carries food and the monkey baby (blocking — `carry` clip, sockets).
- Q-026 Texture approach for body and faces (blocking — renderer needs to know).
- Q-027 Visitor proportions and whether visitors walk.
- Q-029 Is the player clip set complete?
- Q-049 High-angle game camera (characters small on screen).
- Q-050 Outline technique for characters (inverted hull vs. screen-space).
- Q-051 Close-up for dialogue so faces/expressions are visible.
- Q-063 Walk cadence vs. speed (hips dip needed for no foot sliding at 1.4 m/s).
- Q-042 `give` releases the item, but showing food must not consume it (GAME-FEED §4).
