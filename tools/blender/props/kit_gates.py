"""Kit gates — doors, gates and turnstiles with separable moving parts (user request
2026-09-27; concepts: art/props/kit_garden/sheet_furniture_v2.jpg (picket gate),
art/environment/env_night_house/cutaway.png (glass fronts), art/props/kit_buildings/sheet_v2.jpg
(doors, turnstiles)).

Run:  blender -b --factory-startup --python tools/blender/props/kit_gates.py -- [--no-preview] [asset ...]

Outputs  assets/models/props/<asset>.glb, assets/blender/props/kit_gates.blend,
         art/props/kit_gates/model_preview.png (day on top, night-tinted below; the right-hand
         sample row checks gate_wood between two fence_wood pieces)

Front = south = glTF +Z at yaw 0. All leaves are closed as modelled; "open" = rotate the leaf
node about glTF +Y around its pivot (translation of the node = hinge axis foot):
- garden_gate  2 m opening between two ball-topped posts (root, centres x = +-1.05); leaves
               `leaf_l` (pivot x = -0.99) and `leaf_r` (pivot x = +0.99), 0.98 m each; they
               swing to the BACK (-Z): leaf_l +90 deg, leaf_r -90 deg. Turn the gate so its back
               faces the inside of the garden (leaves swing inwards).
- glass_door   2 m gate of a night-house indoor enclosure (no posts: the house's glass front has
               them): `leaf_l` / `leaf_r` pivots x = -+0.975, frame + `glass` panes; open to
               the back like garden_gate (into the enclosure).
- door_wood    building door leaf 0.94 x 2.1 m; ROOT node = the leaf, origin = hinge axis foot
               (like gate_wood); the leaf runs along +X; open = rotate about +Y (+90 swings it
               to the back / inside). Small round window = `door_glow` slot.
- gate_zoo     level gate (GAME-LAYOUT "Gates between the levels", LAYOUT-036): the
               gate_zoo_closed look (kit_barriers) with separable leaves — two stone pillars
               (root, centres x = +-1.25, 0.5 m wide, outer faces +-1.5 = the 3 m gap in the
               hedge), plank leaves `leaf_l` (pivot x = -1.0) and `leaf_r` (pivot x = +1.0),
               1.0 m each with an arched top; padlock on leaf_r. Open to the BACK (-Z) like
               garden_gate: turn the gate so its back faces the new level.
- turnstile    one 1.2 m lane unit: wooden pedestal at x = -0.45 (root) and the metal
               `arms` node (pivot = the hub, x = -0.33) blocking the lane to x = +0.52; three
               units side by side fill the 3.6 m arch opening with 8 cm gaps (no shared faces,
               ARCH-005).
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

KIT = "kit_gates"


# --------------------------------------------------------------------------- garden gate

def picket(cx, w, h, y0, y1, color="trim_white"):
    return pp.prism_xz([(cx - w / 2, 0.06), (cx + w / 2, 0.06), (cx + w / 2, h - 0.08), (cx, h),
                        (cx - w / 2, h - 0.08)], y0, y1, color)


def picket_leaf(side, x_h, x_c, h=0.85):
    """Picket leaf between the hinge x_h and the centre x_c (asset coordinates)."""
    xa, xb = min(x_h, x_c), max(x_h, x_c)
    parts = []
    n = 6
    pw = 0.11
    for k in range(n):
        cx = xa + pw / 2 + 0.01 + (xb - xa - pw - 0.02) * k / (n - 1)
        hh = h - 0.05 * abs(k - (n - 1) / 2) / ((n - 1) / 2) if False else h
        parts.append(picket(cx, pw, hh, FRONT * 0.015, BACK * 0.015))
    # rails and diagonal brace on the back
    for z in (0.22, 0.62):
        parts.append(zb.box((xb - xa, 0.03, 0.07), ((xa + xb) / 2, BACK * 0.03, z), color="trim_white"))
    ax, bx = (xa + 0.06, xb - 0.06) if side < 0 else (xb - 0.06, xa + 0.06)
    parts.append(pp.beam((ax, BACK * 0.035, 0.25), (bx, BACK * 0.035, 0.6), 0.05, h=0.025, color="trim_white"))
    # hinges on the hinge side (iron)
    for z in (0.22, 0.62):
        parts.append(zb.box((0.1, 0.012, 0.035), (x_h - side * 0.05, FRONT * 0.021, z), color="iron"))
    return parts


def garden_gate():
    A = nl.Asset("garden_gate")
    p = []
    for sx in (-1, 1):
        x = sx * 1.05
        p.append(zb.box((0.1, 0.1, 0.95), (x, 0, 0.475), color="trim_white", bevel=0.015, bevel_edges="vertical"))
        p.append(zb.box((0.14, 0.14, 0.05), (x, 0, 0.975), color="trim_white", bevel=0.01))
        p.append(zb.knob(0.065, (x, 0, 1.06), color="trim_white", u=8, v=5))
    A.add(p)
    for side, nm in ((-1, "leaf_l"), (1, "leaf_r")):
        piv = Vector((side * 0.99, 0, 0))
        A.node(nm, pivot=piv)
        A.add(picket_leaf(side, side * 0.99, side * 0.01), node=nm)
    return A


# --------------------------------------------------------------------------- glass door

def glass_door():
    A = nl.Asset("glass_door")
    H = 2.2
    T = 0.06
    for side, nm in ((-1, "leaf_l"), (1, "leaf_r")):
        x_h = side * 0.975
        x_c = side * 0.01
        xa, xb = min(x_h, x_c), max(x_h, x_c)
        piv = Vector((x_h, BACK * T / 2, 0))
        A.node(nm, pivot=piv)
        f = 0.08
        fr = [zb.box((f, T, H), (xa + f / 2, 0, H / 2), color="house_wood_dark"),
              zb.box((f, T, H), (xb - f / 2, 0, H / 2), color="house_wood_dark"),
              zb.box((xb - xa - 2 * f, T, 0.3), ((xa + xb) / 2, 0, 0.15), color="house_wood_dark"),
              zb.box((xb - xa - 2 * f, T, f), ((xa + xb) / 2, 0, H - f / 2), color="house_wood_dark"),
              zb.box((xb - xa - 2 * f, T * 0.8, 0.05), ((xa + xb) / 2, 0, 1.2), color="house_wood_dark")]
        # push handle on both faces
        for fy in (FRONT, BACK):
            fr.append(zb.box((0.03, 0.04, 0.35), (x_c - side * 0.12, fy * (T / 2 + 0.02), 1.05), color="brass",
                             bevel=0.01))
        A.add(fr, node=nm)
        panes = [zb.box((xb - xa - 2 * f, 0.02, 1.2 - 0.3 - 0.025), ((xa + xb) / 2, 0, (0.3 + 1.175) / 2),
                        color="glass_tint"),
                 zb.box((xb - xa - 2 * f, 0.02, H - f - 1.225), ((xa + xb) / 2, 0, (1.225 + H - f) / 2),
                        color="glass_tint")]
        A.add(panes, mat="glass", node=nm, keep_down=True)
    return A


# --------------------------------------------------------------------------- door

def door_wood():
    A = nl.Asset("door_wood")
    W, H, T = 0.94, 2.1, 0.06
    x0 = 0.01
    xm = x0 + W / 2
    p = [zb.box((W, T, H), (xm, 0, H / 2), color="wood_light", bevel=0.012)]
    # vertical plank grooves (front + back)
    for k in (1, 2, 3):
        gx = x0 + W * k / 4
        for fy in (FRONT, BACK):
            p.append(zb.box((0.02, 0.01, H - 0.12), (gx, fy * (T / 2 + 0.002), H / 2), color="wood"))
    # battens on the back (Z brace) and front frame strips
    for z in (0.3, 1.8):
        p.append(zb.box((W - 0.08, 0.03, 0.12), (xm, BACK * (T / 2 + 0.015), z), color="wood"))
    p.append(pp.beam((x0 + 0.08, BACK * (T / 2 + 0.015), 0.38), (x0 + W - 0.08, BACK * (T / 2 + 0.015), 1.72),
                     0.1, h=0.03, color="wood"))
    # round window frame (front) + knob + hinges
    p.append(nl.ring_xz(xm, 1.62, 0.16, 10, FRONT * (T / 2 + 0.025), BACK * (T / 2 + 0.005), "wood", r_in=0.11))
    for fy in (FRONT, BACK):
        p.append(zb.knob(0.04, (x0 + W - 0.12, fy * (T / 2 + 0.03), 1.0), color="brass", u=6, v=4))
    for z in (0.35, 1.75):
        p.append(zb.box((0.3, 0.012, 0.05), (x0 + 0.15, FRONT * (T / 2 + 0.006), z), color="iron"))
    A.add(p)
    pane = nl.ring_xz(xm, 1.62, 0.115, 10, FRONT * (T / 2 + 0.004), BACK * (T / 2 + 0.004), "window_sky")
    A.add(pane, mat="door_glow")
    return A


# --------------------------------------------------------------------------- level gate

def gate_zoo():
    A = nl.Asset("gate_zoo")
    PW, PD, PH = 0.5, 0.6, 2.6
    PX = 1.25
    p = []
    for s in (-1, 1):
        n = 3
        for k in range(n):
            h = (PH - 0.25) / n
            col = "wall_stone" if k % 2 == 0 else "wall_stone_dark"
            p.append(zb.box((PW - 0.02 * (k % 2), PD - 0.02 * (k % 2), h - 0.03), (s * PX, 0, k * h + h / 2),
                            color=col, bevel=0.04, bevel_edges="vertical"))
        p.append(zb.box((PW + 0.12, PD + 0.12, 0.25), (s * PX, 0, PH - 0.125), color="wall_stone",
                        bevel=0.04, bevel_edges="top+vertical"))
    A.add(p)
    x1 = PX - PW / 2  # inner pillar face (hinge line)
    n = 4
    pw = x1 / n
    yf = FRONT * 0.065
    for side, nm in ((-1, "leaf_l"), (1, "leaf_r")):
        A.node(nm, pivot=Vector((side * x1, 0, 0)))
        leaf = []
        for i in range(n):
            xc = side * (pw * (i + 0.5))
            h = 2.0 + 0.35 * math.cos(math.pi / 2 * xc / x1)
            leaf.append(zb.box((pw - 0.02, 0.1, h), (xc, 0, h / 2), color="wood_light" if i % 2 else "wood"))
        # iron straps (hinge side) and a ring handle on the south face
        for z in (0.5, 1.55):
            leaf.append(zb.box((x1 - 0.12, 0.03, 0.1), (side * (x1 / 2 + 0.02), yf, z), color="iron"))
        leaf.append(pp.cyl(0.07, 0.0, 0.03, sides=8, center=(0, 0), color="iron").rotate(-90 * FRONT, "X")
                    .move(side * 0.13, yf, 1.12))
        if side > 0:
            # padlock hanging from the right ring (goes with the leaf when the gate opens)
            leaf.append(zb.box((0.14, 0.07, 0.13), (0.13, FRONT * 0.11, 1.0), color="brass", bevel=0.02))
            leaf.append(pp.beam((0.09, FRONT * 0.11, 1.07), (0.09, FRONT * 0.11, 1.16), 0.03, color="metal"))
            leaf.append(pp.beam((0.17, FRONT * 0.11, 1.07), (0.17, FRONT * 0.11, 1.16), 0.03, color="metal"))
            leaf.append(pp.beam((0.08, FRONT * 0.11, 1.16), (0.18, FRONT * 0.11, 1.16), 0.03, color="metal"))
        A.add(leaf, node=nm)
    return A


# --------------------------------------------------------------------------- turnstile

def turnstile():
    A = nl.Asset("turnstile")
    px = -0.45
    p = [zb.box((0.2, 0.5, 0.95), (px, 0, 0.475), color="wood", bevel=0.03, bevel_edges="vertical"),
         zb.box((0.26, 0.56, 0.06), (px, 0, 0.98), color="wood_dark", bevel=0.02),
         zb.box((0.28, 0.58, 0.08), (px, 0, 0.04), color="wood_dark")]
    # metal hub housing on the lane side
    p.append(pp.cyl(0.08, 0.6, 0.9, sides=8, center=(px + 0.13, 0), color="metal"))
    A.add(p)
    hub = Vector((px + 0.12, 0, 0))
    A.node("arms", pivot=hub)
    arms = []
    for z in (0.62, 0.86):
        arms.append(pp.beam((px + 0.18, 0, z), (0.52, 0, z), 0.045, sides=6, color="metal"))
        arms.append(zb.knob(0.035, (0.53, 0, z), color="iron", u=6, v=3))
    A.add(arms, node="arms")
    return A


# --------------------------------------------------------------------------- preview extras

def extra(roots):
    """Sample row: fence_wood | gate_wood (closed + one 60 deg open) | fence_wood, and a
    half-open garden gate, glass door and three turnstiles in a 3.6 m arch opening."""
    import bpy
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    B = right * 14.0 + fwd * 2.0
    V = Vector
    out = []

    def load(nm):
        bpy.ops.import_scene.gltf(filepath=zb.repo_path("assets", "models", "props", nm + ".glb"))
        o = [o for o in bpy.context.selected_objects if o.type == "MESH"][0]
        o.rotation_mode = "XYZ"
        o.rotation_euler = (0, 0, 0)
        return o

    try:
        s = 0.0
        f1 = load("fence_wood")
        f1.location = B + V((s - 1.0, 0, 0))
        g = load("gate_wood")
        g.location = B + V((s + 0.09, 0, 0))
        f2 = load("fence_wood")
        f2.location = B + V((s + 3.0, 0, 0))
        out += [f1, g, f2]
        f3 = load("fence_wood")
        f3.location = B + V((s - 1.0, -3.0, 0))
        g2 = load("gate_wood")
        g2.location = B + V((s + 0.09, -3.0, 0))
        g2.rotation_mode = "XYZ"
        g2.rotation_euler = (0, 0, math.radians(60))
        f4 = load("fence_wood")
        f4.location = B + V((s + 3.0, -3.0, 0))
        out += [f3, g2, f4]
    except Exception as e:  # noqa: BLE001
        print("preview: fences not loaded:", e)
    out.append(nl.instance_tree(roots["garden_gate"], "s_gg", B + V((7.0, 0, 0)),
                                node_rot={"leaf_l": 50, "leaf_r": -70}))
    out.append(nl.instance_tree(roots["glass_door"], "s_gd", B + V((7.0, -3.5, 0)),
                                node_rot={"leaf_l": 60, "leaf_r": -60}))
    out.append(nl.instance_tree(roots["gate_zoo"], "s_gz", B + V((11.0, 0, 0)),
                                node_rot={"leaf_l": 70, "leaf_r": -70}))
    for k in range(3):
        out.append(nl.instance_tree(roots["turnstile"], f"s_ts{k}", B + V((3.0 + 1.2 * (k - 1), 3.5, 0))))
    return out


def main():
    builders = {
        "garden_gate": garden_gate,
        "glass_door": glass_door,
        "door_wood": door_wood,
        "turnstile": turnstile,
        "gate_zoo": gate_zoo,
    }
    nl.run(KIT, builders, cols=4, spacing=(3.0, 3.0), extra=extra)


if __name__ == "__main__":
    main()
