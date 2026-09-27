"""fennec — Wüstenfuchs / fennec fox (ART-ANIMALS "Night animals"; quadruped rig + tail_3,
quad_kit + night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/fennec.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/fennec.blend, assets/models/animals/fennec.glb,
assets/textures/animals/fennec_body.png, art/animals/fennec/model_preview.png.

Look: art/animals/fennec/{front,side,back,three_quarter}.png (sheet_v2). The smallest fox:
sand-beige slim body on thin legs, **huge pointed ears** (pink inside, half its height),
cream face, chest and belly, big friendly eyes, a bushy drooping tail with a dark-brown tip —
clearly different from the white snow fox. Top of the ears 0.9 m, back 0.4 m (Q-143).
Clips: idle, walk, eat, happy, refuse, sleep (curled up, tail round). Eye caps: `eye_glow`.
"""

import math
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import night_kit as nk  # noqa: E402
import quad_kit as qk  # noqa: E402
from quad_kit import FWD, X, Y, Z, Tube, ellipsoid  # noqa: E402,F401

qr = qk.qr
V = qk.V3
ASSET = "fennec"

COLORS = {
    "fur": "#EBC98E",
    "cream": "#FAF1DE",
    "pink": "#F3C4B8",
    "dark": "#3A2A20",
    "paw": "#DDB67A",
    "eye_white": "#FFFFFF",
    "iris": "#4A2E1E",
    "pupil": "#1A1210",
    "ink": "#2B1B12",
}

HEAD_B = V((0, -0.24, 0.555))
HEAD_D = V((0, -1.0, -0.22)).normalized()
HEAD_PROFILE = [(s * 1.12, a * 1.12, b * 1.12, c * 1.12) for s, a, b, c in [
    (0.01, 0.075, 0.075, 0.075), (0.05, 0.15, 0.14, 0.13), (0.10, 0.175, 0.16, 0.15),
    (0.15, 0.17, 0.15, 0.135), (0.19, 0.125, 0.10, 0.09), (0.22, 0.078, 0.064, 0.062),
    (0.255, 0.052, 0.045, 0.045), (0.28, 0.03, 0.028, 0.028)]]
HEAD_N = HEAD_B + HEAD_D * 0.325
TAIL = [(0, 0.23, 0.38), (0, 0.32, 0.375), (0, 0.41, 0.33), (0, 0.49, 0.26), (0, 0.55, 0.19),
        (0, 0.59, 0.14), (0, 0.61, 0.11)]

P = {
    "hips": (0, 0.16, 0.37), "spine": (0, 0.0, 0.37), "chest": (0, -0.16, 0.38),
    "neck_1": (0, -0.22, 0.43), "neck_2": (0, -0.25, 0.48), "head": (0, -0.27, 0.53),
    "nose": tuple(HEAD_N),
    "ear_base": (0.085, -0.28, 0.655), "ear_tip": (0.22, -0.26, 0.905),
    "tail_1": (0, 0.23, 0.38), "tail_2": (0, 0.40, 0.34), "tail_end": (0, 0.52, 0.22),
    "front_xy": (0.075, -0.16), "front_z": (0.34, 0.18, 0.05),
    "hind_xy": (0.08, 0.15), "hind_z": (0.35, 0.19, 0.05),
    "toe": 0.06,
    "extra_joints": [("tail_3", "tail_2", (0, 0.52, 0.22), (0, 0.61, 0.10))],
}


def build(k):
    pal = k.pal
    T = [(0.25, 0.37, 0.06, 0.06, 0.06), (0.21, 0.37, 0.115, 0.11, 0.115), (0.13, 0.365, 0.135, 0.125, 0.135),
         (0.02, 0.36, 0.13, 0.12, 0.13), (-0.08, 0.365, 0.135, 0.125, 0.14), (-0.17, 0.375, 0.14, 0.125, 0.15),
         (-0.23, 0.385, 0.12, 0.11, 0.125), (-0.27, 0.39, 0.065, 0.065, 0.065)]

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[(z < 0.285) | ((y < -0.17) & ((x / 0.11) ** 2 + ((z - 0.36) / 0.09) ** 2 < 1))] = pal("cream")
        return c
    k.torso(T, color=torso_col, leg_r=0.045, cap=(0.03, 0.03))
    neck = Tube([(0, -0.14, 0.43), (0, -0.21, 0.49), (0, -0.25, 0.55)],
                [(0.085, 0.085, 0.095), (0.08, 0.08, 0.09), (0.075, 0.075, 0.08)], n=10,
                up=V((0, 0.5, 1)), lat=X, cap=(0.01, 0.01))

    def neck_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[v < 0.45] = pal("cream")
        return c
    k.add(neck, lambda co: qr.chain(co.y, [(-0.29, "head"), (-0.22, "neck_1"), (-0.14, "chest")]),
          region="neck", color=neck_col)

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[(v < 0.42) & (u > 0.3)] = pal("cream")
        c[(u > 0.35) & (u < 0.62) & (v > 0.35) & (v < 0.55)] = pal("cream")
        return c
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, color=head_col)
    k.eyes(head, 0.50, 36.0, (0.048, 0.055), depth=0.016)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.38, look=0.10)
    for sg in (1, -1):
        k.add(ellipsoid(HEAD_B + HEAD_D * 0.12 + V((0.14 * sg, 0, -0.035)), V((0.5 * sg, 0.3, -0.2)), Z,
                        0.025, 0.035, 0.04, 0.035, 0.03, rings=3, n=8), "head", cell="cream")
    k.add(ellipsoid(HEAD_N + V((0, 0.01, 0.01)), FWD, Z, 0.018, 0.02, 0.026, 0.02, 0.018, rings=3, n=8),
          "head", cell="dark")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.12, 0.022, V((0.6, -1, 0.1)), "fur", "pink",
              prof=[(-0.12, 0.8, 0.9), (0.12, 1.0, 1.0), (0.4, 0.92, 0.9), (0.68, 0.64, 0.8),
                    (0.9, 0.3, 0.55), (0.98, 0.1, 0.4)], inner_frac=0.66, inner_t=(0.06, 0.9))

    front = [(0.33, 0.045, 0.045, 0.043, 0.0), (0.25, 0.038, 0.037, 0.036, 0.0),
             (0.14, 0.031, 0.03, 0.031, 0.0), (0.05, 0.032, 0.03, 0.031, -0.004)]
    hind = [(0.35, 0.075, 0.068, 0.058, 0.015), (0.27, 0.058, 0.055, 0.05, 0.012),
            (0.17, 0.034, 0.033, 0.033, 0.0), (0.05, 0.032, 0.03, 0.031, -0.004)]

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.025, 0.03), FWD, Z, 0.036, 0.05, 0.04, 0.034, 0.03, rings=4, n=8)
    k.legs(front, hind, cell="fur", paw=paw, paw_cell="paw", n=8)

    R = [0.035, 0.065, 0.085, 0.09, 0.08, 0.06, 0.03]
    tail = Tube(TAIL, [(r, r, r) for r in R], n=10, up=Y, lat=X, cap=(0.02, 0.04),
                mod=lambda i, th: 1 + 0.07 * math.cos(5 * th + i) * (1 if 1 <= i <= 5 else 0))

    def tail_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[u > 0.8] = pal("dark")
        return c
    k.add(tail, k.w_along(tail, [(0.03, "hips"), (0.12, "tail_1"), (0.33, "tail_1"), (0.48, "tail_2"),
                                 (0.62, "tail_2"), (0.76, "tail_3")]),
          region="tail", color=tail_col)


CLIPS = ["idle", "walk", "eat", "happy", "refuse", "sleep"]


def clips(k):
    cs = nk.QuadClips(k, {
        "sc": 0.38,
        "tail": ["tail_1", "tail_2", "tail_3"],
        "tail_amp": 10.0,
        "tail_lift": -2.0,
        "walk": dict(frames=12, stance=0.45, lift=0.05, toe_deg=30.0, fold_deg=55.0, toe_fwd=0.06,
                     dip=0.01),
        "eat": dict(nose_z=0.07, body=(0.03, 5.0), split=(0.62, 0.38, -0.40)),
        "happy": dict(hop=1.5),
        "head_look": 10.0,
        "sleep": dict(drop=0.24, curl=24.0, neck=34.0, head=26.0),
    })
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in cs.walk.drop))
    return cs.clips(CLIPS)


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    nk.run(kit, build, clips,
           dict(loco=("walk", 3), center_z=0.45, height_m=2.2, big_scale=1.5, close_scale=1.3,
                night_pose=("idle", 20)),
           debug=dict(center_z=0.4, head_pt=(0, -0.38, 0.6), head_scale=0.6, strip_scale=1.4))
