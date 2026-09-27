"""monkey — upright comic monkey (ART-ANIMALS; `biped_animal` rig, biped_rig.py).

Run:  blender -b --factory-startup --python tools/blender/animals/monkey.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (head close-ups, clip strips; not committed)

Writes
  assets/blender/animals/monkey.blend            source (never hand-edited)
  assets/models/animals/monkey.glb               game model: 1 skin (23 joints), 1 mesh, 7 clips
  assets/textures/animals/monkey_body.png        256 x 256 body atlas (flat cells + face/belly maps)
  art/animals/monkey/model_preview.png           55 deg game view | front | side | game size

Look: art/animals/monkey/{front,side,back,three_quarter}.png (approved sheet_v2).
Standing height 1.10 m (top of the hair tuft), big round head with a heart-shaped light
face, big round ears, big brown eyes, small chunky body with a light belly, long arms with
light hands, short legs with big light feet, long tail curling into a spiral behind (in the
side plane, readable from the 55 deg camera). Face and belly are painted pattern maps
(mapped per vertex by angle), not geometry.
"""

import math
import os
import sys

import bpy
import numpy as np
from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import biped_rig as br  # noqa: E402

rb = br.rb
zb = rb.zb
ASSET = "monkey"
BLEND = zb.repo_path("assets", "blender", "animals", f"{ASSET}.blend")
GLB = zb.repo_path("assets", "models", "animals", f"{ASSET}.glb")
BODY_PNG = zb.repo_path("assets", "textures", "animals", f"{ASSET}_body.png")
PREVIEW = zb.repo_path("art", "animals", ASSET, "model_preview.png")
TRI_BUDGET = 3000

COLORS = {  # art/animals/monkey/brief.md
    "fur": "#8A5A34",
    "peach": "#F1D2A8",
    "dark": "#6B4226",
    "nose": "#5A3A28",
    "eye_white": "#FFFFFF",
    "iris": "#6B3A1E",
    "pupil": "#1E1E1E",
    "ink": "#2B1B12",
}
REGIONS = {
    "head": (0, 0, 128, 112),     # face map: U = azimuth around the front axis, V = angle
    "torso": (128, 0, 128, 96),   # belly map: U = azimuth (front at 0.5), V = height
    "eye": (0, 112, 64, 64),
}

X, Y, Z = rb.X, rb.Y, rb.Z
FWD = rb.FWD
SS = rb.smoothstep
rx, ry, rz = rb.rx, rb.ry, rb.rz
env = rb.envelope
TAU = rb.TAU

# ---- head: radial surface around the front axis (-Y)
HEAD_C = Vector((0, 0.0, 0.845))
HEAD_R = (0.222, 0.195, 0.212)  # lateral, depth, vertical (top of the skull 1.057)
MUZZLE_DIR = Vector((0, -1, -0.62)).normalized()
THETA_MAX = math.radians(100)   # face map covers 0..100 deg from the front axis
EYE_FV = (0.31, 0.17)           # eye centre in front-view unit coords (x, z)
EYE_R = (0.050, 0.064)          # eye dome half-size: lateral, up

# ---- torso (z, r_lat, r_front, r_back), centre y
TORSO_Y = 0.015
TORSO = [(0.28, 0.090, 0.075, 0.075), (0.32, 0.128, 0.112, 0.105), (0.37, 0.145, 0.134, 0.120),
         (0.43, 0.148, 0.144, 0.122), (0.49, 0.146, 0.138, 0.118), (0.55, 0.150, 0.122, 0.112),
         (0.60, 0.150, 0.104, 0.102), (0.64, 0.120, 0.086, 0.086), (0.665, 0.075, 0.06, 0.06)]
TORSO_Z = (0.255, 0.68)

# ---- limbs
ARM_OUT = math.radians(20)
SHOULDER = Vector((0.155, 0.012, 0.575))
ARM_DIR = Vector((math.sin(ARM_OUT), 0, -math.cos(ARM_OUT)))
L_UP, L_LO, L_HAND = 0.17, 0.155, 0.10
HIP = Vector((0.085, 0.0, 0.335))
KNEE_Z, ANKLE_Z = 0.205, 0.075


def tail_path():
    """Tail centre line in the YZ plane: out of the rump, down-back, up, then a spiral
    (counter-clockwise seen from the left: up the back side, over the top, curling in)."""
    base = [(0.07, 0.345), (0.15, 0.305), (0.24, 0.28), (0.34, 0.295), (0.425, 0.35)]
    C = (0.365, 0.505)
    a0 = math.atan2(base[-1][1] - C[1], base[-1][0] - C[0])
    R0 = math.hypot(base[-1][0] - C[0], base[-1][1] - C[1])
    pts = [Vector((0, y, z)) for y, z in base]
    span = math.radians(390)
    for i in range(1, 21):
        u = i / 20
        a = a0 + span * u
        R = R0 * (1 - 0.70 * u ** 0.9)
        pts.append(Vector((0, C[0] + R * math.cos(a), C[1] + R * math.sin(a))))
    return pts


TAIL_PTS = tail_path()


def _arc(pts):
    acc = [0.0]
    for a, b in zip(pts, pts[1:]):
        acc.append(acc[-1] + (b - a).length)
    return [a / acc[-1] for a in acc], acc[-1]


TAIL_S, TAIL_LEN = _arc(TAIL_PTS)
TAIL_JOINT_S = [0.0, 0.17, 0.36, 0.55, 0.73, 1.0]


def point_at_s(s):
    for i in range(len(TAIL_S) - 1):
        if TAIL_S[i] <= s <= TAIL_S[i + 1]:
            k = (s - TAIL_S[i]) / (TAIL_S[i + 1] - TAIL_S[i])
            return TAIL_PTS[i].lerp(TAIL_PTS[i + 1], k)
    return TAIL_PTS[-1].copy()


P = {
    "hips": (0, 0.01, 0.36), "spine": (0, 0.01, 0.45), "chest": (0, 0.01, 0.54),
    "neck": (0, 0.01, 0.635), "head": (0, 0.0, 0.70), "head_top": (0, 0.0, 1.0),
    "shoulder": tuple(SHOULDER), "elbow": tuple(SHOULDER + ARM_DIR * L_UP),
    "wrist": tuple(SHOULDER + ARM_DIR * (L_UP + L_LO)),
    "hand_end": tuple(SHOULDER + ARM_DIR * (L_UP + L_LO + L_HAND)),
    "hip": tuple(HIP), "knee": (HIP.x, 0.0, KNEE_Z), "ankle": (HIP.x, 0.0, ANKLE_Z),
    "toe": (HIP.x, -0.15, 0.0),
    "tail": [tuple(point_at_s(s)) for s in TAIL_JOINT_S],
}
RIG = br.Rig(P)


def NSS(e0, e1, x):
    t = np.clip((x - e0) / (e1 - e0), 0.0, 1.0)
    return t * t * (3 - 2 * t)


# --------------------------------------------------------------------------- painters

def _rgb(name):
    return np.array(rb.hr.hex_rgb(COLORS[name]), np.float32)


def _canvas(U, base):
    return np.broadcast_to(_rgb(base), U.shape + (3,)).copy()


def face_mask(x, z):
    """Heart-shaped light face in front-view unit coordinates."""
    low = (x / 0.86) ** 2 + ((z + 0.24) / 0.60) ** 2 < 1.0
    lobes = np.zeros(x.shape, bool)
    for sx in (1, -1):
        lobes |= np.hypot(x - sx * EYE_FV[0], z - EYE_FV[1] - 0.02) < 0.39
    return low | lobes


def paint_head(U, V):
    th = V * THETA_MAX
    ph = U * TAU
    x, z = np.sin(th) * np.sin(ph), np.sin(th) * np.cos(ph)
    c = _canvas(U, "fur")
    front = th < math.radians(88)
    c[front & face_mask(x, z)] = _rgb("peach")
    nose = np.zeros(U.shape, bool)
    for sx in (1, -1):
        nose |= np.hypot((x - sx * 0.05) / 1.25, z + 0.085) < 0.028
    smile = (np.abs(z - (-0.335 + 1.25 * x * x)) < 0.020) & (np.abs(x) < 0.25)
    c[front & (nose | smile)] = _rgb("nose")
    return c


def paint_torso(U, V):
    z = TORSO_Z[0] + V * (TORSO_Z[1] - TORSO_Z[0])
    a = (U - 0.5) * TAU  # 0 = front
    c = _canvas(U, "fur")
    belly = (a / 0.98) ** 2 + ((z - 0.425) / 0.13) ** 2 < 1.0
    for sa in (1, -1):  # heart top: two lobes with a soft dip between them
        belly |= np.hypot((a - sa * 0.40) / 0.50, (z - 0.53) / 0.065) < 1.0
    c[belly] = _rgb("peach")
    return c


def paint_eye(U, V):
    x, y = 2 * U - 1, 2 * V - 1
    c = _canvas(U, "ink")
    r = np.hypot(x, y)
    inner = r < 0.84
    c[inner] = _rgb("eye_white")
    c[inner & (np.hypot(x + 0.06, y + 0.06) < 0.60)] = _rgb("iris")
    c[inner & (np.hypot(x + 0.06, y + 0.06) < 0.34)] = _rgb("pupil")
    c[np.hypot(x + 0.22, y - 0.22) < 0.17] = _rgb("eye_white")
    c[np.hypot(x - 0.14, y + 0.30) < 0.07] = _rgb("eye_white")
    return c


PAINTERS = {"head": paint_head, "torso": paint_torso, "eye": paint_eye}


# --------------------------------------------------------------------------- head

def head_dir(th, ph):
    return -Y * math.cos(th) + (Z * math.cos(ph) + X * math.sin(ph)) * math.sin(th)


def head_radius(d):
    a, b, c = HEAD_R
    # depth radius: slightly flatter face, rounder back of the head
    bb = b * (0.94 if d.y < 0 else 1.0)
    r = 1.0 / math.sqrt((d.x / a) ** 2 + (d.y / bb) ** 2 + (d.z / c) ** 2)
    ang = d.angle(MUZZLE_DIR)
    r += 0.058 * math.exp(-(ang / 0.46) ** 2)  # muzzle bulge
    return r


def head_point(th, ph):
    d = head_dir(th, ph)
    return HEAD_C + d * head_radius(d)


def build_head(mb, at):
    n = 18
    thetas = [math.radians(a) for a in (10, 22, 34, 46, 58, 72, 88, 104, 122, 140, 158, 170)]
    rings = [[head_point(th, TAU * k / n) for k in range(n)] for th in thetas]

    def uv(i, k, co):
        th = 0.0 if i < 0 else (math.pi if i >= len(thetas) else thetas[i])
        return at.map_uv("head", k / n, min(th / THETA_MAX, 1.0))

    mb.loft(rings, uv, rb.rigid("head"), pole_start=head_point(0.0, 0.0),
            pole_end=head_point(math.pi, 0.0))


def build_eyes(mb, at):
    for sg in (1.0, -1.0):
        fx, fz = EYE_FV[0] * sg, EYE_FV[1]
        st = math.hypot(fx, fz)
        th, ph = math.asin(st), math.atan2(fx, fz)
        S = head_point(th, ph)
        e = 1e-3
        dth = head_point(th + e, ph) - head_point(th - e, ph)
        dph = head_point(th, ph + e) - head_point(th, ph - e)
        f = dth.cross(dph).normalized()
        if f.dot(S - HEAD_C) < 0:
            f = -f
        ex = (X * sg - f * f.dot(X * sg)).normalized()  # outward (towards the ear)
        ey = f.cross(ex).normalized()
        if ey.dot(Z) < 0:
            ey = -ey
        c = S - f * 0.006
        rings = [qr_ring(c + f * off, ey, ex, EYE_R[1] * sc, EYE_R[0] * sc, 12)
                 for off, sc in ((-0.030, 0.74), (-0.014, 1.0), (0.004, 0.90), (0.014, 0.58))]

        def uv(i, k, co, c=c, ex=ex, ey=ey):
            d = co - c
            u, v = d.dot(ex) / EYE_R[0], d.dot(ey) / EYE_R[1]
            if i <= 0:  # rings behind the widest one (sunk into the body): ink rim only
                r = max(math.hypot(u, v), 1e-6)
                u, v = u / r * 0.97, v / r * 0.97
            return at.map_uv("eye", 0.5 + 0.5 * u, 0.5 + 0.5 * v)

        _, faces = mb.loft(rings, uv, rb.rigid("head"), pole_start=c - f * 0.04, pole_end=c + f * 0.020)
        mb.mark_glow(faces[-12:])  # eye cap -> `eye_glow` slot (NIGHT-006)


qr_ring = rb.ring


def build_ears(mb, at):
    fur, peach = at.cell_uv("fur"), at.cell_uv("peach")
    for s, sg in rb.SIDES:
        nrm = Vector((sg, -0.42, 0.06)).normalized()
        c = Vector((sg * 0.245, 0.035, 0.825))
        up = (Z - nrm * nrm.dot(Z)).normalized()
        lat = nrm.cross(up).normalized()
        prof = [(-0.055, 0.045), (-0.022, 0.082), (-0.004, 0.098), (0.010, 0.094), (0.018, 0.072)]
        rings = [rb.ring(c + nrm * o, up, lat, r, r, 12) for o, r in prof]

        def face_uv(i, cen, c=c, nrm=nrm):
            d = cen - c
            o = d.dot(nrm)
            rr = (d - nrm * o).length
            return peach if (o > 0.012 and rr < 0.075) or i == len(prof) - 1 else fur

        mb.loft(rings, lambda i, k, co: fur, rb.rigid("head"), pole_start=c - nrm * 0.07,
                pole_end=c + nrm * 0.014, face_uv=face_uv)


def build_tuft(mb, at):
    fur = at.cell_uv("fur")
    for dx, dy, tilt, sc in ((0.0, 0.0, 0.0, 1.0), (0.032, 0.03, 0.5, 0.75), (-0.032, 0.03, -0.5, 0.75)):
        base = Vector((dx, 0.02 + dy, 1.018))
        pts = []
        for u in (0.0, 0.35, 0.65, 0.85, 1.0):
            h = 0.085 * sc * u
            pts.append(base + Vector((tilt * h * 0.7, -0.075 * sc * u * u, h - 0.02 * u ** 3)))
        radii = [0.026 * sc, 0.021 * sc, 0.015 * sc, 0.010 * sc, 0.005]
        rb.curve_loft(mb, pts, radii, lambda i, k, co: fur, rb.rigid("head"), n=6, squash=1.6)


# --------------------------------------------------------------------------- body

def w_torso(co):
    w = rb.chain(co.z, [(0.33, "hips"), (0.41, "spine"), (0.47, "spine"), (0.53, "chest"),
                        (0.60, "chest"), (0.66, "neck")])
    for s, sg in rb.SIDES:
        if co.x * sg > 0.03 and co.z < 0.36:
            k = SS(0.36, 0.28, co.z) * SS(0.03, 0.08, co.x * sg) * 0.6
            w = rb.mix(w, {f"upper_leg_{s}": 1.0}, k)
        if co.x * sg > 0.08 and co.z > 0.52:
            k = SS(0.52, 0.60, co.z) * SS(0.08, 0.14, co.x * sg) * 0.5
            w = rb.mix(w, {f"upper_arm_{s}": 1.0}, k)
    return w


def build_torso(mb, at):
    n = 14
    rings = [rb.ring((0, TORSO_Y, z), -FWD, X, rb_, rl, n, r_down=rf) for z, rl, rf, rb_ in TORSO]
    z0, z1 = TORSO_Z

    def uv(i, k, co):
        return at.map_uv("torso", k / n, (co.z - z0) / (z1 - z0))

    mb.loft(rings, uv, w_torso, pole_start=Vector((0, TORSO_Y, z0)),
            pole_end=Vector((0, TORSO_Y, z1)))


def w_leg(s):
    up, lo, ft = br.leg_bones(s)

    def f(co):
        return rb.chain(co.z, [(0.095, ft), (0.13, lo), (KNEE_Z - 0.035, lo), (KNEE_Z + 0.035, up),
                               (0.31, up), (0.38, "hips")])
    return f


def build_legs(mb, at):
    fur, peach = at.cell_uv("fur"), at.cell_uv("peach")
    for s, sg in rb.SIDES:
        x = sg * HIP.x
        prof = [(0.37, 0.072), (0.31, 0.072), (0.25, 0.064), (0.20, 0.058), (0.15, 0.053),
                (0.11, 0.054), (0.085, 0.058)]
        rings = [rb.ring((x, 0.0, z), FWD, X, r, r, 8) for z, r in prof]
        mb.loft(rings, lambda i, k, co: fur, w_leg(s), pole_start=Vector((x, 0.0, 0.40)),
                pole_end=Vector((x, 0.0, 0.075)))
        # big light foot (rigid on foot)
        fp = [(0.055, 0.030, 0.030, 0.034), (0.035, 0.052, 0.050, 0.040), (-0.01, 0.060, 0.052, 0.042),
              (-0.07, 0.064, 0.040, 0.040), (-0.12, 0.058, 0.030, 0.038)]
        rings = [rb.ring((x, y, 0.042), Z, X, ru, rl, 10, r_down=rd) for y, rl, ru, rd in fp]
        mb.loft(rings, lambda i, k, co: peach, rb.rigid(f"foot_{s}"),
                pole_start=Vector((x, 0.075, 0.045)), pole_end=Vector((x, -0.152, 0.036)))


def w_arm(s):
    up, lo, hd = br.arm_bones(s)
    sg = 1.0 if s == "l" else -1.0
    sh = Vector((SHOULDER.x * sg, SHOULDER.y, SHOULDER.z))
    d = Vector((ARM_DIR.x * sg, 0, ARM_DIR.z))

    def f(co):
        t = (co - sh).dot(d)
        return rb.chain(t, [(-0.015, "chest"), (0.05, up), (L_UP - 0.035, up), (L_UP + 0.035, lo),
                            (L_UP + L_LO - 0.03, lo), (L_UP + L_LO + 0.01, hd)])
    return f


def build_arms(mb, at):
    fur, peach = at.cell_uv("fur"), at.cell_uv("peach")
    for s, sg in rb.SIDES:
        sh = Vector((SHOULDER.x * sg, SHOULDER.y, SHOULDER.z))
        d = Vector((ARM_DIR.x * sg, 0, ARM_DIR.z))
        lat = d.cross(FWD).normalized()
        prof = [(-0.03, 0.040), (0.01, 0.052), (0.06, 0.052), (0.12, 0.048), (L_UP, 0.046),
                (0.24, 0.043), (0.30, 0.042), (0.318, 0.050)]
        rings = [rb.ring(sh + d * t, FWD, lat, r, r, 8) for t, r in prof]
        mb.loft(rings, lambda i, k, co: fur, w_arm(s), pole_start=sh - d * 0.055,
                pole_end=sh + d * 0.328)
        # mitten hand, palm towards the body
        hp = [(0.31, 0.036, 0.028), (0.345, 0.056, 0.036), (0.385, 0.060, 0.038), (0.42, 0.048, 0.031)]
        rings = [rb.ring(sh + d * t, FWD, lat, w, th, 10) for t, w, th in hp]

        def wh(co, sh=sh, d=d, s=s):
            t = (co - sh).dot(d)
            return rb.chain(t, [(0.31, f"lower_arm_{s}"), (0.335, f"hand_{s}")])

        mb.loft(rings, lambda i, k, co: peach, wh, pole_start=sh + d * 0.30,
                pole_end=sh + d * 0.445)


def w_tail(co):
    # nearest point on the tail path -> arc parameter
    best, bs = 1e9, 0.0
    for i in range(len(TAIL_PTS) - 1):
        a, b = TAIL_PTS[i], TAIL_PTS[i + 1]
        ab = b - a
        k = min(max((co - a).dot(ab) / ab.length_squared, 0.0), 1.0)
        dd = (a + ab * k - co).length
        if dd < best:
            best, bs = dd, TAIL_S[i] + k * (TAIL_S[i + 1] - TAIL_S[i])
    js = TAIL_JOINT_S
    stops = [(0.02, "hips"), (0.07, "tail_1")]
    for j in range(1, br.TAIL_N):
        stops += [(js[j] - 0.03, f"tail_{j}"), (js[j] + 0.03, f"tail_{j + 1}")]
    return rb.chain(bs, stops)


def build_tail(mb, at):
    fur, dark = at.cell_uv("fur"), at.cell_uv("dark")
    radii = [0.034 - 0.015 * s for s in TAIL_S]
    radii[0] = 0.036

    def face_uv(i, cen):
        return dark if TAIL_S[min(i + 1, len(TAIL_S) - 1)] > 0.93 else fur

    rb.curve_loft(mb, TAIL_PTS, radii, lambda i, k, co: fur, w_tail, n=7, face_uv=face_uv)


def build_mesh(mb, at):
    build_head(mb, at)
    build_eyes(mb, at)
    build_ears(mb, at)
    build_tuft(mb, at)
    build_torso(mb, at)
    build_legs(mb, at)
    build_arms(mb, at)
    build_tail(mb, at)


# --------------------------------------------------------------------------- clips

MOUTH = Vector((0, -0.215, 0.745))


def tail_wave(p, sway=0.0, curl=0.0, phase=0.0, lift=0.0):
    """sway: side-to-side (yaw) travelling wave; curl: tighten (+) / open (-) the spiral."""
    p.rel["tail_1"] = rz(sway * math.sin(phase)) @ rx(-lift)
    for j in range(2, br.TAIL_N + 1):
        p.rel[f"tail_{j}"] = rz(sway * 0.5 * math.sin(phase - 0.7 * j)) @ rx(-curl * (0.6 + 0.2 * j))


def arms_hang(p, swing_l=0.0, swing_r=0.0, elbow=10.0, out=0.0):
    for s, sg, sw in (("l", 1.0, swing_l), ("r", -1.0, swing_r)):
        p.rel[f"upper_arm_{s}"] = rx(sw) @ ry(sg * out)
        p.rel[f"lower_arm_{s}"] = rx(-elbow)
        p.rel[f"hand_{s}"] = rx(-elbow * 0.3)


def clip_idle(f, n=90):
    t = TAU * f / n
    b = math.sin(2 * t)
    p = br.Pose(RIG)
    p.stand((0.008 * math.sin(t), 0, -0.006 + 0.004 * b), hips_rot=ry(1.5 * math.sin(t)),
            spine=rx(-1.2 * b), chest=rx(-0.8 * b) @ ry(-1.5 * math.sin(t)))
    look = env(f, 20, 32, 44, 56)
    p.rel["neck"] = rz(4 * math.sin(t))
    p.rel["head"] = rz(18 * look * math.sin(t + 0.3)) @ rx(2.5 * math.sin(2 * t + 1.0)) \
        @ ry(6 * env(f, 58, 66, 74, 84))
    arms_hang(p, 4 * math.sin(t + 0.4), 4 * math.sin(t + 2.0), elbow=12 + 3 * b)
    tail_wave(p, sway=8.0, curl=3.0 * math.sin(t), phase=t)
    return p


def walk_upper(p, fi, t):
    p.rel["spine"] = rz(-4.5 * math.sin(t)) @ rx(3.0)
    p.rel["chest"] = rz(-3.5 * math.sin(t)) @ ry(2.0 * math.sin(t))
    p.rel["neck"] = rz(2.0 * math.sin(t))
    p.rel["head"] = rx(-3.0 + 2.0 * math.cos(2 * t))
    arms_hang(p, 28 * math.cos(t), -28 * math.cos(t), elbow=18 + 8 * math.sin(t) ** 2, out=4)
    tail_wave(p, sway=9.0, curl=4.0 * math.cos(2 * t), phase=t, lift=3.0 * math.cos(2 * t))


WALK = br.make_walk(RIG, frames=14, speed=1.4, stance=0.5, lift=0.07, toe_deg=26.0,
                    fold_deg=30.0, toe_fwd=0.13, upper_fn=walk_upper, dip=0.012)

# ---- climb: vertical pole in front of the monkey (Blender (0, POLE_Y), radius 0.06);
# in-place loop, gripping hands move down at CLIMB_SPEED (the game moves the monkey up)
POLE_Y = -0.17
CLIMB_FRAMES = 30
HAND_Z = (0.88, 0.52)
CLIMB_SPEED = (HAND_Z[0] - HAND_Z[1]) / (0.5 * CLIMB_FRAMES / br.FPS)
FOOT_Z = (0.33, 0.33 - CLIMB_SPEED * 0.3 * CLIMB_FRAMES / br.FPS)


def _climb_hand(s, u):
    """u in [0,1): 0..0.5 gripping (moving down), 0.5..1 reaching up (off the pole)."""
    sg = 1.0 if s == "l" else -1.0
    hi, lo = HAND_Z
    if u < 0.5:
        z, off = hi - (hi - lo) * (u / 0.5), 0.0
    else:
        k = (u - 0.5) / 0.5
        z, off = lo + (hi - lo) * rb.ease(k), math.sin(math.pi * k)
    return Vector((sg * (0.062 + 0.05 * off), POLE_Y + 0.05 + 0.05 * off, z)), off


def _climb_foot(s, u):
    """u in [0,1): 0..0.3 gripping (moving down), rest: fold up to the next grip."""
    sg = 1.0 if s == "l" else -1.0
    hi, lo = FOOT_Z
    if u < 0.3:
        z, off = hi - (hi - lo) * (u / 0.3), 0.0
    else:
        k = (u - 0.3) / 0.7
        z, off = lo + (hi - lo) * rb.ease(k), math.sin(math.pi * k)
    return Vector((sg * (0.07 + 0.04 * off), POLE_Y + 0.08 + 0.04 * off, z)), off


def clip_climb(f, n=CLIMB_FRAMES):
    t = TAU * f / n
    u = f / n
    p = br.Pose(RIG)
    p.hips_offset = Vector((0, -0.015, 0.012 * math.sin(2 * t)))
    p.rel["hips"] = rx(6.0) @ rz(4 * math.sin(t))
    p.rel["spine"] = rx(4.0) @ rz(-3 * math.sin(t))
    p.rel["chest"] = rx(2.0) @ ry(3 * math.sin(t))
    p.rel["neck"] = rx(-12.0)
    p.rel["head"] = rx(-10.0) @ rz(4 * math.sin(t))
    for s, sg, ph in (("l", 1.0, 0.0), ("r", -1.0, 0.5)):
        T, off = _climb_hand(s, (u + ph) % 1.0)
        p.arm_ik(s, T, Vector((sg * 1.0, 0.5, -0.6)))
        hand_dir = Vector((-sg * 0.45, -1.0, 0.35 + 0.5 * off)).normalized()
        p.set_world(f"hand_{s}", rb.rot_between(RIG.rest_dir[f"hand_{s}"], hand_dir))
    for s, sg, ph in (("l", 1.0, 0.5), ("r", -1.0, 0.0)):  # diagonal to the hands
        T, off = _climb_foot(s, (u + ph) % 1.0)
        fdir = Vector((-sg * 0.45, -0.75, 0.55)).normalized()
        p.leg_ik(s, T, pole=Vector((sg * 1.0, -0.6, 0.4)),
                 foot_world=rb.rot_between(RIG.rest_dir[f"foot_{s}"], fdir))
    tail_wave(p, sway=12.0, curl=-6.0 + 4 * math.sin(t), phase=t, lift=-8.0)
    return p


def _hand_up(s, lean=0.0):
    sg = 1.0 if s == "l" else -1.0
    return Vector((-sg * 0.35, -1.0, 0.9 + lean)).normalized()


def clip_eat(f, n=60):
    k = env(f, 0, 12, 46, 60)
    bites = [22, 32, 42]
    bite = sum(env(f, c - 4, c - 1, c, c + 4) for c in bites)
    p = br.Pose(RIG)
    p.stand((0, 0, -0.012 * k), hips_rot=rx(-2 * k), spine=rx(2 * k), chest=rx(3 * k))
    p.rel["neck"] = rx(6 * k + 5 * bite)
    p.rel["head"] = rx(8 * k + 7 * bite) @ rz(3 * math.sin(TAU * f / 20) * k)
    rest = {s: p.head(f"hand_{s}") for s in ("l", "r")}
    for s, sg in rb.SIDES:
        tgt = MOUTH + Vector((sg * 0.055, -0.03, -0.085 - 0.012 * bite))
        T = rest[s].lerp(tgt, k) + Vector((0, -0.03, 0.03)) * (1 - k)
        p.arm_ik(s, T, Vector((sg * 1.0, 0.6, -0.8)))
        hd = RIG.rest_dir[f"hand_{s}"].lerp(_hand_up(s), k).normalized()
        p.set_world(f"hand_{s}", rb.rot_between(RIG.rest_dir[f"hand_{s}"], hd))
    tail_wave(p, sway=6.0, curl=2 * math.sin(TAU * f / 30), phase=TAU * f / 30)
    return p


def clip_happy(f, n=45):
    crouch = env(f, 0, 6, 6, 10)
    jump = env(f, 8, 15, 15, 23)
    land = env(f, 21, 26, 27, 36)
    clapk = env(f, 0, 3, 9, 13)      # two claps in front of the chest (wind-up)
    up = env(f, 9, 14, 28, 38)       # arms up in a V while jumping
    z = -0.07 * crouch + 0.22 * jump - 0.05 * land
    lift = max(0.0, z) + 0.06 * jump
    p = br.Pose(RIG)
    feet = {s: (p.rest_ankle(s) + Vector((0, 0.03 * jump, lift)), -25 * jump) for s in ("l", "r")}
    p.stand((0, 0, z), hips_rot=rx(4 * crouch - 3 * jump), spine=rx(-4 * jump), chest=rx(-4 * jump),
            feet=feet)
    p.rel["neck"] = rx(-8 * up)
    p.rel["head"] = rx(-8 * up) @ ry(8 * math.sin(TAU * f / 15) * up)
    clap = 0.5 + 0.5 * math.cos(TAU * (f - 2) / 5) if 2 <= f <= 12 else 1.0
    for s, sg in rb.SIDES:
        sh = p.head(f"upper_arm_{s}")
        hang = sh + Vector((sg * 0.12, -0.02, -0.30))
        front = p.head("chest") + Vector((sg * (0.04 + 0.09 * clap), -0.25, 0.06))
        vee = sh + Vector((sg * 0.17, -0.07, 0.27))
        T = hang.lerp(front, clapk).lerp(vee, up)
        p.arm_ik(s, T, Vector((sg * 1.0, 0.3, -0.6)))
        hd = Vector((-sg * 0.8, -0.4, 0.4)).normalized() * clapk + Vector((sg * 0.3, -0.2, 1.0)).normalized() * up \
            + RIG.rest_dir[f"hand_{s}"] * max(0.0, 1 - clapk - up)
        wave = rb.ry(sg * 20 * math.sin(TAU * f / 8) * up)
        p.set_world(f"hand_{s}", wave @ rb.rot_between(RIG.rest_dir[f"hand_{s}"], hd.normalized()))
    tail_wave(p, sway=20.0 * up, curl=-10 * jump + 6 * land, phase=TAU * f / 11)
    return p


def clip_refuse(f, n=36):
    e = env(f, 0, 7, 27, 36)
    sh = env(f, 6, 10, 24, 32)
    ang = 22.0 * math.sin(TAU * (f - 6) / 9) * sh
    p = br.Pose(RIG)
    p.stand((0, 0.02 * e, -0.006 * e), hips_rot=rx(-3 * e), spine=rx(-4 * e), chest=rx(-3 * e))
    p.rel["neck"] = rz(0.4 * ang)
    p.rel["head"] = rz(0.6 * ang) @ rx(-7 * e)
    rest = {s: p.head(f"hand_{s}") for s in ("l", "r")}
    # arms crossed: left forearm under (closer to the belly), right forearm over it
    tgts = {"l": Vector((-0.01, -0.19, 0.515)), "r": Vector((0.01, -0.24, 0.545))}
    for s, sg in rb.SIDES:
        T = rest[s].lerp(tgts[s] + p.hips_offset, e)
        p.arm_ik(s, T, Vector((sg * 1.0, 0.3, -1.0)))
        hd = RIG.rest_dir[f"hand_{s}"].lerp(Vector((-sg * 1.0, -0.15, 0.1)).normalized(), e).normalized()
        p.set_world(f"hand_{s}", rb.rot_between(RIG.rest_dir[f"hand_{s}"], hd))
    tail_wave(p, sway=16.0 * e, curl=4 * e, phase=TAU * f / 9)
    return p


def clip_wave(f, n=40):
    e = env(f, 0, 8, 30, 40)
    w = math.sin(TAU * (f - 8) / 8) * env(f, 6, 10, 28, 32)
    p = br.Pose(RIG)
    p.stand((0, 0, 0), hips_rot=ry(-2 * e), chest=ry(3 * e))
    p.rel["head"] = ry(-6 * e) @ rx(-4 * e)
    arms_hang(p, elbow=12)
    rest = p.head("hand_r")
    tgt = Vector((-0.30 + 0.04 * w, -0.07, 0.87))
    p.arm_ik("r", rest.lerp(tgt, e), Vector((-1.0, 0.3, -0.8)))
    hd = RIG.rest_dir["hand_r"].lerp((ry(24 * w) @ Vector((0.05, -0.15, 1.0))).normalized(), e)
    p.set_world("hand_r", rb.rot_between(RIG.rest_dir["hand_r"], hd.normalized()))
    tail_wave(p, sway=10.0 * e, phase=TAU * f / 14)
    return p


def clips():
    return [
        rb.Clip("idle", 90, True, clip_idle),
        rb.Clip("walk", 14, True, WALK),
        rb.Clip("climb", CLIMB_FRAMES, True, clip_climb),
        rb.Clip("eat", 60, False, clip_eat),
        rb.Clip("happy", 45, False, clip_happy),
        rb.Clip("refuse", 36, False, clip_refuse),
        rb.Clip("wave", 40, False, clip_wave),
    ]


# --------------------------------------------------------------------------- main

def main():
    args = zb.script_args()
    rb.check_gate(ASSET)
    zb.clean_scene()
    bpy.context.scene.render.fps = br.FPS

    arm = RIG.build_armature("monkey_rig")
    atlas = rb.Atlas(256, COLORS, REGIONS)
    body_img = atlas.write(BODY_PNG, PAINTERS)
    mat = rb.body_material(body_img)

    mb = rb.MeshBuilder(RIG)
    build_mesh(mb, atlas)
    mesh = mb.to_object(ASSET, mat, arm)

    cl = clips()
    for c in cl:
        rb.bake_clip(RIG, arm, c)
    rb.hr.reset_pose(arm)
    bpy.context.scene.frame_set(0)

    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    print(f"{ASSET}: {tris} tris, bounds x {mn.x:.3f}..{mx.x:.3f}  y {mn.y:.3f}..{mx.y:.3f}  "
          f"z {mn.z:.3f}..{mx.z:.3f}")
    print("walk body drop per frame:", " ".join(f"{d:.3f}" for d in WALK.drop))
    print(f"climb speed {CLIMB_SPEED:.2f} m/s, feet z {FOOT_Z[0]:.2f}..{FOOT_Z[1]:.2f}")

    zb.save_blend(BLEND)
    rb.qr.export_animal([arm, mesh], GLB)
    print(f"exported {GLB} ({os.path.getsize(GLB) / 1024:.1f} KB)")

    if "--no-preview" not in args:
        ls = rb.setup_preview(mesh, body_img)
        rb.render_preview(arm, PREVIEW, ls, "walk", 3, center_z=0.55, height_m=1.1,
                          big_scale=1.75, close_scale=1.5)
        if "--debug" in args:
            rb.render_debug(arm, args[args.index("--debug") + 1], ls, cl, 0.55, HEAD_C,
                            head_scale=0.75, strip_scale=1.6)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)


main()
