"""kiwi — Kiwi (ART-ANIMALS "Night animals"; `biped_animal` rig of biped_rig.py, night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/kiwi.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/kiwi.blend, assets/models/animals/kiwi.glb,
assets/textures/animals/kiwi_body.png, art/animals/kiwi/model_preview.png.

Look: art/animals/kiwi/{front,side,back,three_quarter}.png (sheet_v2). A round fluffy brown
ball of hair-like feathers (shaggy lower edge), small round head, long pale-ivory beak
curving down, small dark eyes, thick beige legs with big three-toed feet; no visible wings or
tail. Top of the head 0.7 m (Q-143).

Rig: the 23-joint `biped_animal` skeleton (like the monkey; a bird leg needs knee + ankle +
foot for planted steps): the wing stubs are the arm chains and the tail chain sit hidden in
the body. Origin on the ground between the feet. Clips: idle, walk (quick planted steps,
1.4 m/s), eat (probes the ground with its beak), happy, refuse, sleep (sits down, beak tucked
back under the feathers). Eye caps: `eye_glow`.
"""

import math
import os
import sys

from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import biped_rig as br  # noqa: E402
import night_kit as nk  # noqa: E402
import rig_base as rb  # noqa: E402

qr = nk.qr
V = nk.V
X, Y, Z = nk.X, nk.Y, nk.Z
FWD = qr.FWD
Tube, ellipsoid = nk.Tube, nk.ellipsoid
rx, ry, rz = rb.rx, rb.ry, rb.rz
env = rb.envelope
TAU = rb.TAU
SIDES = rb.SIDES
ASSET = "kiwi"

COLORS = {
    "brown": "#8C6443",
    "shade": "#6A4A30",
    "beak": "#E8C9A0",
    "leg": "#C9A77E",
    "eye_white": "#FFFFFF",
    "iris": "#2A1E18",
    "pupil": "#141010",
    "ink": "#241A14",
}

P = {
    "hips": (0, 0.02, 0.26), "spine": (0, 0.0, 0.36), "chest": (0, -0.03, 0.45),
    "neck": (0, -0.07, 0.52), "head": (0, -0.10, 0.59), "head_top": (0, -0.10, 0.70),
    "shoulder": (0.16, -0.02, 0.42), "elbow": (0.18, 0.02, 0.36), "wrist": (0.18, 0.05, 0.31),
    "hand_end": (0.18, 0.07, 0.28),
    "hip": (0.085, 0.0, 0.20), "knee": (0.085, 0.0, 0.125), "ankle": (0.085, 0.0, 0.05),
    "toe": (0.085, -0.12, 0.0),
    "tail": [(0, 0.15, 0.30), (0, 0.17, 0.29), (0, 0.19, 0.28), (0, 0.21, 0.27), (0, 0.22, 0.265),
             (0, 0.23, 0.26)],
}
HEAD_C = V((0, -0.10, 0.615))
BEAK = [(0, -0.175, 0.605), (0, -0.25, 0.575), (0, -0.32, 0.53), (0, -0.37, 0.48), (0, -0.395, 0.445)]


def build(k):
    pal = k.pal
    shag = lambda i, th: 1 + (0.06 * math.cos(9 * th + i) if i <= 2 else 0.02 * math.cos(7 * th))  # noqa: E731

    def body_col(x, y, z, u, v):
        c = pal.fill(len(x), "brown")
        c[z < 0.17] = pal("shade")
        return c
    body = ellipsoid((0, 0.02, 0.335), FWD, Z, 0.22, 0.20, 0.235, 0.20, 0.205, rings=8, n=16, lat=X,
                     mod=lambda i, th: 1 + 0.05 * math.cos(9 * th + 2 * i))
    k.add(body, lambda co: rb.chain(co.z, [(0.2, "hips"), (0.34, "spine"), (0.46, "chest")]),
          region="torso", color=body_col)
    neck = Tube([(0, -0.04, 0.45), (0, -0.08, 0.53), (0, -0.10, 0.60)],
                [(0.08, 0.08, 0.08), (0.075, 0.075, 0.075), (0.07, 0.07, 0.07)], 10, up=FWD, lat=X,
                cap=(0.01, 0.01))
    k.add(neck, lambda co: rb.chain(co.z, [(0.47, "chest"), (0.52, "neck"), (0.59, "head")]), cell="brown")
    head = ellipsoid(HEAD_C, FWD, Z, 0.08, 0.085, 0.085, 0.085, 0.08, rings=6, n=12, lat=X)
    k.add(head, "head", cell="brown")
    radii = [0.03, 0.024, 0.019, 0.015, 0.011]
    beak = Tube(BEAK, [(r, r, r) for r in radii], 8, up=Z, lat=X, cap=(0.01, 0.012))
    k.add(beak, "head", cell="beak")
    for _, sg in SIDES:
        nk.eye_at(k, V((sg * 0.056, -0.162, 0.635)), V((sg * 0.62, -0.75, 0.2)), V((0, -1, 0)),
                  (0.03, 0.033), 0.012)
    k.eye_swatch(iris="iris", iris_r=0.66, pupil_r=0.44, look=0.06)

    for s, sg in SIDES:
        hx = sg * 0.085
        leg = Tube([(hx, 0.0, 0.21), (hx, 0.0, 0.125), (hx, 0.0, 0.05)],
                   [(0.05, 0.05, 0.05), (0.045, 0.045, 0.045), (0.04, 0.04, 0.04)], 8, up=FWD, lat=X,
                   tangent=-Z, cap=(0.02, 0.015))
        k.add(leg, lambda co, s=s: rb.chain(co.z, [(0.09, f"lower_leg_{s}"), (0.16, f"upper_leg_{s}")]),
              cell="leg")
        base = V((hx, -0.015, 0.03))
        for tip in ((hx - 0.055, -0.125, 0.02), (hx, -0.145, 0.02), (hx + 0.055, -0.125, 0.02),
                    (hx, 0.06, 0.02)):
            tip = V(tip)
            toe = Tube([base, base.lerp(tip, 0.55) + V((0, 0, 0.006)), tip],
                       [(0.027,) * 3, (0.024,) * 3, (0.02,) * 3], 6, up=Z, cap=(0.01, 0.02))
            k.add(toe, f"foot_{s}", cell="leg")


# --------------------------------------------------------------------------- clips

class KiwiClips:
    def __init__(self, rig):
        self.sk = rig
        self.walk = br.make_walk(rig, frames=6, speed=1.4, stance=0.45, lift=0.045, toe_deg=18.0,
                                 fold_deg=30.0, toe_fwd=0.12, upper_fn=self._walk_upper, dip=0.008)

    def pose(self):
        return br.Pose(self.sk)

    def stubs(self, p, flap=0.0):
        for s, sg in SIDES:
            p.rel[f"upper_arm_{s}"] = ry(-flap * sg)

    def _walk_upper(self, p, fi, t):
        p.rel["spine"] = rz(-5 * math.sin(t)) @ rx(2.0)
        p.rel["chest"] = rz(-3 * math.sin(t))
        p.rel["neck"] = rx(-2 + 3 * math.cos(2 * t))
        p.rel["head"] = rx(-2 * math.cos(2 * t)) @ rz(3 * math.sin(t))
        self.stubs(p, 6 * math.cos(2 * t))

    def idle(self, f, n=90):
        t = TAU * f / n
        b = math.sin(2 * t)
        p = self.pose()
        p.stand((0.004 * math.sin(t), 0, -0.004 + 0.003 * b), spine=rx(-1.0 * b), chest=rx(-0.8 * b))
        look = env(f, 15, 25, 40, 50)
        p.rel["neck"] = rz(14 * look * math.sin(t)) @ rx(2 * math.sin(2 * t + 1))
        p.rel["head"] = rx(6 * env(f, 60, 64, 68, 72) - 6 * env(f, 72, 76, 80, 84)) @ ry(6 * math.sin(t))
        self.stubs(p)
        return p

    def eat(self, f, n=60):
        k = env(f, 0, 12, 46, 60)
        poke = sum(env(f, c - 4, c - 1, c, c + 4) for c in (22, 31, 40))
        p = self.pose()
        p.stand((0, 0, -0.02 * k), spine=rx(14 * k), chest=rx(16 * k))
        p.rel["neck"] = rx(22 * k + 8 * poke)
        p.rel["head"] = rx(18 * k + 10 * poke)
        self.stubs(p)
        return p

    def happy(self, f, n=45):
        crouch = env(f, 0, 6, 6, 10)
        jump = env(f, 8, 15, 15, 23)
        land = env(f, 21, 26, 27, 36)
        z = -0.03 * crouch + 0.13 * jump - 0.02 * land
        p = self.pose()
        feet = {s: (p.rest_ankle(s) + V((0, 0, max(0.0, z))), 0.0) for s, _ in SIDES}
        p.stand((0, 0, z), chest=rx(-6 * jump + 4 * crouch), feet=feet)
        p.rel["neck"] = rx(-10 * env(f, 6, 14, 22, 36)) @ rz(8 * math.sin(TAU * f / 15))
        self.stubs(p, 30 * env(f, 6, 12, 26, 34) * math.sin(TAU * f / 6))
        return p

    def refuse(self, f, n=36):
        e = env(f, 0, 6, 26, 36)
        sh = env(f, 3, 8, 24, 32)
        ang = 28.0 * math.sin(TAU * (f - 4) / 11) * sh
        p = self.pose()
        p.stand((0, 0.02 * e, -0.005 * e), spine=rx(-4 * e), chest=rx(-5 * e))
        p.rel["neck"] = rz(0.4 * ang) @ rx(-6 * e)
        p.rel["head"] = rz(0.6 * ang)
        self.stubs(p, 8 * e)
        return p

    def sleep(self, f, n=90):
        t = TAU * f / n
        b = math.sin(t)
        p = self.pose()
        p.stand((0, 0.01, -0.07 + 0.003 * b), spine=rx(4 + 0.8 * b), chest=rx(4 + 0.8 * b))
        p.rel["neck"] = rz(70) @ rx(30)
        p.rel["head"] = rz(40) @ rx(35 + 1.5 * math.sin(t + 0.7))
        self.stubs(p, -4)
        return p

    def all(self):
        C = rb.Clip
        return [C("idle", 90, True, self.idle), C("walk", 6, True, self.walk),
                C("eat", 60, False, self.eat), C("happy", 45, False, self.happy),
                C("refuse", 36, False, self.refuse), C("sleep", 90, True, self.sleep)]


if __name__ == "__main__":
    rig = br.Rig(P)
    kit = nk.GKit(ASSET, COLORS, rig)
    nk.run(kit, build, lambda k: KiwiClips(k.rig).all(),
           dict(loco=("walk", 1), center_z=0.36, height_m=2.2, big_scale=1.2, close_scale=1.1,
                night_pose=("idle", 20)),
           debug=dict(center_z=0.36, head_pt=(0, -0.2, 0.6), head_scale=0.5, strip_scale=1.1))
