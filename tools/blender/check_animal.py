#!/usr/bin/env python3
"""Check quadruped animal .glb files against ART-ANIMALS (rig conventions, clips, export rules).

Usage:  python3 tools/blender/check_animal.py [file.glb ...]
        (default: assets/models/animals/*.glb)

Pure Python 3.10+ (no Blender, no dependencies). Reuses the glTF/pose helpers of
check_character.py. Covers AANI-003..AANI-008:
  AANI-003 skeleton: the 23 `quadruped` joint names + parents (+ the animal's optional extra
           leaf joints of EXTRA, e.g. elephant trunk_1..3), <= 24 standard, hard max 32
  AANI-004 skin/mesh: one skinned mesh, material `body`, <= 4 influences, weights sum 1,
           JOINTS_0 UNSIGNED_BYTE, no morphs/vertex colours, smooth normals, texture
           <= 256 x 256, <= 3000 triangles, <= 400 KB
  AANI-005 rest pose: min Y ~ 0, back (withers) height per animal, faces +Z (head in front
           of the tail), left legs at +X, legs vertical, armature node identity
  AANI-006 clips: names == manifest `animations`, length == animal_anims.toml frames,
           root never animated, translation on `hips` only, no scale, loops seamless
  AANI-008 walk: planted hooves drift <= 0.03 m at the authored speed
Exit code 1 if any check fails.
"""

import glob
import math
import os
import re
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from check_character import (Model, manifest_animations, png_size, pose_local,  # noqa: E402
                             qangle, vlen, vsub)
from check_glb import has_extras  # noqa: E402

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
ANIMS_TOML = os.path.join(REPO, "assets", "models", "animals", "animal_anims.toml")
FPS = 30
MAX_TRIS = 3000
MAX_BYTES = 400 * 1024

# ART-ANIMALS rig conventions: quadruped skeleton
PARENTS = {"root": None, "hips": "root", "spine": "hips", "chest": "spine", "neck_1": "chest",
           "neck_2": "neck_1", "head": "neck_2", "ear_l": "head", "ear_r": "head",
           "tail_1": "hips", "tail_2": "tail_1"}
for _end, _par in (("front", "chest"), ("hind", "hips")):
    for _s in ("l", "r"):
        PARENTS[f"{_end}_upper_{_s}"] = _par
        PARENTS[f"{_end}_lower_{_s}"] = f"{_end}_upper_{_s}"
        PARENTS[f"{_end}_foot_{_s}"] = f"{_end}_lower_{_s}"
FEET = [f"{e}_foot_{s}" for e in ("front", "hind") for s in ("l", "r")]

# optional per-animal extra joints (leaf chains; ART-ANIMALS rig conventions §2)
EXTRA = {
    "elephant": {"trunk_1": "head", "trunk_2": "trunk_1", "trunk_3": "trunk_2"},
    "snow_fox": {"tail_3": "tail_2"},
}
MAX_JOINTS = 32

# per-animal rest checks: back (withers) height in m (+-0.05), max total height and
# (optional) game size = total height (+-0.08, ART-ANIMALS "Game sizes")
SIZES = {
    "zebra": {"back": 1.30, "max_height": 2.35},
    "hippo": {"back": 1.43, "max_height": 1.75, "height": 1.70},
    "panda": {"back": 0.84, "max_height": 1.15, "height": 1.10},
    "koala": {"back": 0.61, "max_height": 0.95, "height": 0.90},
    "elephant": {"back": 2.33, "max_height": 3.05, "height": 3.00},
    "giraffe": {"back": 2.30, "max_height": 4.55, "height": 4.50},
    "lion": {"back": 1.02, "max_height": 1.55, "height": 1.50},
    "snow_fox": {"back": 0.62, "max_height": 0.95, "height": 0.90},
}

# non-quadruped rigs (tools/blender/animals/biped_rig.py, fish_rig.py): skeleton, feet for
# AANI-008 and rest checks. biped: min Y = 0, total height +- 0.05; fish: origin = water
# surface above the body centre (hips `depth` m below y = 0), length along Z +- 0.03.
BIPED_PARENTS = {"root": None, "hips": "root", "spine": "hips", "chest": "spine",
                 "neck": "chest", "head": "neck", "tail_1": "hips"}
for _s in ("l", "r"):
    BIPED_PARENTS.update({f"upper_arm_{_s}": "chest", f"lower_arm_{_s}": f"upper_arm_{_s}",
                          f"hand_{_s}": f"lower_arm_{_s}", f"upper_leg_{_s}": "hips",
                          f"lower_leg_{_s}": f"upper_leg_{_s}", f"foot_{_s}": f"lower_leg_{_s}"})
for _i in range(2, 6):
    BIPED_PARENTS[f"tail_{_i}"] = f"tail_{_i - 1}"
FISH_PARENTS = {"root": None, "hips": "root", "head": "hips", "spine_1": "hips",
                "spine_2": "spine_1", "spine_3": "spine_2", "tail_fin": "spine_3",
                "fin_dorsal": "spine_1", "fin_pec_l": "hips", "fin_pec_r": "hips",
                "fin_pelvic_l": "hips", "fin_pelvic_r": "hips", "fin_anal": "spine_2"}
# ambient animals (GAME-AMBIENT; tools/blender/animals/bird_rig.py, frog.py): bird = duck
# rig, origin at the water surface (waterline) under the body; frog = origin at the feet.
BIRD_PARENTS = {"root": None, "hips": "root", "spine": "hips", "chest": "spine",
                "neck_1": "chest", "neck_2": "neck_1", "head": "neck_2", "tail": "hips"}
FROG_PARENTS = {"root": None, "hips": "root", "spine": "hips", "head": "spine",
                "throat": "head"}
for _s in ("l", "r"):
    BIRD_PARENTS.update({f"wing_upper_{_s}": "chest", f"wing_lower_{_s}": f"wing_upper_{_s}",
                         f"leg_upper_{_s}": "hips", f"leg_lower_{_s}": f"leg_upper_{_s}"})
    FROG_PARENTS.update({f"arm_{_s}": "spine", f"leg_upper_{_s}": "hips",
                         f"leg_lower_{_s}": f"leg_upper_{_s}"})
OTHER_RIGS = {  # asset: (rig, parents, feet for AANI-008, rest expectations)
    "monkey": ("biped", BIPED_PARENTS, ["foot_l", "foot_r"], {"height": 1.10}),
    "goldfish": ("fish", FISH_PARENTS, [], {"length": 0.60, "depth": 0.22}),
    "duck": ("bird", BIRD_PARENTS, [], {"length": 0.45, "height": 0.34}),
    "duckling": ("bird", BIRD_PARENTS, [], {"length": 0.22, "height": 0.23}),
    "frog": ("frog", FROG_PARENTS, [], {"length": 0.18, "height": 0.17}),
}
# ambient animals are not in assets/manifest.toml (their concept is approved as part of
# kit_water); their expected clips come from here (== animal_anims.toml, kind = "ambient")
AMBIENT = {
    "duck": ["swim", "idle", "dip", "flap", "preen"],
    "duckling": ["swim", "idle", "dip", "flap", "preen"],
    "frog": ["idle", "croak", "hop", "swim"],
}
MAX_AMBIENT_TRIS = 1200


def check_other_rest(rig, exp, rp, mn, mx, err):
    """AANI-005 for the biped / fish / bird / frog rigs."""
    if rig in ("bird", "frog"):
        if any(abs(c) > 1e-4 for c in rp["root"]):
            err.append(f"AANI-005 {rig} root is not at the origin")
        if abs(mx[2] - mn[2] - exp["length"]) > 0.03:
            err.append(f"AANI-005 length {mx[2] - mn[2]:.3f} m (expected {exp['length']} +- 0.03)")
        if abs(mx[1] - exp["height"]) > 0.03:
            err.append(f"AANI-005 height {mx[1]:.3f} m (expected {exp['height']} +- 0.03)")
        if not rp["head"][2] > rp["hips"][2]:
            err.append("AANI-005 not facing +Z")
        if not rp["leg_upper_l"][0] > 0 > rp["leg_upper_r"][0]:
            err.append("AANI-005 leg_upper_l is not at +X")
        if rig == "bird":
            # waterline origin: body floats (hips above the water), only legs/feet deep below
            if not 0 < rp["hips"][1] < 0.5 * mx[1]:
                err.append(f"AANI-005 bird hips y {rp['hips'][1]:.3f} not just above the water")
            if mn[1] < -0.25 * mx[1]:
                err.append(f"AANI-005 bird reaches {mn[1]:.3f} m below the waterline")
            if not rp["head"][2] > 0 > rp["tail"][2]:
                err.append("AANI-005 bird head / tail not at +Z / -Z")
        elif abs(mn[1]) > 0.01:
            err.append(f"AANI-005 frog min Y = {mn[1]:.4f} (expected 0: origin at the feet)")
        return
    if rig == "biped":
        if abs(mn[1]) > 0.01:
            err.append(f"AANI-005 min Y = {mn[1]:.4f} (expected 0)")
        if abs(mx[1] - exp["height"]) > 0.05:
            err.append(f"AANI-005 height {mx[1]:.3f} m (game size {exp['height']} +- 0.05)")
        if not rp["tail_3"][2] < -0.1 < 0 <= rp["head"][2] + 0.05:
            err.append("AANI-005 not facing +Z (tail must be at -Z)")
        for a in ("upper_arm", "upper_leg"):
            if not rp[f"{a}_l"][0] > 0 > rp[f"{a}_r"][0]:
                err.append(f"AANI-005 {a}_l is not at +X")
        for s in ("l", "r"):
            d = vsub(rp[f"foot_{s}"], rp[f"upper_leg_{s}"])
            tilt = math.degrees(math.atan2(math.hypot(d[0], d[2]), -d[1]))
            if tilt > 2:
                err.append(f"AANI-005 leg {s} tilted {tilt:.1f} deg in rest pose")
    else:
        if any(abs(c) > 1e-4 for c in rp["root"]):
            err.append("AANI-005 fish root is not at the origin (water surface)")
        if abs(rp["hips"][1] + exp["depth"]) > 0.01 or abs(rp["hips"][0]) > 1e-3:
            err.append(f"AANI-005 hips y {rp['hips'][1]:.3f} (expected -{exp['depth']} below the surface)")
        if abs(mx[2] - mn[2] - exp["length"]) > 0.03:
            err.append(f"AANI-005 length {mx[2] - mn[2]:.3f} m (expected {exp['length']} +- 0.03)")
        if mx[1] > 0.05:
            err.append(f"AANI-005 fish sticks {mx[1]:.3f} m out of the water in rest pose")
        if not rp["head"][2] > 0 > rp["tail_fin"][2]:
            err.append("AANI-005 not facing +Z")
        if not rp["fin_pec_l"][0] > 0 > rp["fin_pec_r"][0]:
            err.append("AANI-005 fin_pec_l is not at +X")


def load_anims(path=ANIMS_TOML):
    """Tiny parser for animal_anims.toml: {animal: {clip: {key: value}}}."""
    out = {}
    cur = None
    for line in open(path, encoding="utf-8"):
        line = line.split("#", 1)[0].strip()
        if not line:
            continue
        m = re.match(r"^\[(\w+)\.(\w+)\]$", line)
        if m:
            cur = out.setdefault(m.group(1), {}).setdefault(m.group(2), {})
            continue
        m = re.match(r"^(\w+)\s*=\s*(.+)$", line)
        if not m or cur is None:
            continue
        k, v = m.group(1), m.group(2).strip()
        if v in ("true", "false"):
            cur[k] = v == "true"
        elif v.startswith("{"):
            cur[k] = {a: int(b) for a, b in re.findall(r"(\w+)\s*=\s*(\d+)", v)}
        elif v.startswith("["):
            cur[k] = re.findall(r'"([^"]+)"', v)
        elif v.startswith('"'):
            cur[k] = v.strip('"')
        else:
            cur[k] = float(v) if "." in v else int(v)
    return out


def check(path, anims_data):
    err, info = [], []
    m = Model(path)
    js = m.js
    asset = os.path.splitext(os.path.basename(path))[0]
    size = os.path.getsize(path)

    if not js.get("asset", {}).get("version", "").startswith("2."):
        err.append("asset.version is not 2.x")
    used = " ".join(js.get("extensionsUsed", [])).lower()
    if "draco" in used or "meshopt" in used:
        err.append(f"compression extension used: {used}")
    if has_extras(js):
        err.append("extras / custom properties present")
    if not m.skin:
        return asset, ["no skin"], info

    # AANI-003 skeleton
    jn = [m.names[j] for j in m.joints]
    other = OTHER_RIGS.get(asset)
    if "front_upper_l" not in jn and asset not in SIZES and not other:
        info.append("not a quadruped rig (no front legs) — skipped")
        return asset, err, info
    parents = other[1] if other else dict(PARENTS, **EXTRA.get(asset, {}))
    if sorted(jn) != sorted(parents):
        err.append(f"AANI-003 joint names differ: extra {sorted(set(jn) - set(parents))}, "
                   f"missing {sorted(set(parents) - set(jn))}")
    for j in m.joints:
        n = m.names[j]
        p = m.parent.get(j)
        pn = m.names[p] if p is not None else None
        want = parents.get(n, "?")
        if want is None:
            if pn in parents:
                err.append(f"AANI-003 root has joint parent {pn}")
        elif pn != want:
            err.append(f"AANI-003 parent of {n} is {pn}, expected {want}")
    if len(set(jn) & set(PARENTS)) > 24 or len(jn) > MAX_JOINTS:
        err.append(f"AANI-003 {len(jn)} joints (> 24 standard or > {MAX_JOINTS} in total)")

    idx = {n: i for i, n in enumerate(m.names)}
    arm_node = m.parent.get(idx["root"]) if "root" in idx else None
    if arm_node is not None:
        n = m.nodes[arm_node]
        t, r, s = n.get("translation", [0, 0, 0]), n.get("rotation", [0, 0, 0, 1]), n.get("scale", [1, 1, 1])
        if any(abs(a - b) > 1e-5 for a, b in zip(t + r + s, [0, 0, 0, 0, 0, 0, 1, 1, 1, 1])) or "matrix" in n:
            err.append(f"AANI-005 armature node '{m.names[arm_node]}' has a transform")
    for i, n in enumerate(m.nodes):
        if "mesh" in n and any(k in n for k in ("translation", "rotation", "scale", "matrix")):
            err.append(f"mesh node '{m.names[i]}' has a transform (apply transforms)")
        if "scale" in n and i in m.joints and any(abs(c - 1) > 1e-4 for c in n["scale"]):
            err.append(f"AANI-004 joint {m.names[i]} has scale in bind pose")

    # AANI-005 rest pose: facing, sides, vertical legs
    rest_local = lambda j: m.rest_trs(j)  # noqa: E731
    rp = {m.names[j]: m.world(j, rest_local)[0] for j in m.joints}
    if other:
        pass  # checked after the mesh bounds (check_other_rest)
    elif not (rp["head"][2] > 0 > rp["tail_1"][2]):
        err.append(f"AANI-005 not facing +Z: head z {rp['head'][2]:.2f}, tail z {rp['tail_1'][2]:.2f}")
    for e in (("front", "hind") if not other else ()):
        if not (rp[f"{e}_upper_l"][0] > 0 > rp[f"{e}_upper_r"][0]):
            err.append(f"AANI-005 {e}_upper_l is not at +X")
        if rp[f"front_upper_l"][2] <= rp["hind_upper_l"][2]:
            err.append("AANI-005 front legs are not in front (+Z) of the hind legs")
        for s in ("l", "r"):
            d = vsub(rp[f"{e}_foot_{s}"], rp[f"{e}_upper_{s}"])
            tilt = math.degrees(math.atan2(math.hypot(d[0], d[2]), -d[1]))
            if tilt > 2:
                err.append(f"AANI-005 leg {e}_{s} tilted {tilt:.1f} deg in rest pose")

    # AANI-004 mesh / skin / material
    meshes = js.get("meshes", [])
    skinned = [n for n in m.nodes if "mesh" in n]
    if len(meshes) != 1 or len(skinned) != 1 or "skin" not in skinned[0]:
        err.append(f"AANI-004 expected one skinned mesh, got {len(meshes)} meshes")
    mats = [mt.get("name") for mt in js.get("materials", [])]
    if mats != ["body"]:
        err.append(f"AANI-004 materials {mats}, expected ['body']")
    body_joints = {jn.index(n) for n in ("hips", "spine", "chest") if n in jn}
    tris = 0
    mn, mx = [1e9] * 3, [-1e9] * 3
    back = 0.0
    for prim in meshes[0]["primitives"]:
        a = prim["attributes"]
        if prim.get("mode", 4) != 4:
            err.append("primitive not TRIANGLES")
        if any(k.startswith("COLOR_") for k in a):
            err.append("AANI-004 vertex colours present")
        if "JOINTS_1" in a or "WEIGHTS_1" in a:
            err.append("AANI-004 JOINTS_1/WEIGHTS_1 present")
        if prim.get("targets"):
            err.append("AANI-004 morph targets present")
        for req in ("POSITION", "NORMAL", "TEXCOORD_0", "JOINTS_0", "WEIGHTS_0"):
            if req not in a:
                err.append(f"AANI-004 missing {req}")
        if js["accessors"][a["JOINTS_0"]]["componentType"] != 5121:
            err.append("AANI-004 JOINTS_0 is not UNSIGNED_BYTE")
        pos, nrm = m.acc(a["POSITION"]), m.acc(a["NORMAL"])
        jts = m.acc(a["JOINTS_0"])
        wacc = js["accessors"][a["WEIGHTS_0"]]
        wscale = {5126: 1.0, 5121: 255.0, 5123: 65535.0}[wacc["componentType"]]
        wts = [tuple(x / wscale for x in w) for w in m.acc(a["WEIGHTS_0"])]
        tol = 0.001 if wscale == 1.0 else 2.0 / wscale
        bad = sum(1 for jv, wv in zip(jts, wts)
                  if sum(1 for w in wv if w > 0) > 4 or abs(sum(wv) - 1) > tol
                  or any(j >= len(jn) for j, w in zip(jv, wv) if w > 0))
        if bad:
            err.append(f"AANI-004 {bad} vertices with bad weights")
        tris += (js["accessors"][prim["indices"]]["count"] if "indices" in prim else len(pos)) // 3
        for p, jv, wv in zip(pos, jts, wts):
            for i in range(3):
                mn[i] = min(mn[i], p[i])
                mx[i] = max(mx[i], p[i])
            dom = max(zip(wv, jv))[1]
            if dom in body_joints:
                back = max(back, p[1])
        seen, split = {}, 0
        for p, n in zip(pos, nrm):
            k = tuple(round(c, 5) for c in p)
            if k in seen and max(abs(a_ - b_) for a_, b_ in zip(seen[k], n)) > 1e-3:
                split += 1
            seen.setdefault(k, n)
        if split:
            err.append(f"AANI-004 {split} vertices share a position with differing normals")
    for mt in js.get("materials", []):
        ti = mt.get("pbrMetallicRoughness", {}).get("baseColorTexture", {}).get("index")
        if ti is None:
            err.append(f"material {mt.get('name')} has no texture")
            continue
        wh = png_size(m, js["images"][js["textures"][ti]["source"]])
        if wh is None or wh[0] > 256 or wh[1] > 256:
            err.append(f"AANI-004 texture {wh} exceeds 256 x 256")
    if asset in AMBIENT and tris > MAX_AMBIENT_TRIS:
        err.append(f"GAME-AMBIENT {tris} triangles > {MAX_AMBIENT_TRIS}")
    if tris > MAX_TRIS:
        err.append(f"AANI-004 {tris} triangles > {MAX_TRIS}")
    if size > MAX_BYTES:
        err.append(f"AANI-004 file {size / 1024:.0f} KB > 400 KB")
    if other:
        check_other_rest(other[0], other[3], rp, mn, mx, err)
    elif abs(mn[1]) > 0.01:
        err.append(f"AANI-005 min Y = {mn[1]:.4f} (expected 0)")
    exp = SIZES.get(asset)
    if other:
        pass
    elif exp:
        if abs(back - exp["back"]) > 0.05:
            err.append(f"AANI-005 back height {back:.3f} m (expected {exp['back']} +- 0.05)")
        if mx[1] > exp["max_height"]:
            err.append(f"AANI-005 height {mx[1]:.3f} m > {exp['max_height']}")
        if "height" in exp and abs(mx[1] - exp["height"]) > 0.08:
            err.append(f"AANI-005 height {mx[1]:.3f} m (game size {exp['height']} +- 0.08)")
    else:
        err.append(f"AANI-005 no expected size for {asset} in SIZES")
    info.append(f"{tris} tris, {len(jn)} joints, {size / 1024:.0f} KB, back {back:.3f} m, "
                f"size x {mx[0] - mn[0]:.2f} y {mn[1]:.3f}..{mx[1]:.3f} z {mn[2]:.2f}..{mx[2]:.2f}")

    # AANI-006 clips
    anims = {a["name"]: a for a in js.get("animations", [])}
    want = manifest_animations(asset)
    if want is None and asset in AMBIENT:
        want = AMBIENT[asset]
        if any(r.get("kind") != "ambient" for r in anims_data.get(asset, {}).values()):
            err.append("animal_anims.toml: ambient clip rows need kind = \"ambient\"")
    if want is None:
        err.append("asset not in manifest")
        want = []
    if sorted(anims) != sorted(want):
        err.append(f"AANI-006 clips {sorted(anims)} != manifest {sorted(want)}")
    table = anims_data.get(asset, {})
    if sorted(table) != sorted(want):
        err.append(f"AANI-006 animal_anims.toml clips {sorted(table)} != manifest {sorted(want)}")
    for name, a in anims.items():
        cd = m.clip(a)
        for node, chans in cd.items():
            nn = m.names[node]
            if nn == "root":
                err.append(f"AANI-006 {name}: root is animated")
            if "scale" in chans:
                err.append(f"AANI-006 {name}: scale channel on {nn}")
            if "translation" in chans and nn != "hips":
                err.append(f"AANI-006 {name}: translation on {nn}")
            if node not in m.joints:
                err.append(f"AANI-006 {name}: animates non-joint {nn}")
        dur = max(tr[0][-1] for ch in cd.values() for tr in ch.values())
        row = table.get(name)
        if row is None:
            continue
        frames, loop = row["frames"], row["loop"]
        if abs(dur - frames / FPS) > 1 / FPS + 1e-6:
            err.append(f"AANI-006 {name}: {dur:.3f} s, expected {frames / FPS:.3f} s")
        for ev, f in row.get("events", {}).items():
            if f >= frames:
                err.append(f"AANI-006 {name}: event {ev} at frame {f} >= {frames}")
        if loop:
            p0, p1 = pose_local(m, cd, 0.0), pose_local(m, cd, frames / FPS)
            worst_r = max(qangle(p0(j)[1], p1(j)[1]) for j in m.joints)
            dt = vlen(vsub(p0(idx["hips"])[0], p1(idx["hips"])[0]))
            if worst_r > 0.5 or dt > 0.002:
                err.append(f"AANI-006 {name}: loop seam {worst_r:.2f} deg / {dt * 1000:.1f} mm")
        # AANI-008 planted hooves (locomotion clips carry a speed)
        if "speed" in row and (FEET if not other else other[2]):
            v = row["speed"]
            drift_all = []
            for foot in (other[2] if other else FEET):
                fj = idx[foot]
                loc = [m.world(fj, pose_local(m, cd, f / FPS))[0] for f in range(frames)]
                ymin = min(p[1] for p in loc)
                down = [p[1] <= ymin + 0.004 for p in loc]
                # contact frames form one cyclic run; unwrap it so a stance that spans the
                # loop seam is measured in one piece (frame f + frames is one stride later)
                start = next((f for f in range(frames) if down[f] and not down[f - 1]), 0)
                drift = 0.0
                run = []
                for i in range(frames):
                    f = (start + i) % frames
                    if not down[f]:
                        break
                    ft = start + i  # unwrapped frame number
                    p = loc[f]
                    run.append((p[0], p[2] + v * ft / FPS))  # the animal moves along +Z
                if run:
                    drift = max(math.hypot(a_[0] - b_[0], a_[1] - b_[1]) for a_ in run for b_ in run)
                drift_all.append(drift)
                if drift > 0.03:
                    err.append(f"AANI-008 {name}: {foot} drifts {drift:.3f} m while planted")
            info.append(f"{name}: planted-hoof drift {max(drift_all) * 1000:.1f} mm at {v} m/s")
    info.append("clips: " + ", ".join(
        f"{n} {max(tr[0][-1] for ch in m.clip(a).values() for tr in ch.values()) * FPS:.0f}f"
        f"{' loop' if table.get(n, {}).get('loop') else ''}" for n, a in sorted(anims.items())))
    return asset, err, info


def main(argv):
    files = argv or sorted(glob.glob(os.path.join(REPO, "assets", "models", "animals", "*.glb")))
    if not files:
        print("no animal .glb files found")
        return 1
    anims_data = load_anims()
    failed = 0
    for f in files:
        try:
            asset, err, info = check(f, anims_data)
        except Exception as e:  # noqa: BLE001
            print(f"{os.path.basename(f)}: ERROR {e!r}")
            failed += 1
            continue
        print(f"{asset}: {'OK' if not err else 'FAIL'}")
        for i in info:
            print(f"    {i}")
        for e in err:
            print(f"    - {e}")
        failed += bool(err)
    print(f"\n{len(files) - failed}/{len(files)} ok")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
