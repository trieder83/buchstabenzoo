"""raccoon — Waschbär (ART-ANIMALS "Night animals"; quadruped rig + tail_3, quad_kit + night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/raccoon.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/raccoon.blend, assets/models/animals/raccoon.glb,
assets/textures/animals/raccoon_body.png, art/animals/raccoon/model_preview.png.

Look: art/animals/raccoon/{front,side,back,three_quarter}.png (sheet_v4). Chunky warm-grey
body on four sturdy legs with black paws, big round head with the black eye mask, white
brows and white muzzle, black nose, rounded ears (dark inside), light-grey chest, a thick
bushy tail with black rings. Top of the ears 0.9 m, back 0.55 m (Q-143).
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
ASSET = "raccoon"

COLORS = {
    "fur": "#9A9A96",
    "mask": "#2F2F32",
    "white": "#F2EFE8",
    "light": "#C4C1BA",
    "nose": "#2A2A2A",
    "eye_white": "#FFFFFF",
    "iris": "#3A2418",
    "pupil": "#1A1210",
    "ink": "#241A14",
}

HEAD_B = V((0, -0.19, 0.655))
HEAD_D = V((0, -1.0, -0.2)).normalized()
HEAD_PROFILE = [(s_ * 1.15, a * 1.15, b * 1.15, c * 1.15) for s_, a, b, c in [
    (0.01, 0.09, 0.09, 0.09), (0.05, 0.18, 0.17, 0.16), (0.10, 0.21, 0.195, 0.18),
    (0.15, 0.20, 0.18, 0.16), (0.20, 0.15, 0.12, 0.11), (0.24, 0.085, 0.072, 0.07),
    (0.28, 0.05, 0.045, 0.045), (0.30, 0.03, 0.028, 0.028)]]
HEAD_N = HEAD_B + HEAD_D * 0.355
EYE_U, EYE_TH = 0.52, 36.0
TAIL = [(0, 0.26, 0.50), (0, 0.35, 0.48), (0, 0.43, 0.42), (0, 0.50, 0.34), (0, 0.55, 0.26),
        (0, 0.58, 0.20), (0, 0.59, 0.16)]

P = {
    "hips": (0, 0.17, 0.50), "spine": (0, 0.0, 0.50), "chest": (0, -0.17, 0.51),
    "neck_1": (0, -0.21, 0.56), "neck_2": (0, -0.23, 0.61), "head": (0, -0.25, 0.66),
    "nose": tuple(HEAD_N),
    "ear_base": (0.125, -0.24, 0.80), "ear_tip": (0.18, -0.225, 0.905),
    "tail_1": (0, 0.26, 0.50), "tail_2": (0, 0.43, 0.42), "tail_end": (0, 0.55, 0.26),
    "front_xy": (0.10, -0.17), "front_z": (0.46, 0.24, 0.06),
    "hind_xy": (0.11, 0.17), "hind_z": (0.47, 0.25, 0.06),
    "toe": 0.07,
    "extra_joints": [("tail_3", "tail_2", (0, 0.55, 0.26), (0, 0.595, 0.14))],
}
EYES = []


def build(k):
    pal = k.pal
    T = [(0.30, 0.47, 0.07, 0.07, 0.07), (0.25, 0.47, 0.17, 0.17, 0.19), (0.15, 0.46, 0.20, 0.19, 0.23),
         (0.03, 0.455, 0.20, 0.185, 0.225), (-0.08, 0.46, 0.20, 0.185, 0.225), (-0.17, 0.47, 0.19, 0.18, 0.22),
         (-0.24, 0.48, 0.16, 0.16, 0.18), (-0.28, 0.49, 0.08, 0.08, 0.08)]

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[(y < -0.12) & ((x / 0.13) ** 2 + ((z - 0.44) / 0.12) ** 2 < 1)] = pal("light")
        c[z < 0.28] = pal("light")
        return c
    k.torso(T, color=torso_col, leg_r=0.06, cap=(0.03, 0.03))
    neck = Tube([(0, -0.15, 0.55), (0, -0.21, 0.62), (0, -0.23, 0.68)],
                [(0.13, 0.12, 0.13), (0.125, 0.115, 0.125), (0.12, 0.11, 0.12)], n=12,
                up=V((0, 0.5, 1)), lat=X, cap=(0.01, 0.01))

    def neck_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[v < 0.4] = pal("light")
        return c
    k.add(neck, lambda co: qr.chain(co.y, [(-0.27, "head"), (-0.21, "neck_1"), (-0.14, "chest")]),
          region="neck", color=neck_col)

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        ax = np.abs(x)
        c[(v < 0.42) & (u > 0.5)] = pal("white")
        c[u > 0.74] = pal("white")
        for E in EYES[:1]:
            d = ((ax - E.x) ** 2 + (y - E.y) ** 2) / 0.105 ** 2
            mask = (d + ((z - E.z + 0.008) / 0.064) ** 2 < 1) & (ax > 0.025) & (u > 0.4)
            brow = (d < 1) & (z > E.z + 0.056) & (z < E.z + 0.095) & (ax > 0.03) & (u > 0.4)
            c[brow] = pal("white")
            c[mask] = pal("mask")
        return c
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, color=head_col)
    EYES.append(head.point(EYE_U, math.radians(EYE_TH)))
    k.eyes(head, EYE_U, EYE_TH, (0.05, 0.057), depth=0.017)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.40, look=0.08)
    k.add(ellipsoid(HEAD_N + V((0, 0.012, 0.014)), FWD, Z, 0.024, 0.026, 0.04, 0.03, 0.026, rings=3, n=8),
          "head", cell="nose")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.07, 0.022, V((0.2, -1, 0.1)), "fur", "mask",
              prof=[(-0.2, 0.85, 0.9), (0.15, 1.0, 1.0), (0.5, 0.92, 0.9), (0.8, 0.62, 0.8),
                    (0.97, 0.25, 0.5)], inner_frac=0.55, inner_t=(0.08, 0.85))

    front = [(0.45, 0.07, 0.07, 0.066, 0.0), (0.33, 0.06, 0.058, 0.056, 0.0),
             (0.18, 0.05, 0.048, 0.048, 0.0), (0.06, 0.048, 0.046, 0.047, -0.005)]
    hind = [(0.47, 0.11, 0.10, 0.085, 0.02), (0.36, 0.085, 0.08, 0.07, 0.015),
            (0.20, 0.052, 0.05, 0.05, 0.0), (0.06, 0.048, 0.046, 0.047, -0.005)]

    def leg_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[z < 0.16] = pal("mask")
        return c

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.03, 0.036), FWD, Z, 0.045, 0.06, 0.055, 0.04, 0.036, rings=4, n=8)
    k.legs(front, hind, color=leg_col, paw=paw, paw_cell="mask", n=8)

    R = [0.05, 0.085, 0.10, 0.105, 0.095, 0.075, 0.045]
    tail = Tube(TAIL, [(r, r, r) for r in R], n=10, up=Y, lat=X, cap=(0.02, 0.04))

    def tail_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[(np.floor((u - 0.12) * 9) % 2 == 1) & (u > 0.2)] = pal("mask")
        c[u > 0.9] = pal("mask")
        return c
    k.add(tail, k.w_along(tail, [(0.03, "hips"), (0.12, "tail_1"), (0.33, "tail_1"), (0.48, "tail_2"),
                                 (0.62, "tail_2"), (0.76, "tail_3")]),
          region="tail", color=tail_col)


CLIPS = ["idle", "walk", "eat", "happy", "refuse", "sleep"]


def clips(k):
    cs = nk.QuadClips(k, {
        "sc": 0.5,
        "tail": ["tail_1", "tail_2", "tail_3"],
        "tail_amp": 9.0,
        "tail_lift": -2.0,
        "walk": dict(frames=16, stance=0.5, lift=0.06, toe_deg=30.0, fold_deg=55.0, toe_fwd=0.07,
                     dip=0.012),
        "eat": dict(nose_z=0.08, body=(0.03, 5.0), split=(0.62, 0.38, -0.40)),
        "happy": dict(hop=1.2),
        "head_look": 9.0,
        "sleep": dict(drop=0.32, curl=24.0, neck=30.0, head=24.0),
    })
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in cs.walk.drop))
    return cs.clips(CLIPS)


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    nk.run(kit, build, clips,
           dict(loco=("walk", 3), center_z=0.45, height_m=2.2, big_scale=1.5, close_scale=1.3,
                night_pose=("idle", 20)),
           debug=dict(center_z=0.45, head_pt=(0, -0.35, 0.68), head_scale=0.6, strip_scale=1.4))
