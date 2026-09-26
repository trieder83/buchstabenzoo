"""goldfish — comic goldfish (ART-ANIMALS; `fish` rig, fish_rig.py).

Run:  blender -b --factory-startup --python tools/blender/animals/goldfish.py [-- --no-preview]
      [-- --debug <dir>]   extra review renders (close-ups, clip strips; not committed)

Writes
  assets/blender/animals/goldfish.blend          source (never hand-edited)
  assets/models/animals/goldfish.glb             game model: 1 skin (13 joints), 1 mesh, 5 clips
  assets/textures/animals/goldfish_body.png      256 x 256 body atlas (flat cells + body/fin maps)
  art/animals/goldfish/model_preview.png         55 deg game view | front | side | game size

Look: art/animals/goldfish/{side,top,front,three_quarter}.png (approved sheet_v2).
0.60 m long (nose to tail tips), chunky egg body (0.30 m tall, 0.24 m wide), very big eyes,
cream belly, coral mouth, a big double fan tail (two forked lobes spread in a V so it reads
from above), dorsal sail, pectoral / belly / anal fins — light orange fins with rays and
cream-white edges (painted fin map).

Origin: the WATER SURFACE above the body centre (fish_rig.py). Rest pose: body centre
(hips) DEPTH = 0.16 m below the origin, so the dorsal fin tip just reaches the surface.
"""

import math
import os
import sys

import bpy
import numpy as np
from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import fish_rig as fr  # noqa: E402

rb = fr.rb
zb = rb.zb
ASSET = "goldfish"
BLEND = zb.repo_path("assets", "blender", "animals", f"{ASSET}.blend")
GLB = zb.repo_path("assets", "models", "animals", f"{ASSET}.glb")
BODY_PNG = zb.repo_path("assets", "textures", "animals", f"{ASSET}_body.png")
PREVIEW = zb.repo_path("art", "animals", ASSET, "model_preview.png")
TRI_BUDGET = 3000

COLORS = {  # art/animals/goldfish/brief.md
    "orange": "#F28A1E",
    "cream": "#FFF1DC",
    "fin": "#F5A64A",
    "fin_dark": "#E27A1C",
    "coral": "#E06A55",
    "eye_white": "#FFFFFF",
    "iris": "#3A2416",
    "pupil": "#1E1E1E",
    "ink": "#2B1B12",
}
REGIONS = {
    "body": (0, 0, 256, 96),    # U = nose -> tail stem, V = ring height (1 top, 0 belly)
    "fin": (0, 96, 128, 96),    # U = base -> tip, V = across the fin
    "eye": (128, 96, 96, 96),
}

X, Y, Z = rb.X, rb.Y, rb.Z
rx, ry, rz = rb.rx, rb.ry, rb.rz
env = rb.envelope
TAU = rb.TAU

DEPTH = 0.22
C0 = Vector((0, 0, -DEPTH))  # body centre

# ---- body: (y, r_up, r_down, r_lat) relative to C0, nose (-Y) to tail stem
BODY = [(-0.205, 0.060, 0.058, 0.052), (-0.180, 0.102, 0.102, 0.086), (-0.140, 0.134, 0.138, 0.110),
        (-0.090, 0.148, 0.153, 0.119), (-0.035, 0.150, 0.155, 0.120), (0.020, 0.138, 0.142, 0.110),
        (0.070, 0.108, 0.110, 0.088), (0.110, 0.072, 0.072, 0.060), (0.145, 0.045, 0.045, 0.038),
        (0.170, 0.036, 0.036, 0.030)]
NOSE_Y, STEM_Y = -0.222, 0.188
EYE_Y, EYE_TH = -0.128, math.radians(56)  # ring angle from the top
EYE_R = (0.048, 0.054)

P = {
    "hips": tuple(C0 + Vector((0, -0.03, 0))),
    "head": tuple(C0 + Vector((0, -0.08, 0))), "nose": tuple(C0 + Vector((0, NOSE_Y, 0))),
    "spine_1": tuple(C0 + Vector((0, 0.03, 0))), "spine_2": tuple(C0 + Vector((0, 0.08, 0))),
    "spine_3": tuple(C0 + Vector((0, 0.125, 0))), "tail_fin": tuple(C0 + Vector((0, 0.168, 0))),
    "tail_end": tuple(C0 + Vector((0, 0.38, 0))),
    "fin_dorsal": (tuple(C0 + Vector((0, -0.01, 0.125))), tuple(C0 + Vector((0, 0.075, 0.25)))),
    "fin_pec": (tuple(C0 + Vector((0.095, -0.085, -0.055))), tuple(C0 + Vector((0.175, -0.02, -0.10)))),
    "fin_pelvic": (tuple(C0 + Vector((0.045, -0.02, -0.135))), tuple(C0 + Vector((0.075, 0.03, -0.215)))),
    "fin_anal": (tuple(C0 + Vector((0, 0.085, -0.085))), tuple(C0 + Vector((0, 0.13, -0.155)))),
}
RIG = fr.Rig(P)


def NSS(e0, e1, x):
    t = np.clip((x - e0) / (e1 - e0), 0.0, 1.0)
    return t * t * (3 - 2 * t)


# --------------------------------------------------------------------------- painters

def _rgb(name):
    return np.array(rb.hr.hex_rgb(COLORS[name]), np.float32)


def _canvas(U, base):
    return np.broadcast_to(_rgb(base), U.shape + (3,)).copy()


def body_u(y):
    return (y - NOSE_Y) / (STEM_Y - NOSE_Y)


def paint_body(U, V):
    c = _canvas(U, "orange")
    edge = 0.34 + 0.03 * np.sin(U * 14.0) - 0.10 * NSS(0.55, 0.85, U) - 0.12 * (1 - NSS(0.06, 0.22, U))
    c[V < edge] = _rgb("cream")
    # gill arc, bowed backwards
    gu = 0.30 + 0.05 * (1 - ((V - 0.58) / 0.32) ** 2)
    c[(np.abs(U - gu) < 0.010) & (V > 0.30) & (V < 0.88)] = _rgb("fin_dark")
    return c


def paint_fin(U, V):
    c = _canvas(U, "fin")
    for v0 in (0.2, 0.35, 0.5, 0.65, 0.8):
        ray = np.abs(V - (0.5 + (v0 - 0.5) * (0.6 + 0.4 * U))) < 0.012 + 0.004 * (1 - U)
        c[ray & (U > 0.08) & (U < 0.80)] = _rgb("fin_dark")
    tip = U > 0.70 + 0.05 * np.cos(V * TAU * 1.5)
    edge = (V < 0.07) | (V > 0.93)
    c[tip | (edge & (U > 0.45))] = _rgb("cream")
    return c


def paint_eye(U, V):
    x, y = 2 * U - 1, 2 * V - 1
    c = _canvas(U, "ink")
    r = np.hypot(x, y)
    inner = r < 0.86
    c[inner] = _rgb("eye_white")
    c[inner & (np.hypot(x + 0.18, y + 0.02) < 0.52)] = _rgb("iris")
    c[inner & (np.hypot(x + 0.20, y + 0.02) < 0.30)] = _rgb("pupil")
    c[np.hypot(x + 0.34, y - 0.22) < 0.15] = _rgb("eye_white")
    c[np.hypot(x - 0.02, y + 0.26) < 0.07] = _rgb("eye_white")
    return c


PAINTERS = {"body": paint_body, "fin": paint_fin, "eye": paint_eye}


# --------------------------------------------------------------------------- weights

def w_body(co):
    y = co.y - C0.y
    return rb.chain(y, [(-0.115, "head"), (-0.06, "hips"), (-0.005, "hips"), (0.035, "spine_1"),
                        (0.06, "spine_1"), (0.095, "spine_2"), (0.11, "spine_2"),
                        (0.145, "spine_3"), (0.165, "spine_3"), (0.19, "tail_fin")])


def w_fin(bone, parent, base, D, length):
    def f(co):
        t = (co - base).dot(D) / length
        return rb.chain(t, [(-0.05, parent), (0.25, bone)])
    return f


# --------------------------------------------------------------------------- mesh

def body_surface(y, th):
    """Point on the body at y (relative to C0) and ring angle th (0 = top)."""
    prof = BODY
    for (y0, *r0), (y1, *r1) in zip(prof, prof[1:]):
        if y0 <= y <= y1:
            k = (y - y0) / (y1 - y0)
            ru, rd, rl = [a + (b - a) * k for a, b in zip(r0, r1)]
            break
    cu = math.cos(th)
    return C0 + Vector((0, y, 0)) + Z * (cu * (ru if cu >= 0 else rd)) + X * (math.sin(th) * rl)


def build_body(mb, at):
    n = 18
    rings = [rb.ring(C0 + Vector((0, y, 0)), Z, X, ru, rl, n, r_down=rd) for y, ru, rd, rl in BODY]
    Us = [0.0] + [body_u(y) for y, *_ in BODY] + [1.0]

    def uv(i, k, co):
        return at.map_uv("body", Us[i + 1], rb.ring_h(k, n))

    mb.loft(rings, uv, w_body, pole_start=C0 + Vector((0, NOSE_Y, -0.01)),
            pole_end=C0 + Vector((0, STEM_Y, 0)))


def build_eyes(mb, at):
    for sg in (1.0, -1.0):
        th = sg * EYE_TH
        S = body_surface(EYE_Y, th)
        e = 1e-3
        dy = body_surface(EYE_Y + e, th) - body_surface(EYE_Y - e, th)
        dt = body_surface(EYE_Y, th + e) - body_surface(EYE_Y, th - e)
        f = dy.cross(dt).normalized()
        if f.dot(S - (C0 + Vector((0, EYE_Y, 0)))) < 0:
            f = -f
        f = (f + Vector((0, -0.18, 0.0))).normalized()  # look a bit forward (big front eyes)
        ex = (-Y - f * f.dot(-Y)).normalized()           # towards the nose
        ey = f.cross(ex).normalized()
        if ey.dot(Z) < 0:
            ey = -ey
        c = S - f * 0.004
        rings = [rb.ring(c + f * off, ey, ex, EYE_R[1] * sc, EYE_R[0] * sc, 12)
                 for off, sc in ((-0.030, 0.75), (-0.012, 1.0), (0.006, 0.95), (0.022, 0.60))]

        def uv(i, k, co, c=c, ex=ex, ey=ey):
            d = co - c
            u, v = d.dot(ex) / EYE_R[0], d.dot(ey) / EYE_R[1]
            if i <= 0:  # rings behind the widest one (sunk into the body): ink rim only
                r = max(math.hypot(u, v), 1e-6)
                u, v = u / r * 0.97, v / r * 0.97
            return at.map_uv("eye", 0.5 + 0.5 * u, 0.5 + 0.5 * v)

        mb.loft(rings, uv, rb.rigid("head"), pole_start=c - f * 0.04, pole_end=c + f * 0.031)


def fin(mb, at, base, D, W, length, widths, thick, weight_fn, curl=0.0, n=8):
    """Thin fin: cross-sections along D (base -> tip), width along W, thickness along
    D x W. widths/thick: per-section half sizes at u = i / (len - 1). curl bends the fin
    towards W (m at the tip, quadratic)."""
    D, W = D.normalized(), (W - D * W.dot(D)).normalized()
    T = D.cross(W).normalized()
    m = len(widths)
    rings = []
    for i, w in enumerate(widths):
        u = i / (m - 1)
        cen = base + D * (u * length) + W * (curl * u * u)
        rings.append(rb.ring(cen, W, T, w, thick * (1 - 0.6 * u), n))
    Us = [0.0] + [i / (m - 1) for i in range(m)] + [1.0]

    def uv(i, k, co):
        return at.map_uv("fin", 0.03 + 0.95 * Us[i + 1], rb.ring_h(k, n))

    return mb.loft(rings, uv, weight_fn, pole_start=base - D * 0.012,
                   pole_end=base + D * (length + 0.01) + W * curl)


def build_fins(mb, at):
    # dorsal sail (on spine_1): swept back, widest just above the base
    b, t = (Vector(v) for v in P["fin_dorsal"])
    D = t - b
    fin(mb, at, b - D.normalized() * 0.03, D, Vector((0, -1, 0.2)), D.length + 0.03,
        [0.055, 0.075, 0.072, 0.058, 0.036, 0.012], 0.013,
        w_fin("fin_dorsal", "spine_1", b, D.normalized(), D.length), curl=-0.02)
    # tail: two tails (left / right, tilted out 32 deg), each an upper and a lower lobe
    tb = Vector(P["tail_fin"]) - Vector((0, 0.02, 0))
    for s, sg in fr.SIDES:
        tilt = math.radians(44) * sg
        up_t = Vector((math.sin(tilt), 0, math.cos(tilt)))
        for lobe, a in (("up", 34.0), ("down", -30.0)):
            ang = math.radians(a)
            D = Y * math.cos(ang) + up_t * math.sin(ang)
            W = up_t * math.cos(ang) - Y * math.sin(ang)
            base = tb + Y * (0.0 if lobe == "up" else 0.004)  # no coincident rim vertices
            fin(mb, at, base, D, W, 0.290 if lobe == "up" else 0.276,
                [0.020, 0.040, 0.058, 0.070, 0.070, 0.052, 0.018], 0.011,
                lambda co: rb.chain((co - tb).dot(Y), [(-0.005, "spine_3"), (0.03, "tail_fin")]),
                curl=0.035 * (1 if lobe == "up" else -1), n=8)
    # pectoral fins: out-back-down, roughly horizontal (visible from above)
    for s, sg in fr.SIDES:
        b, t = (Vector(rb.mirror(v, sg)) for v in P["fin_pec"])
        D = t - b
        W = Z.cross(D).normalized()
        Dn = D.normalized()
        W = W * math.cos(math.radians(35)) + Dn.cross(W) * math.sin(math.radians(35)) * sg
        fin(mb, at, b - D.normalized() * 0.02, D, W, D.length + 0.035,
            [0.022, 0.036, 0.044, 0.042, 0.030, 0.010], 0.010,
            w_fin(f"fin_pec_{s}", "hips", b, D.normalized(), D.length), curl=0.01 * sg, n=6)
    for s, sg in fr.SIDES:
        b, t = (Vector(rb.mirror(v, sg)) for v in P["fin_pelvic"])
        D = t - b
        fin(mb, at, b - D.normalized() * 0.02, D, Y, D.length + 0.03,
            [0.018, 0.030, 0.034, 0.028, 0.010], 0.009,
            w_fin(f"fin_pelvic_{s}", "hips", b, D.normalized(), D.length), n=6)
    b, t = (Vector(v) for v in P["fin_anal"])
    D = t - b
    fin(mb, at, b - D.normalized() * 0.02, D, Y, D.length + 0.03,
        [0.020, 0.032, 0.034, 0.026, 0.010], 0.009,
        w_fin("fin_anal", "spine_2", b, D.normalized(), D.length), n=6)


def build_mouth(mb, at):
    """Small round coral lips just below the nose tip (rigid on head)."""
    coral = at.cell_uv("coral")
    c = C0 + Vector((0, NOSE_Y + 0.008, -0.022))
    d = Vector((0, -1, -0.25)).normalized()
    up = (Z - d * d.dot(Z)).normalized()
    rings = [rb.ring(c + d * o, up, X, h, w, 10) for o, w, h in
             ((-0.012, 0.022, 0.012), (0.0, 0.030, 0.017), (0.009, 0.026, 0.014))]
    mb.loft(rings, lambda i, k, co: coral, rb.rigid("head"), pole_start=c - d * 0.02,
            pole_end=c + d * 0.012)


def build_mesh(mb, at):
    build_body(mb, at)
    build_mouth(mb, at)
    build_eyes(mb, at)
    build_fins(mb, at)


# --------------------------------------------------------------------------- clips

def wave(p, t, amp, lag=0.8, head=-4.0):
    """Travelling S-curve (yaw) from the head to the tail fin; amp: hips..tail_fin (deg)."""
    p.rel["head"] = rz(head * math.sin(t))
    for i, (j, a) in enumerate(zip(fr.SPINE[1:], amp[1:])):
        p.rel[j] = rz(a * math.sin(t - lag * (i + 1)))
    p.rel["fin_dorsal"] = rz(-amp[1] * 0.8 * math.sin(t - 1.2))
    p.rel["fin_anal"] = rz(-amp[2] * 0.8 * math.sin(t - 2.0))
    return rz(amp[0] * math.sin(t + 0.5))


def pec(p, flap, sweep=0.0):
    """flap: up(+)/down(-) about the body axis; sweep: fold back (+)."""
    p.rel["fin_pec_l"] = ry(-flap) @ rz(-sweep)
    p.rel["fin_pec_r"] = ry(flap) @ rz(sweep)


def pelvic(p, a):
    p.rel["fin_pelvic_l"] = ry(-a)
    p.rel["fin_pelvic_r"] = ry(a)


def clip_swim(f, n=30):
    t = 2 * TAU * f / n  # two tail beats per loop (2 Hz)
    p = fr.Pose(RIG)
    hips = wave(p, t, (3.0, 7.0, 10.0, 13.0, 20.0))
    p.rel["hips"] = hips @ rx(1.0 * math.sin(t))
    p.hips_offset = Vector((0.004 * math.sin(t), 0, 0.004 * math.sin(t + 1.0)))
    pec(p, 14 * math.sin(t + 1.0), sweep=18.0)
    pelvic(p, 8 * math.sin(t + 2.0))
    return p


def clip_idle(f, n=90):
    t = TAU * f / n
    p = fr.Pose(RIG)
    hips = wave(p, 3 * t, (1.0, 3.0, 4.0, 5.0, 9.0), head=-2.0)
    p.rel["hips"] = rz(6 * math.sin(t)) @ hips @ rx(2.5 * math.sin(2 * t))
    p.hips_offset = Vector((0, 0.006 * math.sin(t), 0.012 * math.sin(2 * t)))
    pec(p, 22 * math.sin(6 * t), sweep=2.0)
    pelvic(p, 10 * math.sin(6 * t + 1.0))
    return p


# eat: rise so the mouth reaches the surface (y = 0), nose up, two snaps
EAT_RISE, EAT_PITCH = 0.16, 25.0


def clip_eat(f, n=45):
    k = env(f, 0, 10, 32, 45)
    snap = sum(env(f, c - 3, c - 1, c, c + 3) for c in (18, 26))
    t = TAU * f / 15
    p = fr.Pose(RIG)
    hips = wave(p, t, (1.0, 4.0 + 4 * k, 5.0 + 5 * k, 7.0, 12.0), head=-2.0)
    p.rel["hips"] = rx(-EAT_PITCH * k) @ hips
    p.rel["head"] = rx(-10 * snap) @ p.rel["head"]
    p.hips_offset = Vector((0, 0.02 * k, EAT_RISE * k + 0.01 * snap))
    pec(p, 20 * math.sin(2 * t), sweep=5.0)
    pelvic(p, 10 * math.sin(2 * t))
    return p


def happy_z(f):
    """Hips height (m, relative to rest): dip, parabolic leap (out 11, peak 20, in 30),
    splash dip, settle."""
    if f <= 8:
        return -0.05 * rb.ease(f / 8)
    if f <= 31:
        u = (f - 20) / 10.5
        return -0.05 + 0.78 * (1 - u * u) if abs(u) <= 1 else -0.05
    if f <= 36:
        return -0.05 - 0.04 * math.sin(math.pi * (f - 31) / 10)
    return -0.07 * (1 - rb.ease((f - 36) / 9)) if f < 45 else 0.0


def clip_happy(f, n=45):
    if f <= 10:
        pitch = -45 * rb.ease(f / 10)
    elif f <= 30:
        pitch = -45 + 100 * (f - 10) / 20
    else:
        pitch = 55 * (1 - rb.ease((f - 30) / 12))
    fly = env(f, 8, 12, 28, 33)
    t = TAU * f / 8
    p = fr.Pose(RIG)
    hips = wave(p, t, (2.0, 8.0, 12.0, 16.0, 24.0))
    p.rel["hips"] = rx(pitch) @ hips
    for j in ("spine_1", "spine_2", "spine_3"):  # happy curl (arched back) in the air
        p.rel[j] = rx(-6 * fly) @ p.rel[j]
    p.hips_offset = Vector((0, 0, happy_z(f)))
    pec(p, 30 * fly + 10 * math.sin(t), sweep=-10 * fly)
    pelvic(p, 15 * fly)
    return p


def clip_refuse(f, n=36):
    e = env(f, 0, 8, 22, 36)
    flick = env(f, 4, 8, 16, 24)
    t = TAU * f / 9
    p = fr.Pose(RIG)
    hips = wave(p, t, (2.0, 6.0, 9.0, 12.0, 18.0 + 10 * flick), head=-3.0)
    p.rel["hips"] = rz(-75 * e) @ hips @ rx(4 * e)
    p.rel["head"] = rz(12 * math.sin(t * 1.3) * flick) @ p.rel["head"]
    p.hips_offset = Vector((0, 0.03 * e, -0.03 * e))
    pec(p, 10 * math.sin(2 * t), sweep=25 * e)
    pelvic(p, 5 * math.sin(2 * t))
    return p


def clips():
    return [
        rb.Clip("swim", 30, True, clip_swim),
        rb.Clip("idle", 90, True, clip_idle),
        rb.Clip("eat", 45, False, clip_eat),
        rb.Clip("happy", 45, False, clip_happy),
        rb.Clip("refuse", 36, False, clip_refuse),
    ]


# --------------------------------------------------------------------------- main

def main():
    args = zb.script_args()
    rb.check_gate(ASSET)
    zb.clean_scene()
    bpy.context.scene.render.fps = fr.FPS

    arm = RIG.build_armature("goldfish_rig")
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
          f"z {mn.z:.3f}..{mx.z:.3f}; length {mx.y - mn.y:.3f} m")
    p = clip_eat(22)
    print("eat: nose z at the snap", round(p.point("head", P["nose"]).z, 3))
    print("happy: max hips z", round(max(happy_z(f) for f in range(46)) - DEPTH, 3))

    zb.save_blend(BLEND)
    rb.qr.export_animal([arm, mesh], GLB)
    print(f"exported {GLB} ({os.path.getsize(GLB) / 1024:.1f} KB)")

    if "--no-preview" not in args:
        ls = rb.setup_preview(mesh, body_img, ground_z=-0.7)
        rb.render_preview(arm, PREVIEW, ls, "swim", 4, center_z=-DEPTH + 0.02, height_m=0.62,
                          big_scale=0.95, close_scale=0.8, ground=rb.WATER)
        if "--debug" in args:
            rb.render_debug(arm, args[args.index("--debug") + 1], ls, cl, -DEPTH + 0.1,
                            Vector(P["head"]), head_scale=0.5, strip_scale=1.6)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)


main()
