---
id: ART-SOUND
title: Sound effects — sourcing and pipeline
aspect: art
module: sound
status: draft
depends_on: [ART-PIPELINE, ART-ANIMALS, GAME-ANIMALS, GAME-FEED, GAME-PLAYER, PERF-BUDGETS]
test_prefix: ASND
updated: 2026-10-07
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
| Golf cart, key box, lock | `cart_board`, `cart_get_out`, `cart_horn`, `cart_bump` (2), `cart_locked`, `cart_park_refuse`, `cart_engine_path` / `cart_engine_grass` (loops), `key_box_open`, `key_pickup`, `lock_wheel_tick` (3), `lock_wrong`, `lock_ok` | GAME-CART; see "Golf cart sounds" |

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
- **Size budget:** whole set ≤ 2.0 MB (raised from 1.5 MB on 2026-10-07, Q-379; PERF-BUDGETS budget 24, Q-200 answered 2026-09-30); loaded
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

## Golf cart sounds (user request 2026-10-07, GAME-CART)

Files exist (`tools/sound/cart_sounds.py`, all synthesised, licence `own`, `approved = false` until a human
listens; brief `art/sound/brief.md` "Group cart / engine"). Playback is **wired** (2026-10-07): zoo-core `sound.rs` /
`cart.rs`, `zoo-web` (`lock_sound`, `honk`, `engine_*`), `web/src/audio.ts`, `lock-panel.ts`, `ui.ts`.

**Assets** (mono, `.ogg` + `.m4a`; one-shots -16 LUFS, peak <= -1.2 dBFS, <= 1.5 s; loops -22 LUFS):

| Cue | Group dir | Variants | Length | Character |
|---|---|---|---|---|
| `cart_board` | `cart` | 1 | 0.84 s | door / seat thump, click, rising electric whirr |
| `cart_get_out` | `cart` | 1 | 0.52 s | motor winds down, click, soft thump |
| `cart_horn` | `cart` | 1 | 0.43 s | FM "beep-beep" 640 Hz, soft |
| `cart_bump` | `cart` | 2 | 0.15 s | soft bonk (wood / rubber), no crash |
| `cart_locked` | `cart` | 1 | 0.31 s | two dull wooden knocks ("nope") |
| `cart_park_refuse` | `cart` | 1 | 0.26 s | two short soft down-blips (not `ui_refuse`) |
| `key_box_open` | `cart` | 1 | 0.86 s | metal latch click-clack, lid, soft chime |
| `key_pickup` | `cart` | 1 | 0.64 s | key jingle |
| `lock_wheel_tick` | `cart` | 3 | 0.05 s | tiny tick |
| `lock_wrong` | `cart` | 1 | 0.39 s | two rounded tones going down, no buzzer |
| `lock_ok` | `cart` | 1 | 0.97 s | happy rising sparkle |
| `cart_engine_path` | `engine` | 1 | 2.00 s loop | soft electric hum + whine + fine gravel crackle |
| `cart_engine_grass` | `engine` | 1 | 2.00 s loop | lower, softer hum + grass swish |

Total `assets/audio` = 1 448 KB of 2.0 MB (ASND-008; Q-379 answered).

**Loudness groups** (constants in `zoo-core/src/sound.rs`; new `Group::Cart`):
`cart_*` (not the engine) -> `Group::Cart` x**0.8** (peak chain 0.28), `key_*` -> `Pickups` x0.7 (0.245),
`lock_*` -> `Ui` x0.6 (0.21). `group_of` must not send the new ids to the `Pickups` fallback by accident:
match the prefixes `cart_`, `key_`, `lock_` explicitly. All peak gains <= 0.5 (ASND-010 list extended).

**Cue table** (position: spatial = distance law from the player as usual, `ui` = distance 0, i.e. full group gain):

| Cue | Trigger | Position | Gain `g` | Rate limit / notes |
|---|---|---|---|---|
| `cart_board` | `GameEvent::CartBoarded` | cart | group | once per event; also starts the engine channel |
| `cart_get_out` | `GameEvent::CartLeft` | cart | group | once per event; also stops the engine channel |
| `cart_locked` | `GameEvent::CartLocked` (both reasons: key missing / level closed) | cart | group | the event is already one per tap; at most 1 per 0.4 s |
| `cart_park_refuse` | `GameEvent::CartNoPark` | cart | group | at most 1 per 0.4 s |
| `cart_horn` | new `GameEvent::CartHorn` from `Game::honk()` (only while seated; trigger = Q-378 horn button, recommendation below) | cart | group | a tap within 0.6 s of the last horn is ignored (`HORN_COOLDOWN_S = 0.6`); no game effect, animals do not react |
| `cart_bump` | new `GameEvent::CartBump { strength }` from `cart.rs` when, while seated, the speed lost to collision slide in one step `lost = speed_before - moved/dt >= BUMP_MIN_LOST_MS (1.0)` | cart | group x `strength` = clamp(lost / 4.5, 0.3, 1.0) | `BUMP_COOLDOWN_S = 0.8`; never from the wedge escape / rescue / push-out of animals; variation `v mod 2` |
| `key_box_open` | `GameEvent::KeyBoxOpened` | key box | group (Pickups) | delayed `d = KEY_BOX_OPEN_DELAY_S = 0.6` s so it follows `lock_ok` |
| `key_pickup` | same event | key box | group (Pickups) | delayed `d = KEY_PICKUP_DELAY_S = 1.2` s |
| `lock_wheel_tick` | host: every digit step of a wheel in `lock-panel.ts` (button, key, wheel drag) | `ui` | group (Ui) | host only like `ui_tap`; one per step, pitch +-5 % |
| `lock_wrong` | host: `enter_code` result "wrong" (together with the 0.4 s shake) | `ui` | group (Ui) | once per try |
| `lock_ok` | host: `enter_code` result "opened" (plays at once, before `key_box_open`) | `ui` | group (Ui) | once |

A sound event with a delay carries an extra JSON field `d` (seconds, default 0): `{cue, x, z, g, r, v, d}`;
the host schedules it at `currentTime + d` (no timers in the host logic beyond that; muted / hidden tab at
fire time -> not played). The sound switch (off) silences all of these as before; a cue without files is skipped.

**Engine channel** (own channel like the ambient bed: two looping `AudioBufferSourceNode`s `path` and `grass`,
each with its own `GainNode`, `loop = true`, `playbackRate` = pitch). Exists only while seated.

Pure functions in `zoo-core/src/sound.rs` (f32, unit-tested, ASND-034/035); `speed` = `Game::cart_speed_now`
(m/s, never negative except the 1.5 m/s wedge escape, then use `abs`):

```
ENGINE_GAIN_MAX     = 0.28     // final gain at 4.5 m/s on a path; no master / group factor on top
ENGINE_IDLE         = 0.25     // fraction of the max while seated and standing still
ENGINE_SPEED_REF    = 4.5      // m/s (path speed)
GRASS_GAIN_FACTOR   = 0.7      // the grass layer is softer
s                   = clamp(abs(speed) / ENGINE_SPEED_REF, 0, 1)
engine_gain(speed)  = ENGINE_GAIN_MAX * (ENGINE_IDLE + (1 - ENGINE_IDLE) * s)      // 0.07 at 0, 0.28 at 4.5
engine_rate(speed)  = 0.75 + 0.55 * s                                              // 0.75 .. 1.30
layer gains         = { path: engine_gain * (1 - b),  grass: engine_gain * GRASS_GAIN_FACTOR * b }
b                   = grass blend 0..1: target 1 when the cell under the cart centre is grass or sand,
                      0 on path / wood / bridge (`step_surface` mapping), moved linearly at 1 / 0.5 s
```
Examples: 2.0 m/s on grass -> gain 0.162 x 0.7 = 0.113, rate 0.99; 4.5 m/s on a path -> 0.28, rate 1.30;
standing -> 0.07, rate 0.75. The host smooths: gain with a one-pole low-pass (time constant 0.2 s), rate
(0.15 s), advanced per frame from the frame clock, no per-frame allocation, no `setTargetAtTime` pile-up.
**Fades:** on `CartBoarded` the engine gain ramps from 0 to the law value in **0.5 s** (the `cart_board` whirr
covers the start); on `CartLeft` it ramps to 0 in **0.4 s**, then both nodes are stopped and released.
At most one pair of sources at any time (like ASND-024). Effective target x sound switch (off -> fades to 0,
setting untouched) x tab visible (hidden -> 0 at once, `AudioContext.suspend()`) x game not paused / no title
overlay. Fetched lazily the first time the player boards a cart after the first gesture (group `engine`,
not prefetched; the group `cart` is prefetched with `steps/ui/doors/pickups` when idle). Peak check: loop file
-22 LUFS x 0.28 = about -34 LUFS, ~3 dB below a footstep (-31 LUFS): never loud. Debug:
`window.__zoo.audio.engine` = `{ playing, gain, rate, grass, fetched }`.
Loop-point caveat: `.ogg` decodes sample-exact; the `.m4a` fallback may carry AAC priming (a few ms); the
files start at the offset whose decoded seam is smoothest (<= 0.56 x the largest step, both formats), so a
residual click is below the engine noise floor. No separate tyre-on-gravel loop: it is part of `cart_engine_path`.

**Horn button (Q-378, answered 2026-10-07, wired):** a 🔔 button (`#horn-btn`, >= 64 px: 76 px, 64 px on small
screens) shown only while seated, in the right column in the slot of the view button (which is hidden while
driving), above the get-out button, plus key `H` while seated (on foot `H` stays the 🧭 hint key). Both call
`Game::honk()` (sound only, animals do not react, 0.6 s cooldown, a pulse on the button when it sounded).

**Host API** (`App`, wasm): `lock_sound("tick" | "wrong" | "ok")` (lock panel; at the player = full group gain, seeded
pitch), `honk() -> bool`, `engine_speed()` (abs m/s, -1 when not seated or paused: the seated flag), `engine_gain()`,
`engine_rate()` (the law of ASND-034, single-sourced in Rust) and `engine_grass()` (the blend, moved at 1 / 0.5 s
by `update_engine_blend`, a new ride starts at its target). `GameEvent::WrongCode` no longer maps to `ui_refuse`
(the host plays `lock_wrong`); `KeyBoxOpened` no longer maps to `pickup_item`. Debug log entries carry `delay`;
`__zoo.audio.engine` also has `layers: [path, grass]` (final gains of the two loops).

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
| ASND-008 | Given the total size of `assets/audio`, then it is ≤ 2.0 MB. | unit |
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
| ASND-030 | Given the entries of the 13 cart / key / lock cues and the checker, then each has licence `own`, `script`, `approved`, both files exist; `check_audio.py` accepts the group `engine` (1-3 s, -22 +/- 2 LUFS, peak <= -1 dBFS, ogg <= 60 KB, seamless seam: decoded jump <= 1 x the largest step, level within +-3 dB) and the one-shots (-16 +/- 2 LUFS, <= 1.5 s); the review page lists them. | unit (`zoo-assets` `audio.rs`, `check_audio.py`) |
| ASND-031 | Given the cue ids of "Golf cart sounds", then `group_of` returns Cart for `cart_*`, Pickups for `key_*`, Ui for `lock_*`, every peak gain <= 0.5 (0.28 / 0.245 / 0.21), and `CUES` lists them all. | unit (`tests/sound.rs`) |
| ASND-032 | Given `CartBoarded`, `CartLeft`, `CartLocked` (both reasons), `CartNoPark`, `CartHorn`, `CartBump`, `KeyBoxOpened`, then the mapping of the cue table holds (`KeyBoxOpened` -> `key_box_open` d 0.6 and `key_pickup` d 1.2, both at the box; the cart cues at the cart's position), a seeded run gives the same variation numbers, other cart events give none. | unit |
| ASND-033 | Given the `cart_bump` rule, then: lost speed < 1 m/s gives none; lost 4.5 gives strength 1.0; lost 1.0 gives 0.3; a second bump within 0.8 s is dropped; a wedge escape / animal push-out / get-out gives none. Given `honk()`: a second tap within 0.6 s gives no second event, never while on foot. | unit (`tests/sound.rs`, `tests/cart.rs`) |
| ASND-034 | Given the engine law, then `engine_gain` is 0.07 at 0, 0.28 at 4.5 (clamped above), monotone, never > 0.28; `engine_rate` is 0.75 at 0, 1.30 at 4.5; `abs` for the wedge reverse speed; layer gains at b = 0 / 1 / 0.5 follow the formulas; 2.0 m/s on grass -> 0.113. | unit |
| ASND-035 | Given the surface under the cart centre (path, grass, sand, wood bridge), then the grass blend target is 0 / 1 / 1 / 0, and the blend moves at most 1 per 0.5 s. | unit |
| ASND-036 | Given the engine channel (fake `AudioContext`), then boarding ramps the gain to the law value in 0.5 s, leaving ramps to 0 in 0.4 s and then stops and releases both nodes, one pair of sources at most (also after board / leave / board in quick succession), gain and rate follow speed with the time constants above, never > 0.28; the file is fetched once, only after the first gesture and the first boarding. | vitest (`audio-engine.test.ts`) |
| ASND-037 | Given the sound switch off while driving, then the engine fades to 0 in 0.4 s without touching the saved setting; hidden tab -> 0 and `suspend()`; no throw and no console output without `AudioContext` / on fetch or decode failure. | vitest (`audio-engine.test.ts`) |
| ASND-038 | Given a sound event with `d > 0`, then the host plays it at `currentTime + d` and not at all when muted at that moment; `lock_wheel_tick` plays once per digit step, `lock_wrong` / `lock_ok` once per result, all with distance 0 and pitch +-5 %. | vitest (`audio.test.ts` delayed events, `ui.test.ts` `resultSound`; the tick per step is covered by e2e: no DOM in vitest) |
| ASND-039 | Given the build, then `assets/audio/cart` and `assets/audio/engine` files are in `assets/index.json` and served with `audio/ogg` / `audio/mp4` (extends ASND-018). | vitest |
| ASND-040 | Given a started game with the key, then board -> `__zoo.audio.log` has `cart_board` once and `__zoo.audio.engine.playing`; driving on a path raises `engine.gain` towards 0.28 (never above), on grass it is lower; get-out -> `cart_get_out`, engine stopped; a locked cart tap logs `cart_locked`; a wrong code logs `lock_wrong`, the right code `lock_ok`, then `key_box_open` and `key_pickup`; a refused park logs `cart_park_refuse`; no console error. | e2e (`web/tests/e2e/audio_cart.spec.ts`; also: horn button only while seated, `H`, one horn per 0.6 s, tick per digit step, muted = nothing) |

## Open questions

- Q-200 answered 2026-09-30: audio budget 1.5 MB (PERF-BUDGETS budget 24); a generator API may be used
  once the sound-artist names it with licence and cost (Q-213, Q-250); the user reviews / approves the sounds.
- Q-210 answered (real recordings, route 1), Q-212 answered (.ogg + .m4a); open: Q-211 size, Q-213, Q-214, Q-215, Q-216, Q-250, Q-251.
- Q-220 Sound for following animals' footsteps (`step_*` per animal) and `_baby` / `_step` cues:
  not in this version (only the player's steps). Proposal: later, quietly, per follower.
- Q-221 Stereo pan: not in this version (the camera yaw would have to reach the host). Proposal:
  add a pan from the direction to the emitter relative to the camera when headphones are common.
- Q-378 (horn button + key H), Q-379 (budget 2.0 MB), Q-380 (delays 0.6 s / 1.2 s) answered 2026-10-07: recommendations adopted.
- Q-222 answered 2026-10-01: night crickets only ("Ambient loops"), no music, no daytime ambience.
