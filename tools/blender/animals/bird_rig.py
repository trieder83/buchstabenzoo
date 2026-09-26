"""Shared `bird` rig + duck builder for the ambient water birds (duck, duckling; GAME-AMBIENT).

Skeleton (16 joints):

    root                          WATER SURFACE point (waterline) below the body, never animated
    └─ hips                       body centre; the only joint with translation (bob, rise)
       ├─ spine ─ chest           chest carries the neck and the wings
       │          ├─ neck_1 ─ neck_2 ─ head        (head: eyes + bill, rigid)
       │          ├─ wing_upper_l ─ wing_lower_l   folded along the body side
       │          └─ wing_upper_r ─ wing_lower_r
       ├─ tail                    upturned tail tip
       ├─ leg_upper_l ─ leg_lower_l                shank (hip -> ankle) and webbed foot
       └─ leg_upper_r ─ leg_lower_r

Space: Blender Z up, the bird faces -Y (glTF +Z), left = +X (as the other animal rigs).
**Origin convention (like the goldfish):** the model origin (root) is the point of the
WATER SURFACE under the body centre. The body floats with its bottom a few cm below
z = 0, legs and feet hang under water; the game puts the origin at the water height and
never sinks the model. No root motion: the game moves / turns the bird (`swim` speed in
animal_anims.toml); `flap` and `dip` translate `hips` vertically only.

All positions are authored in "duck units" (adult duck ~0.45 m) and multiplied by the
species scale `S` (duckling ~0.5), so both share one set of clips.

`path_loft`, `eye_loft`, `paint_eye` are reused by frog.py.
"""

import math
import os
import sys

import numpy as np
from mathutils import Quaternion, Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rig_base as rb  # noqa: E402

zb = rb.zb
FPS = rb.FPS
SIDES = rb.SIDES
X, Y, Z = rb.X, rb.Y, rb.Z
rx, ry, rz = rb.rx, rb.ry, rb.rz
env = rb.envelope
TAU = rb.TAU

JOINTS = [("root", None), ("hips", "root"), ("spine", "hips"), ("chest", "spine"),
          ("neck_1", "chest"), ("neck_2", "neck_1"), ("head", "neck_2"),
          ("wing_upper_l", "chest"), ("wing_lower_l", "wing_upper_l"),
          ("wing_upper_r", "chest"), ("wing_lower_r", "wing_upper_r"),
          ("tail", "hips"),
          ("leg_upper_l", "hips"), ("leg_lower_l", "leg_upper_l"),
          ("leg_upper_r", "hips"), ("leg_lower_r", "leg_upper_r")]
JOINT_NAMES = [n for n, _ in JOINTS]
PARENT = dict(JOINTS)


def V(*a):
    return Vector(a)


class Rig(rb.Skeleton):
    """P (Blender metres, left side for paired joints): hips, spine, chest, neck_1, neck_2,
    head, head_end, tail (base, tip), wing (base, mid, tip), leg (hip, ankle, toe)."""

    def __init__(self, P):
        self.P = P
        m = rb.mirror
        j = [("root", None, (0, 0, 0), (0, 0, 0.05)),
             ("hips", "root", P["hips"], P["spine"]),
             ("spine", "hips", P["spine"], P["chest"]),
             ("chest", "spine", P["chest"], P["neck_1"]),
             ("neck_1", "chest", P["neck_1"], P["neck_2"]),
             ("neck_2", "neck_1", P["neck_2"], P["head"]),
             ("head", "neck_2", P["head"], P["head_end"])]
        wb, wm, wt = P["wing"]
        for s, sg in SIDES:
            j += [(f"wing_upper_{s}", "chest", m(wb, sg), m(wm, sg)),
                  (f"wing_lower_{s}", f"wing_upper_{s}", m(wm, sg), m(wt, sg))]
        j.append(("tail", "hips", *P["tail"]))
        lh, la, lt = P["leg"]
        for s, sg in SIDES:
            j += [(f"leg_upper_{s}", "hips", m(lh, sg), m(la, sg)),
                  (f"leg_lower_{s}", f"leg_upper_{s}", m(la, sg), m(lt, sg))]
        assert [n for n, *_ in j] == JOINT_NAMES
        super().__init__(j)


# --------------------------------------------------------------------------- mesh helpers

def path_loft(mb, pts, radii, uv_fn, weight_fn, n, lat=X, pole_start=None, pole_end=None,
              face_uv=None):
    """Loft along a polyline; radii[i] = (r_up, r_down, r_lat). Ring frames: lateral axis
    `lat` (made perpendicular to the path), up = lat x tangent. pole_*: True = automatic
    cap point, a Vector = that point, None = open end."""
    rings = []
    m = len(pts)
    for i, (p, (ru, rd, rl)) in enumerate(zip(pts, radii)):
        a = (pts[min(i + 1, m - 1)] - pts[max(i - 1, 0)]).normalized()
        li = lat[i] if isinstance(lat, (list, tuple)) else lat
        la = (li - a * a.dot(li)).normalized()
        up = la.cross(a).normalized()
        rings.append(rb.ring(p, up, la, ru, rl, n, r_down=rd))
    if pole_start is True:
        t0 = (pts[1] - pts[0]).normalized()
        pole_start = pts[0] - t0 * min(radii[0]) * 0.7
    if pole_end is True:
        t1 = (pts[-1] - pts[-2]).normalized()
        pole_end = pts[-1] + t1 * min(radii[-1]) * 0.7
    return mb.loft(rings, uv_fn, weight_fn, pole_start=pole_start, pole_end=pole_end,
                   face_uv=face_uv)


def ellipsoid(mb, c, rf, ru, rl, n, rings_n, uv_fn, weight_fn, axis=None, face_uv=None):
    """Ellipsoid lofted along `axis` (default -Y = forward): rf along the axis."""
    axis = (axis or V(0, -1, 0)).normalized()
    pts, radii = [], []
    for i in range(rings_n):
        t = -0.92 + 1.84 * i / (rings_n - 1)          # back (-) .. front (+) along axis
        k = math.sqrt(max(1 - t * t, 0.0))
        pts.append(c + axis * (t * rf))
        radii.append((ru * k, ru * k, rl * k))
    pts, radii = pts[::-1], radii[::-1]              # front -> back
    return path_loft(mb, pts, radii, uv_fn, weight_fn, n,
                     pole_start=c + axis * rf, pole_end=c - axis * rf, face_uv=face_uv)


def eye_loft(mb, at, c, f, ex, R, bone, region="eye", n=10, bulge=0.6):
    """Painted eye cap: rings along the outward normal f around the surface point c.
    ex: in-plane direction that maps to +U (the eye's 'front'). R = (r_u, r_v).
    Rings behind the widest one map to the rim of the eye region (ink / lid)."""
    f = f.normalized()
    ex = (ex - f * f.dot(ex)).normalized()
    ey = f.cross(ex).normalized()
    if ey.dot(Z) < 0:
        ey = -ey
    d0 = max(R) * bulge
    rings = [rb.ring(c + f * off, ey, ex, R[1] * sc, R[0] * sc, n)
             for off, sc in ((-d0 * 0.9, 0.80), (-d0 * 0.35, 1.0), (d0 * 0.2, 0.92),
                             (d0 * 0.55, 0.58))]

    def uv(i, k, co):
        d = co - c
        u, v = d.dot(ex) / R[0], d.dot(ey) / R[1]
        if i <= 0:
            r = max(math.hypot(u, v), 1e-6)
            u, v = u / r * 0.99, v / r * 0.99
        return at.map_uv(region, 0.5 + 0.5 * u, 0.5 + 0.5 * v)

    return mb.loft(rings, uv, rb.rigid(bone), pole_start=c - f * d0 * 1.4,
                   pole_end=c + f * d0 * 0.72)


def rgb(colors, name):
    return np.array(rb.hr.hex_rgb(colors[name]), np.float32)


def paint_eye(colors, iris=None, pupil_r=0.40, pupil_at=(0.10, 0.06), rim="ink",
              outer=None, iris_r=0.62):
    """Comic eye: ink rim, white (or `iris` colour ring), big pupil, two highlights.
    outer: colour beyond the rim (lid / skin) for eyes that are whole balls."""
    def paint(U, Vv):
        x, y = 2 * U - 1, 2 * Vv - 1
        c = np.broadcast_to(rgb(colors, rim), U.shape + (3,)).copy()
        r = np.hypot(x, y)
        if outer:
            c[r > 0.97] = rgb(colors, outer)
        inner = r < 0.86
        c[inner] = rgb(colors, "eye_white")
        px, py = pupil_at
        if iris:
            c[inner & (np.hypot(x - px, y - py) < iris_r)] = rgb(colors, iris)
        c[inner & (np.hypot(x - px, y - py) < pupil_r)] = rgb(colors, "pupil")
        c[np.hypot(x - px + 0.16, y - py - 0.16) < 0.13] = rgb(colors, "eye_white")
        c[np.hypot(x - px - 0.12, y - py + 0.14) < 0.06] = rgb(colors, "eye_white")
        return c
    return paint


# --------------------------------------------------------------------------- duck builder

class Duck:
    """Builds a duck-like bird from a species dict D (duck units, scaled by D['S'])."""

    def __init__(self, D):
        self.D = D
        s = D["S"]
        self.s = s
        sc = lambda p: tuple(c * s for c in p)  # noqa: E731
        self.sc = sc
        self.P = {k: sc(D["joints"][k]) for k in
                  ("hips", "spine", "chest", "neck_1", "neck_2", "head", "head_end")}
        self.P["tail"] = tuple(sc(p) for p in D["joints"]["tail"])
        self.wing_path = self._wing_path()
        wp = self.wing_path[0]
        self.P["wing"] = (tuple(wp[0]), tuple(wp[D["wing"]["mid"]]), tuple(wp[-1]))
        self.P["leg"] = tuple(sc(p) for p in (D["leg"]["hip"], D["leg"]["ankle"],
                                              D["leg"]["toe"]))
        self.rig = Rig(self.P)

    def surface(self, y, th):
        """Body surface point (duck units) at y and ring angle th (0 = top, + = left), and
        the unit tangent of the cross-section towards larger th."""
        prof = self.D["body"]
        y = min(max(y, prof[0][0]), prof[-1][0])
        for (y0, *a), (y1, *b) in zip(prof, prof[1:]):
            if y0 <= y <= y1:
                k = (y - y0) / (y1 - y0)
                zc, ru, rd, rl = [u + (v - u) * k for u, v in zip(a, b)]
                break
        cu = math.cos(th)
        r = ru if cu >= 0 else rd
        p = V(math.sin(th) * rl, y, zc + cu * r)
        t = V(math.cos(th) * rl, 0, -math.sin(th) * r).normalized()
        return p, t

    def _wing_path(self):
        """Left wing centre line on the body side: [(y, th_deg, out)] in D['wing']['path'];
        returns (points in metres, per-point width directions (down the body side))."""
        s = self.s
        pts, lats = [], []
        for y, th, out in self.D["wing"]["path"]:
            p, t = self.surface(y, math.radians(th))
            nrm = V(-t.z, 0, t.x)  # outward normal of the cross-section (left side)
            pts.append((p + nrm * out) * s)
            lats.append(t)
        return pts, lats

    # ---- weights (duck units -> compare against scaled thresholds)
    def w_body(self, co):
        s = self.s
        return rb.chain(co.y / s, self.D["body_weights"])

    def w_neck(self, co):
        return rb.chain(co.z / self.s, self.D["neck_weights"])

    # ---- parts
    def build(self, mb, at):
        D, s = self.D, self.s
        cream = at.cell_uv("body")
        # body: front (-Y) -> tail tip
        pts = [V(0, y * s, z * s) for y, z, *_ in D["body"]]
        radii = [(ru * s, rd * s, rl * s) for _, _, ru, rd, rl in D["body"]]
        ty, tz = D["tail_pole"]
        path_loft(mb, pts, radii, lambda i, k, co: cream, self.w_body, D.get("body_n", 14),
                  pole_start=True, pole_end=V(0, ty * s, tz * s))
        # neck (bottom -> top), hidden ends
        npts = [V(0, y * s, z * s) for y, z, _ in D["neck"]]
        nr = [(r * s, r * s, r * s * 1.05) for *_, r in D["neck"]]
        path_loft(mb, npts, nr, lambda i, k, co: cream, self.w_neck, 10, lat=X)
        # head
        hy, hz = D["head"]
        rf, ru, rl = D["head_r"]
        Hc = V(0, hy * s, hz * s)
        ellipsoid(mb, Hc, rf * s, ru * s, rl * s, 12, D.get("head_rings", 7),
                  lambda i, k, co: cream, rb.rigid("head"))
        self.build_bill(mb, at, Hc)
        self.build_eyes(mb, at, Hc)
        self.build_wings(mb, at)
        self.build_legs(mb, at)
        for tuft in D.get("tufts", []):
            b, t, r = tuft
            b, t = V(*b) * s, V(*t) * s
            mid = b.lerp(t, 0.5) + V(0, 0.004, 0) * s
            path_loft(mb, [b, mid], [(r * s,) * 3, (r * 0.55 * s,) * 3],
                      lambda i, k, co: cream, rb.rigid("head"), 5, lat=X, pole_end=t)
        for tf in D.get("tail_tufts", []):
            b, t, r = tf
            b, t = V(*b) * s, V(*t) * s
            path_loft(mb, [b, b.lerp(t, 0.45)], [(r * s,) * 3, (r * 0.6 * s,) * 3],
                      lambda i, k, co: cream, rb.rigid("tail"), 5, lat=X, pole_end=t)

    def build_bill(self, mb, at, Hc):
        D, s = self.D, self.s
        B = D["bill"]
        bill = at.cell_uv("bill")
        base = V(0, B["base"][0] * s, B["base"][1] * s)
        L = B["len"] * s
        d = V(0, -1, B.get("tilt", 0.0)).normalized()
        pts, radii = [], []
        for t, w, h, lift in B["sections"]:
            pts.append(base + d * (t * L) + Z * (lift * s))
            radii.append((h * s, h * s * 0.8, w * s))
        pts = pts[::-1]  # tip first (front -> back like the body)
        radii = radii[::-1]
        tip = pts[0] + d * (B.get("tip_round", 0.3) * radii[0][2])
        path_loft(mb, pts, radii, lambda i, k, co: bill, rb.rigid("head"), 10,
                  pole_start=tip, pole_end=base - d * 0.01 * s)

    def build_eyes(self, mb, at, Hc):
        D, s = self.D, self.s
        rf, ru, rl = [r * s for r in D["head_r"]]
        az, el = (math.radians(a) for a in D["eye"]["dir"])
        for _, sg in SIDES:
            u = V(sg * math.sin(az) * math.cos(el), -math.cos(az) * math.cos(el), math.sin(el))
            p = Hc + V(u.x * rl, u.y * rf, u.z * ru)
            nrm = V(u.x / rl, u.y / rf, u.z / ru).normalized()
            R = tuple(r * s for r in D["eye"]["r"])
            eye_loft(mb, at, p - nrm * 0.003 * s, nrm, V(0, -1, 0), R, "head", n=10,
                     bulge=D["eye"].get("bulge", 0.55))

    def build_wings(self, mb, at):
        D, s = self.D, self.s
        W = D["wing"]
        pts0, lats0 = self.wing_path
        mid = pts0[W["mid"]]
        widths = W["w"]
        m = len(pts0)
        n = 8
        # cumulative length for U and the upper/lower blend
        cum = [0.0]
        for a, b in zip(pts0, pts0[1:]):
            cum.append(cum[-1] + (b - a).length)
        Lw = cum[-1]
        tm = (mid - pts0[0]).length
        for side, sg in SIDES:
            pts = [V(p.x * sg, p.y, p.z) for p in pts0]
            radii = [(W["thick"] * s * (1 - 0.5 * i / (m - 1)), W["thick"] * s * 0.6,
                      w * s) for i, w in enumerate(widths)]
            # ring 'up' = lat x tangent must point outwards: lat = the width direction
            # (down the body side) for the left wing, mirrored + flipped for the right
            lat = [V(t.x, 0, t.z) if sg > 0 else V(t.x, 0, -t.z) for t in lats0]
            us = [c / Lw for c in cum]

            def uv(i, k, co, us=us):
                U = us[min(max(i, 0), m - 1)] if 0 <= i < m else (0.0 if i < 0 else 1.0)
                Vv = 0.5 + 0.5 * math.sin(2 * math.pi * k / n)
                return at.map_uv("wing", 0.02 + 0.96 * U, Vv)

            base = pts[0]

            def wfn(co, base=base, side=side):
                t = (co - base).length
                return rb.chain(t, [(tm * 0.55, f"wing_upper_{side}"),
                                    (tm * 1.1, f"wing_lower_{side}")])
            path_loft(mb, pts, radii, uv, wfn, n, lat=lat,
                      pole_start=True, pole_end=True)

    def build_legs(self, mb, at):
        D, s = self.D, self.s
        Lg = D["leg"]
        foot = at.cell_uv("foot")
        for side, sg in SIDES:
            m = lambda p: V(p[0] * sg, p[1], p[2]) * s  # noqa: E731
            hip, ank, toe = m(Lg["hip"]), m(Lg["ankle"]), m(Lg["toe"])
            r = Lg["r"] * s
            path_loft(mb, [hip, ank], [(r, r, r), (r * 0.8,) * 3], lambda i, k, co: foot,
                      rb.rigid(f"leg_upper_{side}"), 5, lat=Y, pole_end=True)
            # webbed foot: flat fan from the ankle to the toes
            d = toe - ank
            fw = Lg["foot_w"] * s
            fpts = [ank + d * t for t in (1.0, 0.7, 0.3, 0.02)]
            fr = [(0.006 * s, 0.004 * s, fw * 0.75), (0.007 * s, 0.005 * s, fw),
                  (0.007 * s, 0.005 * s, fw * 0.6), (0.006 * s, 0.005 * s, fw * 0.25)]
            path_loft(mb, fpts, fr, lambda i, k, co: foot, rb.rigid(f"leg_lower_{side}"), 6,
                      lat=X, pole_start=True, pole_end=True)


def paint_wing(colors):
    """U = wing base -> tip, V = across (0/1 = lower/upper edge; both faces share it).
    Scalloped feather rows (comic) + a darker trailing tip."""
    def paint(U, Vv):
        c = np.broadcast_to(rgb(colors, "wing"), U.shape + (3,)).copy()
        for u0 in (0.42, 0.68):
            edge = u0 + 0.07 * np.abs(np.sin(Vv * math.pi * 2.5))
            c[(U > edge) & (U < edge + 0.035) & (Vv > 0.12) & (Vv < 0.9)] = rgb(colors, "wing_line")
        c[U > 0.86 + 0.05 * np.abs(np.sin(Vv * math.pi * 2))] = rgb(colors, "wing_line")
        return c
    return paint


# --------------------------------------------------------------------------- clips

class BirdClips:
    """Clips for the bird rig; translations scale with S (amplitudes in duck units)."""

    def __init__(self, rig, S, speed=1.0):
        self.rig, self.S = rig, S
        self.k = speed  # >1: livelier (duckling)

    def pose(self):
        return rb.Pose(self.rig)

    def wings(self, p, spread=0.0, flap=0.0, lag=0.0, lift=(0.0, 0.0)):
        """spread 0 (folded) .. 1 (out to the side, flat); flap: + = up (deg, about the body
        axis); lag: lower-wing bend (deg); lift: extra raise of a folded wing (l, r)."""
        for (side, sg), lf in zip(SIDES, lift):
            Ws = rz(-90 * sg) @ ry(-90 * sg)
            Wl = Quaternion().slerp(Ws, spread)
            up = ry(-(flap + lf) * sg)
            w = up @ Wl
            p.rel[f"wing_upper_{side}"] = w
            # spread wings are straight: undo the rest fold between the two wing bones
            sk = self.rig
            straight = rb.rot_between(sk.rest_dir[f"wing_lower_{side}"],
                                      sk.rest_dir[f"wing_upper_{side}"])
            p.rel[f"wing_lower_{side}"] = (Wl.inverted() @ ry(-lag * sg) @ Wl
                                           @ Quaternion().slerp(straight, spread))

    def legs(self, p, t, amp=30.0, fold=35.0, phase=math.pi):
        """Paddling: power stroke back with the web open, recovery forward folded."""
        for (side, _), ph in zip(SIDES, (0.0, phase)):
            a = math.sin(t + ph)
            p.rel[f"leg_upper_{side}"] = rx(amp * a)
            p.rel[f"leg_lower_{side}"] = rx(fold * max(0.0, -math.cos(t + ph)))

    # ---- clips
    def swim(self, f, n=30):
        t = TAU * f / n
        S = self.S
        p = self.pose()
        p.rel["hips"] = ry(2.0 * math.sin(t)) @ rx(1.5 * math.sin(2 * t))
        p.hips_offset = V(0, 0, 0.006 * S * math.sin(2 * t + 0.6))
        p.rel["spine"] = rz(1.5 * math.sin(t))
        p.rel["neck_1"] = rx(-3 * math.sin(2 * t + 1.2))
        p.rel["neck_2"] = rx(2 * math.sin(2 * t + 1.6))
        p.rel["head"] = rx(1.5 * math.sin(2 * t + 2.0)) @ rz(-2 * math.sin(t))
        p.rel["tail"] = rz(8 * math.sin(t + 0.5))
        self.legs(p, t, amp=38.0)
        self.wings(p, lift=(1.5 * math.sin(2 * t), 1.5 * math.sin(2 * t)))
        return p

    def idle(self, f, n=90):
        t = TAU * f / n
        S = self.S
        p = self.pose()
        p.rel["hips"] = rz(5 * math.sin(t)) @ rx(2.0 * math.sin(2 * t + 0.5)) @ ry(1.5 * math.sin(3 * t))
        p.hips_offset = V(0, 0, 0.008 * S * math.sin(2 * t))
        look = 28 * math.sin(t) * (0.6 + 0.4 * math.sin(2 * t + 1))
        p.rel["neck_1"] = rz(look * 0.3) @ rx(2 * math.sin(2 * t + 1.0))
        p.rel["neck_2"] = rz(look * 0.35)
        p.rel["head"] = rz(look * 0.35) @ rx(4 * math.sin(3 * t))
        p.rel["tail"] = rz(14 * math.sin(6 * t) * max(0.0, math.sin(3 * t)) ** 2)
        self.legs(p, 2 * t, amp=12.0, fold=15.0)
        self.wings(p)
        return p

    def dip(self, f, n=60):
        """Head under water, tail up (upending), a few kicks, back up with a shake."""
        S = self.S
        e = env(f, 0, 13, 38, 52)
        p = self.pose()
        kick = env(f, 8, 14, 38, 44)
        shake = env(f, 50, 53, 56, 60) * math.sin(TAU * f / 4)
        p.rel["hips"] = rx(112 * e) @ rz(12 * shake)
        p.hips_offset = V(0, -0.02 * S * e, -0.035 * S * e + 0.012 * S * env(f, 48, 52, 54, 60))
        p.rel["spine"] = rx(8 * e)
        p.rel["neck_1"] = rx(22 * e)
        p.rel["neck_2"] = rx(18 * e)
        p.rel["head"] = rx(-10 * e) @ rz(20 * shake)
        p.rel["tail"] = rx(-25 * e) @ rz(18 * kick * math.sin(TAU * f / 6))
        tl = TAU * f / 9
        for side, ph in (("l", 0.0), ("r", math.pi)):
            a = math.sin(tl + ph) * kick
            p.rel[f"leg_upper_{side}"] = rx(-50 * e + 30 * a)
            p.rel[f"leg_lower_{side}"] = rx(30 * max(0.0, -math.cos(tl + ph)) * kick)
        self.wings(p, spread=0.12 * e, flap=6 * e)
        return p

    def flap(self, f, n=45):
        """Rises up on the water, chest up, three big wing beats, drops back (splash)."""
        S = self.S
        e = env(f, 0, 9, 27, 38)
        w = env(f, 2, 8, 29, 37)
        settle = env(f, 36, 39, 41, 45)
        p = self.pose()
        p.rel["hips"] = rx(-30 * e + 6 * settle)
        p.hips_offset = V(0, 0.01 * S * e, 0.035 * S * e - 0.015 * S * settle)
        p.rel["spine"] = rx(-8 * e)
        p.rel["neck_1"] = rx(20 * e)
        p.rel["neck_2"] = rx(10 * e)
        p.rel["head"] = rx(14 * e) @ rx(-6 * math.sin(TAU * f / 8) * w)
        beat = math.sin(TAU * (f - 4) / 8)
        self.wings(p, spread=w, flap=w * (18 + 52 * beat), lag=w * (-22 * math.cos(TAU * (f - 4) / 8)))
        p.rel["tail"] = rx(-15 * e) @ rz(10 * math.sin(TAU * f / 6) * e)
        tl = TAU * f / 8
        for side, ph in (("l", 0.0), ("r", math.pi)):
            p.rel[f"leg_upper_{side}"] = rx(-35 * e + 25 * math.sin(tl + ph) * e)
            p.rel[f"leg_lower_{side}"] = rx(15 * e)
        return p

    def preen(self, f, n=75):
        """Turns the head back to the left wing, nibbles, then the right wing, back."""
        p = self.pose()
        L = env(f, 0, 14, 30, 38)
        R = env(f, 36, 46, 60, 72)
        nib = (L * env(f, 12, 15, 29, 31) + R * env(f, 44, 47, 59, 61)) * math.sin(TAU * f / 5)
        yaw = 62 * L - 62 * R
        dn = max(L, R)
        p.rel["neck_1"] = rz(yaw * 0.9) @ rx(-10 * dn)
        p.rel["neck_2"] = rz(yaw * 0.8) @ rx(30 * dn)
        p.rel["head"] = rz(yaw * 0.6) @ rx(35 * dn + 10 * nib)
        p.rel["hips"] = rz(-6 * L + 6 * R) @ ry(4 * (L - R))
        p.hips_offset = V(0, 0, 0.004 * self.S * math.sin(TAU * f / n * 2))
        p.rel["tail"] = rz(10 * math.sin(TAU * f / 12) * dn)
        self.wings(p, lift=(12 * L + 4 * abs(nib) * L, 12 * R + 4 * abs(nib) * R))
        self.legs(p, TAU * f / 30, amp=10.0, fold=10.0)
        return p

    def all(self):
        return [rb.Clip("swim", 30, True, self.swim),
                rb.Clip("idle", 90, True, self.idle),
                rb.Clip("dip", 60, False, self.dip),
                rb.Clip("flap", 45, False, self.flap),
                rb.Clip("preen", 75, False, self.preen)]


# --------------------------------------------------------------------------- main

def run(asset, D, colors, tri_budget=1200):
    import bpy
    args = zb.script_args()
    rb.check_gate("kit_water")  # concept approved as part of kit 5 (duck, frog)
    zb.clean_scene()
    bpy.context.scene.render.fps = FPS
    blend = zb.repo_path("assets", "blender", "animals", f"{asset}.blend")
    glb = zb.repo_path("assets", "models", "animals", f"{asset}.glb")
    body_png = zb.repo_path("assets", "textures", "animals", f"{asset}_body.png")

    duck = Duck(D)
    arm = duck.rig.build_armature(f"{asset}_rig")
    regions = {"eye": (0, 0, 96, 96), "wing": (96, 0, 128, 64)}
    atlas = rb.Atlas(256, colors, regions)
    body_img = atlas.write(body_png, {"eye": paint_eye(colors, pupil_r=D["eye"].get("pupil", 0.46),
                                                       pupil_at=(0.14, 0.04)),
                                      "wing": paint_wing(colors)})
    mat = rb.body_material(body_img)
    mb = rb.MeshBuilder(duck.rig)
    duck.build(mb, atlas)
    mesh = mb.to_object(asset, mat, arm)

    cl = BirdClips(duck.rig, D["S"]).all()
    for c in cl:
        rb.bake_clip(duck.rig, arm, c)
    rb.hr.reset_pose(arm)
    bpy.context.scene.frame_set(0)

    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    print(f"{asset}: {tris} tris, {len(JOINT_NAMES)} joints, bounds x {mn.x:.3f}..{mx.x:.3f}  "
          f"y {mn.y:.3f}..{mx.y:.3f}  z {mn.z:.3f}..{mx.z:.3f}; length {mx.y - mn.y:.3f} m")
    zb.save_blend(blend)
    rb.qr.export_animal([arm, mesh], glb)
    print(f"exported {glb} ({os.path.getsize(glb) / 1024:.1f} KB)")
    if "--debug" in args:
        out = args[args.index("--debug") + 1]
        ls = rb.setup_preview(mesh, body_img, ground_z=0.0)
        s = D["S"]
        rb.render_debug(arm, out, ls, cl, 0.12 * s, V(0, -0.11 * s, 0.28 * s),
                        head_scale=0.3 * s, strip_scale=0.75 * s, ground=rb.WATER)
    if tris > tri_budget:
        print("OVER BUDGET")
        sys.exit(1)
