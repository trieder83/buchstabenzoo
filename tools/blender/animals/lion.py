"""lion — Löwe (ART-ANIMALS; quadruped rig, built with quad_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/lion.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/lion.blend, assets/models/animals/lion.glb,
assets/textures/animals/lion_body.png, art/animals/lion/model_preview.png.

Look: art/animals/lion/{front,side,back,three_quarter}.png. Golden chunky cat, big round
fluffy orange-brown mane ring around the head (reads like a sun from above), cream muzzle,
bib and paws, rose-brown nose, tail with an orange tuft. Top of the mane 1.5 m.
"""

import math
import os
import sys

import numpy as np

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import quad_kit as qk  # noqa: E402
from quad_kit import FWD, X, Y, Z, Tube, ellipsoid, rx, ry  # noqa: E402,F401

qr = qk.qr
V = qk.V3
ASSET = "lion"

COLORS = {
    "gold": "#E9B656",
    "mane": "#C06A2B",
    "cream": "#F6E2B4",
    "nose": "#9A5A4A",
    "ear_in": "#B87850",
    "eye_white": "#FFFFFF",
    "iris": "#6B3A1E",
    "pupil": "#1E1E1E",
    "ink": "#2B1B12",
}

HEAD_B = V((0, -0.50, 1.19))
HEAD_D = V((0, -1.0, -0.30)).normalized()
HEAD_PROFILE = [  # s (m from B), r_lat, r_up, r_down
    (0.02, 0.12, 0.11, 0.11), (0.08, 0.215, 0.20, 0.19), (0.15, 0.25, 0.235, 0.215),
    (0.23, 0.245, 0.23, 0.20), (0.30, 0.205, 0.19, 0.17), (0.36, 0.165, 0.15, 0.16),
    (0.42, 0.15, 0.125, 0.14), (0.47, 0.11, 0.09, 0.10)]
HEAD_N = HEAD_B + HEAD_D * 0.50

P = {
    "hips": (0, 0.32, 0.78), "spine": (0, 0.02, 0.78), "chest": (0, -0.30, 0.80),
    "neck_1": (0, -0.44, 0.88), "neck_2": (0, -0.50, 1.00), "head": (0, -0.54, 1.12),
    "nose": tuple(HEAD_N),
    "ear_base": (0.15, -0.60, 1.33), "ear_tip": (0.20, -0.61, 1.45),
    "tail_1": (0, 0.55, 0.88), "tail_2": (0, 0.72, 0.62), "tail_end": (0, 0.86, 0.40),
    "front_xy": (0.175, -0.30), "front_z": (0.72, 0.37, 0.09),
    "hind_xy": (0.175, 0.32), "hind_z": (0.74, 0.40, 0.10),
    "toe": 0.10,
}


def build(k):
    pal = k.pal
    # ---- torso (back -> front)
    T = [(0.56, 0.76, 0.13, 0.11, 0.13), (0.49, 0.76, 0.23, 0.20, 0.23),
         (0.38, 0.76, 0.275, 0.235, 0.265), (0.20, 0.755, 0.28, 0.23, 0.25),
         (0.02, 0.755, 0.28, 0.23, 0.245), (-0.16, 0.765, 0.295, 0.245, 0.275),
         (-0.32, 0.775, 0.30, 0.25, 0.285), (-0.45, 0.785, 0.27, 0.225, 0.26),
         (-0.54, 0.795, 0.18, 0.16, 0.18)]
    torso = Tube([(0, y, z) for y, z, *_ in T], [a[2:] for a in T],
                 n=16, up=Z, lat=X, cap=(0.05, 0.05))

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "gold")
        belly = z < 0.585 + 0.02 * np.cos(y * 6)
        bib = (y < -0.42) & (z < 0.80) & (np.abs(x) < 0.17)
        c[belly | bib] = pal("cream")
        return c
    k.add(torso, k.w_torso(leg_r=0.10), region="torso", color=torso_col)

    # ---- neck (hidden in the mane; blends chest -> head)
    neck = Tube([(0, -0.34, 0.84), (0, -0.46, 0.95), (0, -0.54, 1.07)],
                [(0.19, 0.19, 0.20), (0.18, 0.18, 0.19), (0.16, 0.16, 0.17)], n=10, up=V((0, 0.5, 1)),
                lat=X, cap=(0.02, 0.02))
    k.add(neck, k.w_along(neck, [(0.1, "chest"), (0.35, "neck_1"), (0.65, "neck_2"), (0.95, "head")]),
          cell="gold")

    # ---- head
    head = Tube([HEAD_B + HEAD_D * s for s, *_ in HEAD_PROFILE],
                [(rl, ru, rd) for _, rl, ru, rd in HEAD_PROFILE], n=14,
                up=(Z - HEAD_D * HEAD_D.dot(Z)), lat=X, cap=(0.03, 0.03))

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "gold")
        muz = (u > 0.62) & (v < 0.62)
        muz |= (u > 0.70) & (v < 0.80)
        chin = (u > 0.45) & (v < 0.22)
        c[muz | chin] = pal("cream")
        return c
    k.add(head, "head", region="head", color=head_col, name="head")
    k.eyes(head, 0.47, 40.0, (0.068, 0.082), depth=0.03)
    k.eye_swatch(iris="iris", iris_r=0.64, pupil_r=0.40, look=0.10)
    # nose (rose, on top of the muzzle tip)
    nose_c = HEAD_B + HEAD_D * 0.455 + V((0, 0, 0.075))
    k.add(ellipsoid(nose_c, FWD + V((0, 0, -0.2)), Z, 0.03, 0.035, 0.055, 0.035, 0.03, rings=4, n=10),
          "head", cell="nose")
    # muzzle pads (cream, round cheeks under the nose)
    for sg in (1, -1):
        pc = HEAD_B + HEAD_D * 0.40 + V((0.055 * sg, 0, -0.03))
        k.add(ellipsoid(pc, FWD, Z, 0.05, 0.06, 0.07, 0.065, 0.06, rings=4, n=10), "head", cell="cream")

    # ---- ears (small round, in front of the mane)
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.075, 0.03, FWD, "gold", "ear_in",
              prof=[(-0.2, 0.7, 0.9), (0.2, 1.0, 1.0), (0.55, 0.95, 0.9), (0.85, 0.6, 0.7)],
              inner_frac=0.6)

    # ---- mane: big scalloped ball around the head
    def scallop(i, th):
        return 1.0 + 0.075 * math.cos(9 * th) * (0.4 + 0.6 * min(1.0, i / 3))
    mane = ellipsoid((0, -0.50, 1.10), FWD + V((0, 0, 0.12)), Z, 0.32, 0.22, 0.39, 0.40, 0.42,
                     rings=7, n=18, lat=X, mod=scallop)
    k.add(mane, lambda co: qr.chain(co.y, [(-0.56, "head"), (-0.36, "neck_2"), (-0.26, "neck_1")]),
          cell="mane")

    # ---- legs + paws
    front = [(0.70, 0.125, 0.12, 0.12, 0.0), (0.50, 0.115, 0.11, 0.108, 0.0), (0.32, 0.098, 0.092, 0.094, 0.0),
             (0.16, 0.092, 0.088, 0.09, -0.005), (0.06, 0.095, 0.09, 0.094, -0.01)]
    hind = [(0.74, 0.18, 0.17, 0.14, 0.02), (0.56, 0.15, 0.14, 0.125, 0.02), (0.40, 0.10, 0.10, 0.095, 0.0),
            (0.20, 0.092, 0.088, 0.09, -0.005), (0.07, 0.095, 0.09, 0.094, -0.01)]

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.05, 0.066), FWD, Z, 0.085, 0.115, 0.11, 0.075, 0.066, rings=5, n=10)
    k.legs(front, hind, cell="gold", paw=paw, paw_cell="cream")

    # ---- tail + tuft
    tail = Tube([(0, 0.51, 0.90), (0, 0.62, 0.80), (0, 0.72, 0.62), (0, 0.80, 0.50), (0, 0.86, 0.40)],
                [(0.04, 0.04, 0.04), (0.036, 0.036, 0.036), (0.032, 0.032, 0.032), (0.03, 0.03, 0.03),
                 (0.03, 0.03, 0.03)], n=6, up=Y, lat=X, cap=(0.03, 0.02))
    k.add(tail, k.w_along(tail, [(0.15, "hips"), (0.3, "tail_1"), (0.55, "tail_1"), (0.75, "tail_2")]),
          cell="gold")
    k.add(ellipsoid((0, 0.89, 0.34), V((0, 0.3, -1)), Y, 0.07, 0.08, 0.065, 0.06, rings=5, n=8,
                    mod=lambda i, th: 1 + 0.12 * math.cos(5 * th)), "tail_2", cell="mane")


def clipset(k):
    return qk.ClipSet(k, {
        "sc": 0.75,
        "tail_amp": 12.0,
        "walk": dict(frames=20, stance=0.5, lift=0.09, toe_deg=30.0, fold_deg=55.0, toe_fwd=0.10,
                     dip=0.02),
        "eat": dict(nose_z=0.14, body=(0.06, 8.0), split=(0.62, 0.38, -0.45)),
        "drink": dict(nose_z=0.10, body=(0.08, 10.0), split=(0.62, 0.38, -0.45)),
        "happy": dict(hop=0.9),
        "head_look": 7.0,
    })


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    qk.run(kit, build, clipset, ["idle", "walk", "eat", "drink", "happy", "refuse"],
           dict(center_z=0.75, walk_frame=4, big_scale=2.6, close_scale=2.4,
                head_pt=(0, -0.72, 1.12), debug_scale=0.8))
