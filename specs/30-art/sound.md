---
id: ART-SOUND
title: Sound effects — sourcing and pipeline
aspect: art
module: sound
status: draft
depends_on: [ART-PIPELINE, ART-ANIMALS, GAME-ANIMALS, GAME-FEED, GAME-PLAYER, PERF-BUDGETS]
test_prefix: ASND
updated: 2026-10-01
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
- **Size budget:** whole first set ≤ 1.5 MB (PERF-BUDGETS budget 24, Q-200 answered 2026-09-30); loaded
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

## Ambient loops (user request 2026-10-01, Q-222 answered: night crickets only, no music)

"In the night we should have a not too loud background sound of chirping crickets." One cue
`ambient_crickets` (group `ambient`), a **seamless loop** instead of a one-shot cue.

- **Asset:** `assets/audio/ambient/ambient_crickets_1.ogg` + `.m4a` (variants = 1), mono, **20–40 s**
  (now 21.8 s), **−22 LUFS ± 2** (ungated K-weighted; a quiet bed, far below the −16 LUFS cues), peaks ≤ −1 dBFS,
  `.ogg` ≤ 150 KB, spectral peak 3–6 kHz (crickets, nothing else), loop point without a click (jump at the
  seam ≤ the largest normal sample step of the file, level within ±3 dB of the file level). Source: real CC0
  recording by Ted Kerr (OpenGameArt), cleaned and looped by `tools/sound/crickets.py` (source order of this
  spec: real CC0 first; brief `art/sound/brief.md`). `approved = false` until a human has listened;
  review page `art/index.html` section "Sound". `tools/sound/check_audio.py` has a group-aware exception: group
  `ambient` is checked against these limits, not against the one-shot limits of ASND-003.
- **Who decides what (core vs host).** `zoo-core::sound::ambient_target(phase)` is the target gain of the bed:
  `AMBIENT_GAIN` = **0.05** in `dusk` and `night`, **0** in `day`, `sleeping`, `morning`. It is the **final**
  gain (no master / group factor on top: master 0.35 would make the bed inaudible on phones; the file is
  −22 LUFS, so the bed plays at about −40 LUFS, ~9 dB below a footstep at −31 LUFS). `App.ambient_target()`
  (zoo-web) hands it to the host, polled per frame like `poll_sounds()`. The night zoo (`night_1`, phase
  `night`) plays the bed like the day zoo at night. Fade time `AMBIENT_FADE_S` = **3 s** (in and out).
- **Host (`web/src/audio.ts`).** An own channel: `AudioBufferSourceNode` with `loop = true` → its own `GainNode` →
  destination. The gain follows the target **linearly in 3 s** (full swing; `gain` is advanced per frame from
  the frame clock, no per-frame allocation, no `setTargetAtTime` pile-up). The effective target is
  `AMBIENT_GAIN_MAX` (0.05, clamp of the host as a safety net) × sound switch (off → 0, fades out in 3 s, the saved
  setting is not touched) × tab visible (hidden → 0 at once and `AudioContext.suspend()`; visible again →
  `resume()` and fade in) × not on the title / intro overlay.
  - **Lazy:** the file is fetched only the first time the target is > 0 **after the first gesture** (never before
    the first frame, never prefetched by day); decode failure / no files → silent, no console output.
  - **No doubling:** at most one source node exists for the whole session; once the gain has faded to 0 the node
    is stopped and released, a later night starts a new one; phase flips while fading just change the target.
  - Debug: `window.__zoo.audio.ambient` = `{ playing, gain, target, fetched }`.

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
| ASND-020 | Given the `ambient_crickets` entry, then it has licence `CC0`/`public-domain` with `origin_url`, group `ambient`, `approved`, and both files exist; the group-aware checker accepts it (20–40 s, −24…−20 LUFS, peak ≤ −1 dBFS, ogg ≤ 150 KB) and does not apply the 1.5 s one-shot limit; the review page lists it. | unit |
| ASND-021 | Given the decoded loop, then its spectral peak is 3–6 kHz, the loop point has no click (jump ≤ the largest normal sample step) and the level within ±50 ms of the seam is within ±3 dB of the file level. | unit (checker) |
| ASND-022 | Given the phases, then `ambient_target` is 0.05 in dusk and night and 0 in day, sleeping, morning; `AMBIENT_GAIN` ≤ 0.05, fade 3 s; the host constant `AMBIENT_GAIN_MAX` of `audio.ts` equals it. | unit |
| ASND-023 | Given the ambient channel and a target 0.05, then the gain rises linearly to 0.05 in 3 s, never above 0.05; target 0 fades it out in 3 s and then releases the source; one source node at a time. | vitest |
| ASND-024 | Given repeated night / day flips (also mid-fade), then the gain follows without jumps, the file is fetched once and there is never more than one running source. | vitest |
| ASND-025 | Given the sound switch off (target > 0), then the bed fades to silence and the saved setting is unchanged; switching on fades it back in; a hidden tab silences it at once and suspends the context, visible again resumes it. | vitest |
| ASND-026 | Given no gesture yet / no `AudioContext` / failing fetch or decode, then nothing is fetched, nothing throws and nothing is logged to the console; the first fetch happens only after the first gesture and only when the target is > 0. | vitest |
| ASND-027 | Given a game forced to night, then after the first gesture the loop is fetched and plays with gain ≤ 0.05 (`__zoo.audio.ambient`); by day it has faded out and stopped; the page logs no console error. | e2e |

## Open questions

- Q-200 answered 2026-09-30: audio budget 1.5 MB (PERF-BUDGETS budget 24); a generator API may be used
  once the sound-artist names it with licence and cost (Q-213, Q-250); the user reviews / approves the sounds.
- Q-210 answered (real recordings, route 1), Q-212 answered (.ogg + .m4a); open: Q-211 size, Q-213, Q-214, Q-215, Q-216, Q-250, Q-251.
- Q-220 Sound for following animals' footsteps (`step_*` per animal) and `_baby` / `_step` cues:
  not in this version (only the player's steps). Proposal: later, quietly, per follower.
- Q-221 Stereo pan: not in this version (the camera yaw would have to reach the host). Proposal:
  add a pan from the direction to the emitter relative to the camera when headphones are common.
- Q-222 answered 2026-10-01: night crickets only ("Ambient loops"), no music, no daytime ambience.
