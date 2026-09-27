"""Kit garden — vegetable garden of level 1 (concept: art/props/kit_garden/sheet_plants_v1.jpg,
sheet_furniture_v2.jpg, brief.md; GAME-GARDEN, level-1.toml [[garden]] / [[garden_bed]]).

Run:  blender -b --factory-startup --python tools/blender/props/kit_garden.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/<asset>.glb, assets/blender/props/kit_garden.blend,
         art/props/kit_garden/model_preview.png (day on top, night-tinted below)

Front = south = glTF +Z at yaw 0. (`garden_gate` is in kit_gates.)
- garden_bed        raised bed frame 0.8 x 2.9 m (long axis = glTF Z / level north at yaw 0,
                    fits a 1 x 3 cell [[garden_bed]] rect), 0.25 m high, soil top at y 0.22.
- carrot_plant_{sprout,young,ripe}, potato_plant_{sprout,young,ripe}
                    the plant of one [[plant_spot]] with its little soil mound; origin = plant
                    base; place it on the bed soil (y = 0.22). "empty" = no model.
- carrot, potato    harvested items (potato = 3 potatoes); origin = bottom.
- basket            woven basket with handle, carrots + potatoes inside (carried).
- garden_fence / garden_fence_1m  low white picket fence 0.8 m high, 2 m / 1 m between the
                    post centres (posts at the piece ends, like fence_wood).
- wheelbarrow       1.4 m, wheel to the front (+Z), yellow tub, handles to the back.
- watering_can      light-blue can, spout to the front (+Z).
- garden_sign       blank board on a stick, readable face = `sign_face` (UV 0..1), front.
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import night_lib as nl  # noqa: E402
import props_parts as pp  # noqa: E402
from night_lib import zb, FRONT, BACK  # noqa: E402
from mathutils import Vector  # noqa: E402

KIT = "kit_garden"


# --------------------------------------------------------------------------- bed

def garden_bed():
    A = nl.Asset("garden_bed")
    W, L, H = 0.8, 2.9, 0.25
    t = 0.06
    p = []
    for sx in (-1, 1):
        p.append(zb.box((t, L, H), (sx * (W / 2 - t / 2), 0, H / 2), color="wood", top_color="wood_light"))
    for sy in (-1, 1):
        p.append(zb.box((W - 2 * t, t, H), (0, sy * (L / 2 - t / 2), H / 2), color="wood", top_color="wood_light"))
    for sx in (-1, 1):  # plank seam
        p.append(zb.box((0.012, L - 0.04, 0.02), (sx * (W / 2 + 0.004), 0, H / 2), color="wood_dark"))
    p.append(zb.box((W - 2 * t, L - 2 * t, 0.2), (0, 0, 0.12), color="bed_soil"))
    rnd = random.Random(4)
    for k in range(5):  # soil clods
        p.append(zb.knob(0.05, (rnd.uniform(-0.25, 0.25), rnd.uniform(-1.3, 1.3), 0.22), color="soil", u=5, v=3,
                         squash=0.5))
    A.add(p)
    return A


# --------------------------------------------------------------------------- plants

def mound(p, r=0.16):
    p.append(zb.slab(zb.ellipse(0, 0, r, r * 0.9, sides=7), 0.0, 0.05, color="bed_soil", chamfer=0.03))


def carrot_leaves(p, n, h, spread, seed):
    rnd = random.Random(seed)
    for k in range(n):
        a = 2 * math.pi * k / n + rnd.uniform(-0.3, 0.3)
        tip = Vector((spread * math.cos(a), spread * math.sin(a), h * rnd.uniform(0.85, 1.1)))
        p.append(pp.beam((0, 0, 0.03), tuple(tip), 0.015, sides=3, color="carrot_leaf", caps=False))
        # feathery frond: a flat leaf along the upper stem
        mid = tip * 0.45 + Vector((0, 0, 0.03))
        p.append(pp.leaf(tuple(mid), tuple((tip - mid).normalized()), (tip - mid).length * 1.1, h * 0.28,
                         "carrot_leaf"))
        p.append(pp.leaf(tuple(mid), tuple((tip - mid).normalized() + Vector((0, 0, -0.3))), (tip - mid).length * 0.9,
                         h * 0.22, "leaf"))


def carrot_plant(stage):
    A = nl.Asset("carrot_plant_" + stage)
    p = []
    mound(p)
    if stage == "sprout":
        carrot_leaves(p, 3, 0.12, 0.03, 1)
    elif stage == "young":
        carrot_leaves(p, 5, 0.28, 0.08, 2)
    else:
        carrot_leaves(p, 7, 0.42, 0.12, 3)
        p.append(pp.cyl(0.07, 0.0, 0.09, sides=8, r1=0.055, color="carrot_orange"))
        p.append(zb.knob(0.055, (0, 0, 0.09), color="carrot_orange", u=8, v=3, squash=0.5))
    A.add(p)
    return A


def potato_leaves(p, n, h, spread, seed, flowers=0):
    rnd = random.Random(seed)
    for k in range(n):
        a = 2 * math.pi * k / n + rnd.uniform(-0.3, 0.3)
        r = spread * rnd.uniform(0.3, 1.0)
        base = Vector((0.3 * r * math.cos(a), 0.3 * r * math.sin(a), 0.04))
        top = Vector((r * math.cos(a), r * math.sin(a), h * rnd.uniform(0.6, 1.0)))
        p.append(pp.beam(tuple(base), tuple(top), 0.02, sides=3, color="potato_leaf", caps=False))
        for da in (-0.8, 0.8):
            d = (math.cos(a + da), math.sin(a + da), 0.2)
            p.append(pp.leaf(tuple(top), d, 0.14 + 0.1 * h, 0.09 + 0.05 * h, "potato_leaf", droop=0.04))
    for k in range(flowers):
        a = 2 * math.pi * k / flowers + 0.4
        c = (spread * 0.6 * math.cos(a), spread * 0.6 * math.sin(a), h + 0.04)
        p.append(pp.star(c, 0.05, 0.022, 5, "flower_white", phase=a))


def potato_plant(stage):
    A = nl.Asset("potato_plant_" + stage)
    p = []
    mound(p, 0.2)
    if stage == "sprout":
        potato_leaves(p, 2, 0.12, 0.05, 1)
    elif stage == "young":
        potato_leaves(p, 6, 0.32, 0.22, 2)
    else:
        potato_leaves(p, 8, 0.42, 0.3, 3, flowers=5)
    A.add(p)
    return A


def carrot_parts(x=0.0, y=0.0, z=0.0, rot=0.0, s=1.0):
    p = [pp.beam((x, y, z + 0.035 * s), (x + 0.2 * s * math.cos(rot), y + 0.2 * s * math.sin(rot), z + 0.035 * s),
                 0.07 * s, sides=6, taper=0.2, color="carrot_orange")]
    for k in range(3):
        a = rot + math.pi + (k - 1) * 0.35
        p.append(pp.leaf((x, y, z + 0.05 * s), (math.cos(a), math.sin(a), 0.25), 0.12 * s, 0.04 * s, "carrot_leaf"))
    return p


def carrot():
    A = nl.Asset("carrot")
    A.add(carrot_parts())
    return A


def potato():
    A = nl.Asset("potato")
    p = []
    for k, (x, y) in enumerate(((-0.045, 0.0), (0.04, 0.03), (0.0, -0.05))):
        p.append(zb.knob(0.045, (x, y, 0.035), color="potato", u=6, v=4, squash=0.75))
    A.add(p)
    return A


def basket():
    A = nl.Asset("basket")
    p = [pp.cyl(0.15, 0.0, 0.2, sides=10, r1=0.2, color="basket", top=False)]
    p.append(pp.cyl(0.21, 0.18, 0.22, sides=10, color="basket_dark", top=False))
    p.append(pp.disc((0, 0, 0.16), 0.18, "basket_dark", sides=10))
    # handle (arch in the X/Z plane)
    pts = nl.arc_points(-0.18, 0.18, 0.2, 0.45, 6)
    for (x0, z0), (x1, z1) in zip(pts, pts[1:]):
        p.append(pp.beam((x0, 0, z0), (x1, 0, z1), 0.035, sides=4, color="basket_dark"))
    # woven bands
    p.append(pp.cyl(0.18, 0.08, 0.1, sides=10, r1=0.185, color="basket_dark", top=False))
    # carrots and potatoes inside
    p += carrot_parts(-0.08, 0.05, 0.14, rot=-0.4, s=0.9)
    p += carrot_parts(0.02, 0.1, 0.15, rot=-0.9, s=0.8)
    for x, y in ((0.07, -0.05), (-0.05, -0.08)):
        p.append(zb.knob(0.05, (x, y, 0.19), color="potato", u=6, v=4, squash=0.75))
    A.add(p, keep_down=True)
    return A


# --------------------------------------------------------------------------- fence, tools, sign

def picket_fence(name, span):
    A = nl.Asset(name)
    p = []
    H = 0.8
    for sx in (-1, 1):
        p.append(zb.box((0.08, 0.08, H + 0.05), (sx * span / 2, 0, (H + 0.05) / 2), color="trim_white",
                        bevel=0.012, bevel_edges="vertical"))
    for z in (0.22, 0.58):
        p.append(zb.box((span - 0.08, 0.03, 0.06), (0, BACK * 0.04, z), color="trim_white"))
    n = int(round(span / 0.2))
    for k in range(n - 1):
        x = -span / 2 + span * (k + 1) / n
        p.append(pp.prism_xz([(x - 0.045, 0.05), (x + 0.045, 0.05), (x + 0.045, H - 0.07), (x, H),
                              (x - 0.045, H - 0.07)], FRONT * 0.015, BACK * 0.015, "trim_white"))
    A.add(p)
    return A


def garden_fence():
    return picket_fence("garden_fence", 2.0)


def garden_fence_1m():
    return picket_fence("garden_fence_1m", 1.0)


def wheelbarrow():
    A = nl.Asset("wheelbarrow")
    p = []
    # tub: tapered box (prism in X/Z extruded along Y)
    tub = pp.prism_xz([(-0.3, 0.35), (0.25, 0.35), (0.42, 0.72), (-0.42, 0.72)], -0.28, 0.28, "barrow_yellow")
    p.append(tub)
    p.append(zb.box((0.66, 0.46, 0.02), (-0.02, 0, 0.7), color="soil"))
    # handles / frame to the back (-X) and the wheel at the front (+X)
    for sy in (-1, 1):
        p.append(pp.beam((0.5, sy * 0.2, 0.25), (-0.85, sy * 0.28, 0.55), 0.05, sides=4, color="wood"))
        p.append(pp.beam((-0.25, sy * 0.2, 0.33), (-0.3, sy * 0.22, 0.0), 0.05, sides=4, color="wood"))
    wheel = pp.cyl(0.22, -0.04, 0.04, sides=10, color="tyre")
    wheel.rotate(90, "X").move(0.5, 0, 0.22)
    p.append(wheel)
    hub = pp.cyl(0.07, -0.05, 0.05, sides=6, color="metal")
    hub.rotate(90, "X").move(0.5, 0, 0.22)
    p.append(hub)
    for q in p:  # wheel to the front (-Y Blender = +Z glTF)
        q.rotate_z(-90)
    A.add(p)
    return A


def watering_can():
    A = nl.Asset("watering_can")
    p = [pp.cyl(0.13, 0.0, 0.26, sides=10, r1=0.12, color="can_blue")]
    p.append(pp.cyl(0.125, 0.26, 0.29, sides=10, r1=0.08, color="can_blue"))
    p.append(pp.beam((0.1, 0, 0.08), (0.34, 0, 0.3), 0.035, sides=5, color="can_blue"))
    rose = pp.cyl(0.03, 0.0, 0.04, sides=6, r1=0.055, color="can_blue")
    rose.rotate(55, "Y").move(0.34, 0, 0.3)
    p.append(rose)
    pts = nl.arc_points(-0.12, 0.06, 0.26, 0.42, 5)
    for (x0, z0), (x1, z1) in zip(pts, pts[1:]):
        p.append(pp.beam((x0, 0, z0), (x1, 0, z1), 0.03, sides=4, color="can_blue"))
    for q in p:  # spout to the front
        q.rotate_z(-90)
    A.add(p)
    return A


def garden_sign():
    A = nl.Asset("garden_sign")
    p = [zb.box((0.05, 0.04, 0.6), (0, 0, 0.3), color="trim_white")]
    W, H = 0.5, 0.3
    zc = 0.66
    board = zb.box((W, 0.03, H), (0, 0, zc), color="trim_white", bevel=0.008)
    board.rotate(-10 * FRONT, "X").move(0, 0, 0)
    p.append(board)
    A.add(p)
    # face: same tilt as the board, 2 mm proud
    import mathutils
    m = mathutils.Matrix.Rotation(math.radians(-10 * FRONT), 4, "X")
    corners = [Vector((x, FRONT * 0.017, z)) for x, z in ((-W / 2 + 0.02, zc - H / 2 + 0.02),
                                                          (W / 2 - 0.02, zc - H / 2 + 0.02),
                                                          (W / 2 - 0.02, zc + H / 2 - 0.02),
                                                          (-W / 2 + 0.02, zc + H / 2 - 0.02))]
    corners = [m @ c for c in corners]
    A.add(nl.face_quad([tuple(c) for c in corners], "sign_face"), mat="sign_face")
    return A


def main():
    builders = {
        "garden_bed": garden_bed,
        "carrot_plant_sprout": lambda: carrot_plant("sprout"),
        "carrot_plant_young": lambda: carrot_plant("young"),
        "carrot_plant_ripe": lambda: carrot_plant("ripe"),
        "potato_plant_sprout": lambda: potato_plant("sprout"),
        "potato_plant_young": lambda: potato_plant("young"),
        "potato_plant_ripe": lambda: potato_plant("ripe"),
        "carrot": carrot,
        "potato": potato,
        "basket": basket,
        "garden_fence": garden_fence,
        "garden_fence_1m": garden_fence_1m,
        "wheelbarrow": wheelbarrow,
        "watering_can": watering_can,
        "garden_sign": garden_sign,
    }
    nl.run(KIT, builders, cols=5, spacing=(2.2, 2.2))


if __name__ == "__main__":
    main()
