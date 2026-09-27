"""Kit landmarks — night-1 scenery and riddle landmarks (concept: art/environment/
env_night_overview/overview.png, env_night1_overview/brief.md + layout.md, night-1.toml notes).

Run:  blender -b --factory-startup --python tools/blender/props/kit_landmarks.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/<asset>.glb, assets/blender/props/kit_landmarks.blend,
         art/props/kit_landmarks/model_preview.png (day on top, night-tinted below)

Origins = centre of the level rect of the element (night-1.toml), front = south = glTF +Z.
- windmill       rect 2 x 2, 4.0 m: tower (root) + `sails` child node, pivot = the hub on the
                 front; turn the sails about glTF +Z (the hub axis). Balcony at 1.9 m.
- hollow_tree    rect 2 x 2, 6 m: thick trunk, round knot hole (centre y 2.5 m, facing front,
                 `socket_hole` empty at its back wall).
- old_tree       rect 2 x 2, 6 m: big mossy tree (the mushroom_patch lies at its foot).
- crooked_tree   rect 1 x 1, 3 m: low horizontal branch towards +X; `socket_branch` = where a
                 bat hangs (under the branch).
- fir_tree       rect 2 x 2, 9 m: tall dark pointed fir with cones.
- rock_hill      rect 3 x 3, 2 m: round grassy hill with one big round stone on top
                 (`socket_top` = hill top beside the stone).
- brush_pile     rect 3 x 2: heap of dry twigs and branches (no leaves).
- mushroom_patch rect 2 x 2: moss patch with a ring of round brown / cream mushrooms.
- flower_pots    ~0.9 x 0.6 m group of clay pots with white night flowers (stacked empty pots).
- potting_bench  rect 1 x 2 (long axis = X at yaw 0; turn +90 for the west hedge, facing
                 east): bench 1.9 x 0.6 m, top 0.85 m, back shelf to 1.2 m, stacked pots below,
                 white flowers in pots on top.
- telescope      rect 1 x 1, 1.4 m: toy star telescope on a wooden tripod, pointing up to the
                 front-left (the moon).
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import night_lib as nl  # noqa: E402
import props_parts as pp  # noqa: E402
from night_lib import zb, FRONT, BACK  # noqa: E402
from mathutils import Matrix, Vector  # noqa: E402

KIT = "kit_landmarks"


# --------------------------------------------------------------------------- windmill

def windmill():
    A = nl.Asset("windmill")
    p = []
    # stone foot + tapered octagonal wooden tower
    p.append(pp.cyl(0.85, 0.0, 0.3, sides=8, r1=0.82, color="wall_stone"))
    p.append(pp.cyl(0.74, 0.3, 2.9, sides=8, r1=0.5, color="wood_light"))
    # cap: red cone roof
    p.append(pp.cyl(0.62, 2.9, 3.0, sides=8, color="wood_dark"))
    p.append(pp.cyl(0.68, 3.0, 3.75, sides=8, r1=0.08, color="windmill_red"))
    p.append(zb.knob(0.08, (0, 0, 3.8), color="wood_dark", u=6, v=3))
    # balcony: ring platform + posts + rail
    p.append(pp.cyl(0.95, 1.85, 1.93, sides=8, color="wood_light"))
    for k in range(8):
        a = 2 * math.pi * k / 8
        p.append(zb.box((0.05, 0.05, 0.35), (0.9 * math.cos(a), 0.9 * math.sin(a), 2.1), color="wood"))
    ring = [(0.9 * math.cos(2 * math.pi * k / 8), 0.9 * math.sin(2 * math.pi * k / 8)) for k in range(9)]
    for a, b in zip(ring, ring[1:]):
        p.append(pp.beam((a[0], a[1], 2.28), (b[0], b[1], 2.28), 0.05, sides=3, color="wood", caps=False))
    # door + window (front)
    p.append(zb.box((0.5, 0.1, 0.9), (0, FRONT * 0.72, 0.75), color="wood_dark", bevel=0.02))
    p.append(nl.ring_xz(0, 2.3, 0.13, 8, FRONT * 0.62, FRONT * 0.54, "window_sky"))
    # hub axle
    p.append(pp.beam((0, FRONT * 0.4, 2.75), (0, FRONT * 0.75, 2.75), 0.1, sides=6, color="wood_dark"))
    A.add(p)
    hub = Vector((0, FRONT * 0.8, 2.75))
    A.node("sails", pivot=hub)
    s = [zb.knob(0.1, tuple(hub), color="windmill_red", u=6, v=4)]
    R = 1.25
    for k in range(4):
        a = math.radians(45 + 90 * k)
        d = Vector((math.cos(a), 0, math.sin(a)))
        side = Vector((-d.z, 0, d.x))
        tip = hub + d * R
        s.append(pp.beam(hub, tip, 0.05, sides=4, color="wood"))
        # cloth panel on one side of the spar (flat quad, both faces)
        c0 = hub + d * 0.25
        c1 = hub + d * (R - 0.02)
        quad = [c0 + side * 0.03, c1 + side * 0.03, c1 + side * 0.38, c0 + side * 0.38]
        for sgn, off in ((1, FRONT * 0.012), (-1, BACK * 0.0)):
            pts = [tuple(q + Vector((0, off, 0))) for q in quad]
            f = pp.flat(pts if sgn > 0 else list(reversed(pts)), "sail_cloth")
            s.append(f)
        # lattice bars across the cloth
        for t in (0.45, 0.75):
            q = hub + d * (R * t)
            s.append(pp.beam(q + side * 0.03 + Vector((0, FRONT * 0.02, 0)), q + side * 0.38 + Vector((0, FRONT * 0.02, 0)),
                             0.025, sides=3, color="wood"))
    A.add(s, node="sails", keep_down=True)
    return A


# --------------------------------------------------------------------------- trees

def roots(p, r, n, color="trunk_dark", seed=0, h=0.35):
    rnd = random.Random(seed)
    for k in range(n):
        a = 2 * math.pi * k / n + rnd.uniform(-0.3, 0.3)
        p.append(pp.beam((r * 0.6 * math.cos(a), r * 0.6 * math.sin(a), h),
                         ((r + 0.45) * math.cos(a), (r + 0.45) * math.sin(a), 0.0), 0.22, sides=4, taper=0.4,
                         color=color))


def hollow_tree():
    A = nl.Asset("hollow_tree")
    p = []
    p.append(pp.cyl(0.85, 0.0, 1.2, sides=10, r1=0.72, color="trunk"))
    p.append(pp.cyl(0.72, 1.2, 3.6, sides=10, r1=0.55, color="trunk"))
    roots(p, 0.8, 5, seed=1)
    # branches into the crown
    for a, l in ((40, 1.4), (160, 1.3), (280, 1.2)):
        d = Vector((math.cos(math.radians(a)), math.sin(math.radians(a)), 0.9)).normalized()
        p.append(pp.beam((0, 0, 3.3), tuple(Vector((0, 0, 3.3)) + d * l), 0.25, sides=4, taper=0.5, color="trunk"))
    # crown: three big round clumps
    for (x, y, z, r, c) in ((0, 0.5, 5.0, 1.6, "leaf"), (-1.0, 0.7, 4.5, 1.15, "leaf_light"),
                            (1.1, 0.4, 4.6, 1.15, "leaf")):
        p.append(pp.blob((x, y, z), (r, r * 0.9, r * 0.8), c, subdiv=1, seed=int(x * 10) + 3, amp=0.1))
    # knot hole at 2.5 m, facing front: dark recess + thick rim
    hy = FRONT * 0.66
    p.append(nl.ring_xz(0, 2.5, 0.52, 10, hy - 0.08, hy + 0.1, "trunk_dark", r_in=0.38))
    p.append(nl.ring_xz(0, 2.5, 0.39, 10, hy - 0.01, hy + 0.03, "eye_dark"))
    A.add(p)
    A.empty("socket_hole", (0, hy, 2.3))
    return A


def old_tree():
    A = nl.Asset("old_tree")
    p = []
    rnd = random.Random(5)
    p.append(pp.cyl(0.75, 0.0, 1.0, sides=9, r1=0.6, color="trunk"))
    # gnarled trunk: two offset segments
    p.append(pp.beam((0, 0, 0.9), (0.2, -0.1, 2.3), 0.95, sides=7, taper=0.8, color="trunk"))
    p.append(pp.beam((0.2, -0.1, 2.2), (-0.1, 0.1, 3.4), 0.75, sides=7, taper=0.8, color="trunk"))
    roots(p, 0.7, 6, seed=2)
    for a, l in ((20, 1.6), (130, 1.5), (230, 1.4), (320, 1.3)):
        d = Vector((math.cos(math.radians(a)), math.sin(math.radians(a)), 0.7)).normalized()
        p.append(pp.beam((-0.05, 0.05, 3.2), tuple(Vector((-0.05, 0.05, 3.2)) + d * l), 0.28, sides=4, taper=0.5,
                         color="trunk"))
    # moss patches on the trunk foot and roots
    for k in range(4):
        a = 2 * math.pi * k / 4 + 0.4
        p.append(pp.blob((0.62 * math.cos(a), 0.62 * math.sin(a), 0.35), (0.3, 0.3, 0.22), "moss", subdiv=1,
                         seed=k, amp=0.15))
    # wide crown
    for (x, y, z, r, c) in ((0, 0, 4.9, 1.9, "grove_leaf_light"), (-1.3, 0.3, 4.4, 1.3, "grove_leaf"),
                            (1.3, 0.2, 4.5, 1.3, "grove_leaf"), (0.2, -1.0, 4.4, 1.1, "grove_leaf_light")):
        p.append(pp.blob((x, y, z), (r, r, r * 0.75), c, subdiv=1, seed=rnd.randint(0, 99), amp=0.1))
    A.add(p)
    return A


def crooked_tree():
    A = nl.Asset("crooked_tree")
    p = []
    p.append(pp.cyl(0.2, 0.0, 0.3, sides=7, r1=0.16, color="trunk"))
    pts = [Vector((0, 0, 0.25)), Vector((-0.15, 0.05, 1.0)), Vector((0.05, 0.0, 1.7)), Vector((-0.1, 0.05, 2.4))]
    for a, b in zip(pts, pts[1:]):
        p.append(pp.beam(tuple(a), tuple(b), 0.28, sides=6, taper=0.85, color="trunk"))
    # low horizontal branch towards +X at 1.7 m (the bat hangs under it)
    p.append(pp.beam((0.0, 0.0, 1.65), (0.75, 0.0, 1.78), 0.14, sides=5, taper=0.7, color="trunk"))
    p.append(pp.beam((0.75, 0.0, 1.78), (1.15, 0.05, 1.95), 0.09, sides=4, taper=0.6, color="trunk"))
    roots(p, 0.18, 3, seed=4, h=0.2)
    for (x, y, z, r, c) in ((-0.1, 0.05, 2.6, 0.75, "leaf"), (0.4, -0.1, 2.35, 0.5, "leaf_light"),
                            (-0.5, 0.2, 2.3, 0.45, "leaf"), (1.15, 0.05, 2.05, 0.28, "leaf_light")):
        p.append(pp.blob((x, y, z), (r, r, r * 0.8), c, subdiv=1, seed=int(10 * x) + 11, amp=0.12))
    A.add(p)
    A.empty("socket_branch", (0.7, 0.0, 1.7))
    return A


def fir_tree():
    A = nl.Asset("fir_tree")
    p = [pp.cyl(0.22, 0.0, 2.0, sides=6, r1=0.16, color="trunk")]
    roots(p, 0.2, 3, seed=6, h=0.2)
    tiers = [(1.0, 1.25, 3.6), (2.6, 1.0, 5.4), (4.2, 0.78, 7.0), (5.8, 0.55, 8.2), (7.2, 0.33, 9.0)]
    for i, (z0, r, z1) in enumerate(tiers):
        c = "fir_dark" if i % 2 == 0 else "fir_light"
        cone = pp.cyl(r * 1.25, z0, z1, sides=9, apex=True, color=c, phase=0.3 * i)
        p.append(cone)
        # bottom skirt so the tier reads from below the rim
        p.append(pp.cyl(r * 1.25, z0 - 0.15, z0, sides=9, r1=r * 1.25, color="fir_dark", top=False, phase=0.3 * i))
    # a few cones hanging at the tier rims
    for k in range(6):
        a = 2 * math.pi * k / 6 + 0.3
        z0, r, _ = tiers[k % 3 + 1]
        p.append(zb.knob(0.08, (r * 1.15 * math.cos(a), r * 1.15 * math.sin(a), z0 - 0.08), color="mushroom_cap",
                         u=5, v=3, squash=1.4))
    A.add(p)
    return A


# --------------------------------------------------------------------------- hill, brush, mushrooms

def rock_hill():
    A = nl.Asset("rock_hill")
    p = []
    hill = pp.blob((0, 0, 0.0), (1.5, 1.45, 1.2), "grass", subdiv=2, seed=3, amp=0.05, flat_bottom=0.0)
    pp.recolor(hill, lambda f: f.normal.z < 0.35, "grass_dark")
    p.append(hill)
    p.append(pp.blob((0.15, 0.1, 1.5), (0.52, 0.48, 0.45), "rock", subdiv=1, seed=8, amp=0.08))
    p.append(pp.blob((0.9, -0.95, 0.12), (0.28, 0.22, 0.2), "rock_dark", subdiv=1, seed=9, amp=0.1))
    for k in range(5):
        a = 2 * math.pi * k / 5
        p.append(zb.tuft((1.2 * math.cos(a), 1.2 * math.sin(a), 0.35), height=0.2, color="grass_dark", seed=k))
    A.add(p)
    A.empty("socket_top", (-0.45, -0.2, 1.12))
    return A


def brush_pile():
    A = nl.Asset("brush_pile")
    p = [pp.blob((0, 0, 0.0), (1.2, 0.75, 0.7), "twig_dark", subdiv=1, seed=2, amp=0.12, flat_bottom=0.0)]
    rnd = random.Random(12)
    for k in range(34):
        a = rnd.uniform(0, math.pi)
        L = rnd.uniform(0.9, 1.9)
        cx, cy = rnd.uniform(-0.9, 0.9), rnd.uniform(-0.55, 0.55)
        # height on the ellipsoid surface
        t = max(0.0, 1 - (cx / 1.25) ** 2 - (cy / 0.8) ** 2)
        cz = 0.7 * math.sqrt(t) + 0.05
        d = Vector((math.cos(a), math.sin(a), rnd.uniform(-0.35, 0.35)))
        c = Vector((cx, cy, cz))
        p0, p1 = c - d * L / 2, c + d * L / 2
        p0.z = max(p0.z, 0.02)
        p1.z = max(p1.z, 0.02)
        p.append(pp.beam(tuple(p0), tuple(p1), rnd.uniform(0.04, 0.07), sides=3,
                         color=("twig", "twig_dark", "trunk")[k % 3]))
    A.add(p)
    return A


def mushroom(p, x, y, s, cap):
    p.append(pp.cyl(0.05 * s, 0.03, 0.16 * s + 0.03, sides=6, r1=0.045 * s, center=(x, y), color="mushroom_stem"))
    p.append(zb.knob(0.14 * s, (x, y, 0.17 * s + 0.03), color=cap, u=7, v=3, squash=0.6))


def mushroom_patch():
    A = nl.Asset("mushroom_patch")
    p = [zb.slab(zb.ellipse(0, 0, 0.95, 0.9, sides=10), 0.0, 0.05, color="moss", chamfer=0.03)]
    for k in range(8):
        a = 2 * math.pi * k / 8
        s = 1.0 + 0.35 * ((k * 7) % 3) / 2
        mushroom(p, 0.6 * math.cos(a), 0.6 * math.sin(a), s, "mushroom_cap" if k % 3 else "mushroom_stem")
    mushroom(p, 0.1, -0.05, 0.8, "mushroom_cap")
    A.add(p)
    return A


# --------------------------------------------------------------------------- pots, bench, telescope

def pot(p, x, y, z, r=0.13, h=0.22, upside=False, color="clay_pot", sides=7):
    if upside:
        p.append(pp.cyl(r, z, z + h, sides=sides, r1=r * 0.72, center=(x, y), color=color))
    else:
        p.append(pp.cyl(r * 0.72, z, z + h - 0.045, sides=sides, r1=r, center=(x, y), color=color, top=False))
        p.append(pp.cyl(r + 0.02, z + h - 0.045, z + h, sides=sides, center=(x, y), color="clay_pot_dark",
                        top=False))
        p.append(pp.disc((x, y, z + h - 0.03), r + 0.01, "bed_soil", sides=sides))


def white_flowers(p, x, y, z, n=3, seed=0, h=0.28):
    rnd = random.Random(seed)
    for k in range(n):
        a = 2 * math.pi * k / n + rnd.uniform(-0.4, 0.4)
        tip = (x + 0.07 * math.cos(a), y + 0.07 * math.sin(a), z + h * rnd.uniform(0.8, 1.1))
        p.append(pp.beam((x, y, z), tip, 0.015, sides=3, color="leaf", caps=False))
        p.append(pp.star(tip, 0.075, 0.03, 5, "flower_white", phase=a))
    a = rnd.uniform(0, 6.28)
    p.append(pp.leaf((x, y, z + 0.02), (math.cos(a), math.sin(a), 0.4), 0.14, 0.06, "leaf"))


def flower_pots():
    A = nl.Asset("flower_pots")
    p = []
    pot(p, -0.25, 0.0, 0.0)
    white_flowers(p, -0.25, 0.0, 0.19, n=4, seed=1)
    pot(p, 0.12, -0.12, 0.0, r=0.11, h=0.18)
    white_flowers(p, 0.12, -0.12, 0.15, n=3, seed=2, h=0.22)
    # stacked empty pots (upside down)
    for k in range(3):
        pot(p, 0.35, 0.15, 0.0 + 0.1 * k, r=0.12, h=0.16, upside=True)
    A.add(p)
    return A


def potting_bench():
    A = nl.Asset("potting_bench")
    L, D, H = 1.9, 0.6, 0.85
    p = [zb.box((L, D, 0.06), (0, 0, H - 0.03), color="wood_light", bevel=0.015)]
    for sx in (-1, 1):
        for sy in (-1, 1):
            p.append(zb.box((0.08, 0.08, H - 0.06), (sx * (L / 2 - 0.06), sy * (D / 2 - 0.06), (H - 0.06) / 2),
                            color="wood"))
    # lower shelf with stacked pots
    p.append(zb.box((L - 0.1, D - 0.1, 0.04), (0, 0, 0.2), color="wood"))
    for k, x in enumerate((-0.55, 0.0, 0.55)):
        for j in range(2 + k % 2):
            pot(p, x, 0.02, 0.22 + 0.09 * j, r=0.12, h=0.16, upside=True, sides=6)
    # back board + upper shelf (1.2 m)
    for sx in (-1, 1):
        p.append(zb.box((0.07, 0.07, 1.2 - H + 0.03), (sx * (L / 2 - 0.06), BACK * (D / 2 - 0.04), (H + 1.2) / 2),
                        color="wood"))
    p.append(zb.box((L, 0.24, 0.04), (0, BACK * (D / 2 - 0.1), 1.2), color="wood_light"))
    p.append(zb.box((L - 0.1, 0.03, 0.3), (0, BACK * (D / 2 - 0.02), H + 0.15), color="wood_dark"))
    # pots with white flowers on the top and the upper shelf, a small trowel
    for k, x in enumerate((-0.65, -0.2, 0.25)):
        pot(p, x, FRONT * 0.08, H, r=0.12, h=0.18)
        white_flowers(p, x, FRONT * 0.08, H + 0.15, n=3 if k != 1 else 2, seed=10 + k, h=0.25)
    for x in (-0.5, 0.45):
        pot(p, x, BACK * (D / 2 - 0.1), 1.22, r=0.08, h=0.12, upside=True, sides=6)
    p.append(zb.box((0.2, 0.06, 0.02), (0.65, FRONT * 0.1, H + 0.01), color="metal"))
    A.add(p)
    return A


def telescope():
    A = nl.Asset("telescope")
    p = []
    head = Vector((0, 0, 1.0))
    for k in range(3):
        a = math.radians(90 + 120 * k)
        p.append(pp.beam((0.5 * math.cos(a), 0.5 * math.sin(a), 0.0), tuple(head), 0.06, sides=4, color="wood"))
    p.append(pp.cyl(0.09, 0.95, 1.08, sides=8, color="wood_dark"))
    # tube pointing to the moon: up 35 deg towards the front-left
    d = Vector((-0.5, FRONT * 0.6, 0.62)).normalized()
    c = head + Vector((0, 0, 0.12))
    a = c - d * 0.35
    b = c + d * 0.65
    p.append(pp.beam(tuple(a), tuple(b), 0.13, sides=8, taper=1.25, color="telescope_blue"))
    for t, w in ((0.0, 0.16), (0.8, 0.19)):
        q = a + (b - a) * t
        p.append(pp.beam(tuple(q), tuple(q + d * 0.06), w, sides=8, color="brass"))
    p.append(pp.beam(tuple(a - d * 0.12), tuple(a), 0.05, sides=6, color="brass"))
    # little star on the tube
    A.add(p)
    return A


# --------------------------------------------------------------------------- preview extras

def extra(roots):
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    B = right * 30.0 + fwd * 3.0
    V = Vector
    out = [nl.instance_tree(roots["windmill"], "s_mill", B, node_rot={"sails": 0})]
    # sails turned 30 deg about the hub axis (preview of the rotating node)
    for o in nl.descendants(out[0]):
        if o.name.endswith("sails"):
            o.rotation_euler = (0, math.radians(30), 0)
    out.append(nl.instance_tree(roots["old_tree"], "s_old", B + V((0, 4.0, 0))))
    out.append(nl.instance_tree(roots["mushroom_patch"], "s_mush", B + V((1.4, 2.6, 0))))
    return out


def main():
    builders = {
        "windmill": windmill,
        "hollow_tree": hollow_tree,
        "old_tree": old_tree,
        "crooked_tree": crooked_tree,
        "fir_tree": fir_tree,
        "rock_hill": rock_hill,
        "brush_pile": brush_pile,
        "mushroom_patch": mushroom_patch,
        "flower_pots": flower_pots,
        "potting_bench": potting_bench,
        "telescope": telescope,
    }
    nl.run(KIT, builders, cols=6, spacing=(4.5, 6.0), extra=extra)


if __name__ == "__main__":
    main()
