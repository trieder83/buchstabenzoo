---
id: ART-CHARACTERS
title: Characters — concept and models
aspect: art
module: characters
status: draft
depends_on: [ART-PIPELINE, ART-DIRECTION, ART-RIG, GAME-PLAYER]
test_prefix: ACHAR
updated: 2026-09-26
---

# Characters — concept and models

## Goal

Define every human character that needs a turnaround sheet and a 3D model, and what each
one looks like. Skeleton, clips, sockets, expressions and export rules are in ART-RIG.

## Asset list

| Asset id | Role | Turnaround | Animations (ART-RIG §4) | Status |
|---|---|---|---|---|
| `player_girl` | Player character (girl) | required | `idle`, `walk`, `run`, `pick_up`, `give`, `talk`, `cheer`, `wave`, `carry` | concept |
| `player_boy` | Player character (boy) | required | same as `player_girl` (shared rig, identical clips) | concept |
| `visitor_*` | Visitors giving hints (see GAME-QUESTS) | required per visitor type | `idle`, `talk`, `point` | concept |
| `pirate` | Pirate on the pirate ship (if an NPC — Q-015) | required | `idle`, `talk` | blocked |

## Behaviour

1. All human characters use the shared `human` skeleton of ART-RIG §2. `player_girl` and
   `player_boy` additionally share the exact rest pose, so their clips are identical.
2. Player characters look ~6–8 years old, blocky/voxel-inspired as in
   `art/reference/ref-player-style.jpg`: large cube head (head-to-body ≈ 1 : 3.5), box torso, box
   limbs, pixel-art face, flat colour areas. Height 1.20 m to the top of the skull.
3. `player_girl` and `player_boy` are clearly distinguishable **at a glance and from
   behind** (hair length and outfit colour), because the child chooses by tapping a
   preview without reading (GAME-PLAYER §1).
4. Player characters wear **no hat or headwear**, so they never look like a visitor that a
   hint describes ("the lady with the red hat").
5. Visitors differ clearly by silhouette and colour (hat, glasses, stroller, balloon…) so a
   child can be told "ask the lady with the red hat".
6. The concept folder of each character contains `brief.md` and the full turnaround (see
   ART-PIPELINE §3), plus `expressions.png` with the 8 expression cells of ART-RIG §6.
7. All colours come from the shared palette (`art/palette.png`, ADIR-002). The
   hex values below are proposals until that palette exists.

## Character details

### `player_girl`

Kept close to the girl in `art/reference/ref-player-style.jpg`.

| Part | Design |
|---|---|
| Hair | Long, straight, dark brown, falls behind the shoulders to the upper back; blocky straight fringe, slightly side-parted; one block-step "curl" at the ends. Rigid on `head`. |
| Face | Pixel-art: 2 × 2 px dark brown eyes with a white highlight, short brows in hair colour, small rose mouth, light peach skin. |
| Top | White short-sleeve T-shirt with 4 horizontal blue stripes (body and sleeves), sleeves end at mid upper arm. |
| Belt | Brown belt at the hips. |
| Legs | Blue denim trousers to the ankles, slightly darker cuff. |
| Feet | White socks visible at the ankle, dark brown shoes with a white strap/sole. |
| Personality | Curious and calm; walks upright, looks around. |

Colours (proposal): skin `#F2C29B` / shade `#D9A27E`, hair `#4A2A17` / highlight
`#6B3E22`, eyes `#2B1B12`, mouth `#C8645A`, shirt `#F4F4F0`, stripes `#2F5DA8`, belt
`#6B3A1E`, trousers `#3559A0` / cuff `#2A4780`, socks `#FFFFFF`, shoes `#3B2314`, sole
`#F4F4F0`.

### `player_boy`

Matching counterpart: same body, same proportions, same pixel-face style; differs in
hair, outfit colours and trouser length.

| Part | Design |
|---|---|
| Hair | Short chestnut-brown hair with a blocky tuft sticking up at the front and short sides; ears visible. Rigid on `head`. |
| Face | Same eye/brow/mouth style as `player_girl`, eyes `#2B1B12`, brows in hair colour, light peach skin. |
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

## Open questions

- Q-001 Player role/story (influences clothing: visitor vs. zookeeper uniform).
- Q-009 Who creates the turnaround images.
- Q-010 Voxel vs. smooth style.
- Q-015 Is the pirate an NPC, or is the pirate ship empty?
- Q-016 How many visitor types are needed?
- Q-026 Texture approach for body and faces.
- Q-027 Visitor proportions and whether visitors walk.
- Q-028 Choice of skin tone / hair beyond girl and boy.
