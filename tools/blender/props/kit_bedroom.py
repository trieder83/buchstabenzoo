"""Kit bedroom — zookeeper-house furniture (concept: art/props/kit_bedroom/sheet_bedroom_v2.jpg,
bedroom_night_v2.jpg, brief.md; GAME-NIGHT rule 3, GAME-CART 12-16).

Run:  blender -b --factory-startup --python tools/blender/props/kit_bedroom.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/<asset>.glb, assets/blender/props/kit_bedroom.blend,
         art/props/kit_bedroom/model_preview.png (day on top, night-tinted below)

Front = south = glTF +Z at yaw 0 = the `facing` of the [[prop]] / [[item]] in the level data
(facing -z -> yaw 0, +x -> +90, -x -> -90, +z -> 180). Origins on the floor / on the surface
the piece stands on; wall pieces (window_moon, key_box) have the wall plane at glTF z = 0 and
stick out to +Z; their origin is the bottom centre and the game lifts them to the mount height
of README_night.md. Sizes follow the [[prop]] size_m of level-1.toml.
"""

import math
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bmesh  # noqa: E402
import night_lib as nl  # noqa: E402
import props_parts as pp  # noqa: E402
from night_lib import zb, FRONT, BACK  # noqa: E402
from mathutils import Matrix, Vector  # noqa: E402

KIT = "kit_bedroom"


def flat_star_up(cx, cy, z, r, color, phase=0.0):
    pts = []
    for i in range(10):
        rr = r if i % 2 == 0 else r * 0.45
        a = math.pi / 2 + phase + math.pi * i / 5
        pts.append((cx + rr * math.cos(a), cy + rr * math.sin(a), z))
    p = pp.flat(pts, color)
    p.bm.normal_update()
    if p.bm.faces[:][0].normal.z < 0:
        bmesh.ops.reverse_faces(p.bm, faces=p.bm.faces[:])
    return p


def flat_star_xz(cx, cz, y, r, facing, color, phase=0.0):
    pts = []
    for i in range(10):
        rr = r if i % 2 == 0 else r * 0.45
        a = math.pi / 2 + phase + math.pi * i / 5
        pts.append((cx + rr * math.cos(a), y, cz + rr * math.sin(a)))
    p = pp.flat(pts, color)
    p.bm.normal_update()
    if p.bm.faces[:][0].normal.y * facing < 0:
        bmesh.ops.reverse_faces(p.bm, faces=p.bm.faces[:])
    return p


def flat_star_yz(cy, cz, x, r, facing, color, phase=0.0):
    pts = []
    for i in range(10):
        rr = r if i % 2 == 0 else r * 0.45
        a = math.pi / 2 + phase + math.pi * i / 5
        pts.append((x, cy + rr * math.cos(a), cz + rr * math.sin(a)))
    p = pp.flat(pts, color)
    p.bm.normal_update()
    if p.bm.faces[:][0].normal.x * facing < 0:
        bmesh.ops.reverse_faces(p.bm, faces=p.bm.faces[:])
    return p


# --------------------------------------------------------------------------- bed

def bed():
    """1.0 x 2.0 m child bed, long axis along X, headboard at +X (east at yaw 0); the front
    long side (+Z glTF) is the side the child uses ([[item]] facing). level-1 bed_l1 has
    facing +z -> yaw 180 -> headboard west, used side north, as in the level data."""
    A = nl.Asset("bed")
    L, W = 2.0, 1.0
    p = []
    # posts: head posts 1.0 m, foot posts 0.7 m
    for sx, h in ((-1, 1.0), (1, 0.72)):
        for sy in (-1, 1):
            p.append(zb.box((0.12, 0.12, h), (sx * (L / 2 - 0.06), sy * (W / 2 - 0.06), h / 2), color="wood",
                            bevel=0.035, bevel_edges="top+vertical"))
    # headboard: arched panel between the head posts (Y/Z plane)
    arc = nl.arc_points(-(W / 2 - 0.1), W / 2 - 0.1, 0.78, 0.95, 6)
    poly = [(-(W / 2 - 0.1), 0.25)] + [(y, z) for y, z in arc] + [(W / 2 - 0.1, 0.25)]
    hb = pp.prism_xz([(y, z) for y, z in poly], -0.03, 0.03, "wood")
    hb.rotate_z(90)  # X/Z polygon -> Y/Z plane (thickness along X)
    hb.move(-(L / 2 - 0.06), 0, 0)
    p.append(hb)
    p.append(flat_star_yz(0, 0.66, -(L / 2 - 0.06) + 0.035, 0.1, 1, "wood_dark"))
    # footboard (lower arch)
    arc = nl.arc_points(-(W / 2 - 0.1), W / 2 - 0.1, 0.52, 0.62, 5)
    poly = [(-(W / 2 - 0.1), 0.2)] + arc + [(W / 2 - 0.1, 0.2)]
    fb = pp.prism_xz(poly, -0.03, 0.03, "wood")
    fb.rotate_z(90)
    fb.move(L / 2 - 0.06, 0, 0)
    p.append(fb)
    # side rails
    for sy in (-1, 1):
        p.append(zb.box((L - 0.2, 0.06, 0.16), (0, sy * (W / 2 - 0.05), 0.28), color="wood_light"))
    # mattress + sheet
    p.append(zb.box((L - 0.2, W - 0.14, 0.18), (0, 0, 0.4), color="sheet_white", bevel=0.04,
                    bevel_edges="top"))
    # pillow at the head
    p.append(zb.box((0.34, W - 0.3, 0.14), (-(L / 2 - 0.3), 0, 0.54), color="pillow", bevel=0.06, segments=2))
    # blanket from the foot up to 0.55 m before the head, turned-down edge
    bx0, bx1 = -(L / 2 - 0.62), L / 2 - 0.08
    p.append(zb.box((bx1 - bx0, W - 0.08, 0.1), ((bx0 + bx1) / 2, 0, 0.48), color="blanket_blue", bevel=0.04,
                    bevel_edges="top"))
    # hanging sides of the blanket
    for sy in (-1, 1):
        p.append(zb.box((bx1 - bx0 - 0.04, 0.03, 0.2), ((bx0 + bx1) / 2, sy * (W / 2 - 0.035), 0.38),
                        color="blanket_blue"))
    p.append(zb.box((0.16, W - 0.06, 0.12), (bx0 + 0.02, 0, 0.5), color="blanket_light", bevel=0.04,
                    bevel_edges="top"))
    # white stars on the blanket
    stars = [(-0.1, -0.25), (0.25, 0.15), (0.55, -0.2), (0.75, 0.25), (0.05, 0.3), (0.45, 0.0)]
    for i, (sx, sy) in enumerate(stars):
        p.append(flat_star_up(sx, sy, 0.536, 0.06, "pillow", phase=0.3 * i))
    for q in p:  # headboard to +X
        q.rotate_z(180)
    A.add(p)
    return A


# --------------------------------------------------------------------------- night table + lamp

def night_table():
    A = nl.Asset("night_table")
    W, D, H = 0.5, 0.4, 0.55
    p = [zb.box((W, D, 0.05), (0, 0, H - 0.025), color="wood_light", bevel=0.015)]
    p.append(zb.box((W - 0.06, D - 0.06, H - 0.14), (0, 0, 0.09 + (H - 0.14) / 2), color="wood"))
    for sx in (-1, 1):
        for sy in (-1, 1):
            p.append(zb.box((0.06, 0.06, 0.12), (sx * (W / 2 - 0.05), sy * (D / 2 - 0.05), 0.06), color="wood",
                            bevel=0.012, bevel_edges="vertical"))
    # drawer front + knob
    p.append(zb.box((W - 0.12, 0.02, 0.16), (0, FRONT * (D / 2 - 0.02), 0.38), color="wood_light", bevel=0.01))
    p.append(zb.knob(0.022, (0, FRONT * (D / 2 + 0.005), 0.38), color="wood_dark", u=6, v=3))
    # book and cup on top (the lamp stands at the back right, see bedside_lamp)
    p.append(zb.box((0.16, 0.12, 0.04), (-0.1, FRONT * 0.06, H + 0.02), color="book_red", bevel=0.008))
    p.append(zb.box((0.13, 0.1, 0.012), (-0.1, FRONT * 0.06, H + 0.046), color="rug_cream"))
    p.append(pp.cyl(0.035, H, H + 0.08, sides=8, center=(0.14, FRONT * 0.08), color="cup_blue"))
    A.add(p)
    A.empty("socket_lamp", (0.08, BACK * 0.08, H))
    return A


def bedside_lamp():
    """Round lamp for the night table (stands at the table's socket_lamp). Origin = foot."""
    A = nl.Asset("bedside_lamp")
    p = [zb.knob(0.08, (0, 0, 0.08), color="lamp_glass", u=8, v=5, squash=1.0)]
    p.append(pp.cyl(0.03, 0.0, 0.03, sides=8, r1=0.06, color="wood_dark"))
    p.append(pp.cyl(0.015, 0.15, 0.2, sides=6, color="brass"))
    shade = pp.cyl(0.13, 0.19, 0.36, sides=10, r1=0.08, color="lamp_glass")
    A.add(p)
    A.add(shade, mat="lamp_glow")
    A.empty("light", (0, 0, 0.27), light=(2.0, "#FFC46E"))
    return A


# --------------------------------------------------------------------------- window

def window_moon():
    """Wooden window with cross bars and short blue curtains; wall plane at y = 0 (glTF z = 0),
    the room side (front) is -Y. Panes = palette `window_sky` (day sky); child node
    `night_sky` = dark-blue sky with the moon (moon_glow) and two stars, 5 mm in front of the
    panes — shown only at night (hide it by day)."""
    A = nl.Asset("window_moon")
    W, H = 0.8, 0.9
    fr = 0.08
    p = []
    y = FRONT * 0.04
    # frame
    for sx in (-1, 1):
        p.append(zb.box((fr, 0.08, H), (sx * (W / 2 - fr / 2), y, H / 2), color="wood"))
    p.append(zb.box((W + 0.1, 0.12, fr + 0.02), (0, FRONT * 0.06, fr / 2 + 0.01), color="wood_light"))  # sill
    p.append(zb.box((W, 0.08, fr), (0, y, H - fr / 2), color="wood"))
    # panes (day sky) and cross bars
    p.append(zb.box((W - 2 * fr, 0.02, H - 2 * fr - 0.02), (0, FRONT * 0.01, H / 2 + 0.01), color="window_sky"))
    p.append(zb.box((0.05, 0.05, H - 2 * fr), (0, FRONT * 0.055, H / 2 + 0.01), color="wood_light"))
    p.append(zb.box((W - 2 * fr, 0.05, 0.05), (0, FRONT * 0.055, H / 2 + 0.01), color="wood_light"))
    # curtain rod + curtains (short, gathered at both sides)
    p.append(pp.beam((-(W / 2 + 0.2), FRONT * 0.13, H + 0.1), ((W / 2 + 0.2), FRONT * 0.13, H + 0.1), 0.035,
                     sides=6, color="wood_dark"))
    for sx in (-1, 1):
        for k in range(3):
            cx = sx * (W / 2 - 0.02 + 0.07 * k)
            p.append(zb.box((0.08, 0.06, H + 0.02), (cx, FRONT * (0.12 + 0.015 * (k % 2)), H / 2 + 0.08),
                            color="curtain_blue", bevel=0.025, bevel_edges="vertical"))
        p.append(zb.knob(0.04, (sx * (W / 2 + 0.2), FRONT * 0.13, H + 0.1), color="wood_dark", u=6, v=3))
    A.add(p)
    # night view (child node, hidden by day)
    A.node("night_sky", pivot=(0, 0, 0))
    ny = FRONT * 0.022
    n = [zb.box((W - 2 * fr, 0.004, H - 2 * fr - 0.02), (0, ny, H / 2 + 0.01), color="night_sky")]
    A.add(n, node="night_sky")
    moon = nl.ring_xz(-0.14, 0.62, 0.09, 10, FRONT * 0.03, FRONT * 0.025, "moon_cream")
    st = [flat_star_xz(0.15, 0.66, FRONT * 0.027, 0.035, FRONT, "moon_cream"),
          flat_star_xz(0.2, 0.3, FRONT * 0.027, 0.025, FRONT, "moon_cream", phase=0.4)]
    A.add([moon] + st, mat="moon_glow", node="night_sky")
    return A


# --------------------------------------------------------------------------- rug, chest

def rug_round():
    A = nl.Asset("rug_round")
    rings = [(0.7, "rug_blue", 0.012), (0.56, "rug_cream", 0.015), (0.4, "rug_blue", 0.018),
             (0.24, "rug_cream", 0.021), (0.1, "rug_blue", 0.024)]
    p = []
    for r, c, h in rings:
        p.append(zb.slab(zb.ellipse(0, 0, r, r, sides=18), 0.0, h, color=c))
    A.add(p)
    return A


def toy_chest():
    """0.8 x 0.45 m chest, rounded lid as child node `lid` (pivot = hinge axis at the back top
    edge, modelled OPEN 70 deg; rotate +70 deg about glTF +X to close it), plush elephant."""
    A = nl.Asset("toy_chest")
    W, D, H = 0.8, 0.45, 0.42
    p = []
    wall = 0.04
    p.append(zb.box((W, D, 0.05), (0, 0, 0.025), color="wood_dark"))
    for sy in (-1, 1):
        p.append(zb.box((W, wall, H), (0, sy * (D / 2 - wall / 2), H / 2), color="wood"))
    for sx in (-1, 1):
        p.append(zb.box((wall, D - 2 * wall, H), (sx * (W / 2 - wall / 2), 0, H / 2), color="wood"))
    # corner strips + front planks
    for sx in (-1, 1):
        for sy in (-1, 1):
            p.append(zb.box((0.06, 0.06, H + 0.01), (sx * (W / 2 - 0.02), sy * (D / 2 - 0.02), H / 2),
                            color="wood_light", bevel=0.012, bevel_edges="vertical"))
    for z in (0.14, 0.28):
        p.append(zb.box((W - 0.1, 0.012, 0.02), (0, FRONT * (D / 2 + 0.004), z), color="wood_dark"))
    # plush elephant peeking out (head, ears, trunk)
    ex, ey = 0.05, 0.0
    p.append(zb.knob(0.13, (ex, ey, H - 0.02), color="plush_grey", u=8, v=5))
    for sx in (-1, 1):
        ear = zb.knob(0.08, (0, 0, 0), color="plush_grey", u=6, v=4, squash=1.0)
        ear.transform(Matrix.Diagonal((1.0, 0.35, 1.0, 1.0)))
        ear.move(ex + sx * 0.14, ey + 0.02, H + 0.02)
        p.append(ear)
    p.append(pp.beam((ex, ey + FRONT * 0.1, H - 0.03), (ex + 0.02, ey + FRONT * 0.17, H - 0.12), 0.05, sides=6,
                     taper=0.7, color="plush_grey"))
    for sx in (-1, 1):
        p.append(zb.knob(0.018, (ex + sx * 0.05, ey + FRONT * 0.115, H + 0.03), color="eye_dark", u=6, v=3))
    A.add(p)
    # lid: half-cylinder along X, hinged at the back top edge, opened 70 deg
    piv = Vector((0, BACK * (D / 2), H))
    A.node("lid", pivot=piv)
    n = 6
    prof = [(math.cos(math.pi * i / n) * D / 2, math.sin(math.pi * i / n) * 0.14) for i in range(n + 1)]
    poly = [(y, z) for y, z in prof]
    lid = pp.prism_xz([(y, z) for y, z in poly], -W / 2, W / 2, "wood")
    lid.rotate_z(90)          # profile in Y/Z, length along X
    lid.move(0, 0, H)
    lids = [lid]
    for sx in (-1, 1):
        s = pp.prism_xz([(y, z + 0.01) for y, z in poly], -0.03, 0.03, "wood_light")
        s.rotate_z(90)
        s.move(sx * (W / 2 - 0.03), 0, H)
        lids.append(s)
    m = Matrix.Translation(piv) @ Matrix.Rotation(math.radians(-70), 4, "X") @ Matrix.Translation(-piv)
    for q in lids:
        q.transform(m)
    A.add(lids, node="lid", keep_down=True)
    return A


# --------------------------------------------------------------------------- desk, note, key box, key

def desk():
    """1.2 x 0.6 x 0.72 m writing desk; the child sits at the front (+Z). The note lies on the
    desk top (y = 0.74) — socket_note."""
    A = nl.Asset("desk")
    W, D, H = 1.2, 0.6, 0.72
    p = [zb.box((W, D, 0.05), (0, 0, H - 0.025), color="wood_light", bevel=0.015)]
    for sx in (-1, 1):
        for sy in (-1, 1):
            p.append(zb.box((0.07, 0.07, H - 0.05), (sx * (W / 2 - 0.06), sy * (D / 2 - 0.06), (H - 0.05) / 2),
                            color="wood", bevel=0.015, bevel_edges="vertical"))
    # drawer block on the right and back panel
    p.append(zb.box((0.4, D - 0.1, 0.16), (W / 2 - 0.25, 0, H - 0.13), color="wood"))
    p.append(zb.box((0.34, 0.02, 0.1), (W / 2 - 0.25, FRONT * (D / 2 - 0.04), H - 0.13), color="wood_light",
                    bevel=0.008))
    p.append(zb.knob(0.02, (W / 2 - 0.25, FRONT * (D / 2 - 0.02), H - 0.13), color="wood_dark", u=6, v=3))
    p.append(zb.box((W - 0.12, 0.03, 0.3), (0, BACK * (D / 2 - 0.07), 0.4), color="wood"))
    # pencil cup and a small stack of paper at the back
    p.append(pp.cyl(0.04, H, H + 0.1, sides=8, center=(-0.42, BACK * 0.15), color="cup_blue"))
    for k, c in enumerate(("cart_green", "stripe_red")):
        p.append(pp.beam((-0.42 + 0.01 * k, BACK * 0.15, H + 0.08), (-0.44 + 0.03 * k, BACK * 0.14, H + 0.18),
                         0.012, color=c))
    A.add(p)
    A.empty("socket_note", (-0.1, FRONT * 0.05, H))
    return A


def note_paper():
    """A4-ish sheet (0.24 x 0.32 m) lying flat, 3 mm thick. Top face = `note_face` slot with
    UVs 0..1 (u -> +X, v -> north / away from the reader at yaw 0); the game draws the
    'Math Fighter' title and the task on it."""
    A = nl.Asset("note_paper")
    W, D, T = 0.24, 0.32, 0.003
    body = zb.box((W, D, T), (0, 0, T / 2), color="paper")
    body.delete_faces(lambda f: f.normal.z > 0.9)
    A.add(body)
    face = nl.face_quad([(-W / 2, -D / 2, T), (W / 2, -D / 2, T), (W / 2, D / 2, T), (-W / 2, D / 2, T)])
    A.add(face, mat="note_face")
    return A


def key_box():
    """Wall key box 0.34 x 0.44 x 0.12 m with a 3-wheel combination lock on the door. Wall plane
    at y = 0 (glTF z = 0), box sticks out to the front. Door = child node `door`, pivot on the
    hinge axis (left edge, front face); open = rotate -100 deg about glTF +Y (swings out to the
    front-left).
    socket_key = where the cart_key hangs (hook inside)."""
    A = nl.Asset("key_box")
    W, H, D = 0.34, 0.44, 0.12
    p = []
    t = 0.02
    p.append(zb.box((W, t, H), (0, FRONT * t / 2, H / 2), color="keybox_dark"))
    for sx in (-1, 1):
        p.append(zb.box((t, D - t, H), (sx * (W / 2 - t / 2), FRONT * (t + (D - t) / 2), H / 2), color="keybox_red"))
    for sz in (0, 1):
        p.append(zb.box((W - 2 * t, D - t, t), (0, FRONT * (t + (D - t) / 2), t / 2 + sz * (H - t)),
                        color="keybox_red"))
    # small roof lip on top
    p.append(zb.box((W + 0.04, D + 0.03, 0.03), (0, FRONT * (D / 2 + 0.015), H + 0.015), color="keybox_dark",
                    bevel=0.01))
    # hook inside
    p.append(pp.beam((0, FRONT * t, H - 0.12), (0, FRONT * 0.07, H - 0.12), 0.015, color="brass"))
    A.add(p)
    # door
    piv = Vector((-W / 2, FRONT * D, 0))
    A.node("door", pivot=piv)
    d = [zb.box((W - 0.006, 0.02, H - 0.006), (0, FRONT * (D + 0.01), H / 2), color="keybox_red", bevel=0.006)]
    # lock plate with three number wheels
    d.append(zb.box((0.2, 0.012, 0.09), (0.02, FRONT * (D + 0.026), H * 0.45), color="iron"))
    for k in range(3):
        wx = 0.02 + (k - 1) * 0.06
        wheel = pp.cyl(0.03, 0, 0.045, sides=8, color="metal")
        wheel.rotate(90, "Y").move(wx - 0.0225, FRONT * (D + 0.045), H * 0.45)
        d.append(wheel)
        d.append(zb.box((0.034, 0.006, 0.03), (wx, FRONT * (D + 0.076), H * 0.45), color="stripe_white"))
    # handle and hinges
    d.append(zb.box((0.03, 0.03, 0.08), (W / 2 - 0.04, FRONT * (D + 0.03), H * 0.7), color="brass", bevel=0.008))
    for z in (0.08, H - 0.08):
        d.append(zb.box((0.03, 0.02, 0.05), (-W / 2 + 0.01, FRONT * (D + 0.01), z), color="keybox_dark"))
    # key icon painted on the door
    d.append(zb.box((0.1, 0.004, 0.03), (0.0, FRONT * (D + 0.022), H * 0.8), color="brass"))
    d.append(nl.ring_xz(-0.07, H * 0.8, 0.035, 8, FRONT * (D + 0.024), FRONT * (D + 0.02), "brass", r_in=0.018))
    A.add(d, node="door")
    A.empty("socket_key", (0, FRONT * 0.07, H - 0.12))
    return A


def cart_key():
    """Golf-cart key with a green round tag, hanging in the X/Z plane. Origin = bottom;
    socket_ring = the ring's top (hang it on key_box.socket_key)."""
    A = nl.Asset("cart_key")
    p = []
    tag = nl.ring_xz(0, 0.05, 0.05, 10, -0.006, 0.006, "cart_green")
    p.append(tag)
    p.append(nl.ring_xz(0, 0.125, 0.025, 8, -0.004, 0.004, "brass", r_in=0.014))
    # key: bow + blade below the ring... blade points down behind the tag
    p.append(nl.ring_xz(0.05, 0.13, 0.03, 8, -0.005, 0.005, "brass", r_in=0.012))
    p.append(zb.box((0.016, 0.008, 0.09), (0.05, 0, 0.055), color="brass"))
    p.append(zb.box((0.02, 0.008, 0.015), (0.062, 0, 0.025), color="brass"))
    p.append(zb.box((0.02, 0.008, 0.012), (0.062, 0, 0.05), color="brass"))
    A.add(p)
    A.empty("socket_ring", (0, 0, 0.15))
    return A


# --------------------------------------------------------------------------- preview extras

def extra(roots):
    """Bedroom corner sample: bed along the back, night table + lamp, rug, chest, desk + note,
    window on a wall piece."""
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    B = right * 13.5 + fwd * 1.0
    V = Vector
    out = [nl.instance_tree(roots["bed"], "s_bed", B + V((0, 0, 0)), rot_deg=180),
           nl.instance_tree(roots["night_table"], "s_nt", B + V((-1.35, 0.1, 0)), rot_deg=0),
           nl.instance_tree(roots["bedside_lamp"], "s_lamp", B + V((-1.35 + 0.08, 0.18, 0.55))),
           nl.instance_tree(roots["rug_round"], "s_rug", B + V((0.4, -1.3, 0))),
           nl.instance_tree(roots["toy_chest"], "s_chest", B + V((2.0, 0.2, 0))),
           nl.instance_tree(roots["desk"], "s_desk", B + V((1.0, -3.0, 0)), rot_deg=180),
           nl.instance_tree(roots["note_paper"], "s_note", B + V((1.1, -3.05, 0.72)), rot_deg=180),
           nl.instance_tree(roots["key_box"], "s_kb_open", B + V((3.5, -1.5, 1.1)),
                            node_rot={"door": -100})]
    return out


def main():
    builders = {
        "bed": bed,
        "night_table": night_table,
        "bedside_lamp": bedside_lamp,
        "window_moon": window_moon,
        "rug_round": rug_round,
        "toy_chest": toy_chest,
        "desk": desk,
        "note_paper": note_paper,
        "key_box": key_box,
        "cart_key": cart_key,
    }
    nl.run(KIT, builders, cols=5, spacing=(2.4, 2.6), extra=extra)


if __name__ == "__main__":
    main()
