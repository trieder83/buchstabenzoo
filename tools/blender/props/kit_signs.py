"""Kit 3 — signs, boards, food boxes (concept: art/props/kit_signs/sheet_v5.jpg, brief.md).

Run:  blender -b --factory-startup --python tools/blender/props/kit_signs.py -- [--no-preview]

Outputs
  assets/models/props/<asset>.glb     one per asset
  assets/blender/props/kit_signs.blend
  art/props/kit_signs/model_preview.png

All panels are BLANK: text, silhouettes and the level map are drawn by the game from i18n
keys (ART-ENVIRONMENT §2). Every overlay area is its own palette cell, so the renderer can
find it by UV (and the script prints its centre/normal in world / glTF space):
  sign_panel  enclosure_sign text panel        sign_slot   enclosure_sign round slot
  info_panel  info_board panel (riddle)        map_panel   map_board panel
  label_plate food_box / food_box_stack label plates

Placement conventions (world axes, GAME-LAYOUT "Coordinate spaces", Q-056: +X east, +Y up,
north = world -Z = Blender +Y; never mirror, only rotate about +Y; origin on the ground):
- Every board READS TOWARDS SOUTH (world +Z): the default follow camera stands south of the
  player looking north. To face another direction rotate about +Y: +90 deg -> faces east,
  -90 deg -> faces west, 180 deg -> faces north (see README_kits_3_6.md).
- enclosure_sign: 2.36 m wide (posts at x = +-1.09), origin = centre between the posts.
  Panel tilted back 40 deg. Stand it next to / above the enclosure gate, outside the fence.
- info_board: single post, origin at the post foot; panel 0.8 x 0.55 m, tilted back 40 deg,
  panel centre ~1.05 m (child eye height). Fits one 1 m cell.
- map_board: 2.1 m wide on two posts, small gable roof; panel tilted back 15 deg. Origin =
  centre between the posts.
- food_box: 0.6 m crate, origin at the centre of its footprint; label plate on the front
  (south). food_box_stack: three crates, origin at the centre of the two bottom crates.
"""

import math
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import props_parts as pp  # noqa: E402
from props_parts import zb  # noqa: E402

KIT = "kit_signs"
FRONT = pp.FRONT


def tilt(parts, deg, z, y=0.0):
    """Tilt panel-local parts (panel in the X/Z plane, front = FRONT (south), centred on the
    origin) back by `deg` (top edge away from the camera) and move the centre to height z.
    y is measured towards the front."""
    for p in parts:
        p.rotate(deg * FRONT, "X")
        p.move(0, y * FRONT, z)
    return parts


def framed_panel(w, h, cell, frame=0.07, depth=0.07):
    """Wooden board with a raised frame and a blank inset panel face (colour `cell`)."""
    parts = [zb.box((w, depth, h), (0, 0, 0), color="wood", bevel=0.015)]
    fy = depth / 2 + 0.02
    for sx in (-1, 1):
        parts.append(zb.box((frame, 0.04, h), (sx * (w / 2 - frame / 2), FRONT * (fy - 0.02), 0), color="wood_light",
                            bevel=0.012))
    for sz in (-1, 1):
        parts.append(zb.box((w - 2 * frame, 0.04, frame), (0, FRONT * (fy - 0.02), sz * (h / 2 - frame / 2)),
                            color="wood_light", bevel=0.012))
    face = zb.box((w - 2 * frame, 0.012, h - 2 * frame), (0, FRONT * (depth / 2 + 0.004), 0), color="wood")
    pp.recolor(face, lambda f: f.normal.y * FRONT > 0.9, cell)
    parts.append(face)
    return parts


def enclosure_sign():
    W, H = 2.0, 0.9
    TILT = 40.0
    ZC = 1.32
    panel = [zb.box((W, 0.07, H), (0, 0, 0), color="wood", bevel=0.015)]
    # left plate with the round silhouette slot
    lw = 0.56
    lx = -W / 2 + lw / 2
    panel.append(zb.box((lw, 0.06, H + 0.04), (lx, FRONT * 0.05, 0), color="wood_light", bevel=0.02))
    slot = pp.cyl(0.19, 0.0, 0.02, sides=12, color="wood_dark")
    pp.recolor(slot, lambda f: f.normal.z > 0.9, "sign_slot")
    slot.rotate(-90 * FRONT, "X").move(lx, FRONT * 0.075, 0.02)
    panel.append(slot)
    # right: framed text panel
    fw = W - lw - 0.02
    fp = framed_panel(fw, H - 0.04, "sign_panel", depth=0.02)
    for p in fp:
        p.move(W / 2 - fw / 2, FRONT * 0.045, 0)
    panel += fp
    parts = tilt(panel, TILT, ZC)
    for sx, h in ((-1, 1.95), (1, 1.8)):
        parts.append(zb.box((0.18, 0.18, h), (sx * (W / 2 + 0.09), -FRONT * 0.05, h / 2), color="wood", bevel=0.03))
    return parts


def info_board():
    TILT = 40.0
    W, H = 0.8, 0.55
    panel = framed_panel(W, H, "info_panel", frame=0.06, depth=0.06)
    # small gable roof over the top edge (tilts with the panel like on the sheet)
    for s in (-1, 1):
        r = zb.box((W + 0.2, 0.2, 0.035), (0, 0, 0), color="wood_light", bevel=0.01)
        r.rotate(s * 32, "X").move(0, s * 0.085, H / 2 + 0.1)
        panel.append(r)
    panel.append(zb.box((W + 0.2, 0.05, 0.05), (0, 0, H / 2 + 0.155), color="wood_dark", bevel=0.01))
    panel.append(zb.box((0.05, 0.05, 0.12), (-W / 2 + 0.05, 0, H / 2 + 0.05), color="wood"))
    panel.append(zb.box((0.05, 0.05, 0.12), (W / 2 - 0.05, 0, H / 2 + 0.05), color="wood"))
    parts = tilt(panel, TILT, 1.05, y=0.04)
    parts.append(zb.box((0.14, 0.14, 1.0), (0, -FRONT * 0.06, 0.5), color="wood", bevel=0.025))
    parts.append(zb.box((0.1, 0.1, 0.2), (0, -FRONT * 0.02, 0.92), color="wood_dark", bevel=0.01))
    return parts


def map_board():
    W, H = 1.8, 1.15
    TILT = 15.0
    panel = framed_panel(W, H, "map_panel", frame=0.08, depth=0.07)
    parts = tilt(panel, TILT, 1.28, y=0.02)
    PX = W / 2 + 0.09
    for sx in (-1, 1):
        parts.append(zb.box((0.16, 0.16, 2.2), (sx * PX, -FRONT * 0.06, 1.1), color="wood", bevel=0.03))
    # gable roof along X on the post tops
    for s in (-1, 1):
        r = zb.box((2 * PX + 0.25, 0.34, 0.05), (0, 0, 0), color="wood_light", bevel=0.012)
        r.rotate(s * 30, "X").move(0, -FRONT * 0.06 + s * 0.145, 2.3)
        parts.append(r)
    parts.append(zb.box((2 * PX + 0.28, 0.07, 0.07), (0, -FRONT * 0.06, 2.39), color="wood_dark"))
    parts.append(zb.box((2 * PX, 0.1, 0.1), (0, -FRONT * 0.06, 2.17), color="wood_dark"))
    return parts


# --------------------------------------------------------------------------- food boxes

def crate(cx=0.0, cy=0.0, z0=0.0, rot=0.0, s=0.6):
    """Closed wooden crate s x s x s: plank body, framed edges, lid, side brace, label."""
    e = 0.07          # edge beam thickness
    parts = [zb.box((s - 0.04, s - 0.04, s - 0.02), (0, 0, (s - 0.02) / 2), color="wood_light")]
    h = s / 2
    # vertical edge beams
    for sx in (-1, 1):
        for sy in (-1, 1):
            parts.append(zb.box((e, e, s), (sx * (h - e / 2), sy * (h - e / 2), h), color="wood"))
    # lid rim (top ring) and bottom ring on the visible front/side
    for sy in (-1, 1):
        parts.append(zb.box((s - 2 * e, e, e), (0, sy * (h - e / 2), s - e / 2), color="wood"))
        parts.append(zb.box((e, s - 2 * e, e), (sy * (h - e / 2), 0, s - e / 2), color="wood"))
    parts.append(zb.box((s - 2 * e, e, e), (0, FRONT * (h - e / 2), e / 2), color="wood"))
    parts.append(zb.box((e, s - 2 * e, e), (h - e / 2, 0, e / 2), color="wood"))
    # lid planks (two seams)
    for k in (-1, 1):
        parts.append(zb.box((s - 2 * e, 0.025, 0.012), (0, k * (s - 2 * e) / 6, s - 0.012), color="wood_dark"))
    # diagonal brace on the east side
    L = math.hypot(s - 2 * e, s - 2 * e)
    br = zb.box((0.05, L, 0.07), (0, 0, 0), color="wood")
    br.rotate(45, "X").move(h - 0.005, 0, h)
    parts.append(br)
    # label plate on the front (south)
    lab = zb.box((0.3, 0.014, 0.17), (0, FRONT * (h - 0.013), h + 0.02), color="wood_dark")
    pp.recolor(lab, lambda f: f.normal.y * FRONT > 0.9, "label_plate")
    parts.append(lab)
    for p in parts:
        p.rotate_z(rot)
        p.move(cx, cy, z0)
    return parts


def food_box():
    return crate()


def food_box_stack():
    parts = crate(-0.32, 0.03, 0.0, rot=-3)
    parts += crate(0.32, -0.06, 0.0, rot=4)
    parts += crate(0.0, 0.0, 0.6, rot=-6)
    return parts


def extra(objs):
    # sample: sign on a fence line, info board beside it, three food boxes in a row
    fwd, right = zb.camera_basis(zb.PREVIEW_YAW_DEG)
    B = right * 9.0 - fwd * 1.0
    V = zb.Vector
    out = [pp.instance(objs["food_box"], f"s_box{i}", B + V((0.7 * i, 0, 0))) for i in range(3)]
    out.append(pp.instance(objs["info_board"], "s_board_e", B + V((0.0, -2.0, 0)), rot_deg=90))
    return out


def main():
    builders = {
        "enclosure_sign": enclosure_sign,
        "info_board": info_board,
        "map_board": map_board,
        "food_box": food_box,
        "food_box_stack": food_box_stack,
    }
    pp.run_kit(KIT, builders, cols=3, spacing=(3.2, 3.2), extra=extra,
               overlays=("sign_panel", "sign_slot", "info_panel", "map_panel", "label_plate"))


if __name__ == "__main__":
    main()
