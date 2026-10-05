"""Kit landmarks_play — ice cream kiosk (level 3), carousel (level 3), playground slide and
double swing (level 2). Concepts (approved by the user 2026-10-03):
art/props/kit_landmarks_l3/sheet_ice_cream_kiosk_v1.jpg, sheet_carousel_v2.jpg,
art/props/kit_landmarks_l2/sheet_playground_v4.jpg.

Run:  blender -b --factory-startup --python tools/blender/props/kit_landmarks_play.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/<asset>.glb, assets/blender/props/kit_landmarks_play.blend,
         art/props/kit_landmarks_play/model_preview.png (day on top, night-tinted below)

Origins = centre of the level rect of the element, front = south = glTF +Z, 1 unit = 1 m.
- ice_cream_kiosk    rect 4 x 3 (level-3 `ice_cream_kiosk`, a building WITHOUT an enterable interior):
                     cream plank booth at the back, flat roof with a big ice-cream-cone icon (no text),
                     striped pink awning over the service window with a counter, a shelf behind the
                     window, a freezer chest with a glass lid + cone stand in front, back door (decor).
- carousel           rect 4 x 4 (level-3 `carousel_sw`), 4.0 m: low static base ring + steps and the
                     child node `rotor` (pivot = the centre on the ground; the game spins it about
                     +Y, NodeBehaviour "rotor"): floor, hub, 6 golden poles with 6 wooden horses,
                     striped roof with a gold finial.
- playground_slide   rect 2 x 3 (level-2 `playground_se_slide`), 1.8 m: platform with ladder at the
                     back (north), red/yellow chute towards the front (south).
- playground_swings  rect 4 x 2 (level-2 `playground_se_swings`), 2.2 m: two A-frames, top beam,
                     two blue seats on chains (static).
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

KIT = "kit_landmarks_play"
PREVIEW_DIR = "kit_landmarks_play"


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


def strip_yz(path, thick, up, x0, x1, color):
    """A bent strip along the Y/Z path [(y, z)] (top surface), `thick` below it and `up` above."""
    top = [(y, z + up) for y, z in path]
    bot = [(y, z - thick) for y, z in path]
    return prism_yz(top + list(reversed(bot)), x0, x1, color)


def tilt_y(parts, deg, origin):
    m = Matrix.Translation(origin) @ Matrix.Rotation(math.radians(deg), 4, "Y")
    for p in parts:
        p.transform(m)
    return parts


# --------------------------------------------------------------------------- ice cream kiosk

def ice_cream_kiosk():
    A = nl.Asset("ice_cream_kiosk")
    p = []
    W = 3.6                       # booth width (x)
    y0, y1 = -0.4, 1.4            # booth depth (south face at y0)
    H = 2.25                      # wall height
    p.append(zb.box((W, y1 - y0, H), (0, (y0 + y1) / 2, H / 2), color="kiosk_wall"))
    # corner posts + base trim (wood)
    for sx in (-1, 1):
        p.append(zb.box((0.12, 0.12, H), (sx * (W / 2), y0, H / 2), color="wood"))
    p.append(zb.box((W + 0.12, 0.1, 0.12), (0, y0 - 0.02, 0.06), color="wood_dark"))
    # service window: dark opening, frame, counter plank, shelf with tubs behind it
    wz0, wz1 = 1.05, 1.85
    p.append(zb.box((2.5, 0.04, wz1 - wz0), (0, y0 - 0.02, (wz0 + wz1) / 2), color="lantern_metal"))
    p.append(zb.box((2.7, 0.08, 0.08), (0, y0 - 0.04, wz1 + 0.04), color="wood"))
    p.append(zb.box((2.9, 0.4, 0.07), (0, y0 - 0.2, wz0 - 0.03), color="wood_light", bevel=0.01))
    p.append(zb.box((2.2, 0.22, 0.04), (0, y0 + 0.14, 1.5), color="wood"))
    for k, c in enumerate(("scoop_pink", "scoop_mint", "scoop_cream")):
        p.append(zb.box((0.28, 0.2, 0.16), (-0.7 + 0.7 * k, y0 + 0.14, 1.6), color=c))
    # flat roof with a fascia, big cone icon on a pole
    p.append(zb.box((W + 0.4, y1 - y0 + 0.5, 0.14), (0, (y0 + y1) / 2 - 0.05, H + 0.07), color="wood_light"))
    p.append(zb.box((W + 0.4, 0.08, 0.2), (0, y0 - 0.28, H + 0.1), color="wood"))
    # awning: 8 stripes sloping down to the front, with a short valance
    n = 8
    sw = (W + 0.3) / n
    for k in range(n):
        xa = -(W + 0.3) / 2 + k * sw
        col = "kiosk_pink" if k % 2 == 0 else "kiosk_pink_light"
        p.append(prism_yz([(y0 - 0.05, H - 0.1), (y0 - 0.95, H - 0.5), (y0 - 0.95, H - 0.7),
                           (y0 - 0.89, H - 0.7), (y0 - 0.89, H - 0.56), (y0 - 0.05, H - 0.16)],
                          xa, xa + sw - 0.005, col))
    # cone icon: scoops on top, waffle cone pointing down-right, leaning
    cone = [
        pp.cyl(0.02, 0.0, 1.0, sides=8, r1=0.34, color="cone_wafer", phase=0.0),
        zb.knob(0.39, (0, 0, 1.1), color="scoop_mint", u=8, v=4, squash=0.85),
        zb.knob(0.36, (0, 0, 1.5), color="scoop_pink", u=8, v=4, squash=0.9),
    ]
    tilt_y(cone, 22, Vector((0.2, y0 + 0.9, H + 0.17)))
    p += cone
    p.append(pp.beam((0.2, y0 + 0.9, H + 0.14), (0.2, y0 + 0.9, H + 0.5), 0.07, sides=4, color="iron"))
    # back door (decor only: the kiosk has no interior)
    p.append(zb.box((0.8, 0.05, 1.7), (1.0, y1 + 0.02, 0.85), color="wood"))
    # freezer chest in front (left) with a glass lid and tubs, cone stand (right)
    fx, fy = -0.9, -1.0
    p.append(zb.box((1.5, 0.7, 0.8), (fx, fy, 0.4), color="pillow", bevel=0.03))
    p.append(zb.box((1.58, 0.78, 0.07), (fx, fy, 0.82), color="sheet_white"))
    for k, c in enumerate(("scoop_pink", "scoop_mint", "scoop_cream")):
        p.append(zb.box((0.34, 0.5, 0.06), (fx - 0.45 + 0.45 * k, fy, 0.84), color=c))
    A.add(p)
    A.node("glass")
    A.add([zb.box((1.42, 0.62, 0.04), (fx, fy, 0.95), color="glass_tint")], mat="glass", node="glass", keep_down=True)
    sx0 = 1.1
    t = [zb.box((0.9, 0.5, 0.05), (sx0, -1.0, 0.6), color="wood_light")]
    for sx in (-1, 1):
        for sy in (-1, 1):
            t.append(zb.box((0.06, 0.06, 0.58), (sx0 + sx * 0.4, -1.0 + sy * 0.2, 0.29), color="wood"))
    for k in range(4):
        t.append(pp.cyl(0.02, 0.65, 0.87, sides=5, r1=0.075, color="cone_wafer", phase=0.0)
                 .move(sx0 - 0.3 + 0.2 * k, -1.0, 0.0))
    A.add(t)
    return A


# --------------------------------------------------------------------------- carousel

def horse(color, mane):
    """A toy horse facing +X, centred on its body, hooves at z = -0.42."""
    h = [zb.box((0.62, 0.22, 0.3), (0, 0, 0), color=color)]
    h.append(pp.beam((0.22, 0, 0.1), (0.4, 0, 0.45), 0.16, sides=4, color=color))
    h.append(zb.box((0.26, 0.15, 0.15), (0.5, 0, 0.5), color=mane))   # muzzle / mane colour
    h.append(zb.box((0.1, 0.22, 0.34), (0.22, 0, -0.25), color=color))   # front legs (pair)
    h.append(zb.box((0.1, 0.22, 0.34), (-0.22, 0, -0.25), color=color))  # back legs (pair)
    h.append(zb.box((0.2, 0.24, 0.05), (0.0, 0, 0.17), color="barn_red"))  # saddle
    h.append(pp.beam((-0.3, 0, 0.1), (-0.44, 0, -0.2), 0.06, sides=3, color=mane, caps=False))
    return h


def carousel():
    A = nl.Asset("carousel")
    R = 1.9
    base = [pp.cyl(R, 0.0, 0.12, sides=12, color="wood_dark")]
    # steps on the south side
    base.append(zb.box((0.9, 0.3, 0.08), (0, FRONT * (R + 0.05), 0.04), color="wood_light"))
    A.add(base)
    A.node("rotor", pivot=(0, 0, 0))
    r = [pp.cyl(R - 0.1, 0.12, 0.3, sides=12, color="floor_wood")]
    # hub: blue column with golden bands
    r.append(pp.cyl(0.32, 0.3, 2.5, sides=6, color="carousel_blue"))
    r.append(pp.cyl(0.38, 1.0, 1.15, sides=6, color="brass"))
    # roof: wedge-striped cone + striped valance + gold finial
    def stripes(part):
        cols = ("stripe_red", "stripe_white", "carousel_blue")

        def sector(f):
            c = f.calc_center_median()
            return int(round((math.atan2(c.y, c.x) % (2 * math.pi)) / (math.pi / 6))) % 12
        for k, col in enumerate(cols):
            pp.recolor(part, lambda f, k=k: sector(f) % 3 == k, col)
        return part
    r.append(stripes(pp.cyl(2.0, 2.6, 3.85, sides=12, apex=True, color="stripe_red", phase=0.0)))
    r.append(stripes(pp.cyl(2.0, 2.35, 2.6, sides=12, top=False, color="stripe_red", phase=0.0)))
    r.append(zb.knob(0.14, (0, 0, 3.92), color="brass", u=5, v=3))
    # poles + horses (alternate heights, all turning anticlockwise)
    horses = (("horse_brown", "wood_dark"), ("horse_cream", "horse_brown"), ("horse_grey", "iron"),
              ("pillow", "horse_brown"), ("horse_brown", "wood_dark"), ("horse_cream", "horse_grey"))
    n = len(horses)
    for k, (col, mane) in enumerate(horses):
        a = 2 * math.pi * k / n + math.radians(15)
        cx, cy = 1.35 * math.cos(a), 1.35 * math.sin(a)
        r.append(pp.beam((cx, cy, 0.3), (cx, cy, 2.45), 0.07, sides=3, color="brass"))
        z = 0.95 + (0.25 if k % 2 else 0.0)
        hp = horse(col, mane)
        for part in hp:
            part.rotate_z(math.degrees(a) + 90.0)  # facing along the circle
            part.move(cx, cy, z)
        r += hp
    A.add(r, node="rotor")
    return A


# --------------------------------------------------------------------------- slide

def playground_slide():
    A = nl.Asset("playground_slide")
    p = []
    PZ = 1.15
    # platform on four posts; the back (ladder) posts carry the top rail up to 1.8 m
    p.append(zb.box((1.0, 0.8, 0.07), (0, 0.6, PZ - 0.035), color="wood_light"))
    for sx in (-1, 1):
        p.append(zb.box((0.09, 0.09, PZ + 0.65), (sx * 0.5, 1.0, (PZ + 0.65) / 2), color="wood"))
        p.append(zb.box((0.09, 0.09, PZ), (sx * 0.5, 0.2, PZ / 2), color="wood"))
        # side rail of the platform
        p.append(zb.box((0.05, 0.8, 0.05), (sx * 0.5, 0.6, PZ + 0.45), color="wood_dark"))
        # ladder rails
        p.append(pp.beam((sx * 0.32, 1.5, 0.0), (sx * 0.32, 1.02, PZ), 0.07, sides=4, color="wood"))
    p.append(zb.box((1.0, 0.06, 0.06), (0, 1.0, PZ + 0.62), color="slide_red"))       # top bar
    for k in range(4):
        z = 0.25 + 0.25 * k
        y = 1.5 - 0.48 * z / PZ
        p.append(zb.box((0.6, 0.06, 0.05), (0, y, z), color="wood_dark"))
    # chute (south): yellow bed, red side rims, curved
    path = [(0.2, PZ), (-0.15, PZ - 0.22), (-0.6, PZ - 0.62), (-1.0, 0.42), (-1.3, 0.25), (-1.5, 0.2)]
    p.append(strip_yz(path, 0.07, 0.0, -0.34, 0.34, "barrow_yellow"))
    for sx in (-1, 1):
        p.append(strip_yz(path, 0.07, 0.15, 0.34 if sx > 0 else -0.44, 0.44 if sx > 0 else -0.34, "slide_red"))
    # support leg under the chute
    for sx in (-1, 1):
        p.append(pp.beam((sx * 0.42, -0.5, 0.0), (sx * 0.42, -0.4, 0.62), 0.07, sides=4, color="wood"))
    A.add(p)
    return A


# --------------------------------------------------------------------------- swings

def playground_swings():
    A = nl.Asset("playground_swings")
    p = []
    TOP = 2.12
    for sx in (-1, 1):
        x = sx * 1.6
        for sy in (-1, 1):
            p.append(pp.beam((x, sy * 0.75, 0.0), (x, 0.0, TOP - 0.02), 0.15, sides=4, color="wood"))
        p.append(zb.box((0.12, 0.9, 0.08), (x, 0.0, 0.62), color="wood_dark"))   # cross brace between the legs
    p.append(zb.box((3.95, 0.17, 0.17), (0, 0, TOP + 0.03), color="wood_light"))
    A.add(p)
    pivot_z = TOP - 0.04
    for name, x0 in (("swing_l", -0.6), ("swing_r", 0.6)):
        A.node(name, pivot=(x0, 0, pivot_z))
        s = [zb.box((0.56, 0.26, 0.06), (x0, 0, 0.45), color="swing_blue", bevel=0.015),
             zb.box((0.56, 0.05, 0.04), (x0, -0.12, 0.49), color="swing_blue"),
             zb.box((0.56, 0.05, 0.04), (x0, 0.12, 0.49), color="swing_blue")]
        for sx in (-1, 1):
            s.append(pp.beam((x0 + sx * 0.22, 0, 0.48), (x0 + sx * 0.22, 0, pivot_z), 0.045, sides=4, color="iron"))
            for z in (0.8, 1.15, 1.5):    # chain link blocks
                s.append(zb.box((0.07, 0.03, 0.09), (x0 + sx * 0.22, 0, z), color="metal"))
        A.add(s, node=name, keep_down=True)
    return A


# --------------------------------------------------------------------------- preview

def main():
    builders = {
        "ice_cream_kiosk": ice_cream_kiosk,
        "carousel": carousel,
        "playground_slide": playground_slide,
        "playground_swings": playground_swings,
    }
    roots = nl.run(KIT, builders, cols=2, spacing=(6.0, 6.0), budgets={"*": 600}, preview_dir=PREVIEW_DIR)
    args = zb.script_args()
    if "--no-preview" not in args and not [a for a in args if not a.startswith("--")]:
        # low front view (yaw 0, 20 deg): what the 55 deg game view hides
        grid = list(roots.values())
        zb.layout_grid(grid, cols=4, spacing=(6.0, 6.0), yaw_deg=0.0)
        nl.render_preview(grid, zb.repo_path("art", "props", PREVIEW_DIR, "model_preview_front.png"),
                          res=(1600, 700), yaw_deg=0.0, pitch_deg=18.0, night=False)


if __name__ == "__main__":
    main()
