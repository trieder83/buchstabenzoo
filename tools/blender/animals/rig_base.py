"""Generic skeleton helpers for the non-quadruped animal rigs (biped_rig.py, fish_rig.py).

quadruped_rig.py hard-codes the 23-joint `quadruped` skeleton in its Pose / bake / mesh
builder. The biped and fish rigs need the same machinery for a different joint table, so
this module re-implements the joint-table dependent parts generically and re-uses
everything else from quadruped_rig (read-only import): atlas writer, ring lofts, material,
export settings, preview scene.

Blender space: Z up, the animal faces -Y (glTF +Z), its left side is +X. Poses are
relative rotations in rest-armature axes: W_b = W_parent @ q_rel_b (as human_rig /
quadruped_rig), hips translation only.
"""

import math
import os
import shutil
import sys
import tempfile

import bpy
from mathutils import Quaternion, Vector

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import quadruped_rig as qr  # noqa: E402  (generic helpers only; never edited from here)

hr = qr.hr
zb = qr.zb
FPS = qr.FPS
DOWN = qr.DOWN
FWD = qr.FWD
X = Vector((1, 0, 0))
Y = Vector((0, 1, 0))
Z = Vector((0, 0, 1))

rx, ry, rz = hr.rx, hr.ry, hr.rz
qaxis = hr.qaxis
rot_between = hr.rot_between
two_bone = hr.two_bone
ease = hr.ease
envelope = hr.envelope
smoothstep = hr.smoothstep
chain = hr.chain
mix = hr.mix
ring = qr.ring
ring_h = qr.ring_h
Atlas = qr.Atlas
body_material = qr.body_material
mirror = qr.mirror
SIDES = qr.SIDES
TAU = 2 * math.pi


def rigid(bone):
    return lambda co: {bone: 1.0}


class Skeleton:
    """joints: [(name, parent, head, tail)] parent-first. Positions in Blender metres."""

    def __init__(self, joints):
        self.skel = [(n, p, Vector(h), Vector(t)) for n, p, h, t in joints]
        self.names = [n for n, *_ in self.skel]
        self.parent = {n: p for n, p, *_ in self.skel}
        self.rest_head = {n: h for n, _, h, _ in self.skel}
        self.rest_tail = {n: t for n, _, _, t in self.skel}
        self.length = {n: (self.rest_tail[n] - self.rest_head[n]).length for n in self.names}
        self.rest_dir = {n: (self.rest_tail[n] - self.rest_head[n]).normalized() for n in self.names}
        self.rest_rot = {}

    def build_armature(self, name):
        arm = bpy.data.armatures.new(name)
        obj = bpy.data.objects.new(name, arm)
        bpy.context.scene.collection.objects.link(obj)
        for o in bpy.context.view_layer.objects:
            o.select_set(False)
        obj.select_set(True)
        bpy.context.view_layer.objects.active = obj
        bpy.ops.object.mode_set(mode="EDIT")
        ebs = {}
        for n, p, h, t in self.skel:
            eb = arm.edit_bones.new(n)
            eb.head, eb.tail, eb.roll = h, t, 0.0
            eb.use_deform, eb.use_connect = True, False
            if p:
                eb.parent = ebs[p]
            ebs[n] = eb
        bpy.ops.object.mode_set(mode="OBJECT")
        for b in arm.bones:
            self.rest_rot[b.name] = b.matrix_local.to_quaternion()
            b.use_inherit_rotation = True
            b.inherit_scale = "FULL"
        for pb in obj.pose.bones:
            pb.rotation_mode = "QUATERNION"
        return obj


class MeshBuilder(qr.MeshBuilder):
    """quadruped_rig.MeshBuilder with this skeleton's joint names (vertex groups)."""

    def __init__(self, skel):
        super().__init__()
        self.names = skel.names
        self.gidx = {n: i for i, n in enumerate(skel.names)}

    def to_object(self, name, material, arm_obj):
        # parent's to_object creates the quadruped groups: build here with ours
        import bmesh
        me = bpy.data.meshes.new(name)
        bmesh.ops.triangulate(self.bm, faces=self.bm.faces[:], quad_method="SHORT_EDGE",
                              ngon_method="BEAUTY")
        self.bm.normal_update()
        has_glow = any(f.material_index == 1 for f in self.bm.faces)
        self.bm.to_mesh(me)
        self.bm.free()
        me.materials.append(material)
        if has_glow:
            me.materials.append(qr.glow_material(material))
        me.shade_smooth()
        obj = bpy.data.objects.new(name, me)
        bpy.context.scene.collection.objects.link(obj)
        for n in self.names:
            obj.vertex_groups.new(name=n)
        obj.parent = arm_obj
        mod = obj.modifiers.new("Armature", "ARMATURE")
        mod.object = arm_obj
        return obj


def curve_loft(mb, pts, radii, uv_fn, weight_fn, n=6, lat=X, cap_start=True, cap_end=True,
               squash=1.0, face_uv=None):
    """Tube along a polyline `pts` (radii per point). Ring frames use a fixed lateral
    axis `lat` (curves that lie in a plane perpendicular to it). uv_fn(i, k, co)."""
    rings = []
    m = len(pts)
    for i, (p, r) in enumerate(zip(pts, radii)):
        a = (pts[min(i + 1, m - 1)] - pts[max(i - 1, 0)]).normalized()
        la = (lat - a * a.dot(lat)).normalized()
        up = la.cross(a).normalized()
        rings.append(ring(p, up, la, r * squash, r, n))
    t0 = (pts[1] - pts[0]).normalized()
    t1 = (pts[-1] - pts[-2]).normalized()
    ps = pts[0] - t0 * radii[0] * 0.8 if cap_start else None
    pe = pts[-1] + t1 * radii[-1] * 0.8 if cap_end else None
    return mb.loft(rings, uv_fn, weight_fn, pole_start=ps, pole_end=pe, face_uv=face_uv)


# --------------------------------------------------------------------------- posing

class Pose:
    """Relative rotations (rest-armature axes) + hips translation; FK on demand."""

    def __init__(self, skel):
        self.sk = skel
        self.rel = {n: Quaternion() for n in skel.names}
        self.hips_offset = Vector((0, 0, 0))

    def world(self, name):
        q = Quaternion()
        ch = []
        n = name
        while n:
            ch.append(n)
            n = self.sk.parent[n]
        for n in reversed(ch):
            q = q @ self.rel[n]
        return q

    def head(self, name):
        p = self.sk.parent[name]
        rh = self.sk.rest_head
        if p is None:
            return rh[name].copy()
        pos = self.head(p) + self.world(p) @ (rh[name] - rh[p])
        if name == "hips":
            pos += self.hips_offset
        return pos

    def point(self, bone, rest_point):
        return self.head(bone) + self.world(bone) @ (Vector(rest_point) - self.sk.rest_head[bone])

    def set_world(self, name, w):
        p = self.sk.parent[name]
        self.rel[name] = self.world(p).inverted() @ w if p else w

    def limb_ik(self, upper, lower, target, pole):
        """Two-bone IK: end of `lower` at `target`, middle joint towards `pole`."""
        sk = self.sk
        A = self.head(upper)
        K, T = two_bone(A, Vector(target), sk.length[upper], sk.length[lower], Vector(pole))
        wu = rot_between(sk.rest_dir[upper], K - A)
        self.set_world(upper, wu)
        wl = rot_between(wu @ sk.rest_dir[lower], T - K) @ wu
        self.set_world(lower, wl)
        return K, T


class Clip:
    def __init__(self, name, frames, loop, pose_fn):
        self.name, self.frames, self.loop, self.pose_fn = name, frames, loop, pose_fn


def apply_pose(skel, arm_obj, pose, frame, prev=None):
    out = {}
    for n in skel.names:
        if n == "root":
            continue
        pb = arm_obj.pose.bones[n]
        R = skel.rest_rot[n]
        q = R.inverted() @ pose.rel[n] @ R
        q.normalize()
        if prev and n in prev and prev[n].dot(q) < 0:
            q = -q
        pb.rotation_quaternion = q
        pb.keyframe_insert("rotation_quaternion", frame=frame, group=n)
        out[n] = q
    hb = arm_obj.pose.bones["hips"]
    hb.location = skel.rest_rot["hips"].inverted() @ pose.hips_offset
    hb.keyframe_insert("location", frame=frame, group="hips")
    return out


def bake_clip(skel, arm_obj, clip):
    """One action per clip on its own NLA track (exporter ACTIONS mode)."""
    bpy.context.preferences.edit.keyframe_new_interpolation_type = "LINEAR"
    ad = arm_obj.animation_data or arm_obj.animation_data_create()
    act = bpy.data.actions.new(clip.name)
    act.use_fake_user = True
    ad.action = act
    prev = None
    for f in range(clip.frames + 1):
        prev = apply_pose(skel, arm_obj, clip.pose_fn(f), f, prev)
    act.use_frame_range = True
    act.frame_start = 0
    act.frame_end = clip.frames
    track = ad.nla_tracks.new()
    track.name = clip.name
    strip = track.strips.new(clip.name, 0, act)
    strip.name = clip.name
    ad.action = None
    return act


def check_gate(asset):
    """ART-PIPELINE §2: no modelling before the concept is approved."""
    import re
    text = open(zb.repo_path("assets", "manifest.toml"), encoding="utf-8").read()
    for block in text.split("[[asset]]")[1:]:
        if re.search(r'^id\s*=\s*"%s"' % asset, block, re.M):
            if re.search(r"^concept_approved\s*=\s*true", block, re.M):
                return
    print(f"{asset}: concept not approved in assets/manifest.toml — not modelling")
    sys.exit(1)


# --------------------------------------------------------------------------- preview

GRASS = (0.56, 0.75, 0.34)
GREY = (0.90, 0.90, 0.91)
WATER = (0.36, 0.62, 0.78)


def setup_preview(mesh_obj, body_img, ground_z=0.0):
    ls = qr.setup_preview(mesh_obj, body_img)
    for o in bpy.data.objects:
        if o.type == "MESH" and o.name.startswith("Plane"):
            o.location.z = ground_z
    return ls


def render_preview(arm_obj, out_png, ls, loco_clip, loco_frame, center_z, height_m,
                   game_yaw=-60.0, big_scale=2.0, close_scale=1.6, small_px=64,
                   ground=GRASS, back=GREY, side_dir=(-1, 0, -0.06)):
    """55 deg game camera (locomotion pose) | front (rest) | left side (rest) | game camera
    at in-game size (~small_px for `height_m`, nearest upscale)."""
    tmp = tempfile.mkdtemp(prefix="pv_")
    gv = qr.game_view_dir(game_yaw)
    tgt = (0, 0, center_z)
    panels = []
    ls.linestyle.thickness = 3.0
    hr._pose_at(arm_obj, loco_clip, loco_frame)
    loc, d = qr._look(tgt, gv)
    panels.append(hr._render(os.path.join(tmp, "a.png"), (760, 900), loc, d, big_scale, ground))
    hr._pose_at(arm_obj, None, 0)
    loc, d = qr._look(tgt, (0, 1, -0.10))
    panels.append(hr._render(os.path.join(tmp, "b.png"), (620, 900), loc, d, close_scale, back))
    loc, d = qr._look(tgt, side_dir)
    panels.append(hr._render(os.path.join(tmp, "c.png"), (860, 900), loc, d, close_scale, back))
    ls.linestyle.thickness = 1.0
    hr._pose_at(arm_obj, loco_clip, loco_frame)
    res = 150
    loc, d = qr._look(tgt, gv)
    panels.append(hr._render(os.path.join(tmp, "d.png"), (res, res), loc, d,
                             height_m * res / small_px, ground))
    hr._compose(panels, out_png, heights=900)
    shutil.rmtree(tmp, ignore_errors=True)
    print(f"preview -> {out_png}")


def render_debug(arm_obj, out_dir, ls, clips, center_z, head_pt, head_scale, strip_scale,
                 ground=GREY, strip_dir=(-1, 0, -0.05)):
    """Extra review renders (never committed): head close-ups, 55 deg views, clip strips."""
    os.makedirs(out_dir, exist_ok=True)
    ls.linestyle.thickness = 2.0
    hr._pose_at(arm_obj, None, 0)
    for name, d in (("head_front", (0, 1, -0.05)), ("head_34", (0.7, 0.7, -0.15)),
                    ("head_side", (-1, 0, -0.05)), ("top", (0.0, 0.05, -1.0))):
        loc, dd = qr._look(head_pt, d)
        hr._render(os.path.join(out_dir, name + ".png"), (500, 500), loc, dd, head_scale, ground)
    for yaw in (-150.0, 30.0, 120.0):
        loc, dd = qr._look((0, 0, center_z), qr.game_view_dir(yaw))
        hr._render(os.path.join(out_dir, f"game_{int(yaw)}.png"), (600, 600), loc, dd,
                   strip_scale, ground)
    for c in clips:
        step = max(1, c.frames // 7)
        for tag, sd in (("", strip_dir), ("_q", (-0.55, 0.8, -0.3))):
            paths = []
            for f in range(0, c.frames + 1, step):
                hr._pose_at(arm_obj, c.name, f)
                loc, dd = qr._look((0, 0, center_z), sd)
                paths.append(hr._render(os.path.join(out_dir, f"_{c.name}_{f:02d}.png"),
                                        (330, 330), loc, dd, strip_scale, ground))
            hr._compose(paths, os.path.join(out_dir, f"clip_{c.name}{tag}.png"), heights=330)
            for pth in paths:
                os.remove(pth)
