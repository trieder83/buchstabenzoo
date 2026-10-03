"""poison_dart_frog / poison_dart_frog_female / frog_froglet — the terrarium frog family
(GAME-FAMILY, ART-ANIMALS "Terrarium animals", night_2; concept art/animals/poison_dart_frog_family/
family_v6.jpg, approved 2026-10-03).

Run:  blender -b --factory-startup --python tools/blender/animals/poison_dart_frog.py \
          -- --variant male|female|froglet [--no-preview] [--debug <dir>]

Writes (per variant, ASSET = poison_dart_frog | poison_dart_frog_female | frog_froglet)
  assets/blender/animals/<ASSET>.blend
  assets/models/animals/<ASSET>.glb          1 skin (11 joints), 1 mesh, clips idle walk eat happy
                                             refuse sleep; materials body + eye_glow (night level)
  assets/textures/animals/<ASSET>_body.png   body map + limb map + eye + flat colour cells
  art/animals/<ASSET>/model_preview.png

Same small `frog` rig as the ambient pond frog (frog.py: 11 joints root/hips/spine/head/throat/
arm_l,r/leg_upper_l,r/leg_lower_l,r) — one skin for the three variants, different geometry:
  male     length ~0.40 m (nose to rump), eye tops ~0.33 m; black, yellow bands (head, back, hip),
           yellow bands on the legs, yellow toe tips
  female   0.92 x male; orange, black eye patch + neck band + back blotches, yellow-green legs
           with black spots, pale toe tips, eyelashes
  froglet  0.45 x male; rounder, bigger head and eyes (x1.5 relative), orange with black blotches,
           yellow legs, flat tail stump

Origin: on the ground at the feet (min z = 0). Pattern: body map (U nose -> rump, V ring height,
mirror-symmetric) + limb map (U along the limb piece, V ring height), flat colour cells for toes.

`walk` is a looping HOP cycle (24 frames, authored speed 1.0 m/s = 0.8 m per hop): the hips rise
and fall on a vertical arc (no forward root motion); planted feet slide back at the authored
speed during the push/landing frames so nothing skates (WALK_SPEED below = animal_anims.toml).
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
BASE_K = 2.2            # ambient frog (0.18 m) -> 0.40 m
WALK_SPEED = 1.0        # m/s authored hop speed (animal_anims.toml walk.speed)
HOP_FRAMES = 24
TAKEOFF, LAND = 6, 22   # frames of the walk hop (feet leave / touch the ground)
PITCH = 8.0            # extra nose-up posture of the sitting frog (deg)

VARIANTS = {
    "male": dict(asset="poison_dart_frog", scale=1.0, head=1.0, eye=1.0),
    "female": dict(asset="poison_dart_frog_female", scale=0.92, head=1.0, eye=1.0),
    "froglet": dict(asset="frog_froglet", scale=0.45, head=1.18, eye=1.55),
}

COMMON = {"ink": "#2B1B12", "eye_white": "#FFFFFF", "iris": "#2A1E1C", "pupil": "#0E0C0C"}
PALETTES = {
    "male": dict(skin="#25262E", belly="#2E2F38", pat="#FFD60F", smile="#9A7A66", limb="#25262E",
                 toe="#FFD60F", sac="#33343E", **COMMON),
    "female": dict(skin="#F28A1E", belly="#F9B04A", pat="#1F1A1A", smile="#7A3A12", limb="#C9D83C",
                   toe="#EEDDB4", sac="#F9B04A", **COMMON),
    "froglet": dict(skin="#F28A1E", belly="#F9B04A", pat="#1F1A1A", smile="#7A3A12",
                    limb="#FFC826", toe="#FFD84A", sac="#F9B04A", **COMMON),
}
REGIONS = {"body": (0, 0, 192, 96), "eye": (192, 0, 64, 64), "limb": (0, 96, 192, 64)}

JOINTS = ["root", "hips", "spine", "head", "throat", "arm_l", "arm_r", "leg_upper_l",
          "leg_lower_l", "leg_upper_r", "leg_lower_r"]

# ---- frog.py geometry (metres at 0.18 m length), scaled by K in configure()
_BODY0 = [(-0.100, 0.100, 0.028, 0.024, 0.054), (-0.088, 0.101, 0.040, 0.036, 0.070),
          (-0.066, 0.099, 0.052, 0.044, 0.080), (-0.038, 0.092, 0.058, 0.054, 0.080),
          (-0.008, 0.080, 0.062, 0.062, 0.076), (0.022, 0.066, 0.058, 0.058, 0.070),
          (0.046, 0.054, 0.046, 0.044, 0.056), (0.062, 0.048, 0.026, 0.026, 0.034)]
_P0 = {
    "hips": (0, 0.040, 0.055), "spine": (0, -0.005, 0.080), "head": (0, -0.040, 0.095),
    "head_end": (0, -0.108, 0.100), "throat0": (0, -0.018, 0.052), "throat1": (0, -0.060, 0.086),
    "arm0": (0.052, -0.040, 0.066), "arm1": (0.062, -0.058, 0.006),
    "leg_hip": (0.050, 0.040, 0.040), "knee": (0.094, -0.010, 0.034),
    "ankle": (0.078, 0.058, 0.012), "toe": (0.096, -0.022, 0.003),
}
G = {}      # configured geometry (see configure)
RIG = None


def pitch_pt(v, k):
    """Nose-up posture: rotate a body-side point about the rump pivot."""
    t = math.radians(PITCH)
    yp, zp = 0.07 * k, 0.07 * k
    dy, dz = v[1] - yp, v[2] - zp
    return V(v[0], yp + dy * math.cos(t) + dz * math.sin(t), zp - dy * math.sin(t) + dz * math.cos(t))


class Rig(rb.Skeleton):
    def __init__(self, P):
        m = rb.mirror
        j = [("root", None, (0, 0, 0), (0, 0, 0.05)),
             ("hips", "root", P["hips"], P["spine"]),
             ("spine", "hips", P["spine"], P["head"]),
             ("head", "spine", P["head"], P["head_end"]),
             ("throat", "head", P["throat0"], P["throat1"])]
        for s, sg in SIDES:
            j.append((f"arm_{s}", "spine", m(P["arm0"], sg), m(P["arm1"], sg)))
        for s, sg in SIDES:
            j += [(f"leg_upper_{s}", "hips", m(P["leg_hip"], sg), m(P["knee"], sg)),
                  (f"leg_lower_{s}", f"leg_upper_{s}", m(P["knee"], sg), m(P["ankle"], sg))]
        assert [n for n, *_ in j] == JOINTS
        super().__init__(j)


def configure(name):
    global RIG
    cfg = VARIANTS[name]
    k = BASE_K * cfg["scale"]
    G.clear()
    G.update(cfg, name=name, K=k, colors=PALETTES[name])
    hs = cfg["head"]
    body = []
    for i, (y, z, ru, rd, rl) in enumerate(_BODY0):
        f = hs if i <= 3 else 1.0            # bigger head (rings 0..3) on the froglet
        c = pitch_pt((0, y * k, z * k), k)
        body.append((c.y, c.z, ru * k * f, rd * k * f, rl * k * f))
    G["BODY"] = body
    G["NOSE"] = pitch_pt((0, -0.110 * k, 0.099 * k), k)
    G["RUMP"] = pitch_pt((0, 0.070 * k, 0.046 * k), k)
    ec = pitch_pt((0.050 * k * (hs ** 0.5), -0.056 * k, 0.122 * k), k)
    G["EYE_C"] = ec
    G["EYE_R"] = 0.034 * k * cfg["eye"]
    G["SAC_C"] = pitch_pt((0, -0.058 * k, 0.086 * k), k)
    G["SAC_R"] = (0.036 * k, 0.031 * k, 0.046 * k)
    P = {}
    for key, v in _P0.items():
        p = V(*v) * k
        if key in ("spine", "head", "head_end", "throat0", "throat1", "arm0"):
            p = pitch_pt(p, k)
        P[key] = p
    # arms hang down from the chest (the hands stay in front of the body)
    P["arm1"] = V(P["arm0"].x + 0.012 * k, P["arm0"].y - 0.018 * k, 0.006 * k)
    G["P"] = P
    RIG = Rig(P)
    G["REST_ANKLE"] = {s: V(P["ankle"].x * sg, P["ankle"].y, P["ankle"].z) for s, sg in SIDES}
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
    belly = Vv < 0.30 - 0.05 * NSS(0.6, 1.0, U) + 0.03 * np.sin(U * 25)
    put(c, belly & (U < 0.46), "belly")
    if n == "male":
        # bumblebee: yellow band over the brow, two bands across the back, a hip band;
        # bands slant backwards towards the belly like the concept
        sl = 0.05 * (1 - Vv)
        put(c, (U > 0.12 + sl * 0.5) & (U < 0.23 + sl * 0.5) & (Vv > 0.50), "pat")
        put(c, (U > 0.37 + sl) & (U < 0.49 + sl) & (Vv > 0.28), "pat")
        put(c, (U > 0.62 + sl * 0.6) & (U < 0.75 + sl * 0.6) & (Vv > 0.22), "pat")
        c[belly & (U > 0.60)] = C("belly")
    else:
        if n == "female":
            put(c, np.hypot((U - 0.175) * 2.3, Vv - 0.76) < 0.19, "pat")        # eye patch
            put(c, (U > 0.31) & (U < 0.40 + 0.06 * (1 - Vv)) & (Vv > 0.40), "pat")  # neck band
            spots = ((0.50, 0.84, 0.10), (0.60, 0.58, 0.085), (0.70, 0.88, 0.09), (0.80, 0.66, 0.07),
                     (0.46, 0.50, 0.07))
        else:
            put(c, np.hypot((U - 0.17) * 2.3, Vv - 0.78) < 0.14, "pat")
            spots = ((0.44, 0.86, 0.09), (0.60, 0.70, 0.09), (0.76, 0.84, 0.07), (0.70, 0.50, 0.05))
        for u0, v0, r in spots:
            put(c, np.hypot((U - u0) * 1.7, Vv - v0) < r, "pat")
    # wide gentle smile: a line along the side of the head, corners curling up at the back
    mv = 0.47 + 0.05 * NSS(0.16, 0.30, U) - 0.03 * (1 - NSS(0.0, 0.08, U))
    put(c, (np.abs(Vv - mv) < 0.028) & (U > 0.0) & (U < 0.30), "smile" if G["name"] == "male" else "ink")
    # nostrils
    put(c, (np.abs(U - 0.06) < 0.012) & (np.abs(Vv - 0.90) < 0.025), "ink")
    return c


def paint_limb(U, Vv):
    """U along the limb piece (root -> tip), V ring height."""
    n = G["name"]
    c = C("limb") * np.ones(U.shape + (1,), np.float32)
    if n == "male":
        put(c, (U > 0.35) & (U < 0.62), "pat")
    elif n == "female":
        for u0, v0, r in ((0.2, 0.75, 0.09), (0.25, 0.25, 0.08), (0.45, 0.55, 0.10), (0.62, 0.85, 0.08),
                          (0.68, 0.30, 0.09), (0.85, 0.6, 0.07), (0.1, 0.45, 0.06), (0.82, 0.12, 0.06)):
            put(c, np.hypot((U - u0) * 0.9, (Vv - v0) * 0.5) < r * 0.8, "pat")
    return c


# --------------------------------------------------------------------------- mesh

def w_body(co):
    k = G["K"]
    return rb.chain(co.y, [(-0.050 * k, "head"), (-0.022 * k, "spine"), (0.012 * k, "spine"),
                           (0.040 * k, "hips")])


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


def build(mb, at):
    k, P = G["K"], G["P"]
    skin, sac, toe = at.cell_uv("skin"), at.cell_uv("sac"), at.cell_uv("toe")
    n = 12
    pts = [V(0, y, z) for y, z, *_ in G["BODY"]]
    radii = [(ru, rd, rl) for *_, ru, rd, rl in G["BODY"]]
    cum = [0.0]
    for a, b in zip(pts, pts[1:]):
        cum.append(cum[-1] + (b - a).length)
    Us = [0.0] + [c / cum[-1] for c in cum] + [1.0]

    def uv(i, kk, co):
        return at.map_uv("body", 0.02 + 0.96 * Us[i + 1], rb.ring_h(kk, n))

    br.path_loft(mb, pts, radii, uv, w_body, n, pole_start=G["NOSE"], pole_end=G["RUMP"])

    # eyes: whole balls on top of the head, painted eye facing forward-out
    glow = []
    for s, sg in SIDES:
        c = V(G["EYE_C"].x * sg, G["EYE_C"].y, G["EYE_C"].z)
        f = V(0.55 * sg, -1.0, 0.22).normalized()
        back = lambda ri, cen, c=c, f=f: skin if (cen - c).normalized().dot(f) < 0.15 else None  # noqa: E731
        R = G["EYE_R"]
        _, faces = br.ellipsoid(mb, c, R, R, R * 0.97, 9, 5, eye_uv(at, c, f), rb.rigid("head"),
                                axis=f, face_uv=back)
        glow += [fc for fc in faces if (fc.calc_center_median() - c).normalized().dot(f) > 0.55]
        if G["name"] == "female":
            # eyelashes: three little curled strokes on the outer upper rim of the eye
            for a in (62, 40, 20):
                th = math.radians(a)
                base = c + V(sg * math.cos(th), 0.15, math.sin(th)).normalized() * (R * 0.98)
                tip = base + V(sg * math.cos(th) * 0.7, 0.05, math.sin(th) * 0.9).normalized() * (R * 0.55)
                br.path_loft(mb, [base, base.lerp(tip, 0.6) + V(0, -R * 0.08, 0), tip],
                             [(R * 0.075,) * 3, (R * 0.06,) * 3, (R * 0.03,) * 3],
                             lambda i, kk, co: at.cell_uv("ink"), rb.rigid("head"), 4,
                             pole_start=True, pole_end=True)
    mb.mark_glow(glow)

    # vocal sac (rest: tucked inside the chin), rigid on the throat bone
    sr = G["SAC_R"]
    br.ellipsoid(mb, G["SAC_C"], sr[0], sr[1], sr[2], 10, 5, lambda i, kk, co: sac, rb.rigid("throat"))

    def limb_uv(count, bone):
        def f(i, kk, co):
            return at.map_uv("limb", i / max(count - 1, 1), rb.ring_h(kk, 6))
        return f

    def toe_tips(centre, bone, sg, spread=0.020, fwd=-0.012):
        """Three round toe pads fanned out in front of the hand / foot."""
        for dx in (-1, 0, 1):
            tc = centre + V(dx * spread * k * 0.0 + dx * spread * k, fwd * k - 0.006 * k * (1 - abs(dx)), 0)
            br.ellipsoid(mb, tc, 0.0095 * k, 0.0085 * k, 0.0105 * k, 6, 3,
                         lambda i, kk, co: toe, rb.rigid(bone), axis=Y)

    # front legs: tube + flat hand
    for s, sg in SIDES:
        a0 = V(P["arm0"].x * sg, P["arm0"].y, P["arm0"].z)
        a1 = V(P["arm1"].x * sg, P["arm1"].y, P["arm1"].z)
        mid = a0.lerp(a1, 0.5)
        br.path_loft(mb, [a0, mid, a1],
                     [(0.024 * k,) * 3, (0.019 * k,) * 3, (0.015 * k,) * 3],
                     limb_uv(3, f"arm_{s}"), rb.rigid(f"arm_{s}"), 6, lat=Y)
        hand = [a1 + V(0.002 * sg * k, -0.026 * k, 0.001 * k), a1 + V(0.0, -0.008 * k, 0.0),
                a1 + V(0, 0.008 * k, 0.002 * k)]
        br.path_loft(mb, hand, [(0.006 * k, 0.005 * k, 0.014 * k), (0.008 * k, 0.006 * k, 0.020 * k),
                                (0.007 * k, 0.006 * k, 0.014 * k)],
                     lambda i, kk, co: toe, rb.rigid(f"arm_{s}"), 6, lat=X, pole_start=True, pole_end=True)

    # hind legs: fat thigh (hip -> knee), shin (knee -> ankle), webbed foot (ankle -> toes)
    for s, sg in SIDES:
        m = lambda p: V(p.x * sg, p.y, p.z)  # noqa: E731
        hip, knee, ank, tp = m(P["leg_hip"]), m(P["knee"]), m(P["ankle"]), m(P["toe"])
        br.path_loft(mb, [hip, hip.lerp(knee, 0.5), knee],
                     [(0.034 * k, 0.032 * k, 0.032 * k), (0.034 * k, 0.032 * k, 0.032 * k),
                      (0.021 * k, 0.019 * k, 0.021 * k)],
                     limb_uv(3, ""), rb.rigid(f"leg_upper_{s}"), 8, lat=Z, pole_start=True, pole_end=True)
        br.path_loft(mb, [knee, knee.lerp(ank, 0.5), ank],
                     [(0.018 * k,) * 3, (0.016 * k,) * 3, (0.013 * k,) * 3],
                     limb_uv(3, ""), rb.rigid(f"leg_lower_{s}"), 6, lat=Z, pole_start=True, pole_end=True)
        d = tp - ank
        fpts = [ank + d * t for t in (1.0, 0.72, 0.35, 0.05)]
        fr = [(0.005 * k, 0.004 * k, 0.020 * k), (0.006 * k, 0.004 * k, 0.025 * k),
              (0.006 * k, 0.005 * k, 0.015 * k), (0.007 * k, 0.006 * k, 0.010 * k)]
        br.path_loft(mb, fpts, fr, lambda i, kk, co: toe, rb.rigid(f"leg_lower_{s}"), 6, lat=X,
                     pole_start=True, pole_end=True)

    if G["name"] == "froglet":
        # flat tadpole-tail stump behind the rump (rigid on hips)
        r0 = G["RUMP"]
        tail = [V(0, r0.y - 0.01 * k, r0.z), V(0, r0.y + 0.028 * k, r0.z - 0.004 * k),
                V(0, r0.y + 0.060 * k, r0.z - 0.010 * k)]
        br.path_loft(mb, tail, [(0.016 * k, 0.012 * k, 0.040 * k), (0.014 * k, 0.010 * k, 0.052 * k),
                                (0.008 * k, 0.006 * k, 0.030 * k)],
                     lambda i, kk, co: skin, rb.rigid("hips"), 6, lat=X, pole_end=True)


# --------------------------------------------------------------------------- clips

def pose():
    return rb.Pose(RIG)


def legs_ik(p, targets, w=1.0):
    k = G["K"]
    for s, sg in SIDES:
        if w <= 0:
            continue
        hip = p.head(f"leg_upper_{s}")
        pole = hip + V(0.26 * sg, -0.13, 0.04) * G['scale']
        p.limb_ik(f"leg_upper_{s}", f"leg_lower_{s}", targets[s], pole)
        if w < 1:
            for b in (f"leg_upper_{s}", f"leg_lower_{s}"):
                p.rel[b] = Quaternion().slerp(p.rel[b], w)


def clip_idle(f, n=90):
    t = TAU * f / n
    p = pose()
    p.rel["spine"] = rx(1.5 * math.sin(3 * t))
    p.rel["head"] = rz(6 * math.sin(t)) @ rx(-2 * math.sin(2 * t + 0.4))
    p.rel["throat"] = rx(10 * max(0.0, math.sin(6 * t)) ** 2)     # breathing throat pulse
    p.hips_offset = V(0, 0, 0.0035 * G['scale'] * math.sin(3 * t))
    return p


def hop_hips_z(f, rise, crouch):
    """Hips height over a hop: crouch -> push -> arc (TAKEOFF -> LAND) -> squash."""
    k = G["K"]
    tk, ld = TAKEOFF, LAND
    n = HOP_FRAMES
    ff = f % n
    if tk <= ff <= ld:
        u = (ff - tk) / (ld - tk)
        return rise * 4 * u * (1 - u) + 0.0
    t = (ff - ld) % n                       # frames since landing
    stance = n - (ld - tk)
    u = t / stance
    # squash on landing, then push (rise slightly before takeoff)
    return -crouch * math.sin(math.pi * min(u * 1.0, 1.0)) ** 1 * (1 - 0.0 * u)


def hop_pose(f, v, rise, crouch, n, takeoff, land):
    """One hop cycle: feet planted (sliding back at speed v) from `land` to `takeoff` of the
    next cycle, trailing in the air, swinging forward before the landing."""
    k = G["K"]
    p = pose()
    ff = f % n
    air = ff > takeoff and ff < land
    stance = n - (land - takeoff)
    t = (ff - land) % n if not air else 0
    u_air = (ff - takeoff) / (land - takeoff)
    fl = rb.smoothstep(takeoff, takeoff + 3, ff) * (1 - rb.smoothstep(land - 4, land, ff))   # 0 planted .. 1 airborne
    # hips arc: parabola over the air phase, small squash around landing / push
    hz = 0.0
    if takeoff <= ff <= land:
        hz = rise * 4 * u_air * (1 - u_air)
    else:
        tt = (ff - land) % n
        hz = -crouch * math.sin(math.pi * tt / stance)
    # extra crouch just before the takeoff (wind-up)
    hz -= crouch * 0.5 * env(ff, takeoff - 3, takeoff - 1, takeoff, takeoff + 1)
    p.hips_offset = V(0, 0, hz)
    pitch = -16 * env(ff, takeoff - 2, takeoff + 2, land - 8, land - 3) + 12 * env(ff, land - 3, land, land + 2, land + 6)
    p.rel["hips"] = rx(pitch)
    p.rel["head"] = rx(-6 * fl + 6 * env(ff, land, land + 1, land + 2, land + 6))
    p.rel["throat"] = rx(5 * fl)
    for s, sg in SIDES:
        p.rel[f"arm_{s}"] = rx(-50 * fl + 10 * env(ff, land, land + 2, land + 3, land + 7)) @ ry(-10 * sg * fl)
        tg = {}
    for s, sg in SIDES:
        hip = p.head(f"leg_upper_{s}")
        rest = G["REST_ANKLE"][s]
        trail = hip + V(0.044 * sg, 0.253, -0.099) * G['scale']
        # planted ankle: starts forward of rest at landing, slides back with the ground at v
        tt = (ff - land) % n
        planted_y = rest.y - 0.5 * v * stance / 30.0 + v * tt / 30.0
        planted = V(rest.x, planted_y, rest.z)
        if air:
            # push-off end position -> trailing -> forward again for the landing
            takeoff_pos = V(rest.x, rest.y + 0.5 * v * stance / 30.0, rest.z)
            land_pos = V(rest.x, rest.y - 0.5 * v * stance / 30.0, rest.z)
            a = rb.smoothstep(0, 0.30, u_air)
            b = rb.smoothstep(0.65, 1.0, u_air)
            pos = takeoff_pos.lerp(trail, a).lerp(land_pos, b)
        else:
            pos = planted
        tg[s] = pos
    legs_ik(p, tg)
    return p


def clip_walk(f, n=HOP_FRAMES):
    k = G["K"]
    return hop_pose(f, WALK_SPEED, rise=0.16 * G['scale'], crouch=0.030 * G['scale'], n=n,
                    takeoff=TAKEOFF, land=LAND)


def clip_eat(f, n=45):
    """Lean back, lunge at the food (bite at frame 20), two gulps of the throat."""
    k = G["K"]
    wind = env(f, 0, 10, 14, 17)
    lunge = env(f, 14, 20, 22, 28)
    gulp = env(f, 24, 27, 28, 31) + env(f, 33, 36, 37, 40)
    p = pose()
    p.rel["spine"] = rx(-5 * wind + 7 * lunge)
    p.rel["head"] = rx(-12 * wind + 14 * lunge - 6 * gulp)
    p.rel["throat"] = rx(40 * gulp)
    p.hips_offset = V(0, 0, (-0.004 * wind - 0.006 * lunge) * G['scale'])
    for s, sg in SIDES:
        p.rel[f"arm_{s}"] = rx(-8 * lunge)
    return p


def clip_happy(f, n=45):
    """A small hop on the spot with a puffed throat (happy_peak at the top)."""
    k = G["K"]
    p = hop_pose(f, 0.0, rise=0.13 * G['scale'], crouch=0.026 * G['scale'], n=n, takeoff=8, land=22)
    puff = env(f, 6, 12, 28, 38)
    p.rel["throat"] = rx(30 * puff) @ p.rel["throat"]
    p.rel["head"] = rx(-8 * puff) @ p.rel["head"]
    return p


def clip_refuse(f, n=36):
    """Turn the head away and shake it (no, thank you), with a small body twist."""
    t = TAU * f / n
    e = env(f, 0, 5, 30, 36)
    p = pose()
    p.rel["head"] = rz(26 * e * math.sin(3 * t))
    p.rel["spine"] = rz(-8 * e * math.sin(3 * t + 0.6))
    p.rel["hips"] = rz(6 * e * math.sin(3 * t + 1.2))
    p.rel["throat"] = rx(7 * e)
    return p


def clip_sleep(f, n=90):
    """Crouched low, head bowed, slow breathing."""
    k = G["K"]
    t = TAU * f / n
    p = pose()
    p.hips_offset = V(0, 0, (-0.040 + 0.004 * math.sin(t)) * G['scale'])
    p.rel["hips"] = rx(5)
    p.rel["spine"] = rx(6)
    p.rel["head"] = rx(16 + 1.5 * math.sin(t))
    p.rel["throat"] = rx(5 + 5 * math.sin(t))
    for s, sg in SIDES:
        p.rel[f"arm_{s}"] = rx(8)
    tg = {s: G["REST_ANKLE"][s] for s, _ in SIDES}
    legs_ik(p, tg)
    return p


def clips():
    return [rb.Clip("idle", 90, True, clip_idle),
            rb.Clip("walk", HOP_FRAMES, True, clip_walk),
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
    arm = RIG.build_armature("frog_rig")
    colors = dict(G["colors"])
    atlas = rb.Atlas(256, colors, REGIONS)
    body_png = zb.repo_path("assets", "textures", "animals", f"{asset}_body.png")
    img = atlas.write(body_png, {
        "body": paint_body, "limb": paint_limb,
        "eye": br.paint_eye(colors, iris="iris", pupil_r=0.62, pupil_at=(0.0, 0.0), outer="skin",
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
    k = G["K"]
    if "--no-preview" not in args:
        ls = rb.setup_preview(mesh, img, ground_z=0.0)
        out = zb.repo_path("art", "animals", asset, "model_preview.png")
        os.makedirs(os.path.dirname(out), exist_ok=True)
        rb.render_preview(arm, out, ls, "walk", 14, mx.z * 0.5, height_m=mx.z,
                          big_scale=1.0 * G['scale'], close_scale=0.75 * G['scale'], small_px=64,
                          ground=rb.GRASS)
    if "--debug" in args:
        ls = rb.setup_preview(mesh, img, ground_z=0.0)
        rb.render_debug(arm, args[args.index("--debug") + 1], ls, cl, mx.z * 0.5,
                        G["P"]["head"] + V(0, 0, 0.02), head_scale=0.45 * G['scale'],
                        strip_scale=0.8 * G['scale'], ground=rb.GRASS)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)


main()
