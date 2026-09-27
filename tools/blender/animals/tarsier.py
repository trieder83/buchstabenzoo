"""tarsier — Koboldmaki (ART-ANIMALS "Night animals"; `biped_animal` rig of biped_rig.py,
night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/tarsier.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/tarsier.blend, assets/models/animals/tarsier.glb,
assets/textures/animals/tarsier_body.png, art/animals/tarsier/model_preview.png.

Look: art/animals/tarsier/{front,side,back,three_quarter}.png (sheet_v1). Upright on long
legs with long pink feet, a big round head with **giant amber eyes** in a light-beige face,
big round pink-lined ears, small pink nose, hands held at the chest, light-beige belly, a
long thin tail with a dark tuft. Sitting-upright height 0.7 m (Q-143).

Rig: the 23-joint `biped_animal` skeleton (like the monkey). Origin on the ground between the
feet. Locomotion is **`hop`** (loop, 16 frames): both feet together, 4 frames of ground
contact with the feet planted (they move back at 1.4 m/s), then a 0.12 m leap; no root motion
(the game moves the tarsier at 1.4 m/s like `walk`). Clips: idle, hop, eat, happy, refuse,
sleep (crouched, head bowed onto the chest, tail curled round). Eye caps: `eye_glow`.
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
ASSET = "tarsier"

COLORS = {
    "fur": "#B08A62",
    "beige": "#E3CFB0",
    "pink": "#D8A88E",
    "tuft": "#8C6A48",
    "nose": "#B07A68",
    "eye_white": "#FFFFFF",
    "iris": "#C98A2E",
    "pupil": "#1A1210",
    "ink": "#2B1B12",
}

P = {
    "hips": (0, 0.03, 0.24), "spine": (0, 0.03, 0.31), "chest": (0, 0.02, 0.38),
    "neck": (0, 0.0, 0.43), "head": (0, -0.01, 0.47), "head_top": (0, -0.01, 0.66),
    "shoulder": (0.08, 0.01, 0.39), "elbow": (0.10, 0.01, 0.30), "wrist": (0.10, 0.0, 0.23),
    "hand_end": (0.10, -0.01, 0.19),
    "hip": (0.07, 0.03, 0.24), "knee": (0.07, 0.03, 0.14), "ankle": (0.07, 0.03, 0.05),
    "toe": (0.07, -0.09, 0.0),
    "tail": [(0, 0.10, 0.22), (0, 0.18, 0.17), (0, 0.25, 0.12), (0, 0.30, 0.08), (0, 0.34, 0.06),
             (0, 0.39, 0.05)],
}
HEAD_C = V((0, -0.02, 0.535))


def build(k):
    pal = k.pal

    def head_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        face = (y < -0.07) & (z < 0.61) & (((x / 0.13) ** 2 + ((z - 0.53) / 0.1) ** 2) < 1)
        c[face] = pal("beige")
        return c
    head = ellipsoid(HEAD_C, Z, FWD, 0.13, 0.135, 0.155, 0.13, 0.125, rings=8, n=16, lat=X)
    k.add(head, lambda co: rb.chain(co.z, [(0.41, "chest"), (0.44, "neck"), (0.48, "head")]),
          region="head", color=head_col)
    for _, sg in SIDES:
        nk.eye_at(k, V((sg * 0.062, -0.128, 0.555)), V((sg * 0.3, -1, 0.05)), V((-sg, 0, 0)),
                  (0.06, 0.064), 0.026)
    k.eye_swatch(iris="iris", iris_r=0.76, pupil_r=0.42, look=0.04)
    k.add(ellipsoid((0, -0.125, 0.47), FWD, Z, 0.03, 0.035, 0.05, 0.035, 0.035, rings=4, n=10),
          "head", cell="beige")
    k.add(ellipsoid((0, -0.16, 0.485), FWD, Z, 0.01, 0.012, 0.016, 0.01, 0.01, rings=3, n=8),
          "head", cell="nose")
    for _, sg in SIDES:
        k.ear(sg, (0.10, 0.0, 0.61), (0.175, 0.02, 0.695), 0.075, 0.018, V((0.3, -1, 0.1)), "fur", "pink",
              prof=[(-0.2, 0.8, 0.9), (0.1, 1.0, 1.0), (0.5, 1.0, 0.9), (0.85, 0.7, 0.8), (0.99, 0.3, 0.5)],
              inner_frac=0.62, inner_t=(0.08, 0.9), weight=qr.rigid("head"))

    def body_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[(y < -0.02) & ((x / 0.08) ** 2 + ((z - 0.3) / 0.1) ** 2 < 1)] = pal("beige")
        return c
    body = ellipsoid((0, 0.03, 0.315), Z, FWD, 0.13, 0.12, 0.11, 0.105, 0.10, rings=7, n=14, lat=X)
    k.add(body, lambda co: rb.chain(co.z, [(0.24, "hips"), (0.31, "spine"), (0.38, "chest")]),
          region="torso", color=body_col)

    for s, sg in SIDES:
        m = lambda p: V((sg * p[0], p[1], p[2]))  # noqa: E731
        arm = Tube([m(P["shoulder"]), m(P["elbow"]), m(P["wrist"])],
                   [(0.03,) * 3, (0.026,) * 3, (0.022,) * 3], 7, up=FWD, lat=X, cap=(0.01, 0.01))
        L1 = (V(P["elbow"]) - V(P["shoulder"])).length
        sh = m(P["shoulder"])
        k.add(arm, lambda co, sh=sh, s=s: rb.chain((co - sh).length,
                                                   [(0.3 * L1, f"upper_arm_{s}"), (1.1 * L1, f"lower_arm_{s}")]),
              cell="fur")
        k.add(ellipsoid(m((0.10, -0.01, 0.2)), V((0, 0, -1)), FWD, 0.02, 0.03, 0.03, 0.02, 0.02,
                        rings=3, n=8), f"hand_{s}", cell="pink")
        leg = Tube([m((0.07, 0.03, 0.25)), m((0.07, 0.03, 0.14)), m((0.07, 0.03, 0.05))],
                   [(0.06, 0.06, 0.06), (0.045, 0.045, 0.045), (0.028, 0.028, 0.028)], 8, up=FWD, lat=X,
                   tangent=-Z, cap=(0.02, 0.01))
        k.add(leg, lambda co, s=s: rb.chain(co.z, [(0.11, f"lower_leg_{s}"), (0.17, f"upper_leg_{s}")]),
              cell="fur")
        k.add(ellipsoid(m((0.07, -0.02, 0.022)), FWD, Z, 0.06, 0.07, 0.035, 0.022, 0.02, rings=4, n=8),
              f"foot_{s}", cell="pink")

    tp = [V(p) for p in P["tail"]]
    tail = Tube(tp, [(r, r, r) for r in (0.02, 0.016, 0.014, 0.013, 0.013, 0.012)], 6, up=Z, lat=X,
                cap=(0.005, 0.01))
    stops = [(0.0, "hips")] + [(0.1 + 0.18 * i, f"tail_{i + 1}") for i in range(5)]
    k.add(tail, k.w_along(tail, stops), cell="fur")
    k.add(ellipsoid(tp[-1] + V((0, 0.03, 0.01)), V((0, 1, 0.3)), Z, 0.03, 0.045, 0.035, 0.035, 0.03,
                    rings=4, n=8, mod=lambda i, th: 1 + 0.12 * math.cos(5 * th)), "tail_5", cell="tuft")


# --------------------------------------------------------------------------- clips

HOP_N, HOP_C, HOP_H = 16, 0.25, 0.12


class TarsierClips:
    def __init__(self, rig):
        self.sk = rig

    def pose(self):
        return br.Pose(self.sk)

    def hands_at_chest(self, p, lift=0.0, spread=0.0):
        for s, sg in SIDES:
            tgt = p.point("chest", (sg * (0.045 + spread), -0.10, 0.34 + lift))
            p.arm_ik(s, tgt, V((sg * 1.0, 0.6, -0.3)), hand_world=p.world("chest") @ rx(-70))

    def tail(self, p, sway=0.0, curl=0.0):
        for i in range(1, 6):
            p.rel[f"tail_{i}"] = rz(sway * (0.5 + 0.2 * i)) @ rx(curl)

    def look(self, p, yaw, pitch=0.0, tilt=0.0):
        p.rel["neck"] = rz(0.35 * yaw) @ rx(0.4 * pitch)
        p.rel["head"] = rz(0.65 * yaw) @ rx(0.6 * pitch) @ ry(tilt)

    def idle(self, f, n=90):
        t = TAU * f / n
        b = math.sin(2 * t)
        p = self.pose()
        p.stand((0, 0, -0.03 + 0.003 * b), spine=rx(3 - b), chest=rx(-0.8 * b))
        yaw = 40 * env(f, 10, 18, 30, 38) - 40 * env(f, 46, 54, 66, 74)
        self.look(p, yaw, 2 * math.sin(2 * t + 1), 8 * env(f, 76, 80, 84, 89))
        self.hands_at_chest(p, 0.004 * b)
        self.tail(p, 6 * math.sin(t))
        return p

    def hop(self, f, n=HOP_N):
        s = (f % n) / n
        T = n / br.FPS
        D = 1.4 * HOP_C * T
        p = self.pose()
        feet = {}
        if s < HOP_C:
            k = s / HOP_C
            dy = -D / 2 + D * k
            z = -0.035 - 0.02 * math.sin(math.pi * k)
            for side, _ in SIDES:
                feet[side] = (p.rest_ankle(side) + V((0, dy, 0)), 0.0)
            lean = 12
        else:
            u = (s - HOP_C) / (1 - HOP_C)
            e = rb.ease(u)
            h = HOP_H * math.sin(math.pi * u)
            z = -0.035 + h
            dy = D / 2 - D * e
            for side, _ in SIDES:
                feet[side] = (p.rest_ankle(side) + V((0, dy, h + 0.03 * math.sin(math.pi * u))),
                              -25 * math.sin(math.pi * u))
            lean = 12 - 10 * math.sin(math.pi * u)
        p.stand((0, 0, z), hips_rot=rx(lean), spine=rx(-0.5 * lean), feet=feet)
        p.rel["neck"] = rx(-0.3 * lean)
        p.rel["head"] = rx(-0.3 * lean)
        self.hands_at_chest(p)
        self.tail(p, 0.0, -8 * math.sin(TAU * s))
        return p

    def eat(self, f, n=60):
        k = env(f, 0, 10, 48, 60)
        bite = sum(env(f, c - 4, c - 1, c, c + 4) for c in (22, 31, 40))
        p = self.pose()
        p.stand((0, 0, -0.04 * k), spine=rx(6 * k), chest=rx(4 * k))
        self.look(p, 0.0, 14 * k + 6 * bite)
        self.hands_at_chest(p, 0.07 * k, -0.02 * k)
        self.tail(p, 4 * math.sin(TAU * f / 30))
        return p

    def happy(self, f, n=45):
        crouch = env(f, 0, 6, 6, 10)
        jump = env(f, 8, 15, 15, 23)
        land = env(f, 21, 26, 27, 36)
        z = -0.05 * crouch + 0.16 * jump - 0.03 * land
        p = self.pose()
        feet = {s: (p.rest_ankle(s) + V((0, 0, max(0.0, z))), -20 * jump) for s, _ in SIDES}
        p.stand((0, 0, z), spine=rx(-6 * jump + 5 * crouch), feet=feet)
        self.look(p, 0.0, -10 * env(f, 6, 14, 22, 36), 10 * math.sin(TAU * f / 15))
        up = env(f, 8, 14, 24, 32)
        for s, sg in SIDES:
            tgt = p.point("chest", (sg * (0.05 + 0.12 * up), -0.08, 0.34 + 0.2 * up))
            p.arm_ik(s, tgt, V((sg * 1.0, 0.6, -0.3)))
        self.tail(p, 18 * math.sin(TAU * f / 12) * up, -10 * up)
        return p

    def refuse(self, f, n=36):
        e = env(f, 0, 6, 26, 36)
        sh = env(f, 3, 8, 24, 32)
        ang = 30.0 * math.sin(TAU * (f - 4) / 11) * sh
        p = self.pose()
        p.stand((0, 0.02 * e, -0.03), spine=rx(-6 * e))
        self.look(p, ang, -6 * e)
        self.hands_at_chest(p, 0.0, 0.02 * e)
        self.tail(p, 12 * e * math.sin(TAU * f / 9))
        return p

    def sleep(self, f, n=90):
        t = TAU * f / n
        b = math.sin(t)
        p = self.pose()
        p.stand((0, 0.01, -0.09 + 0.003 * b), spine=rx(12 + b), chest=rx(10 + b))
        self.look(p, 0.0, 40 + 2 * math.sin(t + 0.7))
        self.hands_at_chest(p, -0.04)
        for i in range(1, 6):
            p.rel[f"tail_{i}"] = rz(28)
        return p

    def all(self):
        C = rb.Clip
        return [C("idle", 90, True, self.idle), C("hop", HOP_N, True, self.hop),
                C("eat", 60, False, self.eat), C("happy", 45, False, self.happy),
                C("refuse", 36, False, self.refuse), C("sleep", 90, True, self.sleep)]


if __name__ == "__main__":
    rig = br.Rig(P)
    kit = nk.GKit(ASSET, COLORS, rig)
    nk.run(kit, build, lambda k: TarsierClips(k.rig).all(),
           dict(loco=("hop", 8), center_z=0.36, height_m=2.2, big_scale=1.2, close_scale=1.05,
                night_pose=("idle", 5)),
           debug=dict(center_z=0.36, head_pt=(0, -0.08, 0.55), head_scale=0.5, strip_scale=1.1))
