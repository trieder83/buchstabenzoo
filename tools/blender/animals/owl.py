"""owl — Eule (ART-ANIMALS "Night animals"; `bird` rig of bird_rig.py, built with night_kit).

Run:  blender -b --factory-startup --python tools/blender/animals/owl.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/owl.blend, assets/models/animals/owl.glb,
assets/textures/animals/owl_body.png, art/animals/owl/model_preview.png.

Look: art/animals/owl/{front,side,back,three_quarter}.png (sheet_v1). Egg-shaped chunky
brown body with a cream spotted belly, a big round head merged with the body, a pale
heart-shaped face disc with two giant golden eyes, small yellow beak, two small ear tufts,
wings folded at the sides with dark tips, short cream feathered legs and big yellow feet.
Perched height 0.80 m (comic-scaled; user decision 2026-09-27, v1 was 1.01 m). The model is
authored in v1 units (P, WING, build, clip offsets) and scaled uniformly by `SCALE`: the rig
joints and the finished mesh are scaled, clip translations (`hips_offset`) are multiplied by
`SCALE`; rotations and clip timing are unchanged.

Rig: the 16-joint `bird` skeleton (bird_rig.py) with an owl's proportions. **Origin at the
feet** (not the duck's waterline): min Y = 0 in the rest pose. `perch` = idle with the toes
curled over a branch whose top is the origin. `fly` is authored in place at the perched
height (no root motion, no lift): the game raises the origin by the flight height
(`fly_height` in animal_anims.toml, 1.5 m) while following and moves it at 1.4 m/s.

Clips: idle, perch, fly, eat, happy, refuse, sleep (head bowed into the chest, fluffed
down), look (the owl's big head turn). Eye caps: material `eye_glow` (NIGHT-006).
"""

import math
import os
import sys

import numpy as np
from mathutils import Quaternion, Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bird_rig as br  # noqa: E402
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
ASSET = "owl"
# uniform size factor on the v1 geometry (1.01 m perched -> 0.80 m)
SCALE = 0.80 / 1.01

COLORS = {
    "brown": "#9A6A43",
    "dark": "#6E4A2E",
    "cream": "#EAD7B4",
    "beak": "#E7A93A",
    "leg": "#DCC49C",
    "spot": "#7A5335",
    "eye_white": "#FFFFFF",
    "iris": "#F5B82E",
    "pupil": "#1E1E1E",
    "ink": "#2B1B12",
}

P = {
    "hips": (0, 0.03, 0.26), "spine": (0, 0.02, 0.40), "chest": (0, 0.0, 0.52),
    "neck_1": (0, 0.0, 0.60), "neck_2": (0, 0.0, 0.65), "head": (0, 0.0, 0.70),
    "head_end": (0, 0.0, 0.95),
    "tail": ((0, 0.20, 0.20), (0, 0.32, 0.08)),
    "wing": ((0.195, 0.0, 0.58), (0.275, 0.08, 0.38), (0.22, 0.17, 0.17)),
    "leg": ((0.10, 0.0, 0.16), (0.10, -0.005, 0.05), (0.10, -0.13, 0.012)),
}


def scaled_p(p, f=SCALE):
    """P with every joint position multiplied by f (paired entries are tuples of points)."""
    def sc(v):
        return tuple(sc(x) for x in v) if isinstance(v[0], tuple) else tuple(f * x for x in v)
    return {k: sc(v) for k, v in p.items()}


WING = [(0.195, 0.0, 0.58), (0.252, 0.04, 0.48), (0.275, 0.08, 0.38), (0.262, 0.12, 0.275),
        (0.222, 0.165, 0.175)]
WING_W = [0.07, 0.11, 0.12, 0.10, 0.05]


def build(k):
    pal = k.pal
    at = k.atlas

    def body_col(x, y, z, u, v):
        c = pal.fill(len(x), "brown")
        belly = (y < -0.02) & ((x / 0.215) ** 2 + ((z - 0.34) / 0.215) ** 2 < 1)
        c[belly] = pal("cream")
        return c

    body = ellipsoid((0, 0.02, 0.37), Z, FWD, 0.26, 0.26, 0.29, 0.25, 0.24, rings=9, n=16, lat=X)
    w_body = lambda co: rb.chain(co.z, [(0.24, "hips"), (0.36, "spine"), (0.48, "chest")])  # noqa: E731
    k.add(body, w_body, region="torso", color=body_col)
    # spots: small flat domes on the body surface (painted dots would smear near the
    # front / back centre line of the body's pattern map)
    front = [(0.0, 0.46), (0.10, 0.44), (-0.10, 0.44), (0.055, 0.37), (-0.055, 0.37),
             (0.0, 0.29), (0.11, 0.30), (-0.11, 0.30)]
    back = [(0.0, 0.52), (0.085, 0.48), (-0.085, 0.48), (0.045, 0.42), (-0.045, 0.42)]
    for pts, side in ((front, -1.0), (back, 1.0)):
        for sx, sz in pts:
            ry_ = 0.25 if side < 0 else 0.24
            q = 1 - (sx / 0.29) ** 2 - ((sz - 0.37) / 0.26) ** 2
            y = 0.02 + side * ry_ * math.sqrt(max(q, 0.0))
            n_ = V((sx / 0.29 ** 2, (y - 0.02) / ry_ ** 2, (sz - 0.37) / 0.26 ** 2)).normalized()
            S = V((sx, y, sz))
            k.add(ellipsoid(S - n_ * 0.004, n_, Z, 0.004, 0.006, 0.021, 0.03, 0.03, rings=2, n=8),
                  w_body, cell="spot")
    head = ellipsoid((0, 0.0, 0.77), Z, FWD, 0.21, 0.20, 0.29, 0.21, 0.21, rings=8, n=16, lat=X)
    k.add(head, lambda co: rb.chain(co.z, [(0.58, "chest"), (0.63, "neck_1"), (0.67, "neck_2"),
                                           (0.71, "head")]), cell="brown")
    # heart-shaped face disc: two lobes and a chin point
    for _, sg in SIDES:
        lobe = ellipsoid((sg * 0.10, -0.158, 0.765), V((sg * 0.25, -1, 0)), Z, 0.04, 0.06, 0.128,
                         0.145, 0.13, rings=5, n=14)
        k.add(lobe, "head", cell="cream")
    k.add(ellipsoid((0, -0.17, 0.665), FWD, Z, 0.03, 0.04, 0.07, 0.06, 0.05, rings=4, n=10),
          "head", cell="cream")
    for _, sg in SIDES:
        nk.eye_at(k, V((sg * 0.105, -0.222, 0.775)), V((sg * 0.22, -1, 0.04)), V((-sg, 0, 0)),
                  (0.08, 0.085), 0.03)
    k.eye_swatch(iris="iris", iris_r=0.80, pupil_r=0.50, look=0.03)
    k.add(nk.cone((0, -0.21, 0.715), (0, -0.45, -1), 0.034, 0.085, n=6, up=FWD), "head", cell="beak")
    for _, sg in SIDES:
        k.ear(sg, (0.165, -0.06, 0.90), (0.255, -0.04, 1.0), 0.055, 0.022, V((0, -1, 0.2)), "dark",
              weight=qr.rigid("head"))

    # folded wings on the body sides (U = shoulder -> tip, V = front-lower -> back-upper edge)
    def wing_col(U, Vv):
        c = np.broadcast_to(pal("brown"), U.shape + (3,)).copy()
        for u0 in (0.34, 0.52):
            e = u0 + 0.05 * np.abs(np.sin(Vv * math.pi * 3))
            c[(U > e) & (U < e + 0.03) & (Vv > 0.1) & (Vv < 0.95)] = pal("dark")
        for su, sv in [(0.15, 0.3), (0.2, 0.6), (0.28, 0.45), (0.42, 0.3), (0.42, 0.7)]:
            c[((U - su) / 0.035) ** 2 + ((Vv - sv) / 0.07) ** 2 < 1] = pal("spot")
        c[U > 0.66 + 0.05 * np.abs(np.sin(Vv * math.pi * 2.5))] = pal("dark")
        return c
    k.paint_region("extra", wing_col)
    for side, sg in SIDES:
        pts = [V((sg * x, y, z)) for x, y, z in WING]
        radii = [(0.028 * (1 - 0.3 * i / 4), w, w) for i, w in enumerate(WING_W)]
        tube = Tube(pts, radii, 10, up=V((0, 0.35, 1)), lat=X, cap=(0.03, 0.05))
        n = tube.n

        def uv(i, kk, co, tube=tube, n=n):
            U = tube.U[min(max(i + tube.off, 0), len(tube.U) - 1)] if i < tube.R else 1.0
            U = 0.0 if i < 0 else U
            return at.map_uv("extra", 0.02 + 0.96 * U, 0.5 + 0.5 * math.cos(TAU * kk / n))
        wfn = (lambda co, tube=tube, side=side:
               rb.chain(tube.param(co), [(0.0, "chest"), (0.12, f"wing_upper_{side}"),
                                         (0.45, f"wing_upper_{side}"), (0.62, f"wing_lower_{side}")]))
        k.mb.loft(tube.rings, uv, wfn, pole_start=tube.pole0, pole_end=tube.pole1)
    tail = Tube([(0, 0.17, 0.21), (0, 0.25, 0.14), (0, 0.31, 0.075)],
                [(0.07, 0.025, 0.025), (0.09, 0.022, 0.022), (0.075, 0.018, 0.018)], 8,
                up=V((0, 0.7, 0.7)), lat=X, cap=(0.01, 0.03))
    k.add(tail, "tail", cell="dark")

    # feathered legs and big yellow feet (3 toes forward, 1 back)
    for side, sg in SIDES:
        hx = sg * 0.10
        leg = Tube([(hx, 0.0, 0.2), (hx, -0.005, 0.11), (hx, -0.01, 0.05)],
                   [(0.058, 0.058, 0.058), (0.052, 0.052, 0.052), (0.042, 0.042, 0.042)], 8,
                   up=FWD, lat=X, tangent=-Z, cap=(0.02, 0.02))
        k.add(leg, f"leg_upper_{side}", cell="leg")
        base = V((hx, -0.02, 0.028))
        for tip in ((hx - 0.05, -0.135, 0.018), (hx, -0.15, 0.018), (hx + 0.05, -0.135, 0.018),
                    (hx, 0.06, 0.018)):
            tip = V(tip)
            mid = base.lerp(tip, 0.55) + V((0, 0, 0.006))
            toe = Tube([base, mid, tip], [(0.026,) * 3, (0.024,) * 3, (0.02,) * 3], 6,
                       up=Z, cap=(0.01, 0.02))
            k.add(toe, f"leg_lower_{side}", cell="beak")


# --------------------------------------------------------------------------- clips

class OwlClips:
    def __init__(self, rig):
        self.sk = rig
        self.toe0 = {s: rig.rest_tail[f"leg_lower_{s}"].copy() for s, _ in SIDES}
        self.W = nk.Wings(rig, {s: [f"wing_upper_{s}", f"wing_lower_{s}"] for s, _ in SIDES},
                          (1, 0, 0), (1, 0.12, 0))

    def pose(self):
        return rb.Pose(self.sk)

    def wings(self, p, spread=0.0, flap=0.0, lag=0.0, lift=0.0, world=False):
        self.W.apply(p, spread, flap, lag, lift, world=world)

    def plant(self, p, grip=0.0):
        """Toes on their rest spots (IK through the ankle); grip curls the foot over a
        branch (deg)."""
        for s, _ in SIDES:
            p.limb_ik(f"leg_upper_{s}", f"leg_lower_{s}", self.toe0[s], Y)
            if grip:
                p.set_world(f"leg_lower_{s}", rx(grip) @ p.world(f"leg_lower_{s}"))

    def look(self, p, yaw, pitch=0.0, tilt=0.0):
        p.rel["neck_1"] = rz(0.2 * yaw) @ p.rel["neck_1"]
        p.rel["neck_2"] = rz(0.35 * yaw) @ rx(0.4 * pitch) @ p.rel["neck_2"]
        p.rel["head"] = rz(0.45 * yaw) @ rx(0.6 * pitch) @ ry(tilt) @ p.rel["head"]

    # ---- clips
    def idle(self, f, n=90, grip=0.0):
        t = TAU * f / n
        p = self.pose()
        b = math.sin(2 * t)
        p.rel["spine"] = rx(0.8 * b)
        p.rel["chest"] = rx(-1.2 * b)
        self.look(p, 26 * math.sin(t) * (0.6 + 0.4 * math.sin(2 * t + 1)), 3 * math.sin(2 * t + 1),
                  8 * math.sin(t + 1.3) * max(0.0, math.sin(t + 1.3)))
        self.wings(p, lift=2.5 * b)
        p.rel["tail"] = rz(4 * math.sin(2 * t))
        self.plant(p, grip)
        return p

    def perch(self, f, n=90):
        return self.idle(f, n, grip=38.0)

    def fly(self, f, n=20):
        t = TAU * f / n
        p = self.pose()
        p.hips_offset = SCALE * V((0, 0, 0.03 * math.sin(t - 1.2)))
        pitch = 30.0
        p.rel["hips"] = rx(pitch + 2 * math.sin(t))
        p.rel["chest"] = rx(-4 * math.sin(t))
        p.rel["neck_1"] = rx(-0.3 * pitch)
        p.rel["neck_2"] = rx(-0.3 * pitch)
        p.rel["head"] = rx(-0.4 * pitch + 3 * math.sin(t + 0.5))
        self.wings(p, spread=1.0, flap=10 + 38 * math.sin(t), lag=18 * math.cos(t), world=True)
        p.rel["tail"] = rx(-18 + 4 * math.sin(t))
        for s, _ in SIDES:
            p.rel[f"leg_upper_{s}"] = rx(58)
            p.rel[f"leg_lower_{s}"] = rx(45)
        return p

    def eat(self, f, n=60):
        k = env(f, 0, 12, 46, 60)
        bite = sum(env(f, c - 4, c - 1, c, c + 4) for c in (22, 31, 40))
        p = self.pose()
        p.rel["spine"] = rx(12 * k)
        p.rel["chest"] = rx(18 * k)
        p.rel["neck_1"] = rx(12 * k)
        p.rel["neck_2"] = rx(10 * k + 4 * bite)
        p.rel["head"] = rx(14 * k - 14 * bite)
        self.wings(p, lift=6 * k)
        p.rel["tail"] = rx(-10 * k)
        self.plant(p)
        return p

    def happy(self, f, n=45):
        crouch = env(f, 0, 6, 6, 10)
        jump = env(f, 8, 15, 15, 23)
        land = env(f, 21, 26, 27, 36)
        z = -0.04 * crouch + 0.2 * jump - 0.03 * land
        sp = env(f, 5, 11, 26, 34)
        p = self.pose()
        p.hips_offset = SCALE * V((0, 0, z))
        p.rel["chest"] = rx(-6 * jump + 5 * crouch)
        self.look(p, 0.0, -12 * env(f, 6, 14, 22, 36), 10 * math.sin(TAU * f / 15) * sp)
        self.wings(p, spread=0.8 * sp, flap=sp * (25 + 30 * math.sin(TAU * f / 10)),
                   lag=12 * sp * math.cos(TAU * f / 10))
        p.rel["tail"] = rx(-15 * sp) @ rz(12 * math.sin(TAU * f / 8) * sp)
        for s, _ in SIDES:
            tgt = self.toe0[s] + SCALE * V((0, 0, max(0.0, z)))
            p.limb_ik(f"leg_upper_{s}", f"leg_lower_{s}", tgt, Y)
        return p

    def refuse(self, f, n=36):
        e = env(f, 0, 6, 26, 36)
        sh = env(f, 3, 8, 24, 32)
        ang = 32.0 * math.sin(TAU * (f - 4) / 11) * sh
        p = self.pose()
        p.rel["spine"] = rx(-4 * e)
        p.rel["chest"] = rx(-6 * e)
        self.look(p, ang, -6 * e)
        self.wings(p, spread=0.06 * e, lift=10 * e)
        p.rel["tail"] = rz(14 * e * math.sin(TAU * f / 9))
        self.plant(p)
        return p

    def sleep(self, f, n=90):
        t = TAU * f / n
        b = math.sin(t)
        p = self.pose()
        p.hips_offset = SCALE * V((0, 0, -0.045 + 0.004 * b))
        p.rel["spine"] = rx(5 + 0.8 * b)
        p.rel["chest"] = rx(10 + 0.8 * b)
        p.rel["neck_1"] = rx(14)
        p.rel["neck_2"] = rx(16)
        p.rel["head"] = rx(24 + 1.5 * math.sin(t + 0.8))
        self.wings(p, lift=-3.0 + 2 * b)
        p.rel["tail"] = rx(6)
        self.plant(p)
        return p

    def look_clip(self, f, n=60):
        yaw = 100 * env(f, 3, 13, 22, 28) - 100 * env(f, 28, 38, 46, 56)
        p = self.pose()
        self.look(p, yaw, 0.0, 6 * math.sin(TAU * f / 30) * env(f, 3, 13, 46, 56))
        self.wings(p)
        self.plant(p)
        return p

    def all(self):
        C = rb.Clip
        return [C("idle", 90, True, self.idle), C("perch", 90, True, self.perch),
                C("fly", 20, True, self.fly), C("eat", 60, False, self.eat),
                C("happy", 45, False, self.happy), C("refuse", 36, False, self.refuse),
                C("sleep", 90, True, self.sleep), C("look", 60, False, self.look_clip)]


def build_scaled(k):
    """build() in v1 units, then the whole mesh scaled by SCALE about the origin (feet)."""
    build(k)
    for v in k.mb.bm.verts:
        v.co *= SCALE


if __name__ == "__main__":
    S = SCALE
    kit = nk.GKit(ASSET, COLORS, br.Rig(scaled_p(P)))
    nk.run(kit, build_scaled, lambda k: OwlClips(k.rig).all(),
           dict(loco=("fly", 0), center_z=0.5 * S, height_m=2.2, big_scale=1.7 * S,
                close_scale=1.4 * S, night_pose=("idle", 20), loco_z=0.45 * S),
           debug=dict(center_z=0.5 * S, head_pt=(0, -0.1 * S, 0.78 * S), head_scale=0.8 * S,
                      strip_scale=1.5 * S))
