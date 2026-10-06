---
id: GAME-CART
title: Golf carts
aspect: gameplay
module: golf-carts
status: draft
depends_on: [GAME-PLAYER, GAME-LAYOUT, GAME-RESCUE, GAME-SAVE, CONT-MATH, GAME-HINT]
test_prefix: CART
updated: 2026-10-07
---

# Golf carts

## Goal

The zoo has **3 golf carts** (*Golfwagen*) the child can drive to get from one place to
another more quickly (user request 2026-09-26). **Animals do not follow a cart** — rescuing
still means walking the animal home. Carts and their key are an **optional side activity**:
no mission, hint of priority <= 3, night or level change ever needs one (GAME-LAYOUT "Level
design rules"), so a child who never solves the math note is not stuck.

**Names (glossary):** the drivable vehicle is always `golf_cart` (id prefix `cart_`, never
"zookeeper cart"). The static hand cart on the repair barrier (`zookeeper_cart`, ART-ENVIRONMENT
props) is a different prop: it cannot be entered, driven or locked and has no `[[cart]]` entry.

**Status 2026-10-07:** P1 (math core), P2 (note, key box, lock panel) and **P3 (carts, driving)** are implemented:
`zoo-core/src/{math,cart_key,cart}.rs`, `[[cart]]` loading in `level.rs`, save v3 `carts` / `seated`,
`web/src/lock-panel.ts`, `math-aid.ts`, the get-out button / 🔒 badge / bubbles in `ui.ts`, the dynamic cart batch,
seated child, wheels / steering wheel / headlights in `zoo-web` + `zoo-render`. Tests `tests/{math,cart_key,cart,cart_fuzz}.rs`,
`tests/hints.rs` (HINT-032/033, CART-029), `view.rs` (CAMV-024), e2e `web/tests/e2e/{cart_key,cart}.spec.ts`.
Not built: cart sounds (no cue in ART-SOUND yet), `drive` clip of `player_boy` (no boy model; the girl has `drive`,
`drive_turn_l/r`). Implementation notes: the closed box and its `cart_key` and the
open box (`key_box_open.glb`, same kit script `kit_bedroom.py`) are separate placements the host swaps
(`LevelScene::key_boxes`); the note is a reading target (its panel opens by itself like a board and counts as
reading, every opening resets `key_box_tries`), the key box is a button target (lock panel, game paused).
Parked carts are prop colliders of the level (`Level::set_cart_shapes`); the driven one is not (it collides itself).

## Behaviour

1. **Three carts** (Q-119 answered), data `[[cart]]` in each level file (see "Data shape"),
   each with a small parking sign (`parking_sign`) and a painted parking rectangle on the
   ground:

   | Cart | Level | Pose (centre, facing) | Footprint rect (x, z, w, d) | Boarding cell | Why here |
   |---|---|---|---|---|---|
   | `cart_l1` | `level_1` | (4.0, 3.5), `+z` | (3, 2, 2, 3) | (2, 3) on `path_plaza` | East edge of the entrance plaza, 4.3 m from the spawn (0, 2): in the first screen, beside the way to the storage, no gate, board or hiding place near. |
   | `cart_l2` | `level_2` | (28.5, 31.7), `+x` | (27, 31, 3, 2) | (28, 30) on `path_l2_entry` | Grass strip between the entry path and the koala fence at the level entry, 3.4 m from the spawn (27, 29); west of `map_board_l2`, 4 m from `loc_fountain`'s rect. |
   | `cart_l3` | `level_3` | (15.5, 51.0), `+x` | (14, 50, 3, 2) | (15, 52) on `path_l3_entry` | Grass strip between the entry path and the south hedge `hedge_l3_south_b`, 5 m from the spawn (20, 53). |

   Verified 2026-10-06 against `assets/levels/level-1/2/3.toml`: the footprint cells overlap no
   element other than the path they stand on, no hiding-place rect or wander area, scenery,
   garden, gate, door, `[[item]]`/`[[food_box]]`/`[[event_spot]]`/`[[light]]` position and no
   `stand` cell; the 1-cell ring touches only `fountain_sw`/`hedge`/`enc_koala` (decoration
   edges, no gate); the three level entry paths stay free (3 m wide). A level-2 spot at
   (30, 26, 3, 2) was rejected: it overlaps `loc_fountain` (rect 28, 22, 3, 6).
   Parking signs (0.2 m footprint) at (5.5, 2.5) / (26.9, 31.5) / (13.5, 50.5), facing the cart. (Level 2 moved 2026-10-07: cart z 32.0 -> 31.7 and sign x 26.5 -> 26.9 so that no gap of 0.1-0.6 m opens between a parked cart / the sign and a wall near the level gate, LAYOUT-039.)
2. **Locked or usable.** A cart is **locked** while the child has no cart key
   (`has_cart_key = false`) **or** its level is not open yet (`locked_until`, e.g. the barrier
   of the level has not opened — the cart then also stays hidden behind the barrier in
   practice, Q-370 answered). The interact button next to a cart always appears (cart icon,
   with a 🔒 badge while locked) so the child learns that carts exist; tapping a locked cart
   gives the **locked feedback** (rule 13).
3. **Getting in / out:** the cart is *available* when the player stands within **1.5 m of its
   box** (either side; no facing rule — the figure always sits in the driver's seat) and the
   cart is not locked. Interact → she sits; the animals that follow wait (rule 7). Interact
   again or the big **get-out button** (🚶 icon, ≥ 64 px, replaces the interact button while
   seated, always shown) → she steps out on the first free walkable cell of the search
   order: driver's side (left of the heading), passenger side, behind, in front, then a ring
   up to 2.5 m; the cart stays exactly where it was left (Q-119 answered). **Parking check**
   (rule 4): she can only get out where a parked cart is harmless.
4. **Parking check (proposal Q-372, default decided).** On get-out the game tests the parked
   box: (a) its footprint (plus a 0.5 m margin) must not cover a gate, door, entry, barrier
   or `stand` cell of any `[[item]]`, board, storage door or hint target, a food box or a
   hiding-place rect; (b) with the footprint cells blocked, every walkable cell that was
   reachable from the level spawn still is (flood fill of the walkable grid, the code of
   LAYOUT reachability tests). If the check fails, get-out is refused: a 🅿️✖ bubble ("Hier
   kann ich nicht parken" / "I can't park here") and the child keeps driving — she is never
   stuck because driving is always possible; the three parking rects always pass. A parked
   cart is a solid box for the player, animals and visitors (they walk around it); because of
   (b) it never closes a way.
5. **Driving model** (zoo-core `cart.rs`, pure, deterministic):
   - **Same controls as walking** (left thumb / WASD): the stick gives a world direction
     relative to the camera. The cart **steers towards that direction** at up to 150°/s and
     accelerates (6 m/s²) towards `speed_max × cos(angle error)` while the error is < **30°**
     (`DRIVE_ERROR_DEG`, implementation 2026-10-07: with 90° a 180° turn moved the cart > 1 m
     before it pointed the right way); at an error ≥ 30° the target speed is 0 → it brakes
     (10 m/s²) and **turns on the spot**.
   - **No reversing** (decided default, Q-372): pulling the stick back never drives backwards;
     it turns the cart around on the spot (180° in ≤ 1.3 s) and then drives forward. Speed is
     never negative — with one exception, the **wedge escape**: turning on the spot does not
     always work (a nook between a bench and a rock hill is narrower than the box's diagonal), so
     when the held stick makes no progress for 1 s and points backwards the cart backs out at
     1.5 m/s (`REVERSE_SPEED_MS`); after 3 s without any progress the cart is lifted to the nearest
     **roomy pose** within 8 m (one where it can turn through every heading, no animal within
     1 m; `Game::cart_rescues`). A friendly 🔔 horn button is optional.
   - **Speed:** paths **4.5 m/s** (≈ 2.3 × walking), grass **2.0 m/s** (Q-120 answered),
     chosen from the surface of the cell under the box centre, smooth blend (the
     acceleration limits above).
   - **Camera:** zooms out +3 m while driving and follows smoothly; only the zoo view is
     available (GAME-CAMERA-VIEWS rule 11, CAMV-024); look-around/first person return after
     getting out.
6. **Where carts can go:** only walkable cells of **open** levels — paths and grass; never
   through barriers, fences, **gates into enclosures, doors and building interiors**, the
   garden gate, buildings, water (bridges are fine if at least 2 m wide), dense woods and
   sparse-wood trunks, props or other parked carts. The cart collides as an **oriented box
   1.3 × 2.3 m** (the player's collision grid + collider list, box instead of circle) and
   slides along obstacles. **Never stuck:** a turn that would overlap a solid is resolved by
   pushing the box out along the shortest separating axis (≤ 0.5 m per step); if no free
   pose exists within 1.5 m (cannot happen on valid level data) the cart is put back on its
   parking pose and the player stands on the boarding cell (`Game::cart_resets`) — a counter
   test asserts this is never needed in the fuzz (CART-018); the wedge rescue above is the
   normal safety net and may fire (`cart_rescues`). Carts do not exist in the night levels
   (`night_1`, `night_2`, decided default, Q-372); the moon door cannot be used while seated.
7. **Safety, child-friendly:** the cart slows down and stops ≥ 0.5 m before it would touch
   a visitor, an animal or a duck on land (look-ahead along the heading, 1.5 m + stopping
   distance). Animals and visitors never walk into a driven cart's box (it is a collider for
   them as the player is). Nothing can be run over or pushed. No crashes, no damage, no timer.
   When she gets in while animals are `following`, they stop and **wait** at that spot
   (GAME-RESCUE §6; the existing partner/waiting hint leads back to them); when she is back on
   foot within 5 m they follow again. A short bubble ("Wir laufen lieber!" / "We'd rather
   walk!") tells the child; showing food from the cart does nothing.
8. **Carried things:** food, the basket, the fish bowl (with the fish) and the key come along;
   the fish stays safe in the bowl.
9. **Interactions while driving:** reading panels do not open automatically while driving;
   boards, doors, gates, beds, the moon door, food boxes and animals are not interactable
   while seated (the get-out button is always the only action), the map and the settings can
   be opened. The 🧭 hint works as always (a hint target needs "get out" at the end; the
   hint's icon line is unchanged).
10. **Night:** the cart has two small headlights (night and dusk, GAME-NIGHT); a parked cart
    keeps them off.
11. **Art:** a comic zoo golf cart — **red-white: red body, white roof with red stripes** (decided 2026-10-06, changed by the user the same day from green-white because the zoo has much green already;
    Q-371), zoo logo space left blank, 2 seats, a small cargo area where carried items are
    visible, `parking_sign` (post with a P and a cart icon). Concept sheet first (ART-PIPELINE);
    both player characters (`player_girl`, `player_boy`) have a `drive` sitting clip (ART-RIG).
    Model id `golf_cart`; collision footprint B(0, 0, 0.65, 1.15) (the driving box).

## The golf-cart key (user request 2026-09-27)

12. **Carts need a key.** The carts are locked until the child has the **cart key**. The key
    hangs in a **key box with a combination lock** (*Schlüsselkasten mit Zahlenschloss*) on
    the wall of the zookeeper house (level 1, `key_box_l1`, outside, south of the door).
13. **Locked feedback (decided 2026-10-06, Q-369).** Tapping interact at a locked cart shows
    the bubble 🔒🔑 with `cart-locked` ("Der Wagen ist abgeschlossen. Such den Schlüssel!") —
    pictograms on `kiga` — and activates the hint marker for 12 s at the **next key step**
    (see "Hints"): the note while the child has not read it, the key box afterwards (or the
    note again after 3 wrong codes); the 🧭 button pulses three times. A cart that is locked
    only because its level is closed says `cart-closed-level` (🔒 + "Hier geht es noch
    nicht weiter") and gives no hint. Interact is the child's own action, so this is no hint
    loop; it never auto-repeats.
14. **The combination is a math task:** a **note** (`note_math_fighter`) lies on the **desk
    inside the zookeeper house** (`stand` (-11, 2)). It is a sheet with the title **"Math
    Fighter"** (a name, not translated) and one math task; its result is the combination.
    **The lock has 3 digits; the result 1…999 is written with leading zeros** (5 → `005`,
    Q-132). The task follows the child's `math_level` (setting, rule 21; default `mathe1`) and
    is generated per playthrough from the game seed on its own RNG stream (CONT-MATH "Cart
    note"). Reading the note opens a reading panel: title, the task in big numerals
    (`kiga`: with pictures), three empty digit boxes `☐☐☐` and a small picture of the key box;
    **first reading sets `note_read` and resets `key_box_tries` to 0**.
15. **Opening the lock:** interact with the key box (stand (-8, 1), facing `+x`) → a **lock
    panel** with three big number wheels (▲ / ▼ ≥ 72 px per digit, digits 0–9, start `000`,
    no text needed) and a big ✔ (🔓) button; ✖ closes the panel. The ✔ with the **right
    code** closes the panel and plays the opening: the box door swings open (0.5 s), the key
    lifts out and flies into the pocket (0.5 s), the HUD shows 🔑 (a chip next to the basket
    until the game is reset). State: `key_box_open = true`, `has_cart_key = true`; the key is
    **gone from the box** (open mesh `key_box_open`, empty hook, no `cart_key` mesh), and the
    open box is **no longer interactable** (no button, no hint). **Wrong code:** gentle shake
    (0.4 s) and a soft sound, the wheels keep their digits, `key_box_tries += 1`. From 3 wrong
    codes in a row (since the note was last read) the panel shows a **pulsing 📝** and the
    🧭 hint leads to the note (Q-369). **Never a lockout, never a timer**: any number of tries.
16. **Hints (priority 4, optional — GAME-HINT "Cart key hints").** Two kinds: `note` (📝, line
    `hint-cart-note`) at the note's stand cell and `keybox` (🔑, `hint-cart-keybox`) at the key
    box's stand cell. While `has_cart_key = false` exactly one is offered:
    - `key_box_tries >= 3` → **note** (the child needs the task again; reading the note sets
      `key_box_tries = 0`);
    - else `note_read = false` → **note**;
    - else → **key box**.
    Each step changes `Game::hint_signature` (`note_read`, `key_box_tries`, `key_box_open`,
    `has_cart_key`), so it cannot repeat without a state change (HINT-026…029 repeat guard,
    HINT-033). **Never after the key was taken. Never the only candidate while a mission
    is open (they rank behind mission steps) and never blocks `all_done` / the celebration**
    (`hints::what_next` ignores them). The carts themselves are never a hint target.
17. **Fallback without a math level:** `math_level` always has a value (default `mathe1`; a
    missing, unknown or disabled value counts as `mathe1`), so the note always shows an
    age-appropriate task: `mathe1` "2 + 3 = ?" (answer 5, lock `005`). A parent who switches
    math off (CONT-MATH Q-034) still gets the `mathe1` note, because carts are optional fun.
18. **Changing the math level** (settings) regenerates the note task and combination from the
    same seed and the new level, resets `key_box_tries` to 0 and keeps `note_read`; an
    already opened box stays open.
19. **Math is never required for a mission** (MATH-005): only the optional key box uses it.
20. **Art:** `key_box` (wall box with a 3-wheel combination lock; models `key_box` closed and
    `key_box_open`), `cart_key`, `note_math_fighter` (paper with the "Math Fighter" title, drawn
    by the game's text path, the task rendered from data), `desk` (zookeeper house interior),
    `golf_cart`, `parking_sign`.

## Settings: math level (decided 2026-10-06, Q-368)

21. The settings menu (`#settings`) has a **math row** `#settings-math` directly **under the
    reading-level row**: a leading 🔢 icon (not tappable) and five choice buttons
    `mathe1`…`mathe5`, each ≥ 64 × 64 px, showing the numeral 1…5 over a row of that many
    small dots (recognisable without reading; reading row uses 📖 + 🧸/1/2/3). The chosen
    button is highlighted; default `mathe1`. Stored in the host settings as `zoo.mathLevel`
    (like `zoo.readingLevel`) and in the game save (rule "Save"); core API
    `set_math_level(id) -> bool` / `math_level()`; an unknown id is refused (returns false).
    Changing it takes effect at once (rule 18). The row is independent of the reading level
    (CONT-MATH); a hint text per level is not needed.

## Data shape: `[[cart]]` (GAME-LAYOUT "Night lights, interactables and furniture")

```toml
[[cart]]
id = "cart_l1"
pos = [4.0, 3.5]        # centre of the parked cart, level metres
facing = "+z"           # parking heading (+x/-x/+z/-z); the save stores a free yaw
rect = [3, 2, 2, 3]     # parking footprint cells (the oriented 1.3 x 2.3 m box lies inside)
stand = [2, 3]          # walkable boarding cell (tests, scripted player, boarding hint)
sign_pos = [5.5, 2.5]   # parking sign
locked_until = ""       # level id that must be open ("" = always open; "level_2", "level_3")
model = "golf_cart"
notes = "..."
```

`locked_until`: `cart_l1` `""`, `cart_l2` `"level_2"`, `cart_l3` `"level_3"`. All carts also need the
key (`has_cart_key`). The spec table and the toml must agree (LAYOUT-005, LAYOUT-048).

## Save (GAME-SAVE, **format version 3**)

`carts`: per cart `id`, `x`, `z`, `yaw`; `seated`: cart id or none; `has_cart_key`,
`key_box_open`, `note_read`, `key_box_tries` (u8, saturating), `note_aid` (the 3rd wrong code switched the
note's visual aid on, cleared by a new task), `math_level` (`mathe1`…`mathe5`).
All fields have `#[serde(default)]`: a version-2 save loads with the carts on their parking
poses, no key, closed box, `note_read = false`, `tries = 0`, `mathe1`. A save made while seated
restores her in that cart; if the pose is not walkable any more, the nearest free pose within
4 m, else the parking pose. After restore the host's `math_level` setting wins (tries reset if
it differs). `has_cart_key` implies `key_box_open` on load (repair of a broken save).

## Fluent keys (de + en, `assets/i18n/<lang>/cart.ftl`, reading-level variants where noted)

`cart-locked`, `cart-closed-level`, `cart-walk` ("Wir laufen lieber!"), `cart-no-park`, `hint-cart-note`
("Lies den Zettel im Haus"), `hint-cart-keybox` ("Öffne den Schlüsselkasten"), `cart-note-title`
("Math Fighter", same in both), `cart-note-line-<kiga|klasse1|klasse2|klasse3>` ("Das Ergebnis ist der
Code für den Schlüsselkasten"), `cart-keybox-open`, `cart-key-got` ("Du hast den Schlüssel!"),
`next-explore` is unchanged. Lock panel and HUD (implemented): `cart-lock-title`, `cart-lock-wrong`,
`ui-key`, `ui-lock-up`, `ui-lock-down`, `ui-lock-open`, `ui-lock-close`, `ui-cart-get-out`, `ui-cart-board` (aria labels, no text needed), math row
`ui-math-level`, `ui-level-mathe1`…`ui-level-mathe5` (in `math.ftl`). Math task texts: CONT-MATH `math-cart-<kind>-<reading level>`.

## Work packages (implementation order)

| WP | Content | Crates / files | Depends on | Size |
|---|---|---|---|---|
| **P0** | This spec round: data shape, parking, rules, tests, questions (done 2026-10-06) | specs, `assets/levels/*.toml` `[[cart]]` draft | — | done |
| **P1 math core** | `MathLevel` (`mathe1`…`mathe5`, default `mathe1`), `MathTask`, `cart_note_task(seed, level)` on its own RNG stream, 3-digit padding, visual-aid kind, `math.ftl` de+en, settings `math_level` in core + web API | `zoo-core/src/math.rs`, `assets/i18n/*/math.ftl`, `zoo-web` (`set_math_level`, `math_level`), `web/src/ui.ts` settings row + `zoo.mathLevel` | P0 | ~1 d |
| **P2 key box, lock panel, note** | `Target::KeyBox`/`Note`, `Game::enter_code`, `key_box_tries`, `note_read`, `has_cart_key`, `key_box_open`, HUD 🔑, `HintKind::Note`/`KeyBox` (priority 4, stages), compass/what_next exclusion, save v3 (key fields + math_level), render: open/closed key box mesh swap, hide the key mesh, note panel, lock panel with ▲/▼ wheels, `cart.ftl` (key part) | `zoo-core` (`game`, `hints`, `save`, `scene`), `zoo-web`, `web/src/ui.ts`, `web/index.html` | P1 | ~2 d |
| **P3 cart** | `golf_cart` model + concept sheet + `drive` clip + `parking_sign`, `[[cart]]` loading, `cart.rs` kinematics + box collision + animal/visitor stop, parking check, locked feedback, follow-wait, carried items, no panels while driving, camera +3 m zoo only, touch get-out button, headlights, save v3 (carts, seated), manifest + `check_glb` | `tools/blender/props/golf_cart.py`, `character-artist` (clips), `zoo-core/src/cart.rs`, `scene`, `zoo-render`, `zoo-web`, `web` | P0, P2 (key) | ~3–4 d |
| **P4** | gameplay-qa + performance run, spec-manager, i18n check | — | P3 | ~1 d |

Ship order: P1 → P2 (playable alone: opening the box gives the key; until P3 the HUD key and
a "Du hast den Schlüssel!" bubble are the only result — or hide the key-box item behind a build
flag) → P3. P3 can start the art (concept, model, clip) in parallel with P1/P2.

## Never stuck / no hint loops (binding, user rules 2026-10-01 and 2026-10-04)

Report for the implementation (every package must repeat this check):

- **Carts and the key are optional**: no mission, hint of priority <= 3, bed, moon door or
  celebration depends on them; `all_done` ignores the cart hints (HINT-033).
- **Cart cannot trap the child:** no reversing but turn on the spot, slide along obstacles,
  depenetration, parking check (never closes a way), get-out always possible on the next
  valid cell, the cart is never in a night level, the saved seated state restores on a free
  pose, fuzz CART-018.
- **Key box cannot trap the child:** no lockout, unlimited tries, the note is always readable
  (desk in an open house, no key needed), the task is always solvable at the chosen level
  (MATH-007…013), any change of the math level gives a new valid task.
- **Hint loops:** a hint stage ends with a state change (note read → key box → key taken); a
  repeated visit without change is dropped by the repeat guard; the pulse after 3 wrong codes
  ends when the note was read (`tries = 0`); the locked-cart feedback is child-initiated.
- Extend the follow-the-hints fuzz and the invariants **in the same change** as P2 and P3
  (CART-029, HINT-032/033).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| CART-001 | Given a new game, then 3 carts stand at their parking poses (the table in rule 1, pose = centre of `rect`, heading = `facing`) and only the carts of open levels can be entered. | unit (`zoo-core/tests/cart.rs`) |
| CART-002 | Given the player within 1.5 m of a usable cart, when she interacts, then she sits in it; interacting again (or the get-out button) puts her on a free walkable cell next to it (driver's side first) and the cart stays where it is. | unit |
| CART-003 | Given the cart on a path, when driving forward for 1 s at full speed from rest after 1 s of acceleration, then it moves at 4.5 m/s ± 5 % (grass 2.0 m/s ± 5 %); the speed changes smoothly (acceleration <= 6 m/s²). | unit |
| CART-004 | Given the cart driven against a fence, a barrier, water, a building wall, a door, an enclosure gate and the level border, then its box never overlaps them and it never enters a closed level or a building interior. | unit |
| CART-005 | Given an animal, a visitor or a duck on the cart's path, then the cart stops >= 0.5 m before touching it, and no animal or visitor ever stands inside the cart's box. | unit |
| CART-006 | Given the zebra is following, when the player gets into a cart and drives 20 m away, then the zebra waits where she got in; when she returns on foot within 5 m, it follows again. | unit |
| CART-007 | Given the player carries the fish bowl with the fish (and food, the basket, the key), when she drives and gets out, then she still carries all of it. | unit |
| CART-008 | Given the player drives past an info board, then no reading panel opens, and no board, door, bed, box or animal interaction is offered while seated except get-out. | unit |
| CART-009 | Given a save made while sitting in a cart, when restored, then she sits in the same cart at the same pose; given the pose is no longer walkable, then at the nearest free pose within 4 m, else the parking pose. | unit |
| CART-010 | Given touch controls, then driving works with the left thumb exactly like walking and the get-out button (🚶, >= 64 px) is reachable with the right thumb, does not overlap the gear / compass and the view button is hidden (780×360, 360×780). | e2e (`web/tests/e2e/cart.spec.ts`), vitest (`ui.test.ts` icons) |
| CART-011 | Given a new game, then the carts cannot be entered (🔒 badge on the interact button) until the cart key is in the pocket; given the key but a closed level, the cart of that level stays locked. | unit |
| CART-012 | Given the note on the desk, then it shows the title "Math Fighter" and a math task for the child's math level; its result is the key-box combination (3 digits, leading zeros); different seeds give different tasks (CONT-MATH MATH-007…013). | unit (`tests/cart.rs`) |
| CART-013 | Given the key box, when the right combination is entered, then the box opens, the key goes into the pocket and all carts of open levels can be used; a wrong combination shakes and counts a try; from 3 wrong codes the note pulses and the 🧭 hint leads to the note. | unit |
| CART-014 | Given the key box panel on touch, then every digit wheel can be set with big ▲/▼ buttons (>= 72 px) without reading, ✔ opens, ✖ closes. | e2e |
| CART-015 | Given the three level files, then each has exactly the `[[cart]]` of rule 1 with `id`, `pos`, `facing`, `rect`, `stand`, `sign_pos`, `locked_until`, `model`; the oriented 1.3 × 2.3 m box at `pos` lies inside `rect`; the spec table and the toml agree (LAYOUT-005, LAYOUT-048). | unit (`layout`) |
| CART-016 | Given each parking rect, then its cells are walkable and overlap no solid element, hiding-place rect, wander area, scenery, garden, event spot, prop collider, light post or `stand` cell, lie >= 1 cell from every gate / door / entry / barrier cell, the boarding cell is a walkable cell <= 1.5 m from the box, and the parked box does not disconnect the walkable grid (LAYOUT-048). | unit |
| CART-017 | No reversing: given the stick pointing backwards, then the cart turns on the spot (180° in <= 1.3 s, moved <= 0.2 m) and then drives forward; its speed is never negative (except the wedge escape). | unit |
| CART-018 | NEVER STUCK (cart): given seeded random stick sequences (3 seeds × 2000 steps from each parking pose, in the joined zoo, with animals and visitors around, incl. save/restore in the middle), then the box never overlaps a solid, never leaves open levels, never needs the parking-pose reset (a wedged cart is lifted to a roomy pose, `cart_rescues`), and from every end state get-out finds a valid cell after at most a few more driven metres. | unit (fuzz) |
| CART-019 | Parking check: given the cart in a gate apron, a door cell, on a stand cell, in a 2 m corridor or any pose that would disconnect the walkable grid, when the child taps get-out, then she stays seated and the 🅿️✖ bubble shows; after driving to a valid pose get-out works; the three parking rects always pass. | unit |
| CART-020 | Given the key but the level of `cart_l2` closed, then its interact shows `cart-closed-level` and no hint; given the level opens, the cart is usable. | unit |
| CART-021 | Locked feedback: given a locked cart, when the child interacts, then the bubble `cart-locked` shows and the hint marker points for 12 s at the note (`note_read = false` or `key_box_tries >= 3`) or the key box; it does not repeat by itself. | unit |
| CART-022 | Key box state: given the right code, then `key_box_open`, `has_cart_key`, the open mesh is used, the `cart_key` mesh is hidden and the box offers no interaction and no hint any more; given save/restore, all of it persists. | unit |
| CART-023 | Combination: given any seed and `math_level`, exactly one of the 1000 codes `000`…`999` opens the box, and it equals the padded answer of the note task (5 → `005`). | unit |
| CART-024 | Note: given the note read for the first time, then `note_read = true` and `key_box_tries = 0`; the panel shows the title, the task, `☐☐☐` and the key box picture; on `kiga` with pictures and the pictogram line. | unit + e2e |
| CART-025 | No lockout: given 1000 wrong codes in a row, then the right code still opens the box; nothing is timed. | unit |
| CART-026 | Math level change: given a new `math_level` in the settings, then the note task and the combination follow it (same seed), `key_box_tries = 0`, an opened box stays open, `note_read` stays. | unit |
| CART-027 | Save v3: given a version-2 save, then it loads with the carts on their parking poses, no key, closed box, `mathe1`; given a v3 save with all fields, restore equals the original (SAVE-012/013). | unit |
| CART-028 | Given night or dusk, then a driven cart has its headlights on and a parked cart not; the moon door and the bed are not interactable while seated. | unit |
| CART-029 | NEVER STUCK / NO LOOPS: given the follow-the-hints fuzz (HINT-019/027) extended with cart actions (locked-cart taps, wrong and right codes, reading the note, driving, getting in and out, split pairs, save/restore, math level changes), then every state has a hint of priority <= 3 while a mission is open, the run reaches "all animals of the level home" and then dusk → bed, and the loop detector (`follow_one`: > 3 identical hints in a row with an unchanged signature) never fires. | unit (`tests/hints.rs`) |
| CART-030 | Settings: given the settings menu, then the math row has 5 buttons (>= 64 px) under the reading row, the choice persists in `zoo.mathLevel`, default `mathe1`, and `set_math_level("mathe9")` is refused. | vitest (`ui.test.ts`) + e2e |
| CART-031 | i18n: every cart / math key exists in `de` and `en`, with reading-level variants where listed. | unit (`i18n`) |
| CART-032 | Models: `golf_cart`, `parking_sign`, `key_box`, `key_box_open`, `cart_key`, `note_math_fighter`, `desk` exist as `.glb`, load, and are listed in the manifest (`concept_approved` gate for `golf_cart`/`parking_sign`). | unit (`zoo-assets`) |

## Open questions

- Q-119 answered 2026-09-27: parking spots as in rule 1; a cart stays where it was left.
- Q-120 answered 2026-09-27: paths 4.5 m/s, grass 2.0 m/s (carts may drive on grass).
- Q-125 answered 2026-09-27: zoo view only while driving (GAME-CAMERA-VIEWS rule 11, CAMV-024).
- Q-132 answered 2026-09-27: 3 digits with leading zeros.
- Q-368 answered 2026-10-06 (user, as recommended): math level in the settings next to the reading level, default `mathe1`.
- Q-369 answered 2026-10-06 (user, as recommended): after 3 wrong codes the note pulses and 🧭 leads to it; never a lockout, never time pressure.
- Q-370 answered 2026-10-06 (user, as recommended): carts of locked levels exist, with a 🔒 until their level opens.
- Q-371 answered 2026-10-06 (user): cart look RED and WHITE (red body, white roof with red stripes), not green-white (much green in the zoo already).
- Q-372 open (proposal, decided default implemented in the spec): parking check on get-out, no reversing, no carts in night levels.
- Q-373 open (proposal): result range of the key-box task (1…999, 3 digits) limits `mathe4`/`mathe5` tasks.
