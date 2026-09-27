"""slow_loris — Plumplori (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/slow_loris.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/slow_loris.blend, assets/models/animals/slow_loris.glb,
assets/textures/animals/slow_loris_body.png, art/animals/slow_loris/model_preview.png.

Look: art/animals/slow_loris/{front,side,back,three_quarter}.png (sheet_v2). Fluffy warm-tan
body on all fours, a big round head with **huge amber eyes in dark-brown eye patches**, a
cream stripe between the eyes, cream muzzle and chest, small round dark ears, a dark stripe
along the back, big pink hand-like paws, no visible tail. Top of the head 0.7 m (Q-143).
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
ASSET = "slow_loris"

COLORS = {
    "fur": "#C9A27A",
    "shade": "#B08A62",
    "patch": "#6E4A2E",
    "cream": "#F4EBDD",
    "pink": "#D6A08A",
    "nose": "#8A5A48",
    "eye_white": "#FFFFFF",
    "iris": "#E3A53A",
    "pupil": "#1A1210",
    "ink": "#2B1B12",
}

HEAD_B = V((0, -0.13, 0.50))
HEAD_D = V((0, -1.0, -0.05)).normalized()
HEAD_PROFILE = [
    (0.0, 0.10, 0.10, 0.10), (0.05, 0.19, 0.18, 0.17), (0.11, 0.21, 0.20, 0.19),
    (0.17, 0.20, 0.185, 0.17), (0.22, 0.16, 0.14, 0.13), (0.26, 0.10, 0.085, 0.08),
    (0.285, 0.06, 0.05, 0.05), (0.30, 0.035, 0.03, 0.03)]
HEAD_N = HEAD_B + HEAD_D * 0.31
EYE_U, EYE_TH = 0.70, 48.0

P = {
    "hips": (0, 0.17, 0.41), "spine": (0, 0.02, 0.42), "chest": (0, -0.13, 0.43),
    "neck_1": (0, -0.16, 0.46), "neck_2": (0, -0.18, 0.48), "head": (0, -0.20, 0.50),
    "nose": tuple(HEAD_N),
    "ear_base": (0.165, -0.12, 0.60), "ear_tip": (0.195, -0.11, 0.66),
    "tail_1": (0, 0.28, 0.40), "tail_2": (0, 0.31, 0.37), "tail_end": (0, 0.33, 0.35),
    "front_xy": (0.12, -0.13), "front_z": (0.38, 0.20, 0.05),
    "hind_xy": (0.125, 0.17), "hind_z": (0.39, 0.21, 0.05),
    "toe": 0.07,
}
EYES = []


def build(k):
    pal = k.pal
    T = [(0.31, 0.40, 0.07, 0.07, 0.07), (0.26, 0.40, 0.16, 0.15, 0.17), (0.15, 0.40, 0.18, 0.16, 0.19),
         (0.03, 0.40, 0.18, 0.155, 0.19), (-0.08, 0.41, 0.18, 0.155, 0.19), (-0.16, 0.42, 0.17, 0.15, 0.18),
         (-0.22, 0.43, 0.13, 0.12, 0.14), (-0.25, 0.44, 0.07, 0.07, 0.07)]

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        w = 0.05 * np.clip((y + 0.25) / 0.5, 0.2, 1.0)
        c[(np.abs(x) < w) & (z > 0.5)] = pal("patch")
        c[(y < -0.14) & ((x / 0.11) ** 2 + ((z - 0.38) / 0.1) ** 2 < 1)] = pal("cream")
        return c
    k.torso(T, color=torso_col, leg_r=0.06, cap=(0.03, 0.03))

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        ax = np.abs(x)
        c[(v < 0.38) & (u > 0.6)] = pal("cream")
        c[u > 0.86] = pal("cream")
        for E in EYES[:1]:
            d = ((ax - E.x) ** 2 + (y - E.y) ** 2 + (z - E.z) ** 2) / 0.085 ** 2
            c[(d < 1) & (u > 0.35)] = pal("patch")
        c[(ax < 0.028) & (u > 0.45) & (v > 0.45)] = pal("cream")
        return c
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, color=head_col)
    EYES.append(head.point(EYE_U, math.radians(EYE_TH)))
    k.eyes(head, EYE_U, EYE_TH, (0.058, 0.062), depth=0.02)
    k.eye_swatch(iris="iris", iris_r=0.74, pupil_r=0.40, look=0.06)
    k.add(ellipsoid(HEAD_N + V((0, 0.01, 0.01)), FWD, Z, 0.018, 0.02, 0.03, 0.022, 0.02, rings=3, n=8),
          "head", cell="nose")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.05, 0.02, V((0.5, -1, 0.1)), "patch", "shade",
              prof=[(-0.3, 0.85, 0.9), (0.1, 1.0, 1.0), (0.5, 0.95, 0.9), (0.85, 0.6, 0.8),
                    (0.98, 0.25, 0.5)], inner_frac=0.55, inner_t=(0.05, 0.8))

    front = [(0.37, 0.07, 0.068, 0.066, 0.0), (0.27, 0.06, 0.058, 0.057, 0.0),
             (0.15, 0.05, 0.048, 0.05, 0.0), (0.06, 0.046, 0.045, 0.047, -0.004)]
    hind = [(0.39, 0.10, 0.09, 0.08, 0.015), (0.29, 0.08, 0.075, 0.068, 0.01),
            (0.16, 0.052, 0.05, 0.05, 0.0), (0.06, 0.046, 0.045, 0.047, -0.004)]

    def paw(leg, h):
        return ellipsoid((h.x * 1.06, h.y - 0.035, 0.028), FWD, Z, 0.045, 0.075, 0.06, 0.03, 0.028,
                         rings=4, n=8)
    k.legs(front, hind, cell="fur", paw=paw, paw_cell="pink", n=8)
    k.add(ellipsoid((0, 0.30, 0.38), V((0, 1, -0.4)), Z, 0.02, 0.04, 0.04, 0.035, 0.035, rings=3, n=8),
          "tail_1", cell="fur")


CLIPS = ["idle", "walk", "eat", "happy", "refuse", "sleep"]


def clips(k):
    cs = nk.QuadClips(k, {
        "sc": 0.42,
        "tail_amp": 3.0,
        "walk": dict(frames=16, stance=0.5, lift=0.05, toe_deg=26.0, fold_deg=50.0, toe_fwd=0.07,
                     dip=0.012),
        "eat": dict(nose_z=0.08, body=(0.03, 5.0), split=(0.55, 0.45, -0.1)),
        "happy": dict(hop=1.0),
        "head_look": 12.0,
        "sleep": dict(drop=0.26, curl=22.0, neck=30.0, head=26.0),
    })
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in cs.walk.drop))
    return cs.clips(CLIPS)


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    nk.run(kit, build, clips,
           dict(loco=("walk", 3), center_z=0.38, height_m=2.2, big_scale=1.35, close_scale=1.2,
                night_pose=("idle", 20)),
           debug=dict(center_z=0.35, head_pt=(0, -0.3, 0.5), head_scale=0.6, strip_scale=1.3))
