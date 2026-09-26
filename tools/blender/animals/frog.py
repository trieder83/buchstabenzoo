"""frog — ambient comic frog at the pond (GAME-AMBIENT; own tiny `frog` rig, 11 joints).

Run:  blender -b --factory-startup --python tools/blender/animals/frog.py
      [-- --debug <dir>]   extra review renders (not committed)

Writes
  assets/blender/animals/frog.blend
  assets/models/animals/frog.glb             1 skin (11 joints), 1 mesh, clips idle croak hop swim
  assets/textures/animals/frog_body.png

Skeleton:
    root                          ground under the frog (feet), never animated
    └─ hips                       rump; the only joint with translation (hop arc, swim float)
       ├─ spine ─ head ─ throat   throat: vocal sac tucked under the chin; pivot behind and
       │                          below it, so rx+ swings the sac forward-down (croak puff)
       │    ├─ arm_l, arm_r       front legs (one joint each)
       ├─ leg_upper_l ─ leg_lower_l   thigh; shin + webbed foot
       └─ leg_upper_r ─ leg_lower_r

Look: kit 5 frog (art/props/kit_water/sheet_v1.jpg): chunky green frog sitting upright-ish,
light belly and chin, dark green spots, very big bulging eyes on top of the head (pale
yellow, big pupil), wide smile. ~0.2 m long, ~0.17 m high to the eye tops.

Origin: on the ground at the feet (min z = 0) — the game puts it on a lily pad or the
bank. `hop` has NO forward root motion: hips rise and fall on a vertical arc only (takeoff
frame 8, landing frame 22 in animal_anims.toml); the game carries the frog forward
hop_distance = 0.6 m between those frames. `swim` lowers the hips so that the body floats
with the eyes out of the water: for `swim` the origin is the WATER SURFACE (the game puts
the origin at water height while the frog swims).
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
ASSET = "frog"
TRI_BUDGET = 1200

COLORS = {
    "green": "#6CB84A",
    "belly": "#CFE68E",
    "spot": "#3F8C35",
    "eye_white": "#FFF3B0",
    "iris": "#F2D35C",
    "pupil": "#1E1E1E",
    "ink": "#2B1B12",
    "toe": "#8CCB5C",
    "sac": "#F6F0B8",
}
REGIONS = {"body": (0, 0, 192, 96), "eye": (192, 0, 64, 64)}

JOINTS = [("root", None), ("hips", "root"), ("spine", "hips"), ("head", "spine"),
          ("throat", "head"), ("arm_l", "spine"), ("arm_r", "spine"),
          ("leg_upper_l", "hips"), ("leg_lower_l", "leg_upper_l"),
          ("leg_upper_r", "hips"), ("leg_lower_r", "leg_upper_r")]
JOINT_NAMES = [n for n, _ in JOINTS]

# ---- body loft, nose (-Y) -> rump: (y, z, r_up, r_down, r_lat)
BODY = [(-0.100, 0.100, 0.028, 0.024, 0.054), (-0.088, 0.101, 0.038, 0.034, 0.068),
        (-0.066, 0.099, 0.046, 0.040, 0.076), (-0.038, 0.092, 0.052, 0.050, 0.074),
        (-0.008, 0.080, 0.056, 0.058, 0.070), (0.022, 0.066, 0.054, 0.054, 0.066),
        (0.046, 0.054, 0.044, 0.042, 0.054), (0.062, 0.048, 0.026, 0.026, 0.034)]
NOSE = V(0, -0.110, 0.099)
RUMP = V(0, 0.070, 0.046)
EYE_C = V(0.040, -0.052, 0.140)      # left eye ball centre
EYE_R = 0.028
SAC_C = V(0, -0.058, 0.086)          # vocal sac (rest: tucked inside the chin)
SAC_R = (0.036, 0.031, 0.046)         # forward, up, lateral

P = {
    "hips": V(0, 0.040, 0.055), "spine": V(0, -0.005, 0.080), "head": V(0, -0.040, 0.095),
    "head_end": V(0, -0.108, 0.100), "throat": (V(0, -0.018, 0.052), V(0, -0.060, 0.086)),
    "arm": (V(0.052, -0.045, 0.066), V(0.062, -0.080, 0.005)),
    "leg_hip": V(0.048, 0.040, 0.040), "knee": V(0.092, -0.010, 0.034),
    "ankle": V(0.074, 0.058, 0.012), "toe": V(0.092, -0.022, 0.003),
}


class Rig(rb.Skeleton):
    def __init__(self):
        m = rb.mirror
        j = [("root", None, (0, 0, 0), (0, 0, 0.05)),
             ("hips", "root", P["hips"], P["spine"]),
             ("spine", "hips", P["spine"], P["head"]),
             ("head", "spine", P["head"], P["head_end"]),
             ("throat", "head", *P["throat"])]
        for s, sg in SIDES:
            j.append((f"arm_{s}", "spine", m(P["arm"][0], sg), m(P["arm"][1], sg)))
        for s, sg in SIDES:
            j += [(f"leg_upper_{s}", "hips", m(P["leg_hip"], sg), m(P["knee"], sg)),
                  (f"leg_lower_{s}", f"leg_upper_{s}", m(P["knee"], sg), m(P["ankle"], sg))]
        assert [n for n, *_ in j] == JOINT_NAMES
        super().__init__(j)


RIG = Rig()


# --------------------------------------------------------------------------- painters

def NSS(e0, e1, x):
    t = np.clip((x - e0) / (e1 - e0), 0.0, 1.0)
    return t * t * (3 - 2 * t)


def paint_body(U, Vv):
    """U = nose (0) -> rump (1), V = ring height (1 top, 0 belly), symmetric left/right."""
    c = br.rgb(COLORS, "green") * np.ones(U.shape + (1,), np.float32)
    belly = Vv < 0.36 - 0.06 * NSS(0.6, 1.0, U) + 0.04 * np.sin(U * 25)
    c[belly] = br.rgb(COLORS, "belly")
    # dark spots on the back
    for u0, v0, r in ((0.45, 0.86, 0.05), (0.62, 0.72, 0.045), (0.72, 0.92, 0.04),
                      (0.35, 0.66, 0.035), (0.85, 0.70, 0.035), (0.55, 0.98, 0.04)):
        c[np.hypot((U - u0) * 1.6, Vv - v0) < r] = br.rgb(COLORS, "spot")
    # wide smile: a line along the side of the head, corners curling up at the back
    mv = 0.47 + 0.05 * NSS(0.16, 0.30, U) - 0.03 * (1 - NSS(0.0, 0.08, U))
    smile = (np.abs(Vv - mv) < 0.022) & (U > 0.0) & (U < 0.30)
    c[smile] = br.rgb(COLORS, "ink")
    # nostrils
    c[(np.abs(U - 0.06) < 0.012) & (np.abs(Vv - 0.90) < 0.025)] = br.rgb(COLORS, "ink")
    return c


# --------------------------------------------------------------------------- mesh

def w_body(co):
    return rb.chain(co.y, [(-0.050, "head"), (-0.022, "spine"), (0.012, "spine"),
                           (0.040, "hips")])


def build(mb, at):
    green = at.cell_uv("green")
    sac = at.cell_uv("sac")
    toe = at.cell_uv("toe")
    n = 14
    pts = [V(0, y, z) for y, z, *_ in BODY]
    radii = [(ru, rd, rl) for *_, ru, rd, rl in BODY]
    cum = [0.0]
    for a, b in zip(pts, pts[1:]):
        cum.append(cum[-1] + (b - a).length)
    Us = [0.0] + [c / cum[-1] for c in cum] + [1.0]

    def uv(i, k, co):
        return at.map_uv("body", 0.02 + 0.96 * Us[i + 1], rb.ring_h(k, n))

    br.path_loft(mb, pts, radii, uv, w_body, n, pole_start=NOSE, pole_end=RUMP)

    # eyes: whole balls on top of the head, painted eye facing forward-out
    for s, sg in SIDES:
        c = V(EYE_C.x * sg, EYE_C.y, EYE_C.z)
        f = V(0.55 * sg, -1.0, 0.25).normalized()
        # faces behind the painted cap: flat green (lid), no UV smearing across the eye
        back = lambda ri, cen, c=c, f=f: green if (cen - c).normalized().dot(f) < 0.15 else None  # noqa: E731
        br.ellipsoid(mb, c, EYE_R, EYE_R, EYE_R * 0.95, 10, 6,
                     _eye_uv(at, c, f), rb.rigid("head"), axis=f, face_uv=back)

    # vocal sac (pale), rigid on the throat bone
    br.ellipsoid(mb, SAC_C, SAC_R[0], SAC_R[1], SAC_R[2], 10, 5,
                 lambda i, k, co: sac, rb.rigid("throat"))

    # front legs: tube + flat hand
    for s, sg in SIDES:
        a0, a1 = (V(p.x * sg, p.y, p.z) for p in P["arm"])
        br.path_loft(mb, [a0, a0.lerp(a1, 0.55), a1],
                     [(0.019,) * 3, (0.015,) * 3, (0.012,) * 3],
                     lambda i, k, co: green, rb.rigid(f"arm_{s}"), 6, lat=Y)
        hand = [a1 + V(0.004 * sg, -0.022, -0.001), a1 + V(0.002 * sg, -0.008, 0.0),
                a1 + V(0, 0.006, 0.002)]
        br.path_loft(mb, hand, [(0.005, 0.004, 0.012), (0.007, 0.005, 0.019), (0.006, 0.005, 0.012)],
                     lambda i, k, co: toe, rb.rigid(f"arm_{s}"), 6, lat=X,
                     pole_start=True, pole_end=True)

    # hind legs: fat thigh (hip -> knee), shin (knee -> ankle), webbed foot (ankle -> toes)
    for s, sg in SIDES:
        m = lambda p: V(p.x * sg, p.y, p.z)  # noqa: E731
        hip, knee, ank, tp = m(P["leg_hip"]), m(P["knee"]), m(P["ankle"]), m(P["toe"])
        br.path_loft(mb, [hip, hip.lerp(knee, 0.5), knee],
                     [(0.028, 0.026, 0.026), (0.028, 0.026, 0.026), (0.018, 0.016, 0.018)],
                     lambda i, k, co: green, rb.rigid(f"leg_upper_{s}"), 8, lat=Z,
                     pole_start=True, pole_end=True)
        br.path_loft(mb, [knee, knee.lerp(ank, 0.5), ank],
                     [(0.015,) * 3, (0.014,) * 3, (0.011,) * 3],
                     lambda i, k, co: green, rb.rigid(f"leg_lower_{s}"), 6, lat=Z,
                     pole_start=True, pole_end=True)
        d = tp - ank
        fpts = [ank + d * t for t in (1.0, 0.72, 0.35, 0.05)]
        fr = [(0.004, 0.003, 0.018), (0.005, 0.003, 0.022), (0.005, 0.004, 0.013),
              (0.006, 0.005, 0.009)]
        br.path_loft(mb, fpts, fr, lambda i, k, co: toe, rb.rigid(f"leg_lower_{s}"), 6,
                     lat=X, pole_start=True, pole_end=True)


def _eye_uv(at, c, f):
    """UV of an eye ball: angle from the look direction f -> radius in the eye region."""
    f = f.normalized()
    ex = (V(0, -1, 0) - f * f.dot(V(0, -1, 0)))
    ex = ex.normalized() if ex.length > 1e-4 else X
    ey = f.cross(ex).normalized()
    if ey.dot(Z) < 0:
        ey = -ey
    cap = math.radians(82)

    def uv(i, k, co):
        d = (co - c).normalized()
        ang = math.acos(max(-1.0, min(1.0, d.dot(f))))
        r = min(ang / cap, 1.0)
        a = math.atan2(d.dot(ey), d.dot(ex))
        return at.map_uv("eye", 0.5 + 0.5 * r * math.cos(a), 0.5 + 0.5 * r * math.sin(a))
    return uv


# --------------------------------------------------------------------------- clips

REST_ANKLE = {s: V(P["ankle"].x * sg, P["ankle"].y, P["ankle"].z) for s, sg in SIDES}


def legs_ik(p, targets, w=1.0):
    """IK the hind legs to ankle targets (world), blended with the rest pose by w."""
    for s, sg in SIDES:
        if w <= 0:
            continue
        hip = p.head(f"leg_upper_{s}")
        pole = hip + V(0.12 * sg, -0.06, 0.02)
        p.limb_ik(f"leg_upper_{s}", f"leg_lower_{s}", targets[s], pole)
        if w < 1:
            for b in (f"leg_upper_{s}", f"leg_lower_{s}"):
                p.rel[b] = Quaternion().slerp(p.rel[b], w)


def pose():
    return rb.Pose(RIG)


def clip_idle(f, n=90):
    t = TAU * f / n
    p = pose()
    p.rel["spine"] = rx(1.5 * math.sin(3 * t))
    p.rel["head"] = rz(7 * math.sin(t)) @ rx(-2 * math.sin(2 * t + 0.4))
    p.rel["throat"] = rx(9 * max(0.0, math.sin(6 * t)) ** 2)
    p.hips_offset = V(0, 0, 0.0015 * math.sin(3 * t))
    return p


def clip_croak(f, n=45):
    """Head up, the vocal sac puffs out twice (croak events at the peaks)."""
    e = env(f, 0, 6, 38, 45)
    puff = env(f, 5, 10, 13, 18) + env(f, 22, 27, 30, 36)
    p = pose()
    p.rel["spine"] = rx(-4 * e)
    p.rel["head"] = rx(-12 * e - 4 * puff)
    p.rel["throat"] = rx(42 * puff)
    p.hips_offset = V(0, 0, 0.004 * puff)
    for s, sg in SIDES:
        p.rel[f"arm_{s}"] = rx(3 * e)
    return p


HOP_PEAK = 0.17


def hop_z(f):
    """Hips height over the clip: crouch, push (feet planted), air arc (8 -> 22), squash."""
    if f <= 4:
        return -0.010 * rb.ease(f / 4)
    if f <= 8:
        return -0.010 + 0.060 * rb.ease((f - 4) / 4)
    if f <= 22:
        u = (f - 15) / 7.0
        return 0.050 + (HOP_PEAK - 0.050) * (1 - u * u) - 0.05 * max(0.0, (f - 15) / 7.0) ** 2
    if f <= 25:
        return -0.014 * rb.ease((f - 22) / 3)
    return -0.014 * (1 - rb.ease((f - 25) / 5))


def clip_hop(f, n=30):
    p = pose()
    air = env(f, 7, 10, 19, 23)
    crouch = env(f, 0, 4, 5, 8) + env(f, 21, 23, 25, 30)
    pitch = -14 * env(f, 4, 8, 12, 16) + 12 * env(f, 14, 18, 20, 24) + 6 * crouch
    p.rel["hips"] = rx(pitch)
    p.hips_offset = V(0, 0, hop_z(f))
    p.rel["head"] = rx(-6 * air)
    p.rel["throat"] = rx(4 * air)
    for s, sg in SIDES:
        p.rel[f"arm_{s}"] = rx(-55 * air) @ ry(-10 * sg * air)
    # hind legs: planted until full extension, trailing straight back in the air,
    # folded again for the landing
    tg = {}
    for s, sg in SIDES:
        hip = p.head(f"leg_upper_{s}")
        planted = REST_ANKLE[s]
        trail = hip + V(0.02 * sg, 0.115, -0.045)
        k = rb.smoothstep(7, 10, f) * (1 - rb.smoothstep(18, 23, f))
        tg[s] = planted.lerp(trail, k)
    legs_ik(p, tg, w=min(1.0, env(f, 1, 4, 25, 30) + 0.0))
    return p


SWIM_SINK = 0.085


def clip_swim(f, n=30):
    """Floating breaststroke: body flat, eyes out of the water, legs kick together."""
    t = TAU * f / n
    kick = 0.5 - 0.5 * math.cos(t)          # 0 folded .. 1 extended
    p = pose()
    p.rel["hips"] = rx(18 + 3 * math.sin(t))
    p.hips_offset = V(0, 0, -SWIM_SINK + 0.004 * math.sin(t + 1))
    p.rel["head"] = rx(-16)
    tg = {}
    for s, sg in SIDES:
        hip = p.head(f"leg_upper_{s}")
        folded = hip + V(0.050 * sg, -0.010, -0.020)
        ext = hip + V(0.035 * sg, 0.120, -0.010)
        tg[s] = folded.lerp(ext, kick)
        p.rel[f"arm_{s}"] = rx(60) @ ry(-12 * sg)
    legs_ik(p, tg)
    return p


def clips():
    return [rb.Clip("idle", 90, True, clip_idle),
            rb.Clip("croak", 45, False, clip_croak),
            rb.Clip("hop", 30, False, clip_hop),
            rb.Clip("swim", 30, True, clip_swim)]


# --------------------------------------------------------------------------- main

def main():
    import bpy
    args = zb.script_args()
    rb.check_gate("kit_water")  # concept approved as part of kit 5
    zb.clean_scene()
    bpy.context.scene.render.fps = rb.FPS
    arm = RIG.build_armature("frog_rig")
    atlas = rb.Atlas(256, COLORS, REGIONS)
    body_png = zb.repo_path("assets", "textures", "animals", f"{ASSET}_body.png")
    img = atlas.write(body_png, {"body": paint_body,
                                 "eye": br.paint_eye(COLORS, iris="iris", pupil_r=0.40,
                                                     pupil_at=(0.0, 0.0), outer="green",
                                                     iris_r=0.62)})
    mat = rb.body_material(img)
    mb = rb.MeshBuilder(RIG)
    build(mb, atlas)
    mesh = mb.to_object(ASSET, mat, arm)
    cl = clips()
    for c in cl:
        rb.bake_clip(RIG, arm, c)
    rb.hr.reset_pose(arm)
    bpy.context.scene.frame_set(0)
    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    print(f"{ASSET}: {tris} tris, {len(JOINT_NAMES)} joints, bounds x {mn.x:.3f}..{mx.x:.3f}  "
          f"y {mn.y:.3f}..{mx.y:.3f}  z {mn.z:.3f}..{mx.z:.3f}; length {mx.y - mn.y:.3f} m")
    print("hop: peak hips rise", round(max(hop_z(f) for f in range(31)), 3))
    zb.save_blend(zb.repo_path("assets", "blender", "animals", f"{ASSET}.blend"))
    glb = zb.repo_path("assets", "models", "animals", f"{ASSET}.glb")
    rb.qr.export_animal([arm, mesh], glb)
    print(f"exported {glb} ({os.path.getsize(glb) / 1024:.1f} KB)")
    if "--debug" in args:
        ls = rb.setup_preview(mesh, img, ground_z=0.0)
        rb.render_debug(arm, args[args.index("--debug") + 1], ls, cl, 0.09,
                        P["head"] + V(0, 0, 0.02), head_scale=0.2, strip_scale=0.42,
                        ground=rb.GRASS)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)


main()
