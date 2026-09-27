---
id: ART-ANIMALS
title: Animals — concept and models
aspect: art
module: animals
status: draft
depends_on: [ART-PIPELINE, ART-DIRECTION, GAME-ANIMALS, ART-RIG]
test_prefix: AANI
updated: 2026-09-27
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

**Night animals** (GAME-NIGHT rule 6, Q-143): all 10 have model v1 (section "Night
animals" below) — `hedgehog`, `bat`, `owl` (night level 1) and `raccoon`, `badger`, `fennec`,
`kiwi`, `porcupine`, `slow_loris`, `tarsier` (later night levels).

**Required for every animal (Q-143):** a `sleep` clip (loop; lying down / curled / tucked —
GAME-NIGHT rule 1, NIGHT-015) and the `eye_glow` material slot (NIGHT-006, "Rig conventions"
§4). Status 2026-09-27: `eye_glow` is in every mission animal (day and night); `sleep` exists
for the night animals only — the day animals still fall back to `idle` (follow-up).


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
| `hedgehog` | 0.6 m (top of the spines), ≈ 0.75 m long | night; model v1 (0.58 m) |
| `bat` | 0.8 m perched incl. ears | night; model v1 (0.80 m); wingspan in `fly` ≈ 1.1 m (v1 kept, Q-144 answered) |
| `owl` | 1.0 m perched | night; model v1 (1.01 m) |
| `raccoon` | 0.9 m (ears), back 0.55 m | night; model v1 (0.91 m, 1.2 m long incl. tail) |
| `badger` | 0.7 m, 1.1 m long | night; model v1 (0.70 m, 1.08 m long) |
| `fennec` | 0.9 m (ears), back 0.4 m | night; model v1 (0.92 m, back ≈ 0.42 m) |
| `kiwi` | 0.7 m, 0.7 m long + beak | night; model v1 (0.70 m, 0.64 m long incl. beak) |
| `porcupine` | 0.8 m (quills), 1.1 m long | night; model v1 (0.77 m, 1.07 m long) |
| `slow_loris` | 0.7 m on all fours | night; model v1 (0.69 m) |
| `tarsier` | 0.7 m sitting upright | night; model v1 (0.72 m) |

Night-animal sizes are the comic sizes of their approved briefs (user decision Q-143).

Concept art for all 10 animals is approved (2026-09-26).

## Behaviour

1. Every animal follows the common animation set `idle`, `walk` (or `swim`/`climb`/`fly`),
   `eat`, `happy`, `sleep` — the game reacts identically to all animals.
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
   animals have no expression swap). **`eye_glow`** (Q-143, NIGHT-006): the front cap of
   each eye dome (pupil, inner iris and both painted highlights — the inner ≈ 52 % of the
   eye, 12 triangles per eye, no extra geometry) uses a second material `eye_glow` that
   samples the same atlas image and carries `emissiveFactor` = linear `#E6F7A0` (the night glow
   colour, same convention as the props' `*_glow` slots). By day the renderer draws it exactly
   like `body` (emission ignored); at night,
   inside the lantern radius, it draws it unlit/emissive as `#E6F7A0 × (0.25 + 0.75 ×
   luminance(texel))` (highlights shine, the pupil stays dark; never red). So a model has
   the materials `body` (+ `eye_glow`), two primitives in one skinned mesh.
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

## Night animals (models v1)

- Scripts `tools/blender/animals/<id>.py` (shared `night_kit.py`: generic kit on any skeleton,
  `eye_glow`, wing spread, preview with a night panel) → `assets/models/animals/<id>.glb`,
  `assets/blender/animals/<id>.blend`, `assets/textures/animals/<id>_body.png` (256 × 256),
  preview `art/animals/<id>/model_preview.png` (55° game view | front | side | in-game size |
  **night panel**: blue night light, `eye_glow` emissive).
- Look from the approved turnarounds (`art/animals/<id>/`, chosen sheets in each brief).

  | Animal | Rig | Joints | Tris | Clips | Look (key features at small size) |
  |---|---|---|---|---|---|
  | `hedgehog` | `quadruped` (quad_kit) | 23 | 2 759 | `idle`, `walk` (8 f), `eat`, `happy`, `refuse`, `sleep` | brown spiky dome of 77 cone tufts with cream tips, tan face/belly, pink-tan pointed snout, black nose, small pink ears, big blue-grey eyes |
  | `bat` | `bat` (own, below) | 20 | 1 560 | `idle`, `perch`, `hang`, `fly` (16 f), `eat`, `happy`, `refuse`, `sleep` | round brown head, tall pink-lined ears, big dark eyes, pink nose pad, fluffy orange collar with a chest point, dark membrane wings folded at the sides with finger ribs |
  | `owl` | `bird` (bird_rig.py, origin at the feet) | 16 | 2 322 | `idle`, `perch`, `fly` (20 f), `eat`, `happy`, `refuse`, `sleep`, `look` | egg body, cream heart-shaped face disc, giant golden eyes, yellow beak, ear tufts, cream belly with brown spots, folded wings with dark tips, yellow feet |
  | `raccoon` | `quadruped` + `tail_3` | 24 | 1 668 | `idle`, `walk` (16 f), `eat`, `happy`, `refuse`, `sleep` | warm-grey chunky body, black eye mask, white muzzle, dark-lined ears, black paws, thick tail with black rings |
  | `badger` | `quadruped` | 23 | 1 592 | `idle`, `walk` (12 f), `eat`, `happy`, `refuse`, `sleep` | low wide grey body, dark legs, long white face with two black stripes over the eyes, white-rimmed round ears |
  | `fennec` | `quadruped` + `tail_3` | 24 | 1 784 | `idle`, `walk` (12 f), `eat`, `happy`, `refuse`, `sleep` | sand-beige slim fox, huge pink-lined ears, cream face/chest/belly, bushy drooping tail with dark tip |
  | `kiwi` | `biped_animal` (biped_rig.py; wing stubs = arm chains, tail chain hidden in the body) | 23 | 1 116 | `idle`, `walk` (6 f), `eat` (beak probes the ground), `happy`, `refuse`, `sleep` | round shaggy brown ball, small head, long curved ivory beak, thick beige legs, big three-toed feet |
  | `porcupine` | `quadruped` | 23 | 2 341 | `idle`, `walk` (12 f), `eat`, `happy`, `refuse`, `sleep` | dark-brown chunky body, round face, crown of 63 long white quills with a black band sweeping back |
  | `slow_loris` | `quadruped` | 23 | 1 504 | `idle`, `walk` (16 f), `eat`, `happy`, `refuse`, `sleep` | warm-tan fluffy body, big round head, huge amber eyes in dark patches, cream nose stripe/muzzle/chest, dark back stripe, pink paws |
  | `tarsier` | `biped_animal` (biped_rig.py) | 23 | 1 472 | `idle`, `hop` (16 f), `eat`, `happy`, `refuse`, `sleep` | upright on long legs, big round head, giant amber eyes in a beige face, big pink-lined round ears, hands at the chest, long thin tail with a dark tuft |

- **Skeleton `bat`** (20 joints): `root` ─ `hips` ─ `spine` ─ `chest` ─ `neck` ─ `head` ─
  `ear_l`/`ear_r`; `chest` ─ `wing_upper_*` ─ `wing_lower_*` ─ `wing_hand_*` (the membrane is
  skinned along arm, forearm and fingers); `hips` ─ `leg_upper_*` ─ `leg_lower_*` ─ `foot_*`.
  **Owl** uses the 16-joint `bird` skeleton of the ambient duck (GAME-AMBIENT) but with the
  origin **at the feet** (min Y = 0), not at the waterline.
- **Origins and clip conventions (renderer / zoo-core contract):**
  - Quadrupeds (hedgehog, raccoon, badger, fennec, porcupine, slow loris), kiwi and tarsier:
    origin on the ground as every walking animal; extra joints `tail_3` (raccoon, fennec).
  - Tarsier `hop` (loop, `speed = 1.4`, no root motion): both feet planted for 4 of 16 frames
    (they move back with the ground, AANI-008), then a 0.12 m leap; events `land` 0,
    `takeoff` 4. The game moves it like `walk`.
  - Bat / owl: origin at the feet when perched. `idle` stands on flat ground; `perch` = the
    same idle with the toes curled over a branch whose **top is the origin**.
  - `fly` (loop, `speed = 1.4` m/s, no root motion) is authored in place at the perched
    height (bat: body lifted 0.12 m by `hips` and pitched ~66° to horizontal; owl: body
    pitched 30°). The game raises the model origin by **`fly_height` = 1.5 m**
    (`animal_anims.toml`) while the animal follows and moves/turns it; the switch perch ↔ fly
    is a vertical move the game animates (crossfade 0.25 s). Event `wingbeat` = start of
    the downstroke.
  - Bat `hang` (loop): upside down, the **feet at the origin = the grip point** (branch,
    bridge underside); the bat hangs 0.8 m below it (min Y ≈ −0.8 in the clip).
  - `sleep` (loop, crossfade 0.4 s): hedgehog rolled up into a spiky ball (face tucked under
    the coat); bat on its feet, wings pressed to the body, head bowed into the collar; owl
    fluffed down, head bowed into the chest (the face is not visible from the zoo camera);
    the other quadrupeds lie down curled (`night_kit.QuadClips.sleep`: body on the ground,
    legs folded, spine curled to one side, head resting on the ground, tail round); kiwi sits
    down with the beak tucked back into its feathers; tarsier crouches with the head bowed.
    Animals have no eyelids (no expression swap), so sleeping eyes are hidden by the pose
    where possible; the renderer skips `eye_glow` while `sleep` plays; no eyelids for v1 (Q-146 answered).
  - Owl `look` (one-shot): the owl's big head turn left and right (idle variation).
- Other clips as the day animals (`eat` `eat_bite` 22, `happy` `happy_peak` 15, `refuse`).

## Ambient animals (GAME-AMBIENT, M6)

Decorative animals with behaviour but no mission (glossary `ambient_animal`); not part of the
mission-animal list above (they have no `eat` / `happy`; which AANI tests apply to them: Q-122). Look from the approved
`kit_water` sheet (`art/props/kit_water/sheet_v1.jpg`, kit 5 duck / frog); models by script,
origin at the water surface; they replace the static `duck` / `frog` props of `kit_water`.

| Asset id | Script | Rig | Clips (`animal_anims.toml`) | Size | Status |
|---|---|---|---|---|---|
| `duck` | `tools/blender/animals/duck.py` | `bird` (16 joints, `bird_rig.py`) | `swim`, `idle`, `dip`, `flap`, `preen` | ≈ 0.45 m | model v1 |
| `duckling` | `tools/blender/animals/duckling.py` | `bird` (same joints and clips as `duck`) | `swim`, `idle`, `dip`, `flap`, `preen` | ≈ 0.2 m | model v1 |
| `frog` | `tools/blender/animals/frog.py` | `frog` (11 joints) | `idle`, `croak`, `hop`, `swim` | ≈ 0.2 m | model v1 |
| `butterfly` | built-in renderer mesh (12 triangles, no `.glb`) | — | wing beat as instance scale | ≈ 0.34 m wingspan | implemented |

Game scale of duckling / frog: Q-108. Manifest entries and concept gate: Q-122.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| AANI-001 | Given every animal in GAME-ANIMALS, then an asset with the same id exists in the manifest. | asset |
| AANI-002 | Given each animal `.glb`, then it contains `eat` and `happy` plus at least one locomotion animation. | asset |
| AANI-003 | Given each quadruped `.glb`, then its skin has exactly the 23 `quadruped` joint names with the parents of "Rig conventions" §2 plus only its listed optional extra joints (elephant `trunk_1..3`, snow fox `tail_3`; ≤ 32 joints in total). | asset |
| AANI-004 | Given each animal `.glb`, then it has one skinned mesh with material `body` (+ `eye_glow`, required for the night animals; texture ≤ 256 × 256), every vertex has ≤ 4 weights summing to 1 ± 0.001 with `JOINTS_0` UNSIGNED_BYTE, no morph targets / vertex colours / `JOINTS_1`, no duplicated positions with differing normals, ≤ 3 000 triangles and ≤ 400 KB. | asset |
| AANI-005 | Given each quadruped `.glb` in rest pose, then min Y = 0 ± 0.01, the back height matches its model section ± 0.05 m (zebra 1.30 m), `head` is at +Z and `tail_1` at −Z, `*_l` legs at +X, front legs in front of hind legs, legs vertical ± 2°, and the armature node has identity transform. | asset |
| AANI-006 | Given each animal `.glb`, then its clip names equal the manifest `animations` and the `animal_anims.toml` clips, each clip lasts `frames/30` ± 1/30 s, `root` is not animated, only `hips` has translation, no scale channels, looping clips' first/last poses differ ≤ 0.5° / 2 mm, and every event frame is < the clip's frame count. | asset |
| AANI-007 | Given `animal_anims.toml`, then it lists exactly the clips of each animal's model table with the same frames, loop flag, events and `used_for`; `walk` has `speed = 1.4`. | unit |
| AANI-008 | Given `walk` playing for one cycle while the animal moves at its authored speed, then each hoof's horizontal drift during its ground contact is ≤ 0.03 m. | asset |
| AANI-009 | Given the preview renders (55° game camera at ~60 px size and close-ups), then the animal reads as the turnaround animal (zebra: broad stripes, black muzzle, striped mane) and no limb passes through the body in any clip (review checklist). | manual |
| AANI-010 | Given each animal `.glb` with an `eye_glow` material, then `eye_glow` uses the same atlas image as `body`, has `emissiveFactor` = linear `#E6F7A0` and covers only the eye caps (1–120 triangles); every night animal has it. | asset |
| AANI-011 | Given each animal in the manifest (Q-143), then its clips contain `sleep` (loop) — enforced for the night animals now, for the day animals once their `sleep` exists. | asset |
| AANI-012 | Given `bat` / `owl`, then min Y = 0 in the rest pose (origin at the feet), perched height ± 0.05 m (bat 0.80, owl 1.00), `fly` has `speed = 1.4` and `fly_height` in `animal_anims.toml`, and the bat has `hang`. | asset |

## Open questions

- Q-002 Final animal list.
- Q-043 Animation set (hiding-place idles such as `drink`/`sleep`, reactions `not_interested`/`refuse`, locomotion for koala and goldfish while following). The zebra v1 follows the recommendation (`drink`, `refuse`) provisionally.
- Q-040 Monkey baby needed?
- Q-108 Game scale of the ambient animals (duckling, frog).
- Q-122 Ambient models: manifest entries, concept gate, which AANI tests apply.
- Q-143 answered 2026-09-27: night animals with the comic sizes of their briefs, `sleep` and `eye_glow` in every animal's required set.
- Q-144 answered 2026-09-27: bat wingspan in `fly` stays ≈ 1.1 m for v1.
- Q-146 answered 2026-09-27: no eyelids for v1; the renderer skips `eye_glow` while `sleep` plays.
