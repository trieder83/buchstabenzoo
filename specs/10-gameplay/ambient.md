---
id: GAME-AMBIENT
title: Ambient animals (ducks, frogs, butterflies)
aspect: gameplay
module: ambient
status: implemented
depends_on: [GAME-LAYOUT, GAME-ANIMALS, ART-ENVIRONMENT, TECH-WATER]
test_prefix: AMB
updated: 2026-09-26
---

# Ambient animals

## Goal

The zoo must not look static (user request 2026-09-26): besides the mission animals,
small **ambient animals** live in the world — first of all the **ducks on the river**, which
are animated characters, plus frogs at the pond and butterflies on the meadow. They are
decoration with behaviour: no mission, no food, never block the player.

## Ducks (`duck`)

1. **Characters, not props:** each duck is a small rigged model with clips `swim` (paddling
   loop, body bob), `dip` (head under water, tail up), `flap` (wing flap, short rise),
   `preen` (idle, turns head, cleans feathers) and `idle` (float, gentle bob).
2. **Behaviour on the river** (per river element, 2–4 ducks, seeded): they swim slowly
   (0.3–0.6 m/s) along the river — mostly downstream with the flow, sometimes turning and
   paddling back upstream — stay on water cells, keep ≥ 0.6 m apart, never pass through the
   bridge posts or rocks (swim around them), sometimes stop to `dip`, `preen` or `flap`
   (every 5–20 s, seeded). A small group (mother + 2–3 ducklings following in a line) is
   allowed.
3. **React to the player:** when the player comes within 2.5 m of the bank next to a duck,
   it swims away to ≥ 4 m with a short `flap`; after 5–10 s calm it swims normally again.
   Ducks never leave the water and never block walking.
4. **Look:** comic style like the other animals (white/yellow body, orange bill and feet,
   big eyes; ducklings yellow and fluffy), readable from the 55° camera; they sit in the
   water at the waterline (origin at the water surface, like the goldfish).
5. **Water interaction:** a small V-shaped wake and ripples behind swimming ducks and rings
   when they `dip` (drawn by the water shader or small decals — TECH-WATER).
6. Ducks are part of the riddle scenery of `loc_river` ("Enten schwimmen") — at least one
   duck is visible near the bridge most of the time (it drifts back there).

## Frogs and butterflies

7. **Frogs** (`frog`) at the pond: sit on lily pads/ the bank, `croak` (throat puff) and
   sometimes `hop` to another pad; hop into the water when the player comes within 2 m.
8. **Butterflies** only where the level data allows them — the meadow hiding place
   (`loc_meadow`, where they are a riddle clue) and the flower beds by the map board: flutter
   in small loops, land on flowers; drawn as tiny animated quads or a 2-bone model. **Never in
   the vegetable garden** (GAME-GARDEN) or at other hiding places, so they never make another
   place look like the meadow riddle.

## Implementation (M6, 2026-09-26 — values are data in `zoo_core::ambient`)

- **Where:** ducks live on `river` elements only — not on `stream`s (the level-3 stream is a
  riddle guard: "no bridge, no ducks, no lilies"), not on ponds, pools or the fountain. Frogs
  live at `pond` elements only (not at the hippo / elephant / goldfish pools — the tiled
  pool must not look like `loc_pond`; not at the fountain). So in the joined zoo: ducks on
  the level-1 river, frogs at the level-1 pond, none in levels 2–3.
- **Ducks:** per river 3 adults (the first is the mother) + 3 ducklings following her in a
  line (0.42 m apart, in river coordinates, so they follow the bend). They swim in the river
  coordinates `(s, c)` of the water field (TECH-WATER): `|c|` ≤ half width − 0.6 m, never
  closer than 0.2 m to a foam obstacle (bridge piles, rapids stones), adults ≥ 0.6 m apart.
  Home = 2.5 m downstream of the bridge (in view of the default camera, which looks north
  over the bridge); new swim targets 70 % downstream / 30 % back upstream within ± 5 m of
  home, 8 % rare excursions ± 10 m; 0.3–0.6 m/s; short rests. `dip` (2 s, water rings),
  `preen` (2.5 s) or `flap` (1.5 s) every 5–20 s; a dip or preen stops the duck.
- **Fleeing:** player within 2.5 m → `flap` and flee in short legs (± 3 m along the river,
  three lanes, the point farthest from the player) at 2.4 m/s until ≥ 4.5 m away, then calm
  5–10 s (floating) before swimming normally; the ducklings follow their mother.
- **Frogs:** 2 per pond on the six lily pads (start pads near the jetty end and the south
  bank, GAME-LEVEL-1), sitting on one of three spots of a pad; croak (1.5 s) or hop to
  another spot of the same pad (≤ 0.62 m, the `hop` clip carries them between its takeoff
  and land events) every 3–9 s. Player within 2 m → hop 0.6 m away into the water, swim
  (0.3 m/s, eyes above the surface) 1.5 m away from the player; after 6–10 s calm swim to
  the nearest free pad spot and hop onto it. A frog on a pad bobs exactly with its pad.
- **Water interaction (rule 5):** wakes and dip rings are drawn by the water shader
  (TECH-WATER behaviour 13); the opaque water surface hides feet, the dipping head and the
  submerged frog.
- **Butterflies (rule 8):** 4 per `[[scenery]]` area that lists `butterfly` in its `props`
  (level 1: only `tall_grass_ne` = `loc_meadow`; none in the garden). 12 s cycle: 9.5 s of
  loops (radius 0.5–1 m, 0.6–1.2 m high) and 2.5 s resting on a flower (0.5 m); wings beat
  8 Hz (slowly open/close when resting). Drawn as a built-in two-wing mesh (≈ 0.34 m
  wingspan, a little oversized so it reads from the high camera; flat colours yellow,
  white, orange, purple) with the wing beat as instance scale — one dynamic batch. The
  "flower beds by the map board" of rule 8 do not exist in the level data (Q-107).
- **Drawing:** instanced skinning — one draw call per model (`duck`, `duckling`, `frog`)
  plus one for the butterflies = 4 draw calls; clips, crossfades and bobbing are computed
  on the CPU per instance. Ambient animals are not saved and use their own seeded RNG
  (seed 7), independent of the game RNG.

## Rules

9. Ambient behaviour is **deterministic** from seed and time (zoo-core, like wandering), so
   tests are reproducible; ambient animals are **not saved** (they restart naturally).
10. Budget: all ambient animals of a level together ≤ 10 draw calls (instanced/skinned
    batches) and ≤ 0.5 ms CPU per frame.
11. They are purely decorative: no collision with the player, no interaction prompt.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| AMB-001 | Given level 1, then 2–4 ducks are on the river, all on water cells, ≥ 0.6 m apart (plus the ducklings of the mother, also on the water); no ducks on the level-3 stream; the frogs sit on lily pads in the pond. | unit |
| AMB-002 | Given 60 s of simulated time, then every duck moved (swim), each played at least one of `dip`/`preen`/`flap`, and none left the water or overlapped a bridge post or rock. | unit |
| AMB-003 | Given the player walks to the bank within 2.5 m of a duck, then that duck swims to ≥ 4 m away within 3 s. | unit |
| AMB-004 | Given the same seed and inputs, then duck and frog behaviour is identical (deterministic). | unit |
| AMB-005 | Given the river on screen, when two screenshots are taken 1 s apart, then the ducks have moved and are animated (pixel difference in their area). | e2e |
| AMB-006 | Given the player walks near the pond, then a frog within 2 m hops into the water. | unit |
| AMB-007 | Given level 1 with all ambient animals, then they add ≤ 10 draw calls and ≤ 0.5 ms CPU per frame. | e2e |
| AMB-008 | Given the river riddle place `loc_river`, then at least one duck is within 8 m of the bridge 80 % of the time over 5 min. | unit |
| AMB-009 | Given the joined zoo, then butterflies exist only over the scenery areas that list `butterfly` (level 1: `loc_meadow`), stay within 0.5 m of that area at 0.35–1.2 m height over 30 s and now and then rest on a flower; none anywhere else (garden, other hiding places). | unit |
| AMB-010 | Given `animal_anims.toml`, then `duck` and `duckling` have the clips `swim`, `idle`, `dip`, `flap`, `preen` and `frog` has `idle`, `croak`, `hop`, `swim`, and the action durations used by the ambient logic (dip 2 s, preen 2.5 s, flap 1.5 s, croak 1.5 s, hop) equal the clip lengths (`frames / 30`). (Rules 1, 7) | unit |
| AMB-011 | Given the player walks along the bank next to a duck, onto the jetty end next to a frog and through a butterfly, then the player's movement is never blocked by them and no interaction prompt appears. (Rule 11) | unit |
| AMB-012 | Given a swimming and a dipping duck, then the water shader receives a wake for the swimmer and a dip ring for the dipper (TECH-WATER WATER-013). (Rule 5) | unit |
| AMB-013 | Given review shots from the 55° camera at 10, 14 and 20 m zoom, then ducks, ducklings, frogs and butterflies are recognisable and in the comic style (Q-108). (Rule 4) | manual |

## Open questions

- Q-107 Butterflies at the map-board flower beds vs. the meadow-only riddle detail.
- Q-108 Readability of ducklings and frogs from the 20 m camera (scale them up?).
- Q-121 Rule 2 (ducks per river element) and rule 7 (frog hops to another pad) vs. the implementation.
- Q-122 Ambient models not in the manifest / concept gate; old static `duck` / `frog` props.
