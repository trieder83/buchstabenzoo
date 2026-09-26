"""elephant — Elefant (ART-ANIMALS; quadruped rig + trunk_1..3, built with quad_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/elephant.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/elephant.blend, assets/models/animals/elephant.glb,
assets/textures/animals/elephant_body.png, art/animals/elephant/model_preview.png.

Look: art/animals/elephant/{front,side,back,three_quarter}.png. Big round blue-grey body on
thick pillar legs with cream toenails, big domed head with huge round flappy ears (pink
inside) and a thick trunk that hangs down and curls forward at the tip (3 extra joints
trunk_1..3 under `head`), no tusks, thin tail with a dark tuft. Top of the head 3.0 m.
"""

import math
import os
import sys

import numpy as np

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import quad_kit as qk  # noqa: E402
from quad_kit import FWD, TAU, X, Y, Z, Tube, ellipsoid, env, rx, ry, rz  # noqa: E402,F401

qr = qk.qr
V = qk.V3
ASSET = "elephant"

COLORS = {
    "body": "#9AA3AE",
    "pink": "#E7A9B0",
    "nail": "#EDE3D2",
    "tuft": "#4A4F57",
    "eye_white": "#FFFFFF",
    "iris": "#2A1C14",
    "pupil": "#141010",
    "ink": "#2B1B12",
}

HEAD_B = V((0, -0.80, 2.66))
HEAD_D = V((0, -1.0, -0.5)).normalized()
HEAD_PROFILE = [
    (0.03, 0.26, 0.26, 0.26), (0.14, 0.47, 0.46, 0.44), (0.30, 0.53, 0.50, 0.46),
    (0.46, 0.49, 0.45, 0.40), (0.60, 0.40, 0.34, 0.33), (0.72, 0.29, 0.25, 0.26),
    (0.80, 0.21, 0.19, 0.21)]
HEAD_N = HEAD_B + HEAD_D * 0.83
TRUNK = [(0, -1.44, 2.32), (0, -1.60, 2.10), (0, -1.69, 1.80), (0, -1.72, 1.45), (0, -1.70, 1.10),
         (0, -1.68, 0.82), (0, -1.73, 0.60), (0, -1.86, 0.50), (0, -1.97, 0.57)]
TRUNK_R = [0.20, 0.185, 0.165, 0.145, 0.125, 0.11, 0.10, 0.09, 0.08]
EAR_C = V((0.72, -0.76, 2.40))
EAR_N = V((1.0, -1.0, 0.0)).normalized()

P = {
    "hips": (0, 0.55, 1.75), "spine": (0, 0.0, 1.75), "chest": (0, -0.50, 1.80),
    "neck_1": (0, -0.70, 1.98), "neck_2": (0, -0.80, 2.18), "head": (0, -0.86, 2.40),
    "nose": tuple(HEAD_N),
    "ear_base": (0.42, -0.90, 2.55), "ear_tip": (1.00, -0.80, 2.35),
    "tail_1": (0, 0.93, 1.95), "tail_2": (0, 1.02, 1.62), "tail_end": (0, 1.06, 1.30),
    "front_xy": (0.40, -0.55), "front_z": (1.40, 0.75, 0.14),
    "hind_xy": (0.42, 0.58), "hind_z": (1.45, 0.78, 0.14),
    "toe": 0.12,
    "extra_joints": [("trunk_1", "head", (0, -1.60, 2.10), (0, -1.72, 1.45)),
                     ("trunk_2", "trunk_1", (0, -1.72, 1.45), (0, -1.69, 0.85)),
                     ("trunk_3", "trunk_2", (0, -1.69, 0.85), (0, -1.95, 0.56))],
}


def build(k):
    pal = k.pal
    T = [(1.00, 1.74, 0.22, 0.20, 0.22), (0.90, 1.74, 0.46, 0.40, 0.44), (0.72, 1.735, 0.63, 0.53, 0.58),
         (0.44, 1.73, 0.70, 0.575, 0.62), (0.10, 1.73, 0.71, 0.58, 0.62), (-0.24, 1.74, 0.71, 0.58, 0.63),
         (-0.54, 1.76, 0.66, 0.55, 0.60), (-0.78, 1.78, 0.50, 0.44, 0.47), (-0.90, 1.79, 0.28, 0.25, 0.27)]
    k.torso(T, cell="body", leg_r=0.26)
    neck = Tube([(0, -0.62, 1.95), (0, -0.76, 2.18), (0, -0.86, 2.40)],
                [(0.52, 0.50, 0.52), (0.48, 0.46, 0.48), (0.42, 0.40, 0.42)], n=12, up=V((0, 0.4, 1)),
                lat=X, cap=(0.02, 0.02))
    k.add(neck, k.w_along(neck, [(0.1, "chest"), (0.4, "neck_1"), (0.7, "neck_2"), (0.95, "head")]),
          cell="body")
    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE, cell="body")
    k.eyes(head, 0.56, 50.0, (0.09, 0.11), depth=0.045)
    k.eye_swatch(iris="iris", iris_r=0.64, pupil_r=0.42, look=0.10)

    trunk = Tube(TRUNK, [(r, r, r) for r in TRUNK_R], n=10, up=FWD, lat=X, cap=(0.0, 0.05))
    k.add(trunk, k.w_along(trunk, [(0.08, "head"), (0.20, "trunk_1"), (0.45, "trunk_1"), (0.56, "trunk_2"),
                                   (0.74, "trunk_2"), (0.84, "trunk_3")]), cell="body")

    # huge flat round ears, pink inside (the side facing forward-out)
    for s, sg in qr.SIDES:
        c = V(qr.mirror(tuple(EAR_C), sg))
        nrm = V(qr.mirror(tuple(EAR_N), sg))
        ear = ellipsoid(c, nrm, Z, 0.07, 0.07, 0.40, 0.48, 0.55, rings=3, n=16,
                        mod=lambda i, th: 1.0)
        # cup the ear: the rim curls back (reads as a shell, not a blade, when seen edge-on)
        for ring in ear.rings:
            for v in ring:
                d = v - c
                rad = (d - nrm * d.dot(nrm)).length
                v -= nrm * (0.45 * rad * rad)
        base = V(qr.mirror(P["ear_base"], sg))
        out = (V(qr.mirror(P["ear_tip"], sg)) - base).normalized()

        def face_cell(i, cen, c=c, nrm=nrm):
            d = cen - c
            lat = Z.cross(nrm).normalized()
            a, b = d.dot(lat) / 0.40, d.dot(Z) / (0.48 if d.z > 0 else 0.55)
            return "pink" if (d.dot(nrm) > 0.0 and a * a + b * b < 0.62) else "body"
        k.add(ear, lambda co, base=base, out=out, s=s: qr.chain((co - base).dot(out), [(0.0, "head"), (0.14, f"ear_{s}")]),
              cell="body", face_cell=face_cell)

    front = [(1.40, 0.28, 0.27, 0.27, 0.0), (1.05, 0.26, 0.25, 0.25, 0.0), (0.70, 0.24, 0.235, 0.235, 0.0),
             (0.35, 0.235, 0.23, 0.23, 0.0), (0.12, 0.25, 0.245, 0.245, -0.005), (0.0, 0.255, 0.25, 0.25, -0.01)]
    hind = [(1.45, 0.32, 0.30, 0.29, 0.02), (1.10, 0.28, 0.27, 0.26, 0.015), (0.72, 0.245, 0.24, 0.24, 0.0),
            (0.35, 0.235, 0.23, 0.23, 0.0), (0.12, 0.25, 0.245, 0.245, -0.005), (0.0, 0.255, 0.25, 0.25, -0.01)]
    fx = k.rig.rest_head["front_upper_l"]
    hx = k.rig.rest_head["hind_upper_l"]

    def leg_col(x, y, z, u, v):
        c = pal.fill(len(x), "body")
        cy = np.where(y < 0, fx.y, hx.y)
        ang = np.degrees(np.arctan2(x - fx.x, -(y - cy)))
        nail = (z < 0.11) & (np.abs(((ang + 30) % 30) - 15) < 10) & (np.abs(ang) < 70)
        c[nail] = pal("nail")
        return c
    k.legs(front, hind, color=leg_col, n=10)

    tail = Tube([(0, 0.92, 1.98), (0, 0.99, 1.75), (0, 1.03, 1.52), (0, 1.05, 1.32)],
                [(0.05, 0.05, 0.05), (0.04, 0.04, 0.04), (0.035, 0.035, 0.035), (0.03, 0.03, 0.03)],
                n=6, up=Y, lat=X, cap=(0.02, 0.01))
    k.add(tail, k.w_along(tail, [(0.08, "hips"), (0.25, "tail_1"), (0.5, "tail_1"), (0.65, "tail_2")]),
          cell="body")
    k.add(ellipsoid((0, 1.065, 1.22), V((0, 0.15, -1)), Y, 0.06, 0.12, 0.055, 0.05, rings=4, n=8,
                    mod=lambda i, th: 1 + 0.15 * math.cos(4 * th)), "tail_2", cell="tuft")


def ears_fn(p, back, out, fl, fr):
    # flappy ears: rotate about the vertical axis (back = flap back against the head)
    p.rel["ear_l"] = rz(0.8 * back - 0.8 * out + 22 * fl)
    p.rel["ear_r"] = rz(-0.8 * back + 0.8 * out - 22 * fr)


def trunk(p, a1, a2, a3, side=0.0):
    p.rel["trunk_1"] = ry(side) @ rx(a1)
    p.rel["trunk_2"] = ry(side * 1.3) @ rx(a2)
    p.rel["trunk_3"] = ry(side * 1.5) @ rx(a3)


def extra(p, clip, f, n):
    t = TAU * f / n
    if clip == "idle":
        trunk(p, 3 * math.sin(t), 6 * math.sin(t + 0.6), -8 - 10 * math.sin(t + 1.2), side=5 * math.sin(t))
        fl = env(f, 10, 16, 20, 28) + env(f, 50, 56, 60, 68)
        p.rel["ear_l"] = rz(-18 * fl) @ p.rel["ear_l"]
        p.rel["ear_r"] = rz(18 * fl) @ p.rel["ear_r"]
    elif clip == "walk":
        trunk(p, 3 * math.sin(2 * t), 5 * math.sin(2 * t + 0.7), 8 * math.sin(2 * t + 1.4), side=6 * math.sin(t))
    elif clip == "eat":
        reach = env(f, 2, 12, 16, 22)
        curl = env(f, 18, 26, 40, 54)
        chew = sum(env(f, c - 4, c - 1, c, c + 4) for c in (31, 40))
        trunk(p, -12 * reach + 30 * curl, -22 * reach + 70 * curl, -20 * reach + 80 * curl + 5 * chew)
    elif clip == "drink":
        s = 0.5 - 0.5 * math.cos(t)  # 0 = sucking at the water, 1 = at the mouth
        trunk(p, -10 + 38 * s, -18 + 72 * s, -10 + 80 * s)
    elif clip == "happy":
        up = env(f, 4, 13, 26, 40)
        trunk(p, -45 * up, -55 * up, -45 * up + 10 * math.sin(TAU * f / 10) * up)
    elif clip == "refuse":
        e = env(f, 0, 6, 26, 36)
        trunk(p, 8 * e, 10 * e, 10 * e, side=-12 * math.sin(TAU * (f - 4) / 11) * e)


def clipset(k):
    return qk.ClipSet(k, {
        "sc": 1.75,
        "tail_amp": 16.0,
        "ears_fn": ears_fn,
        "extra": extra,
        "walk": dict(frames=28, stance=0.55, lift=0.12, toe_deg=15.0, fold_deg=30.0, toe_fwd=0.12,
                     dip=0.03, roll=2.0),
        "eat": dict(nose_z=1.95, body=(0.02, 3.0), split=(0.5, 0.5, 0.0)),
        "drink": dict(nose_z=1.85, body=(0.03, 4.0), split=(0.5, 0.5, 0.0)),
        "happy": dict(hop=0.3, fold=20.0, toss=0.8),
        "head_look": 5.0,
    })


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    qk.run(kit, build, clipset, ["idle", "walk", "eat", "drink", "happy", "refuse"],
           dict(center_z=1.55, walk_frame=6, big_scale=5.0, close_scale=4.6,
                head_pt=(0, -1.3, 2.3), debug_scale=2.0))
