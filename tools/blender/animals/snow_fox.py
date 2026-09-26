"""snow_fox — Schneefuchs (ART-ANIMALS; quadruped rig + tail_3, built with quad_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/snow_fox.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/snow_fox.blend, assets/models/animals/snow_fox.glb,
assets/textures/animals/snow_fox_body.png, art/animals/snow_fox/model_preview.png.

Look: art/animals/snow_fox/{front,side,back,three_quarter}.png. Small snow-white fox, big
head with big pointed ears (pink inside), fluffy chest ruff, a huge fluffy tail almost as big
as the body raised in a plume (3 tail joints: extra `tail_3`), pale blue-grey lower legs and
belly so it does not vanish on light paths. Top of the ears 0.9 m (comic-scaled).
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
ASSET = "snow_fox"

COLORS = {
    "white": "#FAFAF7",
    "shade": "#DCE5EE",
    "pink": "#F1B8C0",
    "nose": "#2A2A2A",
    "eye_white": "#FFFFFF",
    "iris": "#3A2A20",
    "pupil": "#1A1210",
    "ink": "#2B1B12",
}

HEAD_B = V((0, -0.25, 0.60))
HEAD_D = V((0, -1.0, -0.22)).normalized()
HEAD_PROFILE = [
    (0.01, 0.08, 0.08, 0.08), (0.05, 0.16, 0.15, 0.14), (0.10, 0.19, 0.175, 0.16),
    (0.155, 0.185, 0.165, 0.145), (0.20, 0.14, 0.115, 0.10), (0.23, 0.085, 0.07, 0.07),
    (0.27, 0.06, 0.05, 0.052), (0.30, 0.035, 0.032, 0.032)]
HEAD_N = HEAD_B + HEAD_D * 0.315
TAIL = [(0, 0.24, 0.44), (0, 0.33, 0.47), (0, 0.43, 0.52), (0, 0.52, 0.59), (0, 0.58, 0.67),
        (0, 0.60, 0.75), (0, 0.58, 0.81)]

P = {
    "hips": (0, 0.14, 0.40), "spine": (0, 0.0, 0.40), "chest": (0, -0.15, 0.41),
    "neck_1": (0, -0.22, 0.47), "neck_2": (0, -0.25, 0.53), "head": (0, -0.27, 0.58),
    "nose": tuple(HEAD_N),
    "ear_base": (0.08, -0.30, 0.72), "ear_tip": (0.125, -0.28, 0.90),
    "tail_1": (0, 0.24, 0.44), "tail_2": (0, 0.40, 0.50), "tail_end": (0, 0.47, 0.99),
    "front_xy": (0.085, -0.15), "front_z": (0.36, 0.19, 0.05),
    "hind_xy": (0.09, 0.14), "hind_z": (0.37, 0.20, 0.055),
    "toe": 0.07,
    "extra_joints": [("tail_3", "tail_2", (0, 0.55, 0.62), (0, 0.59, 0.82))],
}
# tail_2 must end where tail_3 starts
P["tail_end"] = (0, 0.55, 0.62)


def build(k):
    pal = k.pal
    T = [(0.27, 0.39, 0.07, 0.07, 0.07), (0.23, 0.39, 0.13, 0.125, 0.13), (0.15, 0.385, 0.16, 0.15, 0.16),
         (0.04, 0.38, 0.155, 0.145, 0.155), (-0.06, 0.385, 0.16, 0.15, 0.165), (-0.15, 0.395, 0.165, 0.15, 0.17),
         (-0.22, 0.405, 0.14, 0.135, 0.145), (-0.26, 0.41, 0.08, 0.08, 0.08)]
    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "white")
        c[z < 0.275] = pal("shade")
        return c
    k.torso(T, color=torso_col, leg_r=0.05, cap=(0.03, 0.03))
    # chest ruff (scalloped)
    ruff = ellipsoid((0, -0.23, 0.44), V((0, -1, -0.35)), Z, 0.08, 0.09, 0.14, 0.13, 0.15, rings=5, n=14,
                     lat=X, mod=lambda i, th: 1 + 0.10 * max(0.0, -math.cos(th)) * math.cos(7 * th))
    k.add(ruff, lambda co: qr.chain(co.z, [(0.40, "chest"), (0.52, "neck_1")]), cell="white")
    neck = Tube([(0, -0.14, 0.45), (0, -0.21, 0.51), (0, -0.25, 0.57)],
                [(0.10, 0.10, 0.11), (0.095, 0.095, 0.10), (0.09, 0.09, 0.09)], n=10, up=V((0, 0.5, 1)),
                lat=X, cap=(0.01, 0.01))
    # short neck: weights by front/back (the back of the neck stays on the chest, the throat
    # follows the head) so nodding does not lift a wedge behind the head
    k.add(neck, lambda co: qr.chain(co.y, [(-0.30, "head"), (-0.23, "neck_1"), (-0.15, "chest")]),
          cell="white")

    def head_col(x, y, z, u, v):
        return pal.fill(len(x), "white")
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, color=head_col)
    k.eyes(head, 0.50, 33.0, (0.05, 0.058), depth=0.018)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.36, look=0.10)
    # cheek fluff
    for sg in (1, -1):
        k.add(ellipsoid(HEAD_B + HEAD_D * 0.13 + V((0.12 * sg, 0, -0.06)), V((0.4 * sg, 0.3, -0.3)), Z,
                        0.04, 0.06, 0.07, 0.06, 0.05, rings=4, n=8), "head", cell="white")
    k.add(ellipsoid(HEAD_N + V((0, 0.012, 0.012)), FWD, Z, 0.02, 0.022, 0.03, 0.022, 0.02, rings=3, n=8),
          "head", cell="nose")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.075, 0.022, V((0.15, -1, 0.1)), "white", "pink",
              prof=[(-0.15, 0.85, 0.9), (0.15, 1.0, 1.0), (0.45, 0.78, 0.9), (0.72, 0.48, 0.75),
                    (0.95, 0.12, 0.5)], inner_frac=0.62, inner_t=(0.05, 0.85))

    front = [(0.35, 0.058, 0.058, 0.056, 0.0), (0.26, 0.05, 0.048, 0.048, 0.0),
             (0.14, 0.042, 0.04, 0.041, 0.0), (0.05, 0.042, 0.04, 0.041, -0.005)]
    hind = [(0.37, 0.09, 0.08, 0.07, 0.015), (0.29, 0.07, 0.065, 0.06, 0.015),
            (0.17, 0.044, 0.042, 0.042, 0.0), (0.05, 0.042, 0.04, 0.041, -0.005)]

    def leg_col(x, y, z, u, v):
        c = pal.fill(len(x), "white")
        c[z < 0.20] = pal("shade")
        return c

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.03, 0.036), FWD, Z, 0.045, 0.06, 0.05, 0.04, 0.036, rings=4, n=8)
    k.legs(front, hind, color=leg_col, paw=paw, paw_cell="shade", n=8)

    # huge fluffy tail (plume), scalloped
    R = [0.05, 0.085, 0.115, 0.13, 0.125, 0.10, 0.06]
    tail = Tube(TAIL, [(r, r, r) for r in R], n=10, up=Y, lat=X, cap=(0.02, 0.05),
                mod=lambda i, th: 1 + 0.08 * math.cos(5 * th + i) * (1 if 1 <= i <= 5 else 0))

    def tail_col(x, y, z, u, v):
        c = pal.fill(len(x), "white")
        c[(v < 0.3) & (u < 0.55)] = pal("shade")
        return c
    k.add(tail, k.w_along(tail, [(0.03, "hips"), (0.12, "tail_1"), (0.35, "tail_1"), (0.5, "tail_2"),
                                 (0.62, "tail_2"), (0.75, "tail_3")]),
          region="tail", color=tail_col)


def clipset(k):
    return qk.ClipSet(k, {
        "sc": 0.4,
        "tail": ["tail_1", "tail_2", "tail_3"],
        "tail_amp": 10.0,
        "walk": dict(frames=12, stance=0.45, lift=0.05, toe_deg=30.0, fold_deg=55.0, toe_fwd=0.07,
                     dip=0.01),
        "eat": dict(nose_z=0.07, body=(0.03, 5.0), split=(0.62, 0.38, -0.40)),
        "drink": dict(nose_z=0.05, body=(0.04, 7.0), split=(0.62, 0.38, -0.40)),
        "happy": dict(hop=1.4),
        "head_look": 9.0,
    })


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    qk.run(kit, build, clipset, ["idle", "walk", "eat", "drink", "happy", "refuse"],
           dict(center_z=0.45, walk_frame=3, big_scale=1.6, close_scale=1.3,
                head_pt=(0, -0.40, 0.66), debug_scale=0.45))
