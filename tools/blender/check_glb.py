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
- size (X, Y, Z extents in metres) matches EXPECTED_SIZES within tolerance
Exit code 1 if any check fails.
"""

import glob
import json
import os
import struct
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

BUDGETS = {"props": 500, "characters": 3000, "animals": 3000}

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
    "fence_wood_corner": (1.18, 1.1, 1.18),
    "fence_wood_end": (0.25, 1.22, 0.25),
    "gate_wood": (1.8, 1.05, None),
    "hedge": (2.0, 3.0, 1.0),
    "hedge_corner": (1.5, 3.0, 1.5),
    "zoo_wall": (2.0, 2.5, 0.8),
    "zoo_wall_corner": (1.4, 2.5, 1.4),
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
    for node in js.get("nodes", []):
        for key in ("rotation", "scale", "matrix"):
            if key in node:
                errors.append(f"node '{node.get('name')}' has {key} (transforms not applied)")
        t = node.get("translation")
        if t and any(abs(c) > 1e-5 for c in t):
            errors.append(f"node '{node.get('name')}' has translation {t}")
    meshes = js.get("meshes", [])
    if len(meshes) != 1:
        errors.append(f"{len(meshes)} meshes (expected 1)")
    if len(js.get("materials", [])) != 1:
        errors.append(f"{len(js.get('materials', []))} materials (expected 1: palette)")
    tris = 0
    mn = [float("inf")] * 3
    mx = [float("-inf")] * 3
    for mesh in meshes:
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
                    mn[i] = min(mn[i], p[i])
                    mx[i] = max(mx[i], p[i])
    size = [mx[i] - mn[i] for i in range(3)]
    budget = BUDGETS.get(kind)
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
