---
id: PROD-GLOSSARY
title: Glossary
aspect: product
module: glossary
status: draft
depends_on: []
test_prefix: GLOS
updated: 2026-09-26
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
| `food` | Futter | food | Item an animal eats (e.g. bamboo, hay). |
| `enclosure_item` | Gehegeausstattung | enclosure item | Non-food items an enclosure needs (pool, stone, logs, bridge). |
| `visitor` | Besucher | visitor | NPC the player can talk to; gives hints. |
| `quest` | Aufgabe | quest | A goal with steps, e.g. find the lost monkey baby. |
| `riddle` | Rätsel | riddle | Reading/logic puzzle, solved by reading a word, sentence or short story. |
| `reading_level` | Lesestufe | reading level | `kiga`, `klasse1`, `klasse2`, `klasse3` (grades 4–5: Q-035). Write "reading level", never just "level". |
| `hint` | Hinweis | hint | Indirect information given by a visitor. |
| `rescue_mission` (code: `mission`) | Rettung † | rescue mission | Bringing one escaped animal species back to its enclosure (GAME-RESCUE). The game goal is completing all of them. |
| `hiding_place` (ids `loc_*`) | Versteck † | hiding place | Location where an escaped animal waits until it is found (GAME-ANIMALS, CONT-MISSIONS). |
| `location_riddle` | Ortsrätsel † | location riddle | Riddle on the info board that describes an animal's hiding place without naming it (`klasse1`+). A kind of `riddle`. |
| `info_board` | Infotafel | info board | Board next to each enclosure sign with the animal's name, its location riddle and its food word. |
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

† German term is a proposal until confirmed (Q-045).
