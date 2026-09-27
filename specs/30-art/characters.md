---
id: ART-CHARACTERS
title: Characters — concept and models
aspect: art
module: characters
status: draft
depends_on: [ART-PIPELINE, ART-DIRECTION, ART-RIG, GAME-PLAYER]
test_prefix: ACHAR
updated: 2026-09-27
---

# Characters — concept and models

## Goal

Define every human character that needs a turnaround sheet and a 3D model, and what each
one looks like. Skeleton, clips, sockets, expressions and export rules are in ART-RIG.

## Asset list

| Asset id | Role | Turnaround | Animations (ART-RIG §4) | Status |
|---|---|---|---|---|
| `player_girl` | Player character (girl) | required | `idle`, `walk`, `run`, `pick_up`, `give`, `talk`, `cheer`, `wave`, `carry` | model v1 (review) |
| `player_boy` | Player character (boy) | required | same as `player_girl` (shared rig, identical clips) | concept |
| `visitor_*` | Visitors giving hints (see GAME-QUESTS) | required per visitor type | `idle`, `talk`, `point` | concept |
| `pirate` | Pirate on the pirate ship (if an NPC — Q-015) | required | `idle`, `talk` | blocked |

## Behaviour

1. All human characters use the shared `human` skeleton of ART-RIG §2. `player_girl` and
   `player_boy` additionally share the exact rest pose, so their clips are identical.
2. Player characters look ~6–8 years old in the **comic style** of `art/style/style.md`
   (Q-010 answered): rounded chunky shapes, large round head (head-to-body ≈ 1 : 3.5),
   big expressive hand-drawn eyes, flat colours with one hard shadow tone (cel shading),
   bold dark-brown outlines drawn by the renderer (ART-RIG §3.5). Height 1.20 m to the top
   of the skull. The girl's design (hair, clothes, colours) is taken from
   the approved turnaround `art/characters/player_girl/front.png` (and views).
3. `player_girl` and `player_boy` are clearly distinguishable **at a glance and from
   behind** (hair length and outfit colour), because the child chooses by tapping a
   preview without reading (GAME-PLAYER §1).
4. **Readable from the high game camera** (≈ 55° from above, Q-049), where a character is
   small on screen: the top of the head (hair shape and colour) and the shoulders/shirt
   are the main identifiers, so hair and shirt use large, strongly contrasting colour
   areas (no fine patterns that turn to noise at small size — stripes are few and
   broad), and the silhouette from above differs (girl: long hair covering the upper
   back; boy: short hair with tuft, bare knees, yellow shirt). Both must stand out
   against grass green, sand paths and wood brown.
5. Player characters wear **no hat or headwear**, so they never look like a visitor that a
   hint describes ("the lady with the red hat").
6. Visitors differ clearly by silhouette and colour (hat, glasses, stroller, balloon…) so a
   child can be told "ask the lady with the red hat".
7. The concept folder of each character contains `brief.md` and the full turnaround (see
   ART-PIPELINE §3), plus `expressions.png` with the 8 hand-drawn comic expressions of ART-RIG §6. Every
   prompt in a `brief.md` contains the CHARACTER SHEET STYLE block and its negative prompt
   the NEGATIVE suffix of `art/style/style.md` verbatim (APIPE-010).
8. All colours come from the shared palette (`art/palette.png`, ADIR-002). The
   hex values below are proposals until that palette exists.

## Character details

### `player_girl`

As in the approved turnaround `art/characters/player_girl/front.png` and its views.

| Part | Design |
|---|---|
| Hair | Long, straight, dark brown, falls behind the shoulders as one chunky rounded mass; soft straight fringe, slightly side-parted; ends flick slightly outwards. Rigid on `head`, no hair bones. The approved sheet (sheet_v1) shows it to the waist; model v1 follows the sheet (hair mass sits behind the back so it need not bend) — Q-062. |
| Face | Comic face decal: big round dark-brown eyes with a white highlight, short brows in hair colour, small rose smile, light rosy cheeks, light peach skin; small rounded nose modelled. |
| Top | White short-sleeve T-shirt with 3 broad horizontal blue stripes (body; one on each sleeve), sleeves end at mid upper arm. |
| Belt | Brown belt at the hips. |
| Legs | Blue denim trousers to the ankles, slightly darker cuff. |
| Feet | White socks visible at the ankle, dark brown shoes with a white strap/sole. |
| Personality | Curious and calm; walks upright, looks around. |

Colours (proposal): skin `#F2C29B` / shade `#D9A27E`, hair `#4A2A17` / highlight
`#6B3E22`, eyes `#2B1B12`, mouth `#C8645A`, shirt `#F4F4F0`, stripes `#2F5DA8`, belt
`#6B3A1E`, trousers `#3559A0` / cuff `#2A4780`, socks `#FFFFFF`, shoes `#3B2314`, sole
`#F4F4F0`.

**Model v1** (`tools/blender/characters/player_girl.py`): 2 156 triangles, one skinned mesh
with primitives `body` (64 × 64 flat-colour atlas, 8 × 8 cells of 8 px, NEAREST,
`assets/textures/characters/player_girl_body.png`) and `face` (256 × 128 decal atlas,
`player_girl_face.png`, alpha MASK); all 9 player clips; preview
`art/characters/player_girl/model_preview.png`. The face atlas is a procedurally drawn
placeholder until the hand-drawn `expressions.png` exists.

### `player_boy`

Matching counterpart: same body, same proportions, same comic face style; differs in
hair, outfit colours and trouser length.

| Part | Design |
|---|---|
| Hair | Short chestnut-brown hair with a big rounded tuft sticking up at the front and short sides; ears visible. Rigid on `head`. |
| Face | Same comic face style as `player_girl` (big round eyes, highlight, brows in hair colour, small grin, rosy cheeks), light peach skin. |
| Top | Mustard-yellow short-sleeve T-shirt with one broad green horizontal band across the chest (and sleeve edges), untucked. |
| Legs | Navy knee-length shorts; knees visible (skin). |
| Feet | White socks to mid-calf with a green stripe, green sneakers with white soles. |
| Personality | Energetic and cheerful; slightly bouncier in previews (same clips — expression only). |

Colours (proposal): skin `#F2C29B` / shade `#D9A27E`, hair `#8A4B22` / highlight
`#A8612F`, eyes `#2B1B12`, mouth `#C8645A`, shirt `#F2B632`, band `#3F8F3A`, shorts
`#2E3F6E`, socks `#FFFFFF` + stripe `#3F8F3A`, sneakers `#3F8F3A`, soles `#F4F4F0`.

Outfits assume the player is a child visiting the zoo; if Q-001 decides "junior
zookeeper", both get a zoo vest over the same base design.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ACHAR-001 | Given `player_girl.glb` and `player_boy.glb`, then both have identical bone names and hierarchy (see RIG-001, RIG-004). | asset |
| ACHAR-002 | Given each character in the list, then its `.glb` contains exactly the listed animations (see RIG-007). | asset |
| ACHAR-003 | Given two visitor types, then their concept `palette` main colours differ (manual review). | manual |
| ACHAR-004 | Given the asset list above and `assets/manifest.toml`, then the `animations` of every character entry are equal. | asset |
| ACHAR-005 | Given the character choice screen, when five children aged 4–6 are asked "which one is the girl / the boy", then each child picks correctly from the front and from the back view. | manual |
| ACHAR-006 | Given the player turnarounds, then neither player character wears headwear. | manual |
| ACHAR-007 | Given both player models rendered in-game from the high camera (≈ 55°, default zoom) at ≤ 80 px character height on a 1080×2340 screen over grass, path and wood backgrounds, then reviewers identify girl vs. boy correctly in every shot and the stripes/band do not flicker or turn to noise. | manual |
| ACHAR-008 | *Retired — duplicate of APIPE-010 (covers every `brief.md` incl. `art/characters/`).* | — |

## Open questions

- Q-001 Player role/story (influences clothing: visitor vs. zookeeper uniform).
- Q-009 Who creates the turnaround images.
- Q-010 Art style — answered: comic (`art/style/style.md`).
- Q-049 answered: high-angle game camera (readability from above, §4).
- Q-015 Is the pirate an NPC, or is the pirate ship empty?
- Q-016 How many visitor types are needed?
- Q-026 Texture approach for body and faces.
- Q-027 Visitor proportions and whether visitors walk.
- Q-028 Choice of skin tone / hair beyond girl and boy.
- Q-051 Close-up for dialogue so faces/expressions are visible.
- Q-062 `player_girl` hair length (sheet: waist; old text: shoulder blades).
- Q-112 Hands in first person (GAME-CAMERA-VIEWS 3): only the carried items are drawn for now; first-person hands would need an extra model/rig.
