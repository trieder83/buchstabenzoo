"""porcupine — Stachelschwein (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/porcupine.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/porcupine.blend, assets/models/animals/porcupine.glb,
assets/textures/animals/porcupine_body.png, art/animals/porcupine/model_preview.png.

Look: art/animals/porcupine/{front,side,back,three_quarter}.png (sheet_v1). Chunky dark-brown
body on short legs, round friendly face with a black nose and small round ears, and a big
crown of long quills sweeping back from the head over the back — white with black bands
(chunky cones, not needles). Top of the quills 0.8 m, length ~1.0 m (Q-143).
Clips: idle, walk, eat, happy, refuse, sleep (lying down, quills flat). Eye caps: `eye_glow`.
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
ASSET = "porcupine"

COLORS = {
    "fur": "#4A3A30",
    "quill": "#F4F1E8",
    "band": "#262626",
    "face": "#54433A",
    "nose": "#2A2A2A",
    "feet": "#6A5446",
    "ear_in": "#9C6F62",
    "eye_white": "#FFFFFF",
    "iris": "#7A4A28",
    "pupil": "#1A1210",
    "ink": "#241A14",
}

HEAD_B = V((0, -0.17, 0.40))
HEAD_D = V((0, -1.0, -0.14)).normalized()
HEAD_PROFILE = [
    (0.0, 0.07, 0.07, 0.07), (0.05, 0.155, 0.145, 0.125), (0.10, 0.17, 0.155, 0.135),
    (0.15, 0.16, 0.14, 0.12), (0.20, 0.125, 0.105, 0.095), (0.24, 0.085, 0.075, 0.068),
    (0.27, 0.052, 0.048, 0.046)]
HEAD_N = HEAD_B + HEAD_D * 0.285

P = {
    "hips": (0, 0.16, 0.33), "spine": (0, 0.0, 0.34), "chest": (0, -0.14, 0.34),
    "neck_1": (0, -0.17, 0.36), "neck_2": (0, -0.20, 0.37), "head": (0, -0.23, 0.39),
    "nose": tuple(HEAD_N),
    "ear_base": (0.12, -0.21, 0.50), "ear_tip": (0.15, -0.20, 0.56),
    "tail_1": (0, 0.30, 0.30), "tail_2": (0, 0.33, 0.28), "tail_end": (0, 0.36, 0.26),
    "front_xy": (0.12, -0.14), "front_z": (0.26, 0.14, 0.045),
    "hind_xy": (0.13, 0.16), "hind_z": (0.27, 0.15, 0.045),
    "toe": 0.06,
}


def build(k):
    pal = k.pal
    at = k.atlas
    T = [(0.33, 0.32, 0.07, 0.07, 0.07), (0.28, 0.32, 0.18, 0.16, 0.17), (0.16, 0.32, 0.21, 0.18, 0.18),
         (0.03, 0.32, 0.21, 0.18, 0.18), (-0.08, 0.325, 0.20, 0.17, 0.17), (-0.16, 0.33, 0.17, 0.15, 0.15),
         (-0.21, 0.34, 0.10, 0.09, 0.09)]
    wt = k.w_torso(0.07, 0.4)
    torso = k.torso(T, cell="fur", leg_r=0.07, cap=(0.03, 0.03))

    # quills: long banded cones sweeping back (region 'extra': U = base -> tip)
    def quill_col(U, Vv):
        c = np.broadcast_to(pal("quill"), U.shape + (3,)).copy()
        c[U < 0.14] = pal("fur")
        c[(U > 0.40) & (U < 0.58)] = pal("band")
        return c
    k.paint_region("extra", quill_col)
    count = 0
    rows = [0.08, 0.2, 0.32, 0.44, 0.56, 0.68, 0.8]
    for ri, u in enumerate(rows):
        m = 9 if ri < 6 else 7
        span = 100.0 if ri < 6 else 80.0
        for j in range(m):
            th = math.radians(-span + 2 * span * j / (m - 1) + (7.0 if ri % 2 else 0.0))
            S = torso.point(u, th)
            if S.z < 0.28:
                continue
            nrm = torso.normal(u, th)
            d = (nrm * 0.8 + Y * 1.1 + Z * 0.35).normalized()
            mid = 1.0 - abs(math.degrees(th)) / 130.0
            L = (0.20 + 0.20 * mid * (1.0 - abs(u - 0.45))) * (0.95 + 0.1 * math.sin(2.3 * ri + 1.3 * j))
            c = nk.cone(S - nrm * 0.02, d, 0.046, L, n=5,
                        up=nrm.cross(X).normalized() if abs(nrm.x) < 0.9 else Z)

            def uv(i, kk, co, c=c):
                return at.map_uv("extra", c.U[i + c.off] if 0 <= i < c.R else 1.0, qr.ring_h(kk, c.n))
            k.mb.loft(c.rings, uv, wt, pole_start=None, pole_end=c.pole1)
            count += 1
    print("quills:", count)

    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, cell="face")
    # crest of quills on the head
    for j, (dx, L) in enumerate(((0.0, 0.26), (0.06, 0.23), (-0.06, 0.23), (0.11, 0.19), (-0.11, 0.19))):
        base = HEAD_B + HEAD_D * 0.06 + V((dx, 0, 0.12 - abs(dx) * 0.25))
        d = V((dx * 1.5, 0.9, 0.75)).normalized()
        c = nk.cone(base, d, 0.04, L, n=5, up=X)

        def uv(i, kk, co, c=c):
            return at.map_uv("extra", c.U[i + c.off] if 0 <= i < c.R else 1.0, qr.ring_h(kk, c.n))
        k.mb.loft(c.rings, uv, qr.rigid("head"), pole_start=None, pole_end=c.pole1)
    k.eyes(head, 0.52, 42.0, (0.045, 0.05), depth=0.017)
    k.eye_swatch(iris="iris", iris_r=0.68, pupil_r=0.40, look=0.08)
    k.add(ellipsoid(HEAD_N + V((0, 0.01, 0.012)), FWD, Z, 0.022, 0.024, 0.04, 0.03, 0.026, rings=3, n=8),
          "head", cell="nose")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.05, 0.018, V((0.3, -1, 0.1)), "face", "ear_in",
              prof=[(-0.3, 0.85, 0.9), (0.1, 1.0, 1.0), (0.5, 0.95, 0.9), (0.85, 0.6, 0.8),
                    (0.98, 0.25, 0.5)], inner_frac=0.6, inner_t=(0.05, 0.85))

    front = [(0.25, 0.07, 0.068, 0.066, 0.0), (0.17, 0.062, 0.06, 0.06, 0.0),
             (0.10, 0.056, 0.054, 0.055, 0.0), (0.045, 0.054, 0.052, 0.054, -0.004)]
    hind = [(0.26, 0.085, 0.08, 0.075, 0.01), (0.18, 0.07, 0.066, 0.064, 0.008),
            (0.10, 0.056, 0.054, 0.055, 0.0), (0.045, 0.054, 0.052, 0.054, -0.004)]

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.03, 0.03), FWD, Z, 0.045, 0.06, 0.058, 0.034, 0.03, rings=4, n=8)
    k.legs(front, hind, cell="fur", paw=paw, paw_cell="feet", n=8)


CLIPS = ["idle", "walk", "eat", "happy", "refuse", "sleep"]


def clips(k):
    cs = nk.QuadClips(k, {
        "sc": 0.3,
        "tail_amp": 4.0,
        "walk": dict(frames=12, stance=0.45, lift=0.045, toe_deg=26.0, fold_deg=48.0, toe_fwd=0.06,
                     dip=0.01),
        "eat": dict(nose_z=0.06, body=(0.02, 4.0), split=(0.55, 0.45, -0.1)),
        "happy": dict(hop=1.0),
        "head_look": 9.0,
        "sleep": dict(drop=0.17, curl=16.0, neck=24.0, head=18.0),
    })
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in cs.walk.drop))
    return cs.clips(CLIPS)


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    nk.run(kit, build, clips,
           dict(loco=("walk", 3), center_z=0.38, height_m=2.2, big_scale=1.4, close_scale=1.25,
                night_pose=("idle", 20)),
           debug=dict(center_z=0.35, head_pt=(0, -0.3, 0.42), head_scale=0.6, strip_scale=1.4))
