"""chameleon / chameleon_female / chameleon_baby — the terrarium chameleon family
(GAME-FAMILY, ART-ANIMALS "Terrarium animals", night_2; concept art/animals/chameleon_family/
family_vA2.jpg, suggestion A "panther chameleon", approved 2026-10-03).

Run:  blender -b --factory-startup --python tools/blender/animals/chameleon.py \
          -- --variant male|female|baby [--no-preview] [--debug <dir>]

Writes (per variant, ASSET = chameleon | chameleon_female | chameleon_baby)
  assets/blender/animals/<ASSET>.blend
  assets/models/animals/<ASSET>.glb          1 skin (30 joints), 1 mesh, clips idle walk perch eat
                                             happy refuse sleep; materials body + eye_glow (night level)
  assets/textures/animals/<ASSET>_body.png   body map (vertical bands) + tail map + eye + flat cells
  art/animals/<ASSET>/model_preview.png

Skeleton (30 joints, Blender Z up, the animal faces -Y = glTF +Z, left = +X):

    root                         ground point under the body, never animated
    └─ hips                      the only joint with translation
       ├─ spine ─ head           head: casque, back-crest-free; jaw = lower half of the head loft
       │          ├─ jaw         opens the mouth (smooth weights along the lip line)
       │          ├─ eye_l/r     independent turret eyes (sphere nodes, painted, `eye_glow` cap)
       │          ├─ lid_l/r     upper eyelid domes (skin colour); half closed in `sleep`, blink
       │          └─ arm_upper_l/r ─ arm_lower_l/r ─ hand_l/r
       ├─ leg_upper_l/r ─ leg_lower_l/r ─ foot_l/r
       └─ tail_1 ... tail_9      spiral tail (about 1.1 turns) in the vertical plane; rest pose =
                                 coiled, a positive X rotation per bone uncurls it

Quadruped-style skeleton built with the generic rig_base (the zoo quadruped rig has no eye / lid /
spiral tail joints), clips use the same names as the other animals.

Sizes (game metres, Q-335: 0.5 m, decided from the proportions): the male is 0.49 m long from the
nose to the back of the coil and 0.39 m high (casque), standing on the ground at the feet; female
x 0.92, baby x 0.45 with bigger head (x 1.2) and eyes (x 1.5).

Origin: on the ground between the feet (min Z = 0). `perch` = the same stance on a branch whose
TOP is the origin (feet wrap around it, tail curled tighter), the pose used at the hiding places
(branch along the facing direction; the tail coil hangs below the branch).

`walk` is a slow deliberate crawl in a diagonal gait (32 frames, authored speed WALK_SPEED m/s at
male size, x scale for female / baby = animal_anims.toml): planted hands / feet slide back at the
authored speed so nothing skates.
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
SIDES = rb.SIDES

TRI_BUDGET = 1200
BASE_K = 1.25           # base geometry (0.39 m long) -> game size 0.49 m
WALK_SPEED = 0.20       # m/s authored at male size; female / baby: x scale
WALK_FRAMES = 32
DUTY = 0.70             # fraction of the walk cycle a foot is on the ground
TAIL_BONES = 9
LID_REST = 50.0         # deg: how far the eyelid dome is tilted back when the eye is open
LID_CLOSED = -12.0      # deg: dome tilted forward over the pupil

VARIANTS = {
    "male": dict(asset="chameleon", scale=1.0, head=1.0, eye=1.0, casque=True, crest=True),
    "female": dict(asset="chameleon_female", scale=0.92, head=1.0, eye=1.0, casque=False,
                   crest=False, lashes=True),
    "baby": dict(asset="chameleon_baby", scale=0.45, head=1.2, eye=1.5, casque=False, crest=False),
}

COMMON = {"ink": "#2B1B12", "eye_white": "#FFFFFF", "pupil": "#0E0C0C"}
PALETTES = {
    "male": dict(skin="#3FBFA5", belly="#8EDCC8", pat="#C8DC3C", blot="#F4F4EA", lip="#F6E7A0",
                 iris="#E07A22", toe="#2FA58E", **COMMON),
    "female": dict(skin="#A8E3C0", belly="#E3F5DE", pat="#F7E7A1", blot="#F4F4EA", lip="#FBF0C0",
                   iris="#F2C94E", toe="#8FD0AA", **COMMON),
    "baby": dict(skin="#3FBFA5", belly="#8EDCC8", pat="#C8DC3C", blot="#F4F4EA", lip="#F6E7A0",
                 iris="#E07A22", toe="#2FA58E", **COMMON),
}
REGIONS = {"body": (0, 0, 192, 96), "eye": (192, 0, 64, 64), "tail": (0, 96, 192, 48)}

JOINTS = (["root", "hips", "spine", "head", "jaw", "eye_l", "eye_r", "lid_l", "lid_r",
           "arm_upper_l", "arm_lower_l", "hand_l", "arm_upper_r", "arm_lower_r", "hand_r",
           "leg_upper_l", "leg_lower_l", "foot_l", "leg_upper_r", "leg_lower_r", "foot_r"]
          + [f"tail_{i}" for i in range(1, TAIL_BONES + 1)])

# ---- geometry in base units (the male before the BASE_K scale), Blender axes, nose = -Y
TORSO = [  # y, z centre, r_up, r_down, r_lat
    (-0.222, 0.192, 0.020, 0.016, 0.026),
    (-0.195, 0.196, 0.036, 0.028, 0.036),
    (-0.160, 0.205, 0.056, 0.044, 0.046),
    (-0.125, 0.212, 0.066, 0.050, 0.050),
    (-0.085, 0.205, 0.078, 0.066, 0.054),
    (-0.035, 0.200, 0.086, 0.074, 0.057),
    (0.020, 0.195, 0.086, 0.074, 0.057),
    (0.065, 0.185, 0.075, 0.066, 0.050),
    (0.095, 0.175, 0.050, 0.046, 0.036),
]
HEAD_RINGS = 4          # rings 0..3 belong to the head (scaled by the variant `head`)
NOSE0 = (-0.242, 0.190)
RUMP0 = (0.108, 0.172)
EYE0 = (0.060, -0.165, 0.247)
EYE_R0 = 0.034
P0 = {
    "hips": (0, 0.065, 0.182), "spine": (0, -0.020, 0.196), "head": (0, -0.090, 0.205),
    "head_end": (0, -0.236, 0.192), "jaw0": (0, -0.128, 0.190), "jaw1": (0, -0.226, 0.182),
    "arm0": (0.042, -0.078, 0.168), "arm1": (0.100, -0.105, 0.098), "arm2": (0.072, -0.120, 0.014),
    "arm3": (0.072, -0.148, 0.007),
    "leg0": (0.040, 0.050, 0.165), "leg1": (0.102, 0.075, 0.095), "leg2": (0.074, 0.050, 0.014),
    "leg3": (0.074, 0.022, 0.007),
}
TAIL_L, TAIL_K0, TAIL_K1, TAIL_TH0 = 0.40, 6.0, 25.0, math.radians(25)
TAIL_START = (0.092, 0.180)

G = {}      # configured geometry (see configure)
RIG = None


def spiral(n=400):
    """Tail centre line (y, z) in base units: curvature rising linearly from root to tip."""
    s = np.linspace(0, TAIL_L, n)
    k = TAIL_K0 + (TAIL_K1 - TAIL_K0) * s / TAIL_L
    th = TAIL_TH0 + np.concatenate([[0], np.cumsum((k[1:] + k[:-1]) / 2 * np.diff(s))])
    d = np.stack([np.cos(th), -np.sin(th)], 1)
    p = np.concatenate([[[0, 0]], np.cumsum((d[1:] + d[:-1]) / 2 * np.diff(s)[:, None], axis=0)])
    return s, p + np.array(TAIL_START)


def tail_radius(s):
    return 0.026 * (1 - (s / TAIL_L) ** 0.9) + 0.008


class Rig(rb.Skeleton):
    def __init__(self, P, tail_pts):
        m = rb.mirror
        j = [("root", None, (0, 0, 0), (0, 0, 0.05)),
             ("hips", "root", P["hips"], P["spine"]),
             ("spine", "hips", P["spine"], P["head"]),
             ("head", "spine", P["head"], P["head_end"]),
             ("jaw", "head", P["jaw0"], P["jaw1"])]
        for s, sg in SIDES:
            j.append((f"eye_{s}", "head", G["EYE_C"][s], G["EYE_C"][s] + G["EYE_F"][s] * (0.02 * G["K"])))
        for s, sg in SIDES:
            j.append((f"lid_{s}", "head", G["EYE_C"][s], G["EYE_C"][s] + G["LID_A"][s] * (0.02 * G["K"])))
        for s, sg in SIDES:
            j += [(f"arm_upper_{s}", "spine", m(P["arm0"], sg), m(P["arm1"], sg)),
                  (f"arm_lower_{s}", f"arm_upper_{s}", m(P["arm1"], sg), m(P["arm2"], sg)),
                  (f"hand_{s}", f"arm_lower_{s}", m(P["arm2"], sg), m(P["arm3"], sg))]
        for s, sg in SIDES:
            j += [(f"leg_upper_{s}", "hips", m(P["leg0"], sg), m(P["leg1"], sg)),
                  (f"leg_lower_{s}", f"leg_upper_{s}", m(P["leg1"], sg), m(P["leg2"], sg)),
                  (f"foot_{s}", f"leg_lower_{s}", m(P["leg2"], sg), m(P["leg3"], sg))]
        for i in range(TAIL_BONES):
            j.append((f"tail_{i + 1}", "hips" if i == 0 else f"tail_{i}", tail_pts[i], tail_pts[i + 1]))
        order = [n for n, *_ in j]
        assert sorted(order) == sorted(JOINTS), set(order) ^ set(JOINTS)
        super().__init__(j)


def configure(name):
    global RIG
    cfg = VARIANTS[name]
    k = BASE_K * cfg["scale"]
    G.clear()
    G.update(cfg, name=name, K=k, colors=PALETTES[name])
    hs = cfg["head"]
    rings = []
    for i, (y, z, ru, rd, rl) in enumerate(TORSO):
        f = hs if i < HEAD_RINGS else 1.0
        rings.append((y * k, z * k, ru * k * f, rd * k * f, rl * k * f))
    G["RINGS"] = rings
    G["NOSE"] = V(0, NOSE0[0] * k, NOSE0[1] * k)
    G["RUMP"] = V(0, RUMP0[0] * k, RUMP0[1] * k)
    P = {key: V(*v) * k for key, v in P0.items()}
    G["P"] = P
    # eyes: ball centres, look direction (forward-out), upper-lid dome axis
    ex, ey, ez = EYE0
    ex *= 1.0 + 0.10 * (cfg["eye"] - 1.0)
    ez += 0.004 * (cfg["eye"] - 1.0) / 0.5
    G["EYE_R"] = EYE_R0 * k * cfg["eye"]
    G["EYE_C"], G["EYE_F"], G["LID_A"], G["LID_N"] = {}, {}, {}, {}
    for s, sg in SIDES:
        c = V(ex * sg, ey, ez) * k
        f = V(0.55 * sg, -1.0, 0.10).normalized()
        u0 = (Z - f * f.dot(Z)).normalized()
        th = math.radians(LID_REST)
        G["EYE_C"][s], G["EYE_F"][s] = c, f
        G["LID_A"][s] = (u0 * math.cos(th) - f * math.sin(th)).normalized()
        G["LID_N"][s] = u0.cross(f).normalized()
        G["U0"] = G.get("U0", {})
        G["U0"][s] = u0
    sp_s, sp_p = spiral()
    G["SP_S"], G["SP_P"] = sp_s, sp_p * k
    # bone joints on the spiral
    tail_pts = []
    for i in range(TAIL_BONES + 1):
        sv = TAIL_L * i / TAIL_BONES
        j = int(np.searchsorted(sp_s, sv))
        j = min(j, len(sp_s) - 1)
        tail_pts.append(V(0, float(sp_p[j][0]), float(sp_p[j][1])) * k)
    G["TAIL_PTS"] = tail_pts
    RIG = Rig(P, tail_pts)
    G["REST_END"] = {}
    for s, sg in SIDES:
        G["REST_END"][f"hand_{s}"] = V(P["arm2"].x * sg, P["arm2"].y, P["arm2"].z)
        G["REST_END"][f"foot_{s}"] = V(P["leg2"].x * sg, P["leg2"].y, P["leg2"].z)
    return RIG


# --------------------------------------------------------------------------- painters

def NSS(e0, e1, x):
    t = np.clip((x - e0) / (e1 - e0), 0.0, 1.0)
    return t * t * (3 - 2 * t)


def C(name):
    return br.rgb(G["colors"], name)


def put(c, mask, name):
    c[mask] = C(name)


def paint_body(U, Vv):
    """U = nose (0) -> rump (1), V = ring height (1 top, 0 belly), symmetric left/right."""
    n = G["name"]
    c = C("skin") * np.ones(U.shape + (1,), np.float32)
    belly = Vv < 0.22 - 0.03 * np.sin(U * 20)
    put(c, belly & (U > 0.12), "belly")
    # bold vertical bands, slanting backwards towards the belly like the concept
    sl = 0.05 * (1 - Vv)
    bands = ((0.40, 0.095), (0.585, 0.095), (0.77, 0.085)) if n != "baby" else ((0.43, 0.13), (0.70, 0.12))
    for u0, w in bands:
        m = (U > u0 + sl) & (U < u0 + w + sl)
        put(c, m & ~belly, "pat")
    if n == "male":
        for u0, v0, r in ((0.45, 0.80, 0.07), (0.56, 0.84, 0.055), (0.65, 0.74, 0.06),
                          (0.72, 0.64, 0.035), (0.77, 0.56, 0.025), (0.50, 0.66, 0.03)):
            put(c, np.hypot((U - u0) * 1.5, Vv - v0) < r, "blot")
    # pale lip stripe under a gentle smile line that curls up at the back; the lower half of the
    # head loft belongs to the jaw bone, so the stripe opens with the mouth
    mv = 0.465 + 0.05 * NSS(0.14, 0.30, U) - 0.03 * (1 - NSS(0.0, 0.08, U))
    lip = (Vv < mv) & (Vv > mv - 0.085) & (U < 0.30) & (U > 0.0)
    put(c, lip, "lip")
    put(c, (np.abs(Vv - mv) < 0.022) & (U > 0.0) & (U < 0.30), "ink")
    put(c, (np.abs(U - 0.055) < 0.014) & (np.abs(Vv - 0.78) < 0.03), "ink")      # nostrils
    return c


def paint_tail(U, Vv):
    """U = tail root (0) -> tip (1), V = ring height. Rings of the band colour like the concept."""
    c = C("skin") * np.ones(U.shape + (1,), np.float32)
    ph = (U * 6.2) % 1.0
    put(c, (ph > 0.15) & (ph < 0.55) & (U > 0.03) & (U < 0.97), "pat")
    return c


# --------------------------------------------------------------------------- mesh

def ring_centres():
    return [(r[0], r[1]) for r in G["RINGS"]]


def torso_z(y):
    ys, zs = zip(*ring_centres())
    return float(np.interp(y, ys, zs))


def w_body(co):
    """Torso chain + jaw: the lower half of the head loft (below the lip line) follows `jaw`."""
    k = G["K"]
    w = rb.chain(co.y, [(-0.12 * k, "head"), (-0.07 * k, "spine"), (0.0, "spine"), (0.04 * k, "hips")])
    if co.y < -0.128 * k:
        zc = torso_z(co.y)
        jw = 1.0 - rb.smoothstep(zc - 0.014 * k, zc - 0.001 * k, co.z)
        jw *= rb.smoothstep(-0.115 * k, -0.150 * k, co.y)
        if jw > 0:
            w = {b: x * (1 - jw) for b, x in w.items()}
            w["jaw"] = jw
    return w


def eye_uv(at, c, f):
    """UV of an eye ball: angle from the look direction f -> radius in the eye region."""
    f = f.normalized()
    ex = (V(0, -1, 0) - f * f.dot(V(0, -1, 0)))
    ex = ex.normalized() if ex.length > 1e-4 else X
    ey = f.cross(ex).normalized()
    if ey.dot(Z) < 0:
        ey = -ey
    cap = math.radians(78)

    def uv(i, k, co):
        d = (co - c).normalized()
        ang = math.acos(max(-1.0, min(1.0, d.dot(f))))
        r = min(ang / cap, 1.0)
        a = math.atan2(d.dot(ey), d.dot(ex))
        return at.map_uv("eye", 0.5 + 0.5 * r * math.cos(a), 0.5 + 0.5 * r * math.sin(a))
    return uv


def tail_weights(co):
    """Nearest point on the tail centre line -> blend of the two neighbouring bones."""
    sp_p = G["SP_P"]
    d = (sp_p[:, 0] - co.y) ** 2 + (sp_p[:, 1] - co.z) ** 2
    i = int(np.argmin(d))
    u = G["SP_S"][i] / TAIL_L * TAIL_BONES        # bone coordinate 0..TAIL_BONES
    b = min(int(u), TAIL_BONES - 1)
    t = u - b
    w = {}
    if t < 0.5:
        w[f"tail_{b + 1}"] = 0.5 + t
        prev = f"tail_{b}" if b > 0 else "hips"
        w[prev] = 0.5 - t
    else:
        w[f"tail_{b + 1}"] = 1.5 - t
        if b + 2 <= TAIL_BONES:
            w[f"tail_{b + 2}"] = t - 0.5
        else:
            w[f"tail_{b + 1}"] = 1.0
    return w


def limb_weights(p0, p1, p2, upper, lower):
    """Weights along a 2-segment limb polyline: blend upper -> lower around the middle joint."""
    def fn(co):
        best, bu = 1e9, 0.0
        for a, b, off in ((p0, p1, 0.0), (p1, p2, 1.0)):
            d = b - a
            t = min(max((co - a).dot(d) / d.dot(d), 0.0), 1.0)
            dist = (co - (a + d * t)).length
            if dist < best:
                best, bu = dist, off + t
        wl = rb.smoothstep(0.65, 1.35, bu)
        return {upper: 1 - wl, lower: wl}
    return fn


def build(mb, at):
    k, P = G["K"], G["P"]
    skin, toe, ink = at.cell_uv("skin"), at.cell_uv("toe"), at.cell_uv("ink")
    n = 12
    rg = G["RINGS"]
    pts = [V(0, y, z) for y, z, *_ in rg]
    radii = [(ru, rd, rl) for *_, ru, rd, rl in rg]
    cum = [0.0]
    for a, b in zip(pts, pts[1:]):
        cum.append(cum[-1] + (b - a).length)
    Us = [0.0] + [c / cum[-1] for c in cum] + [1.0]

    def uv(i, kk, co):
        return at.map_uv("body", 0.02 + 0.96 * Us[i + 1], rb.ring_h(kk, n))

    br.path_loft(mb, pts, radii, uv, w_body, n, pole_start=G["NOSE"], pole_end=G["RUMP"])

    # ---- casque (male): a rounded helmet sweeping back from the top of the head
    if G["casque"]:
        c = V(0, -0.108, 0.268) * k
        br.ellipsoid(mb, c, 0.070 * k, 0.050 * k, 0.034 * k, 8, 4, lambda i, kk, co: skin,
                     rb.rigid("head"), axis=V(0, 1, 0.42))
    # ---- back crest (male): a row of small cones along the spine
    if G["crest"]:
        for y in (-0.065, -0.036, -0.007, 0.022, 0.052):
            zt = torso_z(y * k) / k + 0.083
            base = V(0, y, zt - 0.014) * k
            tip = V(0, y + 0.004, zt + 0.015) * k
            w = rb.chain(y * k, [(-0.07 * k, "spine"), (0.0, "spine"), (0.05 * k, "hips")])
            br.path_loft(mb, [base, tip], [(0.014 * k, 0.014 * k, 0.012 * k), (0.004 * k, 0.004 * k, 0.003 * k)],
                         lambda i, kk, co: skin, lambda co, w=w: w, 4, pole_end=True)

    # ---- eyes (whole balls, painted facing forward-out, bone-driven) and eyelid domes
    glow = []
    R = G["EYE_R"]
    for s, sg in SIDES:
        c, f = G["EYE_C"][s], G["EYE_F"][s]
        back = lambda ri, cen, c=c, f=f: skin if (cen - c).normalized().dot(f) < 0.15 else None  # noqa: E731
        _, faces = br.ellipsoid(mb, c, R, R, R * 0.97, 8, 5, eye_uv(at, c, f), rb.rigid(f"eye_{s}"),
                                axis=f, face_uv=back)
        glow += [fc for fc in faces if (fc.calc_center_median() - c).normalized().dot(f) > 0.6]
        # lid dome: skin cap, axis = LID_A (tilted back when open)
        a = G["LID_A"][s]
        u1 = (X - a * a.dot(X)).normalized()
        u2 = a.cross(u1).normalized()
        R2 = R * 1.10
        rings = []
        for al in (28, 58, 92):
            t = math.radians(al)
            rings.append(rb.ring(c + a * (R2 * math.cos(t)), u1, u2, R2 * math.sin(t), R2 * math.sin(t), 7))
        mb.loft(rings, lambda i, kk, co: skin, rb.rigid(f"lid_{s}"), pole_start=c + a * R2)
        if G.get("lashes") and True:
            for ang in (62, 40, 20):
                th = math.radians(ang)
                base = c + V(sg * math.cos(th), 0.15, math.sin(th)).normalized() * (R * 1.08)
                tip = base + V(sg * math.cos(th) * 0.7, 0.08, math.sin(th) * 0.9).normalized() * (R * 0.75)
                br.path_loft(mb, [base, base.lerp(tip, 0.6) + V(0, -R * 0.08, 0), tip],
                             [(R * 0.07,) * 3, (R * 0.055,) * 3, (R * 0.025,) * 3],
                             lambda i, kk, co: ink, rb.rigid(f"lid_{s}"), 4, pole_start=True, pole_end=True)
    mb.mark_glow(glow)

    # ---- tail: tube along the spiral, UV = arc fraction
    sp_s, sp_p = G["SP_S"], G["SP_P"]
    m = 19
    idx = np.linspace(0, len(sp_s) - 1, m).astype(int)
    tp = [V(0, float(sp_p[i][0]), float(sp_p[i][1])) for i in idx]
    tr = [tail_radius(sp_s[i]) * k for i in idx]
    tn = 6

    def tuv(i, kk, co):
        return at.map_uv("tail", min(max(i, 0), m - 1) / (m - 1) * 0.96 + 0.02, rb.ring_h(kk, tn))

    br.path_loft(mb, tp, [(r, r, r) for r in tr], tuv, tail_weights, tn, pole_start=True, pole_end=True)

    # ---- limbs: 3-ring tubes (shoulder, elbow, wrist) + two toe groups per hand / foot
    def limb(pa, pb, pc, upper, lower, r0, r1, r2):
        br.path_loft(mb, [pa, pb, pc], [(r0 * k,) * 3, (r1 * k,) * 3, (r2 * k,) * 3],
                     lambda i, kk, co: skin, limb_weights(pa, pb, pc, upper, lower), 5, lat=Z,
                     pole_start=True, pole_end=True)

    def toes(end, bone, sg, fwd):
        """Two toe pads (front and rear group) on the foot bone: the grasping zygodactyl feet."""
        for d_y, rr in ((fwd, 0.030), (-0.016, 0.017)):
            c = end + V(-0.004 * sg * (1 if d_y > 0 else -1), -d_y, -0.004) * k
            br.ellipsoid(mb, c, rr * k, 0.012 * k, 0.015 * k, 5, 3, lambda i, kk, co: toe,
                         rb.rigid(bone), axis=V(0, -1, 0))

    for s, sg in SIDES:
        a0, a1, a2 = (V(P[key].x * sg, P[key].y, P[key].z) for key in ("arm0", "arm1", "arm2"))
        limb(a0, a1, a2, f"arm_upper_{s}", f"arm_lower_{s}", 0.032, 0.024, 0.017)
        toes(a2, f"hand_{s}", sg, 0.014)
        l0, l1, l2 = (V(P[key].x * sg, P[key].y, P[key].z) for key in ("leg0", "leg1", "leg2"))
        limb(l0, l1, l2, f"leg_upper_{s}", f"leg_lower_{s}", 0.038, 0.026, 0.017)
        toes(l2, f"foot_{s}", sg, 0.014)


# --------------------------------------------------------------------------- clips

def pose():
    return rb.Pose(RIG)


def sc():
    return G["scale"]


def ik_end(p, side, kind, target, roll=0.0, pitch=0.0):
    """Hand / foot IK: `kind` = "arm" or "leg"; the toe bone keeps its rest orientation + roll."""
    sg = 1 if side == "l" else -1
    up, lo, end = f"{kind}_upper_{side}", f"{kind}_lower_{side}", ("hand" if kind == "arm" else "foot") + f"_{side}"
    hip = p.head(up)
    pole = hip + V(0.30 * sg, 0.0, 0.0) * G["K"]
    p.limb_ik(up, lo, target, pole)
    p.set_world(end, ry(roll) @ rx(pitch))


ENDS = (("arm", "l", "hand_l"), ("arm", "r", "hand_r"), ("leg", "l", "foot_l"), ("leg", "r", "foot_r"))


def plant(p, roll=0.0, pitch=0.0, off=None):
    """All four hands / feet at their rest spots on the ground (optionally shifted by `off`)."""
    for kind, s, end in ENDS:
        t = G["REST_END"][end] + (off.get(s) if off else V(0, 0, 0))
        ik_end(p, s, kind, t, roll=roll * (1 if s == "l" else -1), pitch=pitch)


def tail_pose(p, curl=0.0, wave=0.0, sway=0.0, t=0.0, lag=0.55, lift=0.0):
    """curl: extra uncurl per bone in degrees (positive = uncurl, negative = tighter);
    wave / sway: amplitudes (deg) of a travelling wave in the vertical / horizontal plane."""
    for i in range(1, TAIL_BONES + 1):
        ph = t - lag * i
        p.rel[f"tail_{i}"] = rx(curl + lift * (i / TAIL_BONES) + wave * math.sin(ph + 1.1)) @ rz(sway * math.sin(ph))


def eyes(p, t, amp=28.0, tilt=8.0, closure=0.0, ph=0.0, drop=0.0, h=(1, 3), h2=(2, 1)):
    """Independent turret eyes: sums of integer harmonics (loops cleanly). Lids follow `closure`."""
    a, b = h
    c, d = h2
    yaw_l = amp * (math.sin(a * t + 0.3 + ph) * 0.8 + 0.35 * math.sin(b * t + 1.0 + ph))
    yaw_r = amp * (math.sin(c * t + 2.1 + ph) * 0.8 + 0.35 * math.sin(d * t + 0.2 + ph))
    pit_l = tilt * math.sin(2 * t + 0.6 + ph) - drop
    pit_r = tilt * math.sin(1 * t + 2.4 + ph) - drop
    p.rel["eye_l"] = rz(yaw_l) @ rx(pit_l)
    p.rel["eye_r"] = rz(yaw_r) @ rx(pit_r)
    lids(p, closure)


def lids(p, closure):
    """closure 0 = open (dome tilted back), 1 = closed over the pupil; hinge = lateral axis."""
    ang = (LID_REST - LID_CLOSED) * closure
    for s, _ in SIDES:
        p.rel[f"lid_{s}"] = Quaternion(G["LID_N"][s], math.radians(ang))


def blink(f, f0, d=6):
    return env(f, f0, f0 + d * 0.4, f0 + d * 0.6, f0 + d)


def clip_idle(f, n=120):
    t = TAU * f / n
    p = pose()
    p.rel["spine"] = rx(1.0 * math.sin(2 * t)) @ rz(2.0 * math.sin(t))
    p.rel["head"] = rz(5 * math.sin(t + 0.5)) @ rx(2 * math.sin(2 * t + 0.4))
    p.rel["jaw"] = rx(2.0 + 1.5 * math.sin(2 * t))
    p.hips_offset = V(0, 0, 0.004 * G["K"] * math.sin(2 * t))
    tail_pose(p, curl=0.0, wave=2.2, sway=3.0, t=t, lag=0.45)
    eyes(p, t, amp=30, tilt=9, closure=blink(f, 84, 8) * 1.0)
    plant(p)
    return p


def foot_target(rest, phase, s, lift):
    u = phase % 1.0
    if u < DUTY:
        y = rest.y - s / 2 + s * (u / DUTY)
        z = rest.z
    else:
        tt = (u - DUTY) / (1 - DUTY)
        sm = tt * tt * (3 - 2 * tt)
        y = rest.y + s / 2 - s * sm
        z = rest.z + lift * math.sin(math.pi * tt)
    return V(rest.x, y, z)


def clip_walk(f, n=WALK_FRAMES):
    k = G["K"]
    t = TAU * f / n
    v = WALK_SPEED * sc()
    stride = v * DUTY * n / 30.0                   # distance the planted foot slides back
    p = pose()
    p.hips_offset = V(0, 0, -0.004 * k * math.cos(2 * t))
    p.rel["spine"] = rz(6.0 * math.sin(t))          # diagonal gait: the shoulders twist
    p.rel["hips"] = rz(-3.0 * math.sin(t))
    p.rel["head"] = rz(-4.0 * math.sin(t) + 1.5 * math.sin(2 * t)) @ rx(2.0 * math.sin(2 * t + 0.5))
    p.rel["jaw"] = rx(1.5)
    tail_pose(p, curl=1.6, wave=2.0, sway=4.0, t=t, lag=0.5)
    eyes(p, t, amp=30, tilt=8, closure=blink(f, 20, 6) * 0.9, h=(1, 2), h2=(1, 3), ph=1.0)
    for kind, s, end in ENDS:
        phase = f / n + (0.0 if (kind == "arm") == (s == "l") else 0.5)
        tgt = foot_target(G["REST_END"][end], phase, stride, 0.040 * k)
        ik_end(p, s, kind, tgt)
    return p


def clip_perch(f, n=90):
    """Clinging to a branch (top = origin, along the facing): feet roll inwards around it, the
    belly hugs the branch, the tail curls tighter. Slow turret look-around + breathing."""
    k = G["K"]
    t = TAU * f / n
    p = pose()
    p.hips_offset = V(0, 0, (-0.040 + 0.003 * math.sin(t)) * k)
    p.rel["spine"] = rx(1.2 * math.sin(t)) @ rz(1.5 * math.sin(t + 1.0))
    p.rel["head"] = rz(7 * math.sin(t + 0.5)) @ rx(2 * math.sin(2 * t))
    p.rel["jaw"] = rx(1.0 + 1.0 * math.sin(t))
    tail_pose(p, curl=-3.0, wave=1.5, sway=2.0, t=t, lag=0.5)
    eyes(p, t, amp=30, tilt=8, closure=blink(f, 52, 8), h=(1, 2), h2=(1, 3), ph=2.0)
    off = {"l": V(-0.040 * k, 0, 0.0), "r": V(0.040 * k, 0, 0.0)}   # feet inwards onto the branch
    plant(p, roll=38.0, off=off)
    return p


def clip_eat(f, n=45):
    """Eyes lock forward, lean back, head lunge with the mouth open (bite at frame 20), chew."""
    k = G["K"]
    wind = env(f, 0, 9, 13, 16)
    lunge = env(f, 13, 20, 22, 29)
    chew = env(f, 24, 28, 38, 43) * math.sin(TAU * (f - 24) / 7.0)
    p = pose()
    p.rel["spine"] = rx(-4 * wind + 9 * lunge)
    p.rel["head"] = rx(-5 * wind + 9 * lunge)
    p.rel["jaw"] = rx(3 + 5 * wind + 24 * env(f, 14, 19, 21, 26) + 7 * abs(chew))
    p.hips_offset = V(0, (0.006 * wind - 0.026 * lunge) * k, (-0.003 * wind - 0.006 * lunge) * k)
    tail_pose(p, curl=-1.5 * wind + 2.0 * lunge, wave=0.0, sway=0.0, t=0.0)
    focus = env(f, 2, 8, 36, 44)
    p.rel["eye_l"] = rz(-26 * focus)          # both eyes turn to the front: left looks right
    p.rel["eye_r"] = rz(26 * focus)
    lids(p, -0.2 * focus)                      # wide open
    plant(p)
    return p


def clip_happy(f, n=45):
    """Tail uncurl wiggle (peak at frame 15), a happy body sway, eyes wide and bright."""
    k = G["K"]
    t = TAU * f / 30.0
    e = env(f, 0, 8, 34, 45)
    p = pose()
    p.rel["spine"] = rz(7 * e * math.sin(t * 1.0)) @ rx(-3 * e)
    p.rel["head"] = rz(-6 * e * math.sin(t)) @ rx(-7 * e)
    p.rel["jaw"] = rx(4 * e)
    p.hips_offset = V(0, 0, 0.008 * k * e * abs(math.sin(t)))
    tail_pose(p, curl=7.0 * e, wave=16.0 * e, sway=10.0 * e, t=t * 1.5 + 0.6, lag=0.7)
    p.rel["eye_l"] = rz(20 * e * math.sin(t * 2.0)) @ rx(12 * e)
    p.rel["eye_r"] = rz(-20 * e * math.sin(t * 2.0)) @ rx(12 * e)
    lids(p, -0.25 * e)
    plant(p)
    return p


def clip_refuse(f, n=36):
    """Head turns away and shakes (no, thank you); the eyes roll in circles."""
    k = G["K"]
    t = TAU * f / n
    e = env(f, 0, 5, 30, 36)
    p = pose()
    p.rel["head"] = rz(34 * e + 8 * e * math.sin(3 * t))
    p.rel["spine"] = rz(10 * e + 3 * e * math.sin(3 * t + 0.6))
    p.rel["jaw"] = rx(1.0)
    tail_pose(p, curl=-2.0 * e, wave=0.0, sway=5.0 * e, t=3 * t, lag=0.4)
    p.rel["eye_l"] = rz(28 * e * math.cos(2 * t)) @ rx(26 * e * math.sin(2 * t))
    p.rel["eye_r"] = rz(28 * e * math.cos(2 * t + 1.6)) @ rx(26 * e * math.sin(2 * t + 1.6))
    lids(p, 0.35 * e)
    plant(p)
    return p


def clip_sleep(f, n=90):
    """Settled low, curled tight, head bowed, eyes half closed, slow breathing."""
    k = G["K"]
    t = TAU * f / n
    p = pose()
    p.hips_offset = V(0, 0, (-0.030 + 0.003 * math.sin(t)) * k)
    p.rel["spine"] = rx(4 + 1.2 * math.sin(t))
    p.rel["head"] = rx(9 + 1.2 * math.sin(t))
    p.rel["jaw"] = rx(0.5)
    tail_pose(p, curl=-4.0, wave=0.8, sway=0.0, t=t, lag=0.3)
    p.rel["eye_l"] = rz(8 * math.sin(t * 0.0) + 12) @ rx(-10)
    p.rel["eye_r"] = rz(-12) @ rx(-10)
    lids(p, 0.85 + 0.02 * math.sin(t))
    plant(p)
    return p


def clips():
    return [rb.Clip("idle", 120, True, clip_idle),
            rb.Clip("walk", WALK_FRAMES, True, clip_walk),
            rb.Clip("perch", 90, True, clip_perch),
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
    arm = RIG.build_armature("chameleon_rig")
    colors = dict(G["colors"])
    atlas = rb.Atlas(256, colors, REGIONS)
    body_png = zb.repo_path("assets", "textures", "animals", f"{asset}_body.png")
    img = atlas.write(body_png, {
        "body": paint_body, "tail": paint_tail,
        "eye": br.paint_eye(colors, iris="iris", pupil_r=0.42, pupil_at=(0.0, 0.0), outer="skin",
                            iris_r=0.86)})
    mat = rb.body_material(img)
    mb = rb.MeshBuilder(RIG)
    build(mb, atlas)
    mesh = mb.to_object(asset, mat, arm)
    cl = clips()
    for c in cl:
        rb.bake_clip(RIG, arm, c)
    rb.hr.reset_pose(arm)
    bpy.context.scene.frame_set(0)
    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    print(f"{asset}: {tris} tris, {len(JOINTS)} joints, bounds x {mn.x:.3f}..{mx.x:.3f}  "
          f"y {mn.y:.3f}..{mx.y:.3f}  z {mn.z:.3f}..{mx.z:.3f}; length {mx.y - mn.y:.3f} m, "
          f"height {mx.z:.3f} m")
    zb.save_blend(zb.repo_path("assets", "blender", "animals", f"{asset}.blend"))
    glb = zb.repo_path("assets", "models", "animals", f"{asset}.glb")
    rb.qr.export_animal([arm, mesh], glb)
    print(f"exported {glb} ({os.path.getsize(glb) / 1024:.1f} KB)")
    if "--no-preview" not in args:
        ls = rb.setup_preview(mesh, img, ground_z=0.0)
        out = zb.repo_path("art", "animals", asset, "model_preview.png")
        os.makedirs(os.path.dirname(out), exist_ok=True)
        rb.render_preview(arm, out, ls, "walk", 6, mx.z * 0.5, height_m=mx.z,
                          big_scale=1.35 * G["scale"], close_scale=1.1 * G["scale"], small_px=64,
                          ground=rb.GRASS)
    if "--debug" in args:
        ls = rb.setup_preview(mesh, img, ground_z=0.0)
        rb.render_debug(arm, args[args.index("--debug") + 1], ls, cl, mx.z * 0.5,
                        G["P"]["head"] + V(0, -0.04, 0.04), head_scale=0.45 * G["scale"],
                        strip_scale=1.0 * G["scale"], ground=rb.GRASS)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)


main()
