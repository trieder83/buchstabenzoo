"""player_girl — playable girl (ART-CHARACTERS, rig/clips/export per ART-RIG).

Run:  blender -b --factory-startup --python tools/blender/characters/player_girl.py [-- --no-preview]

Writes
  assets/blender/characters/_rig_human.blend     template armature (ART-RIG §2.5)
  assets/blender/characters/player_girl.blend    source (never hand-edited)
  assets/models/characters/player_girl.glb       game model: 1 skin, 1 mesh (body + face)
  assets/textures/characters/player_girl_body.png  64 x 64 flat-colour body atlas
  assets/textures/characters/player_girl_face.png  256 x 128 face decal atlas (8 cells)
  art/characters/player_girl/model_preview.png   55 deg game view | front | back | game size

Look: art/characters/player_girl/sheet_v1.jpg (approved turnaround). 1.20 m to the top of
the skull, head-to-body ~1 : 3.5, comic chunky low poly, smooth normals, no outlines.
"""

import math
import os
import sys

import bpy
from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import human_rig as hr  # noqa: E402

zb = hr.zb
ASSET = "player_girl"
BLEND = zb.repo_path("assets", "blender", "characters", f"{ASSET}.blend")
RIG_BLEND = zb.repo_path("assets", "blender", "characters", "_rig_human.blend")
GLB = zb.repo_path("assets", "models", "characters", f"{ASSET}.glb")
BODY_PNG = zb.repo_path("assets", "textures", "characters", f"{ASSET}_body.png")
FACE_PNG = zb.repo_path("assets", "textures", "characters", f"{ASSET}_face.png")
PREVIEW = zb.repo_path("art", "characters", ASSET, "model_preview.png")
TRI_BUDGET = 3000

# ART-CHARACTERS colours (proposal hex values, checked against sheet_v1)
COLORS = {
    "skin": "#F2C29B",
    "hair": "#4A2A17",
    "shirt": "#F4F4F0",
    "stripe": "#2F5DA8",
    "belt": "#6B3A1E",
    "jeans": "#3559A0",
    "cuff": "#2A4780",
    "sock": "#FFFFFF",
    "shoe": "#3B2314",
    "strap": "#F4F4F0",
    "lips": "#C8645A",
    "hair_hi": "#6B3E22",
}
# face decal colours
INK = hr.hex_rgb("#2B1B12")      # eyes, lash line
WHITE = hr.hex_rgb("#FFFFFF")
IRIS = hr.hex_rgb("#5A3420")
BROW = hr.hex_rgb("#4A2A17")
MOUTH = hr.hex_rgb("#A2463C")
MOUTH_IN = hr.hex_rgb("#7A2A28")
TONGUE = hr.hex_rgb("#E27A72")
CHEEK = hr.hex_rgb("#F29C8C")

HEAD_C = Vector((0, 0, 1.03))
HEAD_R = (0.184, 0.168, 0.175)
FACE_PX_PER_M = 190.0
FACE_Z0 = 1.00  # height of the face cell centre


# --------------------------------------------------------------------------- face decal

def to_px(x, z):
    return 32 + x * FACE_PX_PER_M, 32 - (z - FACE_Z0) * FACE_PX_PER_M


def face_uv(co):
    col, row = to_px(co.x, co.z)
    col = min(max(col, 0.0), 64.0)
    row = min(max(row, 0.0), 64.0)
    return (col / 256.0, 1.0 - row / 128.0)


def draw_face(cv, expr):
    ex, ez = 0.066, 1.017
    for s in (-1, 1):
        cx, cy = to_px(s * ex, ez)
        # cheeks
        kx, ky = to_px(s * 0.108, 0.962)
        cv.ellipse(kx, ky, 4.2, 2.6, CHEEK)
        # eyes
        if expr in ("blink",):
            cv.arc(cx, cy + 1.0, 5.0, 2.6, 15, 165, 1.5, INK)
        elif expr == "laugh":
            cv.arc(cx, cy + 2.5, 5.0, 4.2, 200, 340, 1.6, INK)
        else:
            big = 1.15 if expr == "surprised" else 1.0
            rx, ry = 5.6 * big, 6.8 * big
            cv.ellipse(cx, cy, rx + 1.1, ry + 1.1, INK)
            cv.ellipse(cx, cy, rx, ry, WHITE)
            ix, iy = cx - s * 0.6, cy + 0.9
            ir = 4.7 if expr != "surprised" else 3.8
            if expr == "thinking":
                ix, iy = cx + 1.2, cy - 1.6
            if expr == "sad":
                iy = cy + 1.6
            m = cv.ellipse_mask(cx, cy, rx, ry)
            cv.fill(m & cv.ellipse_mask(ix, iy, ir, ir), IRIS)
            cv.fill(m & cv.ellipse_mask(ix, iy, ir * 0.55, ir * 0.55), INK)
            cv.ellipse(ix + 1.4, iy - 1.6, 1.4, 1.4, WHITE)
            if expr == "happy":  # lower lid smile
                cv.arc(cx, cy + ry + 1.0, rx + 1.0, 2.2, 200, 340, 1.4, INK)
        # brows
        bx, by = to_px(s * 0.070, 1.074)
        if expr == "surprised":
            by -= 3.0
        if expr == "sad":
            cv.stroke([(bx - s * 4.5, by - 1.0), (bx + s * 3.5, by + 1.5)], 1.7, BROW)  # inner end up
        elif expr == "thinking" and s == 1:
            cv.arc(bx, by + 0.5, 4.5, 2.4, 200, 340, 1.7, BROW)
        else:
            cv.arc(bx, by + 2.0, 4.5, 2.2, 205, 335, 1.7, BROW)
    mx, my = to_px(0.0, 0.948)
    if expr in ("neutral", "blink"):
        cv.arc(mx, my - 1.5, 4.2, 2.6, 25, 155, 1.4, MOUTH)
    elif expr == "happy":
        cv.arc(mx, my - 2.0, 5.5, 3.6, 15, 165, 1.6, MOUTH)
    elif expr == "laugh":
        m = cv.ellipse_mask(mx, my - 1.0, 6.0, 5.0) & (cv.y > my - 1.0)
        cv.fill(m, MOUTH_IN)
        cv.fill(m & cv.ellipse_mask(mx, my + 3.2, 3.6, 2.2), TONGUE)
        cv.stroke([(mx - 6.0, my - 1.0), (mx + 6.0, my - 1.0)], 1.2, MOUTH)
    elif expr == "talk":
        cv.ellipse(mx, my, 3.0, 2.6, MOUTH)
        cv.ellipse(mx, my + 0.3, 2.0, 1.6, MOUTH_IN)
    elif expr == "surprised":
        cv.ellipse(mx, my + 0.5, 2.8, 3.4, MOUTH)
        cv.ellipse(mx, my + 0.6, 1.8, 2.3, MOUTH_IN)
    elif expr == "thinking":
        cv.stroke([(mx - 1.0, my), (mx + 4.5, my - 1.2)], 1.4, MOUTH)
    elif expr == "sad":
        cv.arc(mx, my + 2.2, 4.0, 2.4, 205, 335, 1.4, MOUTH)


# --------------------------------------------------------------------------- body mesh

def build_mesh(mb):
    uv = mb.atlas_uv  # noqa: F841
    # ---- torso (jeans seat, belt, striped shirt): rings z, rx, ry(depth), cy
    torso = [
        (0.445, 0.085, 0.070, 0.0),
        (0.47, 0.126, 0.088, 0.0),
        (0.50, 0.138, 0.095, 0.0),
        (0.54, 0.140, 0.098, -0.002),
        (0.565, 0.142, 0.101, -0.004),
        (0.61, 0.145, 0.103, -0.006),
        (0.64, 0.146, 0.101, -0.006),
        (0.685, 0.148, 0.097, -0.004),
        (0.715, 0.150, 0.094, -0.002),
        (0.76, 0.153, 0.090, 0.0),
        (0.795, 0.148, 0.085, 0.0),
        (0.825, 0.112, 0.070, 0.0),
        (0.848, 0.058, 0.050, 0.0),
    ]
    rings = [hr.ellipse_ring((0, cy, z), rx, ry, 12) for z, rx, ry, cy in torso]

    def torso_color(i, c):
        z = c.z
        if z < 0.50:
            return "jeans"
        if z < 0.54:
            return "belt"
        for a, b in ((0.565, 0.61), (0.64, 0.685), (0.715, 0.76)):
            if a < z < b:
                return "stripe"
        return "shirt"

    mb.rings(rings, torso_color, hr.w_torso, pole_start=Vector((0, 0, 0.432)),
             pole_end=Vector((0, 0, 0.856)))

    # ---- neck
    mb.rings([hr.ellipse_ring((0, 0.005, 0.80), 0.046, 0.044, 8),
              hr.ellipse_ring((0, 0.005, 0.93), 0.043, 0.041, 8)],
             lambda i, c: "skin", hr.w_neck,
             pole_start=Vector((0, 0.005, 0.79)), pole_end=Vector((0, 0.005, 0.94)))

    # ---- legs: z, radius; cuffs are a stepped ring
    leg = [(0.50, 0.066), (0.44, 0.069), (0.36, 0.063), (0.27, 0.057), (0.19, 0.054),
           (0.155, 0.054), (0.155, 0.066), (0.10, 0.066), (0.10, 0.037), (0.075, 0.036)]
    leg_cols = ["jeans"] * 5 + ["cuff", "cuff", "cuff", "sock", "sock"]
    for side, s in (("l", 1), ("r", -1)):
        x = s * 0.075
        rings = [hr.ellipse_ring((x, 0.0, z), r, r * 0.96, 8) for z, r in leg]
        mb.rings(rings, lambda i, c: leg_cols[max(i, 0)] if i >= 0 else "jeans", hr.w_leg(side),
                 pole_start=Vector((x, 0, 0.515)), pole_end=Vector((x, 0, 0.062)))

    # ---- arms: t along the arm from the shoulder joint, radius
    arm = [(-0.05, 0.058), (0.0, 0.064), (0.04, 0.063), (0.085, 0.059), (0.085, 0.041),
           (0.17, 0.037), (0.30, 0.032), (0.335, 0.030)]
    arm_cols = ["shirt", "stripe", "shirt", "shirt", "skin", "skin", "skin", "skin"]
    for side, s in (("l", 1), ("r", -1)):
        d = hr.arm_dir(side)
        sh = Vector((s * 0.19, 0, 0.80))
        rings = [hr.tube_ring(sh + d * t, d, r, r, 8) for t, r in arm]
        mb.rings(rings, lambda i, c: arm_cols[i] if i >= 0 else "shirt", hr.w_arm(side),
                 pole_start=sh + d * -0.075, pole_end=sh + d * 0.345)
        # mitten hand: long axis d, wide along forward, flat along the palm normal
        a = Vector((0, -1, 0))
        b = d.cross(a)
        hc = sh + d * 0.385
        hand = []
        nv = 5
        for k in range(1, nv):
            phi = -math.pi / 2 + math.pi * k / nv
            cen = hc + d * (0.058 * math.sin(phi))
            ring = []
            for j in range(8):
                th = 2 * math.pi * j / 8
                ring.append(cen + a * (0.046 * math.cos(phi) * math.cos(th))
                            + b * (0.030 * math.cos(phi) * math.sin(th)))
            hand.append(ring)
        mb.rings(hand, lambda i, c: "skin", hr.rigid(f"hand_{side}"),
                 pole_start=hc - d * 0.058, pole_end=hc + d * 0.058)

    # ---- shoes: flattened ellipsoids with a white strap
    for side, s in (("l", 1), ("r", -1)):
        rings, p0, p1 = hr.ellipsoid_rings((s * 0.075, -0.035, 0.042), (0.057, 0.100, 0.052), 10, 6)

        def flat(verts):
            for v in verts:
                v.co.z = max(v.co.z, 0.0)

        def shoe_col(i, c):
            if -0.075 < c.y < -0.035 and c.z > 0.045:
                return "strap"
            return "shoe"

        mb.rings(rings, shoe_col, hr.rigid(f"foot_{side}"), pole_start=p0, pole_end=p1, post=flat)

    # ---- head
    rings, p0, p1 = hr.ellipsoid_rings(HEAD_C, HEAD_R, 16, 10)
    mb.rings(rings, lambda i, c: "skin", hr.rigid("head"), pole_start=p0, pole_end=p1)
    # nose
    rings, p0, p1 = hr.ellipsoid_rings((0, -0.168, 0.988), (0.020, 0.016, 0.016), 8, 4)
    mb.rings(rings, lambda i, c: "skin", hr.rigid("head"), pole_start=p0, pole_end=p1)

    # ---- face decal patch: same angular grid as the head, 2 mm out along the normal
    grid = []
    for k in range(2, 8):  # phi -54 .. +36 deg
        phi = -math.pi / 2 + math.pi * k / 10
        row = []
        for j in range(-3, 4):
            th = -math.pi / 2 + j * 2 * math.pi / 16
            p = Vector((HEAD_R[0] * math.cos(phi) * math.cos(th),
                        HEAD_R[1] * math.cos(phi) * math.sin(th),
                        HEAD_R[2] * math.sin(phi)))
            n = Vector((p.x / HEAD_R[0] ** 2, p.y / HEAD_R[1] ** 2, p.z / HEAD_R[2] ** 2)).normalized()
            row.append(HEAD_C + p + n * 0.002)
        grid.append(row)
    mb.patch(grid, face_uv, hr.rigid("head"), mat=1)

    # ---- hair: cap (face opening pulled inside the head) + long back mass
    hair_c = Vector((0, 0.012, 1.036))
    hair_r = (0.203, 0.190, 0.192)
    rings, p0, p1 = hr.ellipsoid_rings(hair_c, hair_r, 16, 10)
    fringe_x = [0.0, 0.05, 0.09, 0.12, 0.15, 0.17, 0.19, 0.21]
    fringe_z = [1.135, 1.128, 1.11, 1.085, 1.04, 0.98, 0.84, 0.70]

    def fringe(ax):
        for (x0, z0), (x1, z1) in zip(zip(fringe_x, fringe_z), zip(fringe_x[1:], fringe_z[1:])):
            if ax <= x1:
                return z0 + (z1 - z0) * (ax - x0) / (x1 - x0)
        return fringe_z[-1]

    def pull(verts):
        for v in verts:
            c = v.co
            if -c.y > -0.005 and c.z < fringe(abs(c.x)):
                d = c - HEAD_C
                # same direction, onto the head ellipsoid scaled to 0.9 (hidden inside)
                u = Vector((d.x / HEAD_R[0], d.y / HEAD_R[1], d.z / HEAD_R[2]))
                u.normalize()
                v.co = HEAD_C + Vector((u.x * HEAD_R[0], u.y * HEAD_R[1], u.z * HEAD_R[2])) * 0.9

    mb.rings(rings, lambda i, c: "hair", hr.rigid("head"), pole_start=p0, pole_end=p1, post=pull)

    back = [  # z, cy, rx, ry
        (0.598, 0.120, 0.105, 0.040),
        (0.625, 0.124, 0.160, 0.062),
        (0.70, 0.124, 0.178, 0.075),
        (0.80, 0.110, 0.190, 0.095),
        (0.90, 0.070, 0.200, 0.140),
        (1.00, 0.040, 0.203, 0.170),
        (1.10, 0.030, 0.175, 0.150),
    ]
    rings = [hr.ellipse_ring((0, cy, z), rx, ry, 16) for z, cy, rx, ry in back]
    mb.rings(rings, lambda i, c: "hair", hr.rigid("head"),
             pole_start=Vector((0, 0.118, 0.588)), pole_end=Vector((0, 0.03, 1.13)))


# --------------------------------------------------------------------------- main

def main():
    args = zb.script_args()
    zb.clean_scene()
    bpy.context.scene.render.fps = hr.FPS

    arm = hr.build_armature("player_girl_rig")
    zb.save_blend(RIG_BLEND)  # template armature, identical for every human character
    arm.name = "player_girl_rig"

    atlas = hr.BodyAtlas(COLORS, cells=8, px=8)
    body_img = atlas.write(BODY_PNG)
    face_img = hr.write_face_atlas(FACE_PNG, draw_face)
    mats = [hr.body_material(body_img), hr.face_material(face_img)]

    mb = hr.MeshBuilder(atlas.uv)
    build_mesh(mb)
    mesh = mb.to_object(ASSET, mats, arm)

    d = hr.arm_dir("r")
    sh_r = Vector((-0.19, 0, 0.80))
    sockets = hr.add_sockets(arm, [
        ("socket_hand_r", "hand_r", sh_r + d * 0.385),
        ("socket_hand_l", "hand_l", Vector((0.19, 0, 0.80)) + hr.arm_dir("l") * 0.385),
        ("socket_carry", "chest", (0, -0.20, 0.62)),
        ("socket_head_top", "head", (0, 0, 1.20)),
    ])

    clips = hr.player_clips()
    for name, frames, _loop, fn in clips:
        hr.bake_clip(arm, name, frames, fn)
    hr.reset_pose(arm)
    bpy.context.scene.frame_set(0)

    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    print(f"{ASSET}: {tris} tris, bounds x {mn.x:.3f}..{mx.x:.3f}  y {mn.y:.3f}..{mx.y:.3f}  "
          f"z {mn.z:.3f}..{mx.z:.3f}")

    zb.save_blend(BLEND)
    hr.export_character([arm, mesh] + sockets, GLB)
    print(f"exported {GLB} ({os.path.getsize(GLB) / 1024:.1f} KB)")

    if "--no-preview" not in args:
        ls = hr.setup_preview(mesh, body_img, face_img)
        hr.render_preview(arm, PREVIEW, ls)
        if "--debug" in args:  # extra review renders into the given folder
            hr.render_debug(arm, args[args.index("--debug") + 1], ls, clips)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)


main()
