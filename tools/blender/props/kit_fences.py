"""Kit 2 — fences, gate, hedges, walls (concept: art/props/kit_fences/sheet_v3.jpg).

Run:  blender -b --python tools/blender/props/kit_fences.py -- [--no-preview]

Outputs
  assets/models/props/<asset>.glb     one per asset
  assets/blender/props/kit_fences.blend
  art/props/kit_fences/model_preview.png

Placement conventions (game axes: +X east, +Y up, +Z north; origin on the ground):
- Straight pieces (fence_wood, hedge, zoo_wall) are 2 m long along X, centred on the
  origin (x = -1 .. +1), thickness centred on z = 0. fence_wood has posts at both ends
  and in the middle; neighbouring segments share the end-post position (identical,
  overlapping post geometry — drop one when batching if wanted).
- Corner pieces (fence_wood_corner, hedge_corner, zoo_wall_corner) are L pieces whose
  origin is the intersection of the two centre lines; the arms run 1 m towards +X (east)
  and +Z (north). Other directions: rotate about +Y in 90 deg steps.
- fence_wood_end: a single post with a rounded top, origin at its centre.
- gate_wood: only the moving leaf (no posts). Origin = HINGE AXIS on the ground. The leaf
  spans local x = 0.02 .. 1.80 (closed = along +X). Hang it in a 2 m fence span whose
  posts (0.18 m) stand at s and s + 2: origin at (s + 0.09) = inner face of the hinge
  post, leaving 2 cm clearance at both posts. Open
  = rotate about local +Y: +90 deg swings the leaf to -Z (south), -90 deg to +Z (north).
  Latch on the free end (x = 1.80); brace, hinge straps and latch are on the -Z (south)
  face, the side the default follow camera (looking north) sees.
- Sizes: fence 1.1 m high, hedge 3.0 m high x 1.0 m thick, zoo wall 2.5 m high x 0.6 m
  thick (0.8 m wooden cap).
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True  # no __pycache__ in the repo
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "lib"))
import zoo_blender as zb  # noqa: E402

KIT = "kit_fences"

# Blender: +Y = game south (-Z), -Y = game north (+Z)
NORTH = -1.0
SOUTH = 1.0

FENCE_H = 1.1
POST_W = 0.18
RAIL_Z = (0.32, 0.62, 0.92)
RAIL_H = 0.13
RAIL_D = 0.07

HEDGE_H = 3.0
HEDGE_T = 1.0
WALL_H = 2.5
WALL_T = 0.6
CAP_H = 0.2
CAP_T = 0.8


# --------------------------------------------------------------------------- fence

def post(x, y, h=FENCE_H, w=POST_W, color="wood"):
    return zb.box((w, w, h), (x, y, h / 2), color=color, bevel=0.03)


def rail(x0, x1, z, y=0.0, along="x", color="wood_light"):
    L = abs(x1 - x0)
    c = (x0 + x1) / 2
    if along == "x":
        p = zb.box((L, RAIL_D, RAIL_H), (c, y, z), color=color, bevel=0.02, bevel_edges="long")
    else:
        p = zb.box((L, RAIL_D, RAIL_H), (0, 0, z), color=color, bevel=0.02, bevel_edges="long")
        p.rotate_z(90).move(y, c, 0)
    return p


def fence_wood():
    parts = [post(x, 0) for x in (-1.0, 0.0, 1.0)]
    parts += [rail(-1.0, 1.0, z) for z in RAIL_Z]
    return parts


def fence_wood_corner():
    n = NORTH
    parts = [post(0, 0), post(1.0, 0), post(0, n * 1.0)]
    parts += [rail(0.0, 1.0, z) for z in RAIL_Z]
    parts += [rail(0.0, n * 1.0, z, y=0.0, along="y") for z in RAIL_Z]
    return parts


def fence_wood_end():
    w = 0.2
    body = zb.box((w, w, 1.02), (0, 0, 0.51), color="wood", bevel=0.035)
    collar = zb.box((w + 0.04, w + 0.04, 0.06), (0, 0, 1.0), color="wood_dark", bevel=0.02)
    top = zb.knob(0.125, (0, 0, 1.1), color="wood", u=8, v=6, squash=0.95)
    return [body, collar, top]


def gate_wood():
    x0, x1 = 0.02, 1.80
    sw = 0.12  # stile width
    parts = []
    # stiles reach the ground so the leaf rests on y = 0 (origin at the hinge foot)
    parts.append(zb.box((sw, RAIL_D + 0.01, 1.05), (x0 + sw / 2, 0, 0.525), color="wood", bevel=0.025))
    parts.append(zb.box((sw, RAIL_D + 0.01, 1.05), (x1 - sw / 2, 0, 0.525), color="wood", bevel=0.025))
    for zc in (0.2, 0.55, 0.91):
        parts.append(zb.box((x1 - x0 - 2 * sw + 0.02, RAIL_D, RAIL_H + 0.01), ((x0 + x1) / 2, 0, zc),
                            color="wood_light", bevel=0.02, bevel_edges="long"))
    # diagonal brace (hinge bottom -> free end top), on the south face (seen by the default camera)
    ax, az = x0 + sw, 0.26
    bx, bz = x1 - sw, 0.85
    L = math.hypot(bx - ax, bz - az)
    ang = math.degrees(math.atan2(bz - az, bx - ax))
    br = zb.box((L, 0.05, 0.1), (0, 0, 0), color="wood_dark", bevel=0.015, bevel_edges="long")
    br.rotate(-ang, "Y").move((ax + bx) / 2, SOUTH * (RAIL_D / 2 + 0.025), (az + bz) / 2)
    parts.append(br)
    # hinge straps and latch (metal)
    yface = SOUTH * (RAIL_D / 2 + 0.012)
    for zc in (0.2, 0.91):
        parts.append(zb.box((0.34, 0.02, 0.06), (x0 + 0.15, yface, zc), color="metal"))
    parts.append(zb.box((0.2, 0.03, 0.05), (x1 - 0.1, yface + SOUTH * 0.01, 0.72), color="metal",
                        bevel=0.008))
    parts.append(zb.box((0.04, 0.05, 0.12), (x1 - 0.16, yface + SOUTH * 0.015, 0.72), color="metal", bevel=0.008))
    return parts


# --------------------------------------------------------------------------- hedge

def hedge_profile(t=HEDGE_T, h=HEDGE_H, r=0.32, seg=3):
    """Cross-section (lateral, z) from the left foot over the rounded top to the right."""
    half = t / 2
    pts = [(half, 0.0), (half, h * 0.3), (half, h * 0.6), (half, h - r)]
    for i in range(1, seg + 1):
        a = math.radians(90 * i / seg)
        pts.append((half - r + r * math.cos(a), h - r + r * math.sin(a)))
    for i in range(0, seg + 1):
        a = math.radians(90 + 90 * i / seg)
        pts.append((-half + r + r * math.cos(a), h - r + r * math.sin(a)))
    pts += [(-half, h * 0.6), (-half, h * 0.3), (-half, 0.0)]
    return pts


def lumpy(is_seam, amp=0.07):
    """Deterministic position-based bumps (fluffy trimmed hedge); zero on the seam rings
    (so segments join without gaps) and on the ground."""
    def f(co, i, j):
        if is_seam(co) or co.z < 1e-4:
            return zb.Vector((0, 0, 0))
        n = math.sin(9.1 * co.x + 5.3 * co.z + 2.0 * co.y) * math.cos(7.7 * co.y - 4.1 * co.z + 1.3 * co.x)
        m = math.sin(6.3 * co.z + 3.1 * co.x - 5.5 * co.y)
        return zb.Vector((amp * m * 0.7, amp * n * 0.7, amp * n))
    return f


# the lumps add up to ~0.05 m, so the base shape is slightly smaller than 3.0 x 1.0
HEDGE_PROFILE = dict(t=HEDGE_T - 0.05, h=HEDGE_H - 0.04)


def hedge():
    path = [(-1.0 + 0.2 * i, 0.0) for i in range(11)]
    seam = lambda co: abs(abs(co.x) - 1.0) < 1e-4  # noqa: E731
    return [zb.sweep(hedge_profile(**HEDGE_PROFILE), path, color="hedge", jitter=lumpy(seam))]


def hedge_corner():
    path = [(1.0 - 0.25 * i, 0.0) for i in range(4)] + [(0.0, 0.0)] + \
           [(0.0, NORTH * 0.25 * i) for i in range(1, 5)]
    seam = lambda co: abs(co.x - 1.0) < 1e-4 or abs(co.y - NORTH) < 1e-4  # noqa: E731
    return [zb.sweep(hedge_profile(**HEDGE_PROFILE), path, color="hedge", jitter=lumpy(seam))]


# --------------------------------------------------------------------------- wall

def wall_block(x0, x1, z0, z1, y0, y1, color, gap=0.06, r=0.13):
    """A stone through the wall: rounded rectangle in the X/Z plane extruded along Y."""
    poly = zb.rounded_rect(x0 + gap / 2, z0 + gap / 2, x1 - gap / 2, z1 - gap / 2, r, 1)
    p = zb.slab(poly, 0.0, y1 - y0, color=color)  # polygon in XY, extruded along Z
    p.rotate(90, "X")                              # (x, y, z) -> (x, -z, y)
    p.move(0, y1, 0)
    # flat tops/bottoms are hidden by the next course / the cap / the ground
    p.delete_faces(lambda f: abs(f.normal.z) > 0.99)
    return p


def course_cuts(length, k, n_full=3):
    """Block boundaries along a course; odd courses are offset by half a block."""
    step = length / n_full
    if k % 2 == 0:
        return [i * step for i in range(n_full + 1)]
    return [0.0] + [step / 2 + i * step for i in range(n_full)] + [length]


def cap_profile(t=CAP_T, h=CAP_H, z0=WALL_H - CAP_H, b=0.05):
    half = t / 2
    return [(half, z0), (half, z0 + h - b), (half - b, z0 + h), (-half + b, z0 + h),
            (-half, z0 + h - b), (-half, z0)]


def zoo_wall(seed=3):
    rnd = random.Random(seed)
    body_h = WALL_H - CAP_H
    courses = 4
    ch = body_h / courses
    half = WALL_T / 2
    parts = [zb.box((2.0, WALL_T - 0.24, body_h), (0, 0, body_h / 2), color="wall_mortar")]
    for k in range(courses):
        cuts = course_cuts(2.0, k)
        for a, b in zip(cuts, cuts[1:]):
            col = rnd.choice(["wall_stone", "wall_stone", "wall_stone_dark"])
            parts.append(wall_block(-1.0 + a, -1.0 + b, k * ch, (k + 1) * ch, -half, half, col))
    parts.append(zb.sweep(cap_profile(), [(-1.0, 0.0), (1.0, 0.0)], color="wood_light"))
    return parts


def zoo_wall_corner(seed=5):
    rnd = random.Random(seed)
    body_h = WALL_H - CAP_H
    courses = 4
    ch = body_h / courses
    half = WALL_T / 2
    core_t = WALL_T - 0.24
    parts = [
        zb.box((1.0 + core_t / 2, core_t, body_h), ((1.0 - core_t / 2) / 2, 0, body_h / 2), color="wall_mortar"),
        zb.box((core_t, 1.0 - core_t / 2, body_h), (0, NORTH * (1.0 + core_t / 2) / 2, body_h / 2),
               color="wall_mortar"),
    ]
    for k in range(courses):
        z0, z1 = k * ch, (k + 1) * ch
        # the corner square belongs to the east arm on even courses, to the north arm on odd
        east_start = -half if k % 2 == 0 else half
        north_start = half if k % 2 == 0 else -half   # measured along game north
        # east arm (along +X)
        L = 1.0 - east_start
        n = 2
        for i in range(n):
            a = east_start + L * i / n
            b = east_start + L * (i + 1) / n
            col = rnd.choice(["wall_stone", "wall_stone", "wall_stone_dark"])
            parts.append(wall_block(a, b, z0, z1, -half, half, col))
        # north arm (along Blender -Y): build along X then rotate into place
        L = 1.0 - north_start
        n = 1 if k % 2 == 0 else 2
        for i in range(n):
            a = north_start + L * i / n
            b = north_start + L * (i + 1) / n
            col = rnd.choice(["wall_stone", "wall_stone", "wall_stone_dark"])
            p = wall_block(a, b, z0, z1, -half, half, col)
            p.rotate_z(-90)  # +X -> -Y (game north)
            parts.append(p)
    parts.append(zb.sweep(cap_profile(), [(1.0, 0.0), (0.0, 0.0), (0.0, NORTH * 1.0)], color="wood_light"))
    return parts


# --------------------------------------------------------------------------- main

def instance(src, name, loc=(0, 0, 0), rot_deg=0.0):
    import bpy
    o = bpy.data.objects.new(name, src.data)
    o.location = loc
    o.rotation_euler = (0, 0, math.radians(rot_deg))
    bpy.context.scene.collection.objects.link(o)
    return o


def main():
    args = zb.script_args()
    zb.clean_scene()
    builders = {
        "hedge": hedge,
        "hedge_corner": hedge_corner,
        "zoo_wall": zoo_wall,
        "zoo_wall_corner": zoo_wall_corner,
        "fence_wood": fence_wood,
        "fence_wood_corner": fence_wood_corner,
        "fence_wood_end": fence_wood_end,
        "gate_wood": gate_wood,
    }
    objs = {name: zb.build_object(name, b()) for name, b in builders.items()}
    ok = zb.report(objs.values())
    for o in objs.values():
        zb.export_glb(o, zb.repo_path("assets", "models", "props", o.name + ".glb"))

    order = list(objs.values())
    zb.layout_grid(order, cols=4, spacing=(3.3, 4.2))
    zb.save_blend(zb.repo_path("assets", "blender", "props", KIT + ".blend"))
    # preview only: turn the corner pieces so the inside of the L faces the camera
    for n in ("hedge_corner", "zoo_wall_corner", "fence_wood_corner"):
        objs[n].rotation_euler.z = math.radians(90)

    if "--no-preview" not in args:
        # sample assembly (seams + placement conventions): wall and hedge runs with
        # corners, fence corner + segment + half-open gate + end post
        fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
        B = right * 15.0 - fwd * 3.0
        V = zb.Vector
        extra = [
            instance(objs["zoo_wall_corner"], "s_wall_c", B),
            instance(objs["zoo_wall"], "s_wall", B + V((2.0, 0, 0))),
            instance(objs["zoo_wall"], "s_wall_n", B + V((0, NORTH * 2.0, 0)), rot_deg=90),
            instance(objs["hedge_corner"], "s_hedge_c", B + V((-4.5, 0, 0))),
            instance(objs["hedge"], "s_hedge", B + V((-2.5, 0, 0))),
            instance(objs["fence_wood_corner"], "s_fence_c", B + V((-3.0, 3.5, 0))),
            instance(objs["fence_wood"], "s_fence", B + V((-1.0, 3.5, 0))),
            instance(objs["gate_wood"], "s_gate", B + V((0.09, 3.5, 0)), rot_deg=60),
            instance(objs["fence_wood_end"], "s_end", B + V((2.0, 3.5, 0))),
        ]
        zb.render_preview(order, zb.repo_path("art", "props", KIT, "model_preview.png"), extra_objs=extra)
    if not ok:
        sys.exit(1)


if __name__ == "__main__":
    main()
