#!/usr/bin/env python3
"""Check human character .glb files against ART-RIG (skeleton, skin, clips, export rules).

Usage:  python3 tools/blender/check_character.py [file.glb ...]
        (default: assets/models/characters/*.glb)
        --clips idle,walk   only require these clips (e.g. a PoC export); default: the
                            asset's `animations` list in assets/manifest.toml (RIG-007)

Pure Python 3.10+ (no Blender, no dependencies). Covers the asset-level test cases
RIG-001..008, RIG-010, RIG-011, RIG-018, RIG-021, RIG-022 plus the general export rules
(glTF 2.0, no Draco/meshopt, no extras, no vertex colours, min Y ~ 0, height 1.20 m).
tools/blender/check_glb.py is the prop checker; it does not understand skins.
Exit code 1 if any check fails.
"""

import glob
import math
import os
import struct
import re
import sys

try:
    import tomllib
except ImportError:  # Python < 3.11: tiny fallback parser for the manifest below
    tomllib = None

sys.dont_write_bytecode = True  # no __pycache__ next to the shared check_glb.py
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from check_glb import accessor_values, has_extras, read_glb  # noqa: E402

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
FPS = 30

# ART-RIG §2.1 hierarchy
PARENTS = {
    "root": None, "hips": "root", "spine": "hips", "chest": "spine", "neck": "chest",
    "head": "neck",
    "shoulder_l": "chest", "upper_arm_l": "shoulder_l", "lower_arm_l": "upper_arm_l",
    "hand_l": "lower_arm_l",
    "shoulder_r": "chest", "upper_arm_r": "shoulder_r", "lower_arm_r": "upper_arm_r",
    "hand_r": "lower_arm_r",
    "upper_leg_l": "hips", "lower_leg_l": "upper_leg_l", "foot_l": "lower_leg_l",
    "upper_leg_r": "hips", "lower_leg_r": "upper_leg_r", "foot_r": "lower_leg_r",
}
C = math.cos(math.radians(45))
# ART-RIG §2.4, glTF space (x = character's left for _l, y up, z forward)
REST = {
    "root": (0, 0, 0), "hips": (0, 0.50, 0), "spine": (0, 0.56, 0), "chest": (0, 0.70, 0),
    "neck": (0, 0.84, 0), "head": (0, 0.86, 0),
    "shoulder_l": (0.10, 0.80, 0), "upper_arm_l": (0.19, 0.80, 0),
    "lower_arm_l": (0.19 + 0.17 * C, 0.80 - 0.17 * C, 0),
    "hand_l": (0.19 + 0.34 * C, 0.80 - 0.34 * C, 0),
    "upper_leg_l": (0.075, 0.48, 0), "lower_leg_l": (0.075, 0.27, 0), "foot_l": (0.075, 0.07, 0),
}
for _k, _v in list(REST.items()):
    if _k.endswith("_l"):
        REST[_k[:-2] + "_r"] = (-_v[0], _v[1], _v[2])

# ART-RIG §4.3 player clip set: frames, loop
PLAYER_CLIPS = {
    "idle": (90, True), "walk": (24, True), "run": (16, True), "pick_up": (24, False),
    "give": (24, False), "talk": (60, True), "cheer": (45, False), "wave": (40, False),
    "carry": (30, True),
}
LOCOMOTION = {"walk": 1.4, "run": 3.0}  # authored speeds, m/s (RIG-010)
SOCKETS = {"socket_hand_r": "hand_r", "socket_hand_l": "hand_l", "socket_carry": "chest",
           "socket_head_top": "head"}
MAX_TRIS = 3000
MAX_BYTES = 400 * 1024


# --------------------------------------------------------------------------- math

def qmul(a, b):
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return (aw * bx + ax * bw + ay * bz - az * by,
            aw * by - ax * bz + ay * bw + az * bx,
            aw * bz + ax * by - ay * bx + az * bw,
            aw * bw - ax * bx - ay * by - az * bz)


def qrot(q, v):
    x, y, z, w = q
    p = (v[0], v[1], v[2], 0.0)
    r = qmul(qmul(q, p), (-x, -y, -z, w))
    return r[:3]


def qnorm(q):
    n = math.sqrt(sum(c * c for c in q)) or 1.0
    return tuple(c / n for c in q)


def qangle(a, b):
    d = abs(sum(x * y for x, y in zip(qnorm(a), qnorm(b))))
    return math.degrees(2 * math.acos(min(1.0, d)))


def vadd(a, b):
    return tuple(x + y for x, y in zip(a, b))


def vsub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def vlen(a):
    return math.sqrt(sum(x * x for x in a))


# --------------------------------------------------------------------------- glTF helpers

class Model:
    def __init__(self, path):
        self.path = path
        self.js, self.bin = read_glb(path)
        js = self.js
        self.nodes = js.get("nodes", [])
        self.names = [n.get("name", f"node{i}") for i, n in enumerate(self.nodes)]
        self.parent = {}
        for i, n in enumerate(self.nodes):
            for c in n.get("children", []):
                self.parent[c] = i
        self.skin = js["skins"][0] if js.get("skins") else None
        self.joints = self.skin["joints"] if self.skin else []

    def acc(self, idx):
        return accessor_values(self.js, self.bin, idx)

    def rest_trs(self, i):
        n = self.nodes[i]
        return (tuple(n.get("translation", (0, 0, 0))), tuple(n.get("rotation", (0, 0, 0, 1))))

    def world(self, i, local):
        """World (translation, rotation) of node i using local(i) -> (t, r)."""
        chain = []
        j = i
        while j is not None:
            chain.append(j)
            j = self.parent.get(j)
        t, r = (0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 1.0)
        for j in reversed(chain):
            lt, lr = local(j)
            t = vadd(t, qrot(r, lt))
            r = qnorm(qmul(r, lr))
        return t, r

    def clip(self, anim):
        """{node: {path: (times, values)}}"""
        out = {}
        for ch in anim["channels"]:
            s = anim["samplers"][ch["sampler"]]
            times = [v[0] for v in self.acc(s["input"])]
            vals = self.acc(s["output"])
            out.setdefault(ch["target"]["node"], {})[ch["target"]["path"]] = (times, vals, s.get("interpolation", "LINEAR"))
        return out


def sample(track, t):
    times, vals, _ = track
    if t <= times[0]:
        return vals[0]
    if t >= times[-1]:
        return vals[-1]
    for k in range(len(times) - 1):
        if times[k] <= t <= times[k + 1]:
            u = (t - times[k]) / (times[k + 1] - times[k] or 1)
            a, b = vals[k], vals[k + 1]
            if len(a) == 4 and sum(x * y for x, y in zip(a, b)) < 0:
                b = tuple(-x for x in b)
            v = tuple(x + (y - x) * u for x, y in zip(a, b))
            return qnorm(v) if len(v) == 4 else v
    return vals[-1]


def pose_local(model, clipd, t):
    def local(j):
        lt, lr = model.rest_trs(j)
        ch = clipd.get(j, {})
        if "translation" in ch:
            lt = sample(ch["translation"], t)
        if "rotation" in ch:
            lr = sample(ch["rotation"], t)
        return lt, lr
    return local


def png_size(model, image):
    bv = model.js["bufferViews"][image["bufferView"]]
    data = model.bin[bv.get("byteOffset", 0): bv.get("byteOffset", 0) + bv["byteLength"]]
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        return None
    return struct.unpack(">II", data[16:24])


# --------------------------------------------------------------------------- checks

def manifest_animations(asset):
    path = os.path.join(REPO, "assets", "manifest.toml")
    if tomllib:
        with open(path, "rb") as f:
            m = tomllib.load(f)
        for a in m.get("asset", []):
            if a.get("id") == asset:
                return a.get("animations", [])
        return None
    for block in open(path, encoding="utf-8").read().split("[[asset]]")[1:]:
        if re.search(r'^id\s*=\s*"%s"' % re.escape(asset), block, re.M):
            mm = re.search(r"^animations\s*=\s*\[(.*?)\]", block, re.M | re.S)
            return re.findall(r'"([^"]+)"', mm.group(1)) if mm else []
    return None


def check(path, only_clips=None):
    err, info = [], []
    m = Model(path)
    js = m.js
    asset = os.path.splitext(os.path.basename(path))[0]
    size = os.path.getsize(path)

    # general export rules
    if not js.get("asset", {}).get("version", "").startswith("2."):
        err.append("asset.version is not 2.x")
    used = " ".join(js.get("extensionsUsed", [])).lower()
    if "draco" in used or "meshopt" in used:
        err.append(f"compression extension used: {used}")
    if has_extras(js):
        err.append("extras / custom properties present")

    # RIG-001 / RIG-002
    jn = [m.names[j] for j in m.joints]
    if not m.skin:
        return asset, ["no skin"], info
    if sorted(jn) != sorted(PARENTS):
        err.append(f"RIG-001 joint names differ: extra {sorted(set(jn) - set(PARENTS))}, "
                   f"missing {sorted(set(PARENTS) - set(jn))}")
    for j in m.joints:
        n = m.names[j]
        p = m.parent.get(j)
        pn = m.names[p] if p is not None else None
        want = PARENTS.get(n, "?")
        if want is None:
            if pn in PARENTS:
                err.append(f"RIG-001 root has joint parent {pn}")
        elif pn != want:
            err.append(f"RIG-001 parent of {n} is {pn}, expected {want}")
    if len(jn) > 32:
        err.append(f"RIG-002 {len(jn)} joints > 32")
    elif len(jn) > 24:
        err.append(f"RIG-002 {len(jn)} joints > 24")

    # armature node identity (RIG-021) + mesh node without transform
    idx = {n: i for i, n in enumerate(m.names)}
    root = idx.get("root")
    arm_node = m.parent.get(root) if root is not None else None
    if arm_node is not None:
        n = m.nodes[arm_node]
        t, r, s = n.get("translation", [0, 0, 0]), n.get("rotation", [0, 0, 0, 1]), n.get("scale", [1, 1, 1])
        if any(abs(a - b) > 1e-5 for a, b in zip(t + r + s, [0, 0, 0, 0, 0, 0, 1, 1, 1, 1])) or "matrix" in n:
            err.append(f"RIG-021 armature node '{m.names[arm_node]}' has a transform")
    for i, n in enumerate(m.nodes):
        if "mesh" in n and any(k in n for k in ("translation", "rotation", "scale", "matrix")):
            err.append(f"mesh node '{m.names[i]}' has a transform (apply transforms)")
        if "scale" in n and i in m.joints:
            if any(abs(c - 1) > 1e-4 for c in n["scale"]):
                err.append(f"RIG §3.4 joint {m.names[i]} has scale in bind pose")

    # RIG-004 rest positions (world, glTF space)
    rest_local = lambda j: m.rest_trs(j)  # noqa: E731
    rest_pos = {m.names[j]: m.world(j, rest_local)[0] for j in m.joints}
    worst = max((vlen(vsub(rest_pos[n], REST[n])), n) for n in rest_pos if n in REST)
    if worst[0] > 0.005:
        err.append(f"RIG-004 {worst[1]} rest position off by {worst[0]:.4f} m")
    info.append(f"rest joints within {worst[0] * 1000:.2f} mm of ART-RIG §2.4")

    # RIG-005 A-pose
    for s, sign in (("l", 1), ("r", -1)):
        d = vsub(rest_pos[f"hand_{s}"], rest_pos[f"upper_arm_{s}"])
        ang = math.degrees(math.atan2(-d[1], sign * d[0]))
        if abs(ang - 45) > 3:
            err.append(f"RIG-005 arm {s} is {ang:.1f} deg below horizontal")
        d = vsub(rest_pos[f"foot_{s}"], rest_pos[f"upper_leg_{s}"])
        tilt = math.degrees(math.atan2(math.hypot(d[0], d[2]), -d[1]))
        if tilt > 2:
            err.append(f"RIG-005 leg {s} tilted {tilt:.1f} deg")

    # RIG-021 facing / sides
    if rest_pos["upper_arm_r"][0] >= 0:
        err.append("RIG-021 upper_arm_r not at -X")

    # RIG-011 sockets
    for s, bone in SOCKETS.items():
        if s not in idx:
            err.append(f"RIG-011 socket {s} missing")
            continue
        p = m.parent.get(idx[s])
        if p is None or m.names[p] != bone:
            err.append(f"RIG-011 {s} parent is {m.names[p] if p is not None else None}, expected {bone}")
        if idx[s] in m.joints:
            err.append(f"RIG-011 {s} is a joint")
        t, r = m.world(idx[s], rest_local)
        if qangle(r, (0, 0, 0, 1)) > 0.5:
            err.append(f"RIG-011 {s} rest orientation is not +Y up / +Z forward ({qangle(r, (0, 0, 0, 1)):.1f} deg)")
    if "socket_carry" in idx:
        t, _ = m.world(idx["socket_carry"], rest_local)
        if vlen(vsub(t, (0, 0.62, 0.20))) > 0.005:
            err.append(f"RIG-011 socket_carry at {t}")

    # mesh / skin / materials (RIG-003, RIG-018, RIG-021, RIG-022)
    meshes = js.get("meshes", [])
    skinned = [n for n in m.nodes if "mesh" in n]
    if len(meshes) != 1 or len(skinned) != 1 or "skin" not in skinned[0]:
        err.append(f"RIG-018 expected one skinned mesh, got {len(meshes)} meshes")
    mats = [mt.get("name") for mt in js.get("materials", [])]
    tris = 0
    mn, mx = [1e9] * 3, [-1e9] * 3
    head_j = jn.index("head")
    for prim in meshes[0]["primitives"]:
        a = prim["attributes"]
        mat = mats[prim["material"]] if "material" in prim else None
        if prim.get("mode", 4) != 4:
            err.append("primitive not TRIANGLES")
        if any(k.startswith("COLOR_") for k in a):
            err.append("vertex colours present")
        if "JOINTS_1" in a or "WEIGHTS_1" in a:
            err.append("RIG-003 JOINTS_1/WEIGHTS_1 present")
        if prim.get("targets"):
            err.append("RIG-003 morph targets present")
        for req in ("POSITION", "NORMAL", "TEXCOORD_0", "JOINTS_0", "WEIGHTS_0"):
            if req not in a:
                err.append(f"missing {req} on {mat}")
        if js["accessors"][a["JOINTS_0"]]["componentType"] != 5121:
            err.append("RIG-003 JOINTS_0 is not UNSIGNED_BYTE")
        pos = m.acc(a["POSITION"])
        nrm = m.acc(a["NORMAL"])
        uvs = m.acc(a["TEXCOORD_0"])
        jts = m.acc(a["JOINTS_0"])
        wacc = js["accessors"][a["WEIGHTS_0"]]
        wscale = {5126: 1.0, 5121: 255.0, 5123: 65535.0}[wacc["componentType"]]
        wts = [tuple(x / wscale for x in w) for w in m.acc(a["WEIGHTS_0"])]
        tol = 0.001 if wscale == 1.0 else 2.0 / wscale
        bad = 0
        for jv, wv in zip(jts, wts):
            if sum(1 for w in wv if w > 0) > 4 or abs(sum(wv) - 1) > tol or any(
                    j >= len(jn) for j, w in zip(jv, wv) if w > 0):
                bad += 1
        if bad:
            err.append(f"RIG-003 {bad} vertices with bad weights in {mat}")
        tris += (js["accessors"][prim["indices"]]["count"] if "indices" in prim else len(pos)) // 3
        for p in pos:
            for i in range(3):
                mn[i] = min(mn[i], p[i])
                mx[i] = max(mx[i], p[i])
        if mat == "face":
            if any(not (abs(w[0] - 1) < tol and j[0] == head_j) for j, w in zip(jts, wts)):
                err.append("RIG-003 face vertices not weighted 1.0 to head")
            if any(not (-1e-4 <= u <= 0.25 + 1e-4 and -1e-4 <= v <= 0.5 + 1e-4) for u, v in uvs):
                err.append("RIG-018 face UVs outside cell 0 of the face atlas")
            cz = sum(p[2] for p in pos) / len(pos)
            cy = sum(p[1] for p in pos) / len(pos)
            if not (cz > 0 and cy > 0.86):
                err.append(f"RIG-021 face centroid y={cy:.3f} z={cz:.3f} (must face +Z)")
            if js["materials"][prim["material"]].get("alphaMode") != "MASK":
                err.append("face material alphaMode is not MASK")
        elif mat == "body":
            seen = {}
            split = 0
            for p, n in zip(pos, nrm):
                k = tuple(round(c, 5) for c in p)
                if k in seen and max(abs(a_ - b_) for a_, b_ in zip(seen[k], n)) > 1e-3:
                    split += 1
                seen.setdefault(k, n)
            if split:
                err.append(f"RIG-022 {split} body vertices share a position with differing normals")
        else:
            err.append(f"RIG-018 unexpected material {mat}")
    if sorted(mats) != ["body", "face"]:
        err.append(f"RIG-018 materials {mats}, expected body + face")
    for tex_i, mt in enumerate(js.get("materials", [])):
        ti = mt.get("pbrMetallicRoughness", {}).get("baseColorTexture", {}).get("index")
        if ti is None:
            err.append(f"material {mt.get('name')} has no texture")
            continue
        wh = png_size(m, js["images"][js["textures"][ti]["source"]])
        lim = (256, 256) if mt.get("name") == "body" else (256, 128)
        if wh is None or wh[0] > lim[0] or wh[1] > lim[1]:
            err.append(f"RIG-018 {mt.get('name')} texture {wh} exceeds {lim}")
    if tris > MAX_TRIS:
        err.append(f"RIG-018 {tris} triangles > {MAX_TRIS}")
    if size > MAX_BYTES:
        err.append(f"RIG-018 file {size / 1024:.0f} KB > 400 KB")
    if abs(mn[1]) > 0.01:
        err.append(f"min Y = {mn[1]:.4f} (expected 0)")
    if not (1.19 <= mx[1] <= 1.245):
        err.append(f"height {mx[1]:.3f} m (expected 1.20, hair <= +0.04)")
    info.append(f"{tris} tris, {len(jn)} joints, {size / 1024:.0f} KB, "
                f"size x {mx[0] - mn[0]:.2f} y {mn[1]:.3f}..{mx[1]:.3f} z {mn[2]:.2f}..{mx[2]:.2f}")

    # clips: RIG-006, RIG-007, RIG-008, RIG-010
    anims = {a["name"]: a for a in js.get("animations", [])}
    want = only_clips if only_clips is not None else manifest_animations(asset)
    if want is None:
        err.append("asset not in manifest")
        want = []
    if sorted(anims) != sorted(want):
        err.append(f"RIG-007 clips {sorted(anims)} != expected {sorted(want)}")
    for name, a in anims.items():
        cd = m.clip(a)
        for node, chans in cd.items():
            nn = m.names[node]
            if nn == "root":
                err.append(f"RIG-006 {name}: root is animated")
            if "scale" in chans:
                err.append(f"RIG-006 {name}: scale channel on {nn}")
            if "translation" in chans and nn != "hips":
                err.append(f"RIG-006 {name}: translation on {nn}")
            if node not in m.joints:
                err.append(f"RIG-006 {name}: animates non-joint {nn}")
        dur = max(tr[0][-1] for ch in cd.values() for tr in ch.values())
        frames, loop = PLAYER_CLIPS.get(name, (None, None))
        if frames is not None and abs(dur - frames / FPS) > 1 / FPS + 1e-6:
            err.append(f"RIG-007 {name}: {dur:.3f} s, expected {frames / FPS:.3f} s")
        if loop:
            t_end = frames / FPS
            p0, p1 = pose_local(m, cd, 0.0), pose_local(m, cd, t_end)
            worst_r = max(qangle(p0(j)[1], p1(j)[1]) for j in m.joints)
            hips = idx["hips"]
            dt = vlen(vsub(p0(hips)[0], p1(hips)[0]))
            if worst_r > 0.5 or dt > 0.002:
                err.append(f"RIG-008 {name}: loop seam {worst_r:.2f} deg / {dt * 1000:.1f} mm")
        if name in LOCOMOTION:
            v = LOCOMOTION[name]
            drift_all = []
            for s in ("l", "r"):
                fj = idx[f"foot_{s}"]
                pts = []
                for f in range(frames + 1):
                    t = f / FPS
                    p, _ = m.world(fj, pose_local(m, cd, t))
                    pts.append((p[0], p[1], p[2] + v * t))  # character moves along +Z
                ymin = min(p[1] for p in pts)
                contact = [p for p in pts if p[1] <= ymin + 0.004]
                drift = max(math.hypot(a_[0] - b_[0], a_[2] - b_[2]) for a_ in contact for b_ in contact)
                drift_all.append(drift)
                if drift > 0.03:
                    err.append(f"RIG-010 {name}: foot_{s} drifts {drift:.3f} m while planted")
            info.append(f"{name}: planted-foot drift {max(drift_all) * 1000:.1f} mm at {v} m/s")
    info.append("clips: " + ", ".join(
        f"{n} {max(tr[0][-1] for ch in m.clip(a).values() for tr in ch.values()) * FPS:.0f}f"
        for n, a in sorted(anims.items())))
    return asset, err, info


def main(argv):
    only = None
    if "--clips" in argv:
        i = argv.index("--clips")
        only = argv[i + 1].split(",")
        argv = argv[:i] + argv[i + 2:]
    files = argv or sorted(glob.glob(os.path.join(REPO, "assets", "models", "characters", "*.glb")))
    if not files:
        print("no character .glb files found")
        return 1
    failed = 0
    for f in files:
        try:
            asset, err, info = check(f, only)
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
