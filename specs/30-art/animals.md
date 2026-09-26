---
id: ART-ANIMALS
title: Animals — concept and models
aspect: art
module: animals
status: draft
depends_on: [ART-PIPELINE, ART-DIRECTION, GAME-ANIMALS, ART-RIG]
test_prefix: AANI
updated: 2026-09-26
---

# Animals — concept and models

## Goal

Every animal in GAME-ANIMALS has an approved turnaround sheet before it is modelled.

## Asset list

Candidate list — same 10 animals as GAME-ANIMALS / CONT-MISSIONS; final scope depends on
Q-002. Animations per animal are provisional until Q-043.

| Asset id | Animal | Variants | Animations | Status |
|---|---|---|---|---|
| `hippo` | Flusspferd | adult | `idle`, `walk`, `eat`, `swim`, `happy`, `refuse` | model v1 |
| `panda` | Panda | adult | `idle`, `walk`, `eat`, `happy`, `refuse` | model v1 |
| `zebra` | Zebra | adult | `idle`, `walk`, `eat`, `drink`, `happy`, `refuse` | model v1 |
| `koala` | Koala | adult | `idle`, `walk`, `eat`, `happy`, `refuse` (`climb` later, Q-043) | model v1 |
| `elephant` | Elefant | adult | `idle`, `walk`, `eat`, `drink`, `happy`, `refuse` | model v1 |
| `goldfish` | Goldfisch | adult | `swim`, `eat` | concept |
| `monkey` | Affe | adult, **baby** (baby only if `quest_monkey_baby` stays — Q-040) | `idle`, `walk`, `climb`, `eat`, `happy`; baby: `hide`, `wave` | concept |
| `giraffe` | Giraffe | adult | `idle`, `walk`, `eat`, `happy`, `refuse` | model v1 |
| `lion` | Löwe | adult | `idle`, `walk`, `eat`, `drink`, `happy`, `refuse` | model v1 |
| `snow_fox` | Schneefuchs | adult | `idle`, `walk`, `eat`, `drink`, `happy`, `refuse` | model v1 |


## Game sizes (user decision 2026-09-26)

Sizes are **comic-scaled for readability** from the 55° game camera, not realistic: small
animals are scaled up (user decision). Height = top of head/ears standing; the player is 1.20 m.

| Animal | Game size | Note |
|---|---|---|
| `zebra` | 2.23 m (withers 1.30 m) | model v1 |
| `hippo` | 1.7 m | model v1 (withers 1.43 m) |
| `panda` | 1.1 m | model v1 (1.07 m, withers 0.84 m) |
| `koala` | **0.9 m** (was 0.65) | **on all fours** (user decision) — uses the quadruped rig and follows on foot; model v1 (0.86 m) |
| `elephant` | 3.0 m | model v1 (withers 2.33 m) |
| `goldfish` | **0.6 m long** (was 0.3) | views side/top/front/¾; own small rig (Q-043) |
| `monkey` | **1.1 m** (was 0.9) | drawn upright on two legs → biped rig (Q-043); baby: Q-040 |
| `giraffe` | 4.5 m | model v1 (4.47 m, withers 2.30 m) |
| `lion` | 1.5 m | model v1 (1.53 m, withers 1.02 m) |
| `snow_fox` | **0.9 m** (was 0.6) | model v1 (0.90 m, withers 0.62 m) |

Concept art for all 10 animals is approved (2026-09-26).

## Behaviour

1. Every animal follows the common animation set `idle`, `walk` (or `swim`/`climb`),
   `eat`, `happy` — the game reacts identically to all animals.
2. Animals are stylised and friendly (big eyes, no teeth shown), same comic style (`art/style/style.md`) as the
   player character.
3. Turnaround for quadrupeds: front, left side, back, ¾ — standing pose, all four feet on
   the ground.

## Rig conventions (quadrupeds)

Technical contract between the animal scripts (`tools/blender/animals/`) and the code, mirroring
ART-RIG for humans. Rules not repeated here (export settings §7, crossfades §4.5, event
firing §4.8, loop seams, sampling) are the same as ART-RIG.

1. **Space:** 1 unit = 1 m, Y-up, the animal faces **+Z** in glTF at yaw 0 (Blender −Y;
   north = Blender +Y, GAME-LAYOUT), never mirrored; `_l`/`_r` = the animal's own
   left/right, left at +X. Origin on the ground between the four hooves/paws (min Y = 0);
   the armature node has an identity transform.
2. **Skeleton `quadruped` (23 joints, ≤ 24, hard max 32):** all four-legged animals use
   these names and parents; only joint *positions* differ per animal (bone roll 0):

   ```
   root                       ground, never animated
   └─ hips                    pelvis (rear of the body); carries the hind legs and the tail
      ├─ spine
      │  └─ chest             front of the body; carries the front legs and the neck
      │     ├─ neck_1 ─ neck_2 ─ head ─ ear_l, ear_r
      │     ├─ front_upper_l ─ front_lower_l ─ front_foot_l
      │     └─ front_upper_r ─ front_lower_r ─ front_foot_r
      ├─ tail_1 ─ tail_2
      ├─ hind_upper_l ─ hind_lower_l ─ hind_foot_l
      └─ hind_upper_r ─ hind_lower_r ─ hind_foot_r
   ```

   **Optional extra joints** (per animal, leaf chains appended to the 23, total ≤ 32): the
   standard joints keep their names and parents. Model v1 uses `trunk_1 ─ trunk_2 ─ trunk_3`
   under `head` (elephant) and `tail_3` under `tail_2` (snow fox big tail); the giraffe's long
   neck uses the standard `neck_1`/`neck_2` with long bones.

   Legs are vertical in the rest pose; `*_upper` starts inside the body, `*_lower` at the
   knee/hock, `*_foot` at the fetlock/ankle (its tail is the toe on the ground). Front
   knees bend forward, hind hocks backward. No jaw (no open mouth, no teeth); ears are
   joints for flicks.
3. **Skinning:** one skin, one skinned mesh, ≤ 4 influences (`JOINTS_0` UNSIGNED_BYTE,
   weights sum 1), smooth blends over body/neck/leg joints; head, eyes and muzzle rigid on
   `head`. Smooth averaged normals, no outline or shell geometry, no morph targets, no
   vertex colours (outline and cel shading come from the renderer).
4. **Material and texture:** one material `body` with a flat-colour atlas ≤ 256 × 256.
   Coat patterns (stripes, spots) are **painted pattern maps** in atlas regions, mapped per
   vertex along each body part — not geometry, not per-face colours — so they stay broad and
   clean at any mesh resolution. Sampler LINEAR + mipmaps (LINEAR_MIPMAP_LINEAR); unused
   atlas space holds the base coat colour so far mip levels do not bleed. Eyes are small
   dome meshes mapped to a drawn eye swatch in the same atlas (no separate face decal;
   animals have no expression swap).
5. **Clips:** 30 fps, linear, sampled every frame; rotation on all joints except `root`,
   translation on `hips` only, no scale. Looping clips end on their first pose. Clip data
   (frames, loop, crossfade, events, authored speed, gameplay reactions) lives in
   `assets/models/animals/animal_anims.toml`, format as `human_anims.toml`. One-shot clips
   return to the current `idle`/`walk`/`drink` with a 0.15 s crossfade.
6. **Locomotion:** no root motion; `walk` is authored for the follow speed **1.4 m/s**
   (same as the player walk, ART-RIG §4.7 rate matching and clamp) with planted hooves that
   do not slide (≤ 0.03 m). Gait: lateral-sequence walk (LH → LF → RH → RF, a quarter cycle
   apart); events `footstep_hl/fl/hr/fr`.
7. **Budget:** ≤ 3 000 triangles (target ~2 000), `.glb` ≤ 400 KB.
8. **Reference implementation:** `tools/blender/animals/quadruped_rig.py` (skeleton, mesh
   builder with pattern-map UVs, atlas writer, leg IK, gait, head-down solver, bake, export,
   preview); `tools/blender/animals/quad_kit.py` (loft/ellipsoid parts, 3D colour fields
   painted into atlas pattern regions, generic weights, generic clips) is the shared kit of
   the v1 animals after the zebra; checker `python3 tools/blender/check_animal.py`
   (AANI-003…006, 008).

## Zebra model (v1)

- Script `tools/blender/animals/zebra.py` → `assets/models/animals/zebra.glb`,
  `assets/blender/animals/zebra.blend`, `assets/textures/animals/zebra_body.png`
  (256 × 256), preview `art/animals/zebra/model_preview.png`.
- Size: back (withers) 1.30 m, top of the ears 2.23 m, length 2.0 m nose to tail tuft,
  width 0.74 m. 2 040 triangles, 23 joints, ~206 KB.
- Look (turnaround): chunky barrel body, short sturdy legs, big round head with big brown
  eyes, black muzzle, upright mane with black top and black forelock, pink ear insides,
  tail with black tuft. 8 broad body stripes that taper to points above a white belly,
  3 neck rings, 2 head bands + forehead line, 5 rings per leg, black hooves.
- Clip set (provisional per the Q-043 recommendation):

  | Clip | Frames | Duration | Loop | Events (frame) | Used for |
  |---|---|---|---|---|---|
  | `idle` | 90 | 3.00 s | yes | — | standing: breathing, tail swish, ear flicks, look around |
  | `walk` | 20 | 0.67 s | yes | `footstep_hl` (0), `footstep_fl` (5), `footstep_hr` (10), `footstep_fr` (15) | following (1.4 m/s) |
  | `eat` | 60 | 2.00 s | no | `eat_bite` (22) | head down grazing, 3 bites (`eat`) |
  | `drink` | 60 | 2.00 s | yes | — | hiding-place idle at the river: head low, lapping |
  | `happy` | 45 | 1.50 s | no | `happy_peak` (15) | small hop with head toss and tail swish (`happy`) |
  | `refuse` | 36 | 1.20 s | no | — | head shake, ears back, lean back (`refuse`, `not_interested`) |

## Models v1 (hippo, panda, koala, elephant, giraffe, lion, snow_fox)

- Scripts `tools/blender/animals/<id>.py` (built with `quad_kit.py`) → `assets/models/animals/<id>.glb`,
  `assets/blender/animals/<id>.blend`, `assets/textures/animals/<id>_body.png` (256 × 256), preview
  `art/animals/<id>/model_preview.png` (55° game view | front | side | game size).
- Common clips (same meaning and events as the zebra): `idle` 90 f loop, `walk` loop (planted feet
  at 1.4 m/s, `footstep_*` a quarter cycle apart), `eat` 60 f (`eat_bite` 22; elephant 30 — trunk
  curls to the mouth), `happy` 45 f (`happy_peak` 15), `refuse` 36 f; `drink` 60 f loop (elephant,
  lion, snow fox); `swim` 48 f loop (hippo: floating, legs paddle, head up — the game sinks the model
  to ~0.9 m so back, eyes and ears stay above the water). Frames per animal are in
  `animal_anims.toml`.

  | Animal | Tris | Joints | Walk frames | Look (key features at ~60 px) |
  |---|---|---|---|---|
  | `hippo` | 1 796 | 23 | 20 | lilac barrel body, wide pink muzzle, eyes and tiny ears on top, pink belly, cream toenails |
  | `panda` | 1 756 | 23 | 16 | white body, black legs + shoulder band, black round ears, big black eye patches |
  | `koala` | 1 712 | 23 | 12 | grey, huge round ears (off-white inside), big black oval nose, white chest |
  | `elephant` | 1 792 | 26 | 28 | blue-grey, huge cupped ears (pink inside), 3-joint trunk curling at the tip, no tusks |
  | `giraffe` | 2 004 | 23 | 28 | long neck, few large brown patches (3D cell pattern), ossicones, `eat` with the head high |
  | `lion` | 2 308 | 23 | 20 | golden body, big scalloped orange-brown mane ball, cream muzzle/bib/paws, tail tuft |
  | `snow_fox` | 1 924 | 24 | 12 | snow-white, big pointed ears (pink inside), huge plume tail (`tail_3`), pale blue-grey lower legs |

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| AANI-001 | Given every animal in GAME-ANIMALS, then an asset with the same id exists in the manifest. | asset |
| AANI-002 | Given each animal `.glb`, then it contains `eat` and `happy` plus at least one locomotion animation. | asset |
| AANI-003 | Given each quadruped `.glb`, then its skin has exactly the 23 `quadruped` joint names with the parents of "Rig conventions" §2 plus only its listed optional extra joints (elephant `trunk_1..3`, snow fox `tail_3`; ≤ 32 joints in total). | asset |
| AANI-004 | Given each animal `.glb`, then it has one skinned mesh with material `body` (texture ≤ 256 × 256), every vertex has ≤ 4 weights summing to 1 ± 0.001 with `JOINTS_0` UNSIGNED_BYTE, no morph targets / vertex colours / `JOINTS_1`, no duplicated positions with differing normals, ≤ 3 000 triangles and ≤ 400 KB. | asset |
| AANI-005 | Given each quadruped `.glb` in rest pose, then min Y = 0 ± 0.01, the back height matches its model section ± 0.05 m (zebra 1.30 m), `head` is at +Z and `tail_1` at −Z, `*_l` legs at +X, front legs in front of hind legs, legs vertical ± 2°, and the armature node has identity transform. | asset |
| AANI-006 | Given each animal `.glb`, then its clip names equal the manifest `animations` and the `animal_anims.toml` clips, each clip lasts `frames/30` ± 1/30 s, `root` is not animated, only `hips` has translation, no scale channels, looping clips' first/last poses differ ≤ 0.5° / 2 mm, and every event frame is < the clip's frame count. | asset |
| AANI-007 | Given `animal_anims.toml`, then it lists exactly the clips of each animal's model table with the same frames, loop flag, events and `used_for`; `walk` has `speed = 1.4`. | unit |
| AANI-008 | Given `walk` playing for one cycle while the animal moves at its authored speed, then each hoof's horizontal drift during its ground contact is ≤ 0.03 m. | asset |
| AANI-009 | Given the preview renders (55° game camera at ~60 px size and close-ups), then the animal reads as the turnaround animal (zebra: broad stripes, black muzzle, striped mane) and no limb passes through the body in any clip (review checklist). | manual |

## Open questions

- Q-002 Final animal list.
- Q-043 Animation set (hiding-place idles such as `drink`/`sleep`, reactions `not_interested`/`refuse`, locomotion for koala and goldfish while following). The zebra v1 follows the recommendation (`drink`, `refuse`) provisionally.
- Q-040 Monkey baby needed?
