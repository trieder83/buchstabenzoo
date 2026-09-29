---
id: ART-SOUND
title: Sound effects — sourcing and pipeline
aspect: art
module: sound
status: draft
depends_on: [ART-PIPELINE, ART-ANIMALS, GAME-ANIMALS, GAME-FEED, GAME-PLAYER, PERF-BUDGETS]
test_prefix: ASND
updated: 2026-09-29
---

# Sound effects — sourcing and pipeline

## Goal

The game needs sound (user request 2026-09-29): **animal sounds**, **footsteps**, **door /
gate sounds** and **drop / pickup sounds**. This spec defines *how we get them*; the
`sound-artist` agent (`.claude/agents/sound-artist.md`) owns the work. Playback lives in the
thin TS host (Web Audio), triggered by game events from `zoo-core` (no audio logic in JS beyond
playing a named cue). Voice / read-aloud is a separate topic (Q-008).

## Sound list (first set)

| Group | Cues (id) | Trigger |
|---|---|---|
| Animals | per species: `animal_<species>_call` (heard when escaped and the player is near), `_happy` (eat, treat, baby), `_refuse` (grunt), `_step` optional; baby variant `_baby` | animal state events (GAME-ANIMALS, GAME-RESCUE, GAME-FAMILY) |
| Footsteps | `step_path`, `step_grass`, `step_sand`, `step_wood` (bridge, jetty, floors), `step_water` (shallows); 3–4 variations each, played per walk-clip footfall | player and following animals, by ground surface (GAME-PLAYER §6) |
| Doors | `door_wood_open/close`, `gate_open/close` (enclosure gates, garden gate), `glass_door`, `moon_door` | door / gate open and close events |
| Drop / pickup | `pickup_food`, `drop_food`, `pickup_item` (bowl, bamboo), `drop_item`, `harvest_plant`, `basket_add` | GAME-FEED §8/§13, GAME-GARDEN |
| UI | `ui_tap`, `ui_refuse`, `ui_success` | existing `ui-refuse` etc. |

A cue with several variations is one manifest entry with `variants = N`; the host picks
one at random (seeded in tests) and varies pitch ±5 % so repetition is not noticed.

## How we get the sounds (source order)

1. **CC0 libraries first** (no attribution, safe for kids' apps and stores): Freesound
   (filter licence *CC0*), Kenney audio packs, OpenGameArt (*CC0*), Sonniss GDC bundles
   where their licence allows games. Search with `tools/sound/` helper notes in the brief.
2. **CC-BY only if unavoidable**, and then the credit goes into `assets/audio/CREDITS.md`
   and the in-game credits page. **Never** CC-NC, ND, "editorial" or unclear licences.
3. **AI-generated** sound effects (e.g. a text-to-sound-effect API; key like `GEMINI_API_KEY`
   in the environment, never committed) for animal calls and anything missing; the prompt is
   logged in the brief. Licence of the generator must allow commercial use; result reviewed
   by a human.
4. **Synthesised by script** (`tools/sound/<cue>.py`, numpy/scipy, deterministic seed) for simple
   sounds: pickup / drop pops, UI sounds, wood knocks, soft steps. Script = source of truth, like
   the Blender scripts.
5. **Own recordings** (human, phone) as a last resort, for special sounds.

Animal calls must be **friendly and cartoon-like** (no aggression, no screaming, no distress),
matching the comic style; a real animal recording is pitched/shortened/layered until it
sounds cute. Lions may roar softly, never scary (child-friendly UX).

## Processing and format

- Cleaned in `tools/sound/process.py` (ffmpeg / sox): trim silence, normalise to **−16 LUFS**
  (peaks ≤ −1 dBFS), fade in/out 5 ms, mono for positional cues.
- Delivered as **Ogg Vorbis/Opus `.ogg` plus `.m4a` (AAC)** fallback where needed (Safari/iOS),
  in `assets/audio/<group>/<cue>_<n>.ogg`. Short cues ≤ 1.5 s (animal calls ≤ 3 s).
- **Size budget:** whole first set ≤ 1.5 MB (PERF-BUDGETS "audio", to be added, Q-200); loaded
  lazily by group, never blocking the first frame.
- Every cue is listed in `assets/manifest.toml` (`kind = "audio"`, `source`, `licence`,
  `origin_url`/`prompt`/`script`, `approved`); a human sets `approved = true` after listening
  (like `concept_approved`). An asset without licence info fails the test.
- Positional: the host attenuates by distance to the player (the camera is high-angle, so use
  the player position, not the camera); footsteps and door sounds are heard within ~15 m.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ASND-001 | Given `assets/manifest.toml`, then every audio cue of the sound list has an entry with `licence`, origin (`origin_url`, `prompt` or `script`) and `approved`, and its `.ogg` files exist and decode. | unit |
| ASND-002 | Given every audio entry, then its licence is `CC0`, `CC-BY` (with a line in `CREDITS.md`), `generated` or `own`; never NC/ND/unknown. | unit |
| ASND-003 | Given the loudness of every cue file, then it is within −16 LUFS ± 2 and peaks ≤ −1 dBFS; short cues are ≤ 1.5 s (animal calls ≤ 3 s). | unit |
| ASND-004 | Given every species in `specs/30-art/animals.md`, then it has `_call`, `_happy` and `_refuse` cues. | unit |
| ASND-005 | Given the player walks on path, grass, sand, wood and shallows, then the matching `step_*` cue plays once per footfall of the walk clip, and nothing plays while standing. | e2e |
| ASND-006 | Given the player opens a door / gate and picks up / drops a box, then the matching cue plays exactly once per event, at the event's position. | e2e |
| ASND-007 | Given the first frame, then no audio file has been fetched yet (lazy loading); audio starts only after the first user gesture (browser autoplay rule). | e2e |
| ASND-008 | Given the total size of `assets/audio`, then it is ≤ 1.5 MB. | unit |
| ASND-009 | Given the sound switch in the settings (mute), then no cue plays; the setting is saved. | e2e |

## Open questions

- Q-200 Audio budget in PERF-BUDGETS (1.5 MB proposal), whether a generator API may be used
  and which (licence and cost), and whether the user wants to supply / review sounds himself.
