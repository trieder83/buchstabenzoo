---
id: GAME-TELESCOPE
title: Telescope - looking at the planets
aspect: gameplay
module: telescope
status: draft
depends_on: [GAME-PLAYER, GAME-NIGHT, GAME-LAYOUT, CONT-L10N]
test_prefix: TELE
updated: 2026-10-04
---

# Telescope - looking at the planets

## Goal

At night the child can look through the toy telescope of the night zoo (`telescope_n1`,
Q-138: "interactive later as a small bonus") and sees the **eight planets of our solar
system**. Tapping a planet shows a small **info box**: its name, its type (rocky planet, gas
giant, ice giant) and ONE short, factually correct, easy-to-remember sentence. A calm bonus:
no points, no timer, no mission progress, nothing is saved.

## Behaviour

1. **Where and when.** The `telescope` element is an interactable (target kind `telescope`,
   interact button icon 🔭) **only at night** ("only at night the stars are out", Q-365) and
   only while its level part is unlocked. It uses the ordinary interaction rules (range
   2 m, facing); there is no readable side. By day it stays decoration and offers nothing.
2. **Telescope view.** Interacting opens a full-screen overlay (`#telescope-view`): a dark
   night sky with a few slowly twinkling stars (still when the system asks for reduced
   motion), a rounded **eyepiece frame** (dark vignette, brass rim) and the **eight planets
   in order from the Sun** (Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune),
   each a big round **tap target** (`.tele-planet`, >= 64 px) drawn as a comic disc (inline SVG: flat colours, one hard shadow tone plus soft shading, rim
     light, a soft glow halo; gentle drift, still for reduced motion; no per-frame JS):
   - Mercury grey with craters, Venus yellowish, Earth blue-green with white clouds, Mars
     red, Jupiter orange-white bands, **Saturn pale gold with a ring** (wider than the disc),
     Uranus pale cyan, Neptune deep blue.
   - Details: Mercury cratered; Venus swirly clouds; Earth with continents, clouds and a thin
     atmosphere glow; Mars with polar cap and dark patches; Jupiter turbulent bands and the Great
     Red Spot; Saturn a tilted ring with gaps, passing in front of and behind the disc; Uranus a
     faint tilted ring; Neptune a white storm.
   - Sizes are only **hinted** (gas / ice giants a bit larger than the rocky planets), not
     to scale, so that all eight stay tappable (Q-366).
3. **Layout.** In the eyepiece sky the **Sun** is big and only partly visible in the
   **lower-left corner** (a glowing comic disc whose centre lies at the corner; decoration
   behind the planets, not a tap target, `.tele-sun`). The eight planets sit **in order
   Mercury -> Neptune on one straight line from near the Sun to the upper-right corner**
   (`.tele-planet[data-planet]`; x strictly increasing, y strictly decreasing, evenly spaced;
   a faint dashed orbit line may show the path). Every tap box is >= 64 px and boxes never
   overlap each other, the info box or the ✖ button at 780x360 and 360x780: consecutive boxes
   differ by >= 64 px in x or in y, so the sky uses the full height in portrait and the info
   column is narrow in landscape (`#telescope-info` beside, or below, the sky). The discs
   themselves may be smaller than their box (the box is transparent); Saturn's ring may reach
   beyond its box. No scrolling or zooming.
4. **Info box.** Tapping a planet marks it (`.selected`) and fills the box
   (`#telescope-info`): the planet name, its type as a line with an icon (🪨 rocky planet,
   ☁️ gas giant, 🧊 ice giant) and the one sentence of the child's **reading level**
   (`kiga`: a very short sentence; `klasse1`..`klasse3`: longer, see "Content"). Tapping
   another planet replaces the box content. Before the first tap the box shows a friendly
   hint with 👆 (icon first, one short line).
5. **Close.** The ✖ button (`#telescope-close`, >= 64 px, in a corner), `Esc`, and tapping
   the interact key again close it. The game is paused while the view is open (like the map,
   MAP-006): movement input and the simulation do nothing; held keys and the stick are
   released on open. Works with touch and mouse; the game can never get stuck (closing is
   always possible).
6. **No tracking, no network, no links.** The view needs no server and sends nothing.
7. **Localisation.** Every string is a Fluent key (de first, en in sync); names and
   sentences follow the language at once. Keys: `telescope-title`, `telescope-close`,
   `telescope-hint`, `telescope-kind-<rocky|gas|ice>`, `telescope-<planet>-name`,
   `telescope-<planet>-<kiga|klasse1|klasse2|klasse3>`.

## Planet data (`zoo-core::telescope`)

A fixed table in `zoo-core` (pure data, no web deps): `id`, `kind` (rocky | gas | ice),
`order` (1..8 from the Sun), `size` (relative display size), visual parameters (colours,
`ring`, `bands`, `craters`/`clouds`) and the Fluent keys. The host reads it as JSON
(`telescope_json`) and draws it; it holds no game state.

| Planet | Order | Type | The one fact (all reading levels tell it) |
|---|---|---|---|
| Mercury | 1 | rocky | the smallest planet and the closest to the Sun |
| Venus | 2 | rocky | very hot and wrapped in thick clouds; it shines brightly in the sky |
| Earth | 3 | rocky | our home; the only planet we know with life; blue water |
| Mars | 4 | rocky | the red planet; the red comes from rusty dust |
| Jupiter | 5 | gas | the biggest planet; a giant storm, the Great Red Spot |
| Saturn | 6 | gas | the ring, made of countless pieces of ice and rock (not asteroids) |
| Uranus | 7 | ice | pale blue-green, rolls around the Sun tilted on its side |
| Neptune | 8 | ice | the farthest planet, deep blue, with very strong winds |

Facts must stay correct; a text change is reviewed against this table. Reading levels: `kiga`
at most 6 words, no subordinate clause; `klasse1` two sentences of at most 5 words (READ-002); `klasse2` one
sentence with a reason; `klasse3` one or two sentences with a number or comparison where it
helps. Only words that fit the level (CONT-L10N, reading levels).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| TELE-001 | Given the planet table, then it has eight planets in order 1..8 named Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune with unique ids. | unit |
| TELE-002 | Given the table, then the kinds are right: Mercury, Venus, Earth, Mars rocky; Jupiter, Saturn gas; Uranus, Neptune ice. | unit |
| TELE-003 | Given the de and en Fluent content, then every planet has a name and one sentence at each of the four reading levels, and every kind has a label, in both languages. | unit |
| TELE-004 | Given the sentences, then the kiga sentence has at most 6 words and each is non-empty and not longer than 250 characters (klasse2 / klasse3 are richer: 2–3 sentences with extra facts, user request 2026-10-04); only Saturn has `ring`. | unit |
| TELE-005 | Given the day, then the telescope offers no interaction; given the night with `night_1` unlocked and the player next to the telescope, then the available target is `telescope` and `interact` returns the telescope result. | unit |
| TELE-006 | Given the telescope view data (`telescope_json`), then it lists the eight planets with their visual parameters and keys. | unit |
| TELE-007 | Given the night and the player at the telescope, when she interacts, then `#telescope-view` is open with eight `.tele-planet` (each >= 64 px) and the game is paused (a held movement key does not move her); closing (✖, `Esc`) resumes it. | e2e |
| TELE-008 | Given the view is open, when Saturn is tapped, then `#telescope-info` shows its name, its type and the sentence of the current reading level (de, and en after switching the language). | e2e |
| TELE-009 | Given 780x360 and 360x780, then the view, the eight planets, the info box and the ✖ button lie inside the viewport and do not overlap. | e2e |
| TELE-010 | Given the day, then the interact button is not offered at the telescope. | e2e |
| TELE-011 | Given a sky size, then the line layout gives eight centres with x strictly increasing and y strictly decreasing, evenly spaced, inside the sky, and consecutive centres >= 64 px apart in x or y at the 780x360 and 360x780 sky sizes. | unit |
| TELE-012 | Given the view open at 780x360 and 360x780 (and 1280x720), then `.tele-sun` is at the lower left, and the bounding boxes of the planets in order Mercury..Neptune go from lower left to upper right (x increasing, y decreasing), with the first planet right of the Sun's centre. | e2e |

## Open questions

- Q-138 answered 2026-09-27: decoration first, interactive later (this spec).
- Q-365 telescope only at night (decided default); Q-366 sizes / Sun / Moon / dwarf planets; Q-367 night_2 has no telescope.
