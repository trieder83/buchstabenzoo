"""Kit 4 — nature (concept: art/props/kit_nature/sheet_v1.jpg, brief.md).

Run:  blender -b --factory-startup --python tools/blender/props/kit_nature.py -- [--no-preview]

Outputs
  assets/models/props/<asset>.glb     one per asset
  assets/blender/props/kit_nature.blend
  art/props/kit_nature/model_preview.png

Placement conventions (world axes, Q-056: +X east, +Y up, north = world -Z = Blender +Y;
origin on the ground at the centre of the footprint; rotate about +Y only):
- Every plant is roughly round: any yaw works; vary yaw (and a uniform scale of 0.85..1.15
  if the renderer supports it) per instance so repeats do not look copied.
- tree_round ~5 m, crown ~3.4 m wide: one per 3 x 3 cells (trees_nw, trees_ne).
- tree_grove ~6 m, crown ~2.8 m wide: dense grove, one per 2.5-3 m (grove_center).
- tree_eucalyptus ~7 m (not used in level 1; koala enclosure later).
- bush ~1.2 m, rock ~1 m, bamboo ~3 m clump (panda enclosure), reed ~1.1 m clump (pond
  edge), grass_tuft ~0.3 m (scatter on grass), flower_bed 2 x 1 m along X.
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import props_parts as pp  # noqa: E402
from props_parts import zb  # noqa: E402

KIT = "kit_nature"


def trunk(h, r0, r1, sides=7, color="trunk", roots=True, seed=0):
    parts = [pp.cyl(r0, 0.0, h, sides=sides, r1=r1, color=color, top=False)]
    if roots:
        rnd = random.Random(seed)
        for i in range(4):
            a = 2 * math.pi * i / 4 + rnd.uniform(-0.3, 0.3) + 0.4
            d = (math.cos(a), math.sin(a))
            parts.append(pp.beam((d[0] * r0 * 0.4, d[1] * r0 * 0.4, 0.35),
                                 (d[0] * (r0 + 0.3), d[1] * (r0 + 0.3), 0.0), r0 * 0.7, color=color, taper=0.3))
    return parts


def crown(clumps, colors, seed=0, subdiv=2, amp=0.1):
    parts = []
    for i, (c, r) in enumerate(clumps):
        rr = r if isinstance(r, tuple) else (r, r, r * 0.9)
        parts.append(pp.blob(c, rr, colors[i % len(colors)], subdiv=subdiv, seed=seed + i, amp=amp))
    return parts


def tree_round():
    parts = trunk(2.9, 0.4, 0.26, seed=1)
    # a branch fork visible under the crown
    parts.append(pp.beam((0.05, 0, 2.0), (0.6, -0.1, 3.0), 0.22, color="trunk", taper=0.6))
    parts += crown([((0.0, 0.0, 3.75), (1.5, 1.45, 0.95)),
                    ((0.8, -0.35, 3.45), (0.85, 0.85, 0.7)),
                    ((-0.85, 0.25, 3.5), (0.85, 0.85, 0.7)),
                    ((0.1, 0.5, 4.3), (0.9, 0.8, 0.65)),
                    ((-0.1, -0.75, 3.9), (0.8, 0.8, 0.65))],
                   ["leaf", "leaf_light", "leaf", "leaf_light", "leaf"], seed=10)
    return parts


def tree_grove():
    parts = trunk(1.2, 0.26, 0.2, roots=False)
    parts += crown([((0.0, 0.0, 3.1), (1.35, 1.3, 2.2)),
                    ((0.55, -0.45, 2.4), (0.85, 0.8, 1.1)),
                    ((-0.55, 0.3, 2.6), (0.85, 0.85, 1.2)),
                    ((0.1, 0.1, 4.6), (0.85, 0.85, 0.9)),
                    ((-0.25, -0.55, 3.8), (0.7, 0.7, 0.8))],
                   ["grove_leaf", "grove_leaf_light", "grove_leaf", "grove_leaf_light", "grove_leaf"], seed=20)
    return parts


def tree_eucalyptus():
    parts = [pp.cyl(0.13, 0.0, 5.2, sides=6, r1=0.07, color="euca_trunk", top=False)]
    branches = [((0, 0, 3.0), (-0.95, 0.2, 4.2)), ((0, 0, 3.6), (0.95, -0.1, 4.6)),
                ((0, 0, 4.4), (0.2, 0.5, 5.8)), ((0, 0, 2.4), (0.7, 0.4, 3.3))]
    for a, b in branches:
        parts.append(pp.beam(a, b, 0.09, color="euca_trunk", taper=0.6))
    for i, (_, b) in enumerate(branches):
        parts.append(pp.blob((b[0], b[1], b[2] + 0.2), (0.62, 0.62, 0.55), "euca_leaf", subdiv=2, seed=30 + i))
    parts.append(pp.blob((0.0, 0.0, 6.3), (0.7, 0.7, 0.65), "euca_leaf", subdiv=2, seed=40))
    return parts


def bush():
    return crown([((0.0, 0.0, 0.5), (0.62, 0.6, 0.5)),
                  ((0.3, -0.2, 0.42), (0.38, 0.38, 0.36)),
                  ((-0.3, 0.15, 0.44), (0.38, 0.38, 0.38)),
                  ((0.05, 0.1, 0.8), (0.36, 0.34, 0.3))],
                 ["leaf", "leaf_light", "leaf", "leaf_light"], seed=50)


def rock():
    p = pp.blob((0.0, 0.0, 0.3), (0.62, 0.5, 0.48), "rock", subdiv=2, seed=61, amp=0.16, flat_bottom=0.0)
    pp.recolor(p, lambda f: f.normal.z < 0.35 and f.normal.x < 0.2, "rock_dark")
    q = pp.blob((0.55, -0.05, 0.16), (0.32, 0.3, 0.24), "rock", subdiv=1, seed=62, amp=0.1, flat_bottom=0.0)
    return [p, q]


def bamboo(seed=70):
    rnd = random.Random(seed)
    parts = []
    stalks = [(0.0, 0.0, 3.0), (0.22, 0.12, 2.6), (-0.2, 0.15, 2.8), (0.12, -0.22, 2.3),
              (-0.18, -0.18, 2.5), (0.34, -0.1, 2.0), (-0.05, 0.32, 2.2)]
    for i, (x, y, h) in enumerate(stalks):
        r = 0.085
        seg = 4
        lean = (x * 0.25, y * 0.25)
        parts.append(pp.beam((x, y, 0.0), (x + lean[0], y + lean[1], h), 2 * r, color="bamboo", sides=5))
        for k in range(1, seg):
            z = h * k / seg
            cx, cy = x + lean[0] * z / h, y + lean[1] * z / h
            parts.append(pp.beam((cx, cy, z - 0.02), (cx, cy, z + 0.02), 2 * r + 0.03, color="bamboo_joint",
                                 sides=5, caps=False))
        # leaves: flat blades at the upper joints, pointing outwards
        for k in range(2, seg + 1):
            z = h * k / seg
            for j in range(2):
                a = rnd.uniform(0, 2 * math.pi)
                d = (math.cos(a), math.sin(a), 0.15)
                base = (x + lean[0] * z / h, y + lean[1] * z / h, z - 0.02)
                parts.append(pp.leaf(base, d, 0.55, 0.16, "bamboo_joint", droop=0.15))
    return parts


def reed(seed=80):
    rnd = random.Random(seed)
    # a green bundle as the body, so the clump reads as a green mass from above
    parts = [pp.cyl(0.2, 0.0, 0.55, sides=7, r1=0.07, color="grass_dark")]
    for i in range(8):
        a = 2 * math.pi * i / 8 + rnd.uniform(-0.2, 0.2)
        r = rnd.uniform(0.05, 0.16)
        base = (r * math.cos(a), r * math.sin(a), 0.0)
        lean = rnd.uniform(0.15, 0.4)
        tip = (base[0] + lean * math.cos(a), base[1] + lean * math.sin(a), rnd.uniform(0.6, 0.95))
        parts.append(pp.blade(base, tip, 0.17, "grass_dark" if i % 2 else "grass"))
    for i, (x, y, h) in enumerate([(0.0, 0.0, 1.15), (0.1, 0.07, 1.0), (-0.09, 0.05, 1.05), (0.03, -0.1, 0.9)]):
        top = (x * 1.8, y * 1.8, h)
        parts.append(pp.beam((x, y, 0.0), top, 0.04, color="grass_dark", sides=4))
        d = zb.Vector(top) - zb.Vector((x, y, 0.0))
        d.normalize()
        c0 = zb.Vector(top) - d * 0.22
        parts.append(pp.beam(tuple(c0), tuple(zb.Vector(top) - d * 0.02), 0.1, color="cattail", sides=6))
        parts.append(pp.beam(tuple(zb.Vector(top) - d * 0.03), tuple(zb.Vector(top) + d * 0.08), 0.015,
                             color="cattail", sides=3))
    return parts


def grass_tuft(seed=90):
    rnd = random.Random(seed)
    parts = [pp.cyl(0.08, 0.0, 0.17, sides=6, r1=0.03, color="grass")]
    for i in range(6):
        a = 2 * math.pi * i / 6 + rnd.uniform(-0.25, 0.25)
        r = rnd.uniform(0.01, 0.06)
        base = (r * math.cos(a), r * math.sin(a), 0.0)
        lean = rnd.uniform(0.04, 0.12)
        tip = (base[0] + lean * math.cos(a), base[1] + lean * math.sin(a), rnd.uniform(0.24, 0.34))
        parts.append(pp.blade(base, tip, 0.11, "grass_dark" if i % 3 else "grass"))
    return parts


def flower_bed(seed=100):
    rnd = random.Random(seed)
    L, W, H = 2.0, 1.0, 0.3
    parts = [zb.box((L - 0.1, W - 0.1, H - 0.06), (0, 0, (H - 0.06) / 2), color="bed_soil")]
    t = 0.08
    for s in (-1, 1):
        parts.append(zb.box((L, t, H), (0, s * (W / 2 - t / 2), H / 2), color="wood_light"))
        parts.append(zb.box((t, W - 2 * t, H), (s * (L / 2 - t / 2), 0, H / 2), color="wood_light"))
        # plank seam on the long sides
        parts.append(zb.box((L - 0.02, t + 0.01, 0.02), (0, s * (W / 2 - t / 2), H / 2), color="wood_dark"))
    for sx in (-1, 1):
        for sy in (-1, 1):
            parts.append(zb.box((0.1, 0.1, H + 0.04), (sx * (L / 2 - 0.05), sy * (W / 2 - 0.05), (H + 0.04) / 2),
                                color="wood"))
    cols = ["flower_red", "flower_yellow", "flower_purple"]
    for i in range(12):
        c, r = divmod(i, 3)
        x = -0.72 + c * 0.48 + rnd.uniform(-0.06, 0.06)
        y = -0.26 + r * 0.26 + rnd.uniform(-0.04, 0.04)
        z = H + rnd.uniform(0.1, 0.2)
        col = cols[min(2, (c * 3 + r) // 4)]
        parts.append(pp.beam((x, y, H - 0.08), (x, y, z), 0.02, color="grass_dark", sides=3))
        parts.append(pp.leaf((x, y, H - 0.02), (rnd.uniform(-1, 1), rnd.uniform(-1, 1), 0.3), 0.14, 0.07,
                             "grass_dark"))
        parts.append(pp.star((x, y, z), 0.085, 0.045, 5, col, phase=rnd.uniform(0, 1)))
        parts.append(pp.disc((x, y, z + 0.006), 0.03, "flower_center", sides=5))
    return parts


def extra(objs):
    # sample: a small grove, bushes along a line, reeds at a pond edge, bamboo clump
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    B = right * 14.0 - fwd * 2.0
    V = zb.Vector
    out = []
    for i, (dx, dy, rot) in enumerate([(0, 0, 0), (2.6, 0.4, 70), (1.2, 2.4, 140)]):
        out.append(pp.instance(objs["tree_grove"], f"s_grove{i}", B + V((dx, dy, 0)), rot))
    for i in range(4):
        out.append(pp.instance(objs["bush"], f"s_bush{i}", B + V((-1.5 + 1.2 * i, -2.5, 0)), 40 * i))
    for i in range(6):
        out.append(pp.instance(objs["grass_tuft"], f"s_tuft{i}", B + V((-1.0 + 0.9 * i, -3.6 + 0.2 * (i % 2), 0)),
                               60 * i))
    return out


def main():
    builders = {
        "tree_round": tree_round,
        "tree_grove": tree_grove,
        "tree_eucalyptus": tree_eucalyptus,
        "bush": bush,
        "flower_bed": flower_bed,
        "rock": rock,
        "bamboo": bamboo,
        "reed": reed,
        "grass_tuft": grass_tuft,
    }
    pp.run_kit(KIT, builders, cols=3, spacing=(4.2, 5.0), extra=extra)


if __name__ == "__main__":
    main()
