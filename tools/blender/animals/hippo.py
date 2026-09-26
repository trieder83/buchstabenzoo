"""hippo — Flusspferd (ART-ANIMALS; quadruped rig, built with quad_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/hippo.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/hippo.blend, assets/models/animals/hippo.glb,
assets/textures/animals/hippo_body.png, art/animals/hippo/model_preview.png.

Look: art/animals/hippo/{front,side,back,three_quarter}.png. Huge round lilac-grey barrel
body on very short stumpy legs, big head with a wide square pink muzzle (the identifying
feature), big eyes set high on the head, tiny round ears on top, pink belly, cream toenails,
short tail with a dark tuft. Top of the ears 1.7 m. `swim`: floating pose (legs paddle
under the body, head up); the game sinks the model to its water line (about 0.9 m, so
back, eyes and ears stay above the water).
"""

import math
import os
import sys

import numpy as np

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import quad_kit as qk  # noqa: E402
from quad_kit import FWD, X, Y, Z, Tube, ellipsoid  # noqa: E402,F401

qr = qk.qr
V = qk.V3
ASSET = "hippo"

COLORS = {
    "body": "#9C8FB0",
    "pink": "#E9A6AE",
    "dpink": "#C97A86",
    "nail": "#EDE3D2",
    "tuft": "#5E5470",
    "eye_white": "#FFFFFF",
    "iris": "#2A1C1C",
    "pupil": "#141010",
    "ink": "#2B1B12",
}

HEAD_B = V((0, -0.60, 1.40))
HEAD_D = V((0, -1.0, -0.42)).normalized()
HEAD_PROFILE = [  # skull (narrower) then the big wide muzzle
    (0.02, 0.16, 0.15, 0.15), (0.10, 0.29, 0.27, 0.26), (0.20, 0.32, 0.28, 0.28),
    (0.30, 0.30, 0.24, 0.27), (0.38, 0.30, 0.215, 0.27), (0.47, 0.35, 0.23, 0.29),
    (0.56, 0.38, 0.24, 0.30), (0.64, 0.37, 0.23, 0.28), (0.70, 0.31, 0.19, 0.23),
    (0.74, 0.20, 0.12, 0.15)]
HEAD_N = HEAD_B + HEAD_D * 0.77
EYE_U, EYE_TH = 0.30, 26.0

P = {
    "hips": (0, 0.40, 0.86), "spine": (0, 0.0, 0.86), "chest": (0, -0.38, 0.90),
    "neck_1": (0, -0.50, 1.02), "neck_2": (0, -0.56, 1.18), "head": (0, -0.62, 1.32),
    "nose": tuple(HEAD_N),
    "ear_base": (0.16, -0.76, 1.55), "ear_tip": (0.19, -0.75, 1.66),
    "tail_1": (0, 0.74, 1.00), "tail_2": (0, 0.80, 0.86), "tail_end": (0, 0.83, 0.72),
    "front_xy": (0.27, -0.36), "front_z": (0.62, 0.32, 0.09),
    "hind_xy": (0.28, 0.40), "hind_z": (0.64, 0.33, 0.09),
    "toe": 0.10,
}


def build(k):
    pal = k.pal
    T = [(0.78, 0.87, 0.16, 0.15, 0.16), (0.70, 0.87, 0.33, 0.30, 0.33), (0.56, 0.865, 0.44, 0.40, 0.44),
         (0.34, 0.86, 0.49, 0.43, 0.48), (0.08, 0.86, 0.50, 0.44, 0.49), (-0.18, 0.865, 0.50, 0.44, 0.49),
         (-0.42, 0.875, 0.46, 0.41, 0.45), (-0.60, 0.885, 0.36, 0.33, 0.35), (-0.70, 0.89, 0.20, 0.19, 0.20)]

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "body")
        belly = z < 0.50 + 0.03 * np.cos(y * 4)
        chest = (y < -0.45) & (z < 0.98) & (np.abs(x) < 0.26)
        c[belly | chest] = pal("pink")
        return c
    k.torso(T, color=torso_col, leg_r=0.17)
    neck = Tube([(0, -0.46, 1.00), (0, -0.55, 1.17), (0, -0.62, 1.32)],
                [(0.34, 0.33, 0.35), (0.31, 0.30, 0.32), (0.27, 0.26, 0.27)], n=12, up=V((0, 0.4, 1)),
                lat=X, cap=(0.02, 0.02))

    def neck_col(x, y, z, u, v):
        c = pal.fill(len(x), "body")
        c[v < 0.30] = pal("pink")
        return c
    # short neck: weights by front/back (the back of the neck stays on the chest, the throat
    # follows the head) so nodding does not lift a wedge behind the head
    k.add(neck, lambda co: qr.chain(co.y, [(-0.85, "head"), (-0.68, "neck_1"), (-0.50, "chest")]),
          region="neck", color=neck_col)

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "body")
        muzzle = u > 0.50 + 0.06 * (v - 0.5)
        c[muzzle] = pal("pink")
        return c
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, color=head_col)
    # eye bumps high on the skull, eyes on them
    for sg in (1, -1):
        e = head.point(EYE_U, sg * math.radians(EYE_TH))
        n = head.normal(EYE_U, sg * math.radians(EYE_TH))
        k.add(ellipsoid(e - n * 0.02, n, Z, 0.06, 0.06, 0.10, 0.10, 0.09, rings=4, n=10), "head",
              cell="body")
    k.eyes(head, EYE_U, EYE_TH + 3, (0.075, 0.085), depth=0.06)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.44, look=0.10)
    # nostrils on the top front of the muzzle
    for sg in (1, -1):
        pt = head.point(0.84, sg * math.radians(22))
        nn = head.normal(0.84, sg * math.radians(22))
        k.add(ellipsoid(pt - nn * 0.012, nn, V((0, -1, 0)), 0.02, 0.02, 0.045, 0.032, rings=3, n=8),
              "head", cell="dpink")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.065, 0.03, FWD, "body", "dpink",
              prof=[(-0.3, 0.8, 0.9), (0.1, 1.0, 1.0), (0.5, 0.95, 0.9), (0.85, 0.6, 0.7)],
              inner_frac=0.6)

    front = [(0.60, 0.19, 0.18, 0.18, 0.0), (0.42, 0.175, 0.165, 0.165, 0.0), (0.24, 0.16, 0.155, 0.155, 0.0),
             (0.10, 0.165, 0.16, 0.16, -0.005), (0.0, 0.17, 0.165, 0.165, -0.01)]
    hind = [(0.64, 0.23, 0.21, 0.20, 0.02), (0.46, 0.20, 0.185, 0.18, 0.015), (0.26, 0.165, 0.16, 0.16, 0.0),
            (0.10, 0.165, 0.16, 0.16, -0.005), (0.0, 0.17, 0.165, 0.165, -0.01)]
    fx = k.rig.rest_head["front_upper_l"]
    hx = k.rig.rest_head["hind_upper_l"]

    def leg_col(x, y, z, u, v):
        c = pal.fill(len(x), "body")
        cy = np.where(y < 0, fx.y, hx.y)
        ang = np.degrees(np.arctan2(x - fx.x, -(y - cy)))
        nail = (z < 0.075) & (np.abs(((ang + 27) % 36) - 18) < 11) & (np.abs(ang) < 50)
        c[nail] = pal("nail")
        return c
    k.legs(front, hind, color=leg_col, n=10)
    tail = Tube([(0, 0.72, 1.02), (0, 0.78, 0.90), (0, 0.82, 0.78)],
                [(0.04, 0.04, 0.04), (0.035, 0.035, 0.035), (0.03, 0.03, 0.03)], n=6, up=Y, lat=X,
                cap=(0.02, 0.01))
    k.add(tail, k.w_along(tail, [(0.1, "hips"), (0.3, "tail_1"), (0.6, "tail_2")]), cell="body")
    k.add(ellipsoid((0, 0.835, 0.72), V((0, 0.2, -1)), Y, 0.04, 0.06, 0.045, 0.04, rings=4, n=8,
                    mod=lambda i, th: 1 + 0.15 * math.cos(4 * th)), "tail_2", cell="tuft")


def clipset(k):
    return qk.ClipSet(k, {
        "sc": 0.95,
        "tail_amp": 16.0,
        "walk": dict(frames=20, stance=0.5, lift=0.07, toe_deg=18.0, fold_deg=35.0, toe_fwd=0.10,
                     dip=0.02, roll=2.5),
        "eat": dict(nose_z=0.30, body=(0.05, 6.0), split=(0.55, 0.45, -0.20)),
        "happy": dict(hop=0.55, fold=25.0),
        "head_look": 6.0,
    })


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    qk.run(kit, build, clipset, ["idle", "walk", "eat", "swim", "happy", "refuse"],
           dict(center_z=0.85, walk_frame=4, big_scale=3.0, close_scale=2.6,
                head_pt=(0, -0.95, 1.20), debug_scale=1.1))
