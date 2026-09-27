"""Kit buildings — zookeeper house, night house, food storage, food hut, entrance arch
(concepts: art/props/kit_buildings/sheet_v2.jpg, art/props/food_storage_building/sheet_v2.jpg,
art/environment/env_night_house/overview.png + cutaway.png, env_night_overview/overview.png).

Run:  blender -b --factory-startup --python tools/blender/props/kit_buildings.py -- [--no-preview] [asset ...]

Outputs  assets/models/buildings/<asset>.glb, assets/blender/buildings/kit_buildings.blend,
         art/props/kit_buildings/model_preview.png (day on top, night-tinted below)

Every building is sized to its level rect (level-1.toml / night-1.toml) and its ORIGIN is the
CENTRE OF THAT RECT (night house: of its `model_rect`), on the ground, yaw 0 (front = south =
glTF +Z). Nodes (README_night.md "Buildings"):
- root (asset id)  floor, walls up to 1.0 m, plinths, built-ins, interior dressing
- `walls_upper`    all walls, gables, window frames and panes above 1.0 m (translation y 1.0)
- `roof`           roof + its INNER CEILING surface (down-facing, `ceiling_wood`) + chimney /
                   ceiling lamps (translation y = eave height)
- `glass`          night house only: the glass fronts of the indoor enclosures (`glass` slot)
Zoo view, player inside (PLAY-028): hide `roof` and `walls_upper`. First person (CAMV-022):
keep both — the ceiling and the inner wall faces are modelled.
Doors are separate `door_wood` models (kit_gates); the door openings are listed in
README_night.md with the hinge position for each building. Lit windows = `window_glow`
(night house portholes: `window_blue_glow` / `window_red_glow`).
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

KIT = "kit_buildings"
CUT = 1.0          # walls above this height belong to `walls_upper`
UP = "walls_upper"


# --------------------------------------------------------------------------- helpers

def prism_yz(poly, x0, x1, color):
    """Polygon in the Y/Z plane [(y, z)], extruded along X from x0 to x1."""
    p = pp.prism_xz(poly, -x1, -x0, color)
    p.rotate_z(90)
    return p


def split_box(A, size, center, color, cut=CUT, **kw):
    """Vertical box split at `cut`: lower part -> root, upper part -> walls_upper."""
    sx, sy, sz = size
    cx, cy, cz = center
    z0, z1 = cz - sz / 2, cz + sz / 2
    if z1 <= cut + 1e-6:
        A.add(zb.box(size, center, color=color, **kw))
    elif z0 >= cut - 1e-6:
        A.add(zb.box(size, center, color=color, **kw), node=UP)
    else:
        A.add(zb.box((sx, sy, cut - z0), (cx, cy, (z0 + cut) / 2), color=color, **kw))
        A.add(zb.box((sx, sy, z1 - cut), (cx, cy, (cut + z1) / 2), color=color, **kw), node=UP)


def wall(A, axis, a0, a1, c, t, z1, color, holes=()):
    """Wall slab with rectangular holes, split at CUT into root / walls_upper."""
    A.add(nl.wall_boxes(axis, a0, a1, c, t, 0.0, CUT, color, holes=holes))
    A.add(nl.wall_boxes(axis, a0, a1, c, t, CUT, z1, color, holes=holes), node=UP)


def window(A, axis, c, pos, z0, z1, w, t, glow="window_glow", frame="wood_light", flower_box=None):
    """Window frame + pane (glow slot, both faces) in a wall hole. axis = wall direction;
    c = wall centre line, pos = centre along the wall, t = wall thickness."""
    f = 0.07
    parts = []
    if axis == "x":
        def bx(sa, st, sz, a, zc, off=0.0):
            return zb.box((sa, st, sz), (a, c + off, zc), color=frame)
    else:
        def bx(sa, st, sz, a, zc, off=0.0):
            return zb.box((st, sa, sz), (c + off, a, zc), color=frame)
    zc = (z0 + z1) / 2
    parts.append(bx(f, t + 0.08, z1 - z0 + f, pos - w / 2 - f / 2 + 0.02, zc))
    parts.append(bx(f, t + 0.08, z1 - z0 + f, pos + w / 2 + f / 2 - 0.02, zc))
    parts.append(bx(w + 2 * f, t + 0.1, f, pos, z1 + f / 2 - 0.02))
    parts.append(bx(w + 2 * f, t + 0.14, f, pos, z0 - f / 2 + 0.02))
    parts.append(bx(0.045, t * 0.5, z1 - z0, pos, zc))
    parts.append(bx(w, t * 0.5, 0.045, pos, zc))
    A.add(parts, node=UP)
    if axis == "x":
        pane = zb.box((w, t * 0.3, z1 - z0), (pos, c, zc), color="window_sky")
    else:
        pane = zb.box((t * 0.3, w, z1 - z0), (c, pos, zc), color="window_sky")
    A.add(pane, mat=glow, node=UP)


def porthole(A, axis, c, pos, zc, r, t, glow, day_cell):
    """Round window: wooden ring (both faces) + glowing pane through the wall."""
    if axis == "x":  # wall along X, faces +-Y
        ring = nl.ring_xz(pos, zc, r + 0.1, 12, c - t / 2 - 0.06, c + t / 2 + 0.06, "wood", r_in=r)
        pane = nl.ring_xz(pos, zc, r, 12, c - t / 2 - 0.02, c + t / 2 + 0.02, day_cell)
    else:            # wall along Y, faces +-X
        ring = nl.ring_xz(pos, zc, r + 0.1, 12, -(c + t / 2 + 0.06), -(c - t / 2 - 0.06), "wood", r_in=r)
        pane = nl.ring_xz(pos, zc, r, 12, -(c + t / 2 + 0.02), -(c - t / 2 - 0.02), day_cell)
        ring.rotate_z(90)
        pane.rotate_z(90)
    # cross bars
    A.add(ring, node=UP)
    A.add(pane, mat=glow, node=UP)


def roof_gable(A, x0, x1, half_d, z_eave, z_ridge, thick, color, dark, rows=3, ridge_color=None):
    """Gable roof with the ridge along X: two slabs from y = +-half_d (at z_eave) to the ridge
    (y = 0, z_ridge), shingle rows, ridge cap. Returns the underside function z(y)."""
    ang = math.atan2(z_ridge - z_eave, half_d)
    L = math.hypot(half_d, z_ridge - z_eave) + 0.05
    for s in (-1, 1):
        sl = zb.box((x1 - x0, L, thick), (0, 0, 0), color=color, top_color=color, side_color=dark)
        pp.recolor(sl, lambda f: f.normal.z < -0.3, "ceiling_wood")
        sl.rotate(-s * math.degrees(ang), "X")
        sl.move((x0 + x1) / 2, s * half_d / 2, (z_eave + z_ridge) / 2 + thick / 2 * math.cos(ang))
        A.add(sl, node="roof", keep_down=True)
        for k in range(rows):
            t = (k + 0.6) / (rows + 0.4)
            y = s * half_d * (1 - t)
            z = z_eave + (z_ridge - z_eave) * t + thick * 1.02 / math.cos(ang)
            st = zb.box((x1 - x0 + 0.04, 0.12, 0.04), (0, 0, 0), color=dark)
            st.rotate(-s * math.degrees(ang), "X")
            st.move((x0 + x1) / 2, y, z)
            A.add(st, node="roof")
    A.add(zb.box((x1 - x0 + 0.08, 0.26, 0.16), ((x0 + x1) / 2, 0, z_ridge + thick + 0.02),
                 color=ridge_color or dark, bevel=0.03, bevel_edges="long"), node="roof")

    def under(y):
        return z_eave + (z_ridge - z_eave) * (1 - abs(y) / half_d)
    return under


def ceiling(A, x0, x1, y0, y1, z, color="ceiling_wood"):
    """Flat inner ceiling (a thin slab, its down face is the ceiling seen in first person)."""
    A.add(zb.box((x1 - x0, y1 - y0, 0.04), ((x0 + x1) / 2, (y0 + y1) / 2, z + 0.02), color=color),
          node="roof", keep_down=True)


def planks(A, axis, a0, a1, c, z0, z1, color, step=0.5, node=None):
    """Vertical plank battens on a facade (thin strips, face offset 1 cm)."""
    n = int((a1 - a0) / step)
    for k in range(1, n):
        a = a0 + (a1 - a0) * k / n
        if axis == "x":
            b = zb.box((0.03, 0.02, z1 - z0), (a, c, (z0 + z1) / 2), color=color)
        else:
            b = zb.box((0.02, 0.03, z1 - z0), (c, a, (z0 + z1) / 2), color=color)
        A.add(b, node=node)


# --------------------------------------------------------------------------- zookeeper house

def zookeeper_house():
    """level-1 `zookeeper_house_1` rect (-14, 0, 6, 5), interior (-13, 1, 4, 3), door (-9, 2):
    origin = rect centre (-11, 2.5). Walls 0.3 m: inner faces on the interior rect (x -2.0,
    y +-1.5), east facade (outer face) on the rect edge x = +3.0 (key box / wall lamp at
    level x -7.95). Door opening y -0.47..+0.47 in the east wall. Stone plinth ring on W/S/N."""
    A = nl.Asset("zookeeper_house")
    A.node(UP, pivot=(0, 0, CUT))
    EAVE, RIDGE = 2.5, 3.75
    A.node("roof", pivot=(0, 0, EAVE))
    X0o, X1o = -2.3, 3.0      # outer faces west / east
    Y0o, Y1o = -1.8, 1.8      # outer faces south / north
    T = 0.3
    # plinth ring (stone) and floor
    A.add(zb.box((X1o - (-2.98) - 0.3, 0.68, 0.22), ((-2.98 + X1o - 0.3) / 2 + 0.0, -2.14, 0.11), color="edge_stone",
                 bevel=0.04, bevel_edges="top"))
    A.add(zb.box((X1o - (-2.98) - 0.3, 0.68, 0.22), ((-2.98 + X1o - 0.3) / 2 + 0.0, 2.14, 0.11), color="edge_stone",
                 bevel=0.04, bevel_edges="top"))
    A.add(zb.box((0.66, 3.6, 0.22), (-2.64, 0, 0.11), color="edge_stone", bevel=0.04, bevel_edges="top"))
    # flower strips on the plinth (south and north)
    for sy in (-1, 1):
        for k in range(5):
            x = -2.2 + 1.1 * k
            A.add(zb.knob(0.15, (x, sy * 2.14, 0.26), color="hedge", u=5, v=3, squash=0.8))
            A.add(pp.star((x + 0.05, sy * 2.14 - 0.04, 0.37), 0.06, 0.03, 5,
                          ("flower_red", "flower_yellow", "flower_purple")[k % 3]))
    A.add(zb.box((X1o - X0o - 2 * T + 0.3, Y1o - Y0o - 2 * T, 0.06), ((X0o + T + X1o) / 2, 0, 0.03),
                 color="floor_wood"))
    # walls
    win = (1.0, 2.0)
    # long walls reach up to the roof underside (2.7 m at the outer face)
    wall(A, "x", X0o, X1o, Y0o + T / 2, T, 2.68, "wood", holes=[(0.3, 1.3, *win)])        # south
    wall(A, "x", X0o, X1o, Y1o - T / 2, T, 2.68, "wood", holes=[(0.0, 1.0, *win)])        # north
    wall(A, "y", Y0o + T, Y1o - T, X0o + T / 2, T, EAVE, "wood", holes=[(-0.45, 0.45, *win)])  # west
    wall(A, "y", Y0o + T, Y1o - T, X1o - T / 2, T, EAVE, "wood", holes=[(-0.47, 0.47, 0.0, 2.15)])  # east
    # corner posts (split at 1 m)
    for x in (X0o, X1o):
        for y in (Y0o, Y1o):
            split_box(A, (0.16, 0.16, EAVE), (x + (0.03 if x < 0 else -0.03), y + (0.03 if y < 0 else -0.03),
                                             EAVE / 2), "wood_dark")
    # horizontal plank lines on the outside (south + north + west), just proud of the walls
    for z in (0.5, 1.5, 2.0):
        for y, s in ((Y0o, -1), (Y1o, 1)):
            node = None if z < CUT else UP
            A.add(zb.box((X1o - X0o - 0.2, 0.02, 0.035), ((X0o + X1o) / 2, y + s * 0.005, z), color="wood_dark"),
                  node=node)
    # windows (south with flower box, north, west behind the window_moon prop)
    window(A, "x", Y0o + T / 2, 0.8, win[0], win[1], 1.0, T)
    A.add(zb.box((1.1, 0.22, 0.16), (0.8, Y0o - 0.11, 0.9), color="wood_dark"))
    for k in range(4):
        A.add(zb.knob(0.08, (0.45 + 0.23 * k, Y0o - 0.12, 1.0), color="hedge", u=6, v=3, squash=0.8))
        A.add(pp.star((0.45 + 0.23 * k, Y0o - 0.16, 1.07), 0.05, 0.025, 5,
                      ("flower_red", "flower_yellow")[k % 2]))
    window(A, "x", Y1o - T / 2, 0.5, win[0], win[1], 1.0, T)
    # west window: pane recessed so the window_moon prop on the inner face never shares a plane
    window(A, "y", X0o + T / 2 - 0.02, 0.0, win[0], win[1], 0.9, T - 0.1)
    # east door frame
    for sy in (-1, 1):
        split_box(A, (0.36, 0.1, 2.2), (X1o - T / 2, sy * 0.52, 1.1), "wood_dark")
    A.add(zb.box((0.36, 1.14, 0.12), (X1o - T / 2, 0, 2.21), color="wood_dark"), node=UP)
    A.add(zb.box((0.5, 1.2, 0.06), (X1o + 0.2, 0, 0.03), color="edge_stone"))  # door step
    # gables W / E (above the eaves, under the roof)
    half_d = 2.5
    under = roof_gable(A, -2.62, 3.25, half_d, EAVE - 0.2, RIDGE, 0.14, "roof_red", "roof_red_dark", rows=3)
    for x in (X0o + T / 2, X1o - T / 2):
        poly = [(Y0o, EAVE), (Y1o, EAVE), (Y1o, under(Y1o) - 0.04), (0.0, under(0) - 0.04), (Y0o, under(Y0o) - 0.04)]
        A.add(prism_yz(poly, x - T / 2, x + T / 2, "wood"), node=UP)
    # chimney (roof node)
    A.add(zb.box((0.45, 0.45, 1.3), (-1.3, 0.9, RIDGE - 0.1), color="wall_stone", bevel=0.03,
                 bevel_edges="vertical"), node="roof")
    A.add(zb.box((0.55, 0.55, 0.12), (-1.3, 0.9, RIDGE + 0.56), color="wall_stone_dark"), node="roof")
    ceiling(A, X0o + T, X1o - T, Y0o + T, Y1o - T, EAVE - 0.04)
    # built-ins in the solid strip beside the door (x 2.0 .. 2.7): boot bench + chest of drawers
    A.add(zb.box((0.6, 0.9, 0.45), (2.35, -1.0, 0.225), color="wood_light", bevel=0.02))
    A.add(zb.box((0.6, 0.9, 0.95), (2.35, 1.0, 0.475), color="wood", bevel=0.02))
    for z in (0.3, 0.65):
        A.add(zb.box((0.02, 0.7, 0.22), (2.04, 1.0, z), color="wood_light"))
    # boots on the bench, cap on a hook
    for k in (-1, 1):
        A.add(zb.box((0.12, 0.22, 0.18), (2.3 + 0.08 * k, -1.0 + 0.15 * k, 0.54), color="cart_green_dark",
                     bevel=0.03))
    return A


# --------------------------------------------------------------------------- food storage

def gambrel(x):
    """Food-storage roof profile (underside) across X: eave 2.6 at +-4.15, knee 3.55 at +-2.6,
    ridge 4.3 at 0."""
    ax = abs(x)
    if ax >= 2.6:
        return 2.6 + (4.15 - ax) / 1.55 * 0.95
    return 3.55 + (2.6 - ax) / 2.6 * 0.75


def food_storage():
    """level-1 `food_storage` rect (-4, 11, 8, 6), door cell (0, 11) on the south facade:
    origin = rect centre (0, 14). Barn with a gambrel roof, ridge north-south; south facade
    (outer face) on the rect edge y = -3.0 (the game's "Futter" board is 5 cm in front of
    it). Door opening x 0.03..0.97 (door_wood, hinge at x 0.03). Not enterable (no interior in
    the level data); roof + ceiling + inner wall faces are modelled anyway."""
    A = nl.Asset("food_storage")
    A.node(UP, pivot=(0, 0, CUT))
    EAVE = 2.6
    A.node("roof", pivot=(0, 0, EAVE))
    X, Y0, Y1 = 3.95, -3.0, 2.95
    T = 0.2
    A.add(zb.box((2 * X - 2 * T, Y1 - Y0 - 2 * T, 0.06), (0, (Y0 + Y1) / 2, 0.03), color="floor_wood"))
    for sx in (-1, 1):  # long walls reach up under the roof slab
        wall(A, "y", Y0 + T, Y1 - T, sx * (X - T / 2), T, EAVE + 0.1, "barn_red", holes=[(-0.5, 0.5, 1.2, 2.0)])
    wall(A, "x", -X, X, Y1 - T / 2, T, EAVE, "barn_red")
    wall(A, "x", -X, X, Y0 + T / 2, T, EAVE, "barn_red", holes=[(0.0, 1.0, 0.0, 2.15)])
    # gables (S / N) following the gambrel underside
    xs = [-X, -2.6, 0.0, 2.6, X]
    for yc in (Y0 + T / 2, Y1 - T / 2):
        poly = [(-X, EAVE), (X, EAVE)] + [(x, gambrel(x) - 0.03) for x in reversed(xs)]
        A.add(pp.prism_xz(poly, yc - T / 2, yc + T / 2, "barn_red"), node=UP)
    # white trim: corner posts, door frame, gable edge boards
    for x in (-X, X):
        for y in (Y0, Y1):
            split_box(A, (0.18, 0.18, EAVE), (x - 0.04 * (1 if x > 0 else -1), y + (0.04 if y < 0 else -0.04),
                                             EAVE / 2), "trim_white")
    for x in (-0.05, 1.05):
        split_box(A, (0.12, T + 0.08, 2.2), (x, Y0 + T / 2, 1.1), "trim_white")
    A.add(zb.box((1.22, T + 0.08, 0.12), (0.5, Y0 + T / 2, 2.21), color="trim_white"), node=UP)
    # vertical plank battens on the long facades (the south facade stays flat for the board)
    for x in (-X - 0.01, X + 0.01):
        for zr, node in (((0.0, CUT), None), ((CUT, EAVE), UP)):
            planks(A, "y", Y0 + 0.2, Y1 - 0.2, x, zr[0], zr[1], "barn_red_dark", step=0.6, node=node)
    # round attic window (south gable) + windows on east / west
    porthole(A, "x", Y0 + T / 2, 0.0, 3.55, 0.3, T, "window_glow", "window_sky")
    window(A, "y", -X + T / 2, 0.0, 1.2, 2.0, 1.0, T, frame="trim_white")
    window(A, "y", X - T / 2, 0.0, 1.2, 2.0, 1.0, T, frame="trim_white")
    # gambrel roof: 4 slabs along Y + ridge cap, eave trim
    yl0, yl1 = Y0 - 0.35, Y1 + 0.35
    pts = [(-X - 0.05, gambrel(X) - 0.06), (-2.6, gambrel(2.6)), (0.0, gambrel(0.0))]
    th = 0.14
    for s in (-1, 1):
        for (xa, za), (xb, zb_) in ((pts[0], pts[1]), (pts[1], pts[2])):
            xa2, xb2 = s * xa, s * xb
            poly = [(xa2, za), (xb2, zb_), (xb2, zb_ + th), (xa2, za + th)]
            if s > 0:
                poly = list(reversed(poly))
            slab = pp.prism_xz(poly, yl0, yl1, "roof_brown")
            pp.recolor(slab, lambda f: f.normal.z < -0.2, "ceiling_wood")
            pp.recolor(slab, lambda f: abs(f.normal.y) > 0.9, "trim_white")
            A.add(slab, node="roof", keep_down=True)
            # shingle rows
            for k in (0.33, 0.66):
                x = xa2 + (xb2 - xa2) * k
                z = za + (zb_ - za) * k + th + 0.01
                A.add(zb.box((0.1, yl1 - yl0 + 0.02, 0.035), (x, (yl0 + yl1) / 2, z), color="roof_brown_dark"),
                      node="roof")
    A.add(zb.box((0.3, yl1 - yl0 + 0.06, 0.12), (0, (yl0 + yl1) / 2, gambrel(0) + th + 0.03),
                 color="roof_brown_dark", bevel=0.03, bevel_edges="long"), node="roof")
    ceiling(A, -X + T, X - T, Y0 + T, Y1 - T, EAVE - 0.04)
    # a few crates inside (seen when the roof is hidden)
    for i, (x, y) in enumerate(((-3.2, 2.2), (-2.5, 2.2), (-3.2, 1.5), (3.1, 2.2), (2.4, 2.2))):
        A.add(zb.box((0.6, 0.6, 0.6), (x, y, 0.3), color="wood_light", side_color="wood"))
    return A


# --------------------------------------------------------------------------- food hut

def food_hut():
    """night-1 `food_storage_n1` (kind food_hut) rect (-44, 26, 5, 6), door (-40, 29) on the
    east facade: origin = rect centre (-41.5, 29). Small plank hut, gable roof with the ridge
    north-south; east facade (outer face) on the rect edge x = +2.5 (wall lamp at level x
    -38.95). Door opening y 0.03..0.97 in the east wall (door_wood). Lit windows on the east
    (serving hatch), south and west."""
    A = nl.Asset("food_hut")
    A.node(UP, pivot=(0, 0, CUT))
    EAVE, RIDGE = 2.3, 3.4
    A.node("roof", pivot=(0, 0, EAVE))
    X0, X1, Y0, Y1 = -2.45, 2.5, -2.95, 2.95
    T = 0.2
    A.add(zb.box((X1 - X0 - 2 * T, Y1 - Y0 - 2 * T, 0.06), ((X0 + X1) / 2, 0, 0.03), color="floor_wood"))
    wall(A, "x", X0, X1, Y0 + T / 2, T, EAVE, "house_wood", holes=[(-0.5, 0.5, 1.0, 1.9)])
    wall(A, "x", X0, X1, Y1 - T / 2, T, EAVE, "house_wood")
    wall(A, "y", Y0 + T, Y1 - T, X0 + T / 2, T, EAVE, "house_wood", holes=[(-0.5, 0.5, 1.0, 1.9)])
    wall(A, "y", Y0 + T, Y1 - T, X1 - T / 2, T, EAVE, "house_wood",
         holes=[(0.03, 0.97, 0.0, 2.15), (-2.1, -0.6, 1.0, 1.8)])
    for yc in (Y0 + T / 2, Y1 - T / 2):
        poly = [(X0, EAVE), (X1, EAVE), (X1, EAVE + 0.1), ((X0 + X1) / 2, RIDGE - 0.1), (X0, EAVE + 0.1)]
        A.add(pp.prism_xz(poly, yc - T / 2, yc + T / 2, "house_wood"), node=UP)
    for x in (X0, X1):
        for y in (Y0, Y1):
            split_box(A, (0.16, 0.16, EAVE), (x + (0.03 if x < 0 else -0.03), y + (0.03 if y < 0 else -0.03),
                                             EAVE / 2), "house_wood_dark")
    for zr, node in (((0.0, CUT), None), ((CUT, EAVE), UP)):
        planks(A, "x", X0 + 0.1, X1 - 0.1, Y0 - 0.01, zr[0], zr[1], "house_wood_dark", step=0.45, node=node)
        planks(A, "y", Y0 + 0.1, Y1 - 0.1, X0 - 0.01, zr[0], zr[1], "house_wood_dark", step=0.45, node=node)
    window(A, "x", Y0 + T / 2, 0.0, 1.0, 1.9, 1.0, T)
    window(A, "y", X0 + T / 2, 0.0, 1.0, 1.9, 1.0, T)
    # serving hatch on the east facade: lit window with a counter and a little awning
    window(A, "y", X1 - T / 2, -1.35, 1.0, 1.8, 1.5, T)
    A.add(zb.box((0.35, 1.7, 0.06), (X1 + 0.15, -1.35, 0.97), color="wood_light"))
    aw = zb.box((0.5, 1.8, 0.05), (0, 0, 0), color="stripe_red")
    aw.rotate(-25, "Y").move(X1 + 0.22, -1.35, 2.05)
    A.add(aw, node=UP)
    for sy in (-1, 1):
        split_box(A, (T + 0.08, 0.1, 2.2), (X1 - T / 2, 0.5 + sy * 0.52, 1.1), "house_wood_dark")
    A.add(zb.box((T + 0.08, 1.14, 0.12), (X1 - T / 2, 0.5, 2.21), color="house_wood_dark"), node=UP)
    # gable roof with the ridge along Y: build along X then rotate
    half_d = (X1 - X0) / 2 + 0.35
    cx = (X0 + X1) / 2
    B = nl.Asset("_tmp")
    B.node("roof")
    roof_gable(B, Y0 - 0.35, Y1 + 0.35, half_d, EAVE - 0.2, RIDGE, 0.12, "roof_brown", "roof_brown_dark", rows=2)
    for mat, parts, keep in B.nodes["roof"]["groups"]:
        for p in parts:
            p.rotate_z(90)
            p.move(cx, 0, 0)
        A.add(parts, mat=mat, node="roof", keep_down=keep)
    ceiling(A, X0 + T, X1 - T, Y0 + T, Y1 - T, EAVE - 0.04)
    # sacks and crates inside
    for x, y in ((-1.8, 2.3), (-1.2, 2.3), (-1.8, 1.7)):
        A.add(zb.box((0.5, 0.5, 0.5), (x, y, 0.25), color="wood_light", side_color="wood"))
    A.add(zb.knob(0.3, (1.6, 2.2, 0.3), color="straw", u=8, v=5, squash=1.1))
    return A


# --------------------------------------------------------------------------- entrance arch

def entrance_arch():
    """level-1 `entrance_gate` rect (-3, -2, 6, 2): origin = rect centre (0, -1). Two stone
    pillars 1.2 x 1.6 m (inner faces x = +-1.8, the 3.6 m passage of the placeholder), wooden
    caps and posts, a curved beam (top 5.0 m) and a blank sign board (`sign_face`, 2.6 x 0.8 m,
    front = south). Turnstiles are separate (`turnstile`, kit_gates). No two faces share a
    plane (ARCH-005): caps, posts and beam all have distinct extents."""
    A = nl.Asset("entrance_arch")
    PH = 3.3
    for sx in (-1, 1):
        cx = sx * 2.4
        A.add(zb.box((1.1, 1.5, PH), (cx, 0, PH / 2), color="wall_mortar"))
        for k in range(5):
            z = PH * (k + 0.5) / 5
            for s in (-1, 1):
                if k % 2 == 0:
                    c, size = (cx + s * 0.3, 0, z), (0.58, 1.6, PH / 5 - 0.05)
                else:
                    c, size = (cx, s * 0.4, z), (1.2, 0.78, PH / 5 - 0.05)
                A.add(zb.box(size, c, color=("wall_stone", "wall_stone_dark")[(k + (s > 0)) % 2], bevel=0.06,
                             bevel_edges="vertical"))
        A.add(zb.box((1.36, 1.76, 0.22), (cx, 0, PH + 0.11), color="wood", bevel=0.04, bevel_edges="top+vertical"))
        A.add(zb.box((0.5, 0.6, 0.9), (cx, 0, PH + 0.22 + 0.45), color="wood_light", bevel=0.03,
                     bevel_edges="vertical"))
        A.add(zb.box((0.66, 0.76, 0.14), (cx, 0, PH + 0.22 + 0.6), color="wood_dark"))
    # curved beam: arc from x -3.0 (z 4.35) over the middle (z 4.75) — thickness 0.3, depth 0.5
    n = 8
    top = [(-3.05 + 6.1 * i / n, 4.35 + 0.4 * math.sin(math.pi * i / n)) for i in range(n + 1)]
    poly = [(x, z) for x, z in top] + [(x, z - 0.42) for x, z in reversed(top)]
    poly = [(x, z + 0.25) for x, z in poly]
    beam = pp.prism_xz(poly, -0.28, 0.28, "wood")
    pp.recolor(beam, lambda f: f.normal.z > 0.5, "wood_light")
    A.add(beam)
    # sign board hanging under the beam: frame + blank face (front) + plain back
    bw, bh = 2.6, 0.8
    bz = 3.55
    A.add(zb.box((bw + 0.16, 0.1, bh + 0.16), (0, 0, bz + bh / 2), color="wood_dark", bevel=0.02))
    A.add(nl.face_quad([(-bw / 2, FRONT * 0.052, bz), (bw / 2, FRONT * 0.052, bz),
                        (bw / 2, FRONT * 0.052, bz + bh), (-bw / 2, FRONT * 0.052, bz + bh)], "sign_face"),
          mat="sign_face")
    for sx in (-1, 1):  # hangers
        A.add(pp.beam((sx * 0.9, 0, bz + bh + 0.08), (sx * 0.9, 0, 4.25 + 0.4 * math.sin(math.pi * (0.5 + sx * 0.15))),
                      0.05, color="iron"))
    return A


# --------------------------------------------------------------------------- night house

NH_X, NH_Y0, NH_Y1 = 8.5, -6.5, 6.5
NH_EAVE_EDGE = 2.0
NH_CROWN = 4.55
NH_SPAN = 8.8


def nh_under(x):
    """Underside of the vaulted night-house roof across X (elliptic arch)."""
    t = min(1.0, abs(x) / NH_SPAN)
    return NH_EAVE_EDGE + (NH_CROWN - NH_EAVE_EDGE) * math.sqrt(1 - t * t)


def arch_wall(A, y0, y1, x0, x1, color, hole=None, z_bottom=CUT):
    """Upper part (z >= CUT) of a gable wall following the vault, from x0 to x1, optional
    rectangular hole (hx0, hx1, hz_top) that starts below CUT."""
    def seg(a, b):
        n = max(2, int(abs(b - a) / 1.2))
        xs = [a + (b - a) * i / n for i in range(n + 1)]
        poly = [(a, z_bottom), (b, z_bottom)] + [(x, nh_under(x) - 0.03) for x in reversed(xs)]
        A.add(pp.prism_xz(poly, y0, y1, color), node=UP)
    if hole is None:
        seg(x0, x1)
    else:
        hx0, hx1, hz = hole
        seg(x0, hx0)
        seg(hx1, x1)
        n = 2
        xs = [hx0 + (hx1 - hx0) * i / n for i in range(n + 1)]
        poly = [(hx0, hz), (hx1, hz)] + [(x, nh_under(x) - 0.03) for x in reversed(xs)]
        A.add(pp.prism_xz(poly, y0, y1, color), node=UP)


def fern(A, x, y, s=1.0, color="fern", seed=0):
    import random
    rnd = random.Random(seed)
    for k in range(5):
        a = 2 * math.pi * k / 5 + rnd.uniform(-0.3, 0.3)
        A.add(pp.leaf((x, y, 0.06), (math.cos(a), math.sin(a), 0.9), 0.45 * s, 0.16 * s, color, droop=0.1 * s))


def night_house():
    """night-1 `night_house` model_rect (-45, 39, 17, 13): origin = its centre (-36.5, 45.5).
    Hall = rect (-45, 39, 17, 5) (y -6.5..-1.5), interior (-44, 40, 15, 4), door (-37, 39) on
    the south facade (opening x -0.47..+0.47). Indoor enclosures behind the glass front at
    y = -1.5: hedgehog x -8.5..-2.5 (gate x -6.5..-4.5), bat -2.5..2.5 (gate -1.5..0.5), owl
    2.5..8.5 (gate 4.5..6.5) — the gates are left open for `glass_door`. Vaulted grass roof
    (ridge along Y) with a wooden ceiling underneath, painted moon and stars on the south
    facade, blue and red-orange portholes, ceiling lamps per room."""
    A = nl.Asset("night_house")
    A.node(UP, pivot=(0, 0, CUT))
    A.node("roof", pivot=(0, 0, NH_EAVE_EDGE))  # the vault springs at 2.0 m (x = +-8.8)
    A.node("glass", pivot=(0, -1.5, 0))
    T = 0.3
    X = NH_X
    # floors
    A.add(zb.box((2 * X - 2 * T, 4.7 - 0.0, 0.06), (0, (-6.2 - 1.5) / 2, 0.03), color="floor_wood"))
    for x0, x1, col in ((-X + T, -2.6, "earth"), (-2.4, 2.4, "earth"), (2.6, X - T, "grass_dark")):
        A.add(zb.box((x1 - x0, 7.6, 0.08), ((x0 + x1) / 2, (-1.4 + 6.2) / 2, 0.04), color=col))
    # outer walls
    wall(A, "x", -X, X, NH_Y0 + T / 2, T, CUT, "house_wood", holes=[(-0.47, 0.47, 0.0, 2.2)])
    wall(A, "x", -X, X, NH_Y1 - T / 2, T, CUT, "house_wood")
    side_h = nh_under(X - T / 2) - 0.03
    for sx in (-1, 1):
        wall(A, "y", NH_Y0 + T, NH_Y1 - T, sx * (X - T / 2), T, side_h, "house_wood")
    arch_wall(A, NH_Y0, NH_Y0 + T, -X, X, "house_wood", hole=(-0.47, 0.47, 2.2))
    arch_wall(A, NH_Y1 - T, NH_Y1, -X, X, "house_wood")
    # door frame (arched look: posts + lintel)
    for sx in (-1, 1):
        split_box(A, (0.12, T + 0.1, 2.3), (sx * 0.53, NH_Y0 + T / 2, 1.15), "house_wood_dark")
    A.add(zb.box((1.18, T + 0.1, 0.14), (0, NH_Y0 + T / 2, 2.27), color="house_wood_dark"), node=UP)
    # facade battens (south + sides), split at 1 m
    for zr, node in (((0.0, CUT), None), ((CUT, 2.2), UP)):
        for a0, a1 in ((-X + 0.2, -0.7), (0.7, X - 0.2)):
            planks(A, "x", a0, a1, NH_Y0 - 0.01, zr[0], zr[1], "house_wood_dark", step=1.4, node=node)
    # painted crescent moon + stars above the door (the game's name board hangs at 2.25-3.05 m)
    pts = []
    n = 9
    for i in range(n + 1):
        a = math.radians(60 + 240 * i / n)
        pts.append((0.7 * math.cos(a) - 0.05, 3.75 + 0.62 * math.sin(a)))
    for i in range(n + 1):
        a = math.radians(300 - 240 * i / n)
        pts.append((0.52 * math.cos(a) + 0.2, 3.85 + 0.5 * math.sin(a)))
    A.add(pp.prism_xz(pts, NH_Y0 - 0.02, NH_Y0 + 0.01, "moon_cream"), node=UP)
    for (sx, sz, r) in ((-1.7, 3.45, 0.22), (1.55, 3.55, 0.24), (-1.1, 4.2, 0.15), (1.0, 4.25, 0.16),
                        (-2.6, 3.0, 0.15), (2.5, 3.0, 0.16)):
        spts = []
        for i in range(10):
            rr = r if i % 2 == 0 else r * 0.45
            a = math.pi / 2 + math.pi * i / 5
            spts.append((sx + rr * math.cos(a), sz + rr * math.sin(a)))
        A.add(pp.prism_xz(spts, NH_Y0 - 0.02, NH_Y0 + 0.01, "star_paint"), node=UP)
    # portholes: south facade blue (left) / red (right), sides blue (hall) and per enclosure
    porthole(A, "x", NH_Y0 + T / 2, -3.2, 1.6, 0.45, T, "window_blue_glow", "porthole_blue")
    porthole(A, "x", NH_Y0 + T / 2, 3.2, 1.6, 0.45, T, "window_red_glow", "porthole_red")
    porthole(A, "x", NH_Y0 + T / 2, -6.4, 1.5, 0.35, T, "window_blue_glow", "porthole_blue")
    porthole(A, "x", NH_Y0 + T / 2, 6.4, 1.5, 0.35, T, "window_red_glow", "porthole_red")
    for sx in (-1, 1):
        porthole(A, "y", sx * (X - T / 2), -4.0, 1.6, 0.4, T, "window_blue_glow", "porthole_blue")
        porthole(A, "y", sx * (X - T / 2), 2.5, 1.6, 0.4, T,
                 "window_red_glow" if sx < 0 else "window_blue_glow",
                 "porthole_red" if sx < 0 else "porthole_blue")
    # partitions between the enclosures
    for x in (-2.5, 2.5):
        wall(A, "y", -1.5, NH_Y1 - T, x, 0.2, nh_under(x) - 0.03, "house_wood_dark")
    # glass front at y = -1.5: posts, kick boards, top beam, header wall; panes in `glass`
    posts = [-X + T, -6.5, -4.5, -2.5, -1.5, 0.5, 2.5, 4.5, 6.5, X - T]
    gates = [(-6.5, -4.5), (-1.5, 0.5), (4.5, 6.5)]
    for x in posts:
        split_box(A, (0.15, 0.18, 2.45), (x, -1.5, 1.225), "house_wood_dark")
    GT = 2.45
    A.add(zb.box((2 * X - 2 * T, 0.2, 0.16), (0, -1.5, GT + 0.08), color="house_wood_dark"), node=UP)
    xs_h = [(-X + T) + (2 * X - 2 * T) * i / 10 for i in range(11)]
    poly = [(-X + T, GT + 0.16), (X - T, GT + 0.16)] + [(x, nh_under(x) - 0.03) for x in reversed(xs_h)]
    A.add(pp.prism_xz(poly, -1.58, -1.42, "house_wood"), node=UP)
    panes = []
    for a, b in zip(posts, posts[1:]):
        if (a, b) in gates:
            continue
        A.add(zb.box((b - a - 0.15, 0.1, 0.5), ((a + b) / 2, -1.5, 0.25), color="house_wood_dark"))
        A.add(zb.box((b - a - 0.1, 0.16, 0.06), ((a + b) / 2, -1.5, 0.52), color="house_wood_dark"))
        panes.append(zb.box((b - a - 0.15, 0.02, GT - 0.55), ((a + b) / 2, -1.5, 0.55 + (GT - 0.55) / 2),
                            color="glass_tint"))
    A.add(panes, mat="glass", node="glass", keep_down=True)
    # low wooden rail in front of the glass (hall side), like the cut-away sheet
    for a, b in zip(posts, posts[1:]):
        if (a, b) in gates:
            continue
        A.add(zb.box((b - a - 0.1, 0.08, 0.08), ((a + b) / 2, -1.85, 0.8), color="house_wood"))
        A.add(zb.box((0.08, 0.08, 0.8), (a + 0.12, -1.85, 0.4), color="house_wood"))
        A.add(zb.box((0.08, 0.08, 0.8), (b - 0.12, -1.85, 0.4), color="house_wood"))
    # planters along the hall walls (the solid band cells between wall and hall interior)
    for x0, x1 in ((-X + T, -1.0), (1.0, X - T)):
        A.add(zb.box((x1 - x0, 0.6, 0.4), ((x0 + x1) / 2, NH_Y0 + T + 0.3, 0.2), color="house_wood_dark",
                     top_color="earth"))
        for k in range(int((x1 - x0) / 1.6)):
            fern(A, x0 + 0.8 + 1.6 * k, NH_Y0 + T + 0.3, s=0.8, seed=k)
    for sx in (-1, 1):
        A.add(zb.box((0.6, 3.9, 0.4), (sx * (X - T - 0.3), -3.6, 0.2), color="house_wood_dark", top_color="earth"))
    # ---- vaulted grass roof with the wooden ceiling underneath (roof node)
    n = 14
    xs = [-NH_SPAN + 2 * NH_SPAN * i / n for i in range(n + 1)]
    poly = [(x, nh_under(x)) for x in xs] + [(x, nh_under(x) + 0.35) for x in reversed(xs)]
    vault = pp.prism_xz(list(reversed(poly)), NH_Y0 - 0.3, NH_Y1 + 0.3, "grass_roof")
    pp.recolor(vault, lambda f: f.normal.z < -0.05 and abs(f.normal.y) < 0.5, "ceiling_wood")
    pp.recolor(vault, lambda f: abs(f.normal.y) > 0.9, "house_wood_dark")
    A.add(vault, node="roof", keep_down=True)
    # ceiling lamps (roof node): hall warm, hedgehog red-orange, bat / owl blue
    lamps = [((-0.5, -4.0), "lamp_glow", "light_hall", (4.0, "#FFC46E")),
             ((-5.5, 2.0), "window_red_glow", "light_hedgehog", (3.5, "#E8735A")),
             ((0.0, 2.0), "window_blue_glow", "light_bat", (3.5, "#5B7FE0")),
             ((5.5, 2.0), "window_blue_glow", "light_owl", (3.5, "#5B7FE0"))]
    for (x, y), mat, nm, light in lamps:
        z = nh_under(x)
        A.add(pp.beam((x, y, z), (x, y, z - 0.5), 0.03, color="iron"), node="roof")
        A.add(pp.cyl(0.08, z - 0.75, z - 0.5, sides=8, r1=0.22, center=(x, y), color="iron"), node="roof")
        A.add(pp.disc((x, y, z - 0.74), 0.2, "lamp_glass", sides=8), mat=mat, node="roof", keep_down=True)
        A.empty(nm, (x, y, z - 0.9), light=light)
    # ---- indoor enclosure dressing
    # hedgehog: straw nest box, hollow log, ferns, earth mounds
    A.add(zb.box((0.9, 0.7, 0.55), (-7.3, 5.3, 0.33), color="wood", top_color="straw"))
    A.add(nl.ring_xz(-7.3, 0.3, 0.16, 8, 4.93, 4.95, "trunk_dark"))
    log = pp.cyl(0.28, 0, 1.4, sides=8, color="trunk", top=False)
    log.rotate(90, "Y").move(-5.8, 3.4, 0.36)
    A.add(log)
    for k, (x, y) in enumerate(((-4.2, 5.2), (-3.4, 1.2), (-7.4, 1.0), (-5.0, 0.3))):
        fern(A, x, y, s=1.1, seed=10 + k)
    A.add(zb.knob(0.6, (-3.8, 3.2, 0.0), color="earth", u=8, v=4, squash=0.35))
    # bat: branches under the ceiling, hanging ropes, shelf with a fruit bowl
    for (a, b) in (((-2.3, 1.0, 3.3), (2.3, 1.6, 3.5)), ((-2.3, 4.2, 3.4), (2.3, 3.4, 3.3)),
                   ((-0.8, 0.2, 3.35), (0.4, 5.8, 3.45))):
        A.add(pp.beam(a, b, 0.12, sides=6, color="trunk"))
    for x, y in ((-1.2, 1.2), (0.9, 3.6), (-0.3, 4.3)):
        A.add(pp.beam((x, y, 3.3), (x, y, 1.6), 0.035, sides=4, color="straw"))
    A.add(zb.box((1.4, 0.4, 0.06), (0.0, NH_Y1 - T - 0.2, 1.5), color="wood_light"), node=UP)
    A.add(pp.cyl(0.22, 1.53, 1.65, sides=8, r1=0.26, center=(0.0, NH_Y1 - T - 0.2), color="clay_pot"), node=UP)
    for k, c in enumerate(("flower_red", "flower_yellow", "carrot_orange")):
        A.add(zb.knob(0.08, (-0.1 + 0.1 * k, NH_Y1 - T - 0.2, 1.7), color=c, u=6, v=3), node=UP)
    # owl: perch poles, owl box high on the back wall, painted moon on the back wall
    for x, y, h in ((4.2, 2.0, 2.2), (6.8, 3.6, 2.6)):
        A.add(zb.box((0.14, 0.14, h), (x, y, h / 2), color="trunk"))
        A.add(pp.beam((x - 0.8, y, h - 0.2), (x + 0.8, y, h - 0.2), 0.1, sides=6, color="trunk"))
    A.add(zb.box((0.7, 0.6, 0.8), (6.0, NH_Y1 - T - 0.3, 2.6), color="wood"), node=UP)
    A.add(nl.ring_xz(6.0, 2.65, 0.14, 8, NH_Y1 - T - 0.61, NH_Y1 - T - 0.59, "trunk_dark"), node=UP)
    mpts = [(4.0 + 0.5 * math.cos(math.radians(a)), 3.2 + 0.5 * math.sin(math.radians(a))) for a in range(0, 360, 30)]
    A.add(pp.prism_xz(mpts, NH_Y1 - T - 0.015, NH_Y1 - T + 0.005, "moon_cream"), node=UP)
    for k, (x, y) in enumerate(((3.3, 0.4), (7.6, 0.6), (5.2, 5.0))):
        fern(A, x, y, s=1.0, seed=20 + k)
    return A


# --------------------------------------------------------------------------- preview extras

def extra(roots):
    """Zookeeper house with the roof / upper walls hidden (zoo view, player inside) and the
    bedroom props placed as in level-1.toml; night house cut away the same way."""
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    V = Vector
    out = []
    zk = nl.instance_tree(roots["zookeeper_house"], "s_zk", right * 44.0 + fwd * 4.0)
    for o in nl.descendants(zk):
        if o.name.endswith("roof") or o.name.endswith(UP):
            o.hide_render = True
    out.append(zk)
    nh = nl.instance_tree(roots["night_house"], "s_nh", right * 44.0 - fwd * 16.0)
    for o in nl.descendants(nh):
        if o.name.endswith("roof") or o.name.endswith(UP):
            o.hide_render = True
    out.append(nh)
    try:
        import bpy
        base = zk.location
        items = [("bed", (-12.0, 1.5), 180), ("night_table", (-12.72, 2.3), 90), ("rug_round", (-11.0, 2.4), 0),
                 ("toy_chest", (-12.45, 3.72), 0), ("desk", (-10.5, 3.7), 0), ("door_wood", (-8.15, 2.03), 90),
                 ("window_moon", (-13.0, 2.5), 90), ("key_box", (-8.0, 1.5), 90), ("wall_lamp", (-8.0, 3.4), 90)]
        mount = {"window_moon": 1.0, "key_box": 1.0, "wall_lamp": 1.6}
        for nm, (lx, lz), yaw in items:
            bpy.ops.import_scene.gltf(filepath=zb.repo_path("assets", "models", "props", nm + ".glb"))
            sel = [o for o in bpy.context.selected_objects if o.parent is None]
            for o in sel:
                o.rotation_mode = "XYZ"
                o.rotation_euler = (0, 0, math.radians(yaw))
                o.location = base + V((lx + 11.0, lz - 2.5, mount.get(nm, 0.06)))
                out.append(o)
    except Exception as e:  # noqa: BLE001
        print("preview: furniture not loaded:", e)
    return out


def main():
    builders = {
        "zookeeper_house": zookeeper_house,
        "food_storage": food_storage,
        "food_hut": food_hut,
        "entrance_arch": entrance_arch,
        "night_house": night_house,
    }
    nl.run(KIT, builders, cols=3, spacing=(14.0, 18.0), folder="buildings",
           budgets={"*": 1500, "night_house": 4000}, extra=extra)


if __name__ == "__main__":
    main()
