"""badger — Dachs (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/badger.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/badger.blend, assets/models/animals/badger.glb,
assets/textures/animals/badger_body.png, art/animals/badger/model_preview.png.

Look: art/animals/badger/{front,side,back,three_quarter}.png (sheet_v2). Low, wide grey body
on short dark-grey legs, long white face with two dark stripes running from the nose over the
eyes to the ears (white crown stripe between), big black nose, small round ears with white
rims, short grey tail. Top of the head 0.7 m, length ~1.05 m (Q-143).
Clips: idle, walk, eat, happy, refuse, sleep (curled up). Eye caps: `eye_glow`.
"""

import math
import os
import sys

import numpy as np

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import night_kit as nk  # noqa: E402
import quad_kit as qk  # noqa: E402
from quad_kit import FWD, X, Y, Z, Tube, ellipsoid  # noqa: E402,F401

qr = qk.qr
V = qk.V3
ASSET = "badger"

COLORS = {
    "fur": "#8C8C8A",
    "leg": "#3A3A3A",
    "white": "#F4F2EC",
    "stripe": "#262626",
    "nose": "#1E1E1E",
    "light": "#A9A8A4",
    "eye_white": "#FFFFFF",
    "iris": "#4A4640",
    "pupil": "#161412",
    "ink": "#241A14",
}

HEAD_B = V((0, -0.265, 0.53))
HEAD_D = V((0, -1.0, -0.33)).normalized()
HEAD_PROFILE = [
    (0.01, 0.09, 0.09, 0.09), (0.05, 0.17, 0.16, 0.15), (0.10, 0.19, 0.175, 0.16),
    (0.16, 0.17, 0.15, 0.13), (0.22, 0.12, 0.10, 0.09), (0.27, 0.078, 0.066, 0.062),
    (0.31, 0.052, 0.046, 0.046), (0.33, 0.032, 0.03, 0.03)]
HEAD_N = HEAD_B + HEAD_D * 0.34

P = {
    "hips": (0, 0.19, 0.36), "spine": (0, 0.0, 0.36), "chest": (0, -0.19, 0.37),
    "neck_1": (0, -0.24, 0.42), "neck_2": (0, -0.27, 0.46), "head": (0, -0.29, 0.50),
    "nose": tuple(HEAD_N),
    "ear_base": (0.135, -0.24, 0.63), "ear_tip": (0.165, -0.225, 0.70),
    "tail_1": (0, 0.30, 0.40), "tail_2": (0, 0.37, 0.36), "tail_end": (0, 0.46, 0.25),
    "front_xy": (0.13, -0.19), "front_z": (0.30, 0.16, 0.05),
    "hind_xy": (0.14, 0.19), "hind_z": (0.31, 0.17, 0.05),
    "toe": 0.07,
}


def build(k):
    pal = k.pal
    T = [(0.33, 0.35, 0.08, 0.08, 0.08), (0.28, 0.35, 0.19, 0.16, 0.18), (0.17, 0.34, 0.23, 0.17, 0.20),
         (0.03, 0.335, 0.23, 0.165, 0.20), (-0.10, 0.34, 0.22, 0.165, 0.20), (-0.20, 0.35, 0.20, 0.16, 0.19),
         (-0.27, 0.36, 0.16, 0.14, 0.16), (-0.31, 0.37, 0.08, 0.08, 0.08)]

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[z < 0.2] = pal("light")
        return c
    k.torso(T, color=torso_col, leg_r=0.07, cap=(0.03, 0.03))
    neck = Tube([(0, -0.19, 0.40), (0, -0.25, 0.45), (0, -0.28, 0.50)],
                [(0.16, 0.12, 0.15), (0.155, 0.12, 0.145), (0.15, 0.12, 0.14)], n=12,
                up=V((0, 0.4, 1)), lat=X, cap=(0.01, 0.01))
    k.add(neck, lambda co: qr.chain(co.y, [(-0.31, "head"), (-0.25, "neck_1"), (-0.17, "chest")]),
          cell="fur")

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "white")
        th = np.degrees(np.arccos(np.clip(2 * v - 1, -1, 1)))   # 0 = top of the head
        c[(th > 22) & (th < 64 - 10 * np.clip(u - 0.6, 0, 1)) & (u > 0.12) & (u < 0.86)] = pal("stripe")
        c[u < 0.1] = pal("fur")
        return c
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, color=head_col)
    k.eyes(head, 0.50, 42.0, (0.045, 0.05), depth=0.016)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.40, look=0.08)
    k.add(ellipsoid(HEAD_N + V((0, 0.012, 0.012)), FWD, Z, 0.022, 0.026, 0.042, 0.03, 0.028, rings=3, n=8),
          "head", cell="nose")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.055, 0.02, V((0.3, -1, 0.1)), "white", "stripe",
              prof=[(-0.3, 0.85, 0.9), (0.1, 1.0, 1.0), (0.5, 0.95, 0.9), (0.85, 0.6, 0.8),
                    (0.98, 0.25, 0.5)], inner_frac=0.6, inner_t=(0.0, 0.8))

    front = [(0.29, 0.085, 0.08, 0.08, 0.0), (0.20, 0.075, 0.07, 0.07, 0.0),
             (0.12, 0.066, 0.064, 0.064, 0.0), (0.05, 0.062, 0.06, 0.062, -0.005)]
    hind = [(0.30, 0.10, 0.095, 0.085, 0.01), (0.22, 0.08, 0.075, 0.072, 0.008),
            (0.13, 0.066, 0.064, 0.064, 0.0), (0.05, 0.062, 0.06, 0.062, -0.005)]

    def leg_col(x, y, z, u, v):
        c = pal.fill(len(x), "leg")
        c[z > 0.27] = pal("fur")
        return c

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.035, 0.035), FWD, Z, 0.05, 0.07, 0.07, 0.04, 0.035, rings=4, n=8)
    k.legs(front, hind, color=leg_col, paw=paw, paw_cell="leg", n=8)

    tail = Tube([(0, 0.29, 0.41), (0, 0.36, 0.37), (0, 0.42, 0.31), (0, 0.46, 0.25)],
                [(0.05, 0.05, 0.05), (0.06, 0.055, 0.055), (0.045, 0.04, 0.04), (0.02, 0.02, 0.02)],
                n=8, up=Y, lat=X, cap=(0.01, 0.03))
    k.add(tail, k.w_along(tail, [(0.05, "hips"), (0.2, "tail_1"), (0.5, "tail_1"), (0.7, "tail_2")]),
          cell="fur")


CLIPS = ["idle", "walk", "eat", "happy", "refuse", "sleep"]


def clips(k):
    cs = nk.QuadClips(k, {
        "sc": 0.36,
        "tail_amp": 8.0,
        "walk": dict(frames=12, stance=0.45, lift=0.05, toe_deg=28.0, fold_deg=50.0, toe_fwd=0.07,
                     dip=0.01),
        "eat": dict(nose_z=0.06, body=(0.03, 5.0), split=(0.6, 0.4, -0.3)),
        "happy": dict(hop=1.0),
        "head_look": 8.0,
        "sleep": dict(drop=0.2, curl=20.0, neck=26.0, head=20.0),
    })
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in cs.walk.drop))
    return cs.clips(CLIPS)


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    nk.run(kit, build, clips,
           dict(loco=("walk", 3), center_z=0.38, height_m=2.2, big_scale=1.5, close_scale=1.3,
                night_pose=("idle", 20)),
           debug=dict(center_z=0.35, head_pt=(0, -0.42, 0.5), head_scale=0.6, strip_scale=1.5))
