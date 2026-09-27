"""Building kit for the quadruped animals (hippo, panda, koala, elephant, giraffe, lion,
snow_fox) on top of the shared `quadruped` rig (quadruped_rig.py, ART-ANIMALS "Rig
conventions"). The zebra (zebra.py) predates the kit and builds its parts by hand; the kit
generalises the same techniques so each animal script is mostly data:

- `Tube`: a loft along a centre polyline with per-ring (r_lat, r_up, r_down) radii and pole
  caps; `ellipsoid()` is a tube with a sphere profile. A tube is either a flat colour (atlas
  cell) or owns a *pattern-map region*: the region is painted by sampling a vectorised 3D
  colour field on the tube surface (U = arc length along the tube, V = 0 underneath .. 1 on
  the `up` side, mirrored left/right), so patches, bands and bellies are defined in world
  space, stay broad and clean, and do not depend on the mesh resolution.
- weights: torso (chest/spine/hips + leg tops), chains along a tube, legs by height
- eyes (dome meshes on a tube surface mapped to a painted eye swatch)
- generic clips (idle, walk, eat, drink, happy, refuse, swim) with per-animal parameters and
  hooks for extra joints (trunk, 3-joint tail)
- `run()`: gate, build, atlas, bake, .blend, .glb, preview
"""

import math
import os
import re
import sys
from functools import lru_cache

import bpy
import numpy as np
from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import quadruped_rig as qr  # noqa: E402

zb = qr.zb
hr = qr.hr
X = Vector((1, 0, 0))
Y = Vector((0, 1, 0))
Z = Vector((0, 0, 1))
FWD = qr.FWD
TAU = 2 * math.pi
rx, ry, rz = qr.rx, qr.ry, qr.rz
env = qr.envelope
SS = qr.smoothstep
TRI_BUDGET = 3000

# pattern-map regions of the 256 x 256 atlas (px, top-left origin); the bottom 4 rows of
# 16 px cells (y >= 192) hold the flat colours
DEFAULT_REGIONS = {
    "torso": (0, 0, 256, 72),
    "head": (0, 72, 128, 56),
    "neck": (128, 72, 128, 56),
    "leg_front": (0, 128, 32, 64),
    "leg_hind": (32, 128, 32, 64),
    "tail": (64, 128, 32, 64),
    "eye": (96, 128, 64, 64),
    "extra": (160, 128, 48, 64),
    "extra2": (208, 128, 48, 64),
}


def V3(p):
    return Vector(p)


# --------------------------------------------------------------------------- colour fields

class Palette:
    def __init__(self, colors):
        self.colors = colors

    def __call__(self, name):
        return np.array(hr.hex_rgb(self.colors[name]), np.float32)

    def fill(self, n, name):
        return np.broadcast_to(self(name), (n, 3)).copy()


def nss(e0, e1, x):
    t = np.clip((x - e0) / (e1 - e0), 0.0, 1.0)
    return t * t * (3 - 2 * t)


def blob_mask(x, y, z, centres, radii):
    """Points inside any of the spheres (x mirrored: centres are given for the left side)."""
    ax = np.abs(x)
    m = np.zeros(x.shape, bool)
    for (cx, cy, cz), r in zip(centres, radii):
        m |= (ax - cx) ** 2 + (y - cy) ** 2 + (z - cz) ** 2 < r * r
    return m


# --------------------------------------------------------------------------- tubes

class Tube:
    """Loft along centre points `pts` (list of 3-tuples) with radii [(r_lat, r_up, r_down)].
    Ring vertex k sits at angle th = 2 pi k / n from the `up` side; `up` is the hint vector
    (or one per ring) made perpendicular to the tangent; `lat` defaults to up x tangent.
    cap = (d0, d1): pole distance beyond the first/last ring along the tangent (None = open).
    mod(i, th) -> radius multiplier (scallops)."""

    def __init__(self, pts, radii, n=12, up=Z, lat=None, cap=(0.0, 0.0), mod=None, tangent=None):
        self.pts = [V3(p) for p in pts]
        self.radii = [tuple(r) for r in radii]
        self.n = n
        R = len(self.pts)
        self.R = R
        ts = []
        for i in range(R):
            a, b = self.pts[max(i - 1, 0)], self.pts[min(i + 1, R - 1)]
            ts.append(V3(tangent).normalized() if tangent is not None else (b - a).normalized())
        ups = up if isinstance(up, (list, tuple)) else [up] * R
        self.t, self.up, self.lat = ts, [], []
        for i in range(R):
            u = V3(ups[i])
            u = (u - ts[i] * ts[i].dot(u)).normalized()
            if lat is not None:
                lv = V3(lat)
                lv = (lv - ts[i] * ts[i].dot(lv) - u * u.dot(lv)).normalized()
            else:
                lv = u.cross(ts[i]).normalized()
            self.up.append(u)
            self.lat.append(lv)
        self.rings = []
        for i in range(R):
            rl, ru, rd = self.radii[i]
            ring = []
            for k in range(n):
                th = TAU * k / n
                cu, su = math.cos(th), math.sin(th)
                m = mod(i, th) if mod else 1.0
                rr = ru if cu >= 0 else rd
                ring.append(self.pts[i] + self.up[i] * (cu * rr * m) + self.lat[i] * (su * rl * m))
            self.rings.append(ring)
        self.pole0 = self.pts[0] - ts[0] * cap[0] if cap[0] is not None else None
        self.pole1 = self.pts[-1] + ts[-1] * cap[1] if cap[1] is not None else None
        # U knots: [pole0] + rings + [pole1], arc length of (centre, mean radius)
        kn = []
        if self.pole0 is not None:
            kn.append((self.pole0, self.up[0], self.lat[0], (0.0, 0.0, 0.0)))
        for i in range(R):
            kn.append((self.pts[i], self.up[i], self.lat[i], self.radii[i]))
        if self.pole1 is not None:
            kn.append((self.pole1, self.up[-1], self.lat[-1], (0.0, 0.0, 0.0)))
        acc = [0.0]
        for (c0, _, _, r0), (c1, _, _, r1) in zip(kn, kn[1:]):
            acc.append(acc[-1] + math.hypot((c1 - c0).length, sum(r1) / 3 - sum(r0) / 3))
        self.U = [a / acc[-1] for a in acc]
        self.knots = kn
        self.off = 1 if self.pole0 is not None else 0
        self._arr = {
            "c": np.array([tuple(k[0]) for k in kn]), "up": np.array([tuple(k[1]) for k in kn]),
            "lat": np.array([tuple(k[2]) for k in kn]), "r": np.array([k[3] for k in kn])}

    def _interp(self, u):
        U = np.array(self.U)
        a = self._arr
        c = np.stack([np.interp(u, U, a["c"][:, j]) for j in range(3)], -1)
        up = np.stack([np.interp(u, U, a["up"][:, j]) for j in range(3)], -1)
        lat = np.stack([np.interp(u, U, a["lat"][:, j]) for j in range(3)], -1)
        r = np.stack([np.interp(u, U, a["r"][:, j]) for j in range(3)], -1)
        return c, up, lat, r

    def points_uv(self, u, v):
        """Surface points for atlas (U, V) arrays (the +lat side)."""
        c, up, lat, r = self._interp(u)
        cu = np.clip(2 * v - 1, -1, 1)
        su = np.sqrt(1 - cu * cu)
        rr = np.where(cu >= 0, r[:, 1], r[:, 2])
        return c + up * (cu * rr)[:, None] + lat * (su * r[:, 0])[:, None]

    def point(self, u, th):
        c, up, lat, r = self._interp(np.array([u]))
        cu, su = math.cos(th), math.sin(th)
        rr = r[0, 1] if cu >= 0 else r[0, 2]
        return Vector(c[0] + up[0] * cu * rr + lat[0] * su * r[0, 0])

    def normal(self, u, th):
        e = 1e-3
        du = self.point(u + e, th) - self.point(u - e, th)
        dt = self.point(u, th + e) - self.point(u, th - e)
        nrm = du.cross(dt).normalized()
        c, *_ = self._interp(np.array([u]))
        if nrm.dot(self.point(u, th) - Vector(c[0])) < 0:
            nrm = -nrm
        return nrm

    def param(self, co):
        """U of the nearest point on the knot centre line."""
        best, bu = 1e9, 0.0
        for i in range(len(self.knots) - 1):
            a, b = self.knots[i][0], self.knots[i + 1][0]
            ab = b - a
            L2 = ab.length_squared
            s = 0.0 if L2 < 1e-12 else min(max((co - a).dot(ab) / L2, 0.0), 1.0)
            d = (a + ab * s - co).length
            if d < best:
                best, bu = d, self.U[i] + (self.U[i + 1] - self.U[i]) * s
        return bu


def ellipsoid(c, fwd, up, a_back, a_front, r_lat, r_up, r_down=None, rings=7, n=12, lat=None,
              mod=None):
    """Ellipsoid as a tube along `fwd` (rings from the back to the front)."""
    c, fwd = V3(c), V3(fwd).normalized()
    r_down = r_up if r_down is None else r_down
    pts, radii = [], []
    for k in range(1, rings + 1):
        ph = (-0.5 + k / (rings + 1)) * math.pi
        s = math.sin(ph)
        pos = s * (a_back if s < 0 else a_front)
        f = math.cos(ph)
        pts.append(c + fwd * pos)
        radii.append((r_lat * f, r_up * f, r_down * f))
    s0 = math.sin((-0.5 + 1 / (rings + 1)) * math.pi)
    s1 = math.sin((-0.5 + rings / (rings + 1)) * math.pi)
    return Tube(pts, radii, n, up=up, lat=lat, cap=(a_back * (1 + s0), a_front * (1 - s1)), mod=mod)


# --------------------------------------------------------------------------- kit

class Kit:
    def __init__(self, asset, colors, P, regions=None):
        self.asset = asset
        self.P = P
        self.rig = qr.Rig(P)
        self.pal = Palette(colors)
        self.atlas = qr.Atlas(256, colors, regions or DEFAULT_REGIONS)
        self.mb = qr.MeshBuilder(self.rig.names)
        self.painters = {}
        self.tubes = {}

    # ---- parts
    def add(self, tube, weight, region=None, color=None, cell=None, face_cell=None, name=None):
        """Add a tube to the mesh. Colour: `cell` (flat atlas cell name) or `region` + vectorised
        `color(x, y, z, U, V) -> (N, 3)` (painted once per region; later tubes share it).
        face_cell(i, centroid) -> cell name or None overrides whole faces (ear insides)."""
        at = self.atlas
        if cell is not None:
            cuv = at.cell_uv(cell)
            uv = lambda i, k, co: cuv  # noqa: E731
        else:
            if region not in self.painters:
                self.painters[region] = self._painter(tube, color)
            n, off, U = tube.n, tube.off, tube.U
            uv = lambda i, k, co: at.map_uv(region, U[i + off], qr.ring_h(k, n))  # noqa: E731
        fuv = None
        if face_cell:
            def fuv(i, cen):
                c = face_cell(i, cen)
                return at.cell_uv(c) if c else None
        if not callable(weight):
            weight = qr.rigid(weight)
        self.mb.loft(tube.rings, uv, weight, pole_start=tube.pole0, pole_end=tube.pole1,
                     face_uv=fuv)
        if name:
            self.tubes[name] = tube
        return tube

    def _painter(self, tube, color):
        def fn(U, V):
            shp = U.shape
            u, v = U.ravel(), V.ravel()
            p = tube.points_uv(u, v)
            rgb = color(p[:, 0], p[:, 1], p[:, 2], u, v)
            return np.asarray(rgb, np.float32).reshape(shp + (3,))
        return fn

    def paint_region(self, region, fn):
        """Custom painter fn(U, V) -> (h, w, 3) (eye swatch etc.)."""
        self.painters[region] = fn

    # ---- weights
    def w_torso(self, leg_r=0.1, leg_k=0.45, blend=None):
        rig = self.rig
        c, s, h = (rig.rest_head[n].y for n in ("chest", "spine", "hips"))
        stops = blend or [(c + 0.25 * (s - c), "chest"), (s - 0.35 * (s - c), "spine"),
                          (s + 0.25 * (h - s), "spine"), (h - 0.15 * (h - s), "hips")]
        tops = []
        for leg in qr.LEGS:
            up, lo, _ = qr.leg_bones(leg)
            tops.append((up, rig.rest_head[up], rig.rest_head[lo].z))

        def f(co):
            w = qr.chain(co.y, stops)
            for up, hd, knee in tops:
                if hd.x * co.x <= 0:
                    continue
                d = math.hypot(co.x - hd.x, co.y - hd.y)
                Lu = hd.z - knee
                k = SS(hd.z - 0.15 * Lu, hd.z - 0.6 * Lu, co.z) * SS(2.4 * leg_r, 0.9 * leg_r, d) * leg_k
                if k > 0:
                    w = qr.mix(w, {up: 1.0}, k)
            return w
        return f

    def w_along(self, tube, stops):
        return lambda co: qr.chain(tube.param(co), stops)

    def w_leg(self, leg, foot_frac=0.85):
        rig = self.rig
        up, lo, ft = qr.leg_bones(leg)
        top, knee, fet = rig.rest_head[up].z, rig.rest_head[lo].z, rig.rest_head[ft].z
        Lu = top - knee
        parent = qr.PARENT[up]
        stops = [(fet * foot_frac, ft), (fet + 0.35 * (knee - fet), lo), (knee - 0.12 * Lu, lo),
                 (knee + 0.12 * Lu, up), (top - 0.3 * Lu, up), (top + 0.08 * Lu, parent)]
        return lambda co: qr.chain(co.z, stops)

    # ---- common parts
    def torso(self, table, color=None, cell=None, n=16, cap=(0.05, 0.05), leg_r=0.1, leg_k=0.45):
        """Body barrel: table = [(y, cz, r_lat, r_up, r_down)] from the rump (+Y) to the chest."""
        t = Tube([(0, y, z) for y, z, *_ in table], [r[2:] for r in table], n=n, up=Z, lat=X, cap=cap)
        if cell:
            return self.add(t, self.w_torso(leg_r, leg_k), cell=cell, name="torso")
        return self.add(t, self.w_torso(leg_r, leg_k), region="torso", color=color, name="torso")

    def head(self, B, D, profile, color=None, cell=None, n=14, cap=(0.03, 0.03), bone="head"):
        """Head loft from the back of the skull B along direction D; profile =
        [(s, r_lat, r_up, r_down)] (s in m from B). U = 0 at the back, 1 at the nose."""
        B, D = V3(B), V3(D).normalized()
        t = Tube([B + D * s for s, *_ in profile], [(a, b, c) for _, a, b, c in profile], n=n,
                 up=(Z - D * D.dot(Z)), lat=X, cap=cap)
        if cell:
            return self.add(t, bone, cell=cell, name="head")
        return self.add(t, bone, region="head", color=color, name="head")

    def legs(self, profile_front, profile_hind, n=8, color=None, cell=None, paw=None,
             paw_cell=None, foot_frac=0.85):
        """Legs: profile = [(z, r_front, r_back, r_lat, dy)], top to ground, at the rig's leg
        x/y. paw(leg) -> ellipsoid Tube or None (rigid on the foot bone)."""
        rig = self.rig
        for leg in qr.LEGS:
            up = qr.leg_bones(leg)[0]
            h = rig.rest_head[up]
            prof = profile_front if leg.startswith("front") else profile_hind
            z0, rf0, rb0, rl0, dy0 = prof[0]
            # rounded top inside the body (no flat cap that could poke out of the torso)
            prof = [(z0 + 0.45 * rl0, 0.7 * rf0, 0.7 * rb0, 0.7 * rl0, dy0)] + list(prof)
            pts = [(h.x, h.y + dy, z) for z, _, _, _, dy in prof]
            radii = [(rl, rf, rb) for _, rf, rb, rl, _ in prof]
            # horizontal rings (fixed tangent): the sole stays flat on the ground
            t = Tube(pts, radii, n, up=FWD, lat=X, cap=(0.35 * rl0, 0.0), tangent=-Z)
            region = "leg_front" if leg.startswith("front") else "leg_hind"
            if cell:
                self.add(t, self.w_leg(leg, foot_frac), cell=cell)
            else:
                # paint from the left leg (right legs share the region)
                self.add(t, self.w_leg(leg, foot_frac), region=region, color=color)
            if paw:
                pt = paw(leg, h)
                if pt is not None:
                    self.add(pt, qr.leg_bones(leg)[2], cell=paw_cell)

    def eyes(self, tube, u, th_deg, r, depth=0.03, bone="head", tilt=0.0, glow=True):
        """Dome eyes on the tube surface at (u, +-th) (radius r = (along, up)), mapped to
        the 'eye' region. tilt rotates the eye swatch (deg, positive = outer corner up).
        glow: the front cap of each dome (pupil, inner iris and both highlights, inner 52 %
        of the eye) goes to the `eye_glow` material slot (NIGHT-006, no extra geometry)."""
        at = self.atlas
        for sg in (1.0, -1.0):
            th = sg * math.radians(th_deg)
            S = tube.point(u, th)
            f = tube.normal(u, th)
            d = tube.point(u + 0.01, th) - tube.point(u - 0.01, th)
            ex = (d - f * d.dot(f)).normalized()        # towards the tube end (nose)
            ey = f.cross(ex).normalized()
            if ey.dot(Z) < 0:
                ey = -ey
            if tilt:
                a = math.radians(tilt) * sg
                ex, ey = ex * math.cos(a) + ey * math.sin(a), ey * math.cos(a) - ex * math.sin(a)
            c = S - f * (0.1 * depth)
            rings = []
            for off, sc in ((-0.9 * depth, 0.72), (-0.35 * depth, 1.0), (0.3 * depth, 0.86),
                            (0.65 * depth, 0.52)):
                rings.append(qr.ring(c + f * off, ey, ex, r[1] * sc, r[0] * sc, 12))

            def uv(i, k, co, c=c, ex=ex, ey=ey):
                dd = co - c
                return at.map_uv("eye", 0.5 + 0.5 * dd.dot(ex) / r[0], 0.5 + 0.5 * dd.dot(ey) / r[1])

            _, faces = self.mb.loft(rings, uv, qr.rigid(bone), pole_start=c - f * (1.3 * depth),
                                    pole_end=c + f * (0.9 * depth))
            if glow:
                self.mb.mark_glow(faces[-12:])  # the pole_end fan (12 = ring vertices)

    def eye_swatch(self, iris="iris", pupil="pupil", white="eye_white", ink="ink", iris_r=0.62,
                   pupil_r=0.36, look=0.12):
        P = self.pal

        def fn(U, V):
            x, y = 2 * U - 1, 2 * V - 1
            c = np.broadcast_to(P(ink), U.shape + (3,)).copy()
            rr = np.hypot(x, y)
            inner = rr < 0.86
            c[inner] = P(white)
            c[inner & (np.hypot(x - look, y + 0.02) < iris_r)] = P(iris)
            c[inner & (np.hypot(x - look * 1.1, y + 0.02) < pupil_r)] = P(pupil)
            c[np.hypot(x - look + 0.12, y - 0.25) < 0.17] = P(white)
            c[np.hypot(x - look - 0.2, y + 0.27) < 0.075] = P(white)
            return c
        self.paint_region("eye", fn)

    def ear(self, side_sign, base, tip, width, thick, front, outer_cell, inner_cell=None,
            prof=None, weight=None, n=8, inner_frac=0.55, inner_t=(0.08, 0.92), side=None):
        """Flat ear (a flattened loft) from base to tip; the side facing `front` gets the
        inner colour. prof = [(t, width_scale, thick_scale)]."""
        base, tip = V3(qr.mirror(base, side_sign)), V3(qr.mirror(tip, side_sign))
        fr = V3(qr.mirror(front, side_sign)).normalized()
        ax = (tip - base).normalized()
        L = (tip - base).length
        fr = (fr - ax * ax.dot(fr)).normalized()
        prof = prof or [(-0.10, 0.60, 0.8), (0.15, 0.95, 1.0), (0.45, 1.0, 0.9), (0.75, 0.80, 0.8),
                        (0.93, 0.45, 0.6)]
        pts = [base + ax * (t * L) for t, _, _ in prof]
        radii = [(width * w, thick * th, thick * th) for _, w, th in prof]
        tube = Tube(pts, radii, n, up=fr, cap=(0.02 * L, 0.06 * L))
        side = side or ("l" if side_sign > 0 else "r")

        def face_cell(i, cen):
            if inner_cell is None:
                return outer_cell
            t = (cen - base).dot(ax) / L
            d = cen - (base + ax * (t * L))
            lat = ax.cross(fr)
            wv = width * np.interp(t, [p[0] for p in prof], [p[1] for p in prof])
            if d.dot(fr) > 0.3 * thick and inner_t[0] < t < inner_t[1] and abs(d.dot(lat)) < inner_frac * wv * 1.25:
                return inner_cell
            return outer_cell
        w = weight or (lambda co: qr.chain((co - base).dot(ax) / L, [(0.0, "head"), (0.25, f"ear_{side}")]))
        self.add(tube, w, cell=outer_cell, face_cell=face_cell)
        return tube

    # ---- build
    def finish(self, arm):
        body_png = zb.repo_path("assets", "textures", "animals", f"{self.asset}_body.png")
        img = self.atlas.write(body_png, self.painters)
        mat = qr.body_material(img)
        mesh = self.mb.to_object(self.asset, mat, arm)
        return mesh, img


# --------------------------------------------------------------------------- clips

class ClipSet:
    """Generic procedural clips. cfg keys (all optional except walk):
    sc            size factor (translations scale with it; zebra = 1)
    tail          tail joints, tail_amp (deg), tail_lift (deg)
    ears_fn       fn(p, back, out, flick_l, flick_r)   (flick in 0..1)
    walk          dict(frames, stance, lift, toe_deg, fold_deg, toe_fwd, dip, roll)
    eat           dict(nose_z, body=(dz, pitch), split, bites) or dict(high=True, ...)
    drink         dict(nose_z, body, split)
    happy         dict(hop, toss)
    extra         fn(p, clip, f, n) extra joints (trunk ...)
    head_look     deg of idle look around"""

    def __init__(self, kit, cfg):
        self.k = kit
        self.rig = kit.rig
        self.c = cfg
        self.sc = cfg.get("sc", 1.0)
        self.tail = cfg.get("tail", ["tail_1", "tail_2"])
        w = cfg["walk"]
        self.walk = qr.make_walk(self.rig, frames=w["frames"], speed=w.get("speed", 1.4),
                                 stance=w.get("stance", 0.52), lift=w.get("lift", 0.10),
                                 toe_deg=w.get("toe_deg", 30.0), fold_deg=w.get("fold_deg", 50.0),
                                 toe_fwd=w.get("toe_fwd", 0.07), upper_fn=self._walk_upper,
                                 dip=w.get("dip", 0.02), roll_deg=w.get("roll", 1.5))

    # helpers
    def extra(self, p, clip, f, n):
        fn = self.c.get("extra")
        if fn:
            fn(p, clip, f, n)

    def tail_swish(self, p, amp, phase, lift=None):
        lift = self.c.get("tail_lift", 4.0) if lift is None else lift
        for i, j in enumerate(self.tail):
            p.rel[j] = rz(amp * (1 + 0.3 * i) * math.sin(phase - 1.0 * i)) @ rx(lift * (0.5 ** i))

    def ears(self, p, back=0.0, out=0.0, fl=0.0, fr=0.0):
        fn = self.c.get("ears_fn")
        if fn:
            fn(p, back, out, fl, fr)
            return
        p.rel["ear_l"] = ry(out + 18 * fl) @ rx(-back - 30 * fl)
        p.rel["ear_r"] = ry(-out - 18 * fr) @ rx(-back - 30 * fr)

    # clips
    def idle(self, f, n=90):
        t = TAU * f / n
        s = self.sc
        p = qr.Pose(self.rig)
        b = math.sin(2 * t)
        p.stand((0.006 * s * math.sin(t), 0, (-0.008 + 0.004 * b) * s), hips_rot=ry(0.8 * math.sin(t)),
                chest=rx(-0.6 * b))
        look = self.c.get("head_look", 5.0)
        p.rel["neck_1"] = rx(-1.2 * b)
        p.rel["neck_2"] = rz(look * math.sin(t + 0.5))
        p.rel["head"] = rx(2.0 * math.sin(2 * t + 1.0)) @ rz(look * 0.7 * math.sin(t))
        self.tail_swish(p, self.c.get("tail_amp", 14.0), 3 * t)
        self.ears(p, back=3.0 * math.sin(t), fl=env(f, 60, 63, 64, 70), fr=env(f, 28, 31, 33, 39))
        self.extra(p, "idle", f, n)
        return p

    def _walk_upper(self, p, fi, t):
        p.rel["neck_1"] = rx(-1.0 + 2.0 * math.cos(2 * t))
        p.rel["neck_2"] = rz(-1.5 * math.sin(t))
        p.rel["head"] = rx(1.0 - 2.5 * math.cos(2 * t + 0.5))
        self.tail_swish(p, self.c.get("tail_amp", 14.0) * 0.65, t, lift=self.c.get("tail_lift", 4.0) + 2)
        self.ears(p, back=2.0 * math.sin(2 * t))
        self.extra(p, "walk", fi, self.c["walk"]["frames"])

    def _low_body(self, p, k, body):
        dz, pitch = body
        p.stand((0, 0, -dz * k * self.sc), hips_rot=rx(pitch * k), spine=rx(2.0 * k))

    @lru_cache(maxsize=None)
    def _head_down(self, nose_z, body, split):
        p = qr.Pose(self.rig)
        self._low_body(p, 1.0, body)
        return p.head_down(nose_z, split)

    def eat(self, f, n=60):
        e = self.c.get("eat", {})
        if e.get("high"):
            return self.eat_high(f, n)
        k = env(f, 0, 14, 46, 60)
        body = e.get("body", (0.03, 6.0))
        split = e.get("split", (0.66, 0.34, -0.70))
        p = qr.Pose(self.rig)
        self._low_body(p, k, body)
        A = self._head_down(e.get("nose_z", 0.15), body, split) * k
        bite = sum(env(f, c - 4, c - 1, c, c + 4) for c in (22, 31, 40))
        p.rel["neck_1"] = rx(A * split[0])
        p.rel["neck_2"] = rx(A * split[1] + 3.0 * bite)
        p.rel["head"] = rx(A * split[2] - 9.0 * bite)
        self.tail_swish(p, 10.0, TAU * f / 30)
        self.ears(p, back=6.0 * k, out=8.0 * k)
        self.extra(p, "eat", f, n)
        return p

    def eat_high(self, f, n=60):
        k = env(f, 0, 12, 48, 60)
        bite = sum(env(f, c - 4, c - 1, c, c + 4) for c in (22, 31, 40))
        p = qr.Pose(self.rig)
        p.stand((0, 0, 0), hips_rot=rx(-1.5 * k))
        p.rel["neck_1"] = rx(-10 * k)
        p.rel["neck_2"] = rx(-6 * k + 2 * bite)
        p.rel["head"] = rx(-22 * k - 10 * bite)
        self.tail_swish(p, 10.0, TAU * f / 30)
        self.ears(p, back=-4.0 * k, out=10.0 * k)
        self.extra(p, "eat", f, n)
        return p

    def drink(self, f, n=60):
        d = self.c.get("drink", {})
        t = TAU * f / n
        body = d.get("body", (0.05, 8.0))
        split = d.get("split", (0.66, 0.34, -0.70))
        p = qr.Pose(self.rig)
        self._low_body(p, 1.0, body)
        A = self._head_down(d.get("nose_z", 0.10), body, split)
        lap = math.sin(4 * t)
        p.rel["neck_1"] = rx(A * split[0])
        p.rel["neck_2"] = rx(A * split[1] + 1.5 * math.sin(4 * t + 1.0))
        p.rel["head"] = rx(A * split[2] - 3.0 * lap)
        self.tail_swish(p, 12.0, 2 * t)
        self.ears(p, back=4.0 + 3.0 * math.sin(t), out=10.0)
        self.extra(p, "drink", f, n)
        return p

    def happy(self, f, n=45):
        h = self.c.get("happy", {})
        hop = h.get("hop", 1.0) * self.sc
        crouch = env(f, 0, 6, 6, 10)
        jump = env(f, 8, 15, 15, 23)
        land = env(f, 21, 26, 27, 36)
        toss = env(f, 6, 14, 22, 38)
        z = -0.06 * crouch * self.sc + 0.17 * jump * hop - 0.05 * land * self.sc
        dz = max(0.0, z) + 0.06 * jump * hop
        fl = h.get("fold", 40.0)
        lift = {"front_l": (0.05 * jump * hop, dz, -fl * jump), "front_r": (0.05 * jump * hop, dz, -fl * jump),
                "hind_l": (0.02 * jump * hop, dz * 0.9, -0.6 * fl * jump),
                "hind_r": (0.02 * jump * hop, dz * 0.9, -0.6 * fl * jump)}
        p = qr.Pose(self.rig)
        p.stand((0, 0, z), hips_rot=rx(-5 * jump + 3 * crouch), lift=lift)
        wig = math.sin(TAU * f / 12)
        tt = h.get("toss", 1.0)
        p.rel["neck_1"] = rx(-16 * toss * tt)
        p.rel["neck_2"] = rx(-8 * toss * tt) @ rz(6 * wig * toss)
        p.rel["head"] = rx((-12 * toss + 5 * crouch) * tt)
        for i, j in enumerate(self.tail):
            p.rel[j] = rz((22 + 6 * i) * math.sin(TAU * f / 12 - i) * toss) @ rx((35 if i == 0 else 15) * toss * 0.7 ** i + (4 if i == 0 else 0))
        self.ears(p, back=-10 * toss, out=12 * toss)
        self.extra(p, "happy", f, n)
        return p

    def refuse(self, f, n=36):
        e = env(f, 0, 6, 26, 36)
        sh = env(f, 3, 8, 24, 32)
        ang = 24.0 * math.sin(TAU * (f - 4) / 11) * sh
        p = qr.Pose(self.rig)
        p.stand((0, 0.03 * e * self.sc, -0.01 * e * self.sc), hips_rot=rx(-2.0 * e))
        p.rel["neck_1"] = rx(-7 * e)
        p.rel["neck_2"] = rz(0.4 * ang)
        p.rel["head"] = rz(0.6 * ang) @ rx(-4 * e)
        self.tail_swish(p, 18.0 * e, TAU * f / 9)
        self.ears(p, back=32 * e, out=22 * e)
        self.extra(p, "refuse", f, n)
        return p

    def swim(self, f, n=48):
        """Floating: legs paddle below the body, head up, gentle bob (the game sinks the
        model to its water line)."""
        t = TAU * f / n
        s = self.sc
        p = qr.Pose(self.rig)
        p.hips_offset = Vector((0, 0, 0.03 * s * math.sin(2 * t)))
        p.rel["hips"] = rx(1.5 * math.sin(2 * t + 0.6))
        rig = self.rig
        for leg, ph in (("front_l", 0.0), ("hind_r", 0.0), ("front_r", 0.5), ("hind_l", 0.5)):
            a = TAU * ph + t
            r0 = p.rest_fetlock(leg)
            sw = 0.10 * s
            tgt = r0 + Vector((0, -sw * math.cos(a), 0.10 * s + 0.06 * s * math.sin(a)))
            p.leg_ik(leg, tgt, -25 - 20 * math.sin(a))
        p.rel["neck_1"] = rx(-6 + 1.5 * math.sin(2 * t))
        p.rel["head"] = rx(-6 + 2 * math.sin(2 * t + 1))
        self.tail_swish(p, 10.0, t)
        self.ears(p, back=3 * math.sin(2 * t), fl=env(f % n, 30, 33, 34, 40))
        self.extra(p, "swim", f, n)
        return p

    def clips(self, names):
        spec = {"idle": (90, True, self.idle), "walk": (self.c["walk"]["frames"], True, self.walk),
                "eat": (60, False, self.eat), "drink": (60, True, self.drink),
                "happy": (45, False, self.happy), "refuse": (36, False, self.refuse),
                "swim": (48, True, self.swim)}
        return [qr.Clip(nm, spec[nm][0], spec[nm][1], spec[nm][2]) for nm in names]


# --------------------------------------------------------------------------- run

def check_gate(asset):
    """ART-PIPELINE §2: no modelling before the concept is approved."""
    text = open(zb.repo_path("assets", "manifest.toml"), encoding="utf-8").read()
    for block in text.split("[[asset]]")[1:]:
        if re.search(r'^id\s*=\s*"%s"' % asset, block, re.M):
            if re.search(r"^concept_approved\s*=\s*true", block, re.M):
                return
    print(f"{asset}: concept not approved in assets/manifest.toml — not modelling")
    sys.exit(1)


def run(kit, build, clipset_fn, clip_names, preview):
    """build(kit) adds all parts; clipset_fn(kit) -> ClipSet; preview: dict for
    qr.render_preview (center_z, walk_frame, big_scale, close_scale) + head_pt, debug_scale."""
    asset = kit.asset
    args = zb.script_args()
    check_gate(asset)
    zb.clean_scene()
    bpy.context.scene.render.fps = qr.FPS
    arm = kit.rig.build_armature(f"{asset}_rig")
    build(kit)
    mesh, img = kit.finish(arm)
    cs = clipset_fn(kit)
    cl = cs.clips(clip_names)
    for c in cl:
        qr.bake_clip(kit.rig, arm, c)
    hr.reset_pose(arm)
    bpy.context.scene.frame_set(0)

    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    print(f"{asset}: {tris} tris, {len(kit.rig.names)} joints, bounds x {mn.x:.3f}..{mx.x:.3f}  "
          f"y {mn.y:.3f}..{mx.y:.3f}  z {mn.z:.3f}..{mx.z:.3f}")
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in cs.walk.drop))

    blend = zb.repo_path("assets", "blender", "animals", f"{asset}.blend")
    glb = zb.repo_path("assets", "models", "animals", f"{asset}.glb")
    zb.save_blend(blend)
    qr.export_animal([arm, mesh], glb)
    print(f"exported {glb} ({os.path.getsize(glb) / 1024:.1f} KB)")
    if "--no-preview" not in args:
        ls = qr.setup_preview(mesh, img)
        pv = dict(preview)
        head_pt = pv.pop("head_pt", (0, -0.5, 1.0))
        dscale = pv.pop("debug_scale", 1.0)
        out = zb.repo_path("art", "animals", asset, "model_preview.png")
        qr.render_preview(arm, out, ls, **pv)
        if "--debug" in args:
            qr.render_debug(arm, args[args.index("--debug") + 1], ls, cl, pv["center_z"],
                            Vector(head_pt), scale=dscale)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)
