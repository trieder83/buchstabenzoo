---
id: GAME-FAMILY
title: Animal pairs and babies
aspect: gameplay
module: families
status: draft
depends_on: [GAME-ANIMALS, GAME-RESCUE, GAME-FEED, GAME-SAVE]
test_prefix: FAM
updated: 2026-10-03
---

# Animal pairs and babies

## Goal

**Every species lives in the zoo as a pair — a male and a female** (user decisions 2026-09-26:
zebras and koalas; 2026-10-01, Q-308 / Q-073 answered: all animals, including the goldfish and
the night animals). When the pair is home and gets the right food, they **may
get a baby later** — a long-term reward that makes caring for the animals worthwhile.

## Pairs

| Species | Level | Pair | Baby name (de / en) | Female / baby model |
|---|---|---|---|---|
| `zebra` | 1 | male + female | Fohlen / foal | own models |
| `hippo` | 1 | male + female | Kalb / calf | fallback (Q-282) |
| `panda` | 1 | male + female | Pandababy / cub | fallback |
| `koala` | 2 | male + female | Koalababy (Joey) / joey | own models |
| `elephant` | 2 | male + female | Elefantenkalb / calf | fallback |
| `giraffe` | 2 | male + female | Giraffenkalb / calf | fallback |
| `lion` | 2 | male + female | Löwenbaby / cub | fallback |
| `monkey` | 3 | male + female | Affenbaby / baby | fallback |
| `snow_fox` | 3 | male + female | Fuchswelpe / kit | fallback |
| `goldfish` | 3 | two fish (male + female) in **one** bowl | Fischbrut / fry | fallback |
| `hedgehog` | night 1 | male + female | Igelbaby / hoglet | fallback |
| `bat` | night 1 | male + female | Fledermausbaby / pup | fallback |
| `owl` | night 1 | male + female | Eulenküken / owlet | fallback |
| `snake` | night 2 | male + female | Schlangenbaby / baby snake (`snake_hatchling`) | own models (concept sheet in review) |
| `chameleon` | night 2 | male + female | Chamäleonbaby / baby chameleon (`chameleon_baby`) | own models (concept sheet in review) |
| `poison_dart_frog` | night 2 | male + female | Fröschlein / froglet (`frog_froglet`; the tadpole is only a board fact) | own models (concept sheet in review) |

No species stays single (Q-308). The goldfish pair swims in one pond, is caught with **one** food
and carried in **one** bowl (rule 9); the night animals (hedgehog, bat, owl, snake, chameleon, poison dart frog) are two in each indoor enclosure (rule 10).

## Behaviour

1. **Escape as a pair:** both animals of a pair wait at the **same hiding place** and wander
   near each other (each within the hiding area, ≤ 3 m, GAME-ANIMALS).
2. **Rescue as a pair:** showing the correct food to either of them makes **both** follow
   (one group, GAME-RESCUE §5); the mission completes when both are in the enclosure.
3. **Look:** male and female are clearly the same species and differ subtly but visibly for
   children: the male is ~10 % larger; the female has a small distinguishing detail
   (proposal: slightly different mane/ear tuft pattern; no clothing or bows — Q-074). Both
   are friendly; nothing stereotyped.
4. *(Retired 2026-10-01, user: no feeding trough, Q-248 / Q-198: the old care-feeding rule is replaced by special food, rule 5.)*
5. **Baby:** when the pair at home is given a liked special food ("Special food and babies", Q-198 answered; replaces the former 3-care-feedings rule), the
   next time the player comes to the enclosure a **baby** is there (small celebration, the
   baby's name via Fluent). One baby per pair in the PoC scope. The baby is a **real member of
   the group at home** (GARD-013): it keeps to a cell next to the female inside the fence (never
   at the fence), follows her, is called to the feeding spot with the pair (third place) and
   reacts to treats. Its position is not saved; on load it is placed next to the female. **The baby
   is always with the female (FAM-011..013, user report 2026-10-01):** beside her at the hiding
   place while she is out (also when a save with the baby flag is restored while the pair is
   out again), walking after her along the nav path (a little faster than she) while she
   follows the child, inside the fence next to her once she is home (a baby that is outside
   her area while she is home, or > 30 m from her, is put beside her at once — never left
   outside), and at rank 2 of the feeding spot. The nearest member for a treat includes the
   baby. A baby is never a mission requirement, so it can never be a dead end.
   **Playful following (user request 2026-10-01, FAM-014..018; module `zoo-core/src/baby.rs`):** the baby is
   not glued to the female and does not walk a straight line behind her; it keeps a **loose leash** and plays.
   Behaviour model (a small state machine per baby, deterministic: own seeded `Pcg32` in the game state,
   no per-frame allocation, only a route is rebuilt when a new target is picked):
   - `Roam`: walks to a target point around her — while she stands on a ring (radius `RING_M` 1.8..4.0 m, the
     angle advances by `ORBIT_STEP_DEG` 40..140° each leg = it circles her); while she walks ahead of / behind /
     beside her (`LATERAL_M` ±1..2.5 m, `AHEAD_M` -3.5..+3.5 m along her heading = trots ahead, drops back, zig-zags).
   - `Burst`: a short run (`BURST_LEG_S`) to such a target at `BURST_FACTOR` 2.2 × her follow speed, with a small
     vertical **hop** (`HOP_HEIGHT_M` 0.12 m, `HOP_PERIOD_S` 0.45 s, sine) and a faster clip rate.
   - `Sniff`: stands still `SNIFF_S` 1.0..3.5 s and plays the `eat` action (sniffing/grazing), only when it is
     within `SNIFF_MAX_M` 3.0 m of her; `Idle`: looks at her for `IDLE_S` 0.8..2.0 s (sometimes hopping on the spot).
   - `Return`: farther than `LEASH_SOFT_M` 5.0 m it trots straight back (catch-up `CATCHUP_FACTOR` 1.6 ×); farther
     than `LEASH_HARD_M` 8.0 m it runs at `BURST_FACTOR` along the shortest route to her until within
     `RETURN_DONE_M` 4.0 m. The existing safety net stays: > `TELEPORT_M` 30 m → placed beside her.
   - Speeds are multiples of the follow speed on the surface under the baby: amble `AMBLE_FACTOR` 0.595 (she
     stands), trot `TROT_FACTOR` 1.0625 (she walks), catch-up 1.36, burst 1.87 (never above `BURST_FACTOR`); these are the first design's 0.7 / 1.25 / 1.6 / 2.2 times `BABY_SLOWDOWN` 0.85 (babies run 15 % slower, user request 2026-10-04, FAM-034).
   - Leg length `LEG_S`: 3..5 s while she stands, 1.5..3 s while she walks. Targets are only valid cells: at home
     inside her wander area, otherwise passable cells (same cell/fence rules as the route finder), so the baby never
     leaves the enclosure while she is home and never clips fences/props. When called to the feeding spot it goes to rank 2
     as before (no playing until the call ends). Mean distance to her stays ≈ 1.8..4 m, never > 8 m.
   - Presentation: `Baby.play` (`BabyPlay`) carries `mode`, `hop` (m, added to the draw height), `clip_t` (own clip clock),
     `rate` (clip rate) and `sniff_t`; zoo-web draws `walk` while it moves, `eat` while sniffing. A pair
   enters its enclosure **together**: a partner still waiting far behind (RESC-006) is called
   to catch up first, then both enter (GARD-014).
6. Wrong or ordinary food given to the pair at home: no baby; gentle feedback (as RESC-005), no
   penalty.
7. **Saving:** pairs, the baby flag, are part of the save
   (GAME-SAVE).
8. **Art:** each pair needs a male and a female variant and a baby model (turnaround
   sheets first — ART-PIPELINE). The baby uses the same rig as its parents, scaled (~45 %).

## Pairs for every species (2026-10-01, Q-280)

9. **Goldfish:** two fish at the same water place. The fish food shown from the bank with the
   filled bowl in the hands puts **both** fish into the bowl (RESC-018..022 for the group): one
   bowl carries both (they swim side by side in it), saving keeps both `in_bowl`, putting the
   bowl at the pond's step brings both home (two cells apart) and completes the mission. An old
   save that knew only one fish reopens the mission (never stuck, RESC-032).
10. **Night animals** (hedgehog, bat, owl): two per indoor enclosure, both at the same night
    hiding place; the bats hang / the owls sit side by side on the perch (rule 12); rescue, home
    texts and eye shine work per animal (GAME-NIGHT). They have no garden treat, so no baby
    yet (rule 13).
11. **Room:** a pair needs room — every hiding place of a pair species has a wander area of
    ≥ 9 cells with two cells at least the species' **pair gap** apart and ≤ 3 m from the spot; every
    enclosure has ≥ 12 home cells (male, female and baby) and, where the species can be fed, a
    feeding spot whose rank 0/1/2 cells are distinct home cells.
12. **No clipping:** the two animals of a pair keep a minimum distance (pair gap, centre to
    centre): hippo, giraffe, elephant 2 m; zebra, panda, lion 1.5 m; all others 1 m
    (`zoo_core::animals::pair_gap_m`).
    - at the hiding place the second animal starts on the cell nearest the first that is ≥ the gap
      away (≤ 3 m); while wandering, targets nearer than the gap to the partner (or to its goal)
      are not drawn and a step that would undercut the gap is cancelled (the animal stops and
      draws a new target); an escaped animal that comes to the player may close up to 60 % of the
      gap (so at least one of the two always reaches the shore, Q-097);
    - entering the enclosure: the second one steps onto the nearest home cell ≥ the gap from the
      first;
    - feeding spot (GAME-GARDEN §6): the male and female cells are `ceil(gap)` cells apart along
      the fence where the fence side is free (the hippo fence side is crowded by hedge and info
      board: 1 m, Q-282); the child stands in front of the pair, both are within reach (2 m);
      the animals walk to the middle of their spot cell;
    - perched pairs (koala, bat, owl): member 1 sits `PERCH_PAIR_OFFSET_M` = 0.7 m beside member
      0 across the branch; a pair species' branch / platform is 1.5 m wide.
13. **Babies:** the baby rules (Special food and babies) apply to every pair. Every species has a treat
    (GAME-FEED "Basic food and treats"), so every pair can get a baby; babies are never required.

## Look without own models (fallback, Q-280)

Until a female / baby model exists (ART-ANIMALS "Family models", only zebra and koala so far)
the game draws the **adult model** for member 1 at **0.92** scale with a slight rosy tint
(`[1.0, 0.72, 0.78, 0.16]`, `CharacterDraw::tint`; the male is the unscaled adult = ~10 % larger)
and a baby at **0.45** scale, untinted (`animals::member_look` / `baby_look`). The two-fish
fallback is the same. Dedicated models replace it automatically when the file exists. The
missing models are art tasks (manifest `<species>_female`, `<species>_calf|cub|…`,
`concept_approved = false`; concept sheets first, Q-282) — **not modelled now**.

## Texts (Q-280)

- **Info board:** every pair species' board shows the generic **pair note** after the facts:
  `mission-pair-note-<reading level>` — `kiga`: "♂ ♀" (symbols only); `klasse1`: "Es sind zwei. Ein
  Männchen. Ein Weibchen."; `klasse2/3`: "Es sind (immer) zwei Tiere: ein Männchen und ein Weibchen. …"
  (en equivalents). Riddles stay as they are: they describe the species (Q-283: plural wording
  of the klasse2/3 riddles of the other species).
- **Mission complete:** `mission-<animal>-home` is plural for all species ("Die Flusspferde sind
  wieder zu Hause." / "The hippos are home again."; also the night animals and the goldfish).

## Special food and babies (user request 2026-09-29; two-role model 2026-10-03)

**Special food** (a liked treat such as a carrot — GAME-GARDEN "Treats" — or another food the
species loves, data per species, Q-100) makes an animal **happy**: it comes over, eats it,
hearts appear (`eat` + happy hearts, GARD-005). If the enclosure holds a **male and a female**
of the species (a pair, rule above) and either is given special food, both are happy and the
pair **makes a baby**: a short celebration (hearts between the two), and a **baby** appears
in the enclosure the next time the child is within view (rule 5 name/model). One baby per pair
(Q-075). A single animal (no pair) is only happy — no baby. Wrong or ordinary food changes
nothing. This **replaces the former counting rule** ("3 care feedings on 3 sessions", retired):
happy-making special food is what triggers the baby; feeding the correct storage food only gives a happy reaction (hearts), never a baby (Q-198 answered 2026-09-30). Babies are a reward
for care, never required for the mission or blocking. All species are pairs (Q-308); koalas take no garden treats, so they need their own special food
(eucalyptus treat, Q-100).

**Treats for every species (user request 2026-10-03, replaces the Q-281 fallback "own favourite food"; proposal Q-334…Q-352):** every species has an explicit **treat**: a garden treat (carrot, potato, apple, orange) or a **box food** that is *not* its basic food (snow fox: meat, lion: bone, koala: leaves, goldfish: leaves, hedgehog: fruit, bat: nectar, owl: worms, snake: eggs, chameleon: frozen insects, poison dart frog: crickets). Master table, giving rules (basic = hearts, treat = hearts + baby, other = gentle refusal) and the info board lines: GAME-FEED "Basic food and treats". The baby is still once per pair and species, optional, saved; a baby already born by the old fallback is kept (FAM-032).

## Implementation status (2026-10-01)

- Pairs: an enclosure with `pair = true` gets two animals (member 0 male, member 1 female) that start
  at the same picked hiding place, follow together when either is shown the right food and complete
  the mission only when both are home (a waiting partner is fetched first, GARD-014). The flag is on
  for every enclosure (Q-308). Saves keep both (`member`, GAME-SAVE v2); old one-zebra saves reopen the
  pair mission (RESC-032).
- Models: `zebra_female`, `zebra_foal`, `koala_female`, `koala_joey` (ART-ANIMALS "Family models");
  other species use a fallback look until their models exist (`zoo_core::animals::female_model`,
  `baby_model`; the host falls back to the male model if a file is missing). The baby (`game.babies`)
  is drawn next to the female with her clips and follows her (FAM-011..018).
- Baby trigger: special food to a pair at home (FAM-008/009). The care-feeding / trough rule is
  retired (FAM-003/005).

## Implementation status (2026-10-01, all species)

`pair = true` is on for all 13 enclosures of the zoo (levels 1–3 and `night_1`): 26 animals.
Spacing, entering, feeding-spot cells and perch offsets: rules 11–12; fallback look and texts:
see above. Renderer: `CharacterDraw { scale, tint }` (no extra draw call per tint; one more skinned
character costs its parts' draw calls like any animal, measured in the performance log).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| FAM-001 | Given a new game, then the zebra pair and the koala pair each start at one shared hiding place, both escaped. | unit |
| FAM-002 | Given the correct food shown to one animal of a pair, then both follow as one group; the mission completes only when both are in the enclosure. | unit |
| FAM-003 | *Retired 2026-10-01 — care feeding / trough replaced by special food (Q-198, Q-248).* | — |
| FAM-004 | Given a liked special food given to the pair at home, when the player next comes within view of the enclosure, then a baby appears once with a celebration; it never appears twice. | unit |
| FAM-005 | *Retired 2026-10-01 — see FAM-009 (ordinary or disliked food: no baby, no penalty).* | — |
| FAM-006 | Given a save with a baby, when restored, then both are unchanged. | unit |
| FAM-008 | Given a pair (male + female) at home and a carrot given to the zebras, then both are happy (hearts) and exactly one baby appears once; given the same carrot to a single animal (no pair), then it is happy and no baby appears. | unit |
| FAM-009 | Given a pair at home and ordinary food or a disliked treat, then no baby appears and no penalty; given a saved game after the baby, then it is restored and never appears twice. | unit |
| FAM-010 | Given a pair, a baby and a child with a liked treat at the feeding spot, then male, female and baby stand on the spot cells (ranks 0, 1, 2), inside the fence, and all three turn to the child (GARD-013, unit). | unit |
| FAM-011 | Given a pair that is out and a save with the baby flag, when restored, then the baby stands beside the female at the hiding place; when the pair is led home, then the baby ends inside the fence in the female's area. | unit |
| FAM-012 | Given the female follows the child along a path, then the baby keeps within the loose leash (5 m, playful following FAM-014) of her. | unit |
| FAM-013 | Given a pair at home and a baby that is outside her area, then it is put inside; a child in front of the baby is offered the treat (nearest member includes the baby). The HINT-021 fuzz includes babies. | unit |
| FAM-014 | Given a pair out following the child (walks a long straight route), then over several minutes the baby never is farther than 8 m from the female, the mean distance is between 1.8 and 4 m, and the baby's lateral deviation from her heading exceeds 0.5 m (not a straight line behind her). | unit |
| FAM-015 | Given a pair at home (nobody around), then over minutes the baby stays inside the female's area, within 6 m of her, its mean distance is 1.8..4 m, it uses at least 3 different modes (roam, sniff, burst) and its speed never exceeds 2.2 × the follow speed (walking 1.6 ×). | unit |
| FAM-016 | Given the same seed and inputs, then two games produce identical baby positions and modes (deterministic); a baby more than 8 m away runs back within 10 s; a baby farther than 30 m is still put beside her. | unit |
| FAM-017 | While the baby plays, its cell is always walkable and not prop-blocked, also on the way out (no clipping); the hop height stays within 0..`HOP_HEIGHT_M` and is 0 while it is not bursting. | unit |
| FAM-018 | Given a treat/feed-spot call, then the baby still goes to rank 2 and takes treats (nearest member), unchanged by playing; the HINT-021 fuzz stays green. | unit |
| FAM-021 | Given a new game, then every species of the game (13) has exactly two members (male = member 0, female = member 1) at one shared hiding place. | unit |
| FAM-022 | Given every hiding place of every pair species, then its wander area has ≥ 9 cells and two cells at least the pair gap and ≤ 3 m from the spot. | unit |
| FAM-023 | Given every enclosure, then ≥ 12 home cells and (if fed) a feeding spot whose rank 0/1/2 cells are distinct home cells, adults a pair gap apart (hippo: 1 m, Q-282). | unit |
| FAM-024 | Given any seed, then the pair starts a pair gap apart (≤ 3 m), perched members at different perch points. | unit |
| FAM-025 | Given two simulated minutes of wandering (escaped, at home, night zoo), then no pair comes nearer than the gap (-5 cm), except at an animal house (footprint and the ring of cells around it, GAME-HOUSE rule 4). | unit |
| FAM-026 | Given the right food shown to one animal of each pair species, then both follow and both enter together; the mission completes. | unit |
| FAM-027 | Given a liked treat, then one baby per pair with a liked treat; looks: dedicated models scale 1, fallback 0.92 / 0.45. | unit |
| FAM-028 | Given every pair species' info board, then the pair note exists in all reading levels (kiga = ♂ ♀) and de/en; home texts are plural. | unit |
| FAM-029 | Given two goldfish, then fish food + filled bowl puts both into the bowl, save/restore keeps both, delivery brings both home; an old one-fish save reopens the mission. | unit |
| FAM-030 | *Retired 2026-10-03 (user request: explicit treats, see FAM-031 and FEED-036…040; the code test is rewritten).* Was: Given a pair at home of a species that likes no garden treat (snow fox, koala, lion), when the child gives it its own favourite food carried from the box, then exactly one baby is born (once); given a species with a liked garden treat (zebra), then its box food gives no baby (user report 2026-10-03). | unit |
| FAM-031 | Given every species of the master table (16), then a pair at home given its treat (garden basket or box in the hands) gets exactly one baby (once, saved), given its basic food only hearts, given any other food a gentle refusal; the baby model / name of the new species are `snake_hatchling`, `chameleon_baby`, `frog_froglet` (fallback 0.45 scale until the models exist). *Implemented 2026-10-04 for the three night_2 species only (`night2_game.rs`).* | unit |
| FAM-032 | Given a save in which a baby was born by the retired own-favourite-food rule (FAM-030), then the baby stays and is not born twice; FAM-030 itself is retired (superseded by FEED-036..040). | unit |
| FAM-033 | Given the new pairs `snake`, `chameleon`, `poison_dart_frog` at their hiding place and at home, then pair gaps 1.0 / 0.7 (perch offset) / 0.5 m, home wander areas ≥ 12 cells, split pairs and a waiting partner are fetched like every pair (GARD-014), group look-ups use the species and never the first member. | unit |
| FAM-034 | Given the baby speed factors, then each equals the first design (0.7 / 1.25 / 1.6 / 2.2) × 0.85 (babies run 15 % slower, user request 2026-10-04) and a baby never exceeds `walk × BURST_FACTOR`. | unit |
| FAM-007 | Given male and female models side by side from the default camera, then children can tell they are a pair of the same species and spot the difference (manual review). | manual |

## Open questions

- Q-198 answered 2026-09-30: special food to a pair triggers the baby; see "Special food and babies".
- Q-073 / Q-030 answered by Q-308 (2026-10-01): every species is a pair.
- Q-074 How male and female differ visually.
- Q-106 answered by the pair flag being on (2026-09-30).
- Q-203 answered (zebra female model v2); Q-204 baby behaviour and placement (FAM-011..018).
- Q-075 answered by Q-198: special food triggers the baby, one baby per pair.
