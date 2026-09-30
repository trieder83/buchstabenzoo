---
id: ART-SOUND
title: Sound effects — sourcing and pipeline
aspect: art
module: sound
status: draft
depends_on: [ART-PIPELINE, ART-ANIMALS, GAME-ANIMALS, GAME-FEED, GAME-PLAYER, PERF-BUDGETS]
test_prefix: ASND
updated: 2026-09-30
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

## Playback (host + core, user decisions 2026-09-30)

Game logic stays in Rust: `zoo-core::sound` decides **which cue, where, how loud** (pure,
seeded, unit-tested); `zoo-web` turns game events into *sound events* (`App.poll_sounds()`, a
JSON array `{cue, x, z, g, r, v}`: cue id, level position, final gain, pitch rate 0.95–1.05,
variation number). The TS host (`web/src/audio.ts`, Web Audio) only loads, decodes and plays
the named cue — it knows no game rules.

**Loudness chain (quiet by decision).** The files are normalised to −16 LUFS, which is loud on
phones, so the host attenuates. Final gain = `MASTER_GAIN × GROUP_GAIN × distance_gain`, all
constants in `zoo-core/src/sound.rs`:

| Constant | Value |
|---|---|
| `MASTER_GAIN` | 0.35 |
| `GROUP_GAIN` ui / steps / animals / doors / pickups | 0.6 / 0.5 / 0.8 / 0.8 / 0.7 |
| Peak chain (distance ≤ 2 m) ui / steps / animals / doors / pickups | 0.21 / 0.175 / 0.28 / 0.28 / 0.245 |

The peak gain of every cue is ≤ 0.5 (ASND-010). **Distance:** full (1.0) within 2 m of the
**player** (not the camera), 0 at 15 m or more, linear in between; sounds with gain 0 are not
emitted. Stereo pan: not in this version (Q-221).

**Triggers** (cue → event; a cue without files is skipped silently):

| Cue | Trigger |
|---|---|
| `step_<surface>` | one per footfall of the player's `walk` clip (`footstep_l/r` at clip time 0 and 0.4 s, RIG-012, scaled by the clip rate), only while moving (also in first person). Surface under the player: `wood` where the ground is raised ≥ 0.08 m (bridge, jetty, floor), `sand` on a `sand` landmark, `water` in pool shallows, else the grid `path` / `grass`. |
| `door_wood_open/close` · `gate_open/close` · `glass_door` · `moon_door` | the visible opening target flips (LAYOUT-031: building door, enclosure / garden gate, level gate → gate, glass door (one cue for both directions), moon door), at the opening's position; none when the level starts |
| `pickup_food` / `drop_food` | `FoodTaken` / `FoodPutBack`, `ItemPutDown` (`food:*`) |
| `pickup_item` / `drop_item` | `ItemTaken` / `ItemPutDown` (the fish bowl), `BambooCut` |
| `harvest_plant` / `basket_add` | `Harvested` (both, basket after the plant) |
| `ui_refuse` | `NotInterested`, `Refuse`, `TreatRefused`, `PutDownRefused`, `BasketFull` |
| `ui_success` | `MissionComplete`, `LevelComplete`, `AllAnimalsHome` |
| `ui_tap` | a tap on a settings button (host only) |
| `animal_<species>_call` | an escaped animal in scope notices the player (comes within 8 m, at most once per animal per 20 s) |
| `animal_<species>_happy` | `StartedFollowing`, `TreatEaten`, `BabyBorn`, `InEnclosure` |
| `animal_<species>_refuse` | `NotInterested`, `Refuse`, `TreatRefused` (together with `ui_refuse`) |

**Host loading.** The `AudioContext` is created only after the first `pointerdown`/`keydown`
(ASND-007). Nothing is fetched before the first frame. Files are loaded lazily per *group*
(`steps`, `ui`, `doors`, `pickups`, and `animals/<species>`): the first time a cue of the group
is needed; after the first gesture, `steps/ui/doors/pickups` are prefetched when the page is
idle, and the animal calls of the species of the unlocked levels (`App.audio_animals()`). The
decoded `AudioBuffer`s are cached. Format: `.ogg` when `canPlayType` allows Vorbis, else
`.m4a` (Q-212, decided: pick by `canPlayType`); if a decode fails the other format is tried
once. Every failure (no Web Audio, fetch or decode error, autoplay refusal) is silent — no
console error, the game works without audio. The files are served under `/assets/audio/`
(listed in `assets/index.json`; types `audio/ogg`, `audio/mp4`; CSP `media-src` / `connect-src`
`'self'`).

**Sound switch.** The settings menu has a sound button (🔊 on / 🔇 off, 72 px), saved as
`zoo.sound` (`0` = off, default on). Off: no cue is played (the decision is still logged).

**Debug hook (e2e).** `window.__zoo.audio` = `{ log, fetched, state, enabled }`: `log` holds
every cue decision `{cue, file, gain, rate, x, z, muted}` *before* playing (works without an
audio device), `fetched` the audio URLs requested so far.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ASND-001 | Given `assets/manifest.toml`, then every audio cue of the sound list has an entry with `licence`, origin (`origin_url`, `prompt` or `script`) and `approved`, and its `.ogg` files exist and decode. | unit |
| ASND-002 | Given every audio entry, then its licence is `CC0` or `public-domain` (both with `origin_url`), `CC-BY` (with a line in `CREDITS.md`), `generated` or `own`; never NC/ND/SA/unknown. | unit |
| ASND-003 | Given the loudness of every cue file, then it is within −16 LUFS ± 2 and peaks ≤ −1 dBFS; short cues are ≤ 1.5 s (animal calls ≤ 3 s). | unit |
| ASND-004 | Given every species in `specs/30-art/animals.md`, then it has `_call`, `_happy` and `_refuse` cues. | unit |
| ASND-005 | Given the player walks on path, grass, sand, wood and shallows, then the matching `step_*` cue plays once per footfall of the walk clip, and nothing plays while standing. | unit (ASND-011/012) + e2e (path / grass / standing) |
| ASND-006 | Given the player opens a door / gate and picks up / drops a box, then the matching cue plays exactly once per event, at the event's position. | e2e |
| ASND-007 | Given the first frame, then no audio file has been fetched yet (lazy loading); audio starts only after the first user gesture (browser autoplay rule). | e2e |
| ASND-008 | Given the total size of `assets/audio`, then it is ≤ 1.5 MB. | unit |
| ASND-009 | Given the sound switch in the settings (mute), then no cue plays; the setting is saved. | e2e + vitest |
| ASND-010 | Given every cue id of the sound list and the gain constants, then the peak gain (distance ≤ 2 m) of every cue is ≤ 0.5: master 0.35, ui ×0.6, steps ×0.5, animals ×0.8. | unit |
| ASND-011 | Given the player walks for 2 s at 1.93 m/s (clip rate 1.38), then the footfall clock fires once per 0.4 clip-seconds (first one at once), nothing while standing, and a new walk starts with a step again. | unit |
| ASND-012 | Given a player on the path, grass, a raised bridge / floor, a sand landmark and pool shallows, then the surface is `path`, `grass`, `wood`, `sand`, `water`. | unit |
| ASND-013 | Given the game events of the trigger table, then each maps to its cue(s) and other events to none; a seeded run gives the same variation numbers. | unit |
| ASND-014 | Given opening kinds and a state flip, then the door cue is the one of the table (open / close; glass and moon door as listed) and no cue is given for the first state. | unit |
| ASND-015 | Given an escaped animal and the player walking up to it, then `animal_<species>_call` is emitted once at 8 m, not again within 20 s, again after 20 s out and back in; never for following or home animals or locked levels. | unit |
| ASND-016 | Given distances 0, 2, 8.5, 15, 40 m, then the distance gain is 1, 1, 0.5, 0, 0. | unit |
| ASND-017 | Given an `assets/index.json` and a `canPlayType`, then the host maps a cue to its group and files, prefers `.ogg`, falls back to `.m4a`, picks variation `v mod n` without repeating the last one, and skips a cue without files silently. | vitest |
| ASND-018 | Given the build, then `assets/audio/**/*.ogg|.m4a` are listed in `assets/index.json` and served with `audio/ogg` / `audio/mp4`; `firebase.json` sets both types and the CSP allows `media-src` / `connect-src 'self'` only. | vitest |
| ASND-019 | Given no `AudioContext`, a failing `fetch` or `decodeAudioData`, then `Audio.play` / `prefetch` never throw and nothing is logged to the console. | vitest |

## Open questions

- Q-200 Audio budget in PERF-BUDGETS (1.5 MB proposal), whether a generator API may be used
  and which (licence and cost), and whether the user wants to supply / review sounds himself.
- Q-220 Sound for following animals' footsteps (`step_*` per animal) and `_baby` / `_step` cues:
  not in this version (only the player's steps). Proposal: later, quietly, per follower.
- Q-221 Stereo pan: not in this version (the camera yaw would have to reach the host). Proposal:
  add a pan from the direction to the emitter relative to the camera when headphones are common.
- Q-222 Ambient / music: not part of this spec. Does the game get a soft background loop?
