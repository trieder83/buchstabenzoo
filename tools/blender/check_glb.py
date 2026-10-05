#!/usr/bin/env python3
"""Check exported .glb files against the ART-PIPELINE export rules (§9, §10).

Usage:  python3 tools/blender/check_glb.py [file.glb ...]   (default: assets/models/**/*.glb)

Pure Python (no Blender, no dependencies). Checks per file:
- valid GLB container, glTF 2.0, one mesh, triangles only
- triangle count within the budget of its kind (APIPE-005)
- min Y ~ 0 (+-0.01) measured on the vertex data (APIPE-004); glTF is Y-up by definition,
  and the height (Y extent) must match the expected size, which catches a Z-up export
- no transforms left on nodes (transforms applied), no Draco, no vertex colours,
  no extras / custom properties, NORMAL + TEXCOORD_0 present, one material (palette)
- multi-node assets (night / building / gate kits, tools/blender/props/night_lib.py): child
  nodes may carry a TRANSLATION (their pivot: hinge, hub, roof origin, `light` / `socket_*`
  empties) but never rotation or scale; root nodes carry nothing. Material slots: `palette`
  plus `*_glow` (palette texture + emissiveFactor), `glass` (alphaMode BLEND) and `*_face`
  (blank text faces). Sizes / min Y are measured in asset space (node translations applied).
- size (X, Y, Z extents in metres) matches EXPECTED_SIZES within tolerance
Exit code 1 if any check fails.
"""

import glob
import json
import os
import struct
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

BUDGETS = {"props": 500, "buildings": 4000, "characters": 3000, "animals": 3000}
# per-asset budgets above the kind budget (brief / README_night.md justifies each)
BUDGET_OVERRIDES = {"moon_door": 1500, "moon_door_open": 1500,
                    # kit_landmarks_play: level landmarks (<= 600, user brief 2026-10-03)
                    "ice_cream_kiosk": 600, "carousel": 600, "playground_slide": 600, "playground_swings": 600,
                    # kit_landmarks_l2: train (7 axle nodes + smoke) and blossom tree (user order 2026-10-04)
                    "zoo_train": 2200, "blossom_tree": 1800}

# expected extents (x, y = height, z) in metres; None = not checked. Tolerance 0.06 m.
EXPECTED_SIZES = {
    # kit_ground — 1 m tiles (see tools/blender/props/kit_ground.py)
    "path_tile_straight": (1.0, (0.05, 0.12), 1.0),
    "path_tile_curve": (1.0, (0.05, 0.12), 1.0),
    "path_tile_t": (1.0, (0.05, 0.12), 1.0),
    "path_tile_cross": (1.0, (0.05, 0.12), 1.0),
    "path_tile_end": (1.0, (0.05, 0.12), 1.0),
    "plaza_tile": (1.0, (0.05, 0.12), 1.0),
    "grass_tile": (1.0, (0.05, 0.16), 1.0),
    "sand_tile": (1.0, (0.05, 0.12), 1.0),
    "path_edge": (1.0, 0.12, 0.12),
    # kit_fences
    "fence_wood": (2.18, 1.1, 0.18),       # 2 m between post centres + one post width
    "fence_wood_1m": (1.18, 1.1, 0.18),    # 1 m variant (Q-057)
    "fence_wood_corner": (1.18, 1.1, 1.18),
    "fence_wood_end": (0.25, 1.22, 0.25),
    "gate_wood": (1.8, 1.05, None),
    "hedge": (2.0, 3.0, 1.0),
    "hedge_1m": (1.0, 3.0, 1.0),
    "hedge_corner": (1.5, 3.0, 1.5),
    "zoo_wall": (2.0, 2.5, 0.8),
    "zoo_wall_1m": (1.0, 2.5, 0.8),
    "zoo_wall_corner": (1.4, 2.5, 1.4),

    # kits 3-6 (merged from check_props_3_6.py)
    "enclosure_sign": (2.36, 1.95, 0.66),
    "info_board": (1.0, 1.49, 0.61),
    "map_board": (2.26, 2.43, 0.6),
    "food_box": (0.62, 0.6, 0.6),
    "food_box_stack": (1.29, 1.2, 0.73),
    "tree_round": ((3.0, 3.6), (4.7, 5.2), (2.7, 3.4)),
    "tree_grove": ((2.5, 3.0), (5.3, 6.2), (2.4, 3.0)),
    "tree_eucalyptus": ((2.8, 3.4), (6.6, 7.2), (1.6, 3.4)),
    "bush": ((1.1, 1.5), (0.9, 1.2), (1.0, 1.5)),
    "flower_bed": (2.0, (0.4, 0.6), 1.0),
    "rock": ((1.0, 1.6), (0.6, 1.0), (0.8, 1.2)),
    "bamboo": ((1.0, 1.8), (2.9, 3.1), (1.0, 1.8)),
    "reed": ((0.6, 1.1), (1.0, 1.3), (0.6, 1.1)),
    "grass_tuft": ((0.15, 0.4), (0.25, 0.4), (0.15, 0.4)),
    "water_river_straight": (1.0, (0.0, 0.01), 1.0),
    "water_river_bank": (1.0, (0.05, 0.16), 1.0),
    "water_river_curve": (1.0, (0.05, 0.16), 1.0),
    "water_river_inner": (1.0, (0.05, 0.16), 1.0),
    "water_pond": (1.0, (0.0, 0.01), 1.0),
    "water_pond_edge": (1.0, (0.05, 0.16), 1.0),
    "water_pond_corner": (1.0, (0.05, 0.16), 1.0),
    "bridge_wood": (3.4, 1.27, 2.61),
    "jetty_wood": (3.8, 0.7, 1.8),
    "lily_pad": (0.88, (0.05, 0.15), 0.71),
    "duck": (0.31, 0.34, 0.52),
    "frog": (0.39, 0.2, 0.4),
    "road_block": (2.1, 1.05, 0.83),
    "repair_sign": (0.73, 1.32, 0.21),
    "zookeeper_cart": (2.31, 1.0, 1.12),
    "traffic_cone": (0.4, 0.46, 0.4),
    "fallen_tree": (2.42, 1.58, 4.26),
    "gate_zoo_closed": (3.12, 2.6, 0.72),
    # night / bedroom / gates / landmarks / garden kits (tools/blender/props/README_night.md)
    "lantern_post": (0.36, 2.45, 0.88),
    "string_lights": (6.08, 2.85, 0.2),
    "string_post": (0.16, 2.85, 0.16),
    "wall_lamp": (0.3, 0.77, 0.47),
    "board_lamp": (0.18, 0.31, 0.55),
    "hand_lantern": (0.2, 0.39, 0.17),
    "moon_door": (3.54, 4.6, 1.04),
    "moon_door_open": (3.54, 4.6, 1.73),
    "bed": (2.0, 1.0, 1.0),
    "night_table": (0.5, 0.63, 0.42),
    "bedside_lamp": (0.25, 0.36, 0.26),
    "window_moon": (1.26, 1.04, 0.17),
    "rug_round": (1.4, (0.01, 0.03), 1.4),
    "toy_chest": (0.82, 0.85, 0.54),
    "desk": (1.2, 0.9, 0.6),
    "note_paper": (0.24, (0.0, 0.01), 0.32),
    "key_box": (0.38, 0.47, 0.2),
    "cart_key": (0.13, 0.16, (0.0, 0.03)),
    "garden_gate": (2.24, 1.12, 0.14),
    "glass_door": (1.95, 2.2, 0.14),
    "door_wood": (0.94, 2.1, 0.2),
    "turnstile": (1.15, 1.01, 0.58),
    "gate_zoo": (3.12, 2.6, 0.72),
    "windmill": ((1.8, 2.4), (3.8, 4.1), (1.8, 2.0)),
    "hollow_tree": ((3.6, 4.4), (6.0, 6.4), (2.6, 3.2)),
    "old_tree": ((4.4, 5.2), (6.0, 6.5), (3.2, 3.8)),
    "crooked_tree": ((2.0, 2.5), (3.0, 3.4), (1.2, 1.6)),
    "fir_tree": ((2.8, 3.2), (8.9, 9.1), (2.8, 3.2)),
    "rock_hill": ((2.8, 3.0), (1.9, 2.1), (2.8, 3.0)),
    "brush_pile": ((2.8, 3.1), (0.9, 1.1), (2.0, 2.6)),
    "mushroom_patch": ((1.8, 2.0), (0.3, 0.4), (1.6, 1.9)),
    "flower_pots": ((0.8, 0.9), (0.45, 0.55), (0.5, 0.6)),
    "potting_bench": (1.9, 1.34, 0.62),
    "telescope": ((0.85, 1.0), (1.5, 1.65), (0.9, 1.05)),
    # kit_landmarks_play (rects: kiosk 4 x 3, carousel 4 x 4, slide 2 x 3, swings 4 x 2)
    "ice_cream_kiosk": (4.0, (4.0, 4.2), (2.9, 3.0)),
    "carousel": (4.0, (4.0, 4.15), (4.0, 4.2)),
    "playground_slide": ((1.0, 1.2), (1.75, 1.9), (3.0, 3.1)),
    "playground_swings": (3.9, (2.2, 2.3), (1.55, 1.7)),
    # kit_landmarks_l2 (rects: train 8 x 2, blossom tree 2 x 2)
    "zoo_train": ((8.0, 8.2), (2.2, 2.45), (2.0, 2.2)),
    "blossom_tree": ((4.6, 5.1), (6.1, 6.4), (4.4, 4.9)),
    "garden_bed": (0.82, 0.25, 2.9),
    "carrot_plant_sprout": ((0.25, 0.35), (0.1, 0.16), (0.25, 0.35)),
    "carrot_plant_young": ((0.25, 0.35), (0.28, 0.36), (0.25, 0.35)),
    "carrot_plant_ripe": ((0.25, 0.35), (0.42, 0.5), (0.25, 0.35)),
    "potato_plant_sprout": ((0.3, 0.45), (0.08, 0.14), (0.3, 0.45)),
    "potato_plant_young": ((0.5, 0.7), (0.25, 0.35), (0.5, 0.7)),
    "potato_plant_ripe": ((0.6, 0.85), (0.4, 0.5), (0.6, 0.85)),
    "apple_tree_sprout": ((0.3, 0.45), (0.3, 0.4), (0.3, 0.5)),
    "apple_tree_young": ((0.7, 1.0), (1.1, 1.3), (0.6, 0.9)),
    "apple_tree_ripe": ((1.0, 1.25), (1.4, 1.65), (0.9, 1.15)),
    "orange_tree_sprout": ((0.3, 0.45), (0.3, 0.4), (0.3, 0.5)),
    "orange_tree_young": ((0.7, 1.0), (1.1, 1.3), (0.6, 0.9)),
    "orange_tree_ripe": ((1.0, 1.25), (1.4, 1.65), (0.9, 1.15)),
    "carrot": (0.32, 0.07, 0.08),
    "potato": (0.16, 0.06, 0.17),
    "basket": (0.4, 0.47, 0.42),
    "garden_fence": (2.08, 0.85, 0.09),
    "garden_fence_1m": (1.08, 0.85, 0.09),
    "wheelbarrow": (0.61, 0.73, 1.57),
    "watering_can": (0.26, 0.43, 0.53),
    "garden_sign": (0.5, 0.8, 0.17),
    # buildings (assets/models/buildings/, kit_buildings) — footprint = level rect (+ eaves)
    "zookeeper_house": (6.43, 4.37, 5.11),
    "food_storage": (8.04, 4.53, 6.71),
    "food_hut": (5.8, 3.62, 6.68),
    "entrance_arch": (6.16, 5.0, 1.76),
    "night_house": (17.6, 4.9, 13.6),
}
TOL = 0.06

COMPONENTS = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4}
CTYPES = {5120: "b", 5121: "B", 5122: "h", 5123: "H", 5125: "I", 5126: "f"}


def read_glb(path):
    data = open(path, "rb").read()
    magic, version, length = struct.unpack_from("<4sII", data, 0)
    if magic != b"glTF" or version != 2:
        raise ValueError("not a glTF 2.0 binary")
    off = 12
    js, binchunk = None, b""
    while off < length:
        clen, ctype = struct.unpack_from("<I4s", data, off)
        chunk = data[off + 8: off + 8 + clen]
        if ctype == b"JSON":
            js = json.loads(chunk)
        elif ctype == b"BIN\x00":
            binchunk = chunk
        off += 8 + clen
    return js, binchunk


def accessor_values(js, binchunk, idx):
    acc = js["accessors"][idx]
    view = js["bufferViews"][acc["bufferView"]]
    n = COMPONENTS[acc["type"]]
    fmt = CTYPES[acc["componentType"]]
    size = struct.calcsize(fmt)
    stride = view.get("byteStride", size * n)
    base = view.get("byteOffset", 0) + acc.get("byteOffset", 0)
    return [struct.unpack_from("<" + fmt * n, binchunk, base + i * stride) for i in range(acc["count"])]


def has_extras(obj):
    if isinstance(obj, dict):
        return "extras" in obj or any(has_extras(v) for v in obj.values())
    if isinstance(obj, list):
        return any(has_extras(v) for v in obj)
    return False


def check(path):
    errors = []
    name = os.path.splitext(os.path.basename(path))[0]
    kind = os.path.basename(os.path.dirname(path))
    js, binchunk = read_glb(path)
    if not js.get("asset", {}).get("version", "").startswith("2."):
        errors.append("asset.version is not 2.x")
    used = js.get("extensionsUsed", [])
    if any("draco" in e.lower() for e in used):
        errors.append("Draco compression used")
    if has_extras(js):
        errors.append("extras / custom properties present")
    nodes = js.get("nodes", [])
    children = {c for n in nodes for c in n.get("children", [])}
    for i, node in enumerate(nodes):
        for key in ("rotation", "scale", "matrix"):
            if key in node:
                errors.append(f"node '{node.get('name')}' has {key} (transforms not applied)")
        t = node.get("translation")
        if t and any(abs(c) > 1e-5 for c in t) and i not in children:
            errors.append(f"root node '{node.get('name')}' has translation {t}")
    # world offset of every node (translations only)
    offset = {}

    def walk(i, base):
        t = nodes[i].get("translation", [0, 0, 0])
        o = [base[k] + t[k] for k in range(3)]
        offset[i] = o
        for c in nodes[i].get("children", []):
            walk(c, o)

    for i in range(len(nodes)):
        if i not in children:
            walk(i, [0.0, 0.0, 0.0])
    meshes = js.get("meshes", [])
    mesh_nodes = [i for i, n in enumerate(nodes) if "mesh" in n]
    if not meshes:
        errors.append("no mesh")
    if len(meshes) > 1 and len({nodes[i].get("name") for i in mesh_nodes}) != len(mesh_nodes):
        errors.append("mesh nodes must have unique names")
    mats = js.get("materials", [])
    names = [m.get("name", "") for m in mats]
    if "palette" not in names and not (len(mats) == 1):
        errors.append(f"materials {names}: no 'palette'")
    for m in mats:
        n = m.get("name", "")
        if n == "palette" or len(mats) == 1:
            continue
        if n.endswith("_glow"):
            if "emissiveFactor" not in m:
                errors.append(f"glow slot '{n}' has no emissiveFactor")
        elif n == "glass":
            if m.get("alphaMode") != "BLEND":
                errors.append("glass slot is not alphaMode BLEND")
        elif not n.endswith("_face"):
            errors.append(f"material '{n}' is not palette / *_glow / glass / *_face")
    tris = 0
    mn = [float("inf")] * 3
    mx = [float("-inf")] * 3
    for ni in mesh_nodes or [None]:
        if ni is None:
            break
        mesh = meshes[nodes[ni]["mesh"]]
        off = offset.get(ni, [0.0, 0.0, 0.0])
        for prim in mesh["primitives"]:
            attrs = prim["attributes"]
            if prim.get("mode", 4) != 4:
                errors.append("primitive is not TRIANGLES")
            if any(a.startswith("COLOR_") for a in attrs):
                errors.append("vertex colours present")
            for req in ("POSITION", "NORMAL", "TEXCOORD_0"):
                if req not in attrs:
                    errors.append(f"missing {req}")
            if "indices" in prim:
                tris += js["accessors"][prim["indices"]]["count"] // 3
            else:
                tris += js["accessors"][attrs["POSITION"]]["count"] // 3
            for p in accessor_values(js, binchunk, attrs["POSITION"]):
                for i in range(3):
                    mn[i] = min(mn[i], p[i] + off[i])
                    mx[i] = max(mx[i], p[i] + off[i])
    size = [mx[i] - mn[i] for i in range(3)]
    budget = BUDGET_OVERRIDES.get(name, BUDGETS.get(kind))
    if budget is not None and tris > budget:
        errors.append(f"{tris} triangles > budget {budget} for {kind}")
    if abs(mn[1]) > 0.01:
        errors.append(f"min Y = {mn[1]:.4f} (expected 0 +- 0.01)")
    exp = EXPECTED_SIZES.get(name)
    if exp is None:
        errors.append("no expected size in EXPECTED_SIZES")
    else:
        for axis, want, got in zip("XYZ", exp, size):
            if want is None:
                continue
            lo, hi = (want, want) if not isinstance(want, tuple) else want
            if not (lo - TOL <= got <= hi + TOL):
                errors.append(f"size {axis} = {got:.3f} m, expected {want}")
    return name, kind, tris, size, mn[1], errors


def main(argv):
    files = argv or sorted(glob.glob(os.path.join(REPO, "assets", "models", "**", "*.glb"), recursive=True))
    # skinned models are checked by check_character.py / check_animal.py
    files = [f for f in files if argv or not any(os.sep + d + os.sep in f for d in ("characters", "animals"))]
    if not files:
        print("no .glb files found")
        return 1
    failed = 0
    print(f"{'asset':22s} {'kind':8s} {'tris':>5s}  {'size x * y(up) * z  (m)':26s} {'minY':>7s}  result")
    for f in files:
        try:
            name, kind, tris, size, miny, errors = check(f)
        except Exception as e:  # noqa: BLE001
            print(f"{os.path.basename(f):22s} ERROR {e}")
            failed += 1
            continue
        s = f"{size[0]:.2f} * {size[1]:.2f} * {size[2]:.2f}"
        print(f"{name:22s} {kind:8s} {tris:5d}  {s:26s} {miny:7.4f}  {'OK' if not errors else 'FAIL'}")
        for e in errors:
            print(f"    - {e}")
        failed += bool(errors)
    print(f"\n{len(files) - failed}/{len(files)} ok")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
