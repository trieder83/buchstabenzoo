"""Kit 6 — barriers (concept: art/props/kit_barriers/sheet_v1.jpg, brief.md).

Run:  blender -b --factory-startup --python tools/blender/props/kit_barriers.py -- [--no-preview]

Outputs
  assets/models/props/<asset>.glb     one per asset
  assets/blender/props/kit_barriers.blend
  art/props/kit_barriers/model_preview.png

Placement conventions (world axes, Q-056: +X east, +Y up, north = world -Z = Blender +Y;
never mirror, only rotate about +Y — +90 deg turns the front from south to east, -90 deg
from south to west; origin on the ground):
- The front (striped face, sign face, cart side with the tools, gate straps) faces SOUTH
  (world +Z, towards the default camera) at yaw 0.
- road_block: 2 m board along X, origin at its centre. Level 1 barrier_east_repair (the
  path comes from the west): yaw -90 so it stands across the path, front to the west.
- repair_sign: single post, origin at the post foot; panel with a shovel pictogram (no text,
  modelled — it does not change with the language).
- zookeeper_cart: 1.9 m long along X, tow handle at +X; origin = centre of the footprint.
- traffic_cone: 0.5 m, origin at the centre of its base.
- fallen_tree: lies along Y (north-south), roots at the south end, crown to the north;
  3.8 m long, origin at the centre of the 2 x 3 barrier rect (level 1 barrier_ne_tree at
  yaw 0 blocks the W-E path_ne; the crown reaches ~0.7 m into the hedge to the north).
- gate_zoo_closed: 3.0 m wide (X) between two stone pillars, doors closed with a padlock;
  origin at the centre of the gate line. Level 1 barrier_north_gate: yaw 0 (front south).
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import props_parts as pp  # noqa: E402
from props_parts import zb  # noqa: E402

KIT = "kit_barriers"
F = pp.FRONT  # -1: Blender -Y = south


def clip_rect(poly, x0, x1, z0, z1):
    """Sutherland-Hodgman clip of a convex polygon [(x, z)] to a rectangle."""
    def clip(pts, inside, inter):
        out = []
        for i, p in enumerate(pts):
            q = pts[i - 1]
            if inside(p):
                if not inside(q):
                    out.append(inter(q, p))
                out.append(p)
            elif inside(q):
                out.append(inter(q, p))
        return out

    def ix(xc):
        return lambda a, b: (xc, a[1] + (b[1] - a[1]) * (xc - a[0]) / (b[0] - a[0]))

    def iz(zc):
        return lambda a, b: (a[0] + (b[0] - a[0]) * (zc - a[1]) / (b[1] - a[1]), zc)

    pts = poly
    pts = clip(pts, lambda p: p[0] >= x0, ix(x0))
    pts = clip(pts, lambda p: p[0] <= x1, ix(x1)) if pts else pts
    pts = clip(pts, lambda p: p[1] >= z0, iz(z0)) if pts else pts
    pts = clip(pts, lambda p: p[1] <= z1, iz(z1)) if pts else pts
    return pts


def road_block():
    L, Z0, Z1, T = 1.9, 0.72, 1.0, 0.08
    parts = []
    sw = 0.24  # stripe width along X
    skew = Z1 - Z0
    x = -L / 2 - skew
    i = 0
    while x < L / 2:
        poly = [(x, Z0), (x + sw, Z0), (x + sw + skew, Z1), (x + skew, Z1)]
        c = clip_rect(poly, -L / 2, L / 2, Z0, Z1)
        if len(c) >= 3:
            parts.append(pp.prism_xz(c, -T / 2, T / 2, "stripe_red" if i % 2 == 0 else "stripe_white"))
        x += sw
        i += 1
    # wooden end caps
    for s in (-1, 1):
        parts.append(zb.box((0.1, T + 0.04, Z1 - Z0 + 0.06), (s * (L / 2 + 0.05), 0, (Z0 + Z1) / 2), color="wood",
                            bevel=0.015))
    # sawhorse legs: two per end, splayed front/back
    for s in (-1, 1):
        for d in (-1, 1):
            parts.append(pp.beam((s * 0.72, d * 0.02, Z0 + 0.02), (s * 0.8, d * 0.36, 0.0), 0.09, color="wood"))
        parts.append(zb.box((0.07, 0.5, 0.06), (s * 0.77, 0, 0.3), color="wood_dark"))
    return parts


def repair_sign():
    parts = [zb.box((0.12, 0.12, 1.25), (0, -F * 0.05, 0.625), color="wood", bevel=0.02)]
    W, H = 0.72, 0.56
    panel = [zb.box((W, 0.06, H), (0, 0, 0), color="wood_light", bevel=0.015)]
    for k in (-1, 0, 1):  # plank seams on the frame edge
        panel.append(zb.box((W + 0.01, 0.065, 0.015), (0, 0, k * H / 3.2), color="wood_dark"))
    face = zb.box((W - 0.14, 0.012, H - 0.14), (0, F * 0.034, 0), color="wood")
    pp.recolor(face, lambda f: f.normal.y * F > 0.9, "sign_blue")
    panel.append(face)
    # shovel pictogram (flat, slightly in front of the blue face), diagonal like on the sheet
    y = F * 0.042
    pic = []
    pic.append([(-0.018, 0.02), (0.018, 0.02), (0.018, 0.2), (-0.018, 0.2)])                   # handle
    pic.append([(-0.06, 0.2), (0.06, 0.2), (0.06, 0.235), (-0.06, 0.235)])                     # grip
    pic.append([(-0.065, 0.02), (0.065, 0.02), (0.065, -0.1), (0.0, -0.17), (-0.065, -0.1)])   # blade
    ang = math.radians(-30)
    for poly in pic:
        pts = []
        for px, pz in poly:
            rx = px * math.cos(ang) - pz * math.sin(ang)
            rz = px * math.sin(ang) + pz * math.cos(ang)
            pts.append((rx, y, rz))
        p = pp.flat(pts, "pictogram")
        p.bm.normal_update()
        if list(p.bm.faces)[0].normal.y * F < 0:
            import bmesh
            bmesh.ops.reverse_faces(p.bm, faces=list(p.bm.faces))
        panel.append(p)
    for p in panel:
        p.rotate(15 * F, "X")
        p.move(0, 0.0, 1.05)
    parts += panel
    return parts


def wheel(x, y, r=0.2, w=0.1):
    t = pp.cyl(r, -w / 2, w / 2, sides=8, color="tyre")
    pp.recolor(t, lambda f: abs(f.normal.z) > 0.9, "metal")
    t.rotate(90, "X").move(x, y, r)
    return t


def zookeeper_cart():
    L, W, Z = 1.8, 0.9, 0.34
    parts = [zb.box((L, W, 0.1), (0, 0, Z + 0.05), color="cart_green")]
    rim = 0.18
    for s in (-1, 1):
        parts.append(zb.box((L, 0.07, rim), (0, s * (W / 2 - 0.035), Z + 0.1 + rim / 2), color="cart_green",
                            bevel=0.015))
        parts.append(zb.box((0.07, W - 0.14, rim), (s * (L / 2 - 0.035), 0, Z + 0.1 + rim / 2), color="cart_green",
                            bevel=0.015))
    parts.append(zb.box((L - 0.2, W + 0.02, 0.05), (0, 0, Z - 0.03), color="cart_green_dark"))
    for sx in (-1, 1):
        for sy in (-1, 1):
            parts.append(wheel(sx * 0.6, sy * (W / 2 + 0.06)))
    # tow handle at +X
    parts.append(pp.beam((L / 2, 0, Z + 0.05), (L / 2 + 0.45, 0, 0.75), 0.05, color="metal"))
    parts.append(pp.beam((L / 2 + 0.45, -0.17, 0.77), (L / 2 + 0.45, 0.17, 0.77), 0.05, color="tyre"))
    zt = Z + 0.1
    # bucket filled with soil
    b = pp.cyl(0.19, zt, zt + 0.3, sides=8, r1=0.25, center=(0.45, 0.1), color="cart_green_dark")
    pp.recolor(b, lambda f: f.normal.z > 0.9, "soil")
    parts.append(b)
    # shovel leaning across the bed
    parts.append(pp.beam((-0.7, F * 0.2, zt + 0.02), (0.3, -F * 0.05, zt + 0.55), 0.04, color="wood_light"))
    blade = zb.box((0.2, 0.03, 0.26), (0, 0, 0), color="metal", bevel=0.01)
    blade.rotate(-60, "Y").move(-0.78, F * 0.22, zt + 0.02)
    parts.append(blade)
    # rake
    parts.append(pp.beam((-0.2, F * 0.33, zt + 0.02), (0.75, F * 0.2, zt + 0.4), 0.035, color="wood_light"))
    head = zb.box((0.05, 0.3, 0.05), (-0.22, F * 0.33, zt + 0.04), color="metal")
    parts.append(head)
    for k in range(4):
        parts.append(pp.beam((-0.24, F * (0.21 + 0.08 * k), zt + 0.04), (-0.3, F * (0.21 + 0.08 * k), zt - 0.04),
                             0.02, color="metal", caps=False))
    return parts


def traffic_cone():
    parts = [zb.slab(zb.rounded_rect(-0.2, -0.2, 0.2, 0.2, 0.05, 1), 0.0, 0.05, color="cone_orange", chamfer=0.015)]
    bands = [(0.05, 0.2, "cone_orange"), (0.2, 0.3, "stripe_white"), (0.3, 0.46, "cone_orange")]
    def rad(z):
        return 0.14 + (0.035 - 0.14) * (z - 0.05) / 0.41
    for z0, z1, c in bands:
        parts.append(pp.cyl(rad(z0), z0, z1, sides=8, r1=rad(z1), color=c, top=(z1 > 0.45)))
    return parts


def fallen_tree(seed=7):
    rnd = random.Random(seed)
    parts = []
    y0, y1 = -1.45, 0.9
    r = 0.3
    t = pp.beam((0, y0, r), (0.05, y1, r * 0.8), 2 * r, color="trunk", sides=8, taper=0.7)
    pp.recolor(t, lambda f: f.normal.y < -0.9, "trunk_dark")
    parts.append(t)
    # roots at the south end
    for i in range(6):
        a = 2 * math.pi * i / 6 + 0.3
        dx, dz = math.cos(a), math.sin(a)
        parts.append(pp.beam((dx * 0.12, y0 + 0.1, r + dz * 0.12),
                             (dx * 0.55, y0 - 0.3 + rnd.uniform(-0.05, 0.05), max(0.0, r + dz * 0.5)),
                             0.13, color="trunk_dark", taper=0.3))
    # branches into the crown
    parts.append(pp.beam((0.0, 0.5, r), (-0.55, 1.35, 0.55), 0.16, color="trunk", taper=0.6))
    parts.append(pp.beam((0.05, 0.6, r), (0.6, 1.25, 0.7), 0.16, color="trunk", taper=0.6))
    # crown lying on its side
    for i, (c, rr, col, sd) in enumerate([((0.0, 1.55, 0.75), (1.0, 0.8, 0.75), "leaf", 2),
                                          ((-0.65, 1.25, 0.55), (0.6, 0.6, 0.55), "leaf_light", 2),
                                          ((0.62, 1.3, 0.62), (0.6, 0.6, 0.6), "leaf_light", 2),
                                          ((0.1, 2.05, 0.5), (0.6, 0.45, 0.5), "leaf", 1)]):
        parts.append(pp.blob(c, rr, col, subdiv=sd, seed=40 + i, amp=0.1, flat_bottom=0.0))
    return parts


def gate_zoo_closed():
    parts = []
    PW, PD, PH = 0.5, 0.6, 2.6
    PX = 1.25
    for s in (-1, 1):
        n = 3
        for k in range(n):
            h = (PH - 0.25) / n
            col = "wall_stone" if k % 2 == 0 else "wall_stone_dark"
            parts.append(zb.box((PW - 0.02 * (k % 2), PD - 0.02 * (k % 2), h - 0.03), (s * PX, 0, k * h + h / 2),
                                color=col, bevel=0.04, bevel_edges="vertical"))
        parts.append(zb.box((PW + 0.12, PD + 0.12, 0.25), (s * PX, 0, PH - 0.125), color="wall_stone",
                            bevel=0.04, bevel_edges="top+vertical"))
    # two door leaves of vertical planks with an arched top
    x0, x1 = -PX + PW / 2, PX - PW / 2
    n = 8
    pw = (x1 - x0) / n
    for i in range(n):
        xc = x0 + pw * (i + 0.5)
        h = 2.0 + 0.35 * math.cos(math.pi / 2 * xc / x1)
        parts.append(zb.box((pw - 0.02, 0.1, h), (xc, 0, h / 2), color="wood_light" if i % 2 else "wood"))
    # iron straps and hinges on the south face
    yf = F * 0.065
    for z in (0.5, 1.55):
        for s in (-1, 1):
            parts.append(zb.box((x1 - 0.12, 0.03, 0.1), (s * (x1 / 2 + 0.02), yf, z), color="iron"))
    # ring handles and padlock at the centre
    for s in (-1, 1):
        parts.append(pp.cyl(0.07, 0.0, 0.03, sides=8, center=(0, 0), color="iron").rotate(-90 * F, "X")
                     .move(s * 0.13, yf, 1.12))
    parts.append(zb.box((0.18, 0.07, 0.15), (0, F * 0.11, 1.0), color="brass", bevel=0.02))
    parts.append(pp.beam((-0.055, F * 0.11, 1.07), (-0.055, F * 0.11, 1.17), 0.03, color="metal"))
    parts.append(pp.beam((0.055, F * 0.11, 1.07), (0.055, F * 0.11, 1.17), 0.03, color="metal"))
    parts.append(pp.beam((-0.07, F * 0.11, 1.17), (0.07, F * 0.11, 1.17), 0.03, color="metal"))
    return parts


def extra(objs):
    # sample: level-1 "path under repair" group — road block across a path, sign, cart, cones
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    B = right * 12.5 - fwd * 0.5
    V = zb.Vector
    return [pp.instance(objs["road_block"], "s_block", B + V((0, 0, 0)), -90),
            pp.instance(objs["repair_sign"], "s_sign", B + V((-0.8, -1.4, 0)), -35),
            pp.instance(objs["zookeeper_cart"], "s_cart", B + V((1.3, 0.3, 0)), 90),
            pp.instance(objs["traffic_cone"], "s_cone1", B + V((-0.5, 1.3, 0))),
            pp.instance(objs["traffic_cone"], "s_cone2", B + V((-0.4, 0.6, 0)))]


def main():
    builders = {
        "road_block": road_block,
        "repair_sign": repair_sign,
        "zookeeper_cart": zookeeper_cart,
        "traffic_cone": traffic_cone,
        "fallen_tree": fallen_tree,
        "gate_zoo_closed": gate_zoo_closed,
    }
    pp.run_kit(KIT, builders, cols=3, spacing=(3.8, 4.0), extra=extra)


if __name__ == "__main__":
    main()
