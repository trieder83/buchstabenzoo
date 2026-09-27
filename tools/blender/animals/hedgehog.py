"""hedgehog — Igel (ART-ANIMALS "Night animals"; quadruped rig, quad_kit + night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/hedgehog.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/hedgehog.blend, assets/models/animals/hedgehog.glb,
assets/textures/animals/hedgehog_body.png, art/animals/hedgehog/model_preview.png.

Look: art/animals/hedgehog/{front,side,back,three_quarter}.png (sheet_v4_clean). Round
teardrop body on four short stubby legs, a brown spiky dome of big rounded cone tufts with
cream tips (rows sweeping back), light tan face and belly, pointed pink-tan snout with a big
round black nose, small round pink ears, big friendly blue-grey eyes. Top of the spines
0.6 m, length ~0.72 m (comic-scaled, Q-143).

Clips: idle, walk (tiny quick steps, 1.4 m/s), eat, happy, refuse, sleep (rolled up into a
spiky ball, face hidden, slow breathing). Eye highlight faces: material `eye_glow`.
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
rx, ry, rz = qr.rx, qr.ry, qr.rz
env = qr.envelope
ASSET = "hedgehog"

COLORS = {
    "spine": "#7A5638",
    "tip": "#E8D2A8",
    "tan": "#E9C99A",
    "snout": "#D9A382",
    "ear_in": "#C98372",
    "foot": "#C99274",
    "nose": "#2A2A2A",
    "eye_white": "#FFFFFF",
    "iris": "#4E6470",
    "pupil": "#1A1614",
    "ink": "#2B1B12",
    "spine_dark": "#664629",
}

HEAD_B = V((0, -0.07, 0.285))
HEAD_D = V((0, -1.0, -0.06)).normalized()
HEAD_PROFILE = [
    (0.0, 0.06, 0.06, 0.06), (0.04, 0.13, 0.12, 0.105), (0.09, 0.15, 0.135, 0.115),
    (0.14, 0.14, 0.12, 0.10), (0.185, 0.10, 0.085, 0.075), (0.225, 0.062, 0.056, 0.05),
    (0.26, 0.04, 0.037, 0.034)]
HEAD_N = HEAD_B + HEAD_D * 0.275

P = {
    "hips": (0, 0.13, 0.23), "spine": (0, 0.0, 0.24), "chest": (0, -0.10, 0.24),
    "neck_1": (0, -0.12, 0.26), "neck_2": (0, -0.14, 0.27), "head": (0, -0.16, 0.285),
    "nose": tuple(HEAD_N),
    "ear_base": (0.10, -0.13, 0.37), "ear_tip": (0.13, -0.12, 0.445),
    "tail_1": (0, 0.26, 0.19), "tail_2": (0, 0.29, 0.17), "tail_end": (0, 0.31, 0.16),
    "front_xy": (0.10, -0.11), "front_z": (0.17, 0.095, 0.035),
    "hind_xy": (0.11, 0.13), "hind_z": (0.17, 0.095, 0.035),
    "toe": 0.04,
}

COAT_C = V((0, 0.05, 0.275))


def build(k):
    pal = k.pal
    T = [(0.27, 0.22, 0.06, 0.06, 0.06), (0.22, 0.22, 0.15, 0.13, 0.13),
         (0.12, 0.22, 0.20, 0.15, 0.15), (0.0, 0.22, 0.21, 0.15, 0.15),
         (-0.10, 0.23, 0.20, 0.14, 0.14), (-0.17, 0.24, 0.16, 0.12, 0.12),
         (-0.21, 0.25, 0.08, 0.07, 0.07)]
    k.torso(T, cell="tan", leg_r=0.045, cap=(0.03, 0.03))

    # spiky coat: a dome over the back and the back of the head
    coat = ellipsoid(COAT_C, FWD, Z, 0.28, 0.235, 0.265, 0.265, 0.13, rings=8, n=16, lat=X)
    wt = k.w_torso(0.045, 0.3)
    k.add(coat, wt, cell="spine", name="coat")

    # cone tufts in rows sweeping back (brown with cream tips: 'extra' region)
    def tuft_col(U, Vv):
        c = np.broadcast_to(pal("spine"), U.shape + (3,)).copy()
        c[U > 0.70] = pal("tip")
        c[(U > 0.30) & (U < 0.70) & (Vv > 0.55)] = pal("spine_dark")
        return c
    k.paint_region("extra", tuft_col)
    at = k.atlas
    rows = [0.12, 0.22, 0.32, 0.42, 0.52, 0.62, 0.72, 0.82, 0.90]
    count = 0
    for ri, u in enumerate(rows):
        m = 9 if ri < 8 else 5
        span = 108.0 if ri < 7 else 70.0
        for j in range(m):
            th = math.radians(-span + 2 * span * (j + 0.5 * (ri % 2)) / (m - 1 + 0.5 * (ri % 2) + 1e-9))
            th = max(min(th, math.radians(span)), math.radians(-span))
            S = coat.point(u, th)
            if S.z < 0.17:
                continue
            nrm = coat.normal(u, th)
            d = (nrm + Y * 0.95 + Z * 0.15).normalized()
            L = 0.072 + 0.015 * math.sin(3.1 * ri + 1.7 * j)
            r = 0.05
            c = nk.cone(S - nrm * 0.018, d, r, L, n=5, up=nrm.cross(X).normalized() if abs(nrm.x) < 0.9 else Z)
            n = c.n

            def uv(i, kk, co, c=c, n=n):
                return at.map_uv("extra", c.U[i + c.off] if 0 <= i < c.R else 1.0, qr.ring_h(kk, n))
            k.mb.loft(c.rings, uv, wt, pole_start=None, pole_end=c.pole1)
            count += 1
    print("tufts:", count)

    head = k.head(HEAD_B, HEAD_D, HEAD_PROFILE,
                  color=lambda x, y, z, u, v: np.where((u > 0.74)[:, None], pal("snout"), pal("tan")))
    k.eyes(head, 0.47, 40.0, (0.05, 0.056), depth=0.022)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.38, look=0.08)
    k.add(ellipsoid(HEAD_N + V((0, 0.006, 0.01)), FWD, Z, 0.02, 0.022, 0.033, 0.03, 0.026, rings=4, n=8),
          "head", cell="nose")
    for s, sg in qr.SIDES:
        k.ear(sg, P["ear_base"], P["ear_tip"], 0.048, 0.016, V((0.3, -1, 0.1)), "snout", "ear_in",
              prof=[(-0.2, 0.8, 0.9), (0.15, 1.0, 1.0), (0.5, 0.95, 0.9), (0.8, 0.7, 0.8),
                    (0.97, 0.3, 0.5)], inner_frac=0.6, inner_t=(0.05, 0.9))

    front = [(0.16, 0.05, 0.05, 0.05, 0.0), (0.10, 0.046, 0.046, 0.046, 0.0),
             (0.05, 0.043, 0.043, 0.043, -0.003)]
    hind = [(0.16, 0.056, 0.056, 0.054, 0.0), (0.10, 0.048, 0.048, 0.048, 0.0),
            (0.05, 0.044, 0.044, 0.044, -0.003)]

    def paw(leg, h):
        return ellipsoid((h.x, h.y - 0.02, 0.027), FWD, Z, 0.038, 0.055, 0.048, 0.032, 0.027,
                         rings=4, n=8)
    k.legs(front, hind, cell="foot", paw=paw, paw_cell="foot", n=8)


class HedgehogClips(qk.ClipSet):
    def sleep(self, f, n=90):
        """Rolled up into a spiky ball: rump down, spine and neck curled so the face is
        tucked under the coat, legs folded in; slow breathing (loop)."""
        t = TAU * f / n
        b = math.sin(t)
        p = qr.Pose(self.rig)
        p.hips_offset = Vector((0, -0.03, -0.085 + 0.004 * b))
        p.rel["hips"] = rx(-30)
        p.rel["spine"] = rx(30 + 1.2 * b)
        p.rel["chest"] = rx(36 + 1.2 * b)
        p.rel["neck_1"] = rx(36)
        p.rel["neck_2"] = rx(36)
        p.rel["head"] = rx(44)
        for leg in qr.LEGS:
            up, lo, ft = qr.leg_bones(leg)
            A = p.head(up)
            fr = leg.startswith("front")
            tgt = Vector((0.8 * A.x, -0.10 if fr else 0.02, 0.035))
            p.leg_ik(leg, tgt, -40 if fr else 30)
        for i, j in enumerate(self.tail):
            p.rel[j] = rx(-20)
        self.ears(p, back=25)
        return p

    def clips(self, names):
        out = []
        for nm in names:
            if nm == "sleep":
                out.append(qr.Clip("sleep", 90, True, self.sleep))
            else:
                out += super().clips([nm])
        return out


TAU = qk.TAU
from mathutils import Vector  # noqa: E402

CLIPS = ["idle", "walk", "eat", "happy", "refuse", "sleep"]


def clips(k):
    cs = HedgehogClips(k, {
        "sc": 0.26,
        "tail_amp": 4.0,
        "walk": dict(frames=8, stance=0.4, lift=0.035, toe_deg=25.0, fold_deg=45.0, toe_fwd=0.04,
                     dip=0.008),
        "eat": dict(nose_z=0.05, body=(0.02, 4.0), split=(0.55, 0.45, -0.1)),
        "happy": dict(hop=1.2),
        "head_look": 10.0,
    })
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in cs.walk.drop))
    return cs.clips(CLIPS)


if __name__ == "__main__":
    kit = qk.Kit(ASSET, COLORS, P)
    nk.run(kit, build, clips,
           dict(loco=("walk", 2), center_z=0.28, height_m=2.2, big_scale=1.05, close_scale=0.95,
                night_pose=("idle", 20)),
           debug=dict(center_z=0.28, head_pt=(0, -0.25, 0.33), head_scale=0.45, strip_scale=1.0))
