# Code & spec map (generated — do not edit)

Regenerate: `python3 tools/agent_codemap.py`. Use it to open **only the lines you need** (Read with offset/limit, or `grep -n`), instead of whole files.

## Source files

| File | Lines | What |
|---|---|---|
| `crates/zoo-assets/src/anim.rs` | 243 | Skeletal animation sampling and blending (ART-RIG §4). Pure math, no allocations in the |
| `crates/zoo-assets/src/lib.rs` | 155 | Asset manifest and asset/concept checks (ART-PIPELINE), glTF loading and skeletal |
| `crates/zoo-assets/src/model.rs` | 560 | glTF 2.0 binary (`.glb`) loading from bytes (ART-PIPELINE, TECH-ARCH decision 4). |
| `crates/zoo-core/src/ads.rs` | 186 | Ad billboards (GAME-ADS): level data of the boards, the seeded campaign-slot assignment and |
| `crates/zoo-core/src/ambient.rs` | 1165 | Ambient animals (GAME-AMBIENT): ducks and ducklings on the river, frogs at the pond. |
| `crates/zoo-core/src/animals.rs` | 330 | Animal data and states (GAME-ANIMALS). |
| `crates/zoo-core/src/baby.rs` | 340 | Playful following of the baby animal (GAME-FAMILY §5, FAM-014..018). |
| `crates/zoo-core/src/carrying.rs` | 375 | Putting items down, picking them up again, and cutting bamboo in a bamboo forest |
| `crates/zoo-core/src/cart.rs` | 919 | Golf carts (GAME-CART): the three drivable carts of the zoo — kinematics, collision of the |
| `crates/zoo-core/src/cart_key.rs` | 138 | The golf-cart key (GAME-CART rules 12–19): the note "Math Fighter" on the desk, the key box |
| `crates/zoo-core/src/collision.rs` | 566 | Prop collision (GAME-PLAYER §7): the player is a circle of radius 0.3 m that cannot |
| `crates/zoo-core/src/content.rs` | 238 | Readable content: reading levels (CONT-READING), languages (CONT-L10N) and the Fluent |
| `crates/zoo-core/src/coords.rs` | 60 | Coordinate spaces (GAME-LAYOUT "Coordinate spaces", Q-056). |
| `crates/zoo-core/src/daytime.rs` | 338 | Day and night (GAME-NIGHT): the time of day as a small state machine, driven by `dt`. |
| `crates/zoo-core/src/food.rs` | 247 | Food, food boxes and carrying food (GAME-FEED). |
| `crates/zoo-core/src/game.rs` | 3439 | Game state and the rescue mission flow (GAME-RESCUE, GAME-ANIMALS, GAME-FEED). |
| `crates/zoo-core/src/garden.rs` | 522 | Vegetable garden, fruit garden and treats (GAME-GARDEN): plant spots that are harvested |
| `crates/zoo-core/src/ground.rs` | 330 | Ground height (GAME-PLAYER rules 8–9, PLAY-035): the top of the visible walkable surface |
| `crates/zoo-core/src/hints.rs` | 1407 | Next-target hint (GAME-HINT) and the night progress indicator (GAME-NIGHT "Night |
| `crates/zoo-core/src/house.rs` | 154 | Animal houses with door-only entry (GAME-HOUSE, HOUSE-001…020): a building inside an |
| `crates/zoo-core/src/level.rs` | 1844 | Level layout data (GAME-LAYOUT, GAME-LEVEL-1) and the walkable grid derived from it. |
| `crates/zoo-core/src/lib.rs` | 47 | Buchstabenzoo game core: pure, deterministic game logic without web dependencies |
| `crates/zoo-core/src/math.rs` | 408 | Math tasks (CONT-MATH): the math level (`mathe1`…`mathe5`) and the one seeded task of the |
| `crates/zoo-core/src/nav.rs` | 233 | Grid navigation: flood fill (LAYOUT-001/002), walking times (LAYOUT-L1-005) and paths for |
| `crates/zoo-core/src/night.rs` | 490 | Nightfall and the night zoo in the game (GAME-NIGHT): moon door, bed, sleeping, the next |
| `crates/zoo-core/src/night_scene.rs` | 587 | Night-only scene parts (GAME-NIGHT §10, `art/night/README.md`): lamps and their light, |
| `crates/zoo-core/src/overview.rs` | 256 | Overview map (GAME-MAP, Q-363): the whole zoo seen from above, built from the level data. |
| `crates/zoo-core/src/player.rs` | 184 | Player movement and interaction range (GAME-PLAYER §3, §5, §6). |
| `crates/zoo-core/src/quality.rs` | 271 | Automatic quality tier for weak phones (PERF-BUDGETS rule 5, Q-170, PERF-R-005). |
| `crates/zoo-core/src/rng.rs` | 86 | Tiny seedable PCG32 (XSH RR 64/32) — deterministic on every platform, no dependency. |
| `crates/zoo-core/src/save.rs` | 658 | Saving and restoring progress (GAME-SAVE): the whole game state as versioned JSON |
| `crates/zoo-core/src/scene/ad_board.rs` | 159 | Geometry of an ad board (GAME-ADS): frame on two posts, a solid footprint and the picture |
| `crates/zoo-core/src/scene/models.rs` | 800 | Models of buildings, doors and gates, furniture and gardens (`tools/blender/props/ |
| `crates/zoo-core/src/scene/round_house.rs` | 375 | Placeholder geometry of a round enterable house (the terrarium house of `night_2`, |
| `crates/zoo-core/src/scene/terrarium.rs` | 215 | The real models of the terrarium house of `night_2` (AENV-018, `tools/blender/props/ |
| `crates/zoo-core/src/scene.rs` | 4293 | Level assembly (GAME-LAYOUT, GAME-LEVEL-1): turns the layout data into model placements |
| `crates/zoo-core/src/sound.rs` | 326 | Sound decisions (ART-SOUND "Playback"): which cue plays, where and how loud. Pure and |
| `crates/zoo-core/src/telescope.rs` | 183 | The telescope (GAME-TELESCOPE, TELE-001…006): the eight planets of the solar system as a |
| `crates/zoo-core/src/view.rs` | 215 | Camera views (GAME-CAMERA-VIEWS): the high-angle **zoo view** (default, GAME-PLAYER §2), |
| `crates/zoo-core/src/wander.rs` | 510 | Wandering animals (GAME-ANIMALS "Animal states", ANIM-008…012): wander areas of hiding |
| `crates/zoo-core/src/water.rs` | 1065 | Living water (TECH-WATER, ART-ENVIRONMENT behaviour 5): the river centrelines from the |
| `crates/zoo-render/src/camera.rs` | 992 | High-angle follow camera (GAME-PLAYER §2) and the close look-around / first-person views |
| `crates/zoo-render/src/lib.rs` | 19 | WebGL2 renderer (TECH-ARCH §7): comic look — 2-tone cel shading and a screen-space |
| `crates/zoo-render/src/night.rs` | 594 | Night mode of the renderer (GAME-NIGHT §10, `art/night/README.md` "Night colours"): |
| `crates/zoo-render/src/renderer.rs` | 3727 | WebGL2 renderer (TECH-ARCH §7): instanced static batches sharing the palette texture, |
| `crates/zoo-render/src/shaders.rs` | 691 | GLSL ES 3.00 sources (TECH-ARCH §7): 2-tone cel shading into a G-buffer (colour + |
| `crates/zoo-render/src/sky.rs` | 199 | Comic sky and distance haze of the close camera views (GAME-CAMERA-VIEWS 5, 7). |
| `crates/zoo-web/src/lib.rs` | 5155 | wasm-bindgen entry point (TECH-ARCH): glue between the TypeScript host, `zoo-core` |
| `web/src/ad-keys.ts` | 9 | Public key(s) that sign the ad campaigns (GAME-ADS "External content", Q-240). |
| `web/src/ads-ui.ts` | 659 | Host side of the ad billboards (GAME-ADS): fills the board pictures (placeholder text or a |
| `web/src/ads.ts` | 630 | External, signed ad content (GAME-ADS "External content", ADS-007…ADS-019, Q-240). |
| `web/src/analytics-config.ts` | 11 | Opt-in analytics configuration (TECH-PLATFORMS "Analytics (opt-in)", PLAT-029). |
| `web/src/analytics-ui.ts` | 106 | Settings row + welcome dialog of the opt-in analytics (TECH-PLATFORMS "Analytics (opt-in)"). |
| `web/src/analytics.ts` | 337 | Opt-in, privacy-friendly analytics (TECH-PLATFORMS "Analytics (opt-in)", PLAT-022..027). |
| `web/src/audio.ts` | 505 | Sound playback of the host shell (ART-SOUND "Playback"). zoo-core decides which cue plays, |
| `web/src/fullscreen.ts` | 92 | Full screen + install hint of the host shell (TECH-PLATFORMS "Installable and full screen", |
| `web/src/input.ts` | 336 | Input forwarding (GAME-PLAYER §3): keyboard, mouse drag/wheel and the two-thumb touch |
| `web/src/lock-panel.ts` | 255 | Key box lock panel (GAME-CART rule 15, CART-014): three big number wheels (▲ / ▼, digits |
| `web/src/main.ts` | 207 | Buchstabenzoo host shell (TECH-ARCH): loads the WASM game, owns the canvas, fetches the |
| `web/src/math-aid.ts` | 80 | Visual aid of the golf-cart note task (CONT-MATH "Cart note" rule 4): drawn with DOM |
| `web/src/overview-map.ts` | 277 | Overview map of the whole zoo (GAME-MAP, MAP-005…021, Q-363). zoo-core builds the data |
| `web/src/pictograms.ts` | 369 | Food pictograms (GAME-FEED §1, FEED-031): one flat-colour vector drawing with a dark outline |
| `web/src/quality.ts` | 14 | Quality tier of the renderer (PERF-BUDGETS rule 5, PERF-R-005). The tier logic lives in |
| `web/src/save.ts` | 129 | Save slot in the browser (GAME-SAVE): the game state is serialised in Rust; the host only |
| `web/src/scroll.ts` | 102 | Drag-to-scroll for the reading panel (GAME-PLAYER §4 "Scrolling long texts", PLAY-032/033). |
| `web/src/telescope.ts` | 425 | Telescope view (GAME-TELESCOPE, TELE-007…009): at night the child looks through the toy |
| `web/src/text.ts` | 179 | Text textures (ART-ENVIRONMENT behaviour 7): the game asks for sign texts (Fluent, current |
| `web/src/ui.ts` | 1712 | HTML overlays of the host shell (GAME-PLAYER §3/§4, GAME-FEED §6, CONT-L10N): interact |
| `tools/ads/keygen.py` | 44 | Creates an Ed25519 key pair for signing the ad campaigns (GAME-ADS "External content"). |
| `tools/ads/prepare_images.py` | 47 | Compresses the campaign source images (specs/10-gameplay/ads/*/resources/) into the |
| `tools/ads/sign.py` | 121 | Builds and signs the ad manifest (GAME-ADS "External content"). |
| `tools/agent_codemap.py` | 124 | Generate .agent/CODEMAP.md: where things are, so agents read only the relevant lines. |
| `tools/agent_state.py` | 93 | Generate .agent/DECISIONS.md from specs/open-questions.md (token-saving digest). |
| `tools/blender/animals/ambient_preview.py` | 204 | Review render of the ambient water animals (GAME-AMBIENT): duck, duckling, frog. |
| `tools/blender/animals/badger.py` | 142 | badger — Dachs (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit). |
| `tools/blender/animals/bat.py` | 324 | bat — Fledermaus, a friendly fruit bat (ART-ANIMALS "Night animals"; own `bat` rig). |
| `tools/blender/animals/biped_rig.py` | 178 | Shared `biped_animal` rig for animals drawn upright on two legs (monkey; later others). |
| `tools/blender/animals/bird_rig.py` | 553 | Shared `bird` rig + duck builder for the ambient water birds (duck, duckling; GAME-AMBIENT). |
| `tools/blender/animals/chameleon.py` | 706 | chameleon / chameleon_female / chameleon_baby — the terrarium chameleon family |
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
| `tools/blender/animals/koala.py` | 145 | koala — Koala (ART-ANIMALS; quadruped rig on all fours, built with quad_kit). |
| `tools/blender/animals/koala_female.py` | 34 | koala_female — the koala pair's female (GAME-FAMILY; ART-ANIMALS; concept art/animals/koala_female/). |
| `tools/blender/animals/koala_joey.py` | 78 | koala_joey — the koala pair's baby (joey) (GAME-FAMILY; ART-ANIMALS; concept art/animals/koala_joey/). |
| `tools/blender/animals/lion.py` | 162 | lion — Löwe (ART-ANIMALS; quadruped rig, built with quad_kit). |
| `tools/blender/animals/monkey.py` | 686 | monkey — upright comic monkey (ART-ANIMALS; `biped_animal` rig, biped_rig.py). |
| `tools/blender/animals/night_kit.py` | 304 | Shared kit for the night animals (GAME-NIGHT rule 6, ART-ANIMALS "Night animals"). |
| `tools/blender/animals/owl.py` | 334 | owl — Eule (ART-ANIMALS "Night animals"; `bird` rig of bird_rig.py, built with night_kit). |
| `tools/blender/animals/panda.py` | 150 | panda — Panda (ART-ANIMALS; quadruped rig, built with quad_kit). |
| `tools/blender/animals/poison_dart_frog.py` | 558 | poison_dart_frog / poison_dart_frog_female / frog_froglet — the terrarium frog family |
| `tools/blender/animals/porcupine.py` | 157 | porcupine — Stachelschwein (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit). |
| `tools/blender/animals/quad_kit.py` | 700 | Building kit for the quadruped animals (hippo, panda, koala, elephant, giraffe, lion, |
| `tools/blender/animals/quadruped_rig.py` | 813 | Shared `quadruped` rig for four-legged animals (ART-ANIMALS "Rig conventions"). |
| `tools/blender/animals/raccoon.py` | 167 | raccoon — Waschbär (ART-ANIMALS "Night animals"; quadruped rig + tail_3, quad_kit + night_kit). |
| `tools/blender/animals/rig_base.py` | 325 | Generic skeleton helpers for the non-quadruped animal rigs (biped_rig.py, fish_rig.py). |
| `tools/blender/animals/slow_loris.py` | 138 | slow_loris — Plumplori (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit). |
| `tools/blender/animals/snake.py` | 754 | snake / snake_female / snake_hatchling — the terrarium snake family (GAME-FAMILY, ART-ANIMALS |
| `tools/blender/animals/snow_fox.py` | 150 | snow_fox — Schneefuchs (ART-ANIMALS; quadruped rig + tail_3, built with quad_kit). |
| `tools/blender/animals/tarsier.py` | 257 | tarsier — Koboldmaki (ART-ANIMALS "Night animals"; `biped_animal` rig of biped_rig.py, |
| `tools/blender/animals/zebra.py` | 731 | zebra — start-mission animal (ART-ANIMALS; rig/clips per the quadruped rig conventions). |
| `tools/blender/animals/zebra_female.py` | 33 | zebra_female — the zebra pair's female (GAME-FAMILY §3; ART-ANIMALS; concept |
| `tools/blender/animals/zebra_foal.py` | 104 | zebra_foal — the zebra pair's baby (GAME-FAMILY §5/§8; ART-ANIMALS; concept |
| `tools/blender/characters/human_rig.py` | 1249 | Shared `human` rig for all human characters (ART-RIG). |
| `tools/blender/characters/player_girl.py` | 353 | player_girl — playable girl (ART-CHARACTERS, rig/clips/export per ART-RIG). |
| `tools/blender/check_animal.py` | 593 | Check quadruped animal .glb files against ART-ANIMALS (rig conventions, clips, export rules). |
| `tools/blender/check_character.py` | 485 | Check human character .glb files against ART-RIG (skeleton, skin, clips, export rules). |
| `tools/blender/check_glb.py` | 367 | Check exported .glb files against the ART-PIPELINE export rules (§9, §10). |
| `tools/blender/check_props_3_6.py` | 71 | Check the .glb files of Kits 3-6 (kit_signs, kit_nature, kit_water, kit_barriers) |
| `tools/blender/lib/zoo_blender.py` | 670 | Shared helpers for the headless Blender asset scripts (ART-PIPELINE stage 3). |
| `tools/blender/props/golf_cart.py` | 164 | Zoo golf cart + parking sign (GAME-CART). Concept (approved by the user 2026-10-06): |
| `tools/blender/props/kit_barriers.py` | 273 | Kit 6 — barriers (concept: art/props/kit_barriers/sheet_v1.jpg, brief.md). |
| `tools/blender/props/kit_bedroom.py` | 426 | Kit bedroom — zookeeper-house furniture (concept: art/props/kit_bedroom/sheet_bedroom_v2.jpg, |
| `tools/blender/props/kit_buildings.py` | 698 | Kit buildings — zookeeper house, night house, food storage, food hut, entrance arch |
| `tools/blender/props/kit_fences.py` | 333 | Kit 2 — fences, gate, hedges, walls (concept: art/props/kit_fences/sheet_v3.jpg). |
| `tools/blender/props/kit_garden.py` | 336 | Kit garden — vegetable garden of level 1 (concept: art/props/kit_garden/sheet_plants_v1.jpg, |
| `tools/blender/props/kit_gates.py` | 280 | Kit gates — doors, gates and turnstiles with separable moving parts (user request |
| `tools/blender/props/kit_ground.py` | 310 | Kit 1 — ground tiles (concept: art/props/kit_ground/sheet_v2.jpg, brief.md). |
| `tools/blender/props/kit_landmarks.py` | 379 | Kit landmarks — night-1 scenery and riddle landmarks (concept: art/environment/ |
| `tools/blender/props/kit_landmarks_l2.py` | 278 | Kit landmarks_l2 — level-2 riddle landmarks: the zoo train at its station and the blossom |
| `tools/blender/props/kit_landmarks_play.py` | 272 | Kit landmarks_play — ice cream kiosk (level 3), carousel (level 3), playground slide and |
| `tools/blender/props/kit_nature.py` | 228 | Kit 4 — nature (concept: art/props/kit_nature/sheet_v1.jpg, brief.md). |
| `tools/blender/props/kit_night.py` | 432 | Kit night — lamps and the moon door (concept: art/props/kit_night/sheet_lights_v2.jpg, |
| `tools/blender/props/kit_signs.py` | 201 | Kit 3 — signs, boards, food boxes (concept: art/props/kit_signs/sheet_v5.jpg, brief.md). |
| `tools/blender/props/kit_terrarium.py` | 720 | Kit terrarium — the real models of the terrarium house of night_2 (concept: art/environment/ |
| `tools/blender/props/kit_water.py` | 336 | Kit 5 — water (concept: art/props/kit_water/sheet_v1.jpg, brief.md). |
| `tools/blender/props/night_lib.py` | 722 | Multi-node assets for the night / building / gate kits (kit_night, kit_bedroom, kit_gates, |
| `tools/blender/props/props_parts.py` | 241 | Extra low-poly parts and the kit runner shared by kit_signs, kit_nature, kit_water and |
| `tools/gen_image.py` | 121 | Generate concept images from an art brief with the Gemini image API. |
| `tools/make_icons.py` | 84 | Draws the Buchstabenzoo app icons (TECH-PLATFORMS "Installable and full screen", PLAT-014). |
| `tools/sound/animals.py` | 254 | Animal cues from real recordings (CC0 / public domain, a few CC-BY) + a synthesised goldfish |
| `tools/sound/check_audio.py` | 96 | Decodes every audio file of the manifest and checks ASND-001 (decodes), ASND-003 (loudness |
| `tools/sound/crickets.py` | 90 | Night cricket loop `ambient_crickets` (ART-SOUND "Ambient loops", ASND-020..). |
| `tools/sound/doors.py` | 91 | Door / gate cues (ART-SOUND). Run: python3 tools/sound/doors.py |
| `tools/sound/pickups.py` | 73 | Pickup / drop cues (ART-SOUND). Run: python3 tools/sound/pickups.py |
| `tools/sound/process.py` | 132 | Processing + encoding for all sound cues (ART-SOUND "Processing and format"). |
| `tools/sound/register.py` | 86 | Writes the audio block of assets/manifest.toml (between the BEGIN/END markers) from the |
| `tools/sound/runway_gen.py` | 34 | Generate sound-effect candidates with Runway (eleven_text_to_sound_v2, POST /v1/sound_effect). |
| `tools/sound/steps.py` | 66 | LEGACY (superseded by steps_real.py, user: "not realistic at all"). Kept only for reference; |
| `tools/sound/steps_real.py` | 124 | Footstep cues from real recordings (CC0) and Runway-generated water splashes (ART-SOUND). |
| `tools/sound/synthlib.py` | 92 | Small synthesis helpers for the Buchstabenzoo sound cues (ART-SOUND, numpy/scipy only). |
| `tools/sound/ui.py` | 36 | UI cues (ART-SOUND). Run: python3 tools/sound/ui.py |
| `tools/spec_index.py` | 104 | Regenerate specs/INDEX.md from spec frontmatter. Usage: python3 tools/spec_index.py [--check] |
| `tools/split_sheet.py` | 103 | Split a turnaround sheet (views side by side in one row) into single view images. |
| `tools/textures/sign_silhouettes.py` | 334 | Enclosure sign silhouettes (ART-ENVIRONMENT behaviour 6, AENV-011). |
| `tools/perf/look.mjs` | 369 | Look check for "no visible change" optimisations (PERF-BUDGETS rule 4, |
| `tools/perf/outline_aa_variants.mjs` | 104 | Outline anti-aliasing prototypes (Q-191, PERF-R-016) as shaderSource patches of the |
| `tools/perf/report.mjs` | 122 | Markdown summary of a perf run for specs/50-performance/measurements.md. |
| `tools/perf/sizes.mjs` | 86 | Size report of a perf run (PERF-BUDGETS: WASM size, total download PLAT-001). |
| `tools/perf/turn.mjs` | 408 | Turning smoothness (user report 2026-09-28 "looking left or right is slightly flickery"; |
| `scripts/deploy-preview.sh` | 30 | Build the release game and deploy it to a Firebase Hosting preview channel |
| `scripts/e2e.sh` | 71 | Run Playwright e2e tests efficiently and safely (see .agent/STATE.md "Testing rules"). |
| `scripts/start.sh` | 105 | Control the Buchstabenzoo dev server (wasm-pack --dev build + Vite, reachable on the LAN). |

## Items in big files (line numbers)

- **`crates/zoo-core/src/ambient.rs`**: 26:pub const ADULT_DUCKS · 28:pub const DUCKLINGS · 30:pub const DUCK_SPEED · 32:pub const DUCK_FLEE_SPEED · 34:pub const DUCK_FLEE_TRIGGER_M · 36:pub const DUCK_FLEE_TO_M · 38:pub const DUCK_CALM_S · 40:pub const ACTION_EVERY_S · 42:pub const DUCK_MIN_GAP_M · 44:pub const DUCK_CLEARANCE_M · 47:pub const DUCK_HOME_M · 50:pub const DUCK_HOME_BELOW_BRIDGE_M · 52:pub const DUCKLING_GAP_M · 54:pub const FROG_FLEE_TRIGGER_M · 56:pub const FROG_SWIM_SPEED · 58:pub const FROGS_PER_POND · 61:pub const DUCK_DIP_S · 62:pub const DUCK_FLAP_S · 63:pub const DUCK_PREEN_S · 64:pub const DUCK_SWIM_SPEED · 65:pub const FROG_CROAK_S · 66:pub const FROG_HOP_S · 68:pub const FROG_TAKEOFF_S · 69:pub const FROG_LAND_S · 74:pub const PAD_SPOTS · 75:pub const PAD_TOP_Y · 83:pub enum AmbientKind · 89:impl AmbientKind · 101:pub struct Action · 107:impl Action · 171:pub struct AmbientAnimal · 205:pub struct Pad · 211:impl Pad · 222:pub struct AmbientPose · 239:pub struct Ripple · 250:pub const BUTTERFLIES_PER_AREA · 252:pub const BUTTERFLY_HEIGHT_M · 256:pub struct Butterfly · 267:pub struct ButterflyPose · 274:impl Butterfly · 306:pub struct Ambient · 339:impl Ambient · 1103:impl AmbientAnimal
- **`crates/zoo-core/src/cart.rs`**: 17:pub const HALF_WIDTH_M · 18:pub const HALF_LENGTH_M · 20:pub const PATH_SPEED_MS · 21:pub const GRASS_SPEED_MS · 23:pub const TURN_RATE_DEG_S · 24:pub const ACCEL_MS2 · 25:pub const BRAKE_MS2 · 28:pub const DRIVE_ERROR_DEG · 30:pub const BOARD_RANGE_M · 32:pub const STOP_MARGIN_M · 43:pub const WEDGE_S · 45:pub const REVERSE_AFTER_S · 46:pub const REVERSE_SPEED_MS · 48:pub const RESCUE_S · 50:pub const ENTER_RANGE_PLAYER_M · 53:pub fn yaw_of_dir · 58:pub fn box_shape · 68:pub enum CartLock · 77:pub enum LeaveResult · 86:pub struct Cart · 108:impl Cart · 198:pub fn distance_to_box · 221:impl Game
- **`crates/zoo-core/src/game.rs`**: 25:pub const MIN_PICK_SPREAD_M · 27:pub const MAX_PICK_DRAWS · 31:pub struct FollowParams · 40:impl Default for FollowParams · 52:pub enum GameEvent · 214:pub const PANEL_SETTLE_S · 216:pub const PANEL_CLOSE_S · 218:pub const PANEL_KEEP_RANGE_M · 222:pub struct ReadingPanel · 233:impl ReadingPanel · 241:pub const READABLE_SIDE_DEG · 243:pub const FACING_DEG · 252:pub enum Target · 333:pub enum Gift · 340:impl Target · 382:pub struct Interactable · 390:pub enum Interaction · 454:pub enum InteractError · 463:pub struct Baby · 474:pub struct Animal · 500:impl Animal · 521:pub fn pick_hiding_places · 581:pub const STALL_HELP_S · 584:pub const GIFT_COOLDOWN_S · 585:pub const STALL_WALK_S · 590:pub struct Mission · 597:pub struct Settings · 604:impl Default for Settings · 616:pub struct InfoBoard · 640:pub enum GameError · 645:impl std::fmt::Display for GameError · 654:impl std::error::Error for GameError · 656:pub struct Game · 746:pub const DOOR_OPEN_M · 748:pub const GATE_OPEN_M · 750:pub const GARDEN_GATE_OPEN_M · 753:pub const CARRY_ANIMAL_SPEED_FACTOR · 755:pub const TABLE_HEIGHT_M · 760:pub struct Bowl · 786:pub struct Autosave · 794:pub const AUTOSAVE_MOVING_S · 796:impl GameEvent · 828:impl Game
- **`crates/zoo-core/src/hints.rs`**: 26:pub const HINT_SHOW_S · 28:pub const HINT_CYCLE · 31:pub const AREA_HINT_AFTER_S · 33:pub const AREA_MIN_DIAMETER_M · 36:pub const WIDE_AREA_MIN_DIAMETER_M · 38:pub const IDLE_NUDGE_S · 40:pub const REACHED_M · 46:pub const DOT_M · 47:pub const MAX_DOTS · 51:pub enum HintKind · 92:impl HintKind · 142:pub const PRIO_EVENT · 143:pub const PRIO_MISSION · 144:pub const PRIO_UNSTARTED · 145:pub const PRIO_OPTIONAL · 149:pub const PRIO_NIGHT · 151:pub const PRIO_FALLBACK · 155:pub struct Hint · 254:pub fn hiding_circle · 274:pub fn candidates · 812:pub const REPEAT_GUARD · 867:pub fn is_useful · 886:pub struct HintTracker · 909:impl HintTracker · 1116:pub fn walking_distance · 1136:pub struct ScreenPlace · 1148:pub fn screen_place · 1185:pub enum ProgressState · 1198:impl ProgressState · 1213:pub struct NightProgress · 1249:pub fn partial_animals · 1269:pub fn night_progress · 1353:pub fn compass_badge · 1373:pub fn what_next · 1395:pub fn step_key_for
- **`crates/zoo-core/src/level.rs`**: 16:pub struct Rect · 23:impl From< · 34:impl Rect · 67:pub fn cell_center · 72:pub fn cell_of · 79:pub enum ElementType · 91:impl ElementType · 113:pub struct Element · 202:pub struct TreeSpot · 207:impl TreeSpot · 213:impl Element · 289:pub enum Surface · 295:pub struct LevelHeader · 312:pub struct Spawn · 320:pub struct EntryData · 329:pub struct ItemData · 346:impl ItemData · 354:pub struct CartData · 377:impl CartData · 394:pub struct PropData · 407:impl PropData · 417:pub struct GardenData · 446:pub struct GardenProp · 453:impl GardenData · 474:pub struct GardenBedData · 490:pub struct PlantSpotData · 506:impl PlantSpotData · 515:pub struct CutSpotData · 527:impl CutSpotData · 544:pub struct WaterSourceData · 557:pub struct LightData · 584:impl LightData · 618:pub struct LevelPart · 630:impl Spawn · 638:pub fn facing_vec · 654:pub struct FoodBoxData · 663:impl FoodBoxData · 675:pub struct HidingPlaceData · 713:impl HidingPlaceData · 725:pub struct SceneryData · 738:pub struct EnclosureFeature · 759:impl EnclosureFeature · 777:pub struct LevelData · 831:pub const ZOO_ID · 835:pub const WATER_KINDS · 838:pub enum LevelError · 843:impl std::fmt::Display for LevelError · 852:impl std::error::Error for LevelError · 854:impl LevelData · 1126:pub enum CellKind · 1149:pub struct Grid · 1158:impl Grid · 1329:pub fn fence_edges · 1351:pub struct Level · 1377:pub const LANTERN_POST_RADIUS_M · 1379:impl Level · 1612:pub enum RunAxis · 1617:impl RunAxis · 1630:pub struct Segment · 1637:pub fn segment_run · … (+5 more — grep -n)
- **`crates/zoo-core/src/scene/models.rs`**: 9:pub const BUILDING_MODELS · 18:pub fn model_path · 29:pub struct BuildingSpec · 44:pub const BUILDING_SPECS · 94:pub fn rot_level · 100:pub fn model_offset · 107:pub fn building_model · 140:pub enum OpeningKind · 159:pub struct Opening · 173:impl Opening · 183:pub struct BuildingModel · 190:pub struct TextFace · 200:pub struct PlantPlacement · 211:pub struct KeyBoxPlacement · 223:pub struct BakeGroup · 231:impl LevelScene · 248:pub const GATE_ZOO_PILLAR_X · 249:pub const GATE_ZOO_PILLAR_HALF · 250:pub const GATE_ZOO_OPENING_M · 252:pub const GATE_ZOO_LEAF_HZ · 255:pub fn is_level_gate_barrier · 270:pub fn level_gate_pose · 297:pub const STORAGE_PLATFORM_M · 299:pub const GARDEN_SOIL_M · 301:pub const DOOR_WOOD_M · 303:pub const GATE_WOOD_M · 305:pub const MOON_DOOR_LIGHTS · 309:pub const TURNSTILE_X · 310:pub const TURNSTILE_Z · 312:pub const BOARD_LAMP_INFO · 313:pub const BOARD_LAMP_MAP · 316:pub const BOARD_LAMP_WALL · 318:pub const LANTERN_LIGHT · 320:pub const WALL_LAMP_LIGHT · 321:pub const WALL_LAMP_MOUNT_M · 323:pub const BOARD_LAMP_LIGHT · 325:pub const STRING_SPAN_M · 327:pub const DESK_NOTE · 328:pub const NIGHT_TABLE_LAMP · 329:pub const KEY_BOX_KEY · 330:pub const CART_KEY_RING · 332:pub const WINDOW_MOON_MOUNT_M · 333:pub const KEY_BOX_MOUNT_M · 335:pub const ENTRANCE_SIGN_KEY · 338:pub fn facing_str_yaw · 342:impl LevelScene · 786:pub fn moon_door_pose
- **`crates/zoo-core/src/scene.rs`**: 30:pub mod colors · 65:pub fn placeholder_kind · 109:pub fn landmark_model · 132:pub fn perch_scenery · 141:pub fn perch_point · 155:pub const PERCH_PAIR_OFFSET_M · 174:pub fn perch_point_of · 182:pub struct Placement · 195:impl Placement · 211:pub struct Fallback · 220:pub struct BoxPlacement · 235:pub enum DecalImage · 262:pub struct Decal · 271:impl Decal · 285:pub const FOOD_STORAGE_SIGN_KEY · 287:pub const NIGHT_HOUSE_SIGN_KEY · 289:pub const TERRARIUM_HOUSE_SIGN_KEY · 292:pub fn silhouette_path · 303:pub const DECAL_LIFT_M · 306:pub const FOOD_LABEL_HALF_M · 307:pub const FOOD_LABEL_Y_M · 309:pub const FOOD_LID_HALF_M · 310:pub const FOOD_LID_Y_M · 311:pub const FOOD_BOX_HALF_M · 315:pub const STORAGE_SIGN_BOARD · 317:pub const STORAGE_SIGN_BOTTOM_M · 322:pub struct LevelScene · 373:pub const WATER_WHEEL_RADIUS_M · 376:pub const WATER_WHEEL_AXLE_Y · 378:pub const WATER_WHEEL_WIDTH_M · 380:pub const WATER_WHEEL_PADDLE_M · 382:pub const WATER_WHEEL_PADDLES · 387:pub const WATER_WHEEL_SPIN · 389:pub const WATER_WHEEL_OFFSET_M · 395:pub struct WaterWheel · 409:impl WaterWheel · 457:pub const BRIDGE_PILES · 459:pub const BRIDGE_PILE_R · 461:pub const JETTY_POSTS · 463:pub const FOUNTAIN_WATER_Y · 467:pub enum Dir · 474:impl Dir · 522:pub fn path_tile · 546:pub fn water_tile · 578:pub const RIVER_ROCKS · 581:pub const RIVER_ROCK_SCALE · 583:pub const RIVER_ROCK_FOAM_R · 586:pub const POND_LILY_PADS · 617:pub fn facing_yaw · 658:pub fn walkable_side_row · 690:pub fn walkable_row_center · 726:pub fn door_facade · 743:pub fn bed_pose · 772:pub fn moon_door_axis · 787:pub fn map_board_pose · 793:pub fn info_board_pose · 810:pub const WALL_BOARD · 811:pub const WALL_BOARD_BOTTOM_M · 813:pub const WALL_PICTOGRAM_PLATE · 815:pub const INDOOR_PICTOGRAM_PLATE · … (+12 more — grep -n)
- **`crates/zoo-core/src/water.rs`**: 21:pub const BANK_M · 23:pub const BANK_SLOPE · 25:pub const CORNER_R · 27:pub const WATERLINE_INSET · 29:pub const TEXELS_PER_M · 31:pub const WATER_LOOP_S · 33:pub const SHORE_MIN · 34:pub const SHORE_MAX · 39:pub const STREAK_HALF_W · 42:pub fn water_time · 48:pub const WATER_PERIODS_S · 65:pub enum TileShape · 76:impl TileShape · 140:pub fn rotate_cw · 146:pub fn to_canonical · 156:pub struct WaterCell · 165:impl WaterCell · 183:pub struct StillWater · 188:impl StillWater · 203:pub struct Obstacle · 214:pub fn flow_dir · 230:pub enum PathPiece · 249:impl PathPiece · 343:pub struct RiverPath · 351:impl RiverPath · 430:pub fn river_paths · 630:pub struct WaterScene · 637:impl WaterScene · 663:pub struct WaterField · 681:impl CellGrid · 692:impl WaterField · 891:pub fn hash1 · 896:pub fn hash2 · 901:pub fn streak_lane · 908:pub fn river_streak_mask · 930:pub fn pond_ring_mask · 952:pub struct BobParams · 958:impl BobParams · 972:pub fn bob_params · 989:pub fn bob_hash · 1002:pub struct Bob · 1008:pub fn bob · 1022:pub fn bob_transform
- **`crates/zoo-render/src/camera.rs`**: 17:pub struct CameraParams · 31:impl Default for CameraParams · 45:pub const PULL_BACK_S · 48:pub const PULL_BACK_BODY_AT · 51:pub const LOOK_AT_HEIGHT_M · 52:pub const NEAR_M · 53:pub const FAR_M · 58:pub struct FollowCamera · 91:pub struct Pose · 102:impl Pose · 111:pub struct Fog · 132:impl FollowCamera · 532:pub fn project
- **`crates/zoo-render/src/renderer.rs`**: 28:pub const SUN_DIR · 30:pub const SHADOW_TINT · 32:pub const OUTLINE · 40:pub const MAX_PIXEL_RATIO · 43:pub const MAX_CROWD · 45:pub const MAX_WATER_OBSTACLES · 47:pub const MAX_WATER_RIPPLES · 52:pub struct Instance · 66:pub const HIDE_ROOF · 68:pub const HIDE_WALLS_UPPER · 70:pub const CART_STEER_MAX_RAD · 72:pub const MAX_NODE_PARTS · 76:pub struct NodeBehaviour · 89:impl NodeBehaviour · 182:pub struct StaticVertices · 191:impl StaticVertices · 213:pub fn bake_vertices · 241:pub const STATIC_VERTEX_FLOATS · 244:pub fn static_vertices · 324:impl Instance · 404:impl U · 492:impl Program · 594:pub struct FrameBlock · 719:impl Default for Region · 731:pub const REGION_ALWAYS · 786:impl Batch · 826:pub const MAX_CHUNK_RUNS · 829:pub const GAP_MERGE_TRIANGLES · 836:pub fn plan_chunk_runs · 875:pub fn chunk_order · 937:pub const CHUNK_M · 950:pub fn static_reach · 1004:pub fn instance_extent · 1062:pub struct CharacterDraw · 1096:pub const UNDER_WATER_DEPTH_BIAS_M · 1098:pub const UNDER_WATER_TINT · 1100:impl CharacterDraw · 1144:pub struct FrameStats · 1193:pub struct Renderer · 1264:pub struct InstanceHandle · 1270:pub const BOX · 1272:pub const CAPSULE · 1274:impl Renderer · 3130:impl Cull · 3237:pub fn decode_png · 3264:pub fn cylinder_mesh · 3316:pub fn butterfly_mesh · 3342:pub fn box_mesh · 3367:pub fn capsule_mesh
- **`crates/zoo-web/src/lib.rs`**: 38:pub const PLAYER_MODEL · 40:pub const ANIMAL_ANIMS · 70:pub const AMBIENT_MODELS · 134:pub fn animal_model_path · 140:pub fn level_animals · 151:pub fn join_levels · 163:pub fn required_assets · 260:impl AnimalView · 330:pub struct App · 450:impl Default for CartSockets · 461:impl App · 3212:impl App · 4766:impl App
- **`web/src/ui.ts`**: 16:export function introEnabled · 23:export interface UiApp · 84:export const WELCOME_PICTURES · 86:export const WELCOME_STEP_ICONS · 89:export const HINT_ICONS · 115:export interface HintView · 130:export function parseHint · 152:export const VIEW_ICONS · 154:export const STRIP_OPEN_MS · 157:export interface NightProgress · 171:export const STRIP_MAX · 174:export function missingAnimals · 179:export function stripView · 185:export function badgeIcon · 194:export function progressInfoKey · 207:export function parseProgress · 230:export const TREATS · 233:export interface Basket · 243:export function parseBasket · 259:export const LANGUAGES · 260:export const READING_LEVELS · 262:export const MATH_LEVELS · 263:export const DEFAULT_MATH_LEVEL · 266:export const FOOD_ICONS · 297:export const PLACE_ICONS · 352:export const BOWL_ICONS · 359:export const ANIMAL_ICONS · 379:export const TARGET_ICONS · 413:export function targetIcon · 427:export interface LyingIcon · 433:export function parseLyingIcons · 455:export interface KeyValue · 460:export interface Settings · 472:export const SAVED_VIEWS · 484:export function loadSettings · 508:export function saveSettings · 561:export interface GameEventMsg · 603:export const MIN_READ_PX · 610:export function fitReadingText · 641:export class Ui
- **`tools/blender/animals/chameleon.py`**: 128:def spiral · 138:def tail_radius · 142:class Rig · 169:def configure · 221:def NSS · 226:def C · 230:def put · 234:def paint_body · 260:def paint_tail · 270:def ring_centres · 274:def torso_z · 279:def w_body · 293:def eye_uv · 312:def tail_weights · 334:def limb_weights · 349:def build · 447:def pose · 451:def sc · 455:def ik_end · 468:def plant · 475:def tail_pose · 483:def eyes · 496:def lids · 503:def blink · 507:def clip_idle · 520:def foot_target · 533:def clip_walk · 553:def clip_perch · 570:def clip_eat · 590:def clip_happy · 608:def clip_refuse · 625:def clip_sleep · 642:def clips · 654:def main
- **`tools/blender/animals/quadruped_rig.py`**: 45:def leg_bones · 50:def _joint_table · 66:def mirror · 70:class Rig · 149:def rigid · 155:class Atlas · 212:def ring · 227:def ring_h · 231:class MeshBuilder · 324:def _face_verts · 336:def glow_material · 350:def body_material · 376:class Pose · 481:class Clip · 486:def apply_pose · 513:def set_variant · 517:def reshape · 528:def apply_variant · 547:def bake_clip · 575:class QuadGait · 624:def make_walk · 683:def export_animal · 690:def setup_preview · 744:def _look · 749:def game_view_dir · 756:def render_preview · 788:def render_debug
- **`tools/blender/animals/snake.py`**: 79:def bone_name · 94:def cr · 108:class Curve · 153:def smooth · 158:def tube_radius · 165:def rest_control_points · 194:def sleep_control_points · 209:def configure · 262:def C · 266:def put · 270:def NSS · 275:def band_layers · 291:def paint_body · 311:def paint_head · 331:def lat_up · 341:def eye_uv · 359:def build · 483:def pose · 487:def pose_from_joints · 513:def solve_chain · 536:def rest_joints · 540:def clip_idle · 555:def clip_eat · 569:def clip_happy · 584:def clip_refuse · 595:def clip_sleep · 618:def walk_speed · 622:def init_slither · 637:def slither_pose · 671:def clip_walk · 683:def clips · 694:def main
- **`tools/blender/animals/zebra.py`**: 118:def NSS · 126:def check_gate · 139:def _rgb · 143:def _canvas · 153:def paint_torso · 166:def paint_neck · 176:def paint_head · 189:def paint_leg · 201:def paint_tail · 210:def paint_eye · 227:def paint_mane · 243:def w_torso · 258:def neck_s · 262:def w_neck · 267:def w_leg · 278:def w_tail · 283:def w_ear · 292:def meridian_params · 302:def build_torso · 323:def build_neck · 339:def head_surface · 351:def build_head · 363:def build_eyes · 391:def build_ears · 412:def build_mane · 457:def build_legs · 477:def build_tail · 502:def build_forelock · 518:def build_mesh · 539:def tail_swish · 544:def ears · 553:def clip_idle · 570:def walk_upper · 583:def head_down_angle · 589:def _low_body · 599:def clip_eat · 617:def clip_drink · 631:def clip_happy · 652:def clip_refuse · 666:def clips · 679:def main
- **`tools/blender/characters/human_rig.py`**: 47:def arm_dir · 52:def _skeleton · 87:def build_armature · 118:def add_sockets · 139:def smoothstep · 144:def chain · 160:def mix · 170:def rigid · 174:def w_torso · 192:def w_neck · 196:def w_arm · 210:def w_leg · 220:class MeshBuilder · 316:def ellipse_ring · 324:def tube_ring · 333:def ellipsoid_rings · 346:def write_rgba_png · 366:def hex_rgb · 371:class BodyAtlas · 403:class Canvas · 474:def write_face_atlas · 487:def body_material · 502:def face_material · 525:def qaxis · 529:def rx · 535:def ry · 539:def rz · 544:def rot_between · 548:class Pose · 614:def two_bone · 627:def apply_pose · 648:def bake_clip · 670:def reset_pose · 678:def ease · 682:def envelope · 696:def base_stand · 704:def clip_idle · 720:class Gait · 799:def make_locomotion · 834:def clip_walk · 840:def clip_run · 847:def clip_pick_up · 869:def clip_give · 887:def clip_talk · 901:def clip_cheer · 925:def clip_wave · 939:def clip_carry · 961:def wheel_grip · 970:def clip_drive · 994:def player_clips · 1015:def export_character · 1064:def _face_toon · 1084:def _pose_at · 1094:def _render · 1109:def setup_preview · 1163:def render_preview · 1198:def hr_outline · 1202:def _compose · 1227:def render_debug
- **`tools/blender/props/kit_terrarium.py`**: 44:def stone · 56:def lump · 61:def quad · 66:def double · 73:def leaf_tent · 93:def log_beam · 116:def scaled · 120:def window · 134:def terrarium_house · 210:def terrarium_front · 222:def frame · 238:def lamp · 252:def terrarium_dish · 262:def floor_snake · 283:def rock_warm · 293:def rock_flat · 302:def branch_low · 321:def floor_frog · 348:def waterfall · 376:def mist_puff · 385:def leaf_big · 399:def moss_log · 416:def fern · 432:def floor_chameleon · 453:def branch_climb · 481:def tree_small · 504:def info_board_wall · 526:def fridge · 594:def front_items · 611:def level_placement · 625:def extra · 638:def place_cases · 653:def house_preview · 674:def builders · 706:def main
- **`tools/blender/props/night_lib.py`**: 61:def hex_rgb · 66:def lin · 75:def reset · 80:def _bsdf · 84:def material · 130:def face_quad · 143:def ring_xz · 166:def arc_points · 177:def wall_boxes · 211:class Asset · 247:def asset_of · 253:def build · 319:def descendants · 326:def mesh_objs · 330:def world_verts · 339:def tris · 343:def size · 354:def export · 375:def describe · 388:def report · 406:def _toon · 444:def _flat_emit · 462:def _assign · 493:def _render · 500:def render_preview · 656:def run · 703:def instance_tree

## Tests → spec test IDs

- `crates/zoo-assets/tests/audio.rs`: ASND-001…004, ASND-008, ASND-020…021
- `crates/zoo-assets/tests/footprints.rs`: LAYOUT-018
- `crates/zoo-assets/tests/invisible_walls.rs`: LAYOUT-017, LAYOUT-019, LAYOUT-L1-025
- `crates/zoo-assets/tests/models.rs`: AENV-013, AENV-016, AENV-018, APIPE-004, ARCH-006, CART-032, GARD-023
- `crates/zoo-assets/tests/pipeline.rs`: APIPE-006…007, APIPE-010
- `crates/zoo-assets/tests/terrarium_scene.rs`: AENV-019, LAYOUT-N2-025…027
- `crates/zoo-assets/tests/water_tiles.rs`: WATER-003, WATER-010
- `crates/zoo-core/tests/ads.rs`: ADS-001…002, ADS-005, ADS-007, ADS-035, ADS-037
- `crates/zoo-core/tests/ambient.rs`: AMB-001…004, AMB-006, AMB-008…010, NIGHT-013
- `crates/zoo-core/tests/arch.rs`: ARCH-001
- `crates/zoo-core/tests/baby_follow.rs`: FAM-011…013
- `crates/zoo-core/tests/baby_play.rs`: FAM-014…017, FAM-034
- `crates/zoo-core/tests/beds_indoors.rs`: LAYOUT-047, LAYOUT-L2-018, LAYOUT-L3-019, NIGHT-011, NIGHT-027
- `crates/zoo-core/tests/camera_views.rs`: CAMV-006, CAMV-008, L2-006, L3-006, LAYOUT-L1-006, LAYOUT-N1-006, PLAY-010
- `crates/zoo-core/tests/cart.rs`: CART-001…009, CART-011, CART-015…017, CART-019…021, CART-028, LAYOUT-048
- `crates/zoo-core/tests/cart_fuzz.rs`: CART-018
- `crates/zoo-core/tests/cart_key.rs`: CART-012…013, CART-022…027, MATH-011, SAVE-005, SAVE-012…013
- `crates/zoo-core/tests/content.rs`: ANIM-006…007, L10N-001…003, L10N-005, MISS-001, MISS-004, MISS-007…008, READ-002, RESC-003, RESC-011
- `crates/zoo-core/tests/coords.rs`: LAYOUT-007…011
- `crates/zoo-core/tests/feeding_drop.rs`: FEED-004, FEED-009…023
- `crates/zoo-core/tests/feeding_spot.rs`: GARD-010, GARD-013…014
- `crates/zoo-core/tests/gameplay_qa.rs`: FAM-001, GARD-009, PLAY-020, PLAY-031, RESC-006, RESC-017, RESC-025, RESC-028
- `crates/zoo-core/tests/garden.rs`: FAM-008, FAM-030, GARD-005, GARD-007, GARD-009…011, GARD-015…025, HINT-026, LAYOUT-L3-030, RESC-005
- `crates/zoo-core/tests/ground.rs`: PLAY-035
- `crates/zoo-core/tests/hints.rs`: CART-029, FAM-011, FEED-024…025, HINT-001…008, HINT-010…012, HINT-014…015, HINT-017, HINT-019…021, HINT-025…029, HINT-032…033, HOUSE-016, NIGHT-019, NIGHT-021, NIGHT-028, NIGHT-031
- `crates/zoo-core/tests/houses.rs`: HINT-019, HINT-021, HOUSE-001…017, HOUSE-020
- `crates/zoo-core/tests/layout.rs`: LAYOUT-001…006, LAYOUT-014…016, LAYOUT-019…020, LAYOUT-023, LAYOUT-026, LAYOUT-040, LAYOUT-L1-001…005, LAYOUT-L1-007…010, LAYOUT-L1-013…018, LAYOUT-L1-020…024, LAYOUT-L1-044
- `crates/zoo-core/tests/level_gates.rs`: LAYOUT-036, LAYOUT-040
- `crates/zoo-core/tests/level_start.rs`: ANIM-013, LAYOUT-044…046
- `crates/zoo-core/tests/levels23.rs`: AENV-014, AENV-016, L3-008, LAYOUT-005, LAYOUT-014, LAYOUT-021…024, LAYOUT-040…041, LAYOUT-046…047, LAYOUT-L2-001…011, LAYOUT-L2-013…014, LAYOUT-L2-016, LAYOUT-L3-001…013, LAYOUT-L3-015
- `crates/zoo-core/tests/math.rs`: CART-012, CART-023, CART-026, CART-031, L10N-001, MATH-001, MATH-006…013
- `crates/zoo-core/tests/modular.rs`: LAYOUT-012…013
- `crates/zoo-core/tests/night.rs`: HINT-030, LAYOUT-047, LAYOUT-L2-018, NIGHT-001…004, NIGHT-006…008, NIGHT-010…011, NIGHT-015…018, NIGHT-033…034
- `crates/zoo-core/tests/night1_layout.rs`: CAMV-008, LAYOUT-005, LAYOUT-027…030, LAYOUT-040, LAYOUT-N1-001…013, LAYOUT-N1-015…017
- `crates/zoo-core/tests/night2_content.rs`: ANIM-003, ANIM-007, MISS-007, MISS-013, READ-002, RESC-011
- `crates/zoo-core/tests/night2_game.rs`: FAM-021…024, FAM-026, FAM-031, HINT-024…025, MAP-010, NIGHT-030…032
- `crates/zoo-core/tests/night2_layout.rs`: CAMV-008, LAYOUT-005, LAYOUT-N1-006, LAYOUT-N1-014, LAYOUT-N2-001…012, N2-014
- `crates/zoo-core/tests/night2_round_house.rs`: AENV-017, LAYOUT-003, LAYOUT-031, LAYOUT-N2-020…024
- `crates/zoo-core/tests/openings.rs`: CAMV-022, LAYOUT-028, LAYOUT-031…036, LAYOUT-038…039, LAYOUT-041, LAYOUT-047
- `crates/zoo-core/tests/overview.rs`: MAP-005, MAP-009…016
- `crates/zoo-core/tests/pairs.rs`: FAM-021…029, GARD-014, LAYOUT-014, RESC-014, RESC-018
- `crates/zoo-core/tests/panel.rs`: PLAY-023…024, PLAY-026…027, RESC-012
- `crates/zoo-core/tests/player.rs`: LAYOUT-033, LAYOUT-041, PLAY-003, PLAY-005…007, PLAY-010, PLAY-019…022, RIG-016
- `crates/zoo-core/tests/rescue.rs`: ANIM-001…005, ANIM-009…010, FEED-001…008, FEED-027…029, FEED-035, LAYOUT-032, LAYOUT-041, LAYOUT-L1-010, READ-003, RESC-001…002, RESC-004…009, RESC-012…013, RESC-015
- `crates/zoo-core/tests/save.rs`: LAYOUT-041, RESC-029, SAVE-001, SAVE-005…007, SAVE-009
- `crates/zoo-core/tests/sound.rs`: ASND-010…016, ASND-022
- `crates/zoo-core/tests/telescope.rs`: TELE-001…006
- `crates/zoo-core/tests/wander.rs`: ANIM-008…012, RESC-014, RESC-016
- `crates/zoo-core/tests/water.rs`: AENV-007…008, WATER-001…002, WATER-004…005, WATER-009
- `crates/zoo-core/tests/water_wheel.rs`: LAYOUT-L3-017
- `crates/zoo-core/tests/welcome_board.rs`: HINT-001, READ-002, RESC-017, RESC-028
- `crates/zoo-core/tests/zfight.rs`: ARCH-005
- `crates/zoo-core/tests/zoo_game.rs`: FAM-001…002, FAM-008…009, GARD-014, HINT-018…019, HINT-022, LAYOUT-025, LAYOUT-L2-015, NIGHT-026, NIGHT-029, RESC-002…003, RESC-014, RESC-017…023, RESC-025…027, RESC-030, RESC-032…033, SAVE-010
- `web/src/ads.test.ts`: ADS-004, ADS-008…019, ADS-024…025, ADS-031, SHA-256
- `web/src/analytics.test.ts`: PLAT-022…027, PLAT-030
- `web/src/audio-ambient.test.ts`: ASND-023…026
- `web/src/audio.test.ts`: ASND-007, ASND-009, ASND-017, ASND-019
- `web/src/deploy.test.ts`: ADC1-001, ADC2-001, ASND-018, PLAT-003…008, PLAT-010…016, PLAT-019, PLAT-028…029, PLAT-032, SHA-256
- `web/src/fullscreen.test.ts`: PLAT-017…018
- `web/src/input.test.ts`: ARCH-002, CAMV-010, PLAY-015…017
- `web/src/overview-map.test.ts`: MAP-020
- `web/src/pictograms.test.ts`: FEED-031…032
- `web/src/quality.test.ts`: PERF-022
- `web/src/save.test.ts`: RESC-014, SAVE-005
- `web/src/scroll.test.ts`: PLAY-032…033
- `web/src/telescope.test.ts`: TELE-011
- `web/src/text.test.ts`: —
- `web/src/ui.test.ts`: ASND-009, CAMV-011, CAMV-025, CART-010, CART-014, CART-030, FEED-001, GARD-015, GARD-026, HINT-007, HINT-016, HINT-023, HINT-028, L10N-005, LAYOUT-N2-015, NIGHT-019, NIGHT-022…023, RESC-029
- `web/tests/e2e/ads.spec.ts`: ADC1-002…006, ADC2-002…003, ADC3-002…004, ADS-001…004, ADS-020…022, ADS-025, ADS-030, PLAT-012
- `web/tests/e2e/ads_carousel.spec.ts`: ADS-032…034, HINT-028, HINT-031
- `web/tests/e2e/ads_phone.spec.ts`: ADC1-004, ADS-026…030
- `web/tests/e2e/ads_prod.spec.ts`: ADS-023
- `web/tests/e2e/analytics.spec.ts`: PLAT-022…024, PLAT-027, PLAT-029…031, PLAT-033
- `web/tests/e2e/audio.spec.ts`: ASND-005…007, ASND-009…010, ASND-027, NIGHT-024
- `web/tests/e2e/camera_views.spec.ts`: CAMV-012…014, CAMV-019, CAMV-022, CAMV-025, CAMV-028, LAYOUT-043
- `web/tests/e2e/cart.spec.ts`: CAMV-024, CART-002, CART-010…011, CART-019…021, CART-028
- `web/tests/e2e/cart_key.spec.ts`: CART-013…014, CART-022, CART-024, CART-026, CART-030, HINT-032, MATH-010
- `web/tests/e2e/feeding.spec.ts`: FEED-009, FEED-016…017, FEED-019, LAYOUT-L3-018
- `web/tests/e2e/food-pictograms.spec.ts`: FEED-030, FEED-033…034
- `web/tests/e2e/fullscreen.spec.ts`: PLAT-017, PLAT-019…021
- `web/tests/e2e/gameplay/controls.spec.ts`: LAYOUT-L1-011, PLAY-005, PLAY-011, PLAY-015…016
- `web/tests/e2e/gameplay/doors.spec.ts`: FEED-027…028, LAYOUT-032, LAYOUT-035, LAYOUT-041
- `web/tests/e2e/gameplay/fruit-garden.spec.ts`: GARD-022
- `web/tests/e2e/gameplay/gifts.spec.ts`: FAM-011, GARD-012
- `web/tests/e2e/gameplay/pairs-intro.spec.ts`: FAM-001…002, FAM-008, GARD-010, RESC-029
- `web/tests/e2e/gameplay/panel.spec.ts`: ADIR-003, PLAY-023, PLAY-025, PLAY-030, PLAY-038, RESC-017
- `web/tests/e2e/gameplay/qa.ts`: PLAY-023
- `web/tests/e2e/gameplay/welcome-scroll.spec.ts`: RESC-028
- `web/tests/e2e/gameplay/zebra-mission-levels.spec.ts`: POC-002…003, RESC-006…008, RESC-010
- `web/tests/e2e/gates.spec.ts`: LAYOUT-031
- `web/tests/e2e/ground.spec.ts`: PLAY-036
- `web/tests/e2e/helpers.ts`: —
- `web/tests/e2e/hints.spec.ts`: HINT-007, HINT-009, HINT-013…014, HINT-023, NIGHT-016, NIGHT-020…023
- `web/tests/e2e/intro.spec.ts`: RESC-029, RESC-031
- `web/tests/e2e/level_gates.spec.ts`: LAYOUT-036…037, LAYOUT-042
- `web/tests/e2e/m5a.spec.ts`: ANIM-006, ANIM-011…012, RESC-010, RESC-014…017, RESC-024
- `web/tests/e2e/m5b.spec.ts`: LAYOUT-022, LAYOUT-L2-010, LAYOUT-L3-014, PLAY-028…029, RESC-010, RESC-018…020, RESC-023, RESC-026, SAVE-011
- `web/tests/e2e/map.spec.ts`: MAP-006, MAP-017…021
- `web/tests/e2e/mission.spec.ts`: ADIR-003, ANIM-006, L10N-005, PLAY-010, PLAY-014, PLAY-023, PLAY-027, POC-002…003, RESC-005, RESC-010
- `web/tests/e2e/night.spec.ts`: NIGHT-001…005, NIGHT-008…009, NIGHT-014, NIGHT-016, PLAY-014
- `web/tests/e2e/night2.spec.ts`: AENV-019, LAYOUT-N2-012…013, LAYOUT-N2-023, LAYOUT-N2-028
- `web/tests/e2e/night_ads.spec.ts`: ADS-002, ADS-036
- `web/tests/e2e/panel.spec.ts`: PLAY-023, PLAY-025, PLAY-038
- `web/tests/e2e/perf/perf.config.ts`: —
- `web/tests/e2e/perf/scenarios.perf.ts`: —
- `web/tests/e2e/perf_rules.spec.ts`: PERF-016…017, PERF-025, PERF-R-001…003, PERF-R-015, PERF-R-018
- `web/tests/e2e/play_landmarks.spec.ts`: AENV-015
- `web/tests/e2e/quality.spec.ts`: PERF-022, PERF-R-005
- `web/tests/e2e/review.spec.ts`: —
- `web/tests/e2e/save.spec.ts`: SAVE-002…004, SAVE-008
- `web/tests/e2e/signs.spec.ts`: AENV-011…012, ARCH-006
- `web/tests/e2e/small_screen.spec.ts`: CAMV-027, HINT-023, PLAY-037
- `web/tests/e2e/smoke.spec.ts`: ARCH-003, LAYOUT-L1-011, POC-001
- `web/tests/e2e/telescope.spec.ts`: TELE-007…010, TELE-012
- `web/tests/e2e/touch.spec.ts`: PLAY-013…019
- `web/tests/e2e/water.spec.ts`: AENV-007, AENV-009…010, AMB-005, AMB-007, WATER-006…008, WATER-011
- `web/tests/e2e/welcome_board.spec.ts`: RESC-028

## Big specs: sections (line numbers)

- **`specs/10-gameplay/golf-carts.md`** (327 lines): 14:Goal · 38:Behaviour · 137:The golf-cart key (user request 2026-09- · 195:Settings: math level (decided 2026-10-06 · 207:Data shape: `[[cart]]` (GAME-LAYOUT "Nig · 225:Save (GAME-SAVE, **format version 3**) · 236:Fluent keys (de + en, `assets/i18n/<lang · 246:Work packages (implementation order) · 260:Never stuck / no hint loops (binding, us · 279:Test cases · 316:Open questions
- **`specs/10-gameplay/layout.md`** (712 lines): 20:Goal · 27:Coordinate system · 33:Coordinate spaces (decided, Q-056, user  · 54:Modular edges: fences, hedges, walls (de · 82:Element types · 164:Joining levels (Q-088, confirmed 2026-10 · 237:Level design rules — how a level is play · 314:Enterable buildings (user request 2026-0 · 347:Moon door and night levels (level design · 376:Night lights, interactables and furnitur · 415:Levels · 432:Level progression (overview, 2026-10-04) · 446:Behaviour · 464:Forests: dense vs. walkable (user decisi · 492:Enclosure features and wandering at home · 512:Collision footprints (Q-087, confirmed 2 · 554:Gates and doors (user request 2026-09-27 · 645:Test cases · 698:Open questions
- **`specs/10-gameplay/levels/level-1.md`** (928 lines): 21:Goal · 57:Design assumptions of this level (Q-numb · 85:Spawn and camera · 94:Map · 189:Elements · 261:Hiding places (candidates) · 346:Hiding places — riddle details and sight · 378:Barriers · 392:Walking distances · 435:High-angle camera (Q-049 answered) · 461:Behaviour · 527:Hippo enclosure pool (user decision 2026 · 574:Woods (user decision 2026-09-26: dense v · 608:Collision and billboards (GAME-LAYOUT "C · 653:Vegetable garden (user request 2026-09-2 · 748:Zookeeper house (Q-096 answered; GAME-NI · 791:Moon door (GAME-NIGHT rules 3, 7; Q-133  · 816:Night lights (GAME-NIGHT rule 1; Q-118 a · 835:Burglar event (GAME-EVENTS rules 4–7; Q- · 845:Pairs (GAME-FAMILY, Q-308 / Q-280, 2026- · 855:Golf cart (GAME-CART, 2026-10-06) · 867:Test cases · 911:Open questions
- **`specs/10-gameplay/levels/level-2.md`** (544 lines): 21:Goal · 50:Design assumptions of this level (Q-numb · 64:Spawn and camera · 75:Map · 155:Elements · 211:Level entry and exit · 218:Hiding places (candidates) · 286:Hiding places — riddle details and guard · 316:Barriers · 326:Walking distances · 347:Food storage 2 (Q-089, confirmed 2026-10 · 364:Bed of level 2 (Q-141 answered, option b · 397:Elephant pool (user decision 2026-09-26: · 404:High-angle camera · 413:Behaviour · 444:Night lights and burglar event (GAME-NIG · 461:Pairs (GAME-FAMILY, Q-308 / Q-280, 2026- · 468:Golf cart (GAME-CART, 2026-10-06) · 480:Test cases · 504:Implementation status (M5b, 2026-09-26) · 530:Open questions
- **`specs/10-gameplay/levels/level-3.md`** (494 lines): 21:Goal · 54:Design assumptions of this level (Q-numb · 69:Spawn and camera · 77:Map · 153:Elements · 202:Fruit garden (user request 2026-10-01, Q · 227:Level entries · 238:Hiding places (candidates) · 299:Hiding places — riddle details and guard · 323:Water sources and the fish bowl (Q-093,  · 336:Barriers · 342:Walking distances · 365:Behaviour · 387:Night lights and burglar event (GAME-NIG · 403:Pairs (GAME-FAMILY, Q-308 / Q-280, 2026- · 410:Golf cart (GAME-CART, 2026-10-06) · 422:Test cases · 448:Implementation status (M5b, 2026-09-26) · 473:Open questions · 489:Ideas (not decided)
- **`specs/10-gameplay/levels/night-1.md`** (495 lines): 22:Goal · 43:Design assumptions of this level (Q-numb · 57:Spawn, entry and camera · 68:Map · 147:Elements · 211:Secret trail (user request 2026-10-04, Q · 246:Night house (GAME-NIGHT rule 4; Q-134 an · 274:Night food storage (Q-135 answered; boxe · 294:Hiding places (candidates) · 338:Hiding places — riddle details (night cl · 357:Terrarium house (user request 2026-10-03 · 363:Barriers · 369:Walking distances · 398:Night lights (GAME-NIGHT rule 1, Q-118,  · 408:Night riddles and the haze (Q-126, CAMV- · 415:Behaviour · 441:Mockups · 448:Pairs (GAME-FAMILY, Q-308 / Q-280, 2026- · 457:Test cases · 479:Open questions
- **`specs/10-gameplay/levels/night-2.md`** (536 lines): 29:Goal · 46:Why a second night level (decision with  · 62:Design assumptions · 74:Spawn, entry and camera · 80:Map · 152:Elements · 197:Scenery (non-solid `[[scenery]]` entries · 207:Terrarium house · 260:Terrarium models (AENV-018 / AENV-019, p · 287:Sign pictograms (LAYOUT-N2-022, ART-ENVI · 296:Night food storage · 304:Hiding places · 323:Riddle details (night clues) · 339:Riddles and facts (texts in `assets/i18n · 345:`snake` — facts · 354:`snake` — riddles · 371:`chameleon` — facts · 380:`chameleon` — riddles · 397:`poison_dart_frog` — facts (the word "gi · 406:`poison_dart_frog` — riddles · 427:Barrier (edit of night_1) · 435:Walking and pacing · 456:Lights · 460:Never stuck (binding) · 472:Behaviour · 486:Mockups and art · 490:Test cases · 527:Open questions · 532:Model needs (round terrarium house, 2026
- **`specs/20-content/missions/start-missions.md`** (973 lines): 53:Overview · 73:1. Zebra — `loc_river`, `loc_meadow`, `l · 131:2. Hippo — `loc_pond`, `loc_mud`, `loc_s · 190:3. Panda — `loc_cave`, `loc_bamboo`, `lo · 252:4. Koala (pair) — `loc_treehouse`, `loc_ · 307:5. Elephant — `loc_fountain`, `loc_log_p · 360:6. Goldfish — `loc_waterfall`, `loc_wate · 426:7. Monkey — `loc_pirate_ship`, `loc_caro · 481:8. Giraffe — `loc_lookout_tower`, `loc_t · 536:9. Lion — `loc_sun_rocks`, `loc_stage`,  · 589:10. Snow fox — `loc_ice_cream_kiosk`, `l · 644:Night level 1 — hedgehog, bat, owl (GAME · 664:N1. Igel / Hedgehog — `loc_brush_pile`,  · 714:N2. Fledermaus / Bat — `loc_windmill`, ` · 764:N3. Eule / Owl — `loc_moon_pond`, `loc_h · 814:Night texts (GAME-NIGHT) and burglar tex · 929:Behaviour · 946:Test cases · 965:Open questions
- **`specs/30-art/animals.md`** (358 lines): 17:Goal · 21:Asset list · 51:Game sizes (user decision 2026-09-26) · 86:Behaviour · 95:Rig conventions (quadrupeds) · 166:Zebra model (v1) · 188:Family models (GAME-FAMILY, 2026-09-30) · 209:Models v1 (hippo, panda, koala, elephant · 231:Night animals (models v1) · 285:Ambient animals (GAME-AMBIENT, M6) · 301:Terrarium animals (night_2, proposal 202 · 319:Family models still missing (Q-280, Q-28 · 330:Test cases · 348:Open questions
- **`specs/30-art/character-rig-and-animation.md`** (308 lines): 17:Goal · 28:Behaviour · 30:1. Coordinate system and units · 39:2. Skeleton (`human` rig) · 112:3. Mesh and skinning · 133:4. Animation clips · 188:5. Clip ownership and budgets · 198:6. Facial expressions · 221:7. Export settings (Blender glTF exporte · 239:8. Reference implementation (model v1) · 255:Acceptance criteria · 267:Test cases · 295:Open questions
- **`specs/30-art/environment.md`** (321 lines): 17:Goal · 22:Mockups required (before modelling) · 87:Hiding places (must appear in a mockup) · 132:Modular props (modelled once, reused) · 154:Built kits (scripted, `tools/blender/pro · 206:Night art (GAME-NIGHT) · 235:Behaviour · 289:Test cases · 313:Open questions
- **`specs/40-tech/water-rendering.md`** (414 lines): 17:Goal · 31:1. Starting point (PoC M3) · 49:2. Approaches compared · 78:3. Decision (recommended combination) · 105:Behaviour · 191:4. Shader specification · 193:Uniforms (water program) · 208:Pseudo-GLSL (fragment) · 251:Parameters (summary) · 264:5. Implementation plan (after M4) · 266:zoo-core (level assembly — `scene.rs` li · 285:zoo-render · 305:zoo-web · 312:Blender water kit (`tools/blender/props/ · 325:Level data / assembly · 333:Cost estimate · 345:Acceptance criteria · 353:Test cases · 380:Implementation (M6, 2026-09-26) · 409:Open questions
- **`specs/50-performance/measurements.md`** (764 lines): 18:How to measure (one command) · 83:Run 2026-09-27 — baseline · 100:Sizes and load · 122:Scenarios — desktop 1920 × 1080 (load av · 140:Scenarios — phone 720 × 1560 drawn (load · 158:Scenarios — desktop_half 960 × 540 (load · 166:Native probe (x86-64 release; WASM is sl · 187:Static draw list (`debug_draw_list`, tri · 198:Findings vs. budgets · 216:Where the time goes (relative, SwiftShad · 236:Run 2026-09-28 — PERF-R-001 + PERF-R-003 · 264:Look (identical?) · 276:GL traffic per frame (census, exact; bef · 314:Frame time — interleaved A/B (fenced fra · 347:Findings vs. budgets (budgets of Q-166…Q · 363:Run 2026-09-28 (2) — PERF-R-002 (a), PER · 382:Counts (`tools/perf/run.sh`, before = ba · 404:Look (`look.mjs`, final build vs. the sh · 411:Frame time — interleaved A/B on the real · 431:Turning flicker (user report 2026-09-28) · 447:Findings vs. budgets · 458:Run 2026-09-29 — PERF-R-018 on (Q-193),  · 468:Turning flicker — `turn.mjs flicker`, 48 · 477:Cost of the prototypes — real iGPU, `loo · 487:Run 2026-09-30 — families (2 zebras, 2 k · 503:Sizes and load · 520:Counts (draw calls / triangles), desktop · 543:Timings (SwiftShader, busy machine — not · 553:Allocations and model budgets · 563:Findings vs. budgets · 581:Run 2026-09-30 (2) — ad boards, baby / f · 593:Sizes and load · 615:Counts (draw calls / triangles), desktop · 638:Allocations · 648:Findings vs. budgets · 667:Run 2026-10-02 — every animal a pair (26 · 679:Sizes and load · 697:Counts (draw calls / triangles), desktop · 726:Allocations · 737:Findings vs. budgets · 756:Test cases · 761:Open questions
- **`specs/50-performance/recommendations.md`** (490 lines): 14:Goal · 23:Behaviour · 32:Recommendations · 55:PERF-R-001 — Night point lights: cull pe · 88:PERF-R-002 — Ground tiles: draw only the · 131:PERF-R-003 — Shared per-frame uniforms ( · 161:PERF-R-004 — Zero per-frame heap allocat · 186:PERF-R-005 — Pixel-ratio quality tier fo · 212:PERF-R-006 — Full-screen pass: one fewer · 225:PERF-R-007 — Skinned animals: `eye_glow` · 237:PERF-R-008 — Animation: skip clips with  · 248:PERF-R-009 — Dynamic batches: upload onl · 259:PERF-R-010 — Triangle budget test (APIPE · 271:PERF-R-011 — Download: drop the embedded · 282:PERF-R-012 — WASM size: profile before o · 293:PERF-R-013 — Measurement: real-GPU runs, · 305:PERF-R-014 — Night light edges: derivati · 336:PERF-R-015 — Conservative culling boxes  · 358:PERF-R-018 — Haze culling in the close v · 379:PERF-R-016 — Smooth turning: outline ant · 425:PERF-R-017 — Frame pacing while turning · 437:PERF-R-019 — Headroom watch: WASM size,  · 454:PERF-R-020 — JS heap 61 - 64 MB in the r · 467:PERF-R-021 — Draw-call creep from pairs  · 478:Acceptance criteria · 483:Test cases · 488:Open questions
