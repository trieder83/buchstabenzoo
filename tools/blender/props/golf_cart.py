"""Zoo golf cart + parking sign (GAME-CART). Concept (approved by the user 2026-10-06):
art/props/golf_cart/sheet_v3_red.jpg, seat_closeup_v2_red.jpg, brief.md.

Run:  blender -b --factory-startup --python tools/blender/props/golf_cart.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/golf_cart.glb, parking_sign.glb,
         assets/blender/props/golf_cart.blend, art/props/golf_cart/model_preview.png
(`key_box_open` is built by kit_bedroom.py, the open variant of `key_box`.)

Origin = centre of the cart on the ground, front = glTF +Z (Blender -Y), 1 unit = 1 m.
glTF +X = the cart's LEFT side (heading +Z), so the driver sits at +X.
- golf_cart      2.6 x 1.4 x 2.0 m, <= 1000 tris. Red body with white lower band / bumpers / fenders,
                 white roof with red stripes + scalloped front/back edge on 4 white posts, 2-seat bench
                 (cream + red cushion, seat top 0.5 m), wicker basket in the cargo bed, blank white
                 emblem discs. Child nodes (translation-only pivots):
                   wheel_fl / wheel_fr / wheel_rl / wheel_rr  pivot = hub centre; spin about glTF X
                   steering_wheel   pivot = hub centre; turn about the column (tilted ~20 deg)
                   headlight_l / headlight_r   `lamp_glow` slot (yellow by day, emissive at night)
                 Empties: socket_driver (seat position of the left seat, where the player sits),
                 light_l / light_r (night point lights ahead of the headlights).
- parking_sign   post with a blue P plate facing +Z, footprint 0.2 m, <= 150 tris.
"""

import math
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import night_lib as nl  # noqa: E402
import props_parts as pp  # noqa: E402
from night_lib import zb, FRONT  # noqa: E402
from mathutils import Matrix  # noqa: E402

KIT = "golf_cart"

# wheel layout: (node, x, y, radius)  (Blender y: front = -y)
WHEELS = [("wheel_fl", 0.6, -0.85, 0.27), ("wheel_fr", -0.6, -0.85, 0.27),
          ("wheel_rl", 0.6, 0.8, 0.30), ("wheel_rr", -0.6, 0.8, 0.30)]
WHEEL_W = 0.2


def _axis_x(part, sx):
    """Rotate a Z-up part so its +Z points to +X (sx = 1) or -X (sx = -1)."""
    return part.rotate(90 * sx, "Y")


def wheel(A, name, x, y, r):
    sx = 1 if x > 0 else -1
    cz = r
    A.node(name, pivot=(x, y, cz))
    tire = _axis_x(pp.cyl(r, -WHEEL_W / 2, WHEEL_W / 2, sides=8, phase=0.0, color="train_black"), sx)
    tire.move(x, y, cz)
    hub = _axis_x(pp.cyl(r * 0.55, WHEEL_W / 2, WHEEL_W / 2 + 0.015, sides=8, color="stripe_white"), sx)
    hub.move(x, y, cz)
    A.add([tire, hub], node=name)


def scallops(y0, y1, z_edge):
    """Five roof-edge scallops (red/white like the roof stripes) as thin prisms in X/Z."""
    out = []
    for i in range(5):
        cx = -0.584 + 0.292 * i
        h = 0.146
        poly = [(cx - h, z_edge), (cx + h, z_edge), (cx + h - 0.03, z_edge - 0.07),
                (cx, z_edge - 0.11), (cx - h + 0.03, z_edge - 0.07)]
        out.append(pp.prism_xz(poly, y0, y1, "cart_red" if i % 2 == 0 else "stripe_white"))
    return out


def golf_cart():
    A = nl.Asset("golf_cart")
    B = zb.box
    p = []
    # chassis, cowl, rear body, side panels, white band / bumpers / fenders
    p.append(B((1.0, 2.5, 0.1), (0, 0, 0.27), color="cart_red"))
    p.append(B((0.96, 0.74, 0.40), (0, -0.91, 0.44), color="cart_red", bevel=0.06, bevel_edges="top+vertical"))
    p.append(B((0.96, 0.72, 0.28), (0, 0.92, 0.38), color="cart_red", bevel=0.03, bevel_edges="top"))
    for sx in (-1, 1):
        p.append(B((0.06, 1.0, 0.25), (sx * 0.5, 0.0, 0.42), color="cart_red"))      # side panel under the seat
        p.append(B((0.08, 1.0, 0.08), (sx * 0.52, 0.0, 0.26), color="stripe_white"))  # white sill
        for y, zc in ((-0.85, 0.6), (0.8, 0.64)):                                      # white fenders
            p.append(B((0.3, 0.74, 0.07), (sx * 0.58, y, zc), color="stripe_white", bevel=0.02))
    p.append(B((1.2, 0.1, 0.12), (0, -1.28, 0.3), color="stripe_white", bevel=0.03))   # bumpers
    p.append(B((1.2, 0.1, 0.12), (0, 1.28, 0.3), color="stripe_white", bevel=0.03))
    # dashboard + steering column
    p.append(B((1.0, 0.16, 0.16), (0, -0.5, 0.7), color="stripe_white", bevel=0.03))
    p.append(pp.beam((0.27, -0.52, 0.66), (0.27, -0.4, 0.85), 0.045, color="metal"))
    # bench: cream block, red cushion, cream backrest
    p.append(B((0.96, 0.55, 0.2), (0, 0.225, 0.35), color="horse_cream"))
    p.append(B((0.94, 0.5, 0.06), (0, 0.225, 0.48), color="cart_red", bevel=0.015))
    p.append(B((0.96, 0.1, 0.42), (0, 0.55, 0.69), color="horse_cream", bevel=0.02))
    p.append(B((0.84, 0.05, 0.3), (0, 0.49, 0.66), color="cart_red"))
    # cargo bed: wooden floor sides + wicker basket
    p.append(B((0.05, 0.66, 0.15), (0.45, 0.95, 0.575), color="wood_light"))
    p.append(B((0.05, 0.66, 0.15), (-0.45, 0.95, 0.575), color="wood_light"))
    p.append(B((0.95, 0.05, 0.15), (0, 1.25, 0.575), color="wood_light"))
    p.append(pp.cyl(0.22, 0.5, 0.78, sides=8, r1=0.27, center=(0, 0.95), color="wicker"))
    p.append(pp.cyl(0.27, 0.78, 0.8, sides=8, center=(0, 0.95), color="wicker_dark", top=False))
    # roof posts (front ones raked), windscreen bar, roof, stripes, scalloped edges
    for sx in (-1, 1):
        p.append(pp.beam((sx * 0.6, -0.68, 0.6), (sx * 0.6, -0.55, 1.95), 0.05, color="stripe_white"))
        p.append(pp.beam((sx * 0.6, 0.62, 0.55), (sx * 0.6, 0.62, 1.95), 0.05, color="stripe_white"))
    p.append(pp.beam((-0.6, -0.63, 1.42), (0.6, -0.63, 1.42), 0.035, color="stripe_white"))
    p.append(B((1.46, 1.9, 0.07), (0, 0.1, 1.965), color="stripe_white", bevel=0.025, bevel_edges="top"))
    for i in (0, 2, 4):
        cx = -0.584 + 0.292 * i
        p.append(pp.flat([(cx - 0.146, -0.84, 2.001), (cx + 0.146, -0.84, 2.001),
                          (cx + 0.146, 1.04, 2.001), (cx - 0.146, 1.04, 2.001)], "cart_red"))
    p += scallops(-0.87, -0.83, 1.94)
    p += scallops(1.01, 1.05, 1.94)
    # emblem discs (blank), tail lights, horn
    for sx in (-1, 1):
        d = pp.disc((0, 0, 0), 0.1, "stripe_white", sides=8)
        p.append(d.rotate(90 * sx, "Y").move(sx * 0.535, 0.0, 0.42))
        p.append(B((0.1, 0.03, 0.09), (sx * 0.36, 1.27, 0.4), color="porthole_red"))
    p.append(pp.cyl(0.04, 0, 0.05, sides=6, color="iron").rotate(90, "X").move(0, -1.28, 0.36))
    A.add(p)
    # wheels
    for name, x, y, r in WHEELS:
        wheel(A, name, x, y, r)
    # headlights (glow slot)
    for name, sx in (("headlight_l", 1), ("headlight_r", -1)):
        c = (sx * 0.32, -1.28, 0.5)
        A.node(name, pivot=c)
        lamp = pp.cyl(0.13, 0, 0.05, sides=8, color="bulb_yellow").rotate(90, "X").move(*c)
        A.add(lamp, mat="lamp_glow", node=name)
        A.empty("light_" + name[-1], (c[0], c[1] - 0.15, c[2]), light=(2.5, "#FFE08A"))
    # steering wheel (tilted ~20 deg towards the driver), pivot = hub
    hub = (0.27, -0.38, 0.88)
    A.node("steering_wheel", pivot=hub)
    ring = nl.ring_xz(0, 0, 0.19, 8, -0.015, 0.015, "cart_red", r_in=0.15)
    spoke = B((0.3, 0.02, 0.03), (0, 0, 0), color="cart_red")
    knob = B((0.07, 0.04, 0.07), (0, 0, 0), color="iron")
    tilt = Matrix.Translation(hub) @ Matrix.Rotation(math.radians(-20), 4, "X")
    A.add([q.transform(tilt) for q in (ring, spoke, knob)], node="steering_wheel")
    # driver socket: centre of the left seat cushion
    A.empty("socket_driver", (0.27, 0.22, 0.5))
    return A


def parking_sign():
    A = nl.Asset("parking_sign")
    B = zb.box
    p = [B((0.2, 0.2, 0.04), (0, 0, 0.02), color="edge_stone", bevel=0.01),
         B((0.06, 0.06, 1.2), (0, 0, 0.64), color="metal"),
         B((0.46, 0.04, 0.46), (0, 0, 1.3), color="sign_blue", bevel=0.015)]
    y = FRONT * 0.0205
    def q(x0, x1, z0, z1):
        return pp.flat([(x0, y, z0), (x1, y, z0), (x1, y, z1), (x0, y, z1)], "stripe_white")
    # the letter P: stem, top, middle bar, right side of the bowl
    p += [q(-0.11, -0.05, 1.14, 1.46), q(-0.11, 0.09, 1.40, 1.46), q(-0.11, 0.09, 1.28, 1.34),
          q(0.05, 0.11, 1.28, 1.46)]
    A.add(p)
    return A


def main():
    nl.run(KIT, {"golf_cart": golf_cart, "parking_sign": parking_sign}, cols=2, spacing=(3.6, 2.4),
           budgets={"golf_cart": 1000, "parking_sign": 150})


if __name__ == "__main__":
    main()
