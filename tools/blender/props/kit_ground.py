"""Kit 1 — ground tiles (concept: art/props/kit_ground/sheet_v2.jpg, brief.md).

Run:  blender -b --python tools/blender/props/kit_ground.py -- [--no-preview]

Outputs
  assets/models/props/<asset>.glb     one per asset
  assets/blender/props/kit_ground.blend
  art/props/kit_ground/model_preview.png

TILE SIZE: 1 m x 1 m, one tile per cell (Q-057; the first brief said 2 m). The level grid (GAME-LAYOUT,
assets/levels/level-1.toml) uses 1 m cells and its paths are 3 cells wide at odd
coordinates (e.g. path_ring_s = [-8, 8, 16, 3]), plaza 10 x 8, bridge 3 x 3 — 2 m tiles
cannot cover them; 1 m tiles map one-to-one onto cells.

PATH AUTOTILING: every path cell gets one path tile chosen from which of its 4 neighbours
(N, E, S, W) are path cells too ("connections"); unconnected sides get a 0.14 m grass
margin, so wide paths get a clean border and a 1-cell path looks like the concept tiles.
Canonical orientation (rotate about +Y in 90 deg steps; N->E is one step clockwise seen
from above = -90 deg about +Y, zoo_core::coords::quarter_turns_cw_to_yaw; north = world -Z
= Blender +Y, GAME-LAYOUT "Coordinate spaces", Q-056):
  path_tile_cross     N E S W   (inner cell of a wide path)
  path_tile_t         N E S     (edge cell of a wide path; grass on W)
  path_tile_straight  N   S     (1-cell path)
  path_tile_curve       E S     (corner cell; rounded outer corner NW)
Where two connected sides meet, the paving runs into the tile corner (no grass notch),
because the diagonal neighbour of such a cell is path in 3-wide paths.
  path_tile_end           S     (rounded dead end towards N)
  (0 connections does not occur)
Tile tops: grass/sand/slab top y = 0.05, path bed 0.065, paving stones up to 0.09,
plaza slabs up to 0.075. Tiles are centred on the cell (origin = cell centre, y = 0).
path_edge: 1 m strip of edging stones along X, origin at its centre; place it on the grass
margin of a path tile, 0.43 m from the cell centre towards the grass side.
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True  # no __pycache__ in the repo
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "lib"))
import zoo_blender as zb  # noqa: E402

KIT = "kit_ground"
SLAB_TOP = 0.05
PLATE_TOP = 0.065
STONE_TOP = 0.09
MARGIN = 0.14          # grass margin on unconnected sides
A = 0.5 - MARGIN       # half width of the paved area

# direction -> Blender unit vector (north = world -Z = Blender +Y, GAME-LAYOUT Q-056)
DIRS = {"E": (1, 0), "N": (0, 1), "W": (-1, 0), "S": (0, -1)}
CCW = ["E", "N", "W", "S"]  # counter-clockwise order in Blender's top view

CANONICAL = {
    "path_tile_cross": {"N", "E", "S", "W"},
    "path_tile_t": {"N", "E", "S"},
    "path_tile_straight": {"N", "S"},
    "path_tile_curve": {"E", "S"},
    "path_tile_end": {"S"},
}
CW = {"N": "E", "E": "S", "S": "W", "W": "N"}


def square(h=0.5):
    return [(-h, -h), (h, -h), (h, h), (-h, h)]


def base_slab(top="grass"):
    return zb.slab(square(), 0.0, SLAB_TOP, color=top, top_color=top, side_color="soil")


def path_polygon(conn):
    """CCW outline of the paved area for a set of connected game directions."""
    pts = []
    E = 0.5
    n_conn = len(conn)
    for k, side in enumerate(CCW):
        nxt = CCW[(k + 1) % 4]
        d = DIRS[side]
        t = DIRS[nxt]
        if side in conn:
            pts.append((E * d[0] - A * t[0], E * d[1] - A * t[1]))
            pts.append((E * d[0] + A * t[0], E * d[1] + A * t[1]))
        a_side, a_next = side in conn, nxt in conn
        if a_side and a_next:
            # paved right into the corner: in wide paths (level 1 paths are 3 cells) the
            # diagonal neighbour is path too; a grass notch here would dot the paving
            pts.append((E * (d[0] + t[0]), E * (d[1] + t[1])))
        elif not a_side and not a_next:
            r = A if n_conn == 1 else 2 * A
            cx, cy = (A - r) * (d[0] + t[0]), (A - r) * (d[1] + t[1])
            a0 = math.atan2(d[1], d[0])
            seg = 4 if n_conn == 1 else 6
            for i in range(seg + 1):
                a = a0 + (math.pi / 2) * i / seg
                pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    # drop consecutive duplicates
    out = []
    for p in pts:
        if not out or math.dist(p, out[-1]) > 1e-6:
            out.append(p)
    if math.dist(out[0], out[-1]) < 1e-6:
        out.pop()
    return out


def inside(poly, x, y):
    c = False
    n = len(poly)
    for i in range(n):
        x1, y1 = poly[i]
        x2, y2 = poly[(i + 1) % n]
        if (y1 > y) != (y2 > y):
            if x < x1 + (y - y1) * (x2 - x1) / (y2 - y1):
                c = not c
    return c


def circle_inside(poly, x, y, r, pad=0.02):
    return all(inside(poly, x + (r + pad) * math.cos(a), y + (r + pad) * math.sin(a))
               for a in [i * math.pi / 6 for i in range(12)]) and inside(poly, x, y)


def stone_pattern(seed=7):
    """One fixed cobble layout per cell (Poisson disc); every tile filters it."""
    rnd = random.Random(seed)
    stones = []
    for _ in range(6000):
        r = rnd.uniform(0.11, 0.165)
        x = rnd.uniform(-0.5 + r + 0.03, 0.5 - r - 0.03)
        y = rnd.uniform(-0.5 + r + 0.03, 0.5 - r - 0.03)
        if all(math.dist((x, y), (sx, sy)) >= r + sr + 0.04 for sx, sy, sr, *_ in stones):
            stones.append((x, y, r, rnd.uniform(0.8, 1.0), rnd.uniform(0, 6.28)))
    return stones


STONES = stone_pattern()


def path_tile_parts(conn):
    parts = [base_slab("grass")]
    poly = path_polygon(conn)
    parts.append(zb.slab(poly, SLAB_TOP - 0.01, PLATE_TOP, color="path_mortar"))
    for x, y, r, sq, ph in STONES:
        for scale in (1.0, 0.72):
            rr = r * scale
            if rr >= 0.075 and circle_inside(poly, x, y, rr):
                pts = zb.ellipse(x, y, rr, rr * sq, sides=8, phase=ph)
                parts.append(zb.slab(pts, PLATE_TOP - 0.008, STONE_TOP, color="path_stone",
                                     chamfer=STONE_TOP - PLATE_TOP + 0.008))
                break
    return parts


def plaza_tile_parts():
    parts = [zb.slab(square(), 0.0, SLAB_TOP, color="plaza_joint", side_color="soil")]
    j = 0.015
    E = 0.5 - j
    rects = [
        ((-E, j, 0.165, E), "plaza_stone"),
        ((0.165 + 2 * j, j, E, E), "plaza_stone_dark"),
        ((-E, -E, -0.195, -j), "plaza_stone_dark"),
        ((-0.195 + 2 * j, -E, E, -j), "plaza_stone"),
    ]
    for (x0, y0, x1, y1), col in rects:
        parts.append(zb.slab(zb.rounded_rect(x0, y0, x1, y1, 0.035, 1), SLAB_TOP - 0.01, 0.075,
                             color=col, chamfer=0.018))
    return parts


def grass_tile_parts():
    return [base_slab("grass"),
            zb.tuft((-0.22, 0.18, SLAB_TOP), 0.09, seed=1),
            zb.tuft((0.25, -0.2, SLAB_TOP), 0.08, seed=2)]


def sand_tile_parts():
    return [base_slab("sand")]


def path_edge_parts():
    parts = []
    n = 4
    L = 1.0 / n
    for i in range(n):
        x0 = -0.5 + i * L + 0.012
        x1 = x0 + L - 0.024
        parts.append(zb.slab(zb.rounded_rect(x0, -0.06, x1, 0.06, 0.035, 1), 0.0, 0.12,
                             color="edge_stone", chamfer=0.035))
    return parts


def tile_for(conn):
    """(asset, clockwise quarter turns) for a set of connected game directions."""
    for name, canon in CANONICAL.items():
        c = set(canon)
        for k in range(4):
            if c == conn:
                return name, k
            c = {CW[d] for d in c}
    raise ValueError(conn)


def sample_patch():
    """Preview-only: a 3-wide path bend + plaza + grass, autotiled (not exported)."""
    size = 6
    path = set()
    for x in range(1, 4):
        for z in range(-1, 4):
            path.add((x, z))
    for x in range(1, 7):
        for z in range(1, 4):
            path.add((x, z))
    plaza = {(x, z) for x in (4, 5) for z in (-1, 0)}
    parts = []
    for x in range(size):
        for z in range(-1, size - 1):
            bx, by = x - size / 2 + 0.5, z - size / 2 + 1.5  # level z north = Blender +Y
            if (x, z) in path:
                conn = {d for d, (dx, dz) in {"N": (0, 1), "E": (1, 0), "S": (0, -1), "W": (-1, 0)}.items()
                        if (x + dx, z + dz) in path or (x + dx, z + dz) in plaza}
                name, _ = tile_for(conn)  # every mask maps onto one of the five tiles
                ps = path_tile_parts(conn)
                # edging stones on unconnected straight sides (not on rounded ones)
                for d in ({"N", "E", "S", "W"} - conn) if name in ("path_tile_t", "path_tile_straight") else ():
                    e = path_edge_parts()
                    rot = {"N": 0, "S": 0, "E": 90, "W": 90}[d]
                    dx, dy = DIRS[d]
                    for p in e:
                        p.rotate_z(rot).move(dx * 0.43, dy * 0.43, SLAB_TOP - 0.01)
                    ps += e
            elif (x, z) in plaza:
                ps = plaza_tile_parts()
            elif x == 0 and z >= 2:
                ps = sand_tile_parts()
            else:
                ps = grass_tile_parts() if (x + z) % 3 == 0 else [base_slab("grass")]
            for p in ps:
                p.move(bx, by, 0)
            parts += ps
    return parts


def weld_patch(obj):
    """Preview only: drop the slab side faces between neighbouring tiles and weld, so the
    Freestyle stand-in outline does not draw a line at every (coplanar) tile seam."""
    import bmesh
    bm = bmesh.new()
    bm.from_mesh(obj.data)
    bm.normal_update()
    xs = [v.co.x for v in bm.verts]
    ys = [v.co.y for v in bm.verts]
    bx, by = (min(xs), max(xs)), (min(ys), max(ys))
    inner = []
    for f in bm.faces:
        zs = [v.co.z for v in f.verts]
        c = f.calc_center_median()
        on_border = min(abs(c.x - bx[0]), abs(c.x - bx[1]), abs(c.y - by[0]), abs(c.y - by[1])) < 1e-4
        fx = c.x - bx[0]
        fy = c.y - by[0]
        on_seam = ((abs(f.normal.x) > 0.99 and abs(fx - round(fx)) < 1e-4)
                   or (abs(f.normal.y) > 0.99 and abs(fy - round(fy)) < 1e-4))
        if on_seam and max(zs) <= PLATE_TOP + 1e-4 and not on_border:
            inner.append(f)
    bmesh.ops.delete(bm, geom=inner, context="FACES_ONLY")
    bmesh.ops.remove_doubles(bm, verts=bm.verts, dist=1e-4)
    bm.to_mesh(obj.data)
    bm.free()


def main():
    args = zb.script_args()
    zb.clean_scene()

    builders = {
        "path_tile_straight": lambda: path_tile_parts(CANONICAL["path_tile_straight"]),
        "path_tile_curve": lambda: path_tile_parts(CANONICAL["path_tile_curve"]),
        "path_tile_t": lambda: path_tile_parts(CANONICAL["path_tile_t"]),
        "path_tile_cross": lambda: path_tile_parts(CANONICAL["path_tile_cross"]),
        "path_tile_end": lambda: path_tile_parts(CANONICAL["path_tile_end"]),
        "plaza_tile": plaza_tile_parts,
        "grass_tile": grass_tile_parts,
        "path_edge": path_edge_parts,
        "sand_tile": sand_tile_parts,
    }
    objs = []
    for name, build in builders.items():
        objs.append(zb.build_object(name, build()))

    ok = zb.report(objs)
    for o in objs:
        zb.export_glb(o, zb.repo_path("assets", "models", "props", o.name + ".glb"))

    zb.layout_grid(objs, cols=3, spacing=(1.7, 1.9))
    zb.save_blend(zb.repo_path("assets", "blender", "props", KIT + ".blend"))

    if "--no-preview" not in args:
        patch = zb.build_object("sample_patch", sample_patch(), smooth_angle=50)
        weld_patch(patch)
        fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
        patch.location = right * 9.2 - fwd * 1.9
        zb.render_preview(objs, zb.repo_path("art", "props", KIT, "model_preview.png"),
                          extra_objs=[patch])
    if not ok:
        sys.exit(1)


if __name__ == "__main__":
    main()
