---
id: GAME-AMBIENT
title: Ambient animals (ducks, frogs, butterflies)
aspect: gameplay
module: ambient
status: draft
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

## Rules

9. Ambient behaviour is **deterministic** from seed and time (zoo-core, like wandering), so
   tests are reproducible; ambient animals are **not saved** (they restart naturally).
10. Budget: all ambient animals of a level together ≤ 10 draw calls (instanced/skinned
    batches) and ≤ 0.5 ms CPU per frame.
11. They are purely decorative: no collision with the player, no interaction prompt.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| AMB-001 | Given level 1, then 2–4 ducks are on the river, all on water cells, ≥ 0.6 m apart. | unit |
| AMB-002 | Given 60 s of simulated time, then every duck moved (swim), each played at least one of `dip`/`preen`/`flap`, and none left the water or overlapped a bridge post or rock. | unit |
| AMB-003 | Given the player walks to the bank within 2.5 m of a duck, then that duck swims to ≥ 4 m away within 3 s. | unit |
| AMB-004 | Given the same seed and inputs, then duck and frog behaviour is identical (deterministic). | unit |
| AMB-005 | Given the river on screen, when two screenshots are taken 1 s apart, then the ducks have moved and are animated (pixel difference in their area). | e2e |
| AMB-006 | Given the player walks near the pond, then a frog within 2 m hops into the water. | unit |
| AMB-007 | Given level 1 with all ambient animals, then they add ≤ 10 draw calls and ≤ 0.5 ms CPU per frame. | e2e |
| AMB-008 | Given the river riddle place `loc_river`, then at least one duck is within 8 m of the bridge 80 % of the time over 5 min. | unit |
