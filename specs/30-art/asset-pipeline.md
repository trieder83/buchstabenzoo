---
id: ART-PIPELINE
title: Asset pipeline — concept to game
aspect: art
module: asset-pipeline
status: draft
depends_on: [ART-DIRECTION]
test_prefix: APIPE
updated: 2026-09-26
---

# Asset pipeline — concept to game

## Goal

No 3D model is built before its look is agreed. Every character, animal and environment
area goes through concept → approval → Blender → glTF → game, and each stage is traceable.

## Behaviour

1. **Stages.** Every asset moves through these stages, in order:

   | # | Stage | Output | Location |
   |---|---|---|---|
   | 1 | Concept | Turnaround sheet (characters, animals) or mockup (environment), listed in `art/catalog.js`, reviewed on `art/index.html` | `art/<kind>/<asset_id>/` |
   | 2 | Approval | Entry in `assets/manifest.toml` with `concept_approved = true` | `assets/manifest.toml` |
   | 3 | Modelling | `.blend` file, built via Blender MCP | `assets/blender/<kind>/<asset_id>.blend` |
   | 4 | Export | `.glb` (glTF 2.0 binary) | `assets/models/<kind>/<asset_id>.glb` |
   | 5 | Integration | Referenced by game data, loads in the game | `crates/zoo-core` data |

   `<kind>` is one of `characters`, `animals`, `props`, `environment`.
2. **Gate.** Stage 3 must not start for an asset whose concept is not approved. The
   Blender MCP agent checks `assets/manifest.toml` before modelling.
3. **Turnaround sheet** (characters and animals): one image per view — `front.png`,
   `side.png` (left profile), `back.png`, `three_quarter.png` — same scale, same pose
   (neutral T- or A-pose for humans, standing pose for animals), plain background, and a
   `palette.png` or palette list with the main colours. Optional `expressions.png`.
4. **Environment mockup:** at least one `overview.png` (elevated ¾ view of the area) and one
   `player_view.png` (from the third-person camera height), plus a `layout.md` listing the
   props and enclosures visible, so the area can be built from parts.
5. **Manifest.** `assets/manifest.toml` lists every asset:
   ```toml
   [[asset]]
   id = "zebra"
   kind = "animals"
   spec = "ART-ANIMALS"
   concept_approved = false
   animations = ["idle", "walk", "eat"]
   ```
6. **Export rules:** 1 unit = 1 m, Y-up, origin at the feet (ground contact), transforms
   applied, animation names from the owning spec, no unused materials.
7. **Budgets** (mobile, per model): characters and animals ≤ 3 000 triangles; props ≤ 500;
   one texture atlas ≤ 512×512 or vertex colours only.

## Acceptance criteria

- Every `.glb` in `assets/models/` has an approved manifest entry and a complete concept
  folder.
- Every manifest entry with `concept_approved = true` has all required concept files.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| APIPE-001 | Given a `.glb` in `assets/models/`, then a manifest entry with the same `id` and `kind` exists with `concept_approved = true`. | asset |
| APIPE-002 | Given an approved character/animal, then `front.png`, `side.png`, `back.png`, `three_quarter.png` exist in its concept folder. | asset |
| APIPE-003 | Given an approved environment area, then `overview.png`, `player_view.png` and `layout.md` exist. | asset |
| APIPE-004 | Given a `.glb`, when loaded with the `gltf` crate, then it parses, has Y-up bounds with min Y ≈ 0 (±0.01) and all animations listed in the manifest. | asset |
| APIPE-005 | Given a `.glb`, then its triangle count is within the budget for its kind. | asset |
| APIPE-006 | Given a manifest entry, then its `spec` refers to an existing spec id. | asset |
| APIPE-007 | Given `art/catalog.js` and `assets/manifest.toml`, then an item has status `approved` in the catalog if and only if its manifest entry has `concept_approved = true`. | asset |
| APIPE-008 | Given every image path in `art/catalog.js` with `required: true` for an `in-review` or `approved` item, then the file exists. | asset |

Asset tests live in a Rust integration test (`cargo test -p zoo-assets` or equivalent) so
they run in CI with the other tests.

## Open questions

- Q-009 Who/what creates concept images.
- Q-010 Voxel vs. smooth low-poly style.
