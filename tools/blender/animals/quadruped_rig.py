"""Shared `quadruped` rig for four-legged animals (ART-ANIMALS "Rig conventions").

Imported by the animal scripts in this folder (zebra.py, later lion, snow_fox, ...).
Everything that must be identical between quadrupeds lives here:

- the 23-joint skeleton (names, hierarchy, bone rolls); joint *positions* come from the
  animal's proportions dict                                         ART-ANIMALS §Rig 2
- a mesh builder with per-vertex skin weights (<= 4 influences) and two kinds of UVs:
  flat atlas cells and *pattern maps* (stripes/spots painted into an atlas region and
  mapped by a per-vertex function, so patterns do not depend on the mesh resolution)
- body atlas writer (flat cells + painted map regions)
- procedural clip helpers: FK pose with 2-bone leg IK, a planted-hoof gait (no sliding),
  stand / head-down solver; clips are baked one action per clip
- export (same glTF settings as ART-RIG §7) and the preview renderer

Blender space: Z up, the animal faces -Y (glTF +Z), its left side is +X. Poses are
relative rotations in rest-armature axes (like human_rig): W_b = W_parent @ q_rel_b.
"""

import math
import os
import sys

import bmesh
import bpy
import numpy as np
from mathutils import Quaternion, Vector

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "lib"))
sys.path.insert(0, os.path.join(HERE, "..", "characters"))
import zoo_blender as zb  # noqa: E402
import human_rig as hr  # noqa: E402  (generic helpers only: weights, IK, export, preview)

FPS = 30
DOWN = Vector((0, 0, -1))
FWD = Vector((0, -1, 0))  # animal forward in Blender space (glTF +Z)

# --------------------------------------------------------------------------- skeleton

LEGS = ("front_l", "front_r", "hind_l", "hind_r")
SIDES = (("l", 1.0), ("r", -1.0))


def leg_bones(leg):
    end, side = leg.split("_")
    return f"{end}_upper_{side}", f"{end}_lower_{side}", f"{end}_foot_{side}"


def _joint_table():
    t = [("root", None), ("hips", "root"), ("spine", "hips"), ("chest", "spine"),
         ("neck_1", "chest"), ("neck_2", "neck_1"), ("head", "neck_2"),
         ("ear_l", "head"), ("ear_r", "head"), ("tail_1", "hips"), ("tail_2", "tail_1")]
    for end, parent in (("front", "chest"), ("hind", "hips")):
        for s, _ in SIDES:
            t += [(f"{end}_upper_{s}", parent), (f"{end}_lower_{s}", f"{end}_upper_{s}"),
                  (f"{end}_foot_{s}", f"{end}_lower_{s}")]
    return t


JOINTS = _joint_table()
JOINT_NAMES = [n for n, _ in JOINTS]
PARENT = dict(JOINTS)


def mirror(p, sign):
    return (sign * p[0], p[1], p[2])


class Rig:
    """Joint positions of one animal. P (Blender coords, metres, x = animal's left):
    hips, spine, chest, neck_1, neck_2, head, nose            points on the midline
    ear_base, ear_tip                                         left ear (mirrored for _r)
    tail_1, tail_2, tail_end                                  tail chain
    front_xy, hind_xy: (x, y) of the left leg; front_z, hind_z: (top, knee, fetlock)
    toe: hoof/paw toe offset forward of the fetlock (foot bone tail).
    extra_joints (optional): [(name, parent, head, tail), ...] per-animal *leaf* chains
    appended after the 23 standard joints (e.g. trunk_1..3 under `head`, tail_3 under
    `tail_2`); the standard joints keep their names and parents (ART-ANIMALS §Rig 2)."""

    def __init__(self, P):
        self.P = P
        j = [("root", None, (0, 0, 0), (0, 0, 0.10)),
             ("hips", "root", P["hips"], P["spine"]),
             ("spine", "hips", P["spine"], P["chest"]),
             ("chest", "spine", P["chest"], P["neck_1"]),
             ("neck_1", "chest", P["neck_1"], P["neck_2"]),
             ("neck_2", "neck_1", P["neck_2"], P["head"]),
             ("head", "neck_2", P["head"], P["nose"])]
        for s, sg in SIDES:
            j.append((f"ear_{s}", "head", mirror(P["ear_base"], sg), mirror(P["ear_tip"], sg)))
        j.append(("tail_1", "hips", P["tail_1"], P["tail_2"]))
        j.append(("tail_2", "tail_1", P["tail_2"], P["tail_end"]))
        for end, parent in (("front", "chest"), ("hind", "hips")):
            x, y = P[f"{end}_xy"]
            top, knee, fet = P[f"{end}_z"]
            for s, sg in SIDES:
                X = sg * x
                j.append((f"{end}_upper_{s}", parent, (X, y, top), (X, y, knee)))
                j.append((f"{end}_lower_{s}", f"{end}_upper_{s}", (X, y, knee), (X, y, fet)))
                j.append((f"{end}_foot_{s}", f"{end}_lower_{s}", (X, y, fet), (X, y - P["toe"], 0.0)))
        assert [n for n, *_ in j] == JOINT_NAMES
        for n, par, h, t in P.get("extra_joints", ()):
            assert n not in JOINT_NAMES and par in [x for x, *_ in j], n
            j.append((n, par, tuple(h), tuple(t)))
        self.skel = j
        self.names = [n for n, *_ in j]
        self.parent = {n: par for n, par, *_ in j}
        self.rest_head = {n: Vector(h) for n, _, h, _ in j}
        self.rest_tail = {n: Vector(t) for n, _, _, t in j}
        self.length = {n: (self.rest_tail[n] - self.rest_head[n]).length for n in self.names}
        self.rest_rot = {}
        self.fet_z = {leg: self.rest_head[leg_bones(leg)[2]].z for leg in LEGS}

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


# --------------------------------------------------------------------------- weights

smoothstep = hr.smoothstep
chain = hr.chain
mix = hr.mix


def rigid(bone):
    return lambda co: {bone: 1.0}


# --------------------------------------------------------------------------- atlas

class Atlas:
    """Body atlas (size x size px). Bottom `cell_rows` rows of 16 px cells hold flat
    colours; the area above holds pattern-map regions painted by functions. UVs of flat
    cells point at cell centres; map UVs stay >= 1 px inside their region."""

    CELL = 16

    def __init__(self, size, colors, regions, cell_rows=4):
        self.size = size
        self.colors = colors            # {name: "#hex"}
        self.names = list(colors)
        self.regions = regions          # {name: (x0, y0, w, h)} px, top-left origin
        self.cols = size // self.CELL
        self.cell_y0 = size - cell_rows * self.CELL
        assert len(colors) <= self.cols * cell_rows

    def cell_uv(self, name):
        i = self.names.index(name)
        c, r = i % self.cols, i // self.cols
        px = (c + 0.5) * self.CELL
        py = self.cell_y0 + (r + 0.5) * self.CELL
        return (px / self.size, 1.0 - py / self.size)

    def map_uv(self, region, U, V):
        """U, V in [0, 1] (V = 1 is the top row of the region)."""
        x0, y0, w, h = self.regions[region]
        U = min(max(U, 0.0), 1.0)
        V = min(max(V, 0.0), 1.0)
        px = x0 + 1 + U * (w - 2)
        py = y0 + 1 + (1 - V) * (h - 2)
        return (px / self.size, 1.0 - py / self.size)

    def write(self, path, painters, ss=3):
        """painters: {region: fn(U, V) -> (h, w, 3) rgb arrays} (U, V numpy grids)."""
        n = self.size
        a = np.zeros((n, n, 4), np.float32)
        # unused space = the first colour (not magenta): the atlas is mipmapped, so
        # far mip levels average neighbouring cells
        a[:] = (*hr.hex_rgb(self.colors[self.names[0]]), 1)
        for i, name in enumerate(self.names):
            c, r = i % self.cols, i // self.cols
            y = self.cell_y0 + r * self.CELL
            a[y:y + self.CELL, c * self.CELL:(c + 1) * self.CELL, :3] = hr.hex_rgb(self.colors[name])
        for region, fn in painters.items():
            x0, y0, w, h = self.regions[region]
            ys, xs = np.mgrid[0:h * ss, 0:w * ss]
            px = (xs + 0.5) / ss
            py = (ys + 0.5) / ss
            U = (px - 1) / (w - 2)
            V = 1 - (py - 1) / (h - 2)
            rgb = fn(U, V).reshape(h, ss, w, ss, 3).mean(axis=(1, 3))
            a[y0:y0 + h, x0:x0 + w, :3] = rgb
        return hr.write_rgba_png(path, a)


# --------------------------------------------------------------------------- mesh building

def ring(center, up, lat, r_up, r_lat, n, r_down=None):
    """Closed ring around an axis: vertex k at angle th = 2 pi k / n, position
    center + up * cos(th) * (r_up or r_down below) + lat * sin(th) * r_lat.
    k = 0 is the `up` side, so h = 0.5 + 0.5 cos(th) is 1 on top, 0 underneath and
    symmetric left/right (pattern maps use it as V)."""
    c = Vector(center)
    out = []
    for k in range(n):
        th = 2 * math.pi * k / n
        cu = math.cos(th)
        ru = r_up if (cu >= 0 or r_down is None) else r_down
        out.append(c + up * (cu * ru) + lat * (math.sin(th) * r_lat))
    return out


def ring_h(k, n):
    return 0.5 + 0.5 * math.cos(2 * math.pi * k / n)


class MeshBuilder:
    """All parts of an animal in one bmesh: UVs, one material (body), deform weights."""

    def __init__(self, names=None):
        self.names = list(names or JOINT_NAMES)
        self.bm = bmesh.new()
        self.uvl = self.bm.loops.layers.uv.new("UVMap")
        self.dl = self.bm.verts.layers.deform.new()
        self.gidx = {n: i for i, n in enumerate(self.names)}

    def _weights(self, verts, weight_fn):
        for v in verts:
            w = {k: x for k, x in weight_fn(v.co).items() if x > 1e-4}
            top = sorted(w.items(), key=lambda kv: -kv[1])[:4]
            tot = sum(x for _, x in top)
            dv = v[self.dl]
            for n, x in top:
                dv[self.gidx[n]] = x / tot

    def loft(self, rings, uv_fn, weight_fn, pole_start=None, pole_end=None, face_uv=None):
        """Loft closed rings (same vertex count n). uv_fn(i, k, co) -> atlas UV for ring i
        (-1 = start pole, len(rings) = end pole) and angle index k (float for poles).
        face_uv(i, centroid) may return a UV for the whole face (flat cell) or None."""
        bm = self.bm
        n = len(rings[0])
        R = len(rings)
        vr = [[bm.verts.new(p) for p in r] for r in rings]
        faces = []  # (face, [(i, k)] per loop, ring index for face_uv)
        for i in range(R - 1):
            a, b = vr[i], vr[i + 1]
            for k in range(n):
                k1 = (k + 1) % n
                f = bm.faces.new((a[k], a[k1], b[k1], b[k]))
                faces.append((f, [(i, k), (i, k + 1), (i + 1, k + 1), (i + 1, k)], i))
        extra = []
        if pole_start is not None:
            p = bm.verts.new(pole_start)
            extra.append(p)
            for k in range(n):
                f = bm.faces.new((vr[0][(k + 1) % n], vr[0][k], p))
                faces.append((f, [(0, k + 1), (0, k), (-1, k + 0.5)], -1))
        if pole_end is not None:
            p = bm.verts.new(pole_end)
            extra.append(p)
            for k in range(n):
                f = bm.faces.new((vr[-1][k], vr[-1][(k + 1) % n], p))
                faces.append((f, [(R - 1, k), (R - 1, k + 1), (R, k + 0.5)], R - 1))
        fl = [f for f, _, _ in faces]
        bmesh.ops.recalc_face_normals(bm, faces=fl)
        for f, ids, ri in faces:
            flat = face_uv(ri, f.calc_center_median()) if face_uv else None
            # recalc_face_normals may have flipped the face: match loops by vertex
            byvert = {}
            for (i, k), v in zip(ids, _face_verts(ids, vr, extra, R)):
                byvert[v] = (i, k)
            for loop in f.loops:
                if flat is not None:
                    loop[self.uvl].uv = flat
                else:
                    i, k = byvert[loop.vert]
                    loop[self.uvl].uv = uv_fn(i, k, loop.vert.co)
        verts = [v for r in vr for v in r] + extra
        self._weights(verts, weight_fn)
        return verts, fl

    def mark_glow(self, faces):
        """Faces of the eye highlight -> second material slot `eye_glow` (GAME-NIGHT
        NIGHT-006 eyeshine; art/night/README.md "Eyeshine rule")."""
        for f in faces:
            f.material_index = 1

    def to_object(self, name, material, arm_obj):
        me = bpy.data.meshes.new(name)
        bmesh.ops.triangulate(self.bm, faces=self.bm.faces[:], quad_method="SHORT_EDGE",
                              ngon_method="BEAUTY")
        self.bm.normal_update()
        has_glow = any(f.material_index == 1 for f in self.bm.faces)
        self.bm.to_mesh(me)
        self.bm.free()
        me.materials.append(material)
        if has_glow:
            me.materials.append(glow_material(material))
        me.shade_smooth()  # smooth averaged normals (ART-RIG §3.5)
        obj = bpy.data.objects.new(name, me)
        bpy.context.scene.collection.objects.link(obj)
        for n in self.names:
            obj.vertex_groups.new(name=n)
        obj.parent = arm_obj
        mod = obj.modifiers.new("Armature", "ARMATURE")
        mod.object = arm_obj
        return obj


def _face_verts(ids, vr, extra, R):
    out = []
    for i, k in ids:
        if i == -1:
            out.append(extra[0])
        elif i == R:
            out.append(extra[-1])
        else:
            out.append(vr[i][int(k) % len(vr[i])])
    return out


def glow_material(body_mat, name="eye_glow"):
    """`eye_glow` slot: same atlas as `body` (renders identically by day); the game makes it
    emissive #E6F7A0 at night inside the lantern radius (NIGHT-006)."""
    mat = body_mat.copy()
    mat.name = name
    # emissiveFactor = the night glow colour (same convention as the props' *_glow slots,
    # tools/blender/props/README_night.md); the renderer applies it only at night
    bsdf = next(n for n in mat.node_tree.nodes if n.type == "BSDF_PRINCIPLED")
    rgb = [zb.srgb_to_linear(c) for c in hr.hex_rgb("#E6F7A0")]
    bsdf.inputs["Emission Color"].default_value = (*rgb, 1.0)
    bsdf.inputs["Emission Strength"].default_value = 1.0
    return mat


def body_material(img, name="body"):
    """Flat-colour atlas material; LINEAR + mipmaps (pattern maps need minification)."""
    mat = bpy.data.materials.new(name)
    mat.use_backface_culling = True
    mat.use_nodes = True
    nt = mat.node_tree
    bsdf = next(n for n in nt.nodes if n.type == "BSDF_PRINCIPLED")
    bsdf.inputs["Roughness"].default_value = 1.0
    bsdf.inputs["Metallic"].default_value = 0.0
    tex = nt.nodes.new("ShaderNodeTexImage")
    tex.image = img
    tex.interpolation = "Linear"
    nt.links.new(tex.outputs["Color"], bsdf.inputs["Base Color"])
    return mat


# --------------------------------------------------------------------------- posing

qaxis = hr.qaxis
rx, ry, rz = hr.rx, hr.ry, hr.rz
rot_between = hr.rot_between
two_bone = hr.two_bone
ease = hr.ease
envelope = hr.envelope


class Pose:
    """Relative rotations (rest-armature axes) + hips translation; FK on demand."""

    def __init__(self, rig):
        self.rig = rig
        self.rel = {n: Quaternion() for n in rig.names}
        self.hips_offset = Vector((0, 0, 0))

    def world(self, name):
        q = Quaternion()
        ch = []
        n = name
        while n:
            ch.append(n)
            n = self.rig.parent[n]
        for n in reversed(ch):
            q = q @ self.rel[n]
        return q

    def head(self, name):
        p = self.rig.parent[name]
        rh = self.rig.rest_head
        if p is None:
            return rh[name].copy()
        pos = self.head(p) + self.world(p) @ (rh[name] - rh[p])
        if name == "hips":
            pos += self.hips_offset
        return pos

    def point(self, bone, rest_point):
        """World position of a rest-space point rigidly attached to `bone`."""
        return self.head(bone) + self.world(bone) @ (Vector(rest_point) - self.rig.rest_head[bone])

    def set_world(self, name, w):
        p = self.rig.parent[name]
        self.rel[name] = self.world(p).inverted() @ w if p else w

    def leg_ik(self, leg, fet_target, pitch_deg=0.0, pole=None):
        """Fetlock (foot joint) at `fet_target`; knee towards `pole` (front legs: forward,
        hind legs: backward); foot at an absolute pitch (positive = toe up)."""
        up, lo, ft = leg_bones(leg)
        if pole is None:
            pole = FWD if leg.startswith("front") else -FWD
        A = self.head(up)
        K, T = two_bone(A, Vector(fet_target), self.rig.length[up], self.rig.length[lo], pole)
        wu = rot_between(DOWN, K - A)
        self.set_world(up, wu)
        self.set_world(lo, rot_between(K - A, T - K) @ wu)
        self.set_world(ft, rx(-pitch_deg))

    def rest_fetlock(self, leg):
        return self.rig.rest_head[leg_bones(leg)[2]].copy()

    def stand(self, offset=(0, 0, 0), hips_rot=Quaternion(), spine=Quaternion(),
              chest=Quaternion(), lift=None):
        """Body transform, then all four hooves planted at their rest spots (IK).
        lift: optional {leg: (dy, dz, pitch)} offsets."""
        self.hips_offset = Vector(offset)
        self.rel["hips"] = hips_rot
        self.rel["spine"] = spine
        self.rel["chest"] = chest
        for leg in LEGS:
            t = self.rest_fetlock(leg)
            pitch = 0.0
            if lift and leg in lift:
                dy, dz, pitch = lift[leg]
                t += Vector((0, dy, dz))
            self.leg_ik(leg, t, pitch)

    def head_down(self, target_z, split=(0.62, 0.38, -0.45), max_deg=150.0):
        """Bend neck_1 / neck_2 / head forward-down (split of the total angle) until the
        nose point is at `target_z`. Returns the total angle (deg)."""
        nose = self.rig.P["nose"]
        base = {n: self.rel[n].copy() for n in ("neck_1", "neck_2", "head")}

        def z_at(a):
            for n, s in zip(("neck_1", "neck_2", "head"), split):
                self.rel[n] = base[n] @ rx(a * s)
            return self.point("head", nose).z

        # z(angle) is not monotonic (a bent-over neck curls back up): scan for the first
        # crossing, then refine; if the target is out of reach use the lowest angle
        prev_a, prev_z = 0.0, z_at(0.0)
        best = (prev_z, 0.0)
        a = 2.0
        while a <= max_deg:
            z = z_at(a)
            best = min(best, (z, a))
            if z <= target_z:
                lo, hi = prev_a, a
                for _ in range(30):
                    mid = (lo + hi) / 2
                    if z_at(mid) > target_z:
                        lo = mid
                    else:
                        hi = mid
                z_at(hi)
                return hi
            prev_a, prev_z = a, z
            a += 2.0
        z_at(best[1])
        self.min_nose_z = best[0]
        return best[1]


class Clip:
    def __init__(self, name, frames, loop, pose_fn):
        self.name, self.frames, self.loop, self.pose_fn = name, frames, loop, pose_fn


def apply_pose(rig, arm_obj, pose, frame, prev=None):
    out = {}
    for n in rig.names:
        if n == "root":
            continue
        pb = arm_obj.pose.bones[n]
        R = rig.rest_rot[n]
        q = R.inverted() @ pose.rel[n] @ R
        q.normalize()
        if prev and n in prev and prev[n].dot(q) < 0:
            q = -q
        pb.rotation_quaternion = q
        pb.keyframe_insert("rotation_quaternion", frame=frame, group=n)
        out[n] = q
    hb = arm_obj.pose.bones["hips"]
    hb.location = rig.rest_rot["hips"].inverted() @ pose.hips_offset
    hb.keyframe_insert("location", frame=frame, group="hips")
    return out


def bake_clip(rig, arm_obj, clip):
    """One action per clip, frames 0..frames (last == first for loops), on its own NLA
    track so the exporter's ACTIONS mode exports it."""
    bpy.context.preferences.edit.keyframe_new_interpolation_type = "LINEAR"
    ad = arm_obj.animation_data or arm_obj.animation_data_create()
    act = bpy.data.actions.new(clip.name)
    act.use_fake_user = True
    ad.action = act
    prev = None
    for f in range(clip.frames + 1):
        prev = apply_pose(rig, arm_obj, clip.pose_fn(f), f, prev)
    act.use_frame_range = True
    act.frame_start = 0
    act.frame_end = clip.frames
    track = ad.nla_tracks.new()
    track.name = clip.name
    strip = track.strips.new(clip.name, 0, act)
    strip.name = clip.name
    ad.action = None
    return act


# --------------------------------------------------------------------------- gait

# phase offsets: lateral-sequence walk LH -> LF -> RH -> RF (a quarter cycle apart)
WALK_PHASES = {"hind_l": 0.0, "front_l": 0.75, "hind_r": 0.5, "front_r": 0.25}


class QuadGait:
    """Hoof path for one leg relative to its rest spot: planted hooves move exactly with
    the ground (no sliding at the authored speed), toe roll at the end of stance, lift
    and fold in swing. Coordinates: fw = forward (-Y) offset, h = fetlock height."""

    def __init__(self, frames, speed, stance, lift, toe_deg, fold_deg, fet_z, toe_fwd,
                 roll_start=0.62):
        self.n = frames
        self.T = frames / FPS
        self.stance = stance
        self.lift = lift
        self.toe = toe_deg
        self.fold = fold_deg
        self.fz = fet_z
        self.tf = toe_fwd
        self.s2 = roll_start
        self.D = speed * self.T * stance
        f0, _ = self._stance(0.0, 0.0)
        f1, _ = self._stance(1.0, 0.0)
        self.a = -(f0 + f1) / 2

    def _stance(self, s, a):
        g = self.D * s
        if s <= self.s2:
            return a - g, self.fz
        q = math.radians(self.toe * (s - self.s2) / (1 - self.s2))
        toe = a - g + self.tf
        return (toe - self.tf * math.cos(q) + self.fz * math.sin(q),
                self.tf * math.sin(q) + self.fz * math.cos(q))

    def _pitch(self, s):
        return 0.0 if s <= self.s2 else -self.toe * (s - self.s2) / (1 - self.s2)

    def foot(self, phase):
        """phase in [0, 1): 0 = hoof strike. -> (fw, h, pitch_deg, planted)."""
        if phase < self.stance:
            s = phase / self.stance
            fw, h = self._stance(s, self.a)
            return fw, h, self._pitch(s), True
        u = (phase - self.stance) / (1 - self.stance)
        f0, h0 = self._stance(1.0, self.a)
        f1, h1 = self._stance(0.0, self.a)
        k = ease(u)
        fw = f0 + (f1 - f0) * k
        h = h0 + (h1 - h0) * k + self.lift * math.sin(math.pi * u) ** 1.2
        pitch = -self.toe * (1 - ease(min(u * 1.25, 1.0))) - self.fold * math.sin(math.pi * min(u * 1.1, 1.0))
        return fw, h, pitch, False


def make_walk(rig, frames, speed, stance, lift, toe_deg, fold_deg, toe_fwd, upper_fn,
              dip=0.02, roll_deg=1.5, phases=WALK_PHASES):
    """Walk cycle pose function. upper_fn(pose, frame_index, t) poses neck/head/tail/ears
    after the body and legs are set. The body is lowered per frame just enough that every
    hoof target stays reachable (smoothed), so planted hooves never slide."""
    gaits = {leg: QuadGait(frames, speed, stance, lift, toe_deg,
                           fold_deg * (1.0 if leg.startswith("front") else 0.6),
                           rig.fet_z[leg], toe_fwd) for leg in LEGS}

    def body(fi):
        t = 2 * math.pi * fi / frames
        p = Pose(rig)
        p.hips_offset = Vector((0, 0, -dip - 0.008 * math.cos(2 * t)))
        p.rel["hips"] = ry(roll_deg * math.sin(t)) @ rx(0.8 * math.sin(2 * t))
        p.rel["spine"] = rz(1.2 * math.sin(t))
        p.rel["chest"] = rz(-1.2 * math.sin(t)) @ ry(-roll_deg * math.sin(t))
        return p

    def target(leg, fi):
        fw, h, pitch, planted = gaits[leg].foot((fi / frames + phases[leg]) % 1.0)
        r = rig.rest_head[leg_bones(leg)[2]]
        return Vector((r.x, r.y - fw, h)), pitch, planted

    need = []
    for fi in range(frames):
        p = body(fi)
        worst = 0.0
        for leg in LEGS:
            up, lo, _ = leg_bones(leg)
            A = p.head(up)
            T, _, planted = target(leg, fi)
            reach = (rig.length[up] + rig.length[lo]) * (0.985 if planted else 0.97)
            dxy = math.hypot(T.x - A.x, T.y - A.y)
            need_z = (T.z + math.sqrt(max(reach * reach - dxy * dxy, 0.0))) - A.z
            worst = min(worst, need_z)
        need.append(worst)
    n = frames
    lo_ = [min(need[(i + k) % n] for k in (-2, -1, 0, 1, 2)) for i in range(n)]
    sm = [(lo_[(i - 1) % n] + 2 * lo_[i] + lo_[(i + 1) % n]) / 4 for i in range(n)]
    drop = [min(a, b) for a, b in zip(sm, need)]

    def pose(f):
        fi = f % frames
        t = 2 * math.pi * fi / frames
        p = body(fi)
        p.hips_offset.z += drop[fi]
        for leg in LEGS:
            T, pitch, _ = target(leg, fi)
            p.leg_ik(leg, T, pitch)
        upper_fn(p, fi, t)
        return p

    pose.gaits = gaits
    pose.drop = drop
    return pose


# --------------------------------------------------------------------------- export

def export_animal(objs, path):
    """Same glTF settings as the human characters (ART-RIG §7)."""
    hr.export_character(objs, path)


# --------------------------------------------------------------------------- preview

def setup_preview(mesh_obj, body_img):
    """Preview-only scene (toon material, ground, camera, sun, Freestyle). Call after the
    .blend is saved and the .glb exported; nothing here is ever exported."""
    scene = bpy.context.scene
    mat = zb._toon_material("pv_body", tex_image=body_img)
    next(n for n in mat.node_tree.nodes if n.type == "TEX_IMAGE").interpolation = "Linear"
    for slot in mesh_obj.material_slots:  # body (+ eye_glow): same toon look by day
        slot.link = "OBJECT"
        slot.material = mat

    bpy.ops.mesh.primitive_plane_add(size=600, location=(0, 0, 0))  # tall animals: no horizon
    ground = bpy.context.active_object
    ground.data.materials.append(zb._toon_material("pv_ground", flat_rgb=(0.56, 0.75, 0.34),
                                                   shadow=(0.72, 0.72, 0.86)))
    cam_data = bpy.data.cameras.new("pv_cam")
    cam_data.type = "ORTHO"
    cam = bpy.data.objects.new("pv_cam", cam_data)
    scene.collection.objects.link(cam)
    scene.camera = cam
    sun_data = bpy.data.lights.new("pv_sun", "SUN")
    sun_data.energy = 3.0
    sun_data.angle = math.radians(0.5)
    sun = bpy.data.objects.new("pv_sun", sun_data)
    scene.collection.objects.link(sun)
    sun.rotation_euler = Vector((0.55, 0.45, -1.0)).normalized().to_track_quat("-Z", "Y").to_euler()
    world = bpy.data.worlds.new("pv_world")
    world.use_nodes = True
    world.node_tree.nodes["Background"].inputs["Color"].default_value = (0, 0, 0, 1)
    scene.world = world
    scene.render.engine = "BLENDER_EEVEE"
    if hasattr(scene.eevee, "taa_render_samples"):
        scene.eevee.taa_render_samples = 16
    scene.render.resolution_percentage = 100
    scene.view_settings.view_transform = "Standard"
    scene.view_settings.look = "None"
    scene.render.use_freestyle = True
    scene.render.line_thickness_mode = "ABSOLUTE"
    vl = scene.view_layers[0]
    vl.use_freestyle = True
    fs = vl.freestyle_settings
    fs.crease_angle = math.radians(120)
    ls = fs.linesets[0] if len(fs.linesets) else fs.linesets.new("outline")
    ls.select_by_visibility = True
    ls.select_by_edge_types = True
    ls.select_silhouette = True
    ls.select_border = False
    ls.select_crease = False
    if ls.linestyle is None:
        ls.linestyle = bpy.data.linestyles.new("outline")
    ls.linestyle.color = tuple(zb.srgb_to_linear(c) for c in zb.palette_rgb("outline"))
    scene.render.image_settings.file_format = "PNG"
    return ls


def _look(center, direction, dist=30.0):
    d = Vector(direction).normalized()
    return Vector(center) - d * dist, d


def game_view_dir(yaw_deg, pitch_deg=55.0):
    """Camera view direction: yaw measured from north (+Y) clockwise, pitched down."""
    y, p = math.radians(yaw_deg), math.radians(pitch_deg)
    fwd = Vector((math.sin(y), math.cos(y), 0))
    return (fwd * math.cos(p) + Vector((0, 0, -math.sin(p)))).normalized()


def render_preview(arm_obj, out_png, ls, center_z, walk_frame, game_yaw=-60.0,
                   big_scale=3.4, close_scale=2.7, small_px=64, height_m=2.2):
    """55 deg game camera walking on grass | front (rest) | left side (rest) | game
    camera at in-game size (~small_px tall, nearest 4x)."""
    import shutil
    import tempfile
    tmp = tempfile.mkdtemp(prefix="pv_")
    grass = (0.56, 0.75, 0.34)
    grey = (0.90, 0.90, 0.91)
    gv = game_view_dir(game_yaw)
    tgt = (0, 0, center_z * 0.8)
    panels = []
    ls.linestyle.thickness = 3.0
    hr._pose_at(arm_obj, "walk", walk_frame)
    loc, d = _look(tgt, gv)
    panels.append(hr._render(os.path.join(tmp, "a.png"), (760, 900), loc, d, big_scale, grass))
    hr._pose_at(arm_obj, None, 0)
    loc, d = _look((0, 0, center_z), (0, 1, -0.10))
    panels.append(hr._render(os.path.join(tmp, "b.png"), (620, 900), loc, d, close_scale, grey))
    loc, d = _look((0, 0, center_z), (-1, 0, -0.06))
    panels.append(hr._render(os.path.join(tmp, "c.png"), (860, 900), loc, d, close_scale, grey))
    ls.linestyle.thickness = 1.0
    hr._pose_at(arm_obj, "walk", walk_frame)
    res = 150
    scale = height_m * res / small_px
    loc, d = _look(tgt, gv)
    panels.append(hr._render(os.path.join(tmp, "d.png"), (res, res), loc, d, scale, grass))
    hr._compose(panels, out_png, heights=900)
    shutil.rmtree(tmp, ignore_errors=True)
    print(f"preview -> {out_png}")


def render_debug(arm_obj, out_dir, ls, clips, center_z, head_pt, scale=1.0):
    """Extra review renders (not committed): head close-ups, top view, clip strips."""
    os.makedirs(out_dir, exist_ok=True)
    grey = (0.90, 0.90, 0.91)
    ls.linestyle.thickness = 2.0
    hr._pose_at(arm_obj, None, 0)
    for name, d in (("head_front", (0, 1, -0.05)), ("head_34", (0.7, 0.7, -0.15)),
                    ("head_side", (-1, 0, -0.05))):
        loc, dd = _look(head_pt, d)
        hr._render(os.path.join(out_dir, name + ".png"), (500, 500), loc, dd, 1.0 * scale, grey)
    loc, dd = _look((0, 0, center_z), (0.7, -0.7, -0.35))
    hr._render(os.path.join(out_dir, "back_34.png"), (700, 700), loc, dd, 2.8 * scale, grey)
    loc, dd = _look((0, 0, center_z), game_view_dir(-150.0))
    hr._render(os.path.join(out_dir, "game_back.png"), (700, 700), loc, dd, 3.2 * scale, (0.56, 0.75, 0.34))
    side = Vector((-1, 0, -0.05))
    for c in clips:
        step = max(1, c.frames // 6)
        paths = []
        for f in range(0, c.frames + 1, step):
            hr._pose_at(arm_obj, c.name, f)
            loc, dd = _look((0, -0.2, center_z), side)
            paths.append(hr._render(os.path.join(out_dir, f"_{c.name}_{f:02d}.png"), (330, 330),
                                    loc, dd, 3.0 * scale, grey))
        hr._compose(paths, os.path.join(out_dir, f"clip_{c.name}.png"), heights=330)
        for pth in paths:
            os.remove(pth)
