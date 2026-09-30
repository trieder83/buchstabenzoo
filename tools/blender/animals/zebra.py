"""zebra — start-mission animal (ART-ANIMALS; rig/clips per the quadruped rig conventions).

Run:  blender -b --factory-startup --python tools/blender/animals/zebra.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (head close-ups, clip strips; not committed)

Writes
  assets/blender/animals/zebra.blend             source (never hand-edited)
  assets/models/animals/zebra.glb                game model: 1 skin (23 joints), 1 mesh, 6 clips
  assets/textures/animals/zebra_body.png         256 x 256 body atlas (flat cells + stripe maps)
  art/animals/zebra/model_preview.png            55 deg game view | front | side | game size

Look: art/animals/zebra/{front,side,back,three_quarter}.png (approved turnaround).
Chunky comic zebra, withers 1.30 m, big round head with big eyes, black muzzle, upright
striped mane, tail with a black tuft. Stripes are NOT geometry: they are painted into
pattern-map regions of the body atlas and mapped per vertex along each body part, so they
stay broad and clean independent of the mesh resolution (few and bold for the 55 deg camera).
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
ASSET = "zebra"
BLEND = zb.repo_path("assets", "blender", "animals", f"{ASSET}.blend")
GLB = zb.repo_path("assets", "models", "animals", f"{ASSET}.glb")
BODY_PNG = zb.repo_path("assets", "textures", "animals", f"{ASSET}_body.png")
PREVIEW = zb.repo_path("art", "animals", ASSET, "model_preview.png")
TRI_BUDGET = 3000

# brief colours (art/animals/zebra/brief.md), checked against the turnaround
COLORS = {
    "white": "#F4F1E8",
    "black": "#262626",
    "muzzle": "#2E2E2E",
    "pink": "#E8A0A8",
    "eye_white": "#FFFFFF",
    "iris": "#6B3A1E",
    "pupil": "#1E1E1E",
    "ink": "#2B1B12",
}
REGIONS = {  # pattern maps in the 256 x 256 atlas (px, top-left origin)
    "torso": (0, 0, 256, 80),
    "neck": (0, 80, 128, 40),
    "head": (128, 80, 128, 40),
    "leg": (0, 120, 32, 72),
    "tail": (32, 120, 32, 72),
    "eye": (64, 120, 64, 64),
    "mane": (128, 120, 128, 72),
}

X = Vector((1, 0, 0))
Z = Vector((0, 0, 1))

# ---- body (barrel): superellipse profile along Y, front (-Y) to back
BODY_L = 0.62
BODY_CZ = 0.93
BODY_R = (0.37, 0.355, 0.38)  # lateral, up, down
BODY_P = 2.5

# ---- neck: straight axis from inside the chest to inside the skull
NECK_A = Vector((0, -0.34, 1.00))
NECK_B = Vector((0, -0.64, 1.66))
NECK_AX = (NECK_B - NECK_A).normalized()
NECK_LEN = (NECK_B - NECK_A).length
NECK_UP = (Z - NECK_AX * NECK_AX.dot(Z)).normalized()  # crest side (up/back)
NECK_PROFILE = [  # s, r_up (crest), r_down (throat), r_lat
    (0.00, 0.22, 0.22, 0.21), (0.20, 0.215, 0.215, 0.20), (0.40, 0.20, 0.20, 0.185),
    (0.60, 0.185, 0.19, 0.175), (0.80, 0.175, 0.185, 0.17), (1.00, 0.17, 0.17, 0.16)]

# ---- head: loft along a forward-down axis from the back of the skull (B) to the nose (N)
HEAD_C = Vector((0, -0.72, 1.78))
HEAD_D = Vector((0, -0.62, -0.50)).normalized()
HEAD_U = (Z - HEAD_D * HEAD_D.dot(Z)).normalized()  # forehead side
HEAD_LEN = 0.80
HEAD_B = HEAD_C - HEAD_D * 0.24
HEAD_N = HEAD_B + HEAD_D * HEAD_LEN
HEAD_PROFILE = [  # s (m from B), r_lat, r_up (forehead), r_down (jaw)
    (0.03, 0.14, 0.12, 0.12), (0.09, 0.23, 0.20, 0.20), (0.17, 0.285, 0.245, 0.24),
    (0.26, 0.30, 0.26, 0.25), (0.35, 0.285, 0.25, 0.235), (0.43, 0.24, 0.22, 0.21),
    (0.50, 0.19, 0.19, 0.19), (0.57, 0.178, 0.185, 0.195), (0.64, 0.182, 0.19, 0.20),
    (0.71, 0.168, 0.175, 0.182), (0.765, 0.12, 0.13, 0.13)]
EYE_S = 0.30
EYE_R = (0.090, 0.104)  # along eye-x (towards the nose), eye-y (up)

# variant knobs (zebra_foal.py / zebra_female.py set them before main(); defaults = adult male)
MANE_W = 0.046     # mane half width (m)
MANE_H = 1.0       # mane height factor
TUFT_K = 1.0       # tail tuft radius factor
FORELOCK = False   # swept forelock over the forehead (female, Q-203)
LASHES = False     # heavy upper lid + outer flick on the eye swatch (female, Q-203)

# ---- skeleton (ART-ANIMALS rig conventions): joint positions of the zebra
P = {
    "hips": (0, 0.30, 0.98), "spine": (0, 0.02, 1.00), "chest": (0, -0.26, 1.02),
    "neck_1": (0, -0.42, 1.14), "neck_2": (0, -0.52, 1.38), "head": (0, -0.60, 1.58),
    "nose": tuple(HEAD_N),
    "ear_base": (0.15, -0.66, 1.97), "ear_tip": (0.25, -0.62, 2.22),
    "tail_1": (0, 0.60, 1.12), "tail_2": (0, 0.70, 0.88), "tail_end": (0, 0.745, 0.62),
    "front_xy": (0.21, -0.34), "front_z": (0.92, 0.48, 0.13),
    "hind_xy": (0.21, 0.36), "hind_z": (0.94, 0.50, 0.13),
    "toe": 0.07,
}
RIG = qr.Rig(P)
SS = qr.smoothstep


def NSS(e0, e1, x):
    """numpy smoothstep for the painters"""
    t = np.clip((x - e0) / (e1 - e0), 0.0, 1.0)
    return t * t * (3 - 2 * t)


# --------------------------------------------------------------------------- manifest gate

def check_gate():
    """ART-PIPELINE §2: no modelling before the concept is approved."""
    text = open(zb.repo_path("assets", "manifest.toml"), encoding="utf-8").read()
    for block in text.split("[[asset]]")[1:]:
        if re.search(r'^id\s*=\s*"%s"' % ASSET, block, re.M):
            if re.search(r"^concept_approved\s*=\s*true", block, re.M):
                return
    print(f"{ASSET}: concept not approved in assets/manifest.toml — not modelling")
    sys.exit(1)


# --------------------------------------------------------------------------- painters

def _rgb(name):
    return np.array(qr.hr.hex_rgb(COLORS[name]), np.float32)


def _canvas(U, base="white"):
    return np.broadcast_to(_rgb(base), U.shape + (3,)).copy()


TORSO_STRIPES = [  # (U centre, width in U, tilt) — U = 0 chest front, 1 rump end
    (0.205, 0.056, 0.02), (0.315, 0.060, 0.03), (0.415, 0.062, 0.03),
    (0.515, 0.062, 0.02), (0.615, 0.062, -0.01), (0.715, 0.060, -0.04), (0.815, 0.056, -0.05),
    (0.905, 0.046, -0.03)]


def paint_torso(U, V):
    c = _canvas(U)
    belly = 1.0 - NSS(0.10, 0.22, np.minimum(U, 1 - U))  # 1 near the poles: no white belly there
    taper = np.maximum(NSS(0.10, 0.42, V), belly)
    m = np.zeros(U.shape, bool)
    for i, (u0, w, tilt) in enumerate(TORSO_STRIPES):
        centre = u0 + tilt * (0.6 - V) + 0.006 * np.sin(V * 9.0 + i * 2.1)
        m |= np.abs(U - centre) < 0.5 * w * (0.2 + 0.8 * taper)
    m &= (V > 0.10) | (belly > 0.5)
    c[m] = _rgb("black")
    return c


def paint_neck(U, V):
    c = _canvas(U)
    m = np.zeros(U.shape, bool)
    for i, u0 in enumerate((0.36, 0.57, 0.78)):
        w = 0.105 * (0.35 + 0.65 * NSS(0.0, 0.5, V))
        m |= np.abs(U - (u0 + 0.02 * (V - 0.5) + 0.01 * np.sin(V * 7 + i))) < 0.5 * w
    c[m] = _rgb("black")
    return c


def paint_head(U, V):
    c = _canvas(U)
    m = np.abs(U - (0.20 + 0.02 * (V - 0.5))) < 0.5 * 0.075 * (0.3 + 0.7 * NSS(0.15, 0.5, V))
    m |= (np.abs(U - (0.515 - 0.03 * (V - 0.5))) < 0.5 * 0.06 * NSS(0.12, 0.45, V)) & (V < 0.84)
    m |= (V > 0.972) & (U > 0.26) & (U < 0.64)       # forehead centre line
    c[m] = _rgb("black")
    c[U > 0.625 + 0.035 * (V - 0.5)] = _rgb("muzzle")
    return c


LEG_BANDS = [(0.175, 0.232), (0.292, 0.349), (0.409, 0.466), (0.526, 0.583), (0.643, 0.70)]


def paint_leg(U, V):  # V = height z (m)
    c = _canvas(U)
    m = V < 0.112
    for a, b in LEG_BANDS:
        m |= (V > a) & (V < b)
    c[m] = _rgb("black")
    return c


TAIL_Z = (0.62, 1.16)


def paint_tail(U, V):
    c = _canvas(U)
    m = V < 0.08
    for v0 in (0.24, 0.46, 0.68):
        m |= np.abs(V - v0) < 0.055
    c[m] = _rgb("black")
    return c


def paint_eye(U, V):
    x, y = 2 * U - 1, 2 * V - 1
    c = _canvas(U, "ink")
    r = np.hypot(x, y)
    inner = r < 0.86
    c[inner] = _rgb("eye_white")
    if LASHES:  # heavy upper lid and an outer (rear) lash flick
        c[inner & (y > 0.42) & (r > 0.50)] = _rgb("ink")
        c[inner & (np.hypot(x + 0.60, y - 0.52) < 0.30)] = _rgb("ink")
    iris = inner & (np.hypot(x - 0.17, y + 0.02) < 0.58)
    c[iris] = _rgb("iris")
    c[inner & (np.hypot(x - 0.19, y + 0.02) < 0.31)] = _rgb("pupil")
    c[np.hypot(x - 0.02, y - 0.25) < 0.16] = _rgb("eye_white")
    c[np.hypot(x - 0.37, y + 0.27) < 0.075] = _rgb("eye_white")
    return c


def paint_mane(U, V):
    c = _canvas(U)
    m = V > 0.76
    for u0 in (0.10, 0.28, 0.46, 0.64):
        m |= np.abs(U - u0) < 0.05
    m |= U > 0.84  # forelock tuft
    c[m] = _rgb("black")
    return c


PAINTERS = {"torso": paint_torso, "neck": paint_neck, "head": paint_head, "leg": paint_leg,
            "tail": paint_tail, "eye": paint_eye, "mane": paint_mane}


# --------------------------------------------------------------------------- weights

def w_torso(co):
    w = qr.chain(co.y, [(-0.34, "chest"), (-0.20, "chest"), (-0.04, "spine"), (0.08, "spine"),
                        (0.24, "hips")])
    for leg in qr.LEGS:
        up = qr.leg_bones(leg)[0]
        h = RIG.rest_head[up]
        if h.x * co.x <= 0:
            continue
        d = math.hypot(co.x - h.x, co.y - h.y)
        k = SS(0.82, 0.62, co.z) * SS(0.22, 0.08, d) * 0.45
        if k > 0:
            w = qr.mix(w, {up: 1.0}, k)
    return w


def neck_s(co):
    return (co - NECK_A).dot(NECK_AX) / NECK_LEN


def w_neck(co):
    return qr.chain(neck_s(co), [(0.06, "chest"), (0.22, "neck_1"), (0.42, "neck_1"),
                                 (0.62, "neck_2"), (0.80, "neck_2"), (0.95, "head")])


def w_leg(leg):
    up, lo, ft = qr.leg_bones(leg)
    top, knee = RIG.rest_head[up].z, RIG.rest_head[lo].z
    parent = qr.PARENT[up]

    def f(co):
        return qr.chain(co.z, [(0.112, ft), (0.165, lo), (knee - 0.06, lo), (knee + 0.06, up),
                               (top - 0.12, up), (top + 0.04, parent)])
    return f


def w_tail(co):
    return qr.chain(co.z, [(0.62, "tail_2"), (0.82, "tail_2"), (0.94, "tail_1"),
                           (1.08, "tail_1"), (1.17, "hips")])


def w_ear(side, base, axis, length):
    def f(co):
        t = (co - base).dot(axis) / length
        return qr.chain(t, [(0.02, "head"), (0.22, f"ear_{side}")])
    return f


# --------------------------------------------------------------------------- mesh

def meridian_params(centres, radii, start, end):
    """Cumulative arc length along the ring centres/radii from the start pole to the end
    pole, normalised to [0, 1]: index 0 = start pole, then the rings, last = end pole."""
    pts = [start] + [(c, r) for c, r in zip(centres, radii)] + [end]
    acc = [0.0]
    for (c0, r0), (c1, r1) in zip(pts, pts[1:]):
        acc.append(acc[-1] + math.hypot(c1 - c0, r1 - r0))
    return [a / acc[-1] for a in acc], acc[-1]


def build_torso(mb, at):
    n = 16
    phis = [(-0.5 + k / 15) * math.pi for k in range(1, 15)]
    ys = [BODY_L * math.sin(ph) for ph in phis]  # front (-Y) to back
    rings, fs = [], []
    for y in ys:
        f = (1 - abs(y / BODY_L) ** BODY_P) ** (1 / BODY_P)
        fs.append(f)
        rings.append(qr.ring((0, y, BODY_CZ), Z, X, BODY_R[1] * f, BODY_R[0] * f, n,
                             r_down=BODY_R[2] * f))
    ravg = [f * sum(BODY_R) / 3 for f in fs]
    tU, length = meridian_params(ys, ravg, (-BODY_L, 0.0), (BODY_L, 0.0))

    def uv(i, k, co):
        return at.map_uv("torso", tU[i + 1], qr.ring_h(k, n))

    mb.loft(rings, uv, w_torso, pole_start=Vector((0, -BODY_L, BODY_CZ)),
            pole_end=Vector((0, BODY_L, BODY_CZ)))
    return length


def build_neck(mb, at):
    n = 12
    lat = X
    rings, ss = [], []
    for s, ru, rd, rl in NECK_PROFILE:
        ss.append(s)
        rings.append(qr.ring(NECK_A + NECK_AX * (s * NECK_LEN), NECK_UP, lat, ru, rl, n, r_down=rd))
    U = [0.0] + ss + [1.0]

    def uv(i, k, co):
        return at.map_uv("neck", U[i + 1], qr.ring_h(k, n))

    mb.loft(rings, uv, w_neck, pole_start=NECK_A - NECK_AX * 0.04,
            pole_end=NECK_B + NECK_AX * 0.03)


def head_surface(s, th):
    """Point on the head loft surface at axial distance s and ring angle th (0 = forehead)."""
    prof = HEAD_PROFILE
    for (s0, *r0), (s1, *r1) in zip(prof, prof[1:]):
        if s0 <= s <= s1:
            k = (s - s0) / (s1 - s0)
            rl, ru, rd = [a + (b - a) * k for a, b in zip(r0, r1)]
            break
    cu = math.cos(th)
    return HEAD_B + HEAD_D * s + HEAD_U * (cu * (ru if cu >= 0 else rd)) + X * (math.sin(th) * rl)


def build_head(mb, at):
    n = 14
    rings = [qr.ring(HEAD_B + HEAD_D * s, HEAD_U, X, ru, rl, n, r_down=rd)
             for s, rl, ru, rd in HEAD_PROFILE]
    U = [0.0] + [s / HEAD_LEN for s, *_ in HEAD_PROFILE] + [1.0]

    def uv(i, k, co):
        return at.map_uv("head", U[i + 1], qr.ring_h(k, n))

    mb.loft(rings, uv, qr.rigid("head"), pole_start=HEAD_B, pole_end=HEAD_N)


def build_eyes(mb, at):
    th = math.radians(36.0)
    for sg in (1.0, -1.0):
        S = head_surface(EYE_S, sg * th)
        # outward normal from finite differences on the surface
        e = 1e-3
        ds = head_surface(EYE_S + e, sg * th) - head_surface(EYE_S - e, sg * th)
        dt = head_surface(EYE_S, sg * th + e) - head_surface(EYE_S, sg * th - e)
        f = ds.cross(dt).normalized()
        if f.dot(S - (HEAD_B + HEAD_D * EYE_S)) < 0:
            f = -f
        ex = (HEAD_D - f * HEAD_D.dot(f)).normalized()  # towards the nose
        ey = f.cross(ex).normalized()
        if ey.dot(Z) < 0:
            ey = -ey
        c = S - f * 0.004
        rings = []
        for off, sc in ((-0.034, 0.72), (-0.016, 1.0), (0.010, 0.86), (0.024, 0.52)):
            rings.append(qr.ring(c + f * off, ey, ex, EYE_R[1] * sc, EYE_R[0] * sc, 12))

        def uv(i, k, co, c=c, ex=ex, ey=ey):
            d = co - c
            return at.map_uv("eye", 0.5 + 0.5 * d.dot(ex) / EYE_R[0], 0.5 + 0.5 * d.dot(ey) / EYE_R[1])

        _, faces = mb.loft(rings, uv, qr.rigid("head"), pole_start=c - f * 0.045, pole_end=c + f * 0.031)
        mb.mark_glow(faces[-12:])  # eye cap -> `eye_glow` slot (NIGHT-006)


def build_ears(mb, at):
    white, pink = at.cell_uv("white"), at.cell_uv("pink")
    for side, sg in qr.SIDES:
        base = Vector(qr.mirror(P["ear_base"], sg))
        tip = Vector(qr.mirror(P["ear_tip"], sg))
        ax = (tip - base).normalized()
        length = (tip - base).length
        fr = (qr.FWD - ax * ax.dot(qr.FWD)).normalized()  # inner (pink) side faces forward
        lat = ax.cross(fr).normalized()
        prof = [(-0.12, 0.055, 0.035), (0.22, 0.078, 0.036), (0.55, 0.068, 0.032), (0.82, 0.038, 0.022)]
        rings = [qr.ring(base + ax * (t * length), fr, lat, th, w, 8) for t, w, th in prof]

        def face_uv(i, cen, base=base, ax=ax, fr=fr, length=length):
            t = (cen - base).dot(ax) / length
            d = cen - (base + ax * (t * length))
            return pink if (d.dot(fr) > 0.016 and 0.1 < t < 0.9) else white

        mb.loft(rings, lambda i, k, co: white, w_ear(side, base, ax, length),
                pole_start=base - ax * (0.16 * length), pole_end=tip + ax * 0.01, face_uv=face_uv)


def build_mane(mb, at):
    # path: withers -> neck crest -> over the skull -> forelock between the ears
    pts, nrm, hgt = [], [], []
    for s in (0.24, 0.40, 0.56, 0.72, 0.88, 1.0):
        prof = NECK_PROFILE
        ru = next(r for (s0, r, *_), (s1, *_rest) in zip(prof, prof[1:]) if s0 <= s <= s1) \
            if s < 1.0 else prof[-1][1]
        pts.append(NECK_A + NECK_AX * (s * NECK_LEN) + NECK_UP * (ru - 0.03))
        nrm.append(NECK_UP)
        hgt.append(0.16 * MANE_H)
    for deg in (40.0, 22.0, 4.0, -14.0):
        d = Vector((0, math.sin(math.radians(deg)), math.cos(math.radians(deg))))
        pts.append(HEAD_C + d * 0.235)
        nrm.append(d)
        hgt.append((0.19 if deg < 10 else 0.17) * MANE_H)
    acc = [0.0]
    for a, b in zip(pts, pts[1:]):
        acc.append(acc[-1] + (b - a).length)
    sU = [a / acc[-1] for a in acc]
    wd = MANE_W
    prof = [(-1.0, 0.0), (-1.0, 0.55), (-0.6, 1.0), (0.6, 1.0), (1.0, 0.55), (1.0, 0.0)]
    vtab = [0.0, 0.55, 1.0, 1.0, 0.55, 0.0]
    rings = []
    for p, nv, hh in zip(pts, nrm, hgt):
        rings.append([p + X * (a * wd) + nv * (b * hh) for a, b in prof])
    U = [0.0] + sU + [1.0]

    def uv(i, k, co):
        kk = k % 6
        k0 = int(math.floor(kk))
        v = vtab[k0] if kk == k0 else 0.5 * (vtab[k0] + vtab[(k0 + 1) % 6])
        return at.map_uv("mane", U[i + 1], v)

    t0 = (pts[1] - pts[0]).normalized()
    t1 = (pts[-1] - pts[-2]).normalized()
    mb.loft(rings, uv, w_neck, pole_start=pts[0] - t0 * 0.05 + nrm[0] * 0.04,
            pole_end=pts[-1] + t1 * 0.05 + nrm[-1] * 0.08)


LEG_PROFILE = [  # z, r_front/back, r_lat  (front leg; hind legs are thicker on top)
    (0.90, 0.100, 0.098), (0.74, 0.096, 0.092), (0.60, 0.087, 0.083), (0.48, 0.078, 0.075),
    (0.32, 0.069, 0.067), (0.155, 0.064, 0.062), (0.112, 0.082, 0.080), (0.05, 0.090, 0.087),
    (0.0, 0.086, 0.083)]


def build_legs(mb, at):
    n = 8
    for leg in qr.LEGS:
        up = qr.leg_bones(leg)[0]
        h = RIG.rest_head[up]
        hind = leg.startswith("hind")
        rings = []
        for z, rf, rl in LEG_PROFILE:
            k = SS(0.55, 0.85, z) if hind else 0.0
            rf, rl = rf * 1.12 * (1 + 0.28 * k), rl * 1.12 * (1 + 0.22 * k)
            yoff = -0.012 if z < 0.12 else 0.0
            rings.append(qr.ring((h.x, h.y + yoff, z), qr.FWD, X, rf, rl, n))

        def uv(i, k, co):
            return at.map_uv("leg", 0.5, co.z)

        mb.loft(rings, uv, w_leg(leg), pole_start=Vector((h.x, h.y, 0.95)),
                pole_end=Vector((h.x, h.y - 0.012, 0.0)))


def build_tail(mb, at):
    pts = [Vector(p) for p in ((0, 0.55, 1.16), (0, 0.63, 1.03), (0, 0.69, 0.89), (0, 0.72, 0.77),
                               (0, 0.735, 0.68))]
    radii = [0.038, 0.034, 0.031, 0.029, 0.028]
    rings = []
    for i, (p, r) in enumerate(zip(pts, radii)):
        a = (pts[min(i + 1, 4)] - pts[max(i - 1, 0)]).normalized()
        up = (Vector((0, 1, 0)) - a * a.y).normalized()
        rings.append(qr.ring(p, up, X, r, r, 6))
    z0, z1 = TAIL_Z

    def uv(i, k, co):
        return at.map_uv("tail", 0.5, (co.z - z0) / (z1 - z0))

    mb.loft(rings, uv, w_tail, pole_start=pts[0] + Vector((0, -0.04, 0.03)),
            pole_end=pts[-1] + Vector((0, 0.005, -0.03)))
    # tuft
    black = at.cell_uv("black")
    tuft = [(0.71, 0.034), (0.64, 0.062 * TUFT_K), (0.555, 0.072 * TUFT_K), (0.47, 0.056 * TUFT_K)]
    rings = [qr.ring((0, 0.74 + 0.02 * (0.71 - z), z), Vector((0, 1, 0)), X, r, r * 0.9, 8)
             for z, r in tuft]
    mb.loft(rings, lambda i, k, co: black, w_tail, pole_start=Vector((0, 0.735, 0.74)),
            pole_end=Vector((0, 0.755, 0.385)))


def build_forelock(mb, at):
    """Swept black forelock lying on the forehead (female, Q-203): a flat tapered lock from the
    top of the skull down between the eyes, bending to one side."""
    black = at.cell_uv("black")
    rings = []
    for s, deg, w, t in ((0.12, 2.0, 0.060, 0.050), (0.20, 6.0, 0.085, 0.048), (0.29, 10.0, 0.095, 0.042),
                         (0.37, 14.0, 0.075, 0.036), (0.44, 17.0, 0.040, 0.026)):
        th = math.radians(deg)
        nrm = (HEAD_U * math.cos(th) + X * math.sin(th)).normalized()
        c = head_surface(s, th) + nrm * (t * 0.55)
        lat = nrm.cross(HEAD_D).normalized()
        rings.append(qr.ring(c, nrm, lat, t, w, 6))
    mb.loft(rings, lambda i, k, co: black, qr.rigid("head"),
            pole_start=rings[0][0] - HEAD_D * 0.06, pole_end=head_surface(0.49, math.radians(19)) + HEAD_U * 0.02)


def build_mesh(mb, at):
    length = build_torso(mb, at)
    build_neck(mb, at)
    build_head(mb, at)
    build_eyes(mb, at)
    build_ears(mb, at)
    build_mane(mb, at)
    if FORELOCK:
        build_forelock(mb, at)
    build_legs(mb, at)
    build_tail(mb, at)
    return length


# --------------------------------------------------------------------------- clips

rx, ry, rz = qr.rx, qr.ry, qr.rz
env = qr.envelope
TAU = 2 * math.pi


def tail_swish(p, amp, phase, lift=4.0):
    p.rel["tail_1"] = rz(amp * math.sin(phase)) @ rx(lift)
    p.rel["tail_2"] = rz(amp * 1.3 * math.sin(phase - 1.0)) @ rx(lift * 0.5)


def ears(p, back=0.0, out=0.0, l_extra=None, r_extra=None):
    p.rel["ear_l"] = ry(out) @ rx(-back)
    p.rel["ear_r"] = ry(-out) @ rx(-back)
    if l_extra:
        p.rel["ear_l"] = l_extra @ p.rel["ear_l"]
    if r_extra:
        p.rel["ear_r"] = r_extra @ p.rel["ear_r"]


def clip_idle(f, n=90):
    t = TAU * f / n
    p = qr.Pose(RIG)
    b = math.sin(2 * t)
    p.stand((0.006 * math.sin(t), 0, -0.008 + 0.004 * b), hips_rot=ry(0.8 * math.sin(t)),
            chest=rx(-0.6 * b))
    p.rel["neck_1"] = rx(-1.2 * b)
    p.rel["neck_2"] = rz(5.0 * math.sin(t + 0.5))
    p.rel["head"] = rx(2.0 * math.sin(2 * t + 1.0)) @ rz(3.0 * math.sin(t))
    tail_swish(p, 14.0, 3 * t)
    e1 = env(f, 28, 31, 33, 39)
    e2 = env(f, 60, 63, 64, 70)
    ears(p, back=3.0 * math.sin(t), l_extra=ry(18 * e2) @ rx(-30 * e2),
         r_extra=ry(-18 * e1) @ rx(-35 * e1))
    return p


def walk_upper(p, fi, t):
    p.rel["neck_1"] = rx(-1.0 + 2.0 * math.cos(2 * t))
    p.rel["neck_2"] = rz(-1.5 * math.sin(t))
    p.rel["head"] = rx(1.0 - 2.5 * math.cos(2 * t + 0.5))
    tail_swish(p, 9.0, t, lift=6.0)
    ears(p, back=2.0 * math.sin(2 * t))


WALK = qr.make_walk(RIG, frames=20, speed=1.4, stance=0.52, lift=0.10, toe_deg=35.0,
                    fold_deg=55.0, toe_fwd=0.07, upper_fn=walk_upper)


@lru_cache(maxsize=None)
def head_down_angle(target_z, body):
    p = qr.Pose(RIG)
    _low_body(p, 1.0, body)
    return p.head_down(target_z, SPLIT)


def _low_body(p, k, body):
    dz, pitch = body
    p.stand((0, 0, -dz * k), hips_rot=rx(pitch * k), spine=rx(2.0 * k))


EAT_BODY = (0.03, 6.0)
EAT_NOSE_Z = 0.15
SPLIT = (0.66, 0.34, -0.70)  # head turns back up so the nose points down, not under the chest


def clip_eat(f, n=60):
    k = env(f, 0, 14, 46, 60)
    p = qr.Pose(RIG)
    _low_body(p, k, EAT_BODY)
    A = head_down_angle(EAT_NOSE_Z, EAT_BODY) * k
    bite = sum(env(f, c - 4, c - 1, c, c + 4) for c in (22, 31, 40))
    p.rel["neck_1"] = rx(A * SPLIT[0])
    p.rel["neck_2"] = rx(A * SPLIT[1] + 3.0 * bite)
    p.rel["head"] = rx(A * SPLIT[2] - 9.0 * bite)
    tail_swish(p, 10.0, TAU * f / 30)
    ears(p, back=6.0 * k, out=8.0 * k)
    return p


DRINK_BODY = (0.05, 8.0)
DRINK_NOSE_Z = 0.10


def clip_drink(f, n=60):
    t = TAU * f / n
    p = qr.Pose(RIG)
    _low_body(p, 1.0, DRINK_BODY)
    A = head_down_angle(DRINK_NOSE_Z, DRINK_BODY)
    lap = math.sin(4 * t)
    p.rel["neck_1"] = rx(A * SPLIT[0])
    p.rel["neck_2"] = rx(A * SPLIT[1] + 1.5 * math.sin(4 * t + 1.0))
    p.rel["head"] = rx(A * SPLIT[2] - 3.0 * lap)
    tail_swish(p, 12.0, 2 * t)
    ears(p, back=4.0 + 3.0 * math.sin(t), out=10.0)
    return p


def clip_happy(f, n=45):
    crouch = env(f, 0, 6, 6, 10)
    jump = env(f, 8, 15, 15, 23)
    land = env(f, 21, 26, 27, 36)
    toss = env(f, 6, 14, 22, 38)
    z = -0.06 * crouch + 0.17 * jump - 0.05 * land
    dz = max(0.0, z) + 0.06 * jump
    lift = {"front_l": (0.05 * jump, dz, -40 * jump), "front_r": (0.05 * jump, dz, -40 * jump),
            "hind_l": (0.02 * jump, dz * 0.9, -25 * jump), "hind_r": (0.02 * jump, dz * 0.9, -25 * jump)}
    p = qr.Pose(RIG)
    p.stand((0, 0, z), hips_rot=rx(-5 * jump + 3 * crouch), lift=lift)
    wig = math.sin(TAU * f / 12)
    p.rel["neck_1"] = rx(-16 * toss)
    p.rel["neck_2"] = rx(-8 * toss) @ rz(6 * wig * toss)
    p.rel["head"] = rx(-12 * toss + 5 * crouch)
    p.rel["tail_1"] = rz(22 * wig * toss) @ rx(4 + 35 * toss)
    p.rel["tail_2"] = rz(28 * math.sin(TAU * f / 12 - 1) * toss) @ rx(15 * toss)
    ears(p, back=-10 * toss, out=12 * toss)
    return p


def clip_refuse(f, n=36):
    e = env(f, 0, 6, 26, 36)
    sh = env(f, 3, 8, 24, 32)
    ang = 24.0 * math.sin(TAU * (f - 4) / 11) * sh
    p = qr.Pose(RIG)
    p.stand((0, 0.03 * e, -0.01 * e), hips_rot=rx(-2.0 * e))
    p.rel["neck_1"] = rx(-7 * e)
    p.rel["neck_2"] = rz(0.4 * ang)
    p.rel["head"] = rz(0.6 * ang) @ rx(-4 * e)
    tail_swish(p, 18.0 * e, TAU * f / 9)
    ears(p, back=32 * e, out=22 * e)
    return p


def clips():
    return [
        qr.Clip("idle", 90, True, clip_idle),
        qr.Clip("walk", 20, True, WALK),
        qr.Clip("eat", 60, False, clip_eat),
        qr.Clip("drink", 60, True, clip_drink),
        qr.Clip("happy", 45, False, clip_happy),
        qr.Clip("refuse", 36, False, clip_refuse),
    ]


# --------------------------------------------------------------------------- main

def main():
    args = zb.script_args()
    check_gate()
    zb.clean_scene()
    bpy.context.scene.render.fps = qr.FPS

    arm = RIG.build_armature("zebra_rig")
    atlas = qr.Atlas(256, COLORS, REGIONS)
    body_img = atlas.write(BODY_PNG, PAINTERS)
    mat = qr.body_material(body_img)

    mb = qr.MeshBuilder()
    torso_len = build_mesh(mb, atlas)
    mesh = mb.to_object(ASSET, mat, arm)

    cl = clips()
    for c in cl:
        qr.bake_clip(RIG, arm, c)
    qr.hr.reset_pose(arm)
    qr.apply_variant(arm, mesh)
    bpy.context.scene.frame_set(0)

    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    print(f"{ASSET}: {tris} tris, bounds x {mn.x:.3f}..{mx.x:.3f}  y {mn.y:.3f}..{mx.y:.3f}  "
          f"z {mn.z:.3f}..{mx.z:.3f}; torso meridian {torso_len:.2f} m")
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in WALK.drop))
    for name, nz, body in (("eat", EAT_NOSE_Z, EAT_BODY), ("drink", DRINK_NOSE_Z, DRINK_BODY)):
        p = qr.Pose(RIG)
        _low_body(p, 1.0, body)
        a = p.head_down(nz, SPLIT)
        print(f"{name}: head-down angle {a:.1f} deg, nose at "
              f"{tuple(round(c, 3) for c in p.point('head', P['nose']))} (target z {nz})")

    zb.save_blend(BLEND)
    qr.export_animal([arm, mesh], GLB)
    print(f"exported {GLB} ({os.path.getsize(GLB) / 1024:.1f} KB)")

    if "--no-preview" not in args:
        ls = qr.setup_preview(mesh, body_img)
        sc = qr.VARIANT["scale"]
        qr.render_preview(arm, PREVIEW, ls, center_z=1.05 * sc, walk_frame=4, big_scale=3.4 * sc,
                          close_scale=2.7 * sc, height_m=2.2 * sc)
        if "--debug" in args:
            qr.render_debug(arm, args[args.index("--debug") + 1], ls, cl, 1.05 * sc, HEAD_C * sc,
                            scale=sc)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)


if __name__ == "__main__":
    main()
