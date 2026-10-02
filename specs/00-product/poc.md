---
id: PROD-POC
title: Proof of concept — level-1 rescue missions
aspect: product
module: poc
status: draft
depends_on: [PROD-VISION, GAME-RESCUE, GAME-PLAYER, GAME-LEVEL-1, CONT-MISSIONS, TECH-ARCH, ART-PIPELINE]
test_prefix: POC
updated: 2026-09-27
---

# Proof of concept — level-1 rescue missions

## Goal

Prove the whole stack end to end with **one playable rescue mission** (zebra) on level 1 — since M5a all three level-1
missions (zebra, hippo, panda; Q-069 answered) —
in a desktop and a mobile browser: Rust/WASM game logic, raw WebGL2 comic renderer with
the high camera, glTF assets from scripted Blender, data-driven level, reading panel in
German and English. Scope is deliberately small; everything else is out.

## In scope

| Area | What | Specs |
|---|---|---|
| Level | Level 1 built at runtime from `assets/levels/level-1.toml` + kit models; walkable grid, path vs. grass speed, sealed by barriers/boundary | GAME-LEVEL-1, GAME-LAYOUT |
| Player | `player_girl` only (boy later), walk with joystick / WASD, idle + walk animation | GAME-PLAYER, ART-RIG |
| Camera | High-angle follow camera: 55°, 35° vertical FOV, zoom 10–20 m, 45° rotation steps | GAME-PLAYER §2 |
| Rendering | WebGL2, 2-tone cel shading, outline pass, flat-colour textures | ART-DIRECTION, TECH-ARCH §7 |
| Mission | Zebra: info board riddle (panel) → food storage → pick `Gras` box → zebra at `loc_river` → show food → follows → enclosure → `happy`. *M5a:* hippo (`Melonen`, pond/mud/shade, home into its pool) and panda (`Bambus`, cave/bamboo/leaves) the same way; discovery (one of 3 places per animal, seeded) and wandering animals | GAME-RESCUE, GAME-FEED, GAME-ANIMALS, CONT-MISSIONS §1–§3 |
| Reading | Text panel with the zebra riddle and food labels, reading level selectable (`kiga`–`klasse3`), `de` + `en` via Fluent | CONT-READING, CONT-L10N |
| Saving | Progress and positions survive a reload (local save) | GAME-SAVE |
| Assets | Kits 1–6 (only props used by level 1), zebra, hippo, panda, girl; buildings as simple block-outs | ART-PIPELINE |

## Out of scope (PoC)

Boy,
map (GAME-MAP), math tasks, visitors/quests, audio/read-aloud, mobile packaging
(Capacitor), final building models, night levels and the morning cut-scene (level
transitions used the temporary rule of the M5b notes; since 2026-09-27 they open the next
morning, GAME-NIGHT).

## Placeholders

Code never waits for art: every missing `.glb` is replaced by a coloured box of the asset's
size (from the layout data), so logic and camera can be tested before models exist.
Placeholders are logged as warnings and must be gone for POC-004.

## Milestones

| # | Milestone | Done when |
|---|---|---|
| M1 | Workspace + core logic | `cargo test --workspace` green: grid/surfaces, walk speeds, animal states, rescue flow, riddle/food data (unit tests from the specs) |
| M2 | Assets v1 | Ground/fence/sign kits, trees, water, bridge, zebra, girl exported as `.glb`; asset tests green (APIPE-001…010) |
| M3 | Renderer | Level 1 renders in the browser with the high camera, cel shading and outlines; placeholder boxes allowed |
| M4 | Playable | The zebra mission can be played end to end with keyboard and touch |
| M5a | All level-1 animals | Hippo and panda missions playable like the zebra, discovery (RESC-014…016), wandering (ANIM-008…012), speeds 1.93 m/s path / 0.98 m/s grass (grass is 1.45 m/s since 2026-09-29, GAME-PLAYER §6), hippo pool, sparse woods, collision footprints (LAYOUT-015…020) |
| M5b | Levels 2 and 3 | Levels 1–3 joined into one zoo (Q-088), the seven new missions playable (koala, elephant, giraffe, lion, monkey, goldfish with the bowl, snow fox), level unlocking, enterable zookeeper house, save v2; LAYOUT-021…024, LAYOUT-L2-*/L3-*, RESC-018…023, FAM-001/002 (flag), PLAY-028/029 |
| M5 | PoC done | All POC tests green on desktop Chrome/Firefox and a mid-range phone |

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| POC-001 | Given the release build served locally, when opened in headless Chromium at 1080×2340, then level 1 renders with a WebGL2 context and no console errors (ARCH-003). | e2e |
| POC-002 | Given reading level `klasse1` and language `de`, when a scripted player runs the zebra mission (board → Gras box → river → show food → enclosure), then mission `zebra` completes (RESC-010). | e2e |
| POC-003 | Given the same run with language `en`, then all panel texts are English and the mission completes. | e2e |
| POC-004 | Given the PoC build, then no placeholder box is rendered (all level-1 assets used by the zebra mission are real `.glb` models). | e2e |
| POC-005 | Given a mid-range phone (Q-013 reference device) in Chrome, then the level renders at ≥ 30 fps while walking. | manual |
| POC-006 | Given the PoC build, then the total download size is ≤ 30 MB (PLAT-001). | e2e |

## M4 notes (2026-09-26)

- Play on desktop: WASD/arrows walk, `E`/Space/Enter interact, `Q`/`R` or mouse drag rotate,
  wheel zooms, gear button = settings (language, reading level). On touch: left thumb
  floating joystick, right thumb swipe rotates / pinch zooms, round button bottom-right
  interacts (GAME-PLAYER §3).
- Food boxes stand in front of the food storage (GAME-LEVEL-1, Q-065); take = interact →
  text panel with the label → take button (GAME-FEED §6).
- The zebra model loads from `assets/models/animals/zebra.glb` when present, otherwise a
  striped placeholder box is drawn (counts for POC-004).
- e2e tests use a debug autopilot (`debug_goto`, grid path over zoo-core navigation with the
  real movement and collision) plus real key presses for interacting.

## M5b notes (2026-09-26)

- All ten animals are in the game: `levels/level-{1,2,3}.toml` are joined (GAME-LAYOUT
  "Joining levels"). **Temporary rule (Q-091 open at the time; superseded 2026-09-27 — barriers now open the next morning, GAME-NIGHT):** a level's exit barrier — and every
  entry barrier of the next level — opens right after the celebration of its last mission
  (nightfall / "the next morning" is not implemented yet). Level 1 → fallen tree →
  level 2; level 2 → construction fence and level-1 north gate → level 3.
- How to play a new mission (e.g. `?seed=4`): read the board of the enclosure → take the
  food from the storage of the same level (all 10 boxes, Q-089) → find the animal where the
  riddle points → show the food (koala / monkey up in the tree / nest: stand below them) →
  lead it through its gate. Goldfish: board (bowl hint) → bowl on the table in the
  zookeeper house → fill it at the tap or a bank → fish food → feed it from the bank → carry
  it (× 0.9 speed) to the stone step of its pond.
- Level-2/3 landmarks without models are coloured placeholder boxes (lists in
  GAME-LEVEL-2/3 "Implementation status"); POC-004 is about level 1 and unaffected.
- Draw calls (1280×720): level-1 spawn 58 / 3.1 k instances, inside level 2 or 3 ≈ 55–60 /
  2.8 k, at a level border ≈ 91 / 6.4 k, worst case the level-3 spawn (three levels meet)
  ≈ 119 / 9.2 k — static batches are culled per level region (QA F12).
- Saves: format v2 for the joined zoo in `zoo.save`; the level-1 save of M4b/M5a is
  migrated (GAME-SAVE §8).

## Open questions

- Q-013 Reference device for POC-005.
- Q-069 answered: all three level-1 missions (zebra, hippo, panda) are in scope (M5a).
