"""Kit terrarium — the real models of the terrarium house of night_2 (concept: art/environment/
env_terrarium_house/overview_v2.jpg + cutaway_v2.jpg, approved; GAME-LEVEL-NIGHT-2, AENV-017).

Run:  blender -b --factory-startup --python tools/blender/props/kit_terrarium.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/<asset>.glb, assets/blender/props/kit_terrarium.blend,
         art/props/kit_terrarium/model_preview*.png (day on top, night-tinted below)

Placement table, frames and the node list of every asset: tools/blender/props/README_terrarium.md.

Frames
- HOUSE: `terrarium_house` is authored in the house frame: origin = middle of the south
  facade, front (glTF +Z) = the street (level south), x = level east, Blender +Y = level north
  (the apex of the half disc is 11 m behind the door). Level position of a house point (hx, hy):
  (-84.5 + hx, 39 + hy). Footprint = round_house.rs (model_rect -94,39,19,11).
- CASE PROPS: authored in the *viewer frame of a case*: front (glTF +Z, Blender -Y) = the glass
  side, x = to the viewer's right, Blender +Y = into the case. The wiring turns each case by its
  yaw: snake +90 (glass east), chameleon 0 (glass south), frog -90 (glass west).
- Floors: origin = floor centre, top at z = 0.08. Props stand ON the floor (placed at y = 0.08).
- Wall pieces (lamp, info_board_wall): wall plane at glTF z = 0, stick out to +Z, origin = their
  lowest point (the game lifts them to the mount height).
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bmesh  # noqa: E402
import bpy  # noqa: E402
import night_lib as nl  # noqa: E402
import props_parts as pp  # noqa: E402
from night_lib import zb, FRONT, BACK  # noqa: E402
from mathutils import Vector  # noqa: E402

KIT = "kit_terrarium"
FLOOR_TOP = 0.08

# --------------------------------------------------------------------------- small helpers


def stone(cx, cy, rx, ry, h, color, seed=0, n=6, z0=0.0, chamfer=None, top_color=None, side_color=None):
    """Flat irregular stone (n-gon slab, domed by a full-height chamfer)."""
    rnd = random.Random(seed)
    poly = []
    for i in range(n):
        a = 2 * math.pi * i / n + rnd.uniform(-0.18, 0.18)
        k = rnd.uniform(0.82, 1.1)
        poly.append((cx + rx * k * math.cos(a), cy + ry * k * math.sin(a)))
    return zb.slab(poly, z0, z0 + h, color, chamfer=h if chamfer is None else chamfer,
                   top_color=top_color, side_color=side_color)


def lump(cx, cy, rx, ry, color, seed=0, z0=0.0, h=0.03, n=7):
    """Flat low patch (sand / moss / soil) — a slab with a small chamfer."""
    return stone(cx, cy, rx, ry, h, color, seed=seed, n=n, z0=z0, chamfer=h)


def quad(points, color):
    """Single flat quad/polygon facing the side its points are counter-clockwise to."""
    return pp.flat(points, color)


def double(points, color):
    """Both sides of a flat polygon (two faces)."""
    a = pp.flat(points, color)
    b = pp.flat(list(reversed(points)), color)
    return [a, b]


def leaf_tent(base, direction, length, width, lift, droop, color):
    """A closed double-sided leaf: a low ridge 'tent' (8 tris) from base to a drooping tip."""
    b = Vector(base)
    d = Vector((direction[0], direction[1], 0)).normalized()
    s = Vector((-d.y, d.x, 0))
    mid = b + d * length * 0.5 + Vector((0, 0, lift))
    tip = b + d * length + Vector((0, 0, lift * 0.3 - droop))
    left, right = mid - s * width / 2 - Vector((0, 0, 0.02)), mid + s * width / 2 - Vector((0, 0, 0.02))
    ridge = mid + Vector((0, 0, 0.045))
    bm = zb._new_bm()
    vb, vl, vt, vr, vk = (bm.verts.new(v) for v in (b, left, tip, right, ridge))
    for tri in ((vb, vl, vk), (vl, vt, vk), (vt, vr, vk), (vr, vb, vk)):
        bm.faces.new(tri)
    # underside: reversed winding on duplicate verts (a separate fan, so no shared edges merge them)
    under = [bm.verts.new(v) for v in (b, left, tip, right, ridge)]
    for i, j in ((0, 1), (1, 2), (2, 3), (3, 0)):
        bm.faces.new((under[j], under[i], under[4]))
    return pp._finish(bm, color)


def log_beam(p0, p1, w, color, sides=6, taper=1.0):
    return pp.beam(p0, p1, w, color=color, sides=sides, taper=taper)


# --------------------------------------------------------------------------- the house

# outer footprint (house frame, m): the convex hull of the stepped half disc of round_house.rs
HULL = [(-9.5, 0), (9.5, 0), (9.5, 7), (8.5, 8), (7.5, 9), (5.5, 10), (3.5, 11),
        (-3.5, 11), (-5.5, 10), (-7.5, 9), (-8.5, 8), (-9.5, 7)]
# wall masses between hall, cases and the outer arc (every polygon CCW)
MASSES = [
    [(-9.5, 0), (-0.5, 0), (-0.5, 1), (-3.5, 1), (-3.5, 2), (-9.5, 2)],      # south left
    [(0.5, 0), (9.5, 0), (9.5, 2), (3.5, 2), (3.5, 1), (0.5, 1)],            # south right
    [(-9.5, 6), (-3.5, 6), (-3.5, 11), (-5.5, 10), (-7.5, 9), (-8.5, 8), (-9.5, 7)],   # north west
    [(3.5, 6), (9.5, 6), (9.5, 7), (8.5, 8), (7.5, 9), (5.5, 10), (3.5, 11)],          # north east
]
WALL_H = 3.15
LOW_H = 1.0
HALL = [(-0.5, 0), (0.5, 0), (0.5, 1), (3.5, 1), (3.5, 2), (4.5, 2), (4.5, 6), (3.5, 6), (3.5, 7),
        (-3.5, 7), (-3.5, 6), (-4.5, 6), (-4.5, 2), (-3.5, 2), (-3.5, 1), (-0.5, 1)]
EMBLEM_Z = 4.0


def scaled(poly, s, c=(0.0, 5.5)):
    return [(c[0] + (x - c[0]) * s, c[1] + (y - c[1]) * s) for x, y in poly]


def window(cx, cy, normal_deg, w, z0=1.25, z1=2.5):
    """Framed warm window on the outer wall; built facing -Y, turned to `normal_deg` (the
    direction its outer face looks to, degrees, 270 = -Y = south)."""
    fr = [zb.box((w + 0.22, 0.1, z1 - z0 + 0.22), (0, -0.05, (z0 + z1) / 2), color="wood_light"),
          zb.box((w + 0.34, 0.16, 0.1), (0, -0.08, z0 - 0.12), color="wood_dark")]
    glass = quad([(-w / 2, -0.101, z0), (w / 2, -0.101, z0), (w / 2, -0.101, z1), (-w / 2, -0.101, z1)],
                 "window_sky")
    mull = zb.box((0.07, 0.02, z1 - z0), (0, -0.105, (z0 + z1) / 2), color="wood_light")
    rot = normal_deg - 270.0
    for p in fr + [glass, mull]:
        p.rotate_z(rot).move(cx, cy, 0)
    return fr, [glass], [mull]


def terrarium_house():
    A = nl.Asset("terrarium_house")
    A.node("shell_upper")   # upper walls, posts, windows, trim — hidden while the player is inside
    A.node("roof")          # the dome roof — hidden while the player is inside
    low, up, trim = [], [], []
    for poly in MASSES:
        low.append(zb.slab(poly, 0, LOW_H, "house_wood_dark", top_color="wood_light"))
        up.append(zb.slab(poly, LOW_H, WALL_H, "house_wood", top_color="wood_light"))
    # thin outer walls of the three cases
    for cx, cy, sx, sy in ((-9.35, 4.0, 0.3, 4.0), (9.35, 4.0, 0.3, 4.0), (0, 10.85, 7.0, 0.3)):
        low.append(zb.box((sx, sy, LOW_H), (cx, cy, LOW_H / 2), color="house_wood_dark"))
        up.append(zb.box((sx, sy, WALL_H - LOW_H), (cx, cy, (LOW_H + WALL_H) / 2), color="house_wood"))
    # corner posts (timber logs) on every hull vertex
    for x, y in HULL:
        up.append(pp.cyl(0.3, 0.0, WALL_H + 0.15, sides=6, center=(x, y), color="wood"))
    # wall cap band, slightly proud of the wall
    up.append(zb.slab(scaled(HULL, 1.012), WALL_H - 0.22, WALL_H, "wood_dark"))
    # windows
    wins = []
    glass = []
    for cx, cy, deg, w in ((-7.8, 0, 270, 1.4), (7.8, 0, 270, 1.4),
                           (-9.5, 4.0, 180, 1.6), (9.5, 4.0, 0, 1.6),
                           (-6.5, 9.5, 153.4, 1.3), (6.5, 9.5, 26.6, 1.3),
                           (0, 11.0, 90, 1.6)):
        f, g, m = window(cx, cy, deg, w)
        wins += f + m
        glass += g
    up += wins
    # door: jambs + arch ring (house frame, door axis x = 0, opening 1.0 x 2.0 + arch to 2.5)
    door = []
    for sx in (-1, 1):
        door.append(zb.box((0.2, 0.2, 2.0), (sx * 0.6, -0.1, 1.0), color="wood"))
    arch_o = [(0.72 * math.cos(math.pi * i / 6), 2.0 + 0.72 * math.sin(math.pi * i / 6)) for i in range(7)]
    arch_i = [(0.5 * math.cos(math.pi * i / 6), 2.0 + 0.5 * math.sin(math.pi * i / 6)) for i in range(7)]
    ring = [(x, z) for x, z in arch_o] + [(x, z) for x, z in reversed(arch_i)]
    # clockwise in X/Z seen from -Y: reverse so prism faces point outward
    door.append(pp.prism_xz(list(reversed(ring)), -0.2, 0.0, "wood"))
    # lintel block above the arch (inside the facade row)
    # polygon in x/z: start bottom-left, arch over the top, down to bottom-right, up, across
    lintel = [(-0.5, 2.0)] + [(0.5 * math.cos(math.pi - math.pi * i / 6),
                               2.0 + 0.5 * math.sin(math.pi - math.pi * i / 6)) for i in range(1, 6)] \
        + [(0.5, 2.0), (0.5, WALL_H), (-0.5, WALL_H)]
    up.append(pp.prism_xz(list(reversed(lintel)), 0.0, 1.0, "house_wood"))
    # hall floor (planks + board lines) — stays visible
    floor = [zb.slab(HALL, 0.0, 0.1, "floor_wood")]
    for yy in (2.0, 3.0, 4.0, 5.0, 6.0):
        x0 = -4.5 if 2 <= yy <= 6 else -3.5
        floor.append(zb.box((9.0, 0.04, 0.012), (0, yy, 0.1), color="wood_dark"))
    # emblem plaque over the door (round cream plate; the game lays the animal pictogram on it)
    plaque = [nl.ring_xz(0, EMBLEM_Z, 1.15, 12, -0.58, -0.38, "wood_dark"),
              nl.ring_xz(0, EMBLEM_Z, 0.98, 12, -0.62, -0.58, "sign_panel")]
    for sx in (-1, 1):
        plaque.append(zb.box((0.12, 0.34, 0.12), (sx * 0.5, -0.3, EMBLEM_Z - 0.62), color="wood"))
    # dome roof: stepped layers of the hull (grass), wooden rim, glowing skylight on top
    roof = [zb.slab(scaled(HULL, 1.04), WALL_H, WALL_H + 0.2, "wood_light", top_color="wood_dark"),
            zb.slab(scaled(HULL, 1.02), WALL_H + 0.2, WALL_H + 0.85, "grass_roof", chamfer=0.2,
                    top_color="grass_roof_light"),
            zb.slab(scaled(HULL, 0.86), WALL_H + 0.85, WALL_H + 1.5, "grass_roof", chamfer=0.2,
                    top_color="grass_roof_light"),
            zb.slab(scaled(HULL, 0.66), WALL_H + 1.5, WALL_H + 2.05, "grass_roof", chamfer=0.18,
                    top_color="grass_roof_light"),
            zb.slab(scaled(HULL, 0.44), WALL_H + 2.05, WALL_H + 2.5, "grass_roof_light", chamfer=0.16)]
    sky = zb.slab(scaled(HULL, 0.2), WALL_H + 2.5, WALL_H + 2.75, "window_sky", chamfer=0.1)
    A.add(low + door + floor + plaque)
    A.add(up, node="shell_upper")
    A.add(glass, mat="window_glow", node="shell_upper", keep_down=True)
    A.add(roof, node="roof")
    A.add([sky], mat="window_glow", node="roof", keep_down=True)
    A.empty("socket_door", (0, 0.5, 0))
    A.empty("socket_emblem", (0, -0.63, EMBLEM_Z))
    A.empty("light_hall", (0, 4.0, 2.2), light=(4.0, "#FFE9B0"))
    return A


# --------------------------------------------------------------------------- case front, frame, lamp

def terrarium_front():
    """1 m glass front module: wooden railing base (0.7 m), glass pane (to 2.3 m), top rail."""
    A = nl.Asset("terrarium_front")
    p = [zb.box((1.0, 0.16, 0.7), (0, 0, 0.35), color="wood_light"),
         zb.box((0.96, 0.02, 0.05), (0, FRONT * 0.085, 0.38), color="wood_dark"),
         zb.box((1.0, 0.12, 0.1), (0, 0, 2.35), color="wood")]
    A.add(p)
    pane = [(-0.5, 0, 0.7), (0.5, 0, 0.7), (0.5, 0, 2.3), (-0.5, 0, 2.3)]
    A.add(double(pane, "glass_tint"), mat="glass")
    return A


def frame(name, width):
    """Case frame: two posts, a lintel and two corner braces around a glass front of `width` m."""
    A = nl.Asset(name)
    w = width
    p = []
    for sx in (-1, 1):
        p.append(zb.box((0.2, 0.2, 2.55), (sx * (w / 2 + 0.1), 0, 1.275), color="wood"))
        br = zb.box((0.5, 0.08, 0.08), (0, 0, 0), color="wood_dark")
        br.rotate_z(0).rotate(-sx * 45, "Y").move(sx * (w / 2 - 0.14), FRONT * 0.02, 2.14)
        p.append(br)
    p.append(zb.box((w + 0.4, 0.24, 0.26), (0, 0, 2.43), color="wood"))
    p.append(zb.box((w + 0.4, 0.28, 0.07), (0, 0, 2.58), color="wood_light"))
    A.add(p)
    return A


def lamp(name, glow):
    """Wall lamp of a case: back plate, arm, dome shade, glowing bulb. Origin = bottom of the bulb."""
    A = nl.Asset(name)
    p = [zb.box((0.16, 0.05, 0.5), (0, FRONT * 0.025, 0.55), color="wood_dark"),
         pp.beam((0, FRONT * 0.04, 0.62), (0, FRONT * 0.5, 0.5), 0.07, color="wood_dark"),
         pp.cyl(0.07, 0.38, 0.5, sides=6, center=(0, FRONT * 0.5), color="clay_pot_dark"),
         pp.cyl(0.26, 0.1, 0.38, r1=0.08, sides=8, center=(0, FRONT * 0.5), color="clay_pot")]
    A.add(p)
    bulb = zb.knob(0.075, (0, FRONT * 0.5, 0.075), color="lamp_glass", u=6, v=4)
    A.add([bulb], mat=glow)
    A.empty("light", (0, FRONT * 0.5, 0.1), light=(3.5, nl.GLOW[glow]))
    return A


def terrarium_dish():
    A = nl.Asset("terrarium_dish")
    A.add([pp.cyl(0.3, 0.0, 0.09, sides=8, r1=0.34, top=False, color="clay_pot"),
           pp.cyl(0.27, 0.07, 0.08, sides=8, color="water_pond_light"),
           pp.cyl(0.3, 0.085, 0.095, sides=8, r1=0.34, top=False, color="clay_pot_dark")])
    return A


# --------------------------------------------------------------------------- snake case

def floor_snake():
    """Snake floor 4.0 (w) x 4.6 (d): sand with darker dunes, a flagstone area and pebbles."""
    A = nl.Asset("terrarium_floor_snake")
    p = [zb.slab(zb.rounded_rect(-2.0, -2.3, 2.0, 2.3, 0.3, 1), 0, FLOOR_TOP, "sand")]
    for i, (x, y, rx, ry) in enumerate(((1.1, -1.5, 0.7, 0.45), (0.5, 0.3, 0.55, 0.4), (1.3, 1.2, 0.6, 0.5))):
        p.append(lump(x, y, rx, ry, "sand_dark", seed=i, z0=FLOOR_TOP - 0.004, h=0.02))
    cols = ("rock", "wall_stone", "rock_dark", "rock_warm", "wall_stone_dark")
    k = 0
    for row in range(5):
        for col in range(2):
            x = -1.55 + 0.82 * col + (0.2 if row % 2 else 0.0)
            y = -1.85 + 0.92 * row + (0.12 if col else 0.0)
            p.append(stone(x, y, 0.4 + 0.04 * ((row + col) % 2), 0.38, 0.05, cols[k % 5], seed=10 + k, n=6,
                           z0=FLOOR_TOP - 0.01))
            k += 1
    for i, (x, y) in enumerate(((0.3, -0.5), (1.5, 0.3))):
        p.append(stone(x, y, 0.1, 0.08, 0.06, "rock_warm_dark", seed=30 + i, n=4, z0=FLOOR_TOP - 0.01))
    A.add(p)
    return A


def rock_warm():
    """Warm rock pile ~1.0 x 0.8 x 0.8 m (three lumpy stones)."""
    A = nl.Asset("terrarium_rock_warm")
    A.add([pp.blob((0, 0, 0.42), (0.78, 0.6, 0.48), "rock_warm", subdiv=2, seed=3, amp=0.14, flat_bottom=0.0),
           pp.blob((0.62, -0.25, 0.28), (0.48, 0.42, 0.3), "rock_warm_dark", subdiv=1, seed=5, amp=0.14,
                   flat_bottom=0.0),
           pp.blob((-0.1, 0.05, 0.86), (0.42, 0.36, 0.3), "rock_warm_light", subdiv=1, seed=8, amp=0.12)])
    return A


def rock_flat():
    """Flat sun-warmed rock (basking stone), two stepped slabs, 1.3 x 0.9 x 0.3 m."""
    A = nl.Asset("terrarium_rock_flat")
    A.add([stone(0, 0, 0.9, 0.62, 0.2, "rock_warm_dark", seed=2, n=8, chamfer=0.05, top_color="rock_warm"),
           stone(0.05, 0.02, 0.68, 0.46, 0.18, "rock_warm", seed=4, n=7, z0=0.19, chamfer=0.06,
                 top_color="rock_warm_light")])
    return A


def branch_low():
    """Thick arched branch the snake coils over: 2.0 m long, 0.6 m high, with a side twig."""
    A = nl.Asset("terrarium_branch_low")
    pts = [(x, 0.0, 0.17 + 0.6 * (1 - (x / 1.3) ** 2) ** 0.8) for x in (-1.3, -0.87, -0.43, 0.0, 0.43, 0.87, 1.3)]
    p = []
    for i in range(len(pts) - 1):
        p.append(pp.beam(pts[i], pts[i + 1], 0.3 if abs(pts[i][0]) < 1.0 else 0.24, color="trunk" if i % 2 == 0
                         else "trunk_dark", sides=5))
    p.append(pp.beam((0.25, 0.0, 0.75), (0.7, 0.14, 1.2), 0.12, color="trunk_dark", sides=4))
    p.append(pp.beam((-0.45, 0.0, 0.7), (-0.8, -0.12, 1.0), 0.1, color="trunk_dark", sides=4))
    A.add(p)
    return A


# --------------------------------------------------------------------------- frog case

POOL = (0.6, 0.5, 1.25, 0.95)   # centre x, y, radii (floor frame) of the frog pool


def floor_frog():
    """Frog floor 4.0 x 4.6: sand bank, moss mounds, a stone-rimmed pool (node `water_pool`)."""
    A = nl.Asset("terrarium_floor_frog")
    A.node("water_pool")
    p = [zb.slab(zb.rounded_rect(-2.0, -2.3, 2.0, 2.3, 0.3, 1), 0, FLOOR_TOP, "sand")]
    for i, (x, y, rx, ry) in enumerate(((-1.1, -1.4, 0.7, 0.5), (-1.2, 0.5, 0.6, 0.6), (-0.1, -1.6, 0.5, 0.35))):
        p.append(lump(x, y, rx, ry, "sand_dark", seed=i, z0=FLOOR_TOP - 0.004, h=0.02, n=5))
    for i, (x, y, rx, ry) in enumerate(((-1.2, 1.5, 0.75, 0.55), (-0.2, 1.9, 0.6, 0.4), (-1.55, -0.4, 0.4, 0.5),
                                        (1.2, -1.75, 0.5, 0.35))):
        p.append(lump(x, y, rx, ry, "moss", seed=20 + i, z0=FLOOR_TOP - 0.004, h=0.07, n=6))
    # pool rim: ring of flat stones
    cx, cy, rx, ry = POOL
    n = 10
    for i in range(n):
        a = 2 * math.pi * i / n + 0.2
        c = ("rock", "rock_dark")[i % 2]
        p.append(stone(cx + (rx + 0.06) * math.cos(a), cy + (ry + 0.06) * math.sin(a), 0.3, 0.22, 0.12, c,
                       seed=40 + i, n=4, z0=FLOOR_TOP - 0.01))
    A.add(p)
    # the water: darker rim ellipse + lighter inside; the node can be wobbled / UV-animated
    w = [lump(cx, cy, rx, ry, "water_pond", seed=1, z0=FLOOR_TOP, h=0.03, n=12),
         lump(cx - 0.25, cy + 0.15, rx * 0.45, ry * 0.35, "water_pond_light", seed=2, z0=FLOOR_TOP + 0.03, h=0.004,
              n=8)]
    A.add(w, node="water_pool")
    return A


def waterfall():
    """Small waterfall: stacked stones 1.2 x 0.9 x 1.1 m with the `water_sheet` flowing down
    the front (pivot = the top lip). `socket_mist` = where the mist puff goes."""
    A = nl.Asset("terrarium_waterfall")
    A.node("water_sheet", pivot=(0, FRONT * 0.4, 1.25))
    p = [stone(0, 0.0, 0.78, 0.55, 0.5, "rock_dark", seed=1, n=7, chamfer=0.12),
         stone(0.05, 0.12, 0.58, 0.42, 0.45, "rock", seed=2, n=6, z0=0.48, chamfer=0.12),
         stone(0.08, 0.2, 0.4, 0.3, 0.38, "rock_dark", seed=3, n=6, z0=0.9, chamfer=0.12),
         stone(-0.85, -0.2, 0.26, 0.24, 0.26, "rock", seed=4, n=5, chamfer=0.08),
         stone(0.88, -0.15, 0.24, 0.22, 0.22, "rock_dark", seed=5, n=5, chamfer=0.08),
         lump(-0.1, 0.2, 0.34, 0.24, "moss", seed=6, z0=1.26, h=0.06, n=7)]
    A.add(p)
    # sheet: three strips down the front faces (just in front of the stones), 0.6 m wide
    strips = []
    for c, xw in (("water_river_light", -0.2), ("water_river", 0.0), ("water_pond_light", 0.2)):
        x0, x1 = xw - 0.1, xw + 0.1
        pts = [(x0, FRONT * 0.4, 1.25), (x0, FRONT * 0.62, 0.72), (x0, FRONT * 0.8, 0.12),
               (x1, FRONT * 0.8, 0.12), (x1, FRONT * 0.62, 0.72), (x1, FRONT * 0.4, 1.25)]
        # CCW as seen from the front (-Y): bottom-left -> bottom-right -> top-right -> top-left
        strips.append(quad([pts[2], pts[3], pts[4], pts[1]], c))
        strips.append(quad([pts[1], pts[4], pts[5], pts[0]], c))
    A.add(strips, node="water_sheet")
    # splash foam at the foot
    A.add([lump(0.0, FRONT * 0.88, 0.5, 0.2, "water_foam", seed=9, z0=0.06, h=0.06, n=8)])
    A.empty("socket_mist", (0, FRONT * 0.85, 0.12))
    return A


def mist_puff():
    """A small cloud of mist (three soft lumps, cream-white); scale / lift it in the game."""
    A = nl.Asset("mist_puff")
    A.add([pp.blob((0, 0, 0.22), (0.3, 0.26, 0.2), "water_foam", subdiv=1, seed=1, amp=0.1),
           pp.blob((0.25, -0.05, 0.16), (0.22, 0.2, 0.15), "water_foam", subdiv=1, seed=2, amp=0.1),
           pp.blob((-0.22, 0.05, 0.14), (0.2, 0.18, 0.13), "water_foam", subdiv=1, seed=3, amp=0.1)])
    return A


def leaf_big():
    """Big tropical leaf plant ~1.2 m high: a short stalk and seven large drooping leaves."""
    A = nl.Asset("terrarium_leaf_big")
    p = [pp.cyl(0.09, 0.0, 0.45, sides=5, r1=0.06, color="trunk")]
    cols = ("leaf", "leaf_light", "fern", "leaf", "leaf_light", "grove_leaf", "fern")
    specs = ((1, 0.2, 1.0, 0.55), (-0.4, 1, 1.05, 0.58), (-1, -0.2, 0.95, 0.52), (0.3, -1, 1.0, 0.55),
             (0.8, 0.7, 0.8, 0.5), (-0.8, 0.8, 0.82, 0.5), (-0.6, -0.8, 0.8, 0.48))
    for i, (dx, dy, ln, wd) in enumerate(specs):
        z0 = 0.42 + 0.05 * i
        p.append(leaf_tent((0, 0, z0), (dx, dy), ln, wd, 0.4 + 0.04 * (i % 3), 0.22, cols[i]))
    A.add(p)
    return A


def moss_log():
    """Mossy log ~1.6 m long, 0.4 m thick, lying along X, moss on top, a stub and a fern tuft."""
    A = nl.Asset("terrarium_moss_log")
    p = [pp.beam((-0.8, 0, 0.2), (0.8, 0, 0.2), 0.4, color="trunk", sides=7, taper=0.85),
         pp.beam((0.1, 0, 0.3), (0.35, 0.2, 0.55), 0.1, color="trunk_dark", sides=4),
         pp.blob((-0.2, 0.02, 0.42), (0.55, 0.22, 0.1), "moss", subdiv=1, seed=2, amp=0.1),
         pp.blob((0.42, -0.03, 0.38), (0.3, 0.2, 0.09), "moss", subdiv=1, seed=4, amp=0.1)]
    p.append(zb.box((0.01, 0.2, 0.2), (-0.806, 0, 0.2), color="trunk_dark"))
    for i in range(7):
        a = 2 * math.pi * i / 7
        p.append(pp.blade((-0.12 + 0.03 * math.cos(a), 0.02 * math.sin(a), 0.5),
                          (-0.12 + 0.2 * math.cos(a), 0.2 * math.sin(a), 0.78 + 0.04 * (i % 2)), 0.07, "fern",
                          bend=0.04))
    A.add(p)
    return A


def fern():
    """Low fern, 0.6 m: a fan of 12 drooping blades."""
    A = nl.Asset("terrarium_fern")
    p = []
    for i in range(12):
        a = 2 * math.pi * i / 12
        r = 0.32 + 0.06 * (i % 2)
        p.append(pp.blade((0.04 * math.cos(a), 0.04 * math.sin(a), 0.02),
                          (r * math.cos(a), r * math.sin(a), 0.55 - 0.1 * (i % 3)), 0.1,
                          ("fern", "leaf", "leaf_light")[i % 3], bend=0.05))
    A.add(p)
    return A


# --------------------------------------------------------------------------- chameleon case

def floor_chameleon():
    """Chameleon floor 7.0 x 3.6: dark earth with soil, bark litter and moss patches, a few stones."""
    A = nl.Asset("terrarium_floor_chameleon")
    p = [zb.slab(zb.rounded_rect(-3.5, -1.8, 3.5, 1.8, 0.35, 1), 0, FLOOR_TOP, "soil")]
    for i, (x, y, rx, ry, c) in enumerate(((-2.4, -0.6, 0.8, 0.5, "earth"), (-0.6, 0.7, 0.7, 0.45, "sand_dark"),
                                          (1.4, -0.7, 0.9, 0.55, "earth"), (2.5, 0.8, 0.6, 0.4, "sand_dark"),
                                          (-2.8, 1.0, 0.55, 0.4, "moss"), (0.4, -1.2, 0.5, 0.3, "moss"),
                                          (3.0, -1.1, 0.4, 0.3, "moss"))):
        p.append(lump(x, y, rx, ry, c, seed=i, z0=FLOOR_TOP - 0.004, h=0.025 if c != "moss" else 0.05))
    for i, (x, y, a, c) in enumerate(((-1.5, 0.2, 20, "twig"), (0.8, 0.3, -30, "twig_dark"), (2.0, 0.2, 60, "twig"),
                                      (-3.0, -0.2, 90, "twig_dark"), (1.1, -1.3, 10, "twig"))):
        b = pp.beam((-0.28, 0, 0), (0.28, 0, 0.0), 0.05, color=c, sides=4)
        b.rotate_z(a).move(x, y, FLOOR_TOP + 0.03)
        p.append(b)
    for i, (x, y) in enumerate(((-1.0, -1.2), (2.6, -0.2), (-2.0, 1.4))):
        p.append(stone(x, y, 0.2, 0.15, 0.1, ("rock", "rock_dark", "rock")[i], seed=60 + i, n=5,
                       z0=FLOOR_TOP - 0.01))
    A.add(p)
    return A


def branch_climb():
    """Climbing branches for the chameleon, 1.6 x 0.9 x 2.0 m: three leaning limbs with forks
    (`socket_perch_1..3` = where to put the chameleon)."""
    A = nl.Asset("terrarium_branch")
    p = [stone(0.0, 0.0, 0.42, 0.32, 0.12, "rock", seed=7, n=6, chamfer=0.05)]
    limbs = (((-0.25, 0.0, 0.05), (0.15, 0.05, 1.8), "trunk"),
             ((0.2, -0.1, 0.05), (-0.5, 0.12, 1.45), "trunk_dark"),
             ((0.05, 0.22, 0.05), (0.35, -0.2, 1.15), "trunk"))
    for a, b, c in limbs:
        p.append(pp.beam(a, b, 0.26, color=c, sides=5, taper=0.45))
    forks = (((0.0, 0.03, 1.1), (0.6, 0.05, 1.5), "trunk_dark", 0.12),
             ((0.1, 0.05, 1.6), (-0.4, 0.2, 1.9), "trunk", 0.1),
             ((-0.28, 0.1, 1.05), (-0.7, -0.1, 1.3), "trunk", 0.1),
             ((0.25, -0.12, 0.8), (0.72, -0.3, 1.0), "trunk_dark", 0.1),
             ((0.12, -0.15, 1.0), (-0.12, -0.35, 1.25), "trunk", 0.08))
    for a, b, c, w in forks:
        p.append(pp.beam(a, b, w, color=c, sides=4, taper=0.6))
    for i, (bx, by, bz, dx, dy, c) in enumerate(((0.15, 0.05, 1.8, 1, 0.4, "leaf"), (-0.5, 0.12, 1.45, -1, 0.3, "leaf_light"),
                                                  (0.6, 0.05, 1.5, 1, -0.3, "fern"), (-0.4, 0.2, 1.9, -0.5, 1, "leaf"),
                                                  (-0.7, -0.1, 1.3, -1, -0.5, "fern"), (0.72, -0.3, 1.0, 1, -0.4, "leaf_light"))):
        p.append(leaf_tent((bx, by, bz), (dx, dy), 0.65, 0.4, 0.2, 0.16, c))
    A.add(p)
    A.empty("socket_perch_1", (0.55, 0.05, 1.5))
    A.empty("socket_perch_2", (-0.35, 0.2, 1.9))
    A.empty("socket_perch_3", (-0.62, -0.1, 1.3))
    return A


def tree_small():
    """Small leafy tree with perches, 2.2 m: trunk, three perch limbs, big leaf crowns
    (`socket_perch_1..3`)."""
    A = nl.Asset("terrarium_tree")
    p = [pp.beam((0, 0, 0), (0.05, 0.05, 1.65), 0.38, color="trunk", sides=6, taper=0.45),
         stone(0, 0, 0.5, 0.45, 0.1, "trunk_dark", seed=3, n=6, chamfer=0.04)]
    limbs = (((0.02, 0.02, 0.75), (1.0, 0.1, 1.0), 0.16), ((0.04, 0.03, 1.1), (-0.95, -0.1, 1.4), 0.16),
             ((0.05, 0.05, 1.4), (0.65, 0.4, 1.75), 0.12))
    for i, (a, b, w) in enumerate(limbs):
        p.append(pp.beam(a, b, w, color="trunk_dark" if i % 2 else "trunk", sides=4, taper=0.6))
    p.append(pp.blob((0.0, 0.05, 1.85), (0.7, 0.6, 0.4), "leaf", subdiv=2, seed=2, amp=0.14))
    p.append(pp.blob((-1.0, -0.05, 1.55), (0.45, 0.4, 0.3), "leaf_light", subdiv=2, seed=4, amp=0.14))
    p.append(pp.blob((0.85, 0.2, 1.2), (0.4, 0.34, 0.26), "grove_leaf", subdiv=1, seed=6, amp=0.14))
    p.append(leaf_tent((1.0, 0.1, 1.0), (1, 0.3), 0.5, 0.3, 0.16, 0.12, "fern"))
    A.add(p)
    A.empty("socket_perch_1", (0.95, 0.1, 1.1))
    A.empty("socket_perch_2", (-0.8, -0.1, 1.48))
    A.empty("socket_perch_3", (0.5, 0.35, 1.8))
    return A


# --------------------------------------------------------------------------- info board + fridge

def info_board_wall():
    """Wall-mounted info board: framed text panel (`info_panel`) tilted 15 deg, with a cream round
    plate above it for the animal pictogram decal. Wall plane at glTF z = 0 (origin = lowest point)."""
    A = nl.Asset("info_board_wall")
    W, H = 0.9, 0.6
    panel = []
    import kit_signs as ks
    for q in ks.framed_panel(W, H, "info_panel", frame=0.06, depth=0.06):
        q.rotate(15 * FRONT, "X")
        q.move(0, FRONT * 0.07, H / 2)
        panel.append(q)
    panel.append(zb.box((W - 0.1, 0.05, 0.07), (0, FRONT * 0.025, 0.04), color="wood_dark"))
    A.add(panel)
    plate = [nl.ring_xz(0, H + 0.4, 0.24, 10, FRONT * 0.1, FRONT * 0.03, "wood_dark"),
             nl.ring_xz(0, H + 0.4, 0.19, 10, FRONT * 0.12, FRONT * 0.1, "sign_panel"),
             zb.box((0.07, 0.05, 0.22), (0, FRONT * 0.025, H + 0.02), color="wood_dark")]
    A.add(plate)
    A.empty("socket_pictogram", (0, FRONT * 0.125, H + 0.4))
    A.empty("socket_lamp", (0, FRONT * 0.1, H + 0.02))
    return A


def fridge():
    """Small fridge for the frozen insects: 0.7 x 0.65 x 1.15 m, cream, steel handle, snowflake sticker."""
    A = nl.Asset("fridge")
    p = [zb.box((0.7, 0.62, 1.1), (0, 0, 0.6), color="trim_white", bevel=0.03),
         zb.box((0.66, 0.5, 0.05), (0, 0, 0.04), color="iron"),
         zb.box((0.7, 0.012, 0.03), (0, FRONT * 0.312, 0.78), color="rail_steel"),
         zb.box((0.04, 0.05, 0.3), (0.27, FRONT * 0.34, 0.95), color="rail_steel"),
         zb.box((0.04, 0.05, 0.22), (0.27, FRONT * 0.34, 0.55), color="rail_steel")]
    for sx in (-1, 1):
        p.append(zb.box((0.07, 0.07, 0.05), (sx * 0.27, 0.2, 0.025), color="iron"))
    star = []
    for i in range(12):
        r = 0.13 if i % 2 == 0 else 0.05
        a = math.pi / 2 + math.pi * i / 6
        star.append((-0.05 + r * math.cos(a), FRONT * 0.317, 0.95 + r * math.sin(a)))
    sp = pp.flat(star, "sign_blue")
    sp.bm.normal_update()
    if sp.bm.faces[:][0].normal.y * FRONT < 0:
        bmesh.ops.reverse_faces(sp.bm, faces=sp.bm.faces[:])
    p.append(sp)
    A.add(p)
    return A


# --------------------------------------------------------------------------- case dressing

# Viewer frame of a case (see the module docstring): origin = floor centre at ground level, x = to
# the viewer's right, y = into the case. (model, x, y, z, yaw deg about the up axis)
CASES = {
    "snake": {   # level: enc_n2_snake (-94, 41, 5, 4); glass east -> yaw 90
        "yaw": 90, "centre_house": (-6.85, 4.0), "glass_y": -2.4, "back_y": 2.35, "gate": (-1.0, 1.0),
        "items": [
            ("terrarium_floor_snake", 0.0, 0.0, 0.0, 0),
            ("terrarium_rock_warm", 1.0, 1.4, FLOOR_TOP, 20),
            ("terrarium_rock_flat", 0.7, -0.2, FLOOR_TOP, 10),
            ("terrarium_branch_low", 0.1, 0.55, FLOOR_TOP, -8),
            ("terrarium_dish", 1.4, -1.5, FLOOR_TOP, 0),
            ("terrarium_lamp", 0.6, 2.35, 1.95, 0),
        ],
    },
    "chameleon": {   # enc_n2_chameleon (-88, 46, 7, 4); glass south -> yaw 0
        "yaw": 0, "centre_house": (0.0, 8.85), "glass_y": -1.85, "back_y": 1.85, "gate": (-1.5, 0.5),
        "items": [
            ("terrarium_floor_chameleon", 0.0, 0.0, 0.0, 0),
            ("terrarium_branch", -1.8, 0.4, FLOOR_TOP, 0),
            ("terrarium_branch", 0.4, 0.9, FLOOR_TOP, 180),
            ("terrarium_tree", 2.1, 0.3, FLOOR_TOP, 0),
            ("terrarium_leaf_big", -2.5, -0.4, FLOOR_TOP, 30),
            ("terrarium_leaf_big", 2.6, -0.6, FLOOR_TOP, 200),
            ("terrarium_lamp_uv", 0.0, 1.85, 1.95, 0),
        ],
    },
    "frog": {   # enc_n2_frog (-80, 41, 5, 4); glass west -> yaw -90
        "yaw": -90, "centre_house": (6.85, 4.0), "glass_y": -2.4, "back_y": 2.35, "gate": (-1.0, 1.0),
        "items": [
            ("terrarium_floor_frog", 0.0, 0.0, 0.0, 0),
            ("terrarium_waterfall", 0.6, 1.7, FLOOR_TOP, 0),
            ("mist_puff", 0.6, 0.95, FLOOR_TOP + 0.1, 0),
            ("terrarium_leaf_big", -1.4, 1.6, FLOOR_TOP, 0),
            ("terrarium_leaf_big", 1.3, -1.2, FLOOR_TOP, 90),
            ("terrarium_moss_log", -1.0, -0.9, FLOOR_TOP, -12),
            ("terrarium_fern", -0.3, 1.9, FLOOR_TOP, 0),
            ("terrarium_lamp_teal", -0.8, 2.35, 1.95, 0),
        ],
    },
}


def front_items(case):
    """Glass-front pieces of a case in its viewer frame: 1 m panes left and right of the gate
    (the 2 m `glass_door` fills the gap) and the frame."""
    c = CASES[case]
    g0, g1 = c["gate"]
    half = 2.0 if case != "chameleon" else 3.5
    xs = []
    x = -half + 0.5
    while x < half:
        if not (g0 - 1e-6 <= x - 0.5 and x + 0.5 <= g1 + 1e-6):
            xs.append(x)
        x += 1.0
    out = [("terrarium_front", x, c["glass_y"], 0.0, 0) for x in xs]
    out.append(("terrarium_frame" if case != "chameleon" else "terrarium_frame_wide", 0.0, c["glass_y"], 0.0, 0))
    return out


def level_placement(case, mx, my):
    """Level position (x east, z north) of a case-frame point (mx right, my into the case)."""
    c = CASES[case]
    hx, hy = c["centre_house"]
    yaw = c["yaw"]
    if yaw == 0:       # glass south: front = level south, right = east
        dx, dz = mx, my
    elif yaw == 90:    # glass east: front = +x, right = north (+z), into the case = west (-x)
        dx, dz = -my, mx
    else:              # glass west: front = -x, right = south (-z), into the case = east (+x)
        dx, dz = my, -mx
    return (-84.5 + hx + dx, 39.0 + hy + dz)


def extra(roots):
    """The three assembled cases (viewer frame, yaw 0) behind the grid."""
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    out = []
    for k, case in enumerate(("snake", "frog", "chameleon")):
        B = -fwd * 15.0 + right * (1.0 + 6.5 * k + (3.0 if k == 2 else 0.0))
        n = 0
        for model, x, y, z, yaw in CASES[case]["items"] + front_items(case):
            n += 1
            out.append(nl.instance_tree(roots[model], f"s_{case}_{n}", B + Vector((x, y, z)), rot_deg=yaw))
    return out


def place_cases(roots, origin, tag):
    """The three cases in their house positions (house frame, turned by their yaw) around `origin`."""
    out = []
    for case, c in CASES.items():
        hx, hy = c["centre_house"]
        yaw = math.radians(c["yaw"])
        cs, sn = math.cos(yaw), math.sin(yaw)
        n = 0
        for model, x, y, z, ry in c["items"] + front_items(case):
            n += 1
            off = Vector((hx + x * cs - y * sn, hy + x * sn + y * cs, z))
            out.append(nl.instance_tree(roots[model], f"{tag}_{case}_{n}", origin + off, rot_deg=ry + c["yaw"]))
    return out


def house_preview(builders):
    """model_preview_house.png: the closed house and, beside it, the house without roof and
    upper walls with the three dressed cases inside (day / night)."""
    nl.reset()
    roots = {n: nl.build(fn()) for n, fn in builders.items()}
    for r in roots.values():
        r.location = (0, -300, 0)
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    closed = nl.instance_tree(roots["terrarium_house"], "pv_closed", right * -11.0)
    opened = nl.instance_tree(roots["terrarium_house"], "pv_open", right * 11.0)
    for o in nl.descendants(opened):
        if o.name.endswith(("roof", "shell_upper")) and o is not opened:
            bpy.data.objects.remove(o, do_unlink=True)
    cases = place_cases(roots, opened.location.copy(), "pv_case")
    for r in roots.values():
        for o in nl.descendants(r):
            bpy.data.objects.remove(o, do_unlink=True)
    out = os.path.join(os.path.dirname(zb.repo_path("art", "props", KIT, "x")), "model_preview_house.png")
    nl.render_preview([closed, opened], out, extra=cases, labels=False)


def builders():
    return {
        "terrarium_front": terrarium_front,
        "terrarium_frame": lambda: frame("terrarium_frame", 4.0),
        "terrarium_frame_wide": lambda: frame("terrarium_frame_wide", 7.0),
        "terrarium_lamp": lambda: lamp("terrarium_lamp", "terrarium_amber_glow"),
        "terrarium_lamp_uv": lambda: lamp("terrarium_lamp_uv", "terrarium_violet_glow"),
        "terrarium_lamp_teal": lambda: lamp("terrarium_lamp_teal", "terrarium_teal_glow"),
        "terrarium_dish": terrarium_dish,
        "terrarium_floor_snake": floor_snake,
        "terrarium_rock_warm": rock_warm,
        "terrarium_rock_flat": rock_flat,
        "terrarium_branch_low": branch_low,
        "terrarium_floor_frog": floor_frog,
        "terrarium_waterfall": waterfall,
        "mist_puff": mist_puff,
        "terrarium_leaf_big": leaf_big,
        "terrarium_moss_log": moss_log,
        "terrarium_fern": fern,
        "terrarium_floor_chameleon": floor_chameleon,
        "terrarium_branch": branch_climb,
        "terrarium_tree": tree_small,
        "info_board_wall": info_board_wall,
        "fridge": fridge,
    }


BUDGETS = {"*": 300, "terrarium_house": 3000, "terrarium_front": 40, "terrarium_frame": 80,
           "terrarium_frame_wide": 80, "terrarium_lamp": 150, "terrarium_lamp_uv": 150,
           "terrarium_lamp_teal": 150}


def main():
    args = zb.script_args()
    if "--house-preview" in args:
        house_preview({**builders(), "terrarium_house": terrarium_house})
        return
    nl.run(KIT, builders(), cols=6, spacing=(4.6, 3.8), budgets=BUDGETS, extra=extra,
           variants={"terrarium_house": terrarium_house})
    if "--no-preview" not in args and not [a for a in args if not a.startswith("--")]:
        import subprocess
        subprocess.run([bpy.app.binary_path, "-b", "--factory-startup", "--python", os.path.abspath(__file__),
                        "--", "--house-preview"], check=True)


if __name__ == "__main__":
    main()
