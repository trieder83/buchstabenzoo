# Night art plan (GAME-NIGHT)

Which art changes at night, and which new art the night needs. Spec: GAME-NIGHT (rules 1, 2, 3,
5, 10), ART-ENVIRONMENT "Night art", ART-DIRECTION. Status: **plan / proposal** — written
2026-09-26; decisions marked *Q-…* are open questions for the user.

**Basic rule (GAME-NIGHT §10):** night is a **renderer mode**, not a second set of models. The
renderer changes the global light (blue moonlight ambient + key from the upper left, one hard
blue shadow tone), adds **point lights with hard cartoon falloff** (lanterns, lamps, the
player's lantern) and switches on **emissive areas** (windows, lamp glass, eyes). Cel shading
and outlines stay. So every existing model keeps working at night; new art is only needed where
something **glows**, where something exists **only at night**, or where a model must **show a
night state** (sleeping pose, open moon door).

Categories used below:

- **(a) no new art** — the renderer darkens/tints it (night light mode).
- **(b) night variant or emissive parts** — the existing model gets an emissive texture area,
  an attached lamp, or a night pose; no second model.
- **(c) night-only new asset** — a new model/texture/effect that exists (or is used) only at
  night.

## Night lighting paragraph (for every night image prompt)

The STYLE block of `art/style/style.md` is copied verbatim (APIPE-010) and says "Bright warm
midday sunlight". Every night prompt therefore adds this **separate paragraph directly after
the STYLE block** (same text in every night brief, so it can later become a NIGHT STYLE block
in `style.md` — *Q-113*):

> NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night.
> There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette —
> medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of
> objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly
> visible with its bold dark outlines and flat colours. The second light is warm yellow-orange
> lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round,
> hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for
> young children — nothing scary, nothing hidden in darkness.

Prop sheets are generated **twice**: once in the normal sheet light (for modelling — emissive
parts drawn as flat bright yellow) and once on a night background (to show the glow and the
light pool). Night negatives add, before the verbatim NEGATIVE suffix: *daylight, pitch black,
deep black shadows, horror, scary, spooky, halloween, creepy, menacing shapes, glowing red eyes,
monsters, ghosts, skulls, bats swarming, fog*.

## Night colours (proposal — renderer constants later, ADIR palette)

| Use | Colour | Notes |
|---|---|---|
| Moonlight ambient / key | `#5B6FB8` ambient, `#AFC3FF` key | deep friendly blue; darkest shadow tone not below `#2B3566` (never black) |
| Lantern / lamp glow (emissive) | `#FFD66B` core, `#FFB547` edge | flat bright shape, no bloom texture |
| Lantern light pool (point light) | `#FFC46E` | hard-edged circle, radius 3 m (post) / 2.5 m (player) / 1.2 m (board lamp) |
| Lit window (emissive) | `#FFC857` | flat pane, frame stays wood-coloured |
| Moon | `#FFF4C9` disc, dark-brown outline | flat comic disc, a few flat crater spots `#EADFAF` |
| Stars | `#FFFFFF` / `#FFF1B8` | 4-point comic sparkles, also reflected on water |
| Fireflies | `#EFFF8A` | tiny flat dots, bobbing |
| Eyeshine | `#E6F7A0` highlight over the dark iris | warm pale gold-green; **never red** |
| Moon door glow | `#FFF4C9` moon sign + `#8FB8FF` soft blue rim | only when open/opening |
| Night house indoor light | `#5B7FE0` blue, `#E8735A` warm red-orange | dim but readable (see `env_night_house`) — red kept warm and soft (*Q-116*) |
| Night sky (close views only) | top `#1E2A5A`, band `#2E3E7A`, horizon `#3B4C8C` | flat bands (no gradients), comic outline on clouds; haze = horizon colour |

## Plan per asset group

| Group | Existing assets | Cat. | Night treatment | New asset id(s) | Brief / sheet | Prio night_1 |
|---|---|---|---|---|---|---|
| Ground and paths | `kit_ground` tiles, `path_edge`, `mud_tile`, decals | **a** | Tinted by the night light; lantern pools are point lights on the ground (no decal). Fallback for weak phones: flat `light_pool_decal` under each lantern (*Q-114*). | (`light_pool_decal`, fallback only) | — | P3 |
| Fences, hedges, walls | `kit_fences` | **a** | Tinted only. String lights may hang on fences (see below). | — | — | — |
| Signs and boards | `info_board` | **b** | Every info board gets a small warm **board lamp** (GAME-NIGHT rule 5) — a separate prop attached to a `socket_lamp` empty on the board, visible only at night (lamp head + emissive glass + 1.2 m point light on the panel). Panel text is rendered by the game as by day (NIGHT-005). | `board_lamp` | `art/props/kit_night` | **P1** |
|  | `map_board` | **b** | Same `board_lamp` under the small roof. | `board_lamp` (reuse) | `kit_night` | P2 |
|  | `enclosure_sign` | **a** | Tinted; readable because a lantern post stands at every enclosure gate (layout rule proposal, *Q-118*). | — | — | — |
|  | `food_box`, `food_box_stack` | **a** | Inside the lit food storage; label rendered by the game. | — | — | — |
| Nature | `kit_nature`, `wildflowers`, `leaf_pile`, trees | **a** | Tinted. Night-level scenery (old hollow tree, night-blooming flowers, logs, bamboo…) comes from the `night_1` layout by the level designer, not from a night variant. | night_1 scenery (level designer) | `env_night_overview` | P2 |
| Water | `kit_water` tiles, `bridge_wood`, `jetty_wood`, `lily_pad` | **b** (renderer only) | Water shader night mode: blue-violet water, **stars reflected** as comic sparkles (rule 1), **lantern reflections** as warm wobbling stripes under each nearby light. No new models. | — (shader) | style frame night | **P1** |
| Ambient animals | `duck`, `frog`, `butterfly`, `bee` | **b / c** | Ducks sleep (head tucked, static pose in the same mesh), butterflies and bees hidden at night, frogs stay (croak). New: **fireflies** (c) over meadows, hedges and the pond. | `firefly` (particle, ≤ 4 tris) | `kit_night` | P2 (*Q-115*) |
| Barriers | `kit_barriers` | **a** | Tinted. The night-zoo gate is a new barrier (c, below). | — | — | — |
| **Moon door** (night-zoo gate) | — | **c** | Big friendly wooden double gate in the day-zoo wall with a round **moon sign** on top; states **closed** (dark sign, by day), **opening** (sign glows, doors swing), **open** (glowing sign, warm lanterns on both pillars, soft blue sparkle in the opening). Animated leaves like `gate_wood`. | `moon_door` | `kit_night` | **P1** |
| Path lighting | — | **c** | **Lantern post** along the paths (switch on at dusk, GAME-NIGHT rule 1); **string lights** between trees/posts over plazas. How many / where: *Q-118*. | `lantern_post`, `string_lights` | `kit_night` | **P1** / P2 |
| Buildings | `entrance_arch`, `zookeeper_house`, `food_storage_building` | **b** | **Lit windows** as emissive texture areas (window panes use a separate emissive material slot, off by day), a small **wall lamp** by each door. Cut-away interiors get a ceiling lamp point light. | `wall_lamp` (reuse on all buildings) | `kit_night` | **P1** (zookeeper house, food storage) |
| Zookeeper house interior | `zookeeper_house` cut-away | **c** | Bedroom corner for the **bed** (GAME-NIGHT rule 3): bed, night table with a warm lamp, window with a moon and stars outside, rug, chest, clothes hook — shown by day and night (the bed is used at night). | `bed`, `night_table`, `bedside_lamp`, `window_moon`, `rug_round`, `toy_chest` | `art/props/kit_bedroom` | **P1** |
| Enclosure buildings | `stone_arch_shelter`, `hut_wood`, `pool_tiled`, `panda_shelter`… | **b** (small) | One `wall_lamp` at each shelter (as in the night style frame), lit window on `hut_wood`; rest tinted. | `wall_lamp` (reuse) | `kit_night` | P2 |
| **Night house** | — | **c** | Dim indoor enclosure building of `night_1` with soft blue and warm red-orange light (like real nocturnal houses), glass-fronted indoor enclosures, enterable (roof disappears, GAME-PLAYER §2). | `night_house` (+ `night_house_cutaway`) | `art/environment/env_night_house` | **P1** |
| Night level mood | — | **c** | Overview mockup of `night_1` (moonlit forest garden: night house, pond, old trees, meadow, small hill, lantern-lit paths, moon door) — sets the mood for the level designer's layout. | `env_night_overview` | `art/environment/env_night_overview` | **P1** |
| Characters | `player_girl`, `player_boy` | **a + c** | Player tinted like everything else, plus a **light circle** (point light around her, GAME-NIGHT rule 2). New: the **hand lantern** she carries (c), and a `sleep` / `lie_down` animation for the bed (NIGHT-003, character-artist, ART-RIG). Visitors are not shown at night (proposal). | `hand_lantern` (+ anim `sleep`) | `kit_night`; anim → ART-RIG | **P1** (*Q-117*) |
| Day animals | 10 day animals + family | **b** | **Eyeshine rule** (below) + a `sleep` pose (rule 1: "the animals in their enclosures lie down"). No new models. | anim `sleep` per animal | note for modelling (ART-ANIMALS) | P2 |
| Night animals | 10 night-animal briefs (`art/animals/{hedgehog,…}`) | **c** (already briefed) | Drawn in neutral studio light like day animals; the night look comes from the renderer. Need the eyeshine texture area. Generate hedgehog, bat, owl first (night_1). | `hedgehog`, `bat`, `owl` (+7 later) | existing briefs | **P1** (3) / P3 (7) |
| UI and panels | text panels, HUD, choice icons | **a + c** | Panels, buttons and HUD look **exactly as by day** (NIGHT-005 — readability first). New: two **choice icons** for the night choice without reading (GAME-NIGHT rule 3): 🛏 sleep and 🌙 moon door. | `icon_sleep`, `icon_moon_door` | UI icon sheet (later, with the HUD icons) | P2 |
| Sky | comic sky of the close views (GAME-CAMERA-VIEWS rule 7) | **c** | Night palette of the flat sky bands + haze, a flat outlined **moon** disc and 4-point **stars** (close views only; the zoo view never sees the sky — stars appear there only as reflections on water). | `sky_moon`, `sky_stars` (textures / renderer) | `kit_night` (moon + star shapes) | P2 |
| Style frame | `style_frame` | **c** | Same scene at night — **sets the whole night look**, generate first. | `style_frame_night` | `art/environment/style_frame_night` (exists) | **P1** |

## Eyeshine rule (for every animal model — note for modelling, NIGHT-006)

- Every animal (day and night) has its eye highlight on a **separate small texture area /
  material slot `eye_glow`** (the white highlight dot + a thin ring inside the iris).
- By day the slot renders as the normal white highlight. At night the renderer makes it
  **emissive `#E6F7A0`** when the animal is inside the player's lantern radius (NIGHT-006),
  otherwise it stays unlit.
- Eyes stay **round, friendly and big**; the glow is a soft highlight, never a full glowing
  eyeball, **never red**, no slit pupils.
- Budget: no extra geometry; one extra material slot per animal (or one UV island in the
  shared atlas).

## Emissive rule for props

- Glowing parts (lamp glass, window panes, moon sign) are separate material slots named
  `*_glow`; the renderer switches them on in night mode and adds the point light at the
  prop's `light` empty (position + radius in the model). By day the glass is pale cream.
- One point light per lamp; lights beyond the draw budget fall back to emissive only
  (renderer decision, TECH).

## New briefs (this plan)

| Brief | Contents | Images |
|---|---|---|
| `art/environment/style_frame_night/brief.md` | the night look (exists) | 2 variants |
| `art/props/kit_night/brief.md` | `lantern_post`, `string_lights`, `hand_lantern`, `board_lamp`, `wall_lamp`, `firefly`, `sky_moon` / `sky_stars`, `moon_door` (3 states) | 2 sheets (lights, moon door), each day + night |
| `art/props/kit_bedroom/brief.md` | `bed`, `night_table`, `bedside_lamp`, `window_moon`, `rug_round`, `toy_chest` + bedroom corner at night | 1 sheet + 1 scene |
| `art/environment/env_night_house/brief.md` | night house closed at night + cut-away interior | overview + cut-away |
| `art/environment/env_night_overview/brief.md` | bird's-eye mood of `night_1` | overview |

## Generation order (when the Gemini quota allows)

1. `style_frame_night` (2 variants; pick the friendliest) — everything else uses it as reference.
2. `kit_night` sheets (lights, then moon door).
3. `kit_bedroom`.
4. `hedgehog`, `bat`, `owl` turnaround sheets (split into views).
5. `env_night_overview`, then `env_night_house`.
6. The other seven night animals.

Status 2026-09-27: **generated, in review** — steps 1–6 done (night style frame, kit_night, kit_bedroom, 10 night-animal sheets + `art/animals/night_lineup.png`, env_night_overview, env_night_house); chosen variants and issues are logged in each brief. The night lighting paragraph produced night images without daylight leaks, so a NIGHT STYLE block (*Q-113*) is not forced — still a user decision.
