---
id: GAME-NIGHT
title: Nightfall and the night zoo
aspect: gameplay
module: night
status: draft
depends_on: [GAME-RESCUE, GAME-ANIMALS, GAME-LAYOUT, GAME-SAVE, GAME-PLAYER, CONT-MISSIONS, ART-DIRECTION]
test_prefix: NIGHT
updated: 2026-09-27
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
     inside — GAME-PLAYER §2). Interacting with the bed → the child's character lies down,
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
   night until the night zoo is complete, and progress there is kept (whether it also opens
   after completion: proposal Q-133 (d)).
8. **Saving:** day/night state, nightfall done, night-zoo progress and which area the player
   is in are part of the save (GAME-SAVE). Reloading at night restores the night.
9. **Reading and levels:** all night texts exist for every reading level and language
   (CONT-READING, CONT-L10N; German reference). Night riddles follow the same rules as day
   riddles (RESC-011: no place word from `klasse1`).
10. **Rendering (TECH):** night lighting is a renderer mode (global light colour/intensity,
    lantern and lamp point lights with hard cartoon falloff, emissive eyes/windows), not
    separate models; the comic style stays (cel shading, outlines). Performance budget as
    by day.

## Implementation data (level design, 2026-09-27 — proposals Q-133…Q-139)

Where the night lives in the level data (owner `zoo-level-designer`; details in GAME-LAYOUT
"Moon door and night levels" / "Night lights, interactables and furniture"):

| Thing | Data | Where |
|---|---|---|
| Bed (rule 3) | `[[item]] bed_l1`, `kind = "bed"`, pos (−12.0, 1.5), stand (−12, 2) | level 1, inside the new enterable `zookeeper_house_1` (x −14…−9, z 0…4, door (−9, 2)) west of the entrance plaza; bedroom props (`night_table`, `window_moon`, `rug_round`, `toy_chest`) as `[[prop]]`, bedside lamp `[[light]] kind = "indoor"` |
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
- After `night_1` is complete the moon door still opens every night (proposal Q-133), so the
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
  `LANTERN_RADIUS_M` = 2.5 m (horizontal) of the player (NIGHT-006; the visible lantern light
  below is smaller, see Q-142). Leaving the night zoo is walking
  back. In the morning a night animal that was following goes back to its hiding place.
- **Bed** (`[[item]] kind = "bed"`, else the bed placeholder of an enterable zookeeper house):
  interactable at night (🛏) → sleep → morning (autosave on `SleepStarted` and `Morning`).
  *Proposal Q-140:* while a visited night level is unfinished, the bed also works **by day**
  and the child sleeps until the evening (dusk → night) — otherwise a child who slept before
  the night zoo was done would never get another night (level 2 waits for `night_1`, Q-078).
- **Save** (GAME-SAVE): `daytime` (phase, time in phase, pending dusk, nightfalls, pending
  exits, nights). A save made while sleeping restores as the finished morning.
- **Renderer** (`zoo_render::night`): the night grading is a colour ramp (mid colours → deep
  blue `(0.20, 0.27, 0.66)`, light colours → pale lavender, half the chroma kept, one darker
  shadow tone ×0.74, never below `#2B3566`); dusk multiplies warm orange / violet. Point
  lights with a hard edge: the player's lantern (centre 1.4 m above her feet, radius 2.3 m —
  she is always lit, ground pool ≈ 1.8 m) + the 8 lamps nearest to the camera target within
  22 m; the next 24 lamps are **light-pool decals** (flat pools evaluated on the ground in the
  same shader, no extra draw call — Q-114); farther lamps stay emissive only. Lamp light is
  pulled 35 % towards a flat warm cream (style frame); coloured indoor lights (night house
  blue `#5B7FE0` / red-orange `#E8735A`) keep their colour. Emissive surfaces (instance colour
  alpha 2): lamp glass `#FFD66B`, lit windows `#FFC857` (placeholder panes on every building
  facade), the moon sign `#FFF4C9` + rim `#8FB8FF`, eyeshine `#E6F7A0`, fireflies `#EFFF8A`.
  Signs (decals) are lit by their lamp. Water: stars reflected as twinkling 4-point
  sparkles, lamp pools on the water. Close views: dark-blue sky gradient + haze
  (`#1E2A5A` → `#3B4C8C`), outlined comic moon, a few stars, blue clouds (Q-126).
- **Placeholders until the models exist:** lantern posts / string lights / wall lamps / board
  lamps (post + glowing box), the moon door (stone pillars with lanterns, beam, moon sign;
  blue plank leaves hidden while open), bed and bedroom furniture (`[[prop]]`), the hand
  lantern (a glowing box in the left hand), night animals (hedgehog, bat, owl as coloured box
  shapes with big eyes that shine inside the lantern radius). Day-animal models have no
  `eye_glow` slot yet: their eyes do not shine (they sleep at night anyway).
- **Host:** 🛏 / 🌙 choice icons at night, the dusk / morning / welcome / night-complete
  cut-in texts per reading level (`night-*` keys), the dream fade while sleeping.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| NIGHT-001 | Given the last day animal enters its enclosure, then after the celebration dusk starts once and reaches night within 10–12 s; it does not start earlier. | unit |
| NIGHT-002 | Given night has fallen, then the moon door is open and the bed in the zookeeper house is interactable; before nightfall the moon door is closed. | unit |
| NIGHT-003 | Given the player interacts with the bed, then the character sleeps, the game is saved and the next morning starts in the day zoo with all day animals home. | e2e |
| NIGHT-004 | Given the player walks through the open moon door, then the night zoo level `night_1` loads with its own missions, and walking back through the door returns to the day zoo at night. | e2e |
| NIGHT-005 | Given the night zoo, then the player, every info board text and every interactable are visible/readable (screenshot brightness of the player and of panels above a minimum contrast). | e2e |
| NIGHT-006 | Given a night animal within the lantern radius, then its eyes shine; outside the radius they do not. | unit |
| NIGHT-007 | Given every night animal, then it has ≥ 3 hiding places in its night level, riddles per reading level and language, and a food box (as RESC-014/RESC-017 by day). | unit |
| NIGHT-008 | Given a save made at night (day zoo or night zoo), when restored, then it is still night and the player is in the same area and position. | unit |
| NIGHT-009 | Given the night scenes, then reviewers confirm nothing is scary for 4–6 year olds (manual review with the art direction checklist). | manual |
| NIGHT-010 | Given a completed level, then its barriers (by `unlock_after`, else the transition) open only the next morning, never at the celebration; the moon door is never opened as an exit; with `night_1` joined the fallen tree opens the morning after `night_1` is complete, without it after level 1. | unit |
| NIGHT-011 | Given the child slept while `night_1` is unfinished (proposal Q-140), then by day the bed is interactable and sleeping leads to dusk and night with the moon door open; night-zoo progress is kept and a following night animal is back at its hiding place. | unit |
| NIGHT-012 | Given the night grading, then no colour is darker than `#2B3566`, grass turns deep blue and paths pale lavender, dusk is warmer than day, lamp light has a hard edge, and the nearest lamps get point lights, the next ones light pools, far ones none (Q-114). | unit |
| NIGHT-013 | Given night, then ducks sleep (no swimming, `sleep` pose), butterflies are hidden and frogs croak more often than by day. | unit |
| NIGHT-014 | Given the same spot by day and at night, then the night adds at most 6 draw calls (night props batched); frame time is reported. | e2e |
| NIGHT-015 | Given night, then day animals at home lie down (`sleep` clip, fallback `idle`) and do not wander, and the wander pauses of the night animals of a night level count down twice as fast as by day (rule 5, engine section). | unit |

## Open questions

- Q-076…Q-079 answered 2026-09-26 (animals of night level 1, owl food, what comes after, no pressure).
- Q-133…Q-139 night level data, night house, night foods, riddle scope of night levels, light/item/prop data, telescope, burglar spots (level design, 2026-09-27).
- Q-126 answered 2026-09-27: dark-blue gradient with moon and stars, haze in the same blue, 16 m fog end kept.
- Q-091 answered 2026-09-27: barriers open the next morning (engine section, NIGHT-010).
- Q-140 (open) Sleeping by day "until the evening" while a night level is unfinished (implemented as proposal).
- Q-141 (open) Night level between level 2 and level 3 (alternation, Q-078) and where the child sleeps after level 2.
- Q-142 (open) Eyeshine radius 2.5 m vs. the visible lantern pool ≈ 1.8 m.
- Q-143 (open) Night animals, `sleep` clip and `eye_glow` slot missing from ART-ANIMALS.
