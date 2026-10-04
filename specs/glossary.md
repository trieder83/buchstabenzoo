---
id: PROD-GLOSSARY
title: Glossary
aspect: product
module: glossary
status: draft
depends_on: []
test_prefix: GLOS
updated: 2026-10-03
---

# Glossary

One name per concept. Code identifiers use the **Code** column; German is the reference
language for player-facing text.

| Code | Deutsch | English | Meaning |
|---|---|---|---|
| `player_character` | Spielfigur | player character | The avatar the child controls (boy or girl, chosen at start). |
| `animal` | Tier | animal | A zoo animal that can be found, led and fed. |
| `enclosure` | Gehege | enclosure | Fenced area where an animal species belongs. Never "cage"/"Käfig" in code or text. |
| `enclosure_sign` | Gehegeschild | enclosure sign | Sign above an enclosure with the animal's name (+ silhouette on low levels). |
| `food_box` | Futterkiste | food box | Box whose label must be read to find the right food. |
| `pictogram` | Piktogramm | pictogram | Simple flat vector drawing of a food on a food box label and panel (GAME-FEED §1); supports the word, never replaces it. |
| `food` | Futter | food | Item an animal eats (e.g. bamboo, hay). |
| `enclosure_item` | Gehegeausstattung | enclosure item | Non-food items an enclosure needs (pool, stone, logs, bridge). |
| `visitor` | Besucher | visitor | NPC the player can talk to; gives hints. |
| `coin` | Münze | coin | Zoo money earned from visitors (ice cream 4, animal food 2) and spent in maths tasks (GAME-ECON). Never real money. |
| `ice_cream_kiosk` | Eisstand | ice cream kiosk | Shop in level 3 where visitors buy ice cream; stock refilled from the fridge. |
| `food_machine` | Futterautomat | food machine | Vending machine for animal food bags; stock refilled from the storage. |
| `fridge` | Kühlschrank | fridge | Prop in the storage building holding the ice boxes for the kiosk. |
| `animal_doctor` | Tierarzt | animal doctor | Visitor NPC who treats a sniffly animal for a fee in coins (GAME-ECON). |
| `quest` | Aufgabe | quest | A goal with steps, e.g. find the lost monkey baby. |
| `riddle` | Rätsel | riddle | Reading/logic puzzle, solved by reading a word, sentence or short story. |
| `reading_level` | Lesestufe | reading level | `kiga`, `klasse1`, `klasse2`, `klasse3` (grades 4–5: Q-035). Write "reading level", never just "level". |
| `hint` | Hinweis | hint | Indirect information given by a visitor. |
| `rescue_mission` (code: `mission`) | Rettung † | rescue mission | Bringing one escaped animal species back to its enclosure (GAME-RESCUE). The game goal is completing all of them. |
| `hiding_place` (ids `loc_*`) | Versteck † | hiding place | Location where an escaped animal waits until it is found (GAME-ANIMALS, CONT-MISSIONS). |
| `location_riddle` | Ortsrätsel † | location riddle | Riddle on the info board that describes an animal's hiding place without naming it (`klasse1`+). A kind of `riddle`. |
| `info_board` | Infotafel | info board | Board next to each enclosure sign with the animal's name, its location riddle and its food word. |
| `analytics_consent` | Statistik-Einwilligung | analytics consent | Opt-in of a parent (parental gate + "Erlauben") to anonymous usage statistics (TECH-PLATFORMS "Analytics (opt-in)"); stored in `zoo.analytics`; off by default. Never "tracking" in player-facing text. |
| `food_storage` | Futterlager † | food storage | Building that holds all food boxes. |
| `escaped` / `following` / `in_enclosure` | entlaufen / folgt / im Gehege † | escaped / following / in enclosure | The three animal states (GAME-ANIMALS). "Following" = walks behind the player after being shown its correct food. |
| `math_task` | Matheaufgabe † | math task | Optional arithmetic task embedded in the world (CONT-MATH). |
| `math_level` | Mathestufe † | math level | `mathe1` … `mathe5` (grades 1–5), independent of `reading_level` (proposal, Q-034). |
| `key` | Schlüssel | key | Item that unlocks the food storage (found on the pirate ship) — only if Q-033 keeps the lock. |
| `pirate_ship` | Piratenschiff | pirate ship | Landmark in the zoo; hiding place of the monkey (`loc_pirate_ship`) and, if Q-033 keeps the lock, of the key. |
| `turnaround` | Turnaround-Sheet | turnaround sheet | Concept image of a character from front, side, back and ¾ view. |
| `mockup` | Umgebungs-Mockup | environment mockup | Concept image of a zoo area before it is modelled. |
| `level` | Level | level | Bounded part of the zoo that is playable at one time (GAME-LAYOUT). Not to be confused with `reading_level`. |
| `path` | Weg | path | Walkable street/path between areas. |
| `building` | Gebäude | building | Non-enclosure structure: entrance, food storage, kiosk, zookeeper house. |
| `landmark` | Wahrzeichen | landmark | Notable place, e.g. the pirate ship. |
| `barrier` | Absperrung | barrier | Removable obstacle limiting a level: road block, stones, fallen tree, construction fence, closed gate. |
| `boundary` | Zoogrenze | boundary | Permanent outer limit of the zoo (wall, hedge, water). |
| `level_coords` | Level-Koordinaten | level coordinates | Coordinates of the layout data and game logic: x east, z north, metres (GAME-LAYOUT). |
| `world_space` | Weltkoordinaten | world space | Right-handed Y-up render/glTF space; `world = (x, 0, −z)` (GAME-LAYOUT, Q-056). |
| `run` | Randstrang † | run | One straight line of modular edge pieces (fence, hedge, wall) along x or z (GAME-LAYOUT). |
| `segment` | Segment | segment | One straight 2 m or 1 m piece of a run (`fence_wood`, `fence_wood_1m`, …). |
| `map` | Karte | map | Full-screen zoo map opened from the HUD; shows only explored areas (GAME-MAP). |
| `explored` | erkundet | explored | A map cell the player has been close to; shown on the map, never hidden again. |
| `fog` | Nebel | fog | How unexplored cells are drawn on the map. Not the camera's distance fog/haze of the close views (`fog_end`, GAME-CAMERA-VIEWS). |
| `water_field` | Wasserfeld | water field | Texture baked at level load by the zoo-core level assembly (`zoo_core::water`, one for the joined zoo), uploaded by the renderer: position along the river flow, offset across it, distance to the shore and river/pond flag; drives the water animation (TECH-WATER). Not player-facing. |
| `nightfall` | Einbruch der Nacht | nightfall | Transition from day to night (dusk) after all animals of a day level are home — once per completed day level (GAME-NIGHT). |
| `compass_strip` | Kompass-Leiste | compass strip | Small round animal icons of the species still missing in the current level, attached to the 🧭 compass button together with the task badge (the kind of the next task); replaces the former moon pane (GAME-NIGHT rule 11, GAME-HINT rule 8). |
| `night_zoo` | Nachtzoo | night zoo | New area with nocturnal animals, reached through the moon door (GAME-NIGHT). |
| `level_gate` (model `gate_zoo`) | Zootor † | level gate | Big double gate across every `[[entry]]` between two levels: closed and solid behind the story barrier while the next level is locked, open for good after unlocking (GAME-LAYOUT "Gates between the levels", LAYOUT-036). |
| `moon_door` | Mondtor | moon door | Gate of the day zoo that opens at nightfall and leads to the night zoo. |
| `lantern` | Laterne | lantern | Light the player carries at night; makes the animals' eyes shine. |
| `nocturnal_animal` | nachtaktives Tier | nocturnal animal | Animal that is active at night (hedgehog, bat, owl, …). |
| `daytime` (phases `day`, `dusk`, `night`, `sleeping`, `morning`) | Tageszeit † | time of day | Saved day/night state of the zoo (`zoo_core::daytime`, GAME-NIGHT "Implementation", GAME-SAVE). Not "level". |
| `night_level` (data `[level] time = "night"`, id `night_<N>`) | Nachtlevel † | night level | A level of the night zoo, reached only through a moon door (GAME-LAYOUT "Moon door and night levels", GAME-LEVEL-NIGHT-1). |
| `night_house` / `indoor_enclosure` (data `indoor = true`) | Nachthaus / Innengehege † | night house / indoor enclosure | Enterable building of a night level whose dim indoor enclosures open into its visitor hall (GAME-LEVEL-NIGHT-1, Q-134 answered). |
| `terrarium` (data `terrarium = true` on an `indoor` enclosure) | Terrarium | terrarium | Glass-fronted indoor enclosure of the terrarium house (snake, chameleon, poison dart frog); never "cage" (GAME-LEVEL-NIGHT-2). |
| `terrarium_house` | Terrarienhaus | terrarium house | Enterable warm-lit building of `night_2` with the three terrariums along its north wall (GAME-LEVEL-NIGHT-2). |
| `snake` | Schlange | snake | Friendly green-yellow corn snake; basic fish, treat eggs; baby `snake_hatchling` (night_2). |
| `chameleon` | Chamäleon | chameleon | Colour-changing perched lizard; basic crickets, treat frozen insects; baby `chameleon_baby` (night_2). |
| `poison_dart_frog` | Pfeilgiftfrosch | poison dart frog | Tiny very colourful frog shown friendly ("giftig" only as a board fact); basic flies, treat crickets; baby `frog_froglet` (night_2). |
| `fish_bowl` | Goldfischglas | fish bowl | Big glass bowl the player carries, fills with water and uses to bring the goldfish home (GAME-RESCUE); found in the zookeeper house of level 3 (Q-093, confirmed 2026-10-01). |
| `water_source` | Wasserstelle † | water source | Place where the fish bowl can be filled: a tap or the bank of a stream, river, pond or fountain (Q-093, confirmed 2026-10-01). |
| `level_entry` (data `[[entry]]`) | Levelzugang † | level entry | Cells of a level directly behind a barrier of an earlier level; the only walkable border cells of a level (GAME-LAYOUT "Joining levels", proposal Q-088). |
| `lying_item` | abgelegter Gegenstand † | item lying on the ground | An item the child put down (a food or the fish bowl — never the basket or the honey pot, which have their own slot, Q-172); stays where it was put, saved, at most 8 in the zoo (GAME-FEED §8–11). |
| `bamboo_forest` (data `decoration` kind `bamboo`, `harvestable = true`) | Bambuswald | bamboo forest | Dense bamboo thicket; when harvestable, bamboo is cut there at its cut spots as a second source of the food `bamboo` (GAME-FEED §14–17). Also the panda's `kiga` place word (CONT-MISSIONS). |
| `cut_spot` (data `[[cut_spot]]`) | Schneidestelle † | cut spot | Place at the edge of a harvestable bamboo forest where a stalk is snapped off with the hands; regrows stump → young shoot → full stalk (GAME-FEED §14–15). |
| `pocket` | Tasche † | pocket | Second carry slot for one food while the player carries the fish bowl (GAME-RESCUE goldfish bowl, proposal Q-084). |
| `unlocked_level` / `locked_level` | freigeschaltetes / gesperrtes Level † | unlocked / locked level | A level of the joined zoo is unlocked when one of its `[[entry]]` barriers is open; a locked level is sealed, its animals are hidden and asleep (GAME-LAYOUT "Joining levels", proposal Q-088). |
| `enterable_building` (data `interior`, `door`) | begehbares Gebäude † | enterable building | Building whose interior and door cells are walkable floor; its roof and upper walls hide while the player is inside (GAME-PLAYER §2, proposal Q-092). |
| `bed` (data `[[item]] kind = "bed"`: `bed_l1`, `bed_l2`) | Bett | bed | Where the child sleeps at night (🛏, interact action) → next morning; at nightfall the nearest unlocked bed is offered (GAME-NIGHT rule 3, Q-141). |
| `static_batch` | — | static batch | Static placements that never move merged into one mesh per group (TECH-ARCH "Multi-node assets", ARCH-008). Not player-facing. |
| `part` (model part) | — | part | Direct mesh child node of a model's root that the renderer can move or hide (`leaf_l`, `door`, `roof`, `sails`, …); part 0 = the root (TECH-ARCH, ARCH-006). Not player-facing. |
| `material_slot` (`*_glow`, `eye_glow`, `glass`, `*_face`) | — | material slot | Named glTF material with a fixed rendering meaning (emission, eyeshine, transparent glass, text decal face) (TECH-ARCH, ARCH-006/007). Not player-facing. |
| `render_region` | — | render region | Group of static batches culled together: one per level, per barrier, per enterable-building roof (GAME-LAYOUT "Joining levels"). Not player-facing. |
| `perch` (data `perch_height_m`) | Sitzplatz oben † | perch | Raised spot where an escaped animal sits instead of wandering on the ground (koala in a tree, monkey in the crow's nest; proposal Q-094). |
| `view_mode` (`zoo`, `look_around`, `first_person`) | Ansicht (Zoo-Ansicht, Umschauen, Ich-Ansicht) † | camera view (zoo view, look-around, first person) | The camera view: the high-angle zoo view (default, GAME-PLAYER §2), the look-around view (held with `F` / right mouse, persistent on touch) and the first-person view (toggle); the one view button cycles them — GAME-CAMERA-VIEWS. |
| `fog_end` (`FOG_END_M`) | Sichtweite † | visibility distance | Distance from the eye beyond which the comic haze hides everything in the close views (20.8 m since the user's +30 % of 2026-09-27, FIX-056; was 16 m, Q-109); hiding places must lie ≥ 22 m from their own board and gate (CAMV-008, GAME-LAYOUT "Sight"). |
| `golf_cart` | Golfwagen | golf cart | Small zoo vehicle the player can drive; animals do not follow it (GAME-CART). |
| `flow` (data key) | Fließrichtung † | flow | Direction a `river` / `stream` element flows in level coordinates (`N`/`E`/`S`/`W`); bridges inherit it; the river pieces chain along it (GAME-LAYOUT "Flowing water"). Not player-facing. |
| `scenery` (data `[[scenery]]`) | Kulisse † | scenery | Non-solid ground dressing a riddle relies on (tall grass, sand, mud, tree shade, leaf pile); `props` may list ambient animals such as `butterfly` (GAME-LAYOUT). |
| `ambient_animal` (`duck`, `duckling`, `frog`, `butterfly`) | Umgebungstier † | ambient animal | Small decorative animal with behaviour but no mission, food or collision; never saved (GAME-AMBIENT). Not an `animal` in the sense above. |
| `pair` (data `pair = true` on an enclosure) | Paar † | pair | Male + female of one species that hide together, follow as one group and enter their enclosure together (GAME-FAMILY; decision 2026-10-01: every species is a pair). |
| `baby` (`game.babies`) | Baby (Fohlen, Joey, …) † | baby | Young animal that appears when a pair at home is given a liked treat; always stays with the female, never a mission requirement (GAME-FAMILY). |
| `treat` | Leckerli | treat | The special food of a species that makes a pair at home happy **and** gives it a baby (once): a garden treat (carrot, potato, apple, orange; basket) or a box food that is not its basic food (GAME-FEED "Basic food and treats", proposal 2026-10-03). Not a `food` of the rescue mission; the same food can be basic for one species and a treat for another. |
| `basic_food` | Grundfutter | basic food | The box food that makes an escaped animal follow in the mission and gives hearts at home (no baby); shown on the info board together with the treat (GAME-FEED "Basic food and treats"). |
| `fruit_garden` (`garden_fruit`) | Obstgarten | fruit garden | Fenced garden of level 3 with two apple and two orange trees; fruit is a treat (GAME-GARDEN "Fruit garden", Q-320). |
| `basket` | Korb † | basket | Carry slot for harvested treats, separate from the food hands (GAME-GARDEN, Q-172). |
| `feeding_spot` (data `feed_spot`) | Futterplatz † | feeding spot | Cells just inside an enclosure's fence on the gate side where the pair and the baby gather when the child stands outside with a liked treat (GAME-GARDEN rule 6a). |
| `welcome_board` (data `kind = "map_board"`) | Willkommenstafel † | welcome board | The big board at the entrance of a level: game description, goal of the level, how to play (GAME-RESCUE "Welcome board"). Not an `info_board`. |
| `intro` | Einführung † | intro | The 3 pages shown at the entrance gate at the first start of a new game; replayable from the settings (GAME-RESCUE "Intro"). |
| `never_stuck` | Nie festgefahren † | never stuck | Rule set that guarantees the child always has a next step: hint priority, `help` hint after 120 s, animals come to the player after 180 s (GAME-HINT, GAME-LAYOUT "Level design rules" 15). Not player-facing. |
| `ad_board` / `campaign` / `parental_gate` | Werbetafel / Kampagne / Elterngate † | ad board / campaign / parental gate | In-world billboard; the signed external content of one own cross-promotion shown on several boards; the plus/minus (or language) task + 3 s hold before a link opens (GAME-ADS). |
| `fullscreen` / `installable` | Vollbild / installierbar | full screen / installable | Playing without browser bars: the settings button (`#fullscreen-toggle`) or the web app added to the home screen (manifest, TECH-PLATFORMS "Installable and full screen"). |

† German term is a proposal until confirmed (Q-045).
