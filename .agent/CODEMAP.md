# Code & spec map (generated — do not edit)

Regenerate: `python3 tools/agent_codemap.py`. Use it to open **only the lines you need** (Read with offset/limit, or `grep -n`), instead of whole files.

## Source files

| File | Lines | What |
|---|---|---|
| `crates/zoo-assets/src/anim.rs` | 229 | Skeletal animation sampling and blending (ART-RIG §4). Pure math, no allocations in the |
| `crates/zoo-assets/src/lib.rs` | 155 | Asset manifest and asset/concept checks (ART-PIPELINE), glTF loading and skeletal |
| `crates/zoo-assets/src/model.rs` | 554 | glTF 2.0 binary (`.glb`) loading from bytes (ART-PIPELINE, TECH-ARCH decision 4). |
| `crates/zoo-core/src/ambient.rs` | 1165 | Ambient animals (GAME-AMBIENT): ducks and ducklings on the river, frogs at the pond. |
| `crates/zoo-core/src/animals.rs` | 200 | Animal data and states (GAME-ANIMALS). |
| `crates/zoo-core/src/carrying.rs` | 375 | Putting items down, picking them up again, and cutting bamboo in a bamboo forest |
| `crates/zoo-core/src/collision.rs` | 537 | Prop collision (GAME-PLAYER §7): the player is a circle of radius 0.3 m that cannot |
| `crates/zoo-core/src/content.rs` | 190 | Readable content: reading levels (CONT-READING), languages (CONT-L10N) and the Fluent |
| `crates/zoo-core/src/coords.rs` | 60 | Coordinate spaces (GAME-LAYOUT "Coordinate spaces", Q-056). |
| `crates/zoo-core/src/daytime.rs` | 338 | Day and night (GAME-NIGHT): the time of day as a small state machine, driven by `dt`. |
| `crates/zoo-core/src/food.rs` | 159 | Food, food boxes and carrying food (GAME-FEED). |
| `crates/zoo-core/src/game.rs` | 2114 | Game state and the rescue mission flow (GAME-RESCUE, GAME-ANIMALS, GAME-FEED). |
| `crates/zoo-core/src/garden.rs` | 378 | Vegetable garden and treats (GAME-GARDEN): plant spots that are harvested into a basket |
| `crates/zoo-core/src/ground.rs` | 330 | Ground height (GAME-PLAYER rules 8–9, PLAY-035): the top of the visible walkable surface |
| `crates/zoo-core/src/hints.rs` | 946 | Next-target hint (GAME-HINT) and the night progress indicator (GAME-NIGHT "Night |
| `crates/zoo-core/src/level.rs` | 1700 | Level layout data (GAME-LAYOUT, GAME-LEVEL-1) and the walkable grid derived from it. |
| `crates/zoo-core/src/lib.rs` | 38 | Buchstabenzoo game core: pure, deterministic game logic without web dependencies |
| `crates/zoo-core/src/nav.rs` | 233 | Grid navigation: flood fill (LAYOUT-001/002), walking times (LAYOUT-L1-005) and paths for |
| `crates/zoo-core/src/night.rs` | 426 | Nightfall and the night zoo in the game (GAME-NIGHT): moon door, bed, sleeping, the next |
| `crates/zoo-core/src/night_scene.rs` | 568 | Night-only scene parts (GAME-NIGHT §10, `art/night/README.md`): lamps and their light, |
| `crates/zoo-core/src/player.rs` | 184 | Player movement and interaction range (GAME-PLAYER §3, §5, §6). |
| `crates/zoo-core/src/quality.rs` | 271 | Automatic quality tier for weak phones (PERF-BUDGETS rule 5, Q-170, PERF-R-005). |
| `crates/zoo-core/src/rng.rs` | 86 | Tiny seedable PCG32 (XSH RR 64/32) — deterministic on every platform, no dependency. |
| `crates/zoo-core/src/save.rs` | 548 | Saving and restoring progress (GAME-SAVE): the whole game state as versioned JSON |
| `crates/zoo-core/src/scene/models.rs` | 780 | Models of buildings, doors and gates, furniture and gardens (`tools/blender/props/ |
| `crates/zoo-core/src/scene.rs` | 3759 | Level assembly (GAME-LAYOUT, GAME-LEVEL-1): turns the layout data into model placements |
| `crates/zoo-core/src/view.rs` | 193 | Camera views (GAME-CAMERA-VIEWS): the high-angle **zoo view** (default, GAME-PLAYER §2), |
| `crates/zoo-core/src/wander.rs` | 385 | Wandering animals (GAME-ANIMALS "Animal states", ANIM-008…012): wander areas of hiding |
| `crates/zoo-core/src/water.rs` | 1065 | Living water (TECH-WATER, ART-ENVIRONMENT behaviour 5): the river centrelines from the |
| `crates/zoo-render/src/camera.rs` | 884 | High-angle follow camera (GAME-PLAYER §2) and the close look-around / first-person views |
| `crates/zoo-render/src/lib.rs` | 19 | WebGL2 renderer (TECH-ARCH §7): comic look — 2-tone cel shading and a screen-space |
| `crates/zoo-render/src/night.rs` | 594 | Night mode of the renderer (GAME-NIGHT §10, `art/night/README.md` "Night colours"): |
| `crates/zoo-render/src/renderer.rs` | 3625 | WebGL2 renderer (TECH-ARCH §7): instanced static batches sharing the palette texture, |
| `crates/zoo-render/src/shaders.rs` | 685 | GLSL ES 3.00 sources (TECH-ARCH §7): 2-tone cel shading into a G-buffer (colour + |
| `crates/zoo-render/src/sky.rs` | 199 | Comic sky and distance haze of the close camera views (GAME-CAMERA-VIEWS 5, 7). |
| `crates/zoo-web/src/lib.rs` | 3912 | wasm-bindgen entry point (TECH-ARCH): glue between the TypeScript host, `zoo-core` |
| `web/src/input.ts` | 391 | Input forwarding (GAME-PLAYER §3): keyboard, mouse drag/wheel and the two-thumb touch |
| `web/src/main.ts` | 160 | Buchstabenzoo host shell (TECH-ARCH): loads the WASM game, owns the canvas, fetches the |
| `web/src/quality.ts` | 14 | Quality tier of the renderer (PERF-BUDGETS rule 5, PERF-R-005). The tier logic lives in |
| `web/src/save.ts` | 129 | Save slot in the browser (GAME-SAVE): the game state is serialised in Rust; the host only |
| `web/src/scroll.ts` | 102 | Drag-to-scroll for the reading panel (GAME-PLAYER §4 "Scrolling long texts", PLAY-032/033). |
| `web/src/text.ts` | 99 | Text textures (ART-ENVIRONMENT behaviour 7): the game asks for sign texts (Fluent, current |
| `web/src/ui.ts` | 997 | HTML overlays of the host shell (GAME-PLAYER §3/§4, GAME-FEED §6, CONT-L10N): interact |
| `tools/agent_codemap.py` | 124 | Generate .agent/CODEMAP.md: where things are, so agents read only the relevant lines. |
| `tools/agent_state.py` | 93 | Generate .agent/DECISIONS.md from specs/open-questions.md (token-saving digest). |
| `tools/blender/animals/ambient_preview.py` | 204 | Review render of the ambient water animals (GAME-AMBIENT): duck, duckling, frog. |
| `tools/blender/animals/badger.py` | 142 | badger — Dachs (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit). |
| `tools/blender/animals/bat.py` | 324 | bat — Fledermaus, a friendly fruit bat (ART-ANIMALS "Night animals"; own `bat` rig). |
| `tools/blender/animals/biped_rig.py` | 178 | Shared `biped_animal` rig for animals drawn upright on two legs (monkey; later others). |
| `tools/blender/animals/bird_rig.py` | 553 | Shared `bird` rig + duck builder for the ambient water birds (duck, duckling; GAME-AMBIENT). |
| `tools/blender/animals/duck.py` | 72 | duck — ambient comic duck on the river (GAME-AMBIENT; `bird` rig, bird_rig.py). |
| `tools/blender/animals/duckling.py` | 75 | duckling — ambient comic duckling following its mother on the river (GAME-AMBIENT; |
| `tools/blender/animals/elephant.py` | 192 | elephant — Elefant (ART-ANIMALS; quadruped rig + trunk_1..3, built with quad_kit). |
| `tools/blender/animals/fennec.py` | 153 | fennec — Wüstenfuchs / fennec fox (ART-ANIMALS "Night animals"; quadruped rig + tail_3, |
| `tools/blender/animals/fish_rig.py` | 70 | Shared `fish` rig for swimming animals (goldfish; later other fish). |
| `tools/blender/animals/frog.py` | 386 | frog — ambient comic frog at the pond (GAME-AMBIENT; own tiny `frog` rig, 11 joints). |
| `tools/blender/animals/giraffe.py` | 188 | giraffe — Giraffe (ART-ANIMALS; quadruped rig, built with quad_kit). |
| `tools/blender/animals/goldfish.py` | 471 | goldfish — comic goldfish (ART-ANIMALS; `fish` rig, fish_rig.py). |
| `tools/blender/animals/hedgehog.py` | 201 | hedgehog — Igel (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit). |
| `tools/blender/animals/hippo.py` | 156 | hippo — Flusspferd (ART-ANIMALS; quadruped rig, built with quad_kit). |
| `tools/blender/animals/kiwi.py` | 200 | kiwi — Kiwi (ART-ANIMALS "Night animals"; `biped_animal` rig of biped_rig.py, night_kit). |
| `tools/blender/animals/koala.py` | 134 | koala — Koala (ART-ANIMALS; quadruped rig on all fours, built with quad_kit). |
| `tools/blender/animals/lion.py` | 162 | lion — Löwe (ART-ANIMALS; quadruped rig, built with quad_kit). |
| `tools/blender/animals/monkey.py` | 686 | monkey — upright comic monkey (ART-ANIMALS; `biped_animal` rig, biped_rig.py). |
| `tools/blender/animals/night_kit.py` | 304 | Shared kit for the night animals (GAME-NIGHT rule 6, ART-ANIMALS "Night animals"). |
| `tools/blender/animals/owl.py` | 334 | owl — Eule (ART-ANIMALS "Night animals"; `bird` rig of bird_rig.py, built with night_kit). |
| `tools/blender/animals/panda.py` | 150 | panda — Panda (ART-ANIMALS; quadruped rig, built with quad_kit). |
| `tools/blender/animals/porcupine.py` | 157 | porcupine — Stachelschwein (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit). |
| `tools/blender/animals/quad_kit.py` | 696 | Building kit for the quadruped animals (hippo, panda, koala, elephant, giraffe, lion, |
| `tools/blender/animals/quadruped_rig.py` | 772 | Shared `quadruped` rig for four-legged animals (ART-ANIMALS "Rig conventions"). |
| `tools/blender/animals/raccoon.py` | 167 | raccoon — Waschbär (ART-ANIMALS "Night animals"; quadruped rig + tail_3, quad_kit + night_kit). |
| `tools/blender/animals/rig_base.py` | 325 | Generic skeleton helpers for the non-quadruped animal rigs (biped_rig.py, fish_rig.py). |
| `tools/blender/animals/slow_loris.py` | 138 | slow_loris — Plumplori (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit). |
| `tools/blender/animals/snow_fox.py` | 150 | snow_fox — Schneefuchs (ART-ANIMALS; quadruped rig + tail_3, built with quad_kit). |
| `tools/blender/animals/tarsier.py` | 257 | tarsier — Koboldmaki (ART-ANIMALS "Night animals"; `biped_animal` rig of biped_rig.py, |
| `tools/blender/animals/zebra.py` | 698 | zebra — start-mission animal (ART-ANIMALS; rig/clips per the quadruped rig conventions). |
| `tools/blender/characters/human_rig.py` | 1206 | Shared `human` rig for all human characters (ART-RIG). |
| `tools/blender/characters/player_girl.py` | 353 | player_girl — playable girl (ART-CHARACTERS, rig/clips/export per ART-RIG). |
| `tools/blender/check_animal.py` | 530 | Check quadruped animal .glb files against ART-ANIMALS (rig conventions, clips, export rules). |
| `tools/blender/check_character.py` | 485 | Check human character .glb files against ART-RIG (skeleton, skin, clips, export rules). |
| `tools/blender/check_glb.py` | 318 | Check exported .glb files against the ART-PIPELINE export rules (§9, §10). |
| `tools/blender/check_props_3_6.py` | 71 | Check the .glb files of Kits 3-6 (kit_signs, kit_nature, kit_water, kit_barriers) |
| `tools/blender/lib/zoo_blender.py` | 670 | Shared helpers for the headless Blender asset scripts (ART-PIPELINE stage 3). |
| `tools/blender/props/kit_barriers.py` | 273 | Kit 6 — barriers (concept: art/props/kit_barriers/sheet_v1.jpg, brief.md). |
| `tools/blender/props/kit_bedroom.py` | 416 | Kit bedroom — zookeeper-house furniture (concept: art/props/kit_bedroom/sheet_bedroom_v2.jpg, |
| `tools/blender/props/kit_buildings.py` | 698 | Kit buildings — zookeeper house, night house, food storage, food hut, entrance arch |
| `tools/blender/props/kit_fences.py` | 333 | Kit 2 — fences, gate, hedges, walls (concept: art/props/kit_fences/sheet_v3.jpg). |
| `tools/blender/props/kit_garden.py` | 283 | Kit garden — vegetable garden of level 1 (concept: art/props/kit_garden/sheet_plants_v1.jpg, |
| `tools/blender/props/kit_gates.py` | 280 | Kit gates — doors, gates and turnstiles with separable moving parts (user request |
| `tools/blender/props/kit_ground.py` | 310 | Kit 1 — ground tiles (concept: art/props/kit_ground/sheet_v2.jpg, brief.md). |
| `tools/blender/props/kit_landmarks.py` | 379 | Kit landmarks — night-1 scenery and riddle landmarks (concept: art/environment/ |
| `tools/blender/props/kit_nature.py` | 228 | Kit 4 — nature (concept: art/props/kit_nature/sheet_v1.jpg, brief.md). |
| `tools/blender/props/kit_night.py` | 432 | Kit night — lamps and the moon door (concept: art/props/kit_night/sheet_lights_v2.jpg, |
| `tools/blender/props/kit_signs.py` | 201 | Kit 3 — signs, boards, food boxes (concept: art/props/kit_signs/sheet_v5.jpg, brief.md). |
| `tools/blender/props/kit_water.py` | 336 | Kit 5 — water (concept: art/props/kit_water/sheet_v1.jpg, brief.md). |
| `tools/blender/props/night_lib.py` | 718 | Multi-node assets for the night / building / gate kits (kit_night, kit_bedroom, kit_gates, |
| `tools/blender/props/props_parts.py` | 241 | Extra low-poly parts and the kit runner shared by kit_signs, kit_nature, kit_water and |
| `tools/gen_image.py` | 121 | Generate concept images from an art brief with the Gemini image API. |
| `tools/split_sheet.py` | 103 | Split a turnaround sheet (views side by side in one row) into single view images. |
| `tools/textures/sign_silhouettes.py` | 214 | Enclosure sign silhouettes (ART-ENVIRONMENT behaviour 6, AENV-011). |
| `tools/perf/look.mjs` | 369 | Look check for "no visible change" optimisations (PERF-BUDGETS rule 4, |
| `tools/perf/outline_aa_variants.mjs` | 104 | Outline anti-aliasing prototypes (Q-191, PERF-R-016) as shaderSource patches of the |
| `tools/perf/report.mjs` | 122 | Markdown summary of a perf run for specs/50-performance/measurements.md. |
| `tools/perf/sizes.mjs` | 86 | Size report of a perf run (PERF-BUDGETS: WASM size, total download PLAT-001). |
| `tools/perf/turn.mjs` | 408 | Turning smoothness (user report 2026-09-28 "looking left or right is slightly flickery"; |
| `scripts/deploy-preview.sh` | 30 | Build the release game and deploy it to a Firebase Hosting preview channel |
| `scripts/e2e.sh` | 57 | Run Playwright e2e tests efficiently and safely (see .agent/STATE.md "Testing rules"). |
| `scripts/start.sh` | 105 | Control the Buchstabenzoo dev server (wasm-pack --dev build + Vite, reachable on the LAN). |

## Items in big files (line numbers)

- **`crates/zoo-core/src/ambient.rs`**: 26:pub const ADULT_DUCKS · 28:pub const DUCKLINGS · 30:pub const DUCK_SPEED · 32:pub const DUCK_FLEE_SPEED · 34:pub const DUCK_FLEE_TRIGGER_M · 36:pub const DUCK_FLEE_TO_M · 38:pub const DUCK_CALM_S · 40:pub const ACTION_EVERY_S · 42:pub const DUCK_MIN_GAP_M · 44:pub const DUCK_CLEARANCE_M · 47:pub const DUCK_HOME_M · 50:pub const DUCK_HOME_BELOW_BRIDGE_M · 52:pub const DUCKLING_GAP_M · 54:pub const FROG_FLEE_TRIGGER_M · 56:pub const FROG_SWIM_SPEED · 58:pub const FROGS_PER_POND · 61:pub const DUCK_DIP_S · 62:pub const DUCK_FLAP_S · 63:pub const DUCK_PREEN_S · 64:pub const DUCK_SWIM_SPEED · 65:pub const FROG_CROAK_S · 66:pub const FROG_HOP_S · 68:pub const FROG_TAKEOFF_S · 69:pub const FROG_LAND_S · 74:pub const PAD_SPOTS · 75:pub const PAD_TOP_Y · 83:pub enum AmbientKind · 89:impl AmbientKind · 101:pub struct Action · 107:impl Action · 171:pub struct AmbientAnimal · 205:pub struct Pad · 211:impl Pad · 222:pub struct AmbientPose · 239:pub struct Ripple · 250:pub const BUTTERFLIES_PER_AREA · 252:pub const BUTTERFLY_HEIGHT_M · 256:pub struct Butterfly · 267:pub struct ButterflyPose · 274:impl Butterfly · 306:pub struct Ambient · 339:impl Ambient · 1103:impl AmbientAnimal
- **`crates/zoo-core/src/game.rs`**: 25:pub const MIN_PICK_SPREAD_M · 27:pub const MAX_PICK_DRAWS · 31:pub struct FollowParams · 40:impl Default for FollowParams · 52:pub enum GameEvent · 175:pub const PANEL_SETTLE_S · 177:pub const PANEL_CLOSE_S · 179:pub const PANEL_KEEP_RANGE_M · 183:pub struct ReadingPanel · 194:impl ReadingPanel · 202:pub const READABLE_SIDE_DEG · 204:pub const FACING_DEG · 213:pub enum Target · 266:impl Target · 298:pub struct Interactable · 306:pub enum Interaction · 342:pub enum InteractError · 349:pub struct Animal · 375:impl Animal · 396:pub fn pick_hiding_places · 455:pub struct Mission · 462:pub struct Settings · 467:impl Default for Settings · 478:pub struct InfoBoard · 495:pub enum GameError · 500:impl std::fmt::Display for GameError · 509:impl std::error::Error for GameError · 511:pub struct Game · 552:pub const DOOR_OPEN_M · 554:pub const GATE_OPEN_M · 556:pub const GARDEN_GATE_OPEN_M · 559:pub const CARRY_ANIMAL_SPEED_FACTOR · 561:pub const TABLE_HEIGHT_M · 566:pub struct Bowl · 592:pub struct Autosave · 600:pub const AUTOSAVE_MOVING_S · 602:impl GameEvent · 631:impl Game
- **`crates/zoo-core/src/hints.rs`**: 25:pub const HINT_SHOW_S · 27:pub const HINT_CYCLE · 30:pub const AREA_HINT_AFTER_S · 32:pub const AREA_MIN_DIAMETER_M · 34:pub const IDLE_NUDGE_S · 36:pub const REACHED_M · 42:pub const DOT_M · 43:pub const MAX_DOTS · 47:pub enum HintKind · 72:impl HintKind · 92:pub const PRIO_EVENT · 93:pub const PRIO_MISSION · 94:pub const PRIO_UNSTARTED · 95:pub const PRIO_OPTIONAL · 99:pub const PRIO_NIGHT · 101:pub const PRIO_FALLBACK · 105:pub struct Hint · 160:pub fn hiding_circle · 175:pub fn candidates · 569:pub fn is_useful · 585:pub struct HintTracker · 601:impl HintTracker · 763:pub fn walking_distance · 783:pub struct ScreenPlace · 795:pub fn screen_place · 832:pub enum ProgressState · 845:impl ProgressState · 860:pub struct NightProgress · 882:pub fn night_progress
- **`crates/zoo-core/src/level.rs`**: 16:pub struct Rect · 23:impl From< · 34:impl Rect · 67:pub fn cell_center · 72:pub fn cell_of · 79:pub enum ElementType · 91:impl ElementType · 113:pub struct Element · 182:pub struct TreeSpot · 187:impl TreeSpot · 193:impl Element · 242:pub enum Surface · 248:pub struct LevelHeader · 265:pub struct Spawn · 273:pub struct EntryData · 282:pub struct ItemData · 299:impl ItemData · 308:pub struct PropData · 321:impl PropData · 331:pub struct GardenData · 360:pub struct GardenProp · 367:impl GardenData · 388:pub struct GardenBedData · 404:pub struct PlantSpotData · 420:impl PlantSpotData · 429:pub struct CutSpotData · 441:impl CutSpotData · 458:pub struct WaterSourceData · 471:pub struct LightData · 498:impl LightData · 532:pub struct LevelPart · 544:impl Spawn · 552:pub fn facing_vec · 568:pub struct FoodBoxData · 577:impl FoodBoxData · 589:pub struct HidingPlaceData · 627:impl HidingPlaceData · 639:pub struct SceneryData · 652:pub struct EnclosureFeature · 665:impl EnclosureFeature · 678:pub struct LevelData · 726:pub const ZOO_ID · 730:pub const WATER_KINDS · 733:pub enum LevelError · 738:impl std::fmt::Display for LevelError · 747:impl std::error::Error for LevelError · 749:impl LevelData · 1012:pub enum CellKind · 1035:pub struct Grid · 1044:impl Grid · 1214:pub fn fence_edges · 1236:pub struct Level · 1260:pub const LANTERN_POST_RADIUS_M · 1262:impl Level · 1468:pub enum RunAxis · 1473:impl RunAxis · 1486:pub struct Segment · 1493:pub fn segment_run · 1515:pub struct Run · 1521:impl Run · … (+3 more — grep -n)
- **`crates/zoo-core/src/scene/models.rs`**: 9:pub const BUILDING_MODELS · 18:pub fn model_path · 29:pub struct BuildingSpec · 44:pub const BUILDING_SPECS · 94:pub fn rot_level · 100:pub fn model_offset · 107:pub fn building_model · 140:pub enum OpeningKind · 159:pub struct Opening · 173:impl Opening · 183:pub struct BuildingModel · 190:pub struct TextFace · 200:pub struct PlantPlacement · 211:pub struct BakeGroup · 219:impl LevelScene · 236:pub const GATE_ZOO_PILLAR_X · 237:pub const GATE_ZOO_PILLAR_HALF · 238:pub const GATE_ZOO_OPENING_M · 240:pub const GATE_ZOO_LEAF_HZ · 243:pub fn is_level_gate_barrier · 258:pub fn level_gate_pose · 285:pub const STORAGE_PLATFORM_M · 287:pub const GARDEN_SOIL_M · 289:pub const DOOR_WOOD_M · 291:pub const GATE_WOOD_M · 293:pub const MOON_DOOR_LIGHTS · 297:pub const TURNSTILE_X · 298:pub const TURNSTILE_Z · 300:pub const BOARD_LAMP_INFO · 301:pub const BOARD_LAMP_MAP · 304:pub const BOARD_LAMP_WALL · 306:pub const LANTERN_LIGHT · 308:pub const WALL_LAMP_LIGHT · 309:pub const WALL_LAMP_MOUNT_M · 311:pub const BOARD_LAMP_LIGHT · 313:pub const STRING_SPAN_M · 315:pub const DESK_NOTE · 316:pub const NIGHT_TABLE_LAMP · 317:pub const KEY_BOX_KEY · 318:pub const CART_KEY_RING · 320:pub const WINDOW_MOON_MOUNT_M · 321:pub const KEY_BOX_MOUNT_M · 323:pub const ENTRANCE_SIGN_KEY · 326:pub fn facing_str_yaw · 330:impl LevelScene · 766:pub fn moon_door_pose
- **`crates/zoo-core/src/scene.rs`**: 25:pub mod colors · 58:pub fn placeholder_kind · 95:pub fn landmark_model · 110:pub fn perch_scenery · 119:pub fn perch_point · 134:pub struct Placement · 147:impl Placement · 163:pub struct Fallback · 172:pub struct BoxPlacement · 187:pub enum DecalImage · 204:pub struct Decal · 213:impl Decal · 227:pub const FOOD_STORAGE_SIGN_KEY · 229:pub const NIGHT_HOUSE_SIGN_KEY · 232:pub fn silhouette_path · 243:pub const DECAL_LIFT_M · 247:pub const STORAGE_SIGN_BOARD · 249:pub const STORAGE_SIGN_BOTTOM_M · 254:pub struct LevelScene · 299:pub const WATER_WHEEL_RADIUS_M · 302:pub const WATER_WHEEL_AXLE_Y · 304:pub const WATER_WHEEL_WIDTH_M · 306:pub const WATER_WHEEL_PADDLE_M · 308:pub const WATER_WHEEL_PADDLES · 313:pub const WATER_WHEEL_SPIN · 315:pub const WATER_WHEEL_OFFSET_M · 321:pub struct WaterWheel · 335:impl WaterWheel · 383:pub const BRIDGE_PILES · 385:pub const BRIDGE_PILE_R · 387:pub const JETTY_POSTS · 389:pub const FOUNTAIN_WATER_Y · 393:pub enum Dir · 400:impl Dir · 448:pub fn path_tile · 472:pub fn water_tile · 504:pub const RIVER_ROCKS · 507:pub const RIVER_ROCK_SCALE · 509:pub const RIVER_ROCK_FOAM_R · 512:pub const POND_LILY_PADS · 543:pub fn facing_yaw · 584:pub fn walkable_side_row · 616:pub fn walkable_row_center · 652:pub fn door_facade · 669:pub fn bed_pose · 698:pub fn moon_door_axis · 713:pub fn map_board_pose · 719:pub fn info_board_pose · 736:pub const WALL_BOARD · 737:pub const WALL_BOARD_BOTTOM_M · 740:pub fn info_board_lamp_socket · 748:impl LevelScene · 3377:impl LevelScene · 3415:pub const ENCLOSURE_SIGN_HALF_W · 3416:pub const ENCLOSURE_SIGN_Z · 3420:pub const ENCLOSURE_SIGN_GAP_M · 3421:pub const ENCLOSURE_SIGN_OUT_M · 3428:pub fn enclosure_sign_spot · 3464:pub fn enclosure_sign_fits · 3504:pub const INDOOR_SIGN_BOTTOM_M · … (+2 more — grep -n)
- **`crates/zoo-core/src/water.rs`**: 21:pub const BANK_M · 23:pub const BANK_SLOPE · 25:pub const CORNER_R · 27:pub const WATERLINE_INSET · 29:pub const TEXELS_PER_M · 31:pub const WATER_LOOP_S · 33:pub const SHORE_MIN · 34:pub const SHORE_MAX · 39:pub const STREAK_HALF_W · 42:pub fn water_time · 48:pub const WATER_PERIODS_S · 65:pub enum TileShape · 76:impl TileShape · 140:pub fn rotate_cw · 146:pub fn to_canonical · 156:pub struct WaterCell · 165:impl WaterCell · 183:pub struct StillWater · 188:impl StillWater · 203:pub struct Obstacle · 214:pub fn flow_dir · 230:pub enum PathPiece · 249:impl PathPiece · 343:pub struct RiverPath · 351:impl RiverPath · 430:pub fn river_paths · 630:pub struct WaterScene · 637:impl WaterScene · 663:pub struct WaterField · 681:impl CellGrid · 692:impl WaterField · 891:pub fn hash1 · 896:pub fn hash2 · 901:pub fn streak_lane · 908:pub fn river_streak_mask · 930:pub fn pond_ring_mask · 952:pub struct BobParams · 958:impl BobParams · 972:pub fn bob_params · 989:pub fn bob_hash · 1002:pub struct Bob · 1008:pub fn bob · 1022:pub fn bob_transform
- **`crates/zoo-render/src/camera.rs`**: 17:pub struct CameraParams · 31:impl Default for CameraParams · 45:pub const LOOK_AT_HEIGHT_M · 46:pub const NEAR_M · 47:pub const FAR_M · 52:pub struct FollowCamera · 78:pub struct Pose · 89:impl Pose · 98:pub struct Fog · 119:impl FollowCamera · 468:pub fn project
- **`crates/zoo-render/src/renderer.rs`**: 28:pub const SUN_DIR · 30:pub const SHADOW_TINT · 32:pub const OUTLINE · 40:pub const MAX_PIXEL_RATIO · 43:pub const MAX_CROWD · 45:pub const MAX_WATER_OBSTACLES · 47:pub const MAX_WATER_RIPPLES · 52:pub struct Instance · 65:pub const HIDE_ROOF · 67:pub const HIDE_WALLS_UPPER · 69:pub const MAX_NODE_PARTS · 73:pub struct NodeBehaviour · 85:impl NodeBehaviour · 150:pub struct StaticVertices · 159:impl StaticVertices · 181:pub fn bake_vertices · 209:pub const STATIC_VERTEX_FLOATS · 212:pub fn static_vertices · 292:impl Instance · 372:impl U · 460:impl Program · 562:pub struct FrameBlock · 687:impl Default for Region · 699:pub const REGION_ALWAYS · 754:impl Batch · 794:pub const MAX_CHUNK_RUNS · 797:pub const GAP_MERGE_TRIANGLES · 804:pub fn plan_chunk_runs · 843:pub fn chunk_order · 905:pub const CHUNK_M · 918:pub fn static_reach · 972:pub fn instance_extent · 1030:pub struct CharacterDraw · 1059:pub const UNDER_WATER_DEPTH_BIAS_M · 1061:pub const UNDER_WATER_TINT · 1063:impl CharacterDraw · 1104:pub struct FrameStats · 1150:pub struct Renderer · 1221:pub struct InstanceHandle · 1227:pub const BOX · 1229:pub const CAPSULE · 1231:impl Renderer · 3031:impl Cull · 3138:pub fn decode_png · 3165:pub fn cylinder_mesh · 3217:pub fn butterfly_mesh · 3243:pub fn box_mesh · 3268:pub fn capsule_mesh
- **`crates/zoo-web/src/lib.rs`**: 36:pub const PLAYER_MODEL · 38:pub const ANIMAL_ANIMS · 66:pub const AMBIENT_MODELS · 109:pub fn animal_model_path · 115:pub fn level_animals · 126:pub fn join_levels · 138:pub fn required_assets · 216:impl AnimalView · 274:pub struct App · 356:impl App · 2533:impl App · 3671:impl App
- **`web/src/ui.ts`**: 9:export interface UiApp · 50:export const HINT_ICONS · 67:export interface HintView · 78:export function parseHint · 98:export interface NightProgress · 105:export function parseProgress · 120:export interface Basket · 128:export function parseBasket · 137:export const LANGUAGES · 138:export const READING_LEVELS · 141:export const FOOD_ICONS · 163:export const PLACE_ICONS · 208:export const BOWL_ICONS · 215:export const ANIMAL_ICONS · 232:export const TARGET_ICONS · 256:export function targetIcon · 265:export interface LyingIcon · 271:export function parseLyingIcons · 293:export interface KeyValue · 298:export interface Settings · 306:export const SAVED_VIEWS · 316:export function loadSettings · 334:export function saveSettings · 379:export const MIN_READ_PX · 386:export function fitReadingText · 403:export class Ui
- **`tools/blender/animals/quadruped_rig.py`**: 45:def leg_bones · 50:def _joint_table · 66:def mirror · 70:class Rig · 149:def rigid · 155:class Atlas · 212:def ring · 227:def ring_h · 231:class MeshBuilder · 324:def _face_verts · 336:def glow_material · 350:def body_material · 376:class Pose · 481:class Clip · 486:def apply_pose · 506:def bake_clip · 534:class QuadGait · 583:def make_walk · 642:def export_animal · 649:def setup_preview · 703:def _look · 708:def game_view_dir · 715:def render_preview · 747:def render_debug
- **`tools/blender/characters/human_rig.py`**: 47:def arm_dir · 52:def _skeleton · 87:def build_armature · 118:def add_sockets · 139:def smoothstep · 144:def chain · 160:def mix · 170:def rigid · 174:def w_torso · 192:def w_neck · 196:def w_arm · 210:def w_leg · 220:class MeshBuilder · 316:def ellipse_ring · 324:def tube_ring · 333:def ellipsoid_rings · 346:def write_rgba_png · 366:def hex_rgb · 371:class BodyAtlas · 403:class Canvas · 474:def write_face_atlas · 487:def body_material · 502:def face_material · 525:def qaxis · 529:def rx · 535:def ry · 539:def rz · 544:def rot_between · 548:class Pose · 614:def two_bone · 627:def apply_pose · 648:def bake_clip · 670:def reset_pose · 678:def ease · 682:def envelope · 696:def base_stand · 704:def clip_idle · 720:class Gait · 799:def make_locomotion · 834:def clip_walk · 840:def clip_run · 847:def clip_pick_up · 869:def clip_give · 887:def clip_talk · 901:def clip_cheer · 925:def clip_wave · 939:def clip_carry · 954:def player_clips · 972:def export_character · 1021:def _face_toon · 1041:def _pose_at · 1051:def _render · 1066:def setup_preview · 1120:def render_preview · 1155:def hr_outline · 1159:def _compose · 1184:def render_debug
- **`tools/blender/props/night_lib.py`**: 57:def hex_rgb · 62:def lin · 71:def reset · 76:def _bsdf · 80:def material · 126:def face_quad · 139:def ring_xz · 162:def arc_points · 173:def wall_boxes · 207:class Asset · 243:def asset_of · 249:def build · 315:def descendants · 322:def mesh_objs · 326:def world_verts · 335:def tris · 339:def size · 350:def export · 371:def describe · 384:def report · 402:def _toon · 440:def _flat_emit · 458:def _assign · 489:def _render · 496:def render_preview · 652:def run · 699:def instance_tree

## Tests → spec test IDs

- `crates/zoo-assets/tests/footprints.rs`: LAYOUT-018
- `crates/zoo-assets/tests/invisible_walls.rs`: LAYOUT-017, LAYOUT-019, LAYOUT-L1-025
- `crates/zoo-assets/tests/models.rs`: APIPE-004, ARCH-006
- `crates/zoo-assets/tests/pipeline.rs`: APIPE-006…007, APIPE-010
- `crates/zoo-assets/tests/water_tiles.rs`: WATER-003, WATER-010
- `crates/zoo-core/tests/ambient.rs`: AMB-001…004, AMB-006, AMB-008…010, NIGHT-013
- `crates/zoo-core/tests/arch.rs`: ARCH-001
- `crates/zoo-core/tests/camera_views.rs`: CAMV-006, CAMV-008, L2-006, L3-006, LAYOUT-L1-006, LAYOUT-N1-006, PLAY-010
- `crates/zoo-core/tests/content.rs`: ANIM-006…007, L10N-001…003, L10N-005, MISS-001, MISS-004, MISS-007…008, READ-002, RESC-003, RESC-011
- `crates/zoo-core/tests/coords.rs`: LAYOUT-007…011
- `crates/zoo-core/tests/feeding_drop.rs`: FEED-004, FEED-009…023
- `crates/zoo-core/tests/gameplay_qa.rs`: GARD-009, PLAY-020, PLAY-031, RESC-006, RESC-017, RESC-025
- `crates/zoo-core/tests/garden.rs`: GARD-005, GARD-007, GARD-009
- `crates/zoo-core/tests/ground.rs`: PLAY-035
- `crates/zoo-core/tests/hints.rs`: FEED-024…025, HINT-001…008, HINT-010…012, HINT-014, NIGHT-019, NIGHT-021
- `crates/zoo-core/tests/layout.rs`: LAYOUT-001…006, LAYOUT-014…016, LAYOUT-019…020, LAYOUT-023, LAYOUT-026, LAYOUT-040, LAYOUT-L1-001…005, LAYOUT-L1-007…010, LAYOUT-L1-013…018, LAYOUT-L1-020…024
- `crates/zoo-core/tests/level_gates.rs`: LAYOUT-036, LAYOUT-040
- `crates/zoo-core/tests/levels23.rs`: L3-008, LAYOUT-005, LAYOUT-014, LAYOUT-021…024, LAYOUT-040…041, LAYOUT-L2-001…011, LAYOUT-L2-013…014, LAYOUT-L2-016, LAYOUT-L3-001…013, LAYOUT-L3-015
- `crates/zoo-core/tests/modular.rs`: LAYOUT-012…013
- `crates/zoo-core/tests/night.rs`: LAYOUT-L2-018, NIGHT-001…004, NIGHT-006…008, NIGHT-010…011, NIGHT-015…018
- `crates/zoo-core/tests/night1_layout.rs`: CAMV-008, LAYOUT-005, LAYOUT-027…030, LAYOUT-N1-001…013
- `crates/zoo-core/tests/openings.rs`: CAMV-022, LAYOUT-028, LAYOUT-031…036, LAYOUT-038…039, LAYOUT-041
- `crates/zoo-core/tests/panel.rs`: PLAY-023…024, PLAY-026…027, RESC-012
- `crates/zoo-core/tests/player.rs`: LAYOUT-033, LAYOUT-041, PLAY-003, PLAY-005…007, PLAY-010, PLAY-019…022, RIG-016
- `crates/zoo-core/tests/rescue.rs`: ANIM-001…005, ANIM-009…010, FEED-001…008, FEED-027…028, LAYOUT-032, LAYOUT-041, LAYOUT-L1-010, READ-003, RESC-001…002, RESC-004…009, RESC-012…013, RESC-015
- `crates/zoo-core/tests/save.rs`: LAYOUT-041, SAVE-001, SAVE-005…007, SAVE-009
- `crates/zoo-core/tests/wander.rs`: ANIM-008…012, RESC-014, RESC-016
- `crates/zoo-core/tests/water.rs`: AENV-007…008, WATER-001…002, WATER-004…005, WATER-009
- `crates/zoo-core/tests/water_wheel.rs`: LAYOUT-L3-017
- `crates/zoo-core/tests/zfight.rs`: ARCH-005
- `crates/zoo-core/tests/zoo_game.rs`: FAM-001…002, LAYOUT-025, LAYOUT-L2-015, RESC-002…003, RESC-014, RESC-017…023, RESC-025…027, SAVE-010
- `web/src/deploy.test.ts`: PLAT-003…008
- `web/src/input.test.ts`: ARCH-002, CAMV-010, PLAY-015…017
- `web/src/quality.test.ts`: PERF-022
- `web/src/save.test.ts`: RESC-014, SAVE-005
- `web/src/scroll.test.ts`: PLAY-032…033
- `web/src/text.test.ts`: —
- `web/src/ui.test.ts`: CAMV-011, FEED-001, HINT-007, L10N-005, NIGHT-019
- `web/tests/e2e/camera_views.spec.ts`: CAMV-012…014, CAMV-019, CAMV-022, LAYOUT-043
- `web/tests/e2e/feeding.spec.ts`: FEED-009, FEED-016…017, FEED-019, LAYOUT-L3-018
- `web/tests/e2e/gameplay/controls.spec.ts`: LAYOUT-L1-011, PLAY-005, PLAY-011, PLAY-015…016
- `web/tests/e2e/gameplay/doors.spec.ts`: FEED-027…028, LAYOUT-032, LAYOUT-035, LAYOUT-041
- `web/tests/e2e/gameplay/panel.spec.ts`: ADIR-003, PLAY-023, PLAY-025, PLAY-030, RESC-017
- `web/tests/e2e/gameplay/qa.ts`: PLAY-023
- `web/tests/e2e/gameplay/zebra-mission-levels.spec.ts`: POC-002…003, RESC-006…008, RESC-010
- `web/tests/e2e/gates.spec.ts`: LAYOUT-031
- `web/tests/e2e/ground.spec.ts`: PLAY-036
- `web/tests/e2e/helpers.ts`: —
- `web/tests/e2e/hints.spec.ts`: HINT-007, HINT-009, HINT-013…014, NIGHT-016, NIGHT-020…021
- `web/tests/e2e/level_gates.spec.ts`: LAYOUT-036…037, LAYOUT-042
- `web/tests/e2e/m5a.spec.ts`: ANIM-006, ANIM-011…012, RESC-010, RESC-014…017, RESC-024
- `web/tests/e2e/m5b.spec.ts`: LAYOUT-022, LAYOUT-L2-010, LAYOUT-L3-014, PLAY-028…029, RESC-010, RESC-018…020, RESC-023, RESC-026, SAVE-011
- `web/tests/e2e/mission.spec.ts`: ADIR-003, ANIM-006, L10N-005, PLAY-010, PLAY-014, PLAY-023, PLAY-027, POC-002…003, RESC-005, RESC-010
- `web/tests/e2e/night.spec.ts`: NIGHT-001…005, NIGHT-008…009, NIGHT-014, NIGHT-016, PLAY-014
- `web/tests/e2e/panel.spec.ts`: PLAY-023, PLAY-025
- `web/tests/e2e/perf/perf.config.ts`: —
- `web/tests/e2e/perf/scenarios.perf.ts`: —
- `web/tests/e2e/perf_rules.spec.ts`: PERF-016…017, PERF-025, PERF-R-001…003, PERF-R-015, PERF-R-018
- `web/tests/e2e/quality.spec.ts`: PERF-022, PERF-R-005
- `web/tests/e2e/review.spec.ts`: —
- `web/tests/e2e/save.spec.ts`: SAVE-002…004, SAVE-008
- `web/tests/e2e/signs.spec.ts`: AENV-011…012, ARCH-006
- `web/tests/e2e/smoke.spec.ts`: ARCH-003, LAYOUT-L1-011, POC-001
- `web/tests/e2e/touch.spec.ts`: PLAY-013…019
- `web/tests/e2e/water.spec.ts`: AENV-007, AENV-009…010, AMB-005, AMB-007, WATER-006…008, WATER-011

## Big specs: sections (line numbers)

- **`specs/10-gameplay/layout.md`** (657 lines): 17:Goal · 24:Coordinate system · 30:Coordinate spaces (decided, Q-056, user  · 51:Modular edges: fences, hedges, walls (de · 79:Element types · 161:Joining levels (proposal, level design — · 235:Level design rules — how a level is play · 291:Enterable buildings (user request 2026-0 · 324:Moon door and night levels (level design · 347:Night lights, interactables and furnitur · 377:Levels · 393:Behaviour · 411:Forests: dense vs. walkable (user decisi · 439:Enclosure features and wandering at home · 458:Collision footprints (proposal, level de · 500:Gates and doors (user request 2026-09-27 · 591:Test cases · 642:Open questions
- **`specs/10-gameplay/levels/level-1.md`** (894 lines): 18:Goal · 54:Proposals used in this level (not yet de · 82:Spawn and camera · 91:Map · 186:Elements · 255:Hiding places (candidates) · 340:Hiding places — riddle details and sight · 372:Barriers · 386:Walking distances · 429:High-angle camera (Q-049 answered) · 455:Behaviour · 521:Hippo enclosure pool (user decision 2026 · 567:Woods (user decision 2026-09-26: dense v · 601:Collision and billboards (GAME-LAYOUT "C · 646:Vegetable garden (user request 2026-09-2 · 737:Zookeeper house (Q-096 answered; GAME-NI · 780:Moon door (GAME-NIGHT rules 3, 7; Q-133  · 805:Night lights (GAME-NIGHT rule 1; Q-118 a · 824:Burglar event (GAME-EVENTS rules 4–7; Q- · 834:Test cases · 877:Open questions
- **`specs/10-gameplay/levels/level-2.md`** (509 lines): 18:Goal · 47:Proposals used in this level (not yet de · 61:Spawn and camera · 72:Map · 151:Elements · 204:Level entry and exit · 211:Hiding places (candidates) · 279:Hiding places — riddle details and guard · 309:Barriers · 319:Walking distances · 340:Food storage 2 (proposal Q-089) · 357:Bed of level 2 (Q-141 answered, option b · 383:Elephant pool (user decision 2026-09-26: · 390:High-angle camera · 399:Behaviour · 430:Night lights and burglar event (GAME-NIG · 447:Test cases · 470:Implementation status (M5b, 2026-09-26) · 495:Open questions
- **`specs/10-gameplay/levels/level-3.md`** (431 lines): 18:Goal · 51:Proposals used in this level (not yet de · 66:Spawn and camera · 74:Map · 149:Elements · 195:Level entries · 206:Hiding places (candidates) · 267:Hiding places — riddle details and guard · 291:Water sources and the fish bowl (proposa · 304:Barriers · 310:Walking distances · 333:Behaviour · 355:Night lights and burglar event (GAME-NIG · 371:Test cases · 394:Implementation status (M5b, 2026-09-26) · 418:Open questions
- **`specs/10-gameplay/levels/night-1.md`** (416 lines): 19:Goal · 40:Proposals used in this level (not yet de · 54:Spawn, entry and camera · 65:Map · 142:Elements · 187:Night house (GAME-NIGHT rule 4; Q-134 an · 215:Night food storage (Q-135 answered; boxe · 235:Hiding places (candidates) · 279:Hiding places — riddle details (night cl · 298:Barriers · 305:Walking distances · 334:Night lights (GAME-NIGHT rule 1, Q-118,  · 344:Night riddles and the haze (Q-126, CAMV- · 351:Behaviour · 377:Mockups · 384:Test cases · 402:Open questions
- **`specs/20-content/missions/start-missions.md`** (970 lines): 50:Overview · 70:1. Zebra — `loc_river`, `loc_meadow`, `l · 128:2. Hippo — `loc_pond`, `loc_mud`, `loc_s · 187:3. Panda — `loc_cave`, `loc_bamboo`, `lo · 249:4. Koala (pair) — `loc_treehouse`, `loc_ · 304:5. Elephant — `loc_fountain`, `loc_log_p · 357:6. Goldfish — `loc_waterfall`, `loc_wate · 423:7. Monkey — `loc_pirate_ship`, `loc_caro · 478:8. Giraffe — `loc_lookout_tower`, `loc_t · 533:9. Lion — `loc_sun_rocks`, `loc_stage`,  · 586:10. Snow fox — `loc_ice_cream_kiosk`, `l · 641:Night level 1 — hedgehog, bat, owl (GAME · 661:N1. Igel / Hedgehog — `loc_brush_pile`,  · 711:N2. Fledermaus / Bat — `loc_windmill`, ` · 761:N3. Eule / Owl — `loc_moon_pond`, `loc_h · 811:Night texts (GAME-NIGHT) and burglar tex · 926:Behaviour · 943:Test cases · 962:Open questions
- **`specs/30-art/character-rig-and-animation.md`** (302 lines): 14:Goal · 25:Behaviour · 27:1. Coordinate system and units · 36:2. Skeleton (`human` rig) · 109:3. Mesh and skinning · 130:4. Animation clips · 183:5. Clip ownership and budgets · 193:6. Facial expressions · 216:7. Export settings (Blender glTF exporte · 234:8. Reference implementation (model v1) · 250:Acceptance criteria · 262:Test cases · 289:Open questions
- **`specs/30-art/environment.md`** (305 lines): 14:Goal · 19:Mockups required (before modelling) · 81:Hiding places (must appear in a mockup) · 126:Modular props (modelled once, reused) · 146:Built kits (scripted, `tools/blender/pro · 197:Night art (GAME-NIGHT) · 226:Behaviour · 280:Test cases · 297:Open questions
- **`specs/40-tech/water-rendering.md`** (411 lines): 14:Goal · 28:1. Starting point (PoC M3) · 46:2. Approaches compared · 75:3. Decision (recommended combination) · 102:Behaviour · 188:4. Shader specification · 190:Uniforms (water program) · 205:Pseudo-GLSL (fragment) · 248:Parameters (summary) · 261:5. Implementation plan (after M4) · 263:zoo-core (level assembly — `scene.rs` li · 282:zoo-render · 302:zoo-web · 309:Blender water kit (`tools/blender/props/ · 322:Level data / assembly · 330:Cost estimate · 342:Acceptance criteria · 350:Test cases · 377:Implementation (M6, 2026-09-26) · 406:Open questions
- **`specs/50-performance/measurements.md`** (495 lines): 18:How to measure (one command) · 83:Run 2026-09-27 — baseline · 100:Sizes and load · 122:Scenarios — desktop 1920 × 1080 (load av · 140:Scenarios — phone 720 × 1560 drawn (load · 158:Scenarios — desktop_half 960 × 540 (load · 166:Native probe (x86-64 release; WASM is sl · 187:Static draw list (`debug_draw_list`, tri · 198:Findings vs. budgets · 216:Where the time goes (relative, SwiftShad · 236:Run 2026-09-28 — PERF-R-001 + PERF-R-003 · 264:Look (identical?) · 276:GL traffic per frame (census, exact; bef · 314:Frame time — interleaved A/B (fenced fra · 347:Findings vs. budgets (budgets of Q-166…Q · 363:Run 2026-09-28 (2) — PERF-R-002 (a), PER · 382:Counts (`tools/perf/run.sh`, before = ba · 404:Look (`look.mjs`, final build vs. the sh · 411:Frame time — interleaved A/B on the real · 431:Turning flicker (user report 2026-09-28) · 447:Findings vs. budgets · 458:Run 2026-09-29 — PERF-R-018 on (Q-193),  · 468:Turning flicker — `turn.mjs flicker`, 48 · 477:Cost of the prototypes — real iGPU, `loo · 487:Test cases · 492:Open questions
- **`specs/50-performance/recommendations.md`** (449 lines): 14:Goal · 23:Behaviour · 32:Recommendations · 55:PERF-R-001 — Night point lights: cull pe · 88:PERF-R-002 — Ground tiles: draw only the · 131:PERF-R-003 — Shared per-frame uniforms ( · 161:PERF-R-004 — Zero per-frame heap allocat · 186:PERF-R-005 — Pixel-ratio quality tier fo · 212:PERF-R-006 — Full-screen pass: one fewer · 225:PERF-R-007 — Skinned animals: `eye_glow` · 237:PERF-R-008 — Animation: skip clips with  · 248:PERF-R-009 — Dynamic batches: upload onl · 259:PERF-R-010 — Triangle budget test (APIPE · 271:PERF-R-011 — Download: drop the embedded · 282:PERF-R-012 — WASM size: profile before o · 293:PERF-R-013 — Measurement: real-GPU runs, · 305:PERF-R-014 — Night light edges: derivati · 336:PERF-R-015 — Conservative culling boxes  · 358:PERF-R-018 — Haze culling in the close v · 379:PERF-R-016 — Smooth turning: outline ant · 425:PERF-R-017 — Frame pacing while turning · 437:Acceptance criteria · 442:Test cases · 447:Open questions
