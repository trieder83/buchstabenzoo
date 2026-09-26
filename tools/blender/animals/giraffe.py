"""giraffe — Giraffe (ART-ANIMALS; quadruped rig, built with quad_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/giraffe.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/giraffe.blend, assets/models/animals/giraffe.glb,
assets/textures/animals/giraffe_body.png, art/animals/giraffe/model_preview.png.

Look: art/animals/giraffe/{front,side,back,three_quarter}.png. Tall warm-yellow giraffe with a
long neck (shortened for the camera; the standard neck_1/neck_2 joints are simply long) and
FEW, LARGE rounded brown patches (a jittered 3D cell pattern painted into the atlas, so it
stays broad at 60 px), brown mane ridge, cream muzzle and belly, two ossicones with dark
knobs, leaf-shaped ears, dark hooves, tail with a brown tuft. Top of the ossicones 4.5 m.
`eat` = nibbling leaves with the head high (brief).
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
ASSET = "giraffe"

COLORS = {
    "yellow": "#F2C75C",
    "brown": "#B8702E",
    "cream": "#F7E3B0",
    "dark": "#6B4424",
    "ear_in": "#E8B07A",
    "eye_white": "#FFFFFF",
    "iris": "#5A3418",
    "pupil": "#1E1410",
    "ink": "#2B1B12",
}

HEAD_B = V((0, -0.95, 4.02))
HEAD_D = V((0, -1.0, -0.55)).normalized()
HEAD_PROFILE = [
    (0.02, 0.10, 0.10, 0.10), (0.07, 0.17, 0.16, 0.15), (0.15, 0.20, 0.185, 0.17),
    (0.24, 0.19, 0.17, 0.155), (0.33, 0.155, 0.135, 0.13), (0.42, 0.14, 0.12, 0.125),
    (0.50, 0.145, 0.12, 0.13), (0.56, 0.12, 0.10, 0.11), (0.60, 0.07, 0.06, 0.065)]
HEAD_N = HEAD_B + HEAD_D * 0.62
NECK_PTS = [(0, -0.38, 2.10), (0, -0.50, 2.45), (0, -0.62, 2.85), (0, -0.73, 3.25), (0, -0.83, 3.62),
            (0, -0.90, 3.95)]
NECK_R = [0.30, 0.24, 0.20, 0.175, 0.155, 0.14]

P = {
    "hips": (0, 0.40, 1.95), "spine": (0, 0.0, 1.97), "chest": (0, -0.40, 2.02),
    "neck_1": (0, -0.48, 2.35), "neck_2": (0, -0.68, 3.05), "head": (0, -0.90, 3.90),
    "nose": tuple(HEAD_N),
    "ear_base": (0.13, -0.93, 4.10), "ear_tip": (0.34, -0.88, 4.16),
    "tail_1": (0, 0.64, 2.05), "tail_2": (0, 0.72, 1.70), "tail_end": (0, 0.76, 1.38),
    "front_xy": (0.21, -0.38), "front_z": (1.85, 0.95, 0.15),
    "hind_xy": (0.21, 0.40), "hind_z": (1.80, 0.92, 0.15),
    "toe": 0.08,
}


def _hash3(i, j, k, s):
    h = np.sin(i * 127.1 + j * 311.7 + k * 74.7 + s * 19.3) * 43758.5453
    return h - np.floor(h)


def patches(x, y, z, S=0.36, fill=0.40, gap=0.07):
    """Jittered 3D cell pattern: True inside a patch (x mirrored)."""
    p = np.stack([np.abs(x) + 0.13, y + 0.07, z], -1) / S
    base = np.floor(p)
    d1 = np.full(len(x), 9.0)
    d2 = np.full(len(x), 9.0)
    for di in (-1, 0, 1):
        for dj in (-1, 0, 1):
            for dk in (-1, 0, 1):
                c = base + np.array((di, dj, dk))
                j = np.stack([_hash3(c[:, 0], c[:, 1], c[:, 2], s) for s in range(3)], -1)
                seed = c + 0.2 + 0.6 * j
                d = np.linalg.norm(p - seed, axis=-1)
                d2 = np.where(d < d1, d1, np.minimum(d2, d))
                d1 = np.minimum(d1, d)
    return (d1 < fill * 1.5) & ((d2 - d1) * S > gap)


def build(k):
    pal = k.pal
    T = [(0.62, 1.95, 0.12, 0.11, 0.12), (0.55, 1.95, 0.24, 0.22, 0.24), (0.42, 1.955, 0.31, 0.27, 0.30),
         (0.22, 1.965, 0.33, 0.28, 0.315), (0.0, 1.98, 0.335, 0.285, 0.32), (-0.22, 2.0, 0.35, 0.30, 0.33),
         (-0.40, 2.02, 0.33, 0.29, 0.32), (-0.52, 2.035, 0.25, 0.22, 0.25), (-0.58, 2.04, 0.14, 0.12, 0.14)]

    def torso_col(x, y, z, u, v):
        c = pal.fill(len(x), "yellow")
        c[patches(x, y, z) & (z > 1.74)] = pal("brown")
        c[z < 1.70 + 0.04 * np.cos(y * 5)] = pal("cream")
        return c
    k.torso(T, color=torso_col, leg_r=0.10)

    neck = Tube(NECK_PTS, [(r, r * 0.95, r) for r in NECK_R], n=12, up=V((0, 1, 0.35)), lat=X,
                cap=(0.02, 0.02))

    def neck_col(x, y, z, u, v):
        c = pal.fill(len(x), "yellow")
        c[patches(x, y, z, S=0.30, fill=0.40, gap=0.06) & (u > 0.06) & (u < 0.93)] = pal("brown")
        c[(v < 0.18) & (u < 0.25)] = pal("cream")
        return c
    k.add(neck, k.w_along(neck, [(0.08, "chest"), (0.20, "neck_1"), (0.45, "neck_1"), (0.62, "neck_2"),
                                 (0.82, "neck_2"), (0.97, "head")]), region="neck", color=neck_col)
    # mane ridge along the back of the neck
    mp, mr = [], []
    for i, (p, r) in enumerate(zip(NECK_PTS, NECK_R)):
        if i == 0:
            continue
        up = neck.up[i]
        mp.append(V(p) + up * (r * 0.95 - 0.02))
        mr.append((0.035, 0.07, 0.03))
    mp.append(HEAD_B + V((0, 0.02, 0.16)))
    mr.append((0.03, 0.05, 0.03))
    mane = Tube(mp, mr, n=6, up=[neck.up[i] for i in range(1, len(NECK_PTS))] + [Z], lat=X, cap=(0.05, 0.04))
    k.add(mane, k.w_along(mane, [(0.05, "neck_1"), (0.35, "neck_1"), (0.55, "neck_2"), (0.8, "neck_2"),
                                 (0.95, "head")]), cell="brown")

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "yellow")
        c[u > 0.66] = pal("cream")
        return c
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, color=head_col)
    k.eyes(head, 0.36, 50.0, (0.05, 0.058), depth=0.025)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.40, look=0.10)
    for sg in (1, -1):
        # nostrils
        pt = head.point(0.86, sg * math.radians(30))
        nn = head.normal(0.86, sg * math.radians(30))
        k.add(ellipsoid(pt - nn * 0.006, nn, FWD, 0.012, 0.012, 0.025, 0.015, rings=3, n=6), "head", cell="dark")
        # ossicones
        b = V((0.07 * sg, -0.94, 4.12))
        t = V((0.085 * sg, -0.95, 4.36))
        k.add(Tube([b, (b + t) / 2, t], [(0.04, 0.04, 0.04), (0.035, 0.035, 0.035), (0.035, 0.035, 0.035)],
                   n=6, up=FWD, lat=X, cap=(0.0, 0.0)), "head", cell="yellow")
        k.add(ellipsoid(t + V((0, 0, 0.05)), Z, FWD, 0.055, 0.055, 0.06, 0.06, rings=4, n=8), "head", cell="dark")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.06, 0.022, V((0.2, -1, 0.3)), "yellow", "ear_in",
              prof=[(-0.1, 0.5, 0.9), (0.2, 0.9, 1.0), (0.5, 1.0, 0.9), (0.8, 0.65, 0.7), (0.97, 0.2, 0.5)],
              inner_frac=0.6)

    front = [(1.85, 0.14, 0.14, 0.13, 0.0), (1.45, 0.12, 0.115, 0.11, 0.0), (1.02, 0.10, 0.10, 0.098, 0.0),
             (0.60, 0.085, 0.082, 0.084, 0.0), (0.22, 0.078, 0.076, 0.077, 0.0), (0.15, 0.085, 0.08, 0.083, -0.005),
             (0.0, 0.092, 0.088, 0.09, -0.012)]
    hind = [(1.80, 0.18, 0.17, 0.14, 0.02), (1.45, 0.14, 0.13, 0.115, 0.02), (1.00, 0.105, 0.10, 0.10, 0.0),
            (0.60, 0.085, 0.082, 0.084, 0.0), (0.22, 0.078, 0.076, 0.077, 0.0), (0.15, 0.085, 0.08, 0.083, -0.005),
            (0.0, 0.092, 0.088, 0.09, -0.012)]

    def leg_col(x, y, z, u, v):
        c = pal.fill(len(x), "yellow")
        c[z < 0.15] = pal("dark")
        return c
    k.legs(front, hind, color=leg_col, n=8)

    tail = Tube([(0, 0.60, 2.08), (0, 0.67, 1.90), (0, 0.72, 1.68), (0, 0.75, 1.48)],
                [(0.04, 0.04, 0.04), (0.032, 0.032, 0.032), (0.028, 0.028, 0.028), (0.025, 0.025, 0.025)],
                n=6, up=Y, lat=X, cap=(0.02, 0.01))
    k.add(tail, k.w_along(tail, [(0.08, "hips"), (0.25, "tail_1"), (0.5, "tail_1"), (0.65, "tail_2")]),
          cell="yellow")
    k.add(ellipsoid((0, 0.765, 1.36), V((0, 0.1, -1)), Y, 0.05, 0.11, 0.055, 0.05, rings=4, n=8,
                    mod=lambda i, th: 1 + 0.15 * math.cos(4 * th)), "tail_2", cell="brown")


def clipset(k):
    return qk.ClipSet(k, {
        "sc": 1.7,
        "tail_amp": 16.0,
        "walk": dict(frames=28, stance=0.55, lift=0.13, toe_deg=30.0, fold_deg=50.0, toe_fwd=0.08,
                     dip=0.03, roll=1.5),
        "eat": dict(high=True),
        "happy": dict(hop=0.6),
        "head_look": 8.0,
    })


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    qk.run(kit, build, clipset, ["idle", "walk", "eat", "happy", "refuse"],
           dict(center_z=2.2, walk_frame=6, big_scale=5.0, close_scale=5.0,
                head_pt=(0, -1.15, 3.95), debug_scale=1.6))
