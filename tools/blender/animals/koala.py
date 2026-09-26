"""koala — Koala (ART-ANIMALS; quadruped rig on all fours, built with quad_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/koala.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/koala.blend, assets/models/animals/koala.glb,
assets/textures/animals/koala_body.png, art/animals/koala/model_preview.png.

Look: art/animals/koala/{front,side,back,three_quarter}.png. Small round grey body standing on
all four paws (user decision: follows on foot), very big round head, huge fluffy round ears
with off-white insides and a big black oval nose (the identifying features), off-white chin
and chest, dark grey paws, no visible tail. Top of the ears 0.9 m (comic-scaled).
"""

import math
import os
import sys

import numpy as np  # noqa: F401

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import quad_kit as qk  # noqa: E402
from quad_kit import FWD, X, Y, Z, Tube, ellipsoid  # noqa: E402,F401

qr = qk.qr
V = qk.V3
ASSET = "koala"

COLORS = {
    "grey": "#9EA3A8",
    "white": "#F2F0EB",
    "nose": "#2A2A2A",
    "paw": "#6E7278",
    "eye_white": "#FFFFFF",
    "iris": "#3A2418",
    "pupil": "#161010",
    "ink": "#2B1B12",
}

HEAD_B = V((0, -0.14, 0.57))
HEAD_D = V((0, -1.0, -0.10)).normalized()
HEAD_PROFILE = [
    (0.02, 0.10, 0.09, 0.09), (0.07, 0.19, 0.175, 0.16), (0.13, 0.225, 0.205, 0.185),
    (0.20, 0.225, 0.20, 0.175), (0.26, 0.19, 0.17, 0.145), (0.30, 0.14, 0.125, 0.11),
    (0.325, 0.08, 0.075, 0.07)]
HEAD_N = HEAD_B + HEAD_D * 0.34

P = {
    "hips": (0, 0.15, 0.36), "spine": (0, 0.0, 0.36), "chest": (0, -0.12, 0.38),
    "neck_1": (0, -0.13, 0.44), "neck_2": (0, -0.14, 0.50), "head": (0, -0.16, 0.57),
    "nose": tuple(HEAD_N),
    "ear_base": (0.17, -0.25, 0.70), "ear_tip": (0.265, -0.23, 0.82),
    "tail_1": (0, 0.25, 0.36), "tail_2": (0, 0.27, 0.33), "tail_end": (0, 0.28, 0.30),
    "front_xy": (0.12, -0.11), "front_z": (0.32, 0.17, 0.05),
    "hind_xy": (0.125, 0.13), "hind_z": (0.33, 0.18, 0.05),
    "toe": 0.07,
}


def build(k):
    pal = k.pal
    T = [(0.27, 0.37, 0.07, 0.06, 0.07), (0.23, 0.37, 0.15, 0.14, 0.15), (0.15, 0.365, 0.195, 0.175, 0.19),
         (0.04, 0.36, 0.195, 0.17, 0.185), (-0.06, 0.365, 0.195, 0.175, 0.185), (-0.14, 0.375, 0.175, 0.165, 0.17),
         (-0.20, 0.38, 0.125, 0.12, 0.12), (-0.23, 0.385, 0.06, 0.06, 0.06)]

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "grey")
        chest = (y < -0.13) & (np.abs(x) < 0.11) & (z < 0.48)
        belly = (z < 0.22) & (np.abs(x) < 0.11) & (y < 0.12)
        c[chest | belly] = pal("white")
        return c
    k.torso(T, color=torso_col, leg_r=0.07)
    neck = Tube([(0, -0.10, 0.42), (0, -0.13, 0.50), (0, -0.15, 0.57)],
                [(0.15, 0.15, 0.16), (0.15, 0.15, 0.16), (0.14, 0.14, 0.14)], n=10, up=V((0, 0.3, 1)),
                lat=X, cap=(0.01, 0.01))
    # short neck: weights by front/back (the back of the neck stays on the chest, the throat
    # follows the head) so nodding does not lift a wedge behind the head
    k.add(neck, lambda co: qr.chain(co.y, [(-0.26, "head"), (-0.19, "neck_1"), (-0.10, "chest")]),
          cell="grey")

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "grey")
        c[(u > 0.55) & (v < 0.22)] = pal("white")  # chin / smile area
        return c
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, color=head_col)
    k.eyes(head, 0.60, 42.0, (0.04, 0.046), depth=0.018)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.40, look=0.08)
    # big black oval nose (taller than wide), on the front of the face
    k.add(ellipsoid(HEAD_N + V((0, 0.03, 0.01)), V((0, -1, 0.1)), Z, 0.04, 0.05, 0.062, 0.085, 0.085,
                    rings=5, n=12), "head", cell="nose")
    # huge fluffy round ears: grey outer ring, off-white fluffy inside
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.13, 0.055, V((0.6, -1, 0.0)), "grey", "white",
              prof=[(-0.35, 0.55, 0.9), (0.0, 0.92, 1.0), (0.35, 1.08, 1.0), (0.70, 0.95, 0.9),
                    (0.98, 0.50, 0.7)], inner_frac=0.52, inner_t=(-0.05, 0.85), n=10)

    front = [(0.32, 0.085, 0.08, 0.08, 0.0), (0.22, 0.075, 0.07, 0.072, 0.0), (0.12, 0.068, 0.065, 0.066, 0.0),
             (0.05, 0.068, 0.065, 0.066, -0.005)]
    hind = [(0.33, 0.11, 0.10, 0.095, 0.01), (0.23, 0.09, 0.085, 0.08, 0.01), (0.13, 0.068, 0.065, 0.066, 0.0),
            (0.05, 0.068, 0.065, 0.066, -0.005)]

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.03, 0.04), FWD, Z, 0.06, 0.08, 0.075, 0.045, 0.04, rings=4, n=10,
                         mod=lambda i, th: 1 + 0.06 * max(0.0, math.cos(th)) * math.cos(4 * th))
    k.legs(front, hind, cell="grey", paw=paw, paw_cell="paw")
    k.add(ellipsoid((0, 0.26, 0.35), V((0, 1, -0.4)), Z, 0.03, 0.03, 0.04, 0.035, rings=3, n=8),
          lambda co: qr.chain(co.y, [(0.24, "hips"), (0.27, "tail_1")]), cell="grey")


def ears_fn(p, back, out, fl, fr):
    # big round ears: flick = small wiggle outwards
    p.rel["ear_l"] = qk.ry(0.5 * out + 12 * fl) @ qk.rx(-0.5 * back - 10 * fl)
    p.rel["ear_r"] = qk.ry(-0.5 * out - 12 * fr) @ qk.rx(-0.5 * back - 10 * fr)


def clipset(k):
    return qk.ClipSet(k, {
        "sc": 0.42,
        "tail_amp": 3.0,
        "ears_fn": ears_fn,
        "walk": dict(frames=12, stance=0.45, lift=0.05, toe_deg=25.0, fold_deg=45.0, toe_fwd=0.07,
                     dip=0.012),
        "eat": dict(nose_z=0.10, body=(0.05, 8.0), split=(0.55, 0.45, -0.30)),
        "happy": dict(hop=1.3),
        "head_look": 9.0,
    })


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    qk.run(kit, build, clipset, ["idle", "walk", "eat", "happy", "refuse"],
           dict(center_z=0.45, walk_frame=3, big_scale=1.4, close_scale=1.25,
                head_pt=(0, -0.32, 0.62), debug_scale=0.45))
