"""Kit 5 — water (concept: art/props/kit_water/sheet_v1.jpg, brief.md).

Run:  blender -b --factory-startup --python tools/blender/props/kit_water.py -- [--no-preview]

Outputs
  assets/models/props/<asset>.glb     one per asset
  assets/blender/props/kit_water.blend
  art/props/kit_water/model_preview.png

World axes (GAME-LAYOUT "Coordinate spaces", Q-056): +X east, +Y up, north = world -Z =
Blender +Y. Models are never mirrored, only rotated about +Y; a clockwise quarter turn
seen from above (N -> E) is -90 deg about +Y.

WATER TILES: 1 m x 1 m, one per water cell, centred on the cell (origin = cell centre,
y = 0), like kit_ground. Autotiled from the 4-neighbour mask of WATER cells
("connections"; count bridge cells over the river and cells beyond the level edge where
the river leaves as water). Sides without water get a bank: a 0.22 m grass strip at the
ground-tile height (y = 0.05, so it meets grass/path tiles flush) and a brown slope down to
the water surface (y = 0, the tile origin height). Canonical orientation (connections):
  water_*_straight / water_pond   N E S W      inner cell (full water)
  water_*_bank / water_pond_edge  N E S        bank on W
  water_*_curve / water_pond_corner  E S       banks on N and W, rounded outer corner
                                               (radius 0.78 m, quarter circle)
  water_river_inner               N E S W but the NE diagonal cell is dry: small
                                  rounded grass notch in the NE corner (inside of a bend)
River tiles are lively light blue with lighter FLOW STREAKS (palette cell
water_river_light) running along the canonical flow axis N-S (curve: around the SE
corner). Rotate so the streaks follow the river; direction of flow is not encoded in the
mesh (the renderer may scroll the streak faces). Pond tiles are darker, plain and still.

Other props (origin rules):
- bridge_wood: arched footbridge, deck runs along X from x = -1.7 to +1.7 (3 m river +
  0.2 m landing on each bank), 2.5 m wide (Y). Origin = centre of the 3 x 3 bridge rect on
  the ground. Deck top height (walk surface) y(x) = 0.07 + 0.43 * (1 - (x / 1.7)^2).
  For level 1 (bridge_river, path runs W-E over a N-S river) use yaw 0.
- jetty_wood: 3 m (X) x 1.6 m (Y) deck + 0.8 m overhang past the -X end over the water;
  deck top y = 0.2. Origin = centre of the 3 x 2 jetty rect; the WATER END is -X
  (level 1 jetty_pond: pond to the west -> yaw 0).
- lily_pad (group of pads + one flower), duck, frog (sits on its own pad): origin at the
  waterline; place them at y = 0 (water surface = tile origin height). duck and frog look south (-Y
  Blender = world +Z) at yaw 0, towards the default camera.
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import props_parts as pp  # noqa: E402
from props_parts import zb  # noqa: E402

KIT = "kit_water"
GROUND_TOP = 0.05      # = kit_ground SLAB_TOP
WATER_Y = 0.0          # water surface (tile origin height; streaks float 4 mm above)
M = 0.22               # bank grass strip width
SLOPE = 0.12           # horizontal width of the bank slope into the water
R = 1.0 - M            # radius of the rounded outer corner
E = 0.5


def arc(cx, cy, r, a0, a1, n):
    return [(cx + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)),
             cy + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


def shape(kind):
    """(water polygon, land polygon or None, bank line with water on the LEFT or None)."""
    if kind == "full":
        return [(-E, -E), (E, -E), (E, E), (-E, E)], None, None
    if kind == "bank":   # dry W
        x = -E + M
        return ([(x, -E), (E, -E), (E, E), (x, E)], [(-E, -E), (x, -E), (x, E), (-E, E)],
                [(x, E), (x, -E)])
    if kind == "corner":  # dry N and W, arc centred on the SE corner
        a = arc(E, -E, R, 90, 180, 8)
        water = [(E, -E)] + a
        land = [(-E, -E)] + list(reversed(a)) + [(E, E), (-E, E)]
        return water, land, a
    if kind == "inner":  # dry NE diagonal: notch of radius M around the NE corner
        a = arc(E, E, M, 270, 180, 4)
        water = [(-E, -E), (E, -E)] + a + [(-E, E)]
        land = [(E, E)] + list(reversed(a))
        return water, land, a
    raise ValueError(kind)


def bank_slope(line):
    """Brown slope from the grass edge (GROUND_TOP) down into the water (left side)."""
    p = zb.sweep([(0.0, GROUND_TOP), (SLOPE, 0.0)], line, color="soil", caps=False)
    p.bm.normal_update()
    if sum(f.normal.z for f in p.bm.faces) < 0:
        import bmesh
        bmesh.ops.reverse_faces(p.bm, faces=list(p.bm.faces))
    # clip the slope foot at the cell border (the sweep offsets the ends by the mitre)
    for v in p.bm.verts:
        v.co.x = max(-E, min(E, v.co.x))
        v.co.y = max(-E, min(E, v.co.y))
    return p


def on_border(f):
    c = f.calc_center_median()
    return max(abs(c.x), abs(c.y)) > E - 1e-4


def streak(x, y, L, w, color="water_river_light"):
    pts = [(x, y - L / 2), (x + w / 2, y - L / 4), (x + w / 2, y + L / 4), (x, y + L / 2),
           (x - w / 2, y + L / 4), (x - w / 2, y - L / 4)]
    return pp.flat([(px, py, WATER_Y + 0.004) for px, py in pts], color)


def arc_streak(cx, cy, r, a0, a1, w, n=5, color="water_river_light"):
    outer, inner = [], []
    for i in range(n + 1):
        t = i / n
        a = math.radians(a0 + (a1 - a0) * t)
        hw = w / 2 * math.sin(math.pi * t) + 0.004
        outer.append((cx + (r + hw) * math.cos(a), cy + (r + hw) * math.sin(a), WATER_Y + 0.004))
        inner.append((cx + (r - hw) * math.cos(a), cy + (r - hw) * math.sin(a), WATER_Y + 0.004))
    p = pp.flat(outer + list(reversed(inner)), color)
    p.bm.normal_update()
    if p.bm.faces[:][0].normal.z < 0:
        import bmesh
        bmesh.ops.reverse_faces(p.bm, faces=list(p.bm.faces))
    return p


def up(p):
    import bmesh
    p.bm.normal_update()
    if list(p.bm.faces)[0].normal.z < 0:
        bmesh.ops.reverse_faces(p.bm, faces=list(p.bm.faces))
    return p


STRAIGHT_STREAKS = [(-0.28, 0.22, 0.42, 0.05), (0.12, -0.12, 0.5, 0.06), (0.33, 0.3, 0.3, 0.04),
                    (-0.1, -0.36, 0.22, 0.035), (0.36, -0.3, 0.26, 0.04), (-0.34, -0.2, 0.28, 0.04)]


def water_tile(kind, body):
    water, land, line = shape(kind)
    color = "water_river" if body == "river" else "water_pond"
    parts = [up(pp.flat([(x, y, WATER_Y) for x, y in water], color))]
    if land:
        g = zb.slab(land, 0.0, GROUND_TOP, color="grass", top_color="grass", side_color="soil")
        g.delete_faces(lambda f: abs(f.normal.z) < 0.3 and not on_border(f))
        parts.append(g)
        parts.append(bank_slope(line))
    if body == "river":
        if kind == "corner":
            for r, a0, a1, w in ((0.32, 100, 160, 0.05), (0.52, 95, 150, 0.06), (0.52, 160, 176, 0.03),
                                 (0.66, 115, 170, 0.045)):
                parts.append(arc_streak(E, -E, r, a0, a1, w))
            parts.append(pp.disc((0.3, -0.05, WATER_Y + 0.005), 0.025, "water_foam", sides=5))
        else:
            xmin = -E + M + SLOPE + 0.04 if kind == "bank" else -E
            for x, y, L, w in STRAIGHT_STREAKS:
                if x - w / 2 < xmin:
                    continue
                if kind == "inner" and math.hypot(x - E, y - E) < M + SLOPE + 0.2:
                    continue
                parts.append(streak(x, y, L, w))
            if kind != "inner":
                parts.append(pp.disc((0.0, 0.3, WATER_Y + 0.005), 0.025, "water_foam", sides=5))
                parts.append(pp.disc((0.05, 0.36, WATER_Y + 0.005), 0.015, "water_foam", sides=5))
    else:
        # still pond: only a bank reed tuft now and then, no streaks
        pass
    if land and kind != "inner":
        parts.append(zb.tuft((-E + 0.1, 0.25, GROUND_TOP), 0.1, seed=3))
    return parts


# --------------------------------------------------------------------------- bridge, jetty

BR_L = 1.7     # half length of the deck along X
BR_W = 1.25    # half width
RISE = 0.43


def deck_y(x):
    return 0.07 + RISE * (1 - (x / BR_L) ** 2)


def arc_band(x0, x1, off_lo, off_hi, n):
    """Polygon in X/Z following the deck curve between deck_y + off_lo and deck_y + off_hi."""
    xs = [x0 + (x1 - x0) * i / n for i in range(n + 1)]
    top = [(x, deck_y(x) + off_hi) for x in xs]
    bot = [(x, max(0.0, deck_y(x) + off_lo)) for x in reversed(xs)]
    return top + bot


def bridge_wood():
    parts = []
    n = 10
    pw = 2 * BR_L / n
    for i in range(n):
        x = -BR_L + pw * (i + 0.5)
        slope = math.degrees(math.atan(-2 * RISE * x / BR_L ** 2))
        pl = zb.box((pw - 0.03, 2 * BR_W - 0.1, 0.07), (0, 0, 0), color="wood_light" if i % 2 else "wood")
        pl.rotate(-slope, "Y").move(x, 0, deck_y(x) - 0.035)
        parts.append(pl)
    for s in (-1, 1):
        # stringer under the deck edge
        parts.append(pp.prism_xz(arc_band(-BR_L, BR_L, -0.22, -0.06, 6), s * (BR_W - 0.12), s * (BR_W - 0.02),
                                 "wood_dark"))
        # handrail
        parts.append(pp.prism_xz(arc_band(-BR_L + 0.08, BR_L - 0.08, 0.66, 0.76, 6), s * BR_W - 0.05,
                                 s * BR_W + 0.05, "wood_light"))
        for x in (-BR_L + 0.08, -0.55, 0.55, BR_L - 0.08):
            parts.append(zb.box((0.11, 0.11, deck_y(x) + 0.82), (x, s * BR_W, (deck_y(x) + 0.82) / 2),
                                color="wood"))
    return parts


def jetty_wood():
    L0, L1 = -1.5 - 0.8, 1.5  # -X end over the water
    W = 0.8
    TOP = 0.2
    parts = []
    n = 12
    pw = (L1 - L0) / n
    for i in range(n):
        x = L0 + pw * (i + 0.5)
        parts.append(zb.box((pw - 0.035, 2 * W, 0.06), (x, 0, TOP - 0.03),
                            color="wood_light" if i % 3 else "wood"))
    for s in (-1, 1):
        parts.append(zb.box((L1 - L0, 0.1, 0.12), ((L0 + L1) / 2, s * (W - 0.1), TOP - 0.1), color="wood_dark"))
        for x, h in ((L0 + 0.08, 0.5), (L0 + 1.4, 0.0), (L1 - 0.1, 0.0)):
            parts.append(pp.cyl(0.08, 0.0, TOP + h, sides=6, center=(x, s * (W + 0.02)), color="wood"))
    return parts


# --------------------------------------------------------------------------- small props

def pad(x, y, r, rot, z=0.0, seed=0):
    """Lily pad: thin disc with a wedge notch."""
    pts = []
    for i in range(9):
        a = math.radians(rot + 25 + 310 * i / 8)
        pts.append((x + r * math.cos(a), y + r * math.sin(a)))
    pts.append((x, y))
    return zb.slab(pts, z, z + 0.02, color="lily_pad", side_color="hedge_dark")


def lily_flower(x, y, z, s=1.0):
    parts = [pp.star((x, y, z + 0.03 * s), 0.1 * s, 0.045 * s, 6, "lily_pink"),
             pp.star((x, y, z + 0.06 * s), 0.065 * s, 0.03 * s, 5, "lily_pink", phase=0.3),
             pp.disc((x, y, z + 0.07 * s), 0.022 * s, "lily_center", sides=5)]
    parts.append(pp.cyl(0.05 * s, z, z + 0.035 * s, sides=6, r1=0.08 * s, color="lily_pink"))
    return parts


def lily_pad():
    parts = []
    for i, (x, y, r, rot) in enumerate([(0.0, 0.0, 0.2, 10), (0.32, 0.12, 0.14, 120), (-0.3, 0.15, 0.13, 250),
                                         (0.12, -0.32, 0.12, 60), (-0.2, -0.26, 0.16, 300),
                                         (0.35, -0.2, 0.09, 190)]):
        parts.append(pad(x, y, r, rot, seed=i))
    parts += lily_flower(-0.02, 0.02, 0.02, s=1.4)
    return parts


def eye(x, y, z, r, look=(0, -1, 0), color_white="eye_white"):
    parts = [pp.blob((x, y, z), (r, r, r), color_white, subdiv=1, amp=0.0)]
    lx, ly, lz = look
    parts.append(pp.blob((x + lx * r * 0.7, y + ly * r * 0.7, z + lz * r * 0.7), (r * 0.5, r * 0.5, r * 0.5),
                         "eye_dark", subdiv=1, amp=0.0))
    return parts


def duck():
    F = pp.FRONT  # looks south (towards the default camera)
    parts = [pp.blob((0, 0, 0.1), (0.14, 0.2, 0.11), "duck", subdiv=2, amp=0.03, flat_bottom=0.0)]
    # tail tip up at the back
    parts.append(pp.beam((0, -F * 0.12, 0.12), (0, -F * 0.25, 0.2), 0.1, color="duck", taper=0.2, sides=5))
    # wings
    for s in (-1, 1):
        w = pp.blob((s * 0.12, -F * 0.02, 0.13), (0.04, 0.12, 0.06), "duck", subdiv=1, amp=0.0)
        parts.append(w)
    # head and neck
    parts.append(pp.blob((0, F * 0.12, 0.24), (0.095, 0.095, 0.1), "duck", subdiv=2, amp=0.0))
    parts.append(pp.blob((0, F * 0.21, 0.22), (0.05, 0.06, 0.022), "duck_beak", subdiv=1, amp=0.0))
    for s in (-1, 1):
        parts += eye(s * 0.05, F * 0.19, 0.28, 0.028, look=(s * 0.3, F * 0.9, 0.1))
    return parts


def frog():
    F = pp.FRONT
    parts = [pad(0, 0, 0.2, 60 if F < 0 else 240)]
    z = 0.02
    parts.append(pp.blob((0, 0, z + 0.07), (0.1, 0.1, 0.075), "frog", subdiv=2, amp=0.03))
    parts.append(pp.blob((0, F * 0.05, z + 0.06), (0.07, 0.05, 0.05), "frog_light", subdiv=1, amp=0.0))
    for s in (-1, 1):
        # hind legs and front feet
        parts.append(pp.blob((s * 0.1, -F * 0.03, z + 0.035), (0.045, 0.07, 0.035), "frog", subdiv=1, amp=0.0))
        parts.append(pp.blob((s * 0.06, F * 0.09, z + 0.015), (0.03, 0.03, 0.015), "frog", subdiv=1, amp=0.0))
        parts += eye(s * 0.05, F * 0.045, z + 0.15, 0.035, look=(s * 0.25, F * 0.9, 0.15))
    return parts


# --------------------------------------------------------------------------- autotiling

CW = {"N": "E", "E": "S", "S": "W", "W": "N"}
CANON = {"full": {"N", "E", "S", "W"}, "bank": {"N", "E", "S"}, "corner": {"E", "S"}}
NAMES = {"river": {"full": "water_river_straight", "bank": "water_river_bank", "corner": "water_river_curve",
                   "inner": "water_river_inner"},
         "pond": {"full": "water_pond", "bank": "water_pond_edge", "corner": "water_pond_corner"}}


def tile_for(conn, dry_diag=()):
    """(kind, clockwise quarter turns) for a set of connected directions. dry_diag: dry
    diagonal corners like 'NE' (only relevant when all four sides are water)."""
    if conn == {"N", "E", "S", "W"} and dry_diag:
        c = "NE"
        for k in range(4):
            if c in dry_diag:
                return "inner", k
            c = CW[c[0]] + CW[c[1]]
            c = c if c in ("NE", "SE", "SW", "NW") else c[::-1]
    for kind, canon in CANON.items():
        c = set(canon)
        for k in range(4):
            if c == conn:
                return kind, k
            c = {CW[d] for d in c}
    raise ValueError(conn)


def sample(objs):
    """Preview-only assembly: 3-wide river bend with bridge + ducks, small pond with lilies,
    frog, reeds and a jetty — autotiled with tile_for."""
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    B = right * 18.0 - fwd * 1.0
    V = zb.Vector
    out = []
    river = {(x, z) for x in range(0, 3) for z in range(0, 7)} | {(x, z) for x in range(0, 6) for z in range(0, 3)}
    pond = {(x, z) for x in range(-8, -4) for z in range(1, 5)}
    for cells, body, org in ((river, "river", V((0, 0, 0))), (pond, "pond", V((0, 0, 0)))):
        for (x, z) in sorted(cells):
            conn = {d for d, (dx, dz) in {"N": (0, 1), "E": (1, 0), "S": (0, -1), "W": (-1, 0)}.items()
                    if (x + dx, z + dz) in cells or (body == "river" and (z + dz > 6 or x + dx > 5))}
            dry = {c for c, (dx, dz) in {"NE": (1, 1), "SE": (1, -1), "SW": (-1, -1), "NW": (-1, 1)}.items()
                   if (x + dx, z + dz) not in cells and not (body == "river" and (z + dz > 6 or x + dx > 5))}
            kind, k = tile_for(conn, dry if conn == {"N", "E", "S", "W"} else ())
            name = NAMES[body][kind]
            out.append(pp.instance(objs[name], f"s_{name}_{x}_{z}", B + org + V((x, z, 0)), -90 * k))
    out.append(pp.instance(objs["bridge_wood"], "s_bridge", B + V((1, 4, 0))))
    for i, (dx, dz, r) in enumerate([(0.9, 1.4, 20), (1.5, 1.9, -30), (3.5, 1.0, 80)]):
        out.append(pp.instance(objs["duck"], f"s_duck{i}", B + V((dx, dz, WATER_Y)), r))
    for i, (dx, dz) in enumerate([(-6.6, 2.2), (-7.2, 3.3), (-5.9, 3.6)]):
        out.append(pp.instance(objs["lily_pad"], f"s_lily{i}", B + V((dx, dz, WATER_Y)), 70 * i))
    out.append(pp.instance(objs["frog"], "s_frog", B + V((-6.2, 1.6, WATER_Y))))
    out.append(pp.instance(objs["jetty_wood"], "s_jetty", B + V((-3.0, 2.5, 0)), 0))
    return out


def main():
    builders = {}
    for body, kinds in NAMES.items():
        for kind, name in kinds.items():
            builders[name] = (lambda k=kind, b=body: water_tile(k, b))
    builders.update({"bridge_wood": bridge_wood, "jetty_wood": jetty_wood, "lily_pad": lily_pad,
                     "duck": duck, "frog": frog})
    pp.run_kit(KIT, builders, cols=4, spacing=(4.2, 3.6), extra=sample)


if __name__ == "__main__":
    main()
