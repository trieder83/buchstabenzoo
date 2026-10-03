"""snake / snake_female / snake_hatchling — the terrarium snake family (GAME-FAMILY, ART-ANIMALS
"Terrarium animals", night_2; concept art/animals/snake_family/family_vA1.jpg = suggestion A,
approved 2026-10-03): a friendly kingsnake, orange with black-cream-black bands.

Run:  blender -b --factory-startup --python tools/blender/animals/snake.py \
          -- --variant male|female|hatchling [--no-preview] [--debug <dir>]

Writes (per variant, ASSET = snake | snake_female | snake_hatchling)
  assets/blender/animals/<ASSET>.blend
  assets/models/animals/<ASSET>.glb          1 skin (16 joints), 1 mesh, clips idle walk eat happy
                                             refuse sleep; materials body + eye_glow (night level)
  assets/textures/animals/<ASSET>_body.png   body band map + head map + eye + flat colour cells
  art/animals/<ASSET>/model_preview.png

Rig `snake` (limbless): root, hips (middle of the body, start of the neck side) and two chains
leaving it — tail_1..tail_8 (towards the tail tip) and spine_1..spine_5 + head (towards the head):
15 bones of ~0.15 m along the centre line. Rest pose = the COILED pose of the concept (two loops,
neck raised, head over the coil, facing -Y = glTF +Z). The `walk` clip is a SLITHER: the body
follows a sine track (A = 0.11 m, wavelength 0.70 m) that is fixed on the ground, the head leads
and is raised; every bone is placed from the track (path following, so the belly never skids
sideways) and the model's own origin advances at WALK_SPEED, one wave per loop (24 frames).
`sleep` = the neck lies down over the coil and the head rests on it.

Sizes (game metres, Q-335: the model is a thick comic snake, a stretched-out body is ~2.2 m
of centre line, ~1.5 m long in the slither; the coil is the reading unit):
  male        coil footprint 0.60 m, head top 0.62 m, tube diameter 0.14 m
  female      0.92 x male
  hatchling   0.45 x male; head 1.45 x and eyes 1.6 x relative, tube 1.15 x (chubby)
Origin: on the ground under the coil centre (min z = 0); one flat tube resting on the ground.
Pattern: a body band map (U along the body from the tail, V ring height; bands repeat
orange - black - cream - black) + a head map (cream chin, smile, black head marking) + flat
colour cells. No vertex colours (AANI-004).
"""

import math
import os
import sys

import numpy as np
from mathutils import Quaternion, Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bird_rig as br  # noqa: E402

rb = br.rb
zb = rb.zb
V = br.V
X, Y, Z = rb.X, rb.Y, rb.Z
rx, ry, rz = rb.rx, rb.ry, rb.rz
env = rb.envelope
TAU = rb.TAU

TRI_BUDGET = 1200
N_BONES = 15
HIPS = 8                     # bone index of `hips` (0 = tail tip bone .. 14 = head bone)
SLITHER_A = 0.11             # m, amplitude of the ground track (male)
SLITHER_L = 0.70             # m, wavelength along the direction of travel (male)
WALK_FRAMES = 24             # one wave per loop

VARIANTS = {
    "male": dict(asset="snake", scale=1.0, head=1.0, eye=1.0, thick=1.0),
    "female": dict(asset="snake_female", scale=0.92, head=1.0, eye=1.0, thick=1.0),
    "hatchling": dict(asset="snake_hatchling", scale=0.45, head=1.45, eye=1.6, thick=1.15),
}
COMMON = {"ink": "#2B1B12", "eye_white": "#FFFFFF", "iris": "#6A4128", "pupil": "#0E0C0C"}
PALETTES = {
    "male": dict(skin="#E9801E", dark="#2A211D", cream="#FAEBC2", smile="#7A3A12", **COMMON),
    "female": dict(skin="#F2A27F", dark="#2A211D", cream="#FCE3C6", smile="#8A4A32",
                   skin2="#F8CDB0", **COMMON),
    "hatchling": dict(skin="#EE8A22", dark="#2A211D", cream="#FAEBC2", smile="#7A3A12", **COMMON),
}
REGIONS = {"body": (0, 0, 256, 40), "head": (0, 40, 128, 72), "eye": (128, 40, 64, 64)}

JOINTS = (["root", "hips"] + [f"tail_{k}" for k in range(1, HIPS + 1)]
          + [f"spine_{k}" for k in range(1, N_BONES - HIPS - 1)] + ["head"])


def bone_name(i):
    if i == HIPS:
        return "hips"
    if i < HIPS:
        return f"tail_{HIPS - i}"
    return "head" if i == N_BONES - 1 else f"spine_{i - HIPS}"


BONE_IDX = {bone_name(i): i for i in range(N_BONES)}
G = {}
RIG = None


# --------------------------------------------------------------------------- centre line

def cr(ctrl, per=14):
    """Uniform Catmull-Rom through the control points (end points duplicated)."""
    c = [np.array(p, float) for p in ctrl]
    c = [c[0]] + c + [c[-1]]
    out = []
    for i in range(1, len(c) - 2):
        p0, p1, p2, p3 = c[i - 1], c[i], c[i + 1], c[i + 2]
        for t in np.linspace(0, 1, per, endpoint=False):
            out.append(0.5 * ((2 * p1) + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t
                              + (-p0 + 3 * p1 - 3 * p2 + p3) * t ** 3))
    out.append(c[-2])
    return np.array(out)


class Curve:
    """Dense polyline with arclength; point(s), tangent(s)."""

    def __init__(self, pts):
        self.p = np.asarray(pts, float)
        d = np.linalg.norm(np.diff(self.p, axis=0), axis=1)
        self.cum = np.concatenate([[0.0], np.cumsum(d)])
        self.L = float(self.cum[-1])

    def point(self, s):
        s = min(max(s, 0.0), self.L)
        i = min(int(np.searchsorted(self.cum, s, side="right")) - 1, len(self.p) - 2)
        t = (s - self.cum[i]) / max(self.cum[i + 1] - self.cum[i], 1e-12)
        return self.p[i] * (1 - t) + self.p[i + 1] * t

    def tangent(self, s, h=0.01):
        a, b = self.point(s - h), self.point(s + h)
        d = b - a
        return d / max(np.linalg.norm(d), 1e-9)

    def step(self, s0, p0, chord, direction):
        """Next arclength beyond s0 (direction +1/-1) whose point is `chord` away from p0."""
        ds = 0.002 * direction
        s = s0
        lo = s0
        for _ in range(4000):
            s += ds
            if s < 0 or s > self.L:
                break
            if np.linalg.norm(self.point(s) - p0) >= chord:
                a, b = lo, s
                for _ in range(24):
                    m = 0.5 * (a + b)
                    if np.linalg.norm(self.point(m) - p0) >= chord:
                        b = m
                    else:
                        a = m
                return b
            lo = s
        # ran out of curve: extrapolate straight on
        end = self.point(min(max(s, 0.0), self.L))
        t = self.tangent(min(max(s, 0.0), self.L))
        return None, end + t * direction * 0.0


def smooth(e0, e1, x):
    t = min(max((x - e0) / (e1 - e0), 0.0), 1.0)
    return t * t * (3 - 2 * t)


def tube_radius(s, S, k, thick):
    """Body radius at arclength s from the tail tip (m, already scaled by k)."""
    r = 0.07 * thick
    neck = 1.0 - 0.30 * smooth(S / k - 0.62, S / k - 0.30, s / k)
    return k * (0.012 + (r - 0.012) * smooth(0.0, 0.9, s / k)) * neck


def rest_control_points(k, head_scale):
    """Control points (m) of the coiled centre line, tail tip -> nose."""
    pts = []
    phi0 = math.radians(216.0)
    n1, n2 = 0.95, 1.62                       # turns: ring 1 ends / ring 2 ends
    steps = 34
    for i in range(steps + 1):
        u = n2 * i / steps
        if u < 0.35:
            R = 0.10 + 0.10 * smooth(0.0, 0.35, u)
        else:
            R = 0.20 - 0.065 * smooth(n1, 1.35, u)
        z = 0.07 + 0.12 * smooth(n1, 1.38, u)
        phi = phi0 - TAU * u
        pts.append((R * math.cos(phi), R * math.sin(phi), z))
    # neck: leaves ring 2 heading -Y, rises, bows forward (head over the coil, facing -Y)
    e = np.array(pts[-1])
    neck = [(e[0] - 0.035, e[1] - 0.075, e[2] + 0.07),
            (e[0] - 0.085, e[1] - 0.095, e[2] + 0.17),
            (e[0] - 0.115, e[1] - 0.085, e[2] + 0.28),
            (e[0] - 0.115, e[1] - 0.085, e[2] + 0.36),
            (e[0] - 0.120, e[1] - 0.120, e[2] + 0.400),
            (e[0] - 0.120, e[1] - 0.190, e[2] + 0.395),
            (e[0] - 0.120, e[1] - 0.270, e[2] + 0.385),
            (e[0] - 0.120, e[1] - 0.360, e[2] + 0.375)]
    pts += neck
    return [tuple(np.array(p) * k) for p in pts]


def sleep_control_points(k):
    """Same ring part, the neck lies down over the coil: the head rests on ring 2."""
    base = rest_control_points(k, 1.0)
    n_ring = 35
    ring = base[:n_ring]
    e = np.array(ring[-1]) / k
    neck = [(e[0] - 0.05, e[1] - 0.05, e[2] + 0.04),
            (e[0] - 0.11, e[1] - 0.04, e[2] + 0.075),
            (e[0] - 0.17, e[1] + 0.00, e[2] + 0.075),
            (e[0] - 0.22, e[1] + 0.08, e[2] + 0.06),
            (e[0] - 0.24, e[1] + 0.19, e[2] + 0.050),
            (e[0] - 0.22, e[1] + 0.30, e[2] + 0.045)]
    return ring + [tuple(np.array(p) * k) for p in neck]


def configure(name):
    global RIG
    cfg = VARIANTS[name]
    k = cfg["scale"]
    G.clear()
    G.update(cfg, name=name, K=k, colors=PALETTES[name])
    ctrl = rest_control_points(k, cfg["head"])
    dense = cr(ctrl, 14)
    curve = Curve(dense)
    S = curve.L
    G["CTRL"] = ctrl
    G["CURVE"] = curve
    G["S"] = S
    # lift so that the lowest tube point is on the ground (z = 0)
    # (the tube rests with its bottom on z = 0 by construction: z(s) = radius(s))
    ctrl2, dense2 = [], None
    for _ in range(2):
        dense = cr(ctrl, 14)
        curve = Curve(dense)
        S = curve.L
        # re-seat the first part on the ground: z = radius of the tube
        ctrl = [(p[0], p[1], max(p[2], tube_radius(curve.cum[min(i * 14, len(curve.cum) - 1)], S, k,
                                                    cfg["thick"]))) if i <= 30 else p
                for i, p in enumerate(ctrl)]
    dense = cr(ctrl, 14)
    curve = Curve(dense)
    S = curve.L
    G.update(CTRL=ctrl, CURVE=curve, S=S)
    dl = S / N_BONES
    G["DL"] = dl
    # joints at equal arclength
    J = [V(*curve.point(i * dl)) for i in range(N_BONES + 1)]
    G["J"] = J
    G["HEAD_LEN"] = 0.17 * k * (cfg["head"] ** 0.5)
    joints = [("root", None, (0, 0, 0), (0, 0, 0.05))]
    for nm in JOINTS[1:]:
        i = BONE_IDX[nm]
        par = "root" if nm == "hips" else (
            "hips" if nm in ("tail_1", "spine_1") else
            (f"tail_{HIPS - i - 1}" if i < HIPS else
             (f"spine_{i - HIPS - 1}" if nm != "head" else f"spine_{N_BONES - 2 - HIPS}")))
        if i >= HIPS:
            h, t = J[i], J[i + 1]
        else:
            h, t = J[i + 1], J[i]
        joints.append((nm, par, h, t))
    RIG = rb.Skeleton(joints)
    G["REST_LEN"] = [(J[i + 1] - J[i]).length for i in range(N_BONES)]
    return RIG


# --------------------------------------------------------------------------- painters

def C(name):
    return br.rgb(G["colors"], name)


def put(c, mask, name):
    c[mask] = C(name)


def NSS(e0, e1, x):
    t = np.clip((x - e0) / (e1 - e0), 0.0, 1.0)
    return t * t * (3 - 2 * t)


def band_layers(sm):
    """Bands over metres-along-body sm (array, already divided by the size factor k)."""
    n = G["name"]
    if n == "female":
        P = 0.225
        ph = np.mod(sm, P)
        dark = ((ph > 0.095) & (ph < 0.117)) | ((ph > 0.150) & (ph < 0.172))
        light = (ph > 0.117) & (ph <= 0.150)
        return dark, light
    P = 0.20
    ph = np.mod(sm, P)
    dark = ((ph > 0.080) & (ph < 0.112)) | ((ph > 0.168) & (ph < 0.200))
    light = (ph >= 0.112) & (ph <= 0.168)
    return dark, light


def paint_body(U, Vv):
    """U = tail tip (0) -> neck end (1), V = ring height (1 top, 0 belly)."""
    k, S = G["K"], G["BODY_S"]
    s = U * S / k
    c = C("skin") * np.ones(U.shape + (1,), np.float32)
    dark, light = band_layers(s)
    neckz = NSS(S / k - 0.62, S / k - 0.42, s)
    # cream belly (hidden on the ground) widening to the whole front of the neck
    thr = 0.20 + 0.34 * neckz
    belly = Vv < thr
    lightc = "cream" if G["name"] != "female" else "skin2"
    put(c, light & ~belly, lightc)
    put(c, dark, "dark")
    put(c, belly, "cream")
    # the black bands stay on the neck; on the belly side they stop at the cream throat
    put(c, dark & (Vv > thr + 0.04) & (neckz > 0), "dark")
    # tail tip: darker last bit (concept: bands run to the tip)
    return c


def paint_head(U, Vv):
    """U = back of the head (0) -> nose (1), V = ring height (1 top, 0 chin)."""
    n = G["name"]
    c = C("skin") * np.ones(U.shape + (1,), np.float32)
    cheek = 0.40 + 0.07 * NSS(0.05, 0.40, U) - 0.12 * NSS(0.80, 1.0, U)
    chin = Vv < cheek
    put(c, chin, "cream")
    # smile: a line along the mouth, the corner curls up at the back
    mv = cheek - 0.015 + 0.07 * (1 - NSS(0.0, 0.22, U))
    put(c, (np.abs(Vv - mv) < 0.030) & (U > 0.02) & (U < 0.985), "smile" if n != "female" else "ink")
    # black head marking behind the eyes (male + hatchling: wide, female: none)
    if n != "female":
        put(c, (U > 0.12) & (U < 0.36 + 0.06 * (Vv - 0.8)) & (Vv > 0.80), "dark")
    # nostrils
    put(c, (np.abs(U - 0.90) < 0.020) & (np.abs(Vv - 0.76) < 0.045), "ink")
    return c


# --------------------------------------------------------------------------- mesh

def lat_up(a, prev_lat=None):
    """Ring frame for tangent a: up ~ +Z (belly down), lateral = a x Z (continuous)."""
    la = a.cross(Z)
    if la.length < 0.2:
        la = prev_lat if prev_lat is not None else X
    la = la.normalized()
    up = la.cross(a).normalized()
    return la, up


def eye_uv(at, c, f):
    f = f.normalized()
    ex = (V(0, -1, 0) - f * f.dot(V(0, -1, 0)))
    ex = ex.normalized() if ex.length > 1e-4 else X
    ey = f.cross(ex).normalized()
    if ey.dot(Z) < 0:
        ey = -ey
    cap = math.radians(78)

    def uv(i, kk, co):
        d = (co - c).normalized()
        ang = math.acos(max(-1.0, min(1.0, d.dot(f))))
        r = min(ang / cap, 1.0)
        a = math.atan2(d.dot(ey), d.dot(ex))
        return at.map_uv("eye", 0.5 + 0.5 * r * math.cos(a), 0.5 + 0.5 * r * math.sin(a))
    return uv


def build(mb, at):
    k, S, dl, J = G["K"], G["S"], G["DL"], G["J"]
    curve = G["CURVE"]
    thick, hs = G["thick"], G["head"]
    skin, ink = at.cell_uv("skin"), at.cell_uv("ink")
    hl = G["HEAD_LEN"]
    s_head0 = S - hl                          # head loft starts (inside the neck end)
    body_end = S - hl * 0.80                  # body loft ends a little inside the head
    G["BODY_S"] = body_end
    wmap = {}

    def weights_for(s):
        pos = s / dl - 0.5
        i0 = int(math.floor(pos))
        f = pos - i0
        i0c, i1c = min(max(i0, 0), N_BONES - 1), min(max(i0 + 1, 0), N_BONES - 1)
        w = {}
        w[bone_name(i0c)] = w.get(bone_name(i0c), 0) + (1 - f)
        w[bone_name(i1c)] = w.get(bone_name(i1c), 0) + f
        return w

    def key(co):
        return (round(co.x, 5), round(co.y, 5), round(co.z, 5))

    def wfn(co):
        return wmap[key(co)]

    # body rings: more rings where the tube turns tightly (coil), uniform otherwise
    nb = 46
    ss = [body_end * i / (nb - 1) for i in range(nb)]
    pts = [V(*curve.point(s)) for s in ss]
    rings = []
    prev = None
    ring_s = []
    for i, (s, p) in enumerate(zip(ss, pts)):
        a = V(*(curve.point(min(s + 0.012, S)) - curve.point(max(s - 0.012, 0)))).normalized()
        la, up = lat_up(a, prev)
        prev = la
        r = tube_radius(s, S, k, thick)
        if i == 0:
            r = max(r, 0.004)
        ring = rb.ring(p, up, la, r, r, 8, r_down=r * 0.94)
        for v in ring:
            wmap[key(v)] = weights_for(s)
        rings.append(ring)
        ring_s.append(s)
    pole_s = V(*curve.point(0)) - V(*curve.tangent(0.0)) * 0.012 * k
    wmap[key(pole_s)] = weights_for(0.0)
    pole_e = None

    def uv(i, kk, co):
        ii = min(max(i, 0), nb - 1)
        return at.map_uv("body", 0.01 + 0.98 * ring_s[ii] / body_end, rb.ring_h(kk, 8))

    ae = V(*curve.tangent(body_end))
    pole_e = pts[-1] + ae * (tube_radius(body_end, S, k, thick) * 0.7)
    wmap[key(pole_e)] = weights_for(body_end)
    mb.loft(rings, uv, wfn, pole_start=pole_s, pole_end=pole_e)
    # head loft: n = 12, rings along the end of the curve; widens behind the eyes, tapers to the nose
    hn = 12
    stops = [  # u (0 back .. 1 nose), r_up, r_down, r_lat (relative to the head size)
        (0.00, 0.034, 0.030, 0.038), (0.12, 0.060, 0.050, 0.072), (0.30, 0.064, 0.052, 0.080),
        (0.50, 0.058, 0.046, 0.072), (0.72, 0.048, 0.040, 0.058), (0.90, 0.037, 0.033, 0.044)]
    hr_s = [s_head0 + 0.010 * k + (S - s_head0 - 0.010 * k) * u for u in (0, .12, .30, .50, .72, .90)]
    hrings = []
    hu = []
    prev = None
    # the head looks a little downwards: tilt the last tangent
    for (u, ru, rd, rl), s in zip(stops, hr_s):
        p = V(*curve.point(s))
        a = V(*(curve.point(min(s + 0.012, S)) - curve.point(max(s - 0.012, 0)))).normalized()
        la, up = lat_up(a, prev)
        prev = la
        f = k * hs * 1.3
        ring = rb.ring(p, up, la, ru * f, rl * f, hn, r_down=rd * f)
        for v in ring:
            wmap[key(v)] = {"head": 1.0} if u > 0.2 else weights_for(s)
        hrings.append(ring)
        hu.append(u)
    a_end = V(*curve.tangent(S - 0.01))
    nose = V(*curve.point(S)) + a_end * (0.022 * k * hs) + V(0, 0, -0.004 * k)
    wmap[key(nose)] = {"head": 1.0}

    def huv(i, kk, co):
        ii = min(max(i, 0), len(hu) - 1)
        u = hu[ii] if i < len(hu) else 1.0
        return at.map_uv("head", 0.02 + 0.96 * (u / 0.95 if i < len(hu) else 1.0), rb.ring_h(kk, hn))

    mb.loft(hrings, huv, wfn, pole_start=None, pole_end=nose)
    G["NOSE"] = nose

    # eyes: whole balls on top-front of the head, painted eye facing forward-out
    ue = 0.40
    s_e = s_head0 + 0.010 * k + (S - s_head0 - 0.010 * k) * ue
    pe = V(*curve.point(s_e))
    ae_ = V(*(curve.point(min(s_e + 0.012, S)) - curve.point(max(s_e - 0.012, 0)))).normalized()
    la_e, up_e = lat_up(ae_)
    f = k * hs * 1.3
    R = 0.040 * k * G["eye"] * (hs ** 0.3)
    glow = []
    G["EYES"] = []
    for sg in (1, -1):
        c = pe + la_e * (sg * 0.050 * f) + up_e * (0.026 * f)
        fd = (ae_ * 1.0 + la_e * (0.55 * sg) + up_e * 0.20).normalized()
        back = lambda ri, cen, c=c, fd=fd: skin if (cen - c).normalized().dot(fd) < 0.15 else None  # noqa: E731
        _, faces = br.ellipsoid(mb, c, R, R, R * 0.97, 9, 5, eye_uv(at, c, fd), rb.rigid("head"),
                                axis=fd, face_uv=back)
        glow += [fc for fc in faces if (fc.calc_center_median() - c).normalized().dot(fd) > 0.55]
        G["EYES"].append(c)
        if G["name"] == "female":
            for ang in (62, 40, 20):
                th = math.radians(ang)
                base = c + (la_e * (sg * math.cos(th)) + ae_ * 0.15 + up_e * math.sin(th)).normalized() * (R * 0.98)
                tip = base + (la_e * (sg * math.cos(th) * 0.7) + ae_ * 0.05 + up_e * (math.sin(th) * 0.9)
                              ).normalized() * (R * 0.60)
                mid = base.lerp(tip, 0.6) - ae_ * (R * 0.08)
                br.path_loft(mb, [base, mid, tip],
                             [(R * 0.08,) * 3, (R * 0.065,) * 3, (R * 0.035,) * 3],
                             lambda i, kk, co: ink, rb.rigid("head"), 4, pole_start=True, pole_end=True)
    mb.mark_glow(glow)


# --------------------------------------------------------------------------- poses

def pose():
    return rb.Pose(RIG)


def pose_from_joints(Jt, hips_shift=None):
    """Bone rotations that put every joint at Jt[i] (|chord| == rest chord)."""
    p = pose()
    rest = G["J"]
    p.hips_offset = Jt[HIPS] - rest[HIPS]
    if hips_shift is not None:
        p.hips_offset = p.hips_offset + hips_shift
    for nm in JOINTS[1:]:
        i = BONE_IDX[nm]
        a, b = (Jt[i], Jt[i + 1]) if i >= HIPS else (Jt[i + 1], Jt[i])
        d = b - a
        w = rb.rot_between(RIG.rest_dir[nm], d)
        # keep the belly down: roll about the bone axis so that its up vector is Z-ish again
        dn = d.normalized()
        r0 = RIG.rest_dir[nm]
        u0 = Z - r0 * r0.dot(Z)
        u2 = Z - dn * dn.dot(Z)
        if u0.length > 0.2 and u2.length > 0.2:
            u1 = w @ u0.normalized()
            u2 = u2.normalized()
            ang = math.atan2(dn.dot(u1.cross(u2)), u1.dot(u2))
            w = Quaternion(dn, ang) @ w
        p.set_world(nm, w)
    return p


def solve_chain(curve, s_hips, dirs_sign=1):
    """Joints on `curve` (arclength increasing towards the head), hips joint at s_hips,
    successive joints at the rest chord lengths."""
    L = G["REST_LEN"]
    Jt = [None] * (N_BONES + 1)
    Jt[HIPS] = V(*curve.point(s_hips))
    s = s_hips
    for i in range(HIPS, N_BONES):
        r = curve.step(s, np.array(Jt[i]), L[i], +1)
        if isinstance(r, tuple):
            r = curve.L
        s = r
        Jt[i + 1] = V(*curve.point(s))
    s = s_hips
    for i in range(HIPS - 1, -1, -1):
        r = curve.step(s, np.array(Jt[i + 1]), L[i], -1)
        if isinstance(r, tuple):
            r = 0.0
        s = r
        Jt[i] = V(*curve.point(s))
    return Jt


def rest_joints():
    return [j.copy() for j in G["J"]]


def clip_idle(f, n=90):
    """Head sways slowly side to side, a little bob; the tail tip flicks now and then."""
    t = TAU * f / n
    p = pose()
    for i, b in enumerate(("spine_2", "spine_3", "spine_4", "spine_5")):
        p.rel[b] = rz((2.0 + 0.5 * i) * math.sin(t + 0.3 * i)) @ rx(0.8 * math.sin(2 * t + 0.4 * i))
    p.rel["head"] = rz(5 * math.sin(t - 0.5)) @ rx(-2.5 * math.sin(2 * t + 0.4))
    flick = max(0.0, math.sin(3 * t + 0.6)) ** 3
    for i in range(1, 9):
        w = i / 8.0
        p.rel[f"tail_{i}"] = rz(14 * w * flick * math.sin(6 * t + i * 0.7))
    p.hips_offset = V(0, 0, 0.002 * G["K"] * math.sin(2 * t))
    return p


def clip_eat(f, n=45):
    """Lean the neck forward over the food (bite at frame 20), head up, one gulp ripple."""
    wind = env(f, 0, 8, 12, 16)
    dip = env(f, 12, 20, 22, 30)
    rise = env(f, 26, 32, 36, 44)
    p = pose()
    for i, b in enumerate(("spine_1", "spine_2", "spine_3", "spine_4", "spine_5")):
        w = 0.4 + 0.6 * (i / 4)
        gulp = env(f, 26 + 2 * i, 28 + 2 * i, 29 + 2 * i, 33 + 2 * i)
        p.rel[b] = rx(-3.0 * wind * w + 9.0 * dip * w - 4.0 * rise * w + 4.0 * gulp)
    p.rel["head"] = rx(-8 * wind + 20 * dip - 5 * rise)
    return p


def clip_happy(f, n=45):
    """Neck up high, head bobs and the whole body wiggles; the tail wags."""
    t = TAU * f / n
    e = env(f, 0, 6, 38, 45)
    p = pose()
    for i, b in enumerate(("spine_1", "spine_2", "spine_3", "spine_4", "spine_5")):
        p.rel[b] = rz(7 * e * math.sin(2 * t - 0.5 * i)) @ rx(-3.5 * e + 1.5 * e * math.sin(4 * t - i))
    p.rel["head"] = rz(6 * e * math.sin(2 * t - 2.5)) @ rx(5 * e * math.sin(4 * t))
    for i in range(1, 9):
        w = i / 8.0
        p.rel[f"tail_{i}"] = rz(18 * w * e * math.sin(4 * t - i * 0.6))
    p.hips_offset = V(0, 0, 0.006 * G["K"] * e * abs(math.sin(2 * t)))
    return p


def clip_refuse(f, n=36):
    """Turn the head away and shake it (no, thank you), neck leaning back."""
    t = TAU * f / n
    e = env(f, 0, 5, 30, 36)
    p = pose()
    for i, b in enumerate(("spine_2", "spine_3", "spine_4", "spine_5")):
        p.rel[b] = rz(-7 * e * math.sin(3 * t + 0.4 * i)) @ rx(-2.5 * e)
    p.rel["head"] = rz(28 * e * math.sin(3 * t)) @ rx(-6 * e)
    return p


def clip_sleep(f, n=90):
    """Neck lies down over the coil, the head rests on it; slow breathing (loop)."""
    t = TAU * f / n
    k = G["K"]
    curve = G.get("SLEEP_CURVE")
    if curve is None:
        curve = Curve(cr(sleep_control_points(k), 14))
        G["SLEEP_CURVE"] = curve
    # hips at the same place as in the rest pose
    rest = G["J"]
    hs = int(np.argmin(np.linalg.norm(curve.p - np.array(rest[HIPS]), axis=1)))
    Jt = solve_chain(curve, float(curve.cum[hs]))
    p = pose_from_joints(Jt)
    br_ = math.sin(t)
    for b in ("spine_3", "spine_4", "spine_5"):
        p.rel[b] = rx(0.9 * br_) @ p.rel[b]
    p.rel["head"] = rx(-0.8 * br_) @ p.rel["head"]
    p.hips_offset = p.hips_offset + V(0, 0, 0.0015 * k * br_)
    return p


# ---- slither

def walk_speed():
    return G["WAVELEN"] / (WALK_FRAMES / 30.0)


def init_slither():
    k = G["K"]
    A, lam = SLITHER_A * k, SLITHER_L * k
    kw = TAU / lam
    ys = np.linspace(-6 * k - 2.5, 6 * k + 2.5, 20000)
    xs = A * np.sin(kw * ys)
    d = np.hypot(np.diff(ys), np.diff(xs))
    sig = np.concatenate([[0.0], np.cumsum(d)])
    # arclength of one wavelength
    S_lam = float(np.interp(lam, ys - ys[0], sig))
    d_h = G["S"] - HIPS * G["DL"]
    yref = float(np.interp(2.5 - d_h, sig, ys))
    G.update(WAVELEN=lam, TRACK=(ys, xs, sig, A, kw, S_lam), YREF=yref)


def slither_pose(f, n=WALK_FRAMES):
    k, S = G["K"], G["S"]
    ys, xs, sig, A, kw, S_lam = G["TRACK"]
    t = f / n
    sigma_h = 2.5 + S_lam * t                       # arclength position of the head on the track
    c = G["WAVELEN"]
    # material points: arclength d behind the head; the neck / head are raised
    Ltot = S + 0.4
    ds = np.linspace(-0.25, Ltot, 220)
    pts = []
    for d in ds:
        sg_ = sigma_h - d
        y = float(np.interp(sg_, sig, ys))
        x = A * math.sin(kw * y)
        sm = S - d                                  # arclength from the tail tip
        r = tube_radius(min(max(sm, 0.0), S), S, k, G["thick"])
        lift = (0.215 * k) * (1 - smooth(0.12 * k, 0.62 * k, d))
        pts.append((x, -(y - c * t - G['YREF']), r + lift))
    cv = Curve(np.array(pts))
    # hips on this curve: material arclength from the tail = HIPS * dl
    s_h = Ltot - (S - HIPS * G["DL"]) + 0.25 * 0 - 0.0
    # curve parameter 0 is at d = Ltot (tail side); arclength of the hips = distance from there
    d_h = S - HIPS * G["DL"]                        # material distance from the head
    idx = int(np.argmin(np.abs(ds - d_h)))
    # curve points were created for increasing d, i.e. towards the tail: reverse for solve
    cv = Curve(np.array(pts[::-1]))
    sh = float(cv.cum[len(pts) - 1 - idx])
    Jt = solve_chain(cv, sh)
    # keep the origin in the middle: remove the mean x, y of hips
    shift = V(-sum(j.x for j in Jt) / len(Jt), 0, 0)
    ycent = V(0, -sum(j.y for j in G["J"]) / len(G["J"]), 0)
    return Jt, shift


def clip_walk(f, n=WALK_FRAMES):
    Jt, shift = slither_pose(f, n)
    ym = G.setdefault("WALK_Y0", None)
    p = pose_from_joints(Jt, hips_shift=shift)
    # head looks more to the front than the track: blend its direction towards -Y
    a = Jt[N_BONES] - Jt[N_BONES - 1]
    d = a.normalized() * 0.45 + V(0, -1, 0) * 0.55
    d.z = a.normalized().z
    p.set_world("head", rb.rot_between(RIG.rest_dir["head"], d.normalized()))
    return p


def clips():
    return [rb.Clip("idle", 90, True, clip_idle),
            rb.Clip("walk", WALK_FRAMES, True, clip_walk),
            rb.Clip("eat", 45, False, clip_eat),
            rb.Clip("happy", 45, False, clip_happy),
            rb.Clip("refuse", 36, False, clip_refuse),
            rb.Clip("sleep", 90, True, clip_sleep)]


# --------------------------------------------------------------------------- main

def main():
    import bpy
    args = zb.script_args()
    name = args[args.index("--variant") + 1] if "--variant" in args else "male"
    configure(name)
    asset = G["asset"]
    rb.check_gate(asset)
    zb.clean_scene()
    bpy.context.scene.render.fps = rb.FPS
    arm = RIG.build_armature("snake_rig")
    colors = dict(G["colors"])
    atlas = rb.Atlas(256, colors, REGIONS)
    mb = rb.MeshBuilder(RIG)
    G["BODY_S"] = G["S"] - G["HEAD_LEN"] * 0.80
    build(mb, atlas)
    body_png = zb.repo_path("assets", "textures", "animals", f"{asset}_body.png")
    img = atlas.write(body_png, {
        "body": paint_body, "head": paint_head,
        "eye": br.paint_eye(colors, iris="iris", pupil_r=0.58, pupil_at=(0.0, 0.0), outer="skin",
                            iris_r=0.88)})
    mat = rb.body_material(img)
    mesh = mb.to_object(asset, mat, arm)
    init_slither()
    if "--probe" in args:
        Jt, sh = slither_pose(0)
        print("PROBE", [tuple(round(c, 3) for c in j) for j in Jt], sh)
    cl = clips()
    for c in cl:
        rb.bake_clip(RIG, arm, c)
    rb.hr.reset_pose(arm)
    bpy.context.scene.frame_set(0)
    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    print(f"{asset}: {tris} tris, {len(JOINTS)} joints, bounds x {mn.x:.3f}..{mx.x:.3f}  "
          f"y {mn.y:.3f}..{mx.y:.3f}  z {mn.z:.3f}..{mx.z:.3f}; centre line {G['S']:.3f} m, "
          f"height {mx.z:.3f} m; walk speed {walk_speed():.3f} m/s ({walk_speed() / 1.0:.3f})")
    if "--min-z" in args:
        print("min z", mn.z)
    zb.save_blend(zb.repo_path("assets", "blender", "animals", f"{asset}.blend"))
    glb = zb.repo_path("assets", "models", "animals", f"{asset}.glb")
    rb.qr.export_animal([arm, mesh], glb)
    print(f"exported {glb} ({os.path.getsize(glb) / 1024:.1f} KB)")
    k = G["K"]
    if "--no-preview" not in args:
        ls = rb.setup_preview(mesh, img, ground_z=0.0)
        out = zb.repo_path("art", "animals", asset, "model_preview.png")
        os.makedirs(os.path.dirname(out), exist_ok=True)
        rb.render_preview(arm, out, ls, "idle", 12, mx.z * 0.5, height_m=mx.z,
                          big_scale=1.25 * k * (1.0 if k > 0.6 else 1.15), close_scale=1.1 * k,
                          small_px=64, ground=rb.GRASS)
    if "--debug" in args:
        ls = rb.setup_preview(mesh, img, ground_z=0.0)
        rb.render_debug(arm, args[args.index("--debug") + 1], ls, cl, mx.z * 0.5,
                        V(*G["NOSE"]) + V(0, 0, 0.0), head_scale=0.45 * k,
                        strip_scale=1.4 * k, ground=rb.GRASS)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)


main()
