"""panda — Panda (ART-ANIMALS; quadruped rig, built with quad_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/panda.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/panda.blend, assets/models/animals/panda.glb,
assets/textures/animals/panda_body.png, art/animals/panda/model_preview.png.

Look: art/animals/panda/{front,side,back,three_quarter}.png. Round chunky teddy bear, warm
white body and big round head, black legs, black shoulder band, black round ears and big
black eye patches (the identifying features), small black nose, tiny white tail.
Top of the ears 1.1 m.
"""

import os
import sys

import numpy as np

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import quad_kit as qk  # noqa: E402
from quad_kit import FWD, X, Y, Z, Tube, ellipsoid  # noqa: E402,F401

qr = qk.qr
V = qk.V3
ASSET = "panda"

COLORS = {
    "white": "#F6F3EA",
    "black": "#262626",
    "pad": "#4A4A4A",
    "eye_white": "#FFFFFF",
    "iris": "#2A1C14",
    "pupil": "#141010",
    "ink": "#141010",
}

HEAD_B = V((0, -0.30, 0.78))
HEAD_D = V((0, -1.0, -0.18)).normalized()
HEAD_PROFILE = [
    (0.02, 0.12, 0.11, 0.11), (0.08, 0.225, 0.21, 0.20), (0.15, 0.265, 0.245, 0.225),
    (0.23, 0.26, 0.235, 0.21), (0.30, 0.215, 0.19, 0.17), (0.35, 0.15, 0.13, 0.125),
    (0.39, 0.10, 0.085, 0.085), (0.42, 0.06, 0.05, 0.05)]
HEAD_N = HEAD_B + HEAD_D * 0.44
EYE_U, EYE_TH = 0.56, 36.0

P = {
    "hips": (0, 0.26, 0.54), "spine": (0, 0.0, 0.54), "chest": (0, -0.22, 0.56),
    "neck_1": (0, -0.30, 0.62), "neck_2": (0, -0.32, 0.70), "head": (0, -0.34, 0.78),
    "nose": tuple(HEAD_N),
    "ear_base": (0.17, -0.40, 0.95), "ear_tip": (0.215, -0.40, 1.07),
    "tail_1": (0, 0.44, 0.60), "tail_2": (0, 0.50, 0.55), "tail_end": (0, 0.53, 0.50),
    "front_xy": (0.16, -0.22), "front_z": (0.50, 0.27, 0.07),
    "hind_xy": (0.17, 0.24), "hind_z": (0.52, 0.28, 0.07),
    "toe": 0.09,
}


def build(k):
    pal = k.pal
    T = [(0.46, 0.55, 0.10, 0.09, 0.10), (0.41, 0.55, 0.20, 0.18, 0.20), (0.31, 0.545, 0.25, 0.225, 0.25),
         (0.16, 0.54, 0.26, 0.225, 0.25), (0.0, 0.545, 0.26, 0.225, 0.25), (-0.15, 0.555, 0.27, 0.23, 0.26),
         (-0.28, 0.565, 0.25, 0.22, 0.24), (-0.37, 0.57, 0.19, 0.17, 0.18), (-0.42, 0.575, 0.10, 0.09, 0.10)]

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "white")
        band = (y > -0.34 + 0.25 * (z - 0.55)) & (y < -0.03 + 0.10 * (z - 0.55))
        haunch = (y > 0.12 + 0.35 * (z - 0.40)) & (z < 0.50)
        front = (y < -0.30) & (z < 0.42)
        c[band | haunch | front] = pal("black")
        return c
    k.torso(T, color=torso_col, leg_r=0.12)
    neck = Tube([(0, -0.24, 0.62), (0, -0.30, 0.70), (0, -0.33, 0.77)],
                [(0.20, 0.20, 0.21), (0.19, 0.19, 0.20), (0.17, 0.17, 0.17)], n=10, up=V((0, 0.4, 1)),
                lat=X, cap=(0.01, 0.01))
    # short neck: weights by front/back (the back of the neck stays on the chest, the throat
    # follows the head) so nodding does not lift a wedge behind the head
    k.add(neck, lambda co: qr.chain(co.y, [(-0.40, "head"), (-0.32, "neck_1"), (-0.22, "chest")]),
          cell="white")

    head_t = [None]

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "white")
        m = np.zeros(len(x), bool)
        h = head_t[0]
        for sg in (1,):
            e = h.point(EYE_U, np.radians(EYE_TH))
            n = h.normal(EYE_U, np.radians(EYE_TH))
            # tilted oval patch around the eye (drops down towards the nose)
            dx, dy, dz = np.abs(x) - e.x, y - e.y, z - e.z
            d = np.stack([dx, dy, dz], -1)
            nn = np.array(tuple(n))
            dd = d - (d @ nn)[:, None] * nn
            up = np.array((0, 0.25, 1.0))
            up = up - (up @ nn) * nn
            up /= np.linalg.norm(up)
            side = np.cross(nn, up)
            a, b = dd @ side, dd @ up
            ang = np.radians(-30)
            a2 = a * np.cos(ang) - b * np.sin(ang)
            b2 = a * np.sin(ang) + b * np.cos(ang)
            m |= (a2 / 0.10) ** 2 + ((b2 + 0.018) / 0.074) ** 2 < 1
        c[m] = pal("black")
        return c
    # build the tube first (the colour function needs its surface for the patch position)
    head = Tube([HEAD_B + HEAD_D * s for s, *_ in HEAD_PROFILE],
                [(a, b, c) for _, a, b, c in HEAD_PROFILE], n=14,
                up=(Z - HEAD_D * HEAD_D.dot(Z)), lat=X, cap=(0.03, 0.03))
    head_t[0] = head
    k.add(head, "head", region="head", color=head_col, name="head")
    k.eyes(head, EYE_U, EYE_TH, (0.045, 0.05), depth=0.02)
    k.eye_swatch(iris="iris", iris_r=0.70, pupil_r=0.45, look=0.08)
    k.add(ellipsoid(HEAD_N + V((0, 0.02, 0.045)), V((0, -1, 0.3)), Z, 0.025, 0.03, 0.05, 0.03, 0.028,
                    rings=4, n=10), "head", cell="black")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.085, 0.045, FWD, "black",
              prof=[(-0.3, 0.75, 0.9), (0.1, 1.0, 1.0), (0.5, 0.95, 0.95), (0.85, 0.6, 0.7)])

    front = [(0.50, 0.15, 0.14, 0.14, 0.0), (0.36, 0.13, 0.125, 0.125, 0.0), (0.22, 0.115, 0.11, 0.112, 0.0),
             (0.07, 0.115, 0.11, 0.112, -0.01)]
    hind = [(0.52, 0.17, 0.16, 0.15, 0.02), (0.38, 0.14, 0.13, 0.13, 0.015), (0.22, 0.115, 0.11, 0.112, 0.0),
            (0.07, 0.115, 0.11, 0.112, -0.01)]

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.04, 0.06), FWD, Z, 0.10, 0.12, 0.125, 0.07, 0.06, rings=5, n=10)
    k.legs(front, hind, cell="black", paw=paw, paw_cell="black")
    tail = ellipsoid((0, 0.47, 0.58), V((0, 1, -0.6)), Z, 0.05, 0.06, 0.07, 0.07, rings=4, n=8)
    k.add(tail, lambda co: qr.chain(co.y, [(0.44, "hips"), (0.47, "tail_1"), (0.52, "tail_2")]),
          cell="white")


def clipset(k):
    return qk.ClipSet(k, {
        "sc": 0.6,
        "tail_amp": 8.0,
        "walk": dict(frames=16, stance=0.5, lift=0.07, toe_deg=25.0, fold_deg=45.0, toe_fwd=0.09,
                     dip=0.015),
        "eat": dict(nose_z=0.14, body=(0.06, 8.0), split=(0.60, 0.40, -0.35)),
        "happy": dict(hop=0.9),
        "head_look": 8.0,
    })


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    qk.run(kit, build, clipset, ["idle", "walk", "eat", "happy", "refuse"],
           dict(center_z=0.55, walk_frame=4, big_scale=1.9, close_scale=1.7,
                head_pt=(0, -0.50, 0.80), debug_scale=0.6))
