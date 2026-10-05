"""Kit landmarks_l2 — level-2 riddle landmarks: the zoo train at its station and the blossom
tree. Concepts (chosen sheets, built on the user's order 2026-10-04):
art/props/kit_landmarks_l2/sheet_zoo_train_v1.jpg, sheet_blossom_tree_v4.jpg.

Run:  blender -b --factory-startup --python tools/blender/props/kit_landmarks_l2.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/<asset>.glb, assets/blender/props/kit_landmarks_l2.blend,
         art/props/kit_landmarks_l2/model_preview.png (day on top, night-tinted below)

Origins = centre of the level rect, front = south = glTF +Z, 1 unit = 1 m.
- zoo_train      rect 8 x 2 (level-2 `train_se`), 2.2 m: long axis = X, engine at the west end
                 facing -X (red cab, cream boiler, black chimney, bell, cowcatcher), a blue and a
                 yellow open wagon, a short piece of track on a gravel bed and a small wooden
                 station platform with a plank-roofed shelter and a bench on the NORTH side.
                 Child nodes (translation only): `wheel_e0..e2` (engine axles) and
                 `wheel_w1a/w1b/w2a/w2b` (wagon axles), each with its pivot on the axle centre
                 (turn about glTF +Z... the axle axis = Blender Y = glTF -Z), and `smoke`
                 (white puffs above the chimney, pivot = chimney top). Empty `socket_chimney`.
- blossom_tree   rect 2 x 2, 6 m: forked brown trunk, big round pink crown with darker shade
                 clumps and light blossom clumps, a drift of petals on the ground; child nodes
                 `petals` (falling petals, pivot = crown centre) and `bees` (three bees, pivot =
                 crown centre; the game may orbit it).
"""

import math
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import night_lib as nl  # noqa: E402
import props_parts as pp  # noqa: E402
from night_lib import zb, FRONT, BACK  # noqa: E402
from mathutils import Vector  # noqa: E402

KIT = "kit_landmarks_l2"
PREVIEW_DIR = "kit_landmarks_l2"


# --------------------------------------------------------------------------- helpers

def prism_yz(poly, x0, x1, color):
    """Polygon in the Y/Z plane [(y, z)] extruded along X from x0 to x1."""
    bm = zb._new_bm()
    a = [bm.verts.new((x0, y, z)) for y, z in poly]
    b = [bm.verts.new((x1, y, z)) for y, z in poly]
    n = len(poly)
    bm.faces.new(list(reversed(a)))
    bm.faces.new(b)
    for i in range(n):
        j = (i + 1) % n
        bm.faces.new((a[i], a[j], b[j], b[i]))
    return pp._finish(bm, color)


def along_y(x, y0, y1, z, d, color, sides=8):
    """Round disc / cylinder with its axis along Y (wheels, hubs)."""
    return pp.beam((x, y0, z), (x, y1, z), d, sides=sides, color=color)


def along_x(x0, x1, y, z, d, color, sides=10):
    return pp.beam((x0, y, z), (x1, y, z), d, sides=sides, color=color)


# --------------------------------------------------------------------------- zoo train

Y0 = -0.4            # track centre line (y); the platform is behind it (north, +y)
GAUGE = 0.56
RAIL_TOP = 0.12


def wheel(A, name, x, r, spokes=True, ofs=0.31):
    """One axle (two wheels + the axle) as a child node, pivot on the axle centre."""
    z = RAIL_TOP + r
    A.node(name, pivot=(x, Y0, z))
    parts = [pp.beam((x, Y0 - ofs, z), (x, Y0 + ofs, z), 0.05, sides=4, color="train_black")]
    for sy in (-1, 1):
        yc = Y0 + sy * ofs
        parts.append(along_y(x, yc - 0.035, yc + 0.035, z, 2 * r, "train_black", sides=8))
        ys = yc + sy * 0.037
        parts.append(zb.knob(r * 0.28, (x, ys, z), color="brass", u=4, v=2, squash=0.6))
        if spokes:
            parts.append(zb.box((2 * r * 0.88, 0.012, r * 0.13), (x, ys, z), color="wood_dark"))
            parts.append(zb.box((r * 0.13, 0.012, 2 * r * 0.88), (x, ys, z), color="wood_dark"))
    A.add(parts, node=name, keep_down=True)


def engine(A, p):
    x0, x1 = -3.9, -1.6
    # chassis + cowcatcher
    p.append(zb.box((x1 - x0 - 0.1, 0.5, 0.2), ((x0 + x1) / 2 + 0.05, Y0, 0.3), color="train_black"))
    p.append(pp.prism_xz([(-4.0, 0.14), (-3.75, 0.14), (-3.75, 0.56), (-3.9, 0.36)], Y0 - 0.34, Y0 + 0.34,
                         "train_red"))
    for k in range(3):                     # cowcatcher slats
        p.append(zb.box((0.02, 0.05, 0.22), (-3.93 - 0.01, Y0 - 0.2 + 0.2 * k, 0.28), color="train_red_dark"))
    # boiler (cream) with brass bands and the brown smoke-box door
    p.append(along_x(-3.85, -2.35, Y0, 0.82, 0.66, "train_cream", sides=10))
    p.append(along_x(-3.93, -3.8, Y0, 0.82, 0.54, "wood_dark", sides=10))
    p.append(zb.knob(0.07, (-3.97, Y0, 0.82), color="brass", u=5, v=3, squash=0.6))
    for bx in (-3.35, -2.75):
        p.append(along_x(bx - 0.04, bx + 0.04, Y0, 0.82, 0.7, "brass", sides=10))
    # headlamp
    p.append(zb.knob(0.08, (-3.92, Y0, 1.22), color="bulb_yellow", u=5, v=3))
    # chimney (black, flared) and bell on its stand
    p.append(pp.cyl(0.11, 1.1, 1.5, sides=8, color="train_black", center=(-3.45, Y0)))
    p.append(pp.cyl(0.11, 1.5, 1.68, sides=8, r1=0.2, color="train_black", center=(-3.45, Y0)))
    p.append(pp.beam((-2.85, Y0, 1.1), (-2.85, Y0, 1.32), 0.05, sides=4, color="train_black"))
    p.append(pp.cyl(0.02, 1.2, 1.46, sides=6, r1=0.12, color="brass", center=(-2.85, Y0)))
    # cab: red box, darker roof overhang, windows (dark) on both sides and the back
    cx = -2.0
    p.append(zb.box((0.8, 0.82, 1.05), (cx, Y0, 0.9), color="train_red"))
    p.append(zb.box((0.95, 0.96, 0.1), (cx, Y0, 1.47), color="train_red_dark"))
    for sy in (-1, 1):
        p.append(zb.box((0.34, 0.03, 0.3), (cx - 0.02, Y0 + sy * 0.42, 1.0), color="eye_dark"))
    p.append(zb.box((0.03, 0.34, 0.3), (cx - 0.41, Y0, 1.0), color="eye_dark"))
    p.append(zb.box((0.03, 0.34, 0.3), (cx + 0.41, Y0, 1.0), color="eye_dark"))
    # side rods + footplate rail
    for sy in (-1, 1):
        yy = Y0 + sy * 0.55
        p.append(pp.beam((-2.0, yy, 0.46), (-2.9, yy, 0.29), 0.04, sides=4, color="wood_dark"))


def wagon(A, p, cx, body, wheel_names):
    L, W = 1.5, 0.82
    p.append(zb.box((L + 0.1, 0.5, 0.14), (cx, Y0, 0.3), color="train_black"))
    # open box: floor, 4 walls, bench inside
    p.append(zb.box((L, W, 0.06), (cx, Y0, 0.4), color="wood_dark"))
    for sy in (-1, 1):
        p.append(zb.box((L, 0.07, 0.55), (cx, Y0 + sy * (W / 2 - 0.035), 0.7), color=body))
        p.append(zb.box((L + 0.04, 0.09, 0.05), (cx, Y0 + sy * (W / 2 - 0.035), 0.99), color="wood_light"))
    for sx in (-1, 1):
        p.append(zb.box((0.07, W - 0.14, 0.55), (cx + sx * (L / 2 - 0.035), Y0, 0.7), color=body))
        p.append(zb.box((0.09, W, 0.05), (cx + sx * (L / 2 - 0.035), Y0, 0.99), color="wood_light"))
    p.append(zb.box((0.5, 0.62, 0.05), (cx, Y0, 0.5), color="wood"))
    p.append(zb.box((0.5, 0.62, 0.05), (cx, Y0, 0.62), color="wood_light"))
    # coupler stubs
    for sx in (-1, 1):
        p.append(zb.box((0.14, 0.06, 0.06), (cx + sx * (L / 2 + 0.1), Y0, 0.3), color="train_black"))
    for sx, n in zip((-1, 1), wheel_names):
        wheel(A, n, cx + sx * 0.45, 0.15, spokes=False, ofs=0.46)


def track(p):
    bed = [(-4.0, -0.95), (4.0, -0.95), (4.0, 0.12), (-4.0, 0.12)]
    p.append(zb.slab(bed, 0.0, 0.05, color="gravel", chamfer=0.03))
    for k in range(16):
        x = -3.75 + k * 0.5
        p.append(zb.box((0.12, 0.84, 0.05), (x, Y0, 0.075), color="wood_dark"))
    for sy in (-1, 1):
        p.append(zb.box((8.0, 0.06, 0.07), (0, Y0 + sy * GAUGE / 2, 0.085), color="rail_steel"))


def station(p):
    x0, x1 = -0.6, 3.9
    y0, y1 = 0.3, 1.0
    top = 0.42
    p.append(zb.box((x1 - x0, y1 - y0, 0.08), ((x0 + x1) / 2, (y0 + y1) / 2, top - 0.04), color="wood_light"))
    p.append(zb.box((x1 - x0, 0.06, 0.12), ((x0 + x1) / 2, y0 - 0.02, top - 0.14), color="wood"))
    for x in (x0 + 0.15, (x0 + x1) / 2, x1 - 0.15):
        for y in (y0 + 0.08, y1 - 0.08):
            p.append(zb.box((0.12, 0.12, top - 0.08), (x, y, (top - 0.08) / 2), color="wood"))
    # shelter: posts, roof (single slope, plank strips) and diagonal braces
    PH, RH = 1.8, 1.95
    for x in (0.0, 1.95, 3.8):
        p.append(zb.box((0.1, 0.1, PH - top), (x, y0 + 0.1, top + (PH - top) / 2), color="wood"))
        p.append(zb.box((0.1, 0.1, RH - top), (x, y1 - 0.06, top + (RH - top) / 2), color="wood"))
        p.append(pp.beam((x, y0 + 0.1, top + 1.05), (x, y0 + 0.5, PH - 0.05), 0.06, sides=4, color="wood_dark"))
    n = 9
    sw = (4.3 + 0.0) / n
    for k in range(n):
        xa = -0.2 + k * sw
        col = "wood_light" if k % 2 == 0 else "roof_brown"
        p.append(prism_yz([(y0 - 0.2, PH + 0.02), (y1 + 0.12, RH + 0.14), (y1 + 0.12, RH + 0.04),
                           (y0 - 0.2, PH - 0.08)], xa, xa + sw - 0.02, col))
    # bench against the back
    p.append(zb.box((1.3, 0.3, 0.05), (2.5, y1 - 0.25, top + 0.4), color="wood"))
    p.append(zb.box((1.3, 0.05, 0.3), (2.5, y1 - 0.1, top + 0.6), color="wood_dark"))
    for sx in (-1, 1):
        p.append(zb.box((0.06, 0.26, 0.4), (2.5 + sx * 0.55, y1 - 0.25, top + 0.2), color="wood"))


def zoo_train():
    A = nl.Asset("zoo_train")
    p = []
    track(p)
    engine(A, p)
    wagon(A, p, -0.7, "train_blue", ("wheel_w1a", "wheel_w1b"))
    wagon(A, p, 0.95, "train_yellow", ("wheel_w2a", "wheel_w2b"))
    station(p)
    A.add(p)
    wheel(A, "wheel_e0", -3.4, 0.17, spokes=False, ofs=0.47)
    wheel(A, "wheel_e1", -2.9, 0.17, spokes=False, ofs=0.47)
    wheel(A, "wheel_e2", -2.0, 0.34, ofs=0.47)
    top = Vector((-3.45, Y0, 1.68))
    A.node("smoke", pivot=top)
    A.add([zb.knob(0.11, tuple(top + Vector((0.0, 0.0, 0.14))), color="sheet_white", u=6, v=4),
           zb.knob(0.14, tuple(top + Vector((0.05, 0.0, 0.34))), color="sheet_white", u=6, v=4),
           zb.knob(0.17, tuple(top + Vector((0.14, 0.0, 0.52))), color="sheet_white", u=6, v=4)],
          node="smoke", keep_down=True)
    A.empty("socket_chimney", (-3.45, Y0, 1.75))
    return A


# --------------------------------------------------------------------------- blossom tree

def blossom_tree():
    A = nl.Asset("blossom_tree")
    p = []
    # trunk: flared foot, two-fork
    p.append(pp.cyl(0.52, 0.0, 0.5, sides=8, r1=0.4, color="trunk"))
    p.append(pp.beam((0, 0, 0.4), (0.0, 0.0, 2.4), 0.8, sides=8, taper=0.7, color="trunk"))
    for a, l in ((200, 1.1), (340, 1.1), (90, 0.9)):
        d = Vector((math.cos(math.radians(a)) * 0.5, math.sin(math.radians(a)) * 0.5, 0.9)).normalized()
        p.append(pp.beam((0, 0, 2.0), tuple(Vector((0, 0, 2.0)) + d * l), 0.34, sides=5, taper=0.6, color="trunk"))
    for k in range(5):                      # roots
        a = 2 * math.pi * k / 5 + 0.3
        p.append(pp.beam((0.3 * math.cos(a), 0.3 * math.sin(a), 0.35),
                         (0.85 * math.cos(a), 0.85 * math.sin(a), 0.0), 0.24, sides=4, taper=0.4, color="trunk_dark"))
    # crown
    C = (0.0, 0.0, 4.15)
    p.append(pp.blob(C, (2.0, 2.0, 1.85), "blossom_pink", subdiv=2, seed=7, amp=0.07))
    for k in range(9):                     # scalloped rim: ring of clumps around the crown
        a = 2 * math.pi * k / 9 + 0.2
        z = 4.25 + 0.35 * math.sin(k * 2.1)
        p.append(pp.blob((1.62 * math.cos(a), 1.62 * math.sin(a), z), (0.8, 0.8, 0.7),
                         "blossom_pink" if k % 3 else "blossom_pink_lt", subdiv=1, seed=k + 20, amp=0.12))
    for k in range(6):                     # darker underside clumps
        a = 2 * math.pi * k / 6 + 0.6
        p.append(pp.blob((1.35 * math.cos(a), 1.35 * math.sin(a), 3.2), (0.8, 0.8, 0.55),
                         "blossom_pink_dk", subdiv=1, seed=k + 30, amp=0.14))
    for k, (x, y, z, r) in enumerate(((-0.4, -0.5, 5.75, 0.7), (0.9, 0.3, 5.5, 0.55), (-1.2, 0.3, 5.35, 0.55),
                                      (0.2, 1.0, 5.45, 0.5), (0.9, -1.2, 5.0, 0.45))):
        p.append(pp.blob((x, y, z), (r, r, r * 0.6), "blossom_pink_lt", subdiv=1, seed=k + 40, amp=0.12))
    # a few green leaf tips
    for a in (30, 150, 250):
        d = Vector((math.cos(math.radians(a)), math.sin(math.radians(a)), 0.3))
        p.append(pp.leaf((1.8 * d.x, 1.8 * d.y, 4.2), d, 0.35, 0.16, "leaf"))
    # petal drift on the ground
    p.append(pp.disc((0, 0, 0.02), 1.5, "blossom_pink_lt", sides=10))
    p.append(pp.disc((0.2, -0.1, 0.03), 1.05, "blossom_pink", sides=9))
    A.add(p)
    # falling petals
    A.node("petals", pivot=C)
    pet = []
    for k, (x, y, z) in enumerate(((-1.6, -1.1, 2.6), (1.5, -1.3, 2.2), (0.6, -1.8, 1.7), (-0.5, 1.7, 2.1),
                                    (1.9, 0.3, 1.5), (-1.9, 0.5, 1.2), (0.0, -1.0, 1.0), (1.0, 1.2, 0.9))):
        s = 0.11
        a = k * 0.8
        c, sn = math.cos(a) * s, math.sin(a) * s
        pts = [(x - c, y - sn, z), (x, y, z + s * 0.6), (x + c, y + sn, z), (x, y, z - s * 0.6)]
        pet.append(pp.flat(pts, "blossom_pink_lt"))
        pet.append(pp.flat(list(reversed(pts)), "blossom_pink_lt"))
    A.add(pet, node="petals", keep_down=True)
    A.node("bees", pivot=C)
    bees = []
    for (x, y, z) in ((-2.35, -0.6, 4.6), (1.2, -2.3, 3.7), (2.3, 0.9, 5.1)):
        bees.append(zb.knob(0.1, (x, y, z), color="bee_yellow", u=6, v=3, squash=0.8))
        bees.append(zb.knob(0.065, (x - 0.09, y, z + 0.01), color="eye_dark", u=5, v=3))
        bees.append(zb.box((0.03, 0.06, 0.025), (x + 0.01, y, z + 0.085), color="eye_dark"))
        for sy in (-1, 1):
            bees.append(pp.flat([(x, y + sy * 0.03, z + 0.07), (x + 0.07, y + sy * 0.11, z + 0.17),
                                 (x - 0.03, y + sy * 0.09, z + 0.17)], "sheet_white"))
            bees.append(pp.flat([(x, y + sy * 0.03, z + 0.07), (x - 0.03, y + sy * 0.09, z + 0.17),
                                 (x + 0.07, y + sy * 0.11, z + 0.17)], "sheet_white"))
    A.add(bees, node="bees", keep_down=True)
    return A


# --------------------------------------------------------------------------- main

def main():
    builders = {"zoo_train": zoo_train, "blossom_tree": blossom_tree}
    nl.run(KIT, builders, cols=1, spacing=(11.0, 8.0), budgets={"zoo_train": 2200, "blossom_tree": 1800, "*": 1800},
           preview_dir=PREVIEW_DIR)


if __name__ == "__main__":
    main()
