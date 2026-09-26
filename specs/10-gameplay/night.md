---
id: GAME-NIGHT
title: Nightfall and the night zoo
aspect: gameplay
module: night
status: draft
depends_on: [GAME-RESCUE, GAME-ANIMALS, GAME-LAYOUT, GAME-SAVE, GAME-PLAYER, CONT-MISSIONS, ART-DIRECTION]
test_prefix: NIGHT
updated: 2026-09-26
---

# Nightfall and the night zoo

## Goal

When **all animals of the day zoo are home**, the day ends: **night falls**. The child
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

1. **Trigger:** the moment the last day animal of the current zoo enters its enclosure,
   the celebration plays; after it, **dusk** starts: the light turns warm orange, then deep
   blue over ~10 s; lanterns along the paths switch on; stars appear on the water; the
   animals in their enclosures lie down. Nightfall happens once per zoo (saved, GAME-SAVE).
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
   night until the night zoo is complete, and progress there is kept.
8. **Saving:** day/night state, nightfall done, night-zoo progress and which area the player
   is in are part of the save (GAME-SAVE). Reloading at night restores the night.
9. **Reading and levels:** all night texts exist for every reading level and language
   (CONT-READING, CONT-L10N; German reference). Night riddles follow the same rules as day
   riddles (RESC-011: no place word from `klasse1`).
10. **Rendering (TECH):** night lighting is a renderer mode (global light colour/intensity,
    lantern and lamp point lights with hard cartoon falloff, emissive eyes/windows), not
    separate models; the comic style stays (cel shading, outlines). Performance budget as
    by day.

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

## Open questions

- Q-076…Q-079 answered 2026-09-26 (animals of night level 1, owl food, what comes after, no pressure).
