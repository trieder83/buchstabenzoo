"""bat — Fledermaus, a friendly fruit bat (ART-ANIMALS "Night animals"; own `bat` rig).

Run:  blender -b --factory-startup --python tools/blender/animals/bat.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (not committed)

Writes assets/blender/animals/bat.blend, assets/models/animals/bat.glb,
assets/textures/animals/bat_body.png, art/animals/bat/model_preview.png.

Look: art/animals/bat/{front,side,back,three_quarter}.png (sheet_v2). Perched upright on
its feet, dog-like round brown head with a short snout and pink nose pad, big tall ears
(pink inside), big dark friendly eyes, a fluffy orange-brown collar with a point at the
chest, pear-shaped brown body, dark-brown wing membranes folded at the sides like a cloak
(thumbs at the collar, finger ribs), small feet. Perched height 0.8 m incl. ears (Q-143).

Skeleton `bat` (20 joints; Blender space as the other animal rigs, faces -Y = glTF +Z):

    root                               ground / grip point, never animated
    └─ hips                            pelvis; the only joint with translation
       ├─ spine ─ chest                chest carries the neck and the wings
       │          ├─ neck ─ head ─ ear_l, ear_r
       │          ├─ wing_upper_l ─ wing_lower_l ─ wing_hand_l   (arm, forearm, fingers:
       │          └─ wing_upper_r ─ wing_lower_r ─ wing_hand_r    the membrane is skinned
       ├─ leg_upper_l ─ leg_lower_l ─ foot_l                       along them)
       └─ leg_upper_r ─ leg_lower_r ─ foot_r

Origin at the feet when perched (min Y = 0 in the rest pose). `fly` is authored in place
(no root motion), the body lifted 0.12 m by `hips`; the game raises the origin by
`fly_height` (1.5 m, animal_anims.toml) while following at 1.4 m/s. `hang` is upside down
with the feet at the origin: the game puts the origin at the grip point (branch / bridge
underside) and the bat hangs 0.8 m below it. Wingspan in `fly` ≈ 1.1 m.

Clips: idle, perch, hang, fly, eat, happy, refuse, sleep (wrapped in its wings, head bowed).
Eye caps: material `eye_glow` (NIGHT-006).
"""

import math
import os
import sys

import numpy as np
from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
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
ASSET = "bat"

COLORS = {
    "fur": "#8A5A3A",
    "belly": "#9A6848",
    "collar": "#D9873E",
    "wing": "#4A3328",
    "rib": "#6B4A38",
    "wing_dark": "#35241C",
    "ear_in": "#C98B7A",
    "nose": "#B87A6A",
    "eye_white": "#FFFFFF",
    "iris": "#3A2618",
    "pupil": "#160F0B",
    "ink": "#241A14",
}

# left wing span line (shoulder at the collar -> wing tip near the ground)
SPAN = [(0.10, -0.035, 0.445), (0.135, -0.02, 0.35), (0.155, -0.005, 0.25), (0.17, 0.01, 0.15),
        (0.18, 0.03, 0.06)]
CHORD = V((0.5, 0.85, 0.0)).normalized()     # leading (front) -> trailing (back) edge
NORMAL = V((0.85, -0.5, 0.0)).normalized()   # outward face of the folded left wing
WIDTH = [0.09, 0.18, 0.25, 0.22, 0.20]

JOINTS = [("root", None, (0, 0, 0), (0, 0, 0.1)),
          ("hips", "root", (0, 0.01, 0.20), (0, 0.0, 0.30)),
          ("spine", "hips", (0, 0.0, 0.30), (0, 0.0, 0.39)),
          ("chest", "spine", (0, 0.0, 0.39), (0, -0.01, 0.46)),
          ("neck", "chest", (0, -0.01, 0.46), (0, -0.02, 0.51)),
          ("head", "neck", (0, -0.02, 0.51), (0, -0.02, 0.68))]
for _s, _sg in SIDES:
    _m = lambda p, sg=_sg: (sg * p[0], p[1], p[2])  # noqa: E731
    JOINTS.append((f"ear_{_s}", "head", _m((0.075, 0.0, 0.63)), _m((0.14, 0.02, 0.80))))
for _s, _sg in SIDES:
    _m = lambda p, sg=_sg: (sg * p[0], p[1], p[2])  # noqa: E731
    JOINTS += [(f"wing_upper_{_s}", "chest", _m(SPAN[0]), _m(SPAN[1])),
               (f"wing_lower_{_s}", f"wing_upper_{_s}", _m(SPAN[1]), _m(SPAN[3])),
               (f"wing_hand_{_s}", f"wing_lower_{_s}", _m(SPAN[3]), _m(SPAN[4]))]
for _s, _sg in SIDES:
    _m = lambda p, sg=_sg: (sg * p[0], p[1], p[2])  # noqa: E731
    JOINTS += [(f"leg_upper_{_s}", "hips", _m((0.065, 0.0, 0.15)), _m((0.07, -0.005, 0.085))),
               (f"leg_lower_{_s}", f"leg_upper_{_s}", _m((0.07, -0.005, 0.085)), _m((0.07, 0.0, 0.03))),
               (f"foot_{_s}", f"leg_lower_{_s}", _m((0.07, 0.0, 0.03)), _m((0.07, -0.07, 0.0)))]
JOINT_NAMES = [j[0] for j in JOINTS]


class Rig(rb.Skeleton):
    def __init__(self):
        self.P = None
        super().__init__(JOINTS)


def build(k):
    pal = k.pal
    at = k.atlas

    def body_col(x, y, z, u, v):
        c = pal.fill(len(x), "fur")
        c[(y < -0.06) & (z < 0.40) & (z > 0.12)] = pal("belly")
        return c
    body = ellipsoid((0, 0.0, 0.29), Z, FWD, 0.19, 0.18, 0.15, 0.13, 0.13, rings=8, n=14, lat=X)
    k.add(body, lambda co: rb.chain(co.z, [(0.17, "hips"), (0.28, "spine"), (0.40, "chest")]),
          region="torso", color=body_col)
    head = ellipsoid((0, -0.02, 0.565), Z, FWD, 0.12, 0.12, 0.15, 0.13, 0.12, rings=7, n=14, lat=X)
    k.add(head, lambda co: rb.chain(co.z, [(0.46, "chest"), (0.49, "neck"), (0.53, "head")]),
          cell="fur")
    snout = ellipsoid((0, -0.135, 0.525), FWD, Z, 0.03, 0.05, 0.062, 0.048, 0.042, rings=4, n=10)
    k.add(snout, "head", cell="fur")
    k.add(ellipsoid((0, -0.185, 0.535), FWD, Z, 0.012, 0.012, 0.036, 0.024, 0.022, rings=3, n=8),
          "head", cell="nose")
    for _, sg in SIDES:
        nk.eye_at(k, V((sg * 0.064, -0.132, 0.59)), V((sg * 0.38, -1, 0.12)), V((-sg, 0, 0)),
                  (0.043, 0.047), 0.018)
    k.eye_swatch(iris="iris", iris_r=0.72, pupil_r=0.46, look=0.05)
    for _, sg in SIDES:
        k.ear(sg, (0.075, 0.0, 0.63), (0.14, 0.02, 0.80), 0.065, 0.018, V((0.25, -1, 0.0)), "fur",
              "ear_in", prof=[(-0.1, 0.8, 0.9), (0.15, 1.0, 1.0), (0.45, 0.9, 0.9), (0.75, 0.6, 0.8),
                              (0.95, 0.2, 0.5)], inner_frac=0.62, inner_t=(0.08, 0.88))
    # fluffy collar (scalloped lower edge) + the point at the chest
    collar = ellipsoid((0, -0.005, 0.455), Z, FWD, 0.08, 0.05, 0.175, 0.16, 0.155, rings=5, n=20,
                       lat=X, mod=lambda i, th: 1 + (0.12 * math.cos(10 * th) if i <= 2 else 0.0))
    k.add(collar, lambda co: rb.chain(co.z, [(0.44, "chest"), (0.50, "neck")]), cell="collar")
    k.add(nk.cone((0, -0.125, 0.43), (0, -0.25, -1), 0.05, 0.075, n=6, up=FWD), "chest", cell="collar")

    # wing membranes: U = span (shoulder -> tip), V = chord (leading 0 -> trailing 1)
    def wing_col(U, Vv):
        c = np.broadcast_to(pal("wing"), U.shape + (3,)).copy()
        c[Vv < 0.07] = pal("rib")                          # forearm along the leading edge
        for v0, u0 in ((0.32, 0.28), (0.58, 0.42), (0.84, 0.55)):
            # finger ribs fanning from the wrist towards the trailing edge
            vv = v0 * np.clip((U - u0 + 0.35) / 0.9, 0.0, 1.0) + 0.04
            c[(np.abs(Vv - vv) < 0.022) & (U > u0 - 0.25)] = pal("rib")
        c[(Vv > 0.9 - 0.06 * np.abs(np.sin(U * math.pi * 3)))] = pal("wing_dark")
        return c
    k.paint_region("extra", wing_col)
    for side, sg in SIDES:
        m = lambda p: V((sg * p[0], p[1], p[2]))  # noqa: E731
        ch = V((sg * CHORD.x, CHORD.y, CHORD.z))
        nrm = V((sg * NORMAL.x, NORMAL.y, NORMAL.z))
        pts = [m(p) + ch * (0.5 * w) for p, w in zip(SPAN, WIDTH)]
        radii = [(0.012, 0.5 * w, 0.5 * w) for w in WIDTH]
        tube = Tube(pts, radii, 10, up=ch, lat=nrm, cap=(0.02, 0.03))
        n = tube.n

        def uv(i, kk, co, tube=tube, n=n):
            U = 0.0 if i < 0 else (1.0 if i >= tube.R else tube.U[i + tube.off])
            return at.map_uv("extra", 0.02 + 0.96 * U, 0.5 + 0.5 * math.cos(TAU * kk / n))
        stops = [(0.0, "chest"), (0.07, f"wing_upper_{side}"), (0.22, f"wing_upper_{side}"),
                 (0.34, f"wing_lower_{side}"), (0.62, f"wing_lower_{side}"), (0.78, f"wing_hand_{side}")]
        k.mb.loft(tube.rings, uv, lambda co, tube=tube: rb.chain(tube.param(co), stops),
                  pole_start=tube.pole0, pole_end=tube.pole1)
        # thumb claw at the collar
        k.add(nk.cone(m(SPAN[0]) + V((0, -0.02, 0.0)), V((-sg * 0.4, -0.6, -0.4)), 0.018, 0.045, n=5),
              f"wing_upper_{side}", cell="wing_dark")
    # legs and small feet
    for side, sg in SIDES:
        m = lambda p: V((sg * p[0], p[1], p[2]))  # noqa: E731
        leg = Tube([m((0.065, 0.0, 0.16)), m((0.07, -0.005, 0.085)), m((0.07, 0.0, 0.04))],
                   [(0.04,) * 3, (0.032,) * 3, (0.028,) * 3], 7, up=FWD, lat=X, tangent=-Z,
                   cap=(0.02, 0.01))
        k.add(leg, lambda co, side=side: rb.chain(co.z, [(0.06, f"leg_lower_{side}"),
                                                        (0.11, f"leg_upper_{side}")]), cell="fur")
        k.add(ellipsoid(m((0.07, -0.03, 0.02)), FWD, Z, 0.03, 0.05, 0.035, 0.022, 0.02, rings=4, n=8),
              f"foot_{side}", cell="fur")


# --------------------------------------------------------------------------- clips

class BatClips:
    def __init__(self, rig):
        self.sk = rig
        self.ank = {s: rig.rest_head[f"foot_{s}"].copy() for s, _ in SIDES}
        self.W = nk.Wings(rig, {s: [f"wing_upper_{s}", f"wing_lower_{s}", f"wing_hand_{s}"]
                                for s, _ in SIDES}, tuple(NORMAL), (1, 0.08, 0))

    def pose(self):
        return rb.Pose(self.sk)

    def plant(self, p, grip=0.0, lift=0.0):
        for s, _ in SIDES:
            p.limb_ik(f"leg_upper_{s}", f"leg_lower_{s}", self.ank[s] + V((0, 0, lift)), FWD)
            p.set_world(f"foot_{s}", rx(grip))

    def ears(self, p, back=0.0, fl=0.0, fr=0.0):
        p.rel["ear_l"] = ry(10 * fl) @ rx(-back - 20 * fl)
        p.rel["ear_r"] = ry(-10 * fr) @ rx(-back - 20 * fr)

    def look(self, p, yaw, pitch=0.0, tilt=0.0):
        p.rel["neck"] = rz(0.4 * yaw) @ rx(0.4 * pitch) @ p.rel["neck"]
        p.rel["head"] = rz(0.6 * yaw) @ rx(0.6 * pitch) @ ry(tilt) @ p.rel["head"]

    def idle(self, f, n=90, grip=0.0):
        t = TAU * f / n
        b = math.sin(2 * t)
        p = self.pose()
        p.rel["spine"] = rx(0.8 * b)
        p.rel["chest"] = rx(-1.0 * b)
        self.look(p, 18 * math.sin(t), 3 * math.sin(2 * t + 1), 7 * math.sin(t + 1.4))
        self.W.apply(p, lift=2.5 * b, wrap=1.5 * b)
        self.ears(p, back=3 * math.sin(t), fl=env(f, 60, 63, 64, 70), fr=env(f, 26, 29, 31, 37))
        self.plant(p, grip)
        return p

    def perch(self, f, n=90):
        return self.idle(f, n, grip=38.0)

    def hang(self, f, n=90):
        """Upside down, feet gripping at the origin (the game puts the origin at the grip
        point), wings wrapped, gentle sway, head looking around."""
        t = TAU * f / n
        p = self.pose()
        p.rel["hips"] = ry(180)
        p.hips_offset = V((0, 0, -0.40))
        p.rel["spine"] = rx(1.2 * math.sin(2 * t)) @ ry(2.5 * math.sin(t))
        p.rel["chest"] = ry(2.0 * math.sin(t + 0.6))
        # upside down: a positive pitch lifts the face towards the ground
        self.look(p, 20 * math.sin(t), -8 + 3 * math.sin(2 * t), 6 * math.sin(t + 1))
        self.W.apply(p, lift=-4.0, wrap=14.0 + 1.5 * math.sin(2 * t))
        self.ears(p, back=4 * math.sin(t), fl=env(f, 40, 43, 44, 50))
        for s, _ in SIDES:
            p.rel[f"leg_upper_{s}"] = rx(0)
            p.set_world(f"foot_{s}", ry(180) @ rx(-45))
        return p

    def fly(self, f, n=16):
        t = TAU * f / n
        p = self.pose()
        pitch = 66.0
        p.hips_offset = V((0, 0, 0.12 + 0.025 * math.sin(t - 1.3)))
        p.rel["hips"] = rx(pitch + 3 * math.sin(t))
        p.rel["chest"] = rx(-3 * math.sin(t))
        p.rel["neck"] = rx(-0.5 * pitch)
        p.rel["head"] = rx(-0.35 * pitch + 3 * math.sin(t + 0.4))
        self.W.apply(p, spread=1.0, flap=8 + 36 * math.sin(t), lag=14 * math.cos(t), world=True)
        self.ears(p, back=25)
        for s, _ in SIDES:
            p.rel[f"leg_upper_{s}"] = rx(-15)
            p.rel[f"foot_{s}"] = rx(-25)
        return p

    def eat(self, f, n=60):
        k = env(f, 0, 12, 46, 60)
        bite = sum(env(f, c - 4, c - 1, c, c + 4) for c in (22, 31, 40))
        p = self.pose()
        p.rel["chest"] = rx(8 * k)
        self.look(p, 0.0, 22 * k - 10 * bite, 6 * k)
        self.W.apply(p, lift=4 * k, wrap=6 * k)
        self.ears(p, back=-6 * k)
        self.plant(p)
        return p

    def happy(self, f, n=45):
        crouch = env(f, 0, 6, 6, 10)
        jump = env(f, 8, 15, 15, 23)
        land = env(f, 21, 26, 27, 36)
        z = -0.03 * crouch + 0.16 * jump - 0.02 * land
        sp = env(f, 5, 11, 26, 34)
        p = self.pose()
        p.hips_offset = V((0, 0, z))
        p.rel["chest"] = rx(-5 * jump + 4 * crouch)
        self.look(p, 0.0, -10 * env(f, 6, 14, 22, 36), 10 * math.sin(TAU * f / 15) * sp)
        self.W.apply(p, spread=0.55 * sp, flap=sp * (20 + 30 * math.sin(TAU * f / 9)),
                     lag=10 * sp * math.cos(TAU * f / 9))
        self.ears(p, back=-8 * sp)
        self.plant(p, lift=max(0.0, z))
        return p

    def refuse(self, f, n=36):
        e = env(f, 0, 6, 26, 36)
        sh = env(f, 3, 8, 24, 32)
        ang = 28.0 * math.sin(TAU * (f - 4) / 11) * sh
        p = self.pose()
        p.rel["spine"] = rx(-4 * e)
        p.rel["chest"] = rx(-5 * e)
        self.look(p, ang, -5 * e)
        self.W.apply(p, wrap=10 * e, lift=-2 * e)
        self.ears(p, back=30 * e)
        self.plant(p)
        return p

    def sleep(self, f, n=90):
        t = TAU * f / n
        b = math.sin(t)
        p = self.pose()
        p.hips_offset = V((0, 0, -0.03 + 0.003 * b))
        p.rel["spine"] = rx(6 + 0.8 * b)
        p.rel["chest"] = rx(8 + 0.8 * b)
        p.rel["neck"] = rx(22)
        p.rel["head"] = rx(26 + 1.5 * math.sin(t + 0.8))
        self.W.apply(p, lift=-4.0, wrap=14.0 + 1.0 * b)
        self.ears(p, back=30)
        self.plant(p)
        return p

    def all(self):
        C = rb.Clip
        return [C("idle", 90, True, self.idle), C("perch", 90, True, self.perch),
                C("hang", 90, True, self.hang), C("fly", 16, True, self.fly),
                C("eat", 60, False, self.eat), C("happy", 45, False, self.happy),
                C("refuse", 36, False, self.refuse), C("sleep", 90, True, self.sleep)]


if __name__ == "__main__":
    kit = nk.GKit(ASSET, COLORS, Rig())
    nk.run(kit, build, lambda k: BatClips(k.rig).all(),
           dict(loco=("fly", 0), center_z=0.4, height_m=2.2, big_scale=1.35, close_scale=1.1,
                night_pose=("idle", 20), loco_z=0.4),
           debug=dict(center_z=0.4, head_pt=(0, -0.08, 0.58), head_scale=0.55, strip_scale=1.2))
