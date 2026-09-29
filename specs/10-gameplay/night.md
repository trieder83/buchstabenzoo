---
id: GAME-NIGHT
title: Nightfall and the night zoo
aspect: gameplay
module: night
status: draft
depends_on: [GAME-RESCUE, GAME-ANIMALS, GAME-LAYOUT, GAME-SAVE, GAME-PLAYER, CONT-MISSIONS, ART-DIRECTION]
test_prefix: NIGHT
updated: 2026-09-28
---

# Nightfall and the night zoo

## Goal

When **all animals of a day level are home**, the day ends: **night falls**. The child
chooses between **going to sleep** in a bed or **opening a door into a new area, the night
zoo**, where **nocturnal animals** live (up to 10, e.g. hedgehog, bat, owl) — the next
chapter of the game (user decision 2026-09-26; answers Q-031 in part).

## Flow

```
all day animals home ──▶ celebration ──▶ nightfall (dusk → night, ~10 s)
                                             │
                     ┌───────────────────────┴───────────────────────┐
                     ▼                                               ▼
        sleep in the bed (zookeeper house)              moon door opens to the night zoo
        → dream/fade → next morning                     → night missions (nocturnal animals)
        (day zoo, free play, babies GAME-FAMILY)        → when all are home: sleep → morning
```

## Behaviour

1. **Trigger:** the moment the last animal of the current day level enters its enclosure,
   the celebration plays; after it, **dusk** starts: the light turns warm orange, then deep
   blue over ~10 s; lanterns along the paths switch on; stars appear on the water; the
   animals in their enclosures lie down. Nightfall happens once per completed day level (day and night levels alternate, Q-078; saved, GAME-SAVE).
2. **Night is friendly, never scary:** warm lanterns, soft blue shadows, cute glowing eyes,
   gentle night sounds (crickets, an owl's hoo-hoo); no darkness the child cannot see in, no
   jump scares, no threatening shapes. The player always stays clearly visible (a soft light
   circle around her).
3. **Two choices, shown without reading** (icons + audio, GAME-PLAYER UX rules):
   - **🛏 Sleep:** a bed in the **zookeeper house** (enterable building, roof disappears
     inside — GAME-PLAYER §2); after level 2 the bed `bed_l2` at level 2's food storage
     (GAME-LEVEL-2). At nightfall the 🛏 icon offers the **nearest unlocked bed** (Q-141 b). Interacting with the bed → the child's character lies down,
     short dream/fade → **next morning** in the day zoo (free play; care feeding and babies
     continue, GAME-FAMILY). The game is saved.
   - **🌙 Moon door:** a big door with a moon sign (the **night-zoo gate**, a new barrier of
     the day zoo) glows and **opens** at nightfall; walking through it enters the **night
     zoo**, a new level.
   The child can do either at any time during the night; the bed is always available later.
4. **Night zoo (new level `night_1`):** a separate area — a moonlit forest garden with a
   **night house** (dim indoor enclosures with red/blue light, like real zoo nocturnal
   houses), a pond, old trees, a meadow and a small hill. Layout, barriers and hiding places
   are designed by the `zoo-level-designer` (GAME-LAYOUT rules, levels spec
   `specs/10-gameplay/levels/night-1.md`).
5. **Night missions:** the same rescue loop as by day (GAME-RESCUE): info board with a
   location riddle → food box (night food storage) → find the animal → show the food → it
   follows → lead it home. Night-specific additions:
   - **Lantern:** the player carries a lantern; animals are hard to see in the dark but
     their **eyes shine** when the lantern light reaches them (discovery by looking), and
     riddles use night clues (moonlight, sounds, the smell of flowers at night…).
   - **Night boards** have a small built-in lamp so text is always readable; panels look the
     same as by day.
   - Animals are awake and **wander more** at night (GAME-ANIMALS wandering, shorter pauses).
6. **Night animals** (Q-076 answered): up to 10. **Night level 1 (`night_1`): hedgehog, bat,
   owl**; the other seven come in later night levels:

   | # | Animal id | de / en | Food (box word, proposal) | Night habitat clue (proposal) |
   |---|---|---|---|---|
   | 1 | `hedgehog` | Igel / hedgehog | Käfer / beetles | under the leaf pile by the hedge |
   | 2 | `bat` | Fledermaus / bat | Obst / fruit (fruit bat) | hanging under the bridge / in the old tree |
   | 3 | `owl` | Eule / owl | Käfer / beetles (Q-077: no prey animals in food boxes) | on the highest branch, round eyes in the moonlight |
   | 4 | `raccoon` | Waschbär / raccoon | Fisch / fish | washing its paws at the pond |
   | 5 | `badger` | Dachs / badger | Würmer / worms | digging at the hill |
   | 6 | `fennec` | Wüstenfuchs / fennec fox | Insekten / insects | in the sand with its huge ears |
   | 7 | `kiwi` | Kiwi / kiwi | Würmer / worms | poking in the soft earth |
   | 8 | `porcupine` | Stachelschwein / porcupine | Rinde / bark | by the fallen logs |
   | 9 | `slow_loris` | Plumplori / slow loris | Nektar / nectar | slowly climbing in the flowering bush |
   | 10 | `tarsier` | Koboldmaki / tarsier | Grillen / crickets | with giant eyes in the bamboo |

7. **After the night zoo:** when all night animals of `night_1` are home, the child goes to
   sleep (bed) → morning → the **next day-zoo level** (level 2 behind the fallen tree opens);
   day and night levels alternate (Q-078).
   **No pressure (Q-079):** the child can sleep at any time; the moon door stays open every
   night until the night zoo is complete, and progress there is kept (it also opens every
   night after completion: Q-133 (d) answered).
8. **Saving:** day/night state, nightfall done, night-zoo progress and which area the player
   is in are part of the save (GAME-SAVE). Reloading at night restores the night.
9. **Reading and levels:** all night texts exist for every reading level and language
   (CONT-READING, CONT-L10N; German reference). Night riddles follow the same rules as day
   riddles (RESC-011: no place word from `klasse1`).
10. **Rendering (TECH):** night lighting is a renderer mode (global light colour/intensity,
    lantern and lamp point lights with hard cartoon falloff, emissive eyes/windows), not
    separate models; the comic style stays (cel shading, outlines). Performance budget as
    by day.

11. **Night progress (user request 2026-09-28):** the child must always see what is still
    missing before night falls. A small **🌙 progress indicator** (HUD, no reading needed)
    shows one icon per animal species of the **current day level**: filled = home, empty
    (faded) = still missing. It is visible during day play. When all animals of the level are
    home it shows that **night is coming** (all icons filled, the 🌙 glows) through the
    celebration and dusk. At night it shows the animals of the unfinished night level
    (night zoo); when nothing is missing any more it shows 🛏 (sleep) — also by day while the
    night zoo waits (Q-140). While sleeping and in the morning it is hidden, and when every
    unlocked level is complete by day it is hidden. **Tapping it is the same as the 🧭 hint
    button** (GAME-HINT): the hint leads to the next missing animal's mission step. No text
    is needed; its label for screen readers is the Fluent key `ui-night-progress`.

## Implementation data (level design, 2026-09-27 — Q-133…Q-139 answered)

Where the night lives in the level data (owner `zoo-level-designer`; details in GAME-LAYOUT
"Moon door and night levels" / "Night lights, interactables and furniture"):

| Thing | Data | Where |
|---|---|---|
| Bed (rule 3) | `[[item]] bed_l1`, `kind = "bed"`, pos (−12.0, 1.5), stand (−12, 2) | level 1, inside the new enterable `zookeeper_house_1` (x −14…−9, z 0…4, door (−9, 2)) west of the entrance plaza; bedroom props (`night_table`, `window_moon`, `rug_round`, `toy_chest`) as `[[prop]]`, bedside lamp `[[light]] kind = "indoor"` |
| Bed after level 2 (rule 3) | `[[item]] bed_l2`, `kind = "bed"`, pos (42.0, 26.5), stand (42, 25) | level 2, outdoors against the south wall of `food_storage_2` (GAME-LEVEL-2 "Bed of level 2", Q-141; shelter Q-152) |
| Moon door (rule 3) | barrier `moon_door`, `kind = "moon_door"`, `transition = "level_1->night_1"`, `unlock_after = "level_1"`, `opens_at = "night"` | level 1, west zoo wall x −24…−23, z 29…30, reached by `path_moon`; open every night after level 1's nightfall, closed by day, never removed (Q-133) |
| Night zoo (rule 4) | `assets/levels/night-1.toml`, `[level] time = "night"`, `[[entry]] entry_n1_moon` (−25, 29…30) | west of level 1, bounds (−72, 6, 48, 48) — GAME-LEVEL-NIGHT-1 |
| Night house (rule 4) | `night_house` (enterable hall, `model_rect`) + `enc_n1_hedgehog` / `enc_n1_bat` / `enc_n1_owl` (`indoor = true`), boards outside; indoor lights `#E8735A` / `#5B7FE0` (Q-116) | `night_1`, north-east (Q-134) |
| Night foods (rule 6) | food ids `beetles` (hedgehog, owl), `fruit` (bat), distractors `worms`, `nectar`; 4 `[[food_box]]` at `food_storage_n1` | `night_1` plaza (Q-135) |
| Night hiding places (rule 5, NIGHT-007) | 9 `[[hiding_place]]`, 3 per animal, perch heights for bat and owl | `night_1` (GAME-LEVEL-NIGHT-1 "Hiding places") |
| Lanterns (rule 1) | `[[light]]` in all four level files: lantern posts (main paths ≈ 10 m + every gate), string lights (level-1 entrance plaza, `night_1` plaza), wall lamps, board lamps (rule 5), indoor lights | Q-118 answered; data shape Q-137 |
| Fireflies | only `firefly_meadow_n1` (`loc_fireflies`, Q-115) | `night_1` |
| Texts | `assets/i18n/{de,en}/night.ftl`: night riddles, facts, names, home texts, `night-dusk` / `night-bed` / `night-moon-door` / `night-welcome` / `night-complete` / `night-morning` per reading level, `sign-night-house`, `food-beetles` … | CONT-MISSIONS "Night level 1" |
| Next day level (rule 7) | `barrier_ne_tree`: `unlock_after = "night_1"`, `opens_at = "morning"` | level 1 (Q-078 answered) |

Rules for the implementation that follow from the data:
- The moon door is **not** an exit barrier of the Q-091 rule (barriers open the next morning;
  it must not be counted by `exit_barriers()`); it opens at nightfall and closes in the morning.
- After `night_1` is complete the moon door still opens every night (Q-133 answered), so the
  child can visit the night animals.
- Burglars (GAME-EVENTS) use `[[event_spot]]`s of the **day** levels only (Q-139).

## Implementation (engine, 2026-09-27)

- **Time of day** (`zoo_core::daytime`): `day` → `dusk` → `night` → `sleeping` → `morning` →
  `day`. The host's mission celebration lasts 7 s (`CELEBRATION_S`); dusk starts after it and
  lasts 10.5 s (`DUSK_S`: warm orange first, then deep blue); sleeping 2.5 s (`SLEEP_S`, dream
  fade 💤), morning 3 s (`MORNING_S`, the light comes back). Nightfall is scheduled once per
  completed **day** level (`nightfalls`, saved). Night animals of a night level wander twice
  as much (pauses count down twice as fast); day animals at home lie down (`sleep` clip,
  fallback `idle`) and do not wander at night.
- **Barriers open the next morning** (replaces the temporary rule of Q-091): a completed
  level is queued (`pending_exits`, saved); in the morning every barrier it unlocks opens —
  `unlock_after = "<level>"` when that level is joined (the fallen tree after `night_1`,
  Q-078), else the transition's source level — plus the next level's entry barriers.
- **Moon door**: opens at every nightfall once its `unlock_after` level had its nightfall
  (default: the level it belongs to), closes in the morning, never counts as an exit
  (`Level::exit_barriers_of` / `barriers_unlocked_by` skip `kind = "moon_door"`). Walking
  through the open door enters `night_1` (joined like the day levels); near the door the
  interact button shows 🌙 and takes the child through it. Night-animal eyes shine within
  `LANTERN_RADIUS_M` = 2.5 m (horizontal) of the player (NIGHT-006; the visible lantern
  ground pool has the same 2.5 m radius, Q-142, NIGHT-018). Leaving the night zoo is walking
  back. In the morning a night animal that was following goes back to its hiding place.
- **Bed** (`[[item]] kind = "bed"`, else the bed placeholder of an enterable zookeeper house):
  interactable at night (🛏) → sleep → morning (autosave on `SleepStarted` and `Morning`).
  Sleeping uses the normal **interact action** (user decision 2026-09-27): standing next to
  the bed, desktop `E` (or Space/Enter) and the touch action button (🛏 icon) start sleeping —
  no extra menu or text needed.
  *Q-140 answered (user 2026-09-27):* while a visited night level is unfinished, the bed also works **by day**
  and the child sleeps until the evening (dusk → night) — otherwise a child who slept before
  the night zoo was done would never get another night (level 2 waits for `night_1`, Q-078).
- **Save** (GAME-SAVE): `daytime` (phase, time in phase, pending dusk, nightfalls, pending
  exits, nights). A save made while sleeping restores as the finished morning.
- **Renderer** (`zoo_render::night`): the night grading is a colour ramp (mid colours → deep
  blue `(0.20, 0.27, 0.66)`, light colours → pale lavender, half the chroma kept, one darker
  shadow tone ×0.74, never below `#2B3566`); dusk multiplies warm orange / violet. Point
  lights with a hard edge: the player's lantern (centre 1.4 m above her feet, radius ≈ 2.87 m —
  she is always lit, ground pool 2.5 m, Q-142) + the 8 lamps nearest to the camera target within
  22 m; the next 24 lamps are **light-pool decals** (flat pools evaluated on the ground in the
  same shader, no extra draw call — Q-114); farther lamps stay emissive only. On weak phones
  the automatic low quality tier (PERF-BUDGETS rule 5, Q-170) lights the lantern + the 4
  nearest lamps as point lights and the next 24 as light pools. The hard light edge is
  anti-aliased over the pixel footprint from position derivatives taken once per fragment
  before the light loops (PERF-R-014, Q-180). Lamp light is
  pulled 35 % towards a flat warm cream (style frame); coloured indoor lights (night house
  blue `#5B7FE0` / red-orange `#E8735A`) keep their colour. Emissive surfaces (instance colour
  alpha 2): lamp glass `#FFD66B`, lit windows `#FFC857` (placeholder panes on procedural building
  facades; building models bring their own `*_glow` windows), the moon sign `#FFF4C9` + rim `#8FB8FF`, eyeshine `#E6F7A0`, fireflies `#EFFF8A`.
  Signs (decals) are lit by their lamp. Water: stars reflected as twinkling 4-point
  sparkles, lamp pools on the water. Close views: dark-blue sky gradient + haze
  (`#1E2A5A` → `#3B4C8C`), outlined comic moon, a few stars, blue clouds (Q-126).
- **Models (since 2026-09-27, `kit_night` / `kit_bedroom` / `kit_landmarks` / the night
  animals; TECH-ARCH "Multi-node assets"):** `lantern_post`, `string_lights` (spans ≤ 6 m
  from post to post, the cord stretched to the span — proposal Q-147 — and a `string_post`
  at the far end), `wall_lamp` (on the facade, 1.6 m), `board_lamp` (on the board's socket),
  the `hand_lantern` in her left hand (its grip on the hand), the `moon_door` (its leaves
  swing open to the back at nightfall), bed, desk with the note, night table with the
  bedside lamp, moon window (its night sky only at night), rug, toy chest, key box with the
  cart key, and the night-1 landmarks (windmill with turning sails, hollow / old / crooked /
  fir tree, rock hill, potting bench, telescope, mushroom patch). Their `*_glow` slots
  light up with the lamps (dusk on); lamp lights sit at the models' `light` empties. The
  building models bring their own glowing windows and ceiling lamps (no placeholder panes).
  Night animals are drawn with their models: `sleep` while asleep, the bat `hang`s and the
  owl `perch`es at their perch, both `fly` `fly_height` (1.5 m) above the ground while they
  follow (NIGHT-017); `eye_glow` shines inside the lantern radius and never while `sleep`
  plays (Q-146 answered). The player's lantern light is a sphere of radius √(2.5² + 1.4²)
  ≈ 2.87 m centred 1.4 m above her feet, so its hard-edged ground pool is 2.5 m = the
  eyeshine radius (Q-142 answered, NIGHT-018). Placeholders (boxes) remain only as
  fallbacks for missing models.
- **Night progress** (rule 11, `zoo_core::hints::night_progress`): state `missing` (the
  first unlocked day level with a missing animal), `night_coming` (dusk pending or dusk:
  the level of the last nightfall), `night` (the first unlocked, unfinished night level),
  `sleep` (night with nothing missing, or by day while `night_zoo_waiting`), `hidden`
  (sleeping, morning, or nothing left by day); one entry per species (a pair counts as home
  when both are). Host: `#night-progress` (right, below the 🧭 button; a row left of the gear
  on low landscape screens), icons ≥ 40 px, missing ones faded and grey, tap = hint press
  (`App::night_progress_json`).
- **Host:** 🛏 / 🌙 choice icons at night, the dusk / morning / welcome / night-complete
  cut-in texts per reading level (`night-*` keys), the dream fade while sleeping.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| NIGHT-001 | Given the last day animal enters its enclosure, then after the celebration dusk starts once and reaches night within 10–12 s; it does not start earlier. | unit |
| NIGHT-002 | Given night has fallen, then the moon door is open and the unlocked bed (`bed_l1` in the zookeeper house; after level 2 also `bed_l2`, Q-141) is interactable; before nightfall the moon door is closed. | unit |
| NIGHT-003 | Given the player interacts with the bed, then the character sleeps, the game is saved and the next morning starts in the day zoo with all day animals home. | e2e |
| NIGHT-004 | Given the player walks through the open moon door, then the night zoo level `night_1` loads with its own missions, and walking back through the door returns to the day zoo at night. | e2e |
| NIGHT-005 | Given the night zoo, then the player, every info board text and every interactable are visible/readable (screenshot brightness of the player and of panels above a minimum contrast). | e2e |
| NIGHT-006 | Given a night animal within the lantern radius, then its eyes shine; outside the radius they do not. | unit |
| NIGHT-007 | Given every night animal, then it has ≥ 3 hiding places in its night level, riddles per reading level and language, and a food box (as RESC-014/RESC-017 by day). | unit |
| NIGHT-008 | Given a save made at night (day zoo or night zoo), when restored, then it is still night and the player is in the same area and position. | unit |
| NIGHT-009 | Given the night scenes, then reviewers confirm nothing is scary for 4–6 year olds (manual review with the art direction checklist). | manual |
| NIGHT-010 | Given a completed level, then its barriers (by `unlock_after`, else the transition) open only the next morning, never at the celebration; the moon door is never opened as an exit; with `night_1` joined the fallen tree opens the morning after `night_1` is complete, without it after level 1. | unit |
| NIGHT-011 | Given the child slept while `night_1` is unfinished (Q-140 answered), then by day the bed is interactable and sleeping leads to dusk and night with the moon door open; night-zoo progress is kept and a following night animal is back at its hiding place. | unit |
| NIGHT-012 | Given the night grading, then no colour is darker than `#2B3566`, grass turns deep blue and paths pale lavender, dusk is warmer than day, lamp light has a hard edge, and the nearest lamps get point lights, the next ones light pools, far ones none (Q-114). | unit |
| NIGHT-013 | Given night, then ducks sleep (no swimming, `sleep` pose), butterflies are hidden and frogs croak more often than by day. | unit |
| NIGHT-014 | Given the same spot by day and at night, then the night adds at most 6 draw calls (night props batched); frame time is reported. | e2e |
| NIGHT-015 | Given night, then day animals at home lie down (`sleep` clip, fallback `idle`) and do not wander, and the wander pauses of the night animals of a night level count down twice as fast as by day (rule 5, engine section). | unit |
| NIGHT-016 | Given night and the player standing next to the bed, when `E` is pressed (desktop) or the touch action button (🛏) is tapped, then sleeping starts; away from the bed neither does anything bed-related. | e2e |
| NIGHT-017 | Given the bat or the owl (`fly_height` in `animal_anims.toml`), then perched at its hiding place it plays `hang` (bat, data pose) / `perch` at the perch height, following the child it plays `fly` 1.5 m above the ground, at home it stands (`idle`); its `eye_glow` shines only inside the lantern radius and never while `sleep` plays (Q-146). | unit |
| NIGHT-018 | Given the player's lantern at night, then its visible ground pool (sphere radius, centre 1.4 m above her feet) is 2.5 m ± 0.01 m = the eyeshine radius `LANTERN_RADIUS_M` (Q-142). | unit |
| NIGHT-019 | Given a new game, then the 🌙 progress shows the three level-1 animals empty; each animal home fills its icon; all home → `night_coming` through the celebration and dusk; at night the `night_1` animals; all of them home → `sleep`; sleeping/morning hidden; the next day the level-2 animals; by day with the night zoo waiting → `sleep` and the hint points at the bed (rule 11). | unit |
| NIGHT-020 | Given day play on a phone in portrait, then the 🌙 progress is visible (not covering the 🧭 button, the gear or the carried-food HUD), shows one icon per level-1 animal, a mission completed fills one icon, and tapping it shows a hint (HINT-013). | e2e |
| NIGHT-021 | Given all level-1 missions played to the end (scripted child following the hints, real movement), then the 🌙 shows night coming, dusk turns into night, the moon door opens, the bed works with `E` and the next morning comes (the whole night cycle is reachable end to end). | e2e |

## Open questions

- Q-076…Q-079 answered 2026-09-26 (animals of night level 1, owl food, what comes after, no pressure).
- Q-133…Q-139 answered 2026-09-27 (as recommended): night level data, night house, night foods, riddle scope of night levels, light/item/prop data, telescope, burglar spots (level design, 2026-09-27).
- Q-126 answered 2026-09-27: dark-blue gradient with moon and stars, haze in the same blue, day fog end kept (20.8 m since FIX-056).
- Q-091 answered 2026-09-27: barriers open the next morning (engine section, NIGHT-010).
- Q-140 answered 2026-09-27: sleeping by day "until the evening" while a visited night level is unfinished (NIGHT-011).
- Q-141 answered 2026-09-27: until `night_2` exists, level 3 opens the morning after level 2's night; bed `bed_l2` at level 2's food storage, the 🛏 icon offers the nearest unlocked bed.
- Q-142 answered 2026-09-27: eyeshine stays 2.5 m, the visible lantern ground pool is 2.5 m too (NIGHT-018).
- Q-146 answered 2026-09-27: `eye_glow` is skipped while `sleep` plays; no eyelids for v1 (NIGHT-017).
- Q-188 (open, proposal): the 🌙 progress also shows the night-zoo animals at night and 🛏 when nothing is missing (rule 11).
- Q-147 (open) String-light spans ≤ 6 m by stretching the model.
- Q-143 answered 2026-09-27: night animals, `sleep` and `eye_glow` are in ART-ANIMALS (models v1).
- Q-152 shelter over `bed_l2`. Q-153 / Q-154 night models: manifest entries and ART-ENVIRONMENT listing.
