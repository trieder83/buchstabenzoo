"""Kit night — lamps and the moon door (concept: art/props/kit_night/sheet_lights_v2.jpg,
sheet_lights_night_v1.jpg, sheet_moon_door_v2.jpg; brief.md; art/night/README.md).

Run:  blender -b --factory-startup --python tools/blender/props/kit_night.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/<asset>.glb, assets/blender/props/kit_night.blend,
         art/props/kit_night/model_preview.png (day on top, night-tinted below)

Conventions (night_lib.py, tools/blender/props/README_night.md): glowing parts are `*_glow`
material slots (day colour from the palette, emissiveFactor = night glow), every lamp has a
`light` empty (point-light position; radius in README_night.md), moving parts are child
nodes with a translation-only pivot. Front = south = glTF +Z at yaw 0.

Assets
- lantern_post   2.4 m post, arm towards the FRONT (+Z at yaw 0 = the `facing` of [[light]]),
                 lantern hanging 0.55 m in front of the post. Origin = post foot.
- string_lights  one 6 m span: a 2.8 m post at x = 0 and the cord with 10 bulbs to the hook
                 point at x = +6 (where the next span's post or a `string_post` stands).
- string_post    the lone end post of a string-light line (same post as in string_lights).
- wall_lamp      bracket lantern for facades; the wall plane is glTF z = 0 (the lamp sticks out
                 towards +Z); origin = on the wall plane below the lamp (min y = 0); mount it
                 with its origin 1.75 m above the ground.
- board_lamp     brass clip lamp; origin = bottom centre of the clip; sockets in README_night.md.
- hand_lantern   player's toy lantern; origin at its foot, `socket_handle` empty = grip point.
- moon_door      pillars + beam + moon sign (root), leaves `leaf_l` / `leaf_r` as child nodes
                 hinged at their outer edges; `moon_door_open` = same with the leaves baked
                 open 90 deg (for a renderer without node transforms).
"""

import math
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import night_lib as nl  # noqa: E402
import props_parts as pp  # noqa: E402
from night_lib import zb, FRONT, BACK  # noqa: E402
from mathutils import Matrix, Vector  # noqa: E402

KIT = "kit_night"


# --------------------------------------------------------------------------- lantern head

def lantern(cx, cy, z0, w=0.26, h=0.30, roof=0.16, ring=True, cheap=False):
    """Old-fashioned four-sided lantern standing on z0: base plate, glass box (glow), four
    corner bars, pyramid roof, top knob/ring. Returns (palette parts, glow parts, centre z)."""
    pal, glow = [], []
    pal.append(zb.box((w + 0.06, w + 0.06, 0.05), (cx, cy, z0 + 0.025), color="lantern_metal",
                      bevel=0.0 if cheap else 0.012))
    gz0, gz1 = z0 + 0.05, z0 + 0.05 + h
    glow.append(zb.box((w - 0.02, w - 0.02, h), (cx, cy, (gz0 + gz1) / 2), color="lamp_glass"))
    b = 0.035
    for sx in (-1, 1):
        for sy in (-1, 1):
            pal.append(zb.box((b, b, h), (cx + sx * (w / 2 - b / 2 + 0.005), cy + sy * (w / 2 - b / 2 + 0.005),
                                         (gz0 + gz1) / 2), color="lantern_metal"))
    pal.append(zb.box((w + 0.04, w + 0.04, 0.04), (cx, cy, gz1 + 0.02), color="lantern_metal"))
    pal.append(pp.cyl((w + 0.1) / 2 * math.sqrt(2), gz1 + 0.04, gz1 + 0.04 + roof, sides=4, apex=True,
                      center=(cx, cy), color="lantern_metal", phase=math.pi / 4))
    if ring:
        rr = nl.ring_xz(cx, gz1 + roof + 0.07, 0.045, 6, cy - 0.012, cy + 0.012, "lantern_metal", r_in=0.025)
        pal.append(rr)
    return pal, glow, (gz0 + gz1) / 2


# --------------------------------------------------------------------------- lantern_post

def lantern_post():
    A = nl.Asset("lantern_post")
    H = 2.4
    parts = [zb.box((0.17, 0.17, H), (0, 0, H / 2), color="wood", bevel=0.03, bevel_edges="vertical")]
    parts.append(zb.box((0.26, 0.26, 0.12), (0, 0, 0.06), color="wood_dark", bevel=0.02, bevel_edges="top"))
    parts.append(zb.box((0.2, 0.2, 0.05), (0, 0, H + 0.02), color="wood_dark", bevel=0.015))
    # arm towards the front (-Y) and a knee brace
    ay = FRONT * 0.62
    parts.append(zb.box((0.1, 0.72, 0.12), (0, FRONT * 0.3, H - 0.18), color="wood", bevel=0.02,
                        bevel_edges="long"))
    parts.append(pp.beam((0, FRONT * 0.08, H - 0.62), (0, FRONT * 0.42, H - 0.22), 0.08, color="wood_dark"))
    # hook under the arm end, lantern hanging below
    parts.append(pp.beam((0, ay + BACK * 0.05, H - 0.24), (0, ay + BACK * 0.05, H - 0.34), 0.025,
                         color="lantern_metal"))
    pal, glow, zc = lantern(0, ay + BACK * 0.05, H - 0.86)
    A.add(parts + pal)
    A.add(glow, mat="lamp_glow")
    A.empty("light", (0, ay + BACK * 0.05, zc), light=(3.0, "#FFC46E"))
    return A


# --------------------------------------------------------------------------- string lights

POST_H = 2.8
SPAN = 6.0


def string_post_parts(x=0.0):
    p = [zb.box((0.13, 0.13, POST_H), (x, 0, POST_H / 2), color="wood", bevel=0.025, bevel_edges="vertical")]
    p.append(zb.box((0.16, 0.16, 0.05), (x, 0, POST_H + 0.02), color="wood_dark", bevel=0.012))
    # hook ring the cord hangs from
    p.append(nl.ring_xz(x, POST_H - 0.12, 0.05, 6, -0.075, -0.055, "lantern_metal", r_in=0.03))
    return p


def string_post():
    A = nl.Asset("string_post")
    A.add(string_post_parts())
    return A


def string_lights():
    A = nl.Asset("string_lights")
    A.add(string_post_parts())
    zt = POST_H - 0.16
    sag = 0.42
    n = 14
    pts = []
    for i in range(n + 1):
        t = i / n
        pts.append(Vector((SPAN * t, -0.065, zt - sag * 4 * t * (1 - t))))
    cord = [pp.beam(pts[i], pts[i + 1], 0.02, sides=3, color="cord", caps=False) for i in range(n)]
    A.add(cord, keep_down=True)
    mats = ["bulb_glow", "bulb_orange_glow", "bulb_cream_glow"]
    cells = ["bulb_yellow", "bulb_orange", "lamp_glass"]
    nb = 10
    for k in range(nb):
        t = (k + 0.5) / nb
        x = SPAN * t
        z = zt - sag * 4 * t * (1 - t)
        A.add(zb.knob(0.08, (x, -0.065, z - 0.09), color=cells[k % 3], u=5, v=3, squash=1.2),
              mat=mats[k % 3])
    return A


# --------------------------------------------------------------------------- wall lamp

def wall_lamp():
    """Wall plane = Blender y = 0 (the wall is behind, +Y); lamp sticks out to the front."""
    A = nl.Asset("wall_lamp")
    parts = [zb.box((0.14, 0.05, 0.3), (0, FRONT * 0.025, 0.62), color="wood", bevel=0.015)]
    parts.append(zb.box((0.07, 0.34, 0.08), (0, FRONT * 0.2, 0.7), color="wood", bevel=0.015,
                        bevel_edges="long"))
    parts.append(pp.beam((0, FRONT * 0.05, 0.5), (0, FRONT * 0.24, 0.67), 0.05, color="wood_dark"))
    parts.append(pp.beam((0, FRONT * 0.32, 0.66), (0, FRONT * 0.32, 0.58), 0.02, color="lantern_metal"))
    pal, glow, zc = lantern(0, FRONT * 0.32, 0.0, w=0.2, h=0.24, roof=0.12)
    A.add(parts + pal)
    A.add(glow, mat="lamp_glow")
    A.empty("light", (0, FRONT * 0.32, zc), light=(2.0, "#FFC46E"))
    return A


# --------------------------------------------------------------------------- board lamp

def board_lamp():
    """Brass clip lamp. Origin = bottom centre of the clip (clamped onto the info-board roof
    ridge); the arm reaches 0.5 m to the front (-Y) and the shade looks down/back."""
    A = nl.Asset("board_lamp")
    parts = [zb.box((0.1, 0.12, 0.09), (0, 0, 0.045), color="brass", bevel=0.015)]
    pts = [Vector((0, 0.0, 0.08)), Vector((0, 0.0, 0.2)), Vector((0, FRONT * 0.1, 0.28)),
           Vector((0, FRONT * 0.3, 0.3)), Vector((0, FRONT * 0.44, 0.26))]
    for a, b in zip(pts, pts[1:]):
        parts.append(pp.beam(a, b, 0.03, sides=6, color="brass"))
    # shade: a cone opening downwards-back towards the panel
    shade = pp.cyl(0.03, 0.0, 0.1, sides=8, r1=0.1, color="brass")  # narrow top at z 0 -> wide at 0.1
    inner = pp.disc((0, 0, 0.1), 0.085, "lamp_glass", sides=8)
    for p in (shade, inner):
        p.transform(Matrix.Rotation(math.pi, 4, "X"))              # wide end down
        p.transform(Matrix.Rotation(math.radians(-35 * FRONT), 4, "X"))  # tilt towards the back
        p.move(0, FRONT * 0.47, 0.24)
    pp.recolor(shade, lambda f: True, "brass")
    A.add(parts + [shade])
    A.add(inner, mat="lamp_glow")
    A.empty("light", (0, FRONT * 0.43, 0.12), light=(1.2, "#FFC46E"))
    return A


# --------------------------------------------------------------------------- hand lantern

def hand_lantern():
    A = nl.Asset("hand_lantern")
    p = []
    p.append(pp.cyl(0.085, 0.0, 0.04, sides=10, r1=0.08, color="lantern_cream"))
    p.append(pp.cyl(0.075, 0.04, 0.07, sides=10, color="lantern_red"))
    glow = pp.cyl(0.07, 0.07, 0.2, sides=10, color="lamp_glass")
    # belly: widen the middle ring
    p.append(pp.cyl(0.078, 0.2, 0.25, sides=10, r1=0.06, color="lantern_red"))
    p.append(pp.cyl(0.06, 0.25, 0.27, sides=10, r1=0.045, color="lantern_cream"))
    # two guard wires
    for sx in (-1, 1):
        p.append(pp.beam((sx * 0.085, 0, 0.06), (sx * 0.085, 0, 0.21), 0.018, sides=4, color="lantern_cream"))
    # carrying ring (in the X/Z plane)
    ring = nl.ring_xz(0, 0.33, 0.06, 8, -0.012, 0.012, "lantern_cream", r_in=0.038)
    p.append(ring)
    p.append(zb.knob(0.018, (0.09, FRONT * 0.03, 0.03), color="lantern_cream", u=6, v=3))
    A.add(p)
    A.add(glow, mat="lamp_glow")
    A.empty("socket_handle", (0, 0, 0.385))
    A.empty("light", (0, 0, 0.135), light=(2.5, "#FFC46E"))
    return A


# --------------------------------------------------------------------------- moon door

OPEN_HALF = 1.0     # opening x -1 .. +1 (2 m door rect)
PIL_W, PIL_D = 0.7, 0.9
PIL_X = OPEN_HALF + PIL_W / 2
PIL_H = 3.2
LEAF_T = 0.08
HINGE_X = 0.96
HINGE_Y = BACK * LEAF_T / 2


def pillar(cx):
    parts = [zb.box((PIL_W - 0.06, PIL_D - 0.06, PIL_H), (cx, 0, PIL_H / 2), color="wall_mortar")]
    courses = 4
    ch = PIL_H / courses
    for k in range(courses):
        z = ch * (k + 0.5)
        col = ("wall_stone", "wall_stone_dark")
        for s in (-1, 1):
            if k % 2 == 0:  # split across x
                w = PIL_W / 2 - 0.02
                c = (cx + s * (PIL_W / 4), 0, z)
                size = (w, PIL_D, ch - 0.04)
            else:           # split across y
                c = (cx, s * PIL_D / 4, z)
                size = (PIL_W, PIL_D / 2 - 0.02, ch - 0.04)
            parts.append(zb.box(size, c, color=col[(k + (s > 0)) % 2], bevel=0.05, bevel_edges="vertical"))
    parts.append(zb.box((PIL_W + 0.14, PIL_D + 0.14, 0.2), (cx, 0, PIL_H + 0.1), color="wood",
                        bevel=0.03, bevel_edges="top+vertical"))
    return parts


def flat_star(cx, cz, r, y, facing, phase=0.0):
    """Flat 5-point star in the X/Z plane at y, facing `facing` (FRONT or BACK)."""
    pts = []
    for i in range(10):
        rr = r if i % 2 == 0 else r * 0.45
        a = math.pi / 2 + phase + math.pi * i / 5
        pts.append((cx + rr * math.cos(a), y, cz + rr * math.sin(a)))
    p = pp.flat(pts, "star_paint")
    p.bm.normal_update()
    if p.bm.faces[:][0].normal.y * facing < 0:
        import bmesh
        bmesh.ops.reverse_faces(p.bm, faces=p.bm.faces[:])
    return p


def star_prism(cx, cz, r, y0, y1, phase=0.0):
    pts = []
    for i in range(10):
        rr = r if i % 2 == 0 else r * 0.45
        a = math.pi / 2 + phase + math.pi * i / 5
        pts.append((cx + rr * math.cos(a), cz + rr * math.sin(a)))
    return pp.prism_xz(pts, y0, y1, "star_paint")


def leaf_parts(side):
    """One door leaf in asset coordinates. side -1 = left (hinge at x = -HINGE_X)."""
    x_h = side * HINGE_X
    x_c = side * 0.01
    xa, xb = min(x_h, x_c), max(x_h, x_c)
    z0 = 0.05

    def top(x):  # arched top: low at the hinge (2.35), high at the middle (2.65)
        t = abs(x) / HINGE_X
        return 2.65 - 0.3 * t * t

    parts = []
    n = 6
    xs = [xa + (xb - xa) * i / n for i in range(n + 1)]
    fr = 0.09
    # plank body (inside the frame), front/back faces blue
    poly = [(xa + fr, z0 + fr)] + [(x, top(x) - fr) for x in xs[::-1] if xa + fr <= x <= xb - fr]
    poly = [(xa + fr, z0 + fr), (xb - fr, z0 + fr), (xb - fr, top(xb - fr) - fr)] + \
           [(x, top(x) - fr) for x in reversed(xs[1:-1])] + [(xa + fr, top(xa + fr) - fr)]
    parts.append(pp.prism_xz(poly, -LEAF_T / 2 + 0.01, LEAF_T / 2 - 0.01, "door_blue"))
    # plank grooves on both faces
    for k in range(1, 3):
        gx = xa + fr + (xb - xa - 2 * fr) * k / 3
        gh = top(gx) - fr - (z0 + fr)
        for fy in (FRONT, BACK):
            parts.append(zb.box((0.025, 0.012, gh - 0.04), (gx, fy * (LEAF_T / 2 - 0.006), z0 + fr + gh / 2),
                                color="door_blue_dark"))
    # frame: stiles, bottom rail, arched top rail
    parts.append(zb.box((fr, LEAF_T, top(xa) - z0), (xa + fr / 2, 0, (z0 + top(xa)) / 2), color="wood"))
    parts.append(zb.box((fr, LEAF_T, top(xb) - z0), (xb - fr / 2, 0, (z0 + top(xb)) / 2), color="wood"))
    parts.append(zb.box((xb - xa - 2 * fr, LEAF_T, fr), ((xa + xb) / 2, 0, z0 + fr / 2), color="wood"))
    arc = [(x, top(x)) for x in xs] + [(x, top(x) - fr) for x in reversed(xs)]
    parts.append(pp.prism_xz(arc, -LEAF_T / 2, LEAF_T / 2, "wood"))
    # hinge straps on the hinge side (both faces)
    for zh in (0.5, 1.9):
        for fy in (FRONT, BACK):
            parts.append(zb.box((0.26, 0.015, 0.06), (x_h - side * 0.13, fy * (LEAF_T / 2 + 0.006), zh),
                                color="iron"))
    # painted stars, front and back
    stars = [(0.45, 1.85, 0.15), (0.72, 1.25, 0.12), (0.35, 0.7, 0.14)]
    for fx, fz, r in stars:
        x = x_h - side * fx * (HINGE_X - 0.1)
        for fy in (FRONT, BACK):
            parts.append(flat_star(x, fz, r, fy * (LEAF_T / 2 - 0.01 + 0.016), fy, phase=0.2 * side))
    # knob handle near the middle, both faces
    for fy in (FRONT, BACK):
        parts.append(zb.knob(0.04, (x_c - side * 0.18, fy * (LEAF_T / 2 + 0.02), 1.25), color="iron", u=4, v=2))
    return parts


def moon_door_asset(name, open_deg=0.0):
    A = nl.Asset(name)
    parts = pillar(-PIL_X) + pillar(PIL_X)
    # beam between the pillar caps
    BZ = PIL_H + 0.12
    parts.append(zb.box((2 * PIL_X + 0.3, 0.34, 0.32), (0, 0, BZ + 0.16), color="wood_light", bevel=0.03,
                        bevel_edges="long"))
    for sx in (-1, 1):
        parts.append(zb.box((0.12, 0.36, 0.36), (sx * (PIL_X + 0.25), 0, BZ + 0.16), color="wood_dark"))
    # moon sign in front of the beam: wooden ring frame, dark-blue disc, glowing crescent and
    # rim ring (front and back)
    R = 0.62
    SZ = BZ + 0.32 + R * 0.55
    SY = FRONT * 0.22
    parts.append(nl.ring_xz(0, SZ, R, 14, SY - 0.1, SY + 0.1, "wood", r_in=R - 0.12))
    parts.append(nl.ring_xz(0, SZ, R - 0.1, 14, SY - 0.07, SY + 0.07, "moon_sign_bg"))
    rim, moon = [], []
    for fy in (FRONT, BACK):
        ring = []
        for i in range(14):
            a = 2 * math.pi * i / 14 + math.pi / 14
            ring.append((math.cos(a), math.sin(a)))
        yv = SY + fy * 0.075
        bm_pts_o = [(0.49 * c, yv, SZ + 0.49 * s_) for c, s_ in ring]
        bm_pts_i = [(0.43 * c, yv, SZ + 0.43 * s_) for c, s_ in ring]
        for i in range(14):
            j = (i + 1) % 14
            q = pp.flat([bm_pts_o[i], bm_pts_o[j], bm_pts_i[j], bm_pts_i[i]], "door_blue")
            q.bm.normal_update()
            if q.bm.faces[:][0].normal.y * fy < 0:
                import bmesh
                bmesh.ops.reverse_faces(q.bm, faces=q.bm.faces[:])
            rim.append(q)
        pts = []
        n = 7
        for i in range(n + 1):  # outer arc of the crescent
            a = math.radians(60 + 240 * i / n)
            pts.append((0.34 * math.cos(a) - 0.03, SZ + 0.34 * math.sin(a)))
        for i in range(n + 1):  # inner arc back
            a = math.radians(300 - 240 * i / n)
            pts.append((0.26 * math.cos(a) + 0.1, SZ + 0.05 + 0.27 * math.sin(a)))
        pts = pts if fy == FRONT else [(-x, z) for x, z in pts]
        y0, y1 = sorted((SY + fy * 0.07, SY + fy * 0.09))
        moon.append(pp.prism_xz(pts, y0, y1, "moon_cream"))
    A.add(parts)
    A.add(moon, mat="moon_glow")
    A.add(rim, mat="rim_glow")
    # lanterns on the caps
    for sx, nm in ((-1, "light_l"), (1, "light_r")):
        pal, glow, zc = lantern(sx * PIL_X, 0, PIL_H + 0.2, w=0.3, h=0.34, roof=0.18, ring=False, cheap=True)
        A.add(pal)
        A.add(glow, mat="lamp_glow")
        A.empty(nm, (sx * PIL_X, 0, zc), light=(2.5, "#FFC46E"))
    # leaves as child nodes, pivot = hinge axis foot
    for side, nm in ((-1, "leaf_l"), (1, "leaf_r")):
        piv = Vector((side * HINGE_X, HINGE_Y, 0))
        lp = leaf_parts(side)
        if open_deg:
            ang = math.radians(-side * open_deg)  # left +90 (towards +Y = back), right -90
            m = Matrix.Translation(piv) @ Matrix.Rotation(ang, 4, "Z") @ Matrix.Translation(-piv)
            for p in lp:
                p.transform(m)
        A.node(nm, pivot=piv)
        A.add(lp, node=nm)
    return A


def moon_door():
    return moon_door_asset("moon_door")


def moon_door_open():
    return moon_door_asset("moon_door_open", open_deg=90.0)


# --------------------------------------------------------------------------- preview extras

def extra(roots):
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    B = right * 15.0 - fwd * 0.0
    V = Vector
    out = [nl.instance_tree(roots["moon_door"], "s_moon_half", B, node_rot={"leaf_l": 45, "leaf_r": -45})]
    # info board + map board with their board lamps (sockets of README_night.md)
    try:
        import bpy
        for nm in ("info_board", "map_board"):
            bpy.ops.import_scene.gltf(filepath=zb.repo_path("assets", "models", "props", nm + ".glb"))
            ob = [o for o in bpy.context.selected_objects if o.type == "MESH"][0]
            ob.name = "s_" + nm
            base = B + V((4.5 if nm == "info_board" else 7.0, 0, 0))
            ob.location = base
            out.append(ob)
            sock = BOARD_SOCKETS[nm]
            out.append(nl.instance_tree(roots["board_lamp"], "s_lamp_" + nm,
                                        base + V((sock[0], -sock[2], sock[1]))))
    except Exception as e:  # noqa: BLE001
        print("preview: boards not loaded:", e)
    return out


# board_lamp origin in the board's model space (glTF x, y, z) — README_night.md
BOARD_SOCKETS = {
    "info_board": (0.0, 1.405, -0.236),
    "map_board": (0.0, 1.93, -0.09),
}


def main():
    builders = {
        "lantern_post": lantern_post,
        "wall_lamp": wall_lamp,
        "board_lamp": board_lamp,
        "hand_lantern": hand_lantern,
        "string_post": string_post,
        "string_lights": string_lights,
        "moon_door": moon_door,
        "moon_door_open": moon_door_open,
    }
    nl.run(KIT, builders, cols=4, spacing=(4.0, 6.0), budgets={"*": 500, "moon_door": 1500,
                                                                "moon_door_open": 1500},
           extra=extra)


if __name__ == "__main__":
    main()
