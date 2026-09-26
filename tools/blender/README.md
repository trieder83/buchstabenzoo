# tools/blender — scripted 3D assets

ART-PIPELINE stage 3: the **Python script is the source of truth**. A kit script builds
every asset of its kit from code, exports one `.glb` per asset, saves the `.blend` and
renders a preview. Hand edits to a `.blend` are not allowed: change the script and rerun it.

```
tools/blender/
  palette.toml          colour table -> assets/textures/palette.png
  lib/zoo_blender.py    shared helpers (scene, palette, mesh parts, export, preview)
  props/kit_ground.py   Kit 1 — ground tiles
  props/kit_fences.py   Kit 2 — fences, gate, hedges, walls
  check_glb.py          export-rule checker for every .glb (plain python3)
  animals/quadruped_rig.py  shared quadruped skeleton, pattern-map atlas, gait/IK, clips
  animals/zebra.py      zebra (skinned, 6 clips) -> assets/models/animals/zebra.glb
  check_animal.py       rig/clip checker for animal .glb files (ART-ANIMALS AANI-003..008)
```

## Run

```bash
blender -b --factory-startup --python tools/blender/props/kit_ground.py            # + preview render
blender -b --factory-startup --python tools/blender/props/kit_fences.py -- --no-preview
python3 tools/blender/check_glb.py                  # all of assets/models/**/*.glb
python3 tools/blender/check_glb.py assets/models/props/hedge.glb
```

Tested with Blender 5.2 (`/snap/bin/blender`). A kit script writes:

| Output | Path |
|---|---|
| models | `assets/models/props/<asset>.glb` |
| source | `assets/blender/props/<kit>.blend` (objects laid out in a grid; each mesh is origin-centred, the object location is only layout) |
| preview | `art/props/<kit>/model_preview.png` (listed in `art/catalog.js`) |
| palette | `assets/textures/palette.png` (regenerated from `palette.toml` on every run) |

It prints triangle count and size per asset and exits non-zero if an asset is over budget.

## Palette approach (flat colours, one texture)

- All models share **one material** `palette` whose base colour is the palette atlas
  `assets/textures/palette.png`: 16 x 16 cells of 16 px (256 x 256), one flat colour per cell.
- `palette.toml` maps a colour name to a cell `index` (row-major from the top-left) and a hex
  value picked from the approved concept sheets and the style frame. Never renumber an index
  that is in use; add new colours in free cells. Unused cells are magenta.
- A face gets a colour by placing all its UVs on the **centre of that cell**
  (`zoo_blender.palette_uv`). Sampling is NEAREST (exported sampler), so no bleeding.
- The renderer should load `palette.png` once and bind it for all static props (every `.glb`
  also embeds the same small PNG, ~1 KB, so the files stay self-contained). One material for
  everything means props can be batched into few draw calls.
- No vertex colours, no per-model textures. Shading (2-tone cel) and outlines come from the
  renderer (`art/style/style.md`) — nothing is baked, no outline geometry.

## Modelling conventions

- 1 unit = 1 m. Scripts model in Blender Z-up; the exporter writes glTF **Y-up**
  (glTF = Blender (x, z, -y)).
  World axes (GAME-LAYOUT "Coordinate spaces", Q-056): east = +X = Blender +X, up = +Y =
  Blender +Z, **north = world -Z = Blender +Y** (Blender's own top view: north up, east
  right). Level coordinates (x east, z north) become world (x, 0, -z) through
  `zoo_core::coords::level_to_world` only. Models are **never mirrored**; they are
  oriented by rotation about +Y (a clockwise quarter turn seen from above = -90 deg).
- One mesh object per asset, named after the asset id; transforms applied (the checker
  rejects node translation/rotation/scale).
- **Origin at the ground:** min Y = 0 exactly. Pieces are centred on the origin in X/Z unless
  the kit doc says otherwise (corners: centre-line intersection; gate: hinge axis).
- Low-poly parts from `zoo_blender`: `box` (optionally bevelled), `slab` (polygon prism with a
  chamfered top — stones, tiles), `sweep` (profile along a mitred polyline — hedges, rails,
  wall caps), `knob`, `tuft`. Smooth normals with edges sharper than 50 deg kept hard.
- Faces pointing straight down are deleted (never seen from the high camera). Material is
  single-sided (`doubleSided: false`).
- Export (`export_glb`): GLB, Y-up, no Draco, no vertex colours, no extras/custom
  properties, no animations/cameras/lights, normals + one UV set, no tangents.
- Budget: props <= 500 triangles (ART-PIPELINE §10); ground tiles are kept far lower because
  there is one per walkable cell.

## Preview render

`zoo_blender.render_preview` renders from the game camera pitch (55 deg, orthographic; pass
`ortho=False` for a 35 deg vertical-FOV perspective), turned 45 deg to look north-west so
X-aligned pieces run lower-left to upper-right like on the concept sheets,
with a preview-only 2-tone toon material, one hard-shadow sun and Freestyle lines as a rough
stand-in for the in-game cel shader and outline pass. Preview objects/materials are added
after the `.blend` is saved and are never exported.

## Kit notes

- **kit_ground** uses **1 m tiles** (the level grid is 1 m and level-1 paths are 3 cells
  wide at odd coordinates, so 2 m tiles cannot cover them). Path tiles are autotiled from the
  4-neighbour mask; the mapping and rotations are documented at the top of `kit_ground.py`.
- **kit_fences**: straight pieces in **2 m and 1 m** (`fence_wood`/`fence_wood_1m`,
  `hedge`/`hedge_1m`, `zoo_wall`/`zoo_wall_1m`, Q-057); a straight run of L m gets 2 m pieces
  from its start and one 1 m piece at the end if L is odd (GAME-LAYOUT "Modular edges",
  `zoo_core::level::segment_run`). Corner pieces are L pieces with 1 m arms (east and
  north), `gate_wood` is only the leaf with its origin on the hinge axis (open = rotate
  about Y; there are no separate closed/open models). Placement rules are at the top of
  `kit_fences.py`.
