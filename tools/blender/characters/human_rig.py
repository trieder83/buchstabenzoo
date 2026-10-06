"""Shared `human` rig for all human characters (ART-RIG).

Imported by the character scripts in this folder (player_girl.py, later player_boy.py,
visitors). Everything that must be identical between characters lives here:

- the 20-joint skeleton (names, hierarchy, rest A-pose, bone rolls)      ART-RIG §2
- mesh building helpers with per-vertex skin weights (<= 4 influences)  ART-RIG §3
- the player clip set, generated procedurally (IK legs, no foot slide)  ART-RIG §4
- sockets, materials (`body` flat-colour atlas + `face` decal atlas)     ART-RIG §2.6, §6
- glTF export with the ART-RIG §7 settings and a preview render

Blender space: Z up, the character faces -Y (glTF +Z), its left side is +X.
Poses are described as *relative rotations in rest-armature axes* (`q_rel`): a bone's
world delta is W_b = W_parent @ q_rel_b; the pose-bone quaternion Blender needs is
L_b = R_b^-1 @ q_rel_b @ R_b (R_b = bone rest rotation). That keeps clip code independent
of bone rolls.
"""

import math
import os
import sys

import bmesh
import bpy
import numpy as np
from mathutils import Matrix, Quaternion, Vector

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "lib"))
import zoo_blender as zb  # noqa: E402  (shared, read-only use)

FPS = 30
C45 = math.cos(math.radians(45.0))
DOWN = Vector((0, 0, -1))
FWD = Vector((0, -1, 0))  # character forward in Blender space (glTF +Z)

# --------------------------------------------------------------------------- skeleton

UPPER_ARM = 0.17
LOWER_ARM = 0.17
UPPER_LEG = 0.21
LOWER_LEG = 0.20
HIP_Z = 0.48
ANKLE_Z = 0.07


def arm_dir(side):
    s = 1.0 if side == "l" else -1.0
    return Vector((s * C45, 0.0, -C45))


def _skeleton():
    j = []  # (name, parent, head, tail)
    j.append(("root", None, (0, 0, 0), (0, 0, 0.10)))
    j.append(("hips", "root", (0, 0, 0.50), (0, 0, 0.56)))
    j.append(("spine", "hips", (0, 0, 0.56), (0, 0, 0.70)))
    j.append(("chest", "spine", (0, 0, 0.70), (0, 0, 0.84)))
    j.append(("neck", "chest", (0, 0, 0.84), (0, 0, 0.86)))
    j.append(("head", "neck", (0, 0, 0.86), (0, 0, 1.20)))
    for side in ("l", "r"):
        s = 1.0 if side == "l" else -1.0
        d = arm_dir(side)
        sh = Vector((s * 0.19, 0, 0.80))
        el = sh + d * UPPER_ARM
        wr = el + d * LOWER_ARM
        j.append((f"shoulder_{side}", "chest", (s * 0.10, 0, 0.80), tuple(sh)))
        j.append((f"upper_arm_{side}", f"shoulder_{side}", tuple(sh), tuple(el)))
        j.append((f"lower_arm_{side}", f"upper_arm_{side}", tuple(el), tuple(wr)))
        j.append((f"hand_{side}", f"lower_arm_{side}", tuple(wr), tuple(wr + d * 0.08)))
    for side in ("l", "r"):
        s = 1.0 if side == "l" else -1.0
        x = s * 0.075
        j.append((f"upper_leg_{side}", "hips", (x, 0, HIP_Z), (x, 0, 0.27)))
        j.append((f"lower_leg_{side}", f"upper_leg_{side}", (x, 0, 0.27), (x, 0, ANKLE_Z)))
        j.append((f"foot_{side}", f"lower_leg_{side}", (x, 0, ANKLE_Z), (x, -0.10, 0.02)))
    return j


SKELETON = _skeleton()
JOINT_NAMES = [n for n, *_ in SKELETON]
PARENT = {n: p for n, p, *_ in SKELETON}
REST_HEAD = {n: Vector(h) for n, _, h, _ in SKELETON}
REST_TAIL = {n: Vector(t) for n, _, _, t in SKELETON}
REST_ROT = {}  # filled by build_armature (bone rest rotation, armature space)


def build_armature(name="human"):
    """Create the armature object (identity transform) with the 20 joints in A-pose."""
    arm = bpy.data.armatures.new(name)
    obj = bpy.data.objects.new(name, arm)
    bpy.context.scene.collection.objects.link(obj)
    for o in bpy.context.view_layer.objects:
        o.select_set(False)
    obj.select_set(True)
    bpy.context.view_layer.objects.active = obj
    bpy.ops.object.mode_set(mode="EDIT")
    ebs = {}
    for n, p, h, t in SKELETON:
        eb = arm.edit_bones.new(n)
        eb.head = h
        eb.tail = t
        eb.roll = 0.0
        eb.use_deform = True
        eb.use_connect = False
        if p:
            eb.parent = ebs[p]
        ebs[n] = eb
    bpy.ops.object.mode_set(mode="OBJECT")
    for b in arm.bones:
        REST_ROT[b.name] = b.matrix_local.to_quaternion()
        b.use_inherit_rotation = True
        b.inherit_scale = "FULL"
    for pb in obj.pose.bones:
        pb.rotation_mode = "QUATERNION"
    return obj


def add_sockets(arm_obj, sockets):
    """sockets: [(name, parent_bone, position)] -> empties parented to bones, identity
    world rotation in rest pose (glTF: +Y up, +Z forward)."""
    out = []
    for name, bone, pos in sockets:
        e = bpy.data.objects.new(name, None)
        e.empty_display_type = "ARROWS"
        e.empty_display_size = 0.05
        bpy.context.scene.collection.objects.link(e)
        e.parent = arm_obj
        e.parent_type = "BONE"
        e.parent_bone = bone
        bpy.context.view_layer.update()
        e.matrix_world = Matrix.Translation(Vector(pos))
        out.append(e)
    bpy.context.view_layer.update()
    return out


# --------------------------------------------------------------------------- weights

def smoothstep(e0, e1, x):
    t = min(max((x - e0) / (e1 - e0), 0.0), 1.0)
    return t * t * (3 - 2 * t)


def chain(t, stops):
    """Blend weights along a parameter: stops = [(t, bone), ...] sorted by t. Between two
    stops with different bones the weight fades linearly (smoothstep)."""
    if t <= stops[0][0]:
        return {stops[0][1]: 1.0}
    if t >= stops[-1][0]:
        return {stops[-1][1]: 1.0}
    for (t0, b0), (t1, b1) in zip(stops, stops[1:]):
        if t0 <= t <= t1:
            if b0 == b1:
                return {b0: 1.0}
            k = smoothstep(t0, t1, t)
            return {b0: 1.0 - k, b1: k}
    return {stops[-1][1]: 1.0}


def mix(a, b, k):
    """(1-k)*a + k*b for weight dicts."""
    out = {}
    for n, w in a.items():
        out[n] = out.get(n, 0.0) + w * (1 - k)
    for n, w in b.items():
        out[n] = out.get(n, 0.0) + w * k
    return out


def rigid(bone):
    return lambda co: {bone: 1.0}


def w_torso(co):
    w = chain(co.z, [(0.50, "hips"), (0.60, "spine"), (0.64, "spine"), (0.74, "chest"),
                     (0.83, "chest"), (0.87, "neck")])
    side = "l" if co.x > 0 else "r"
    ax = abs(co.x)
    # upper legs pull the lower pelvis (trouser seat) a little
    kl = smoothstep(0.50, 0.44, co.z) * smoothstep(0.01, 0.06, ax) * 0.5
    if kl > 0:
        w = mix(w, {f"upper_leg_{side}": 1.0}, kl)
    # shoulders: top outer torso follows shoulder / upper arm
    ks = smoothstep(0.08, 0.14, ax) * smoothstep(0.70, 0.78, co.z)
    if ks > 0:
        target = mix({f"shoulder_{side}": 1.0}, {f"upper_arm_{side}": 1.0},
                     smoothstep(0.12, 0.17, ax) * 0.5)
        w = mix(w, target, ks * 0.7)
    return w


def w_neck(co):
    return chain(co.z, [(0.82, "chest"), (0.85, "neck"), (0.88, "neck"), (0.92, "head")])


def w_arm(side):
    d = arm_dir(side)
    s = 1.0 if side == "l" else -1.0
    sh = Vector((s * 0.19, 0, 0.80))

    def f(co):
        t = (co - sh).dot(d)
        return chain(t, [(-0.07, "chest"), (-0.035, f"shoulder_{side}"),
                         (0.0, f"shoulder_{side}"), (0.04, f"upper_arm_{side}"),
                         (0.145, f"upper_arm_{side}"), (0.195, f"lower_arm_{side}"),
                         (0.31, f"lower_arm_{side}"), (0.345, f"hand_{side}")])
    return f


def w_leg(side):
    def f(co):
        return chain(co.z, [(0.08, f"foot_{side}"), (0.11, f"lower_leg_{side}"),
                            (0.245, f"lower_leg_{side}"), (0.30, f"upper_leg_{side}"),
                            (0.43, f"upper_leg_{side}"), (0.52, "hips")])
    return f


# --------------------------------------------------------------------------- mesh building

class MeshBuilder:
    """Collects all parts of a character into one bmesh with UVs (atlas cells),
    material index (0 = body, 1 = face) and deform weights."""

    def __init__(self, atlas_uv):
        self.bm = bmesh.new()
        self.uvl = self.bm.loops.layers.uv.new("UVMap")
        self.dl = self.bm.verts.layers.deform.new()
        self.atlas_uv = atlas_uv
        self.gidx = {n: i for i, n in enumerate(JOINT_NAMES)}

    def _weights(self, verts, weight_fn):
        for v in verts:
            w = {k: x for k, x in weight_fn(v.co).items() if x > 1e-4}
            top = sorted(w.items(), key=lambda kv: -kv[1])[:4]
            tot = sum(x for _, x in top)
            dv = v[self.dl]
            for n, x in top:
                dv[self.gidx[n]] = x / tot

    def rings(self, rings, color_fn, weight_fn, pole_start=None, pole_end=None,
              post=None):
        """Loft closed rings (lists of Vectors, same length). Faces between ring i and
        i+1 get color_fn(i, centroid). Poles close the ends as triangle fans.
        post(bm_verts) may move verts before weighting."""
        bm = self.bm
        vr = [[bm.verts.new(p) for p in r] for r in rings]
        faces = []
        n = len(rings[0])
        for i in range(len(vr) - 1):
            a, b = vr[i], vr[i + 1]
            for k in range(n):
                faces.append((i, bm.faces.new((a[k], a[(k + 1) % n], b[(k + 1) % n], b[k]))))
        if pole_start is not None:
            p = bm.verts.new(pole_start)
            for k in range(n):
                faces.append((-1, bm.faces.new((vr[0][(k + 1) % n], vr[0][k], p))))
            vr.insert(0, [p])
        if pole_end is not None:
            p = bm.verts.new(pole_end)
            for k in range(n):
                faces.append((len(rings) - 1, bm.faces.new((vr[-1][k], vr[-1][(k + 1) % n], p))))
            vr.append([p])
        verts = [v for r in vr for v in r]
        if post:
            post(verts)
        fl = [f for _, f in faces]
        bmesh.ops.recalc_face_normals(bm, faces=fl)
        for i, f in faces:
            c = f.calc_center_median()
            uv = self.atlas_uv(color_fn(i, c))
            for loop in f.loops:
                loop[self.uvl].uv = uv
            f.material_index = 0
        self._weights(verts, weight_fn)
        return verts, fl

    def patch(self, grid, uv_fn, weight_fn, mat=1):
        """Open quad grid (rows of Vectors) with per-vertex UVs from uv_fn(co). Faces keep
        the winding given (rows bottom->top, columns left->right seen from outside)."""
        bm = self.bm
        vg = [[bm.verts.new(p) for p in row] for row in grid]
        fl = []
        for i in range(len(vg) - 1):
            for k in range(len(vg[0]) - 1):
                f = bm.faces.new((vg[i][k], vg[i][k + 1], vg[i + 1][k + 1], vg[i + 1][k]))
                f.material_index = mat
                for loop in f.loops:
                    loop[self.uvl].uv = uv_fn(loop.vert.co)
                fl.append(f)
        verts = [v for r in vg for v in r]
        self._weights(verts, weight_fn)
        return verts, fl

    def to_object(self, name, materials, arm_obj):
        me = bpy.data.meshes.new(name)
        # triangulate here (shortest diagonal) so mirrored quads get mirrored diagonals —
        # the exporter would otherwise split every quad the same way (asymmetric hairline)
        bmesh.ops.triangulate(self.bm, faces=self.bm.faces[:], quad_method="SHORT_EDGE",
                              ngon_method="BEAUTY")
        self.bm.normal_update()
        self.bm.to_mesh(me)
        self.bm.free()
        for m in materials:
            me.materials.append(m)
        me.shade_smooth()  # smooth averaged normals, no sharp edges (ART-RIG §3.5)
        obj = bpy.data.objects.new(name, me)
        bpy.context.scene.collection.objects.link(obj)
        for n in JOINT_NAMES:
            obj.vertex_groups.new(name=n)
        obj.parent = arm_obj
        mod = obj.modifiers.new("Armature", "ARMATURE")
        mod.object = arm_obj
        return obj


def ellipse_ring(center, rx, ry, n, z=None, phase=-math.pi / 2):
    """Horizontal ring (CCW from above) starting at the front (-Y)."""
    cx, cy, cz = center
    z = cz if z is None else z
    return [Vector((cx + rx * math.cos(phase + 2 * math.pi * k / n),
                    cy + ry * math.sin(phase + 2 * math.pi * k / n), z)) for k in range(n)]


def tube_ring(center, axis, ra, rb, n, ref=Vector((0, -1, 0))):
    """Ring around `axis` through `center`; ra along `ref` (projected), rb along axis x ref."""
    axis = axis.normalized()
    a = (ref - axis * ref.dot(axis)).normalized()
    b = axis.cross(a).normalized()
    return [center + a * (ra * math.cos(2 * math.pi * k / n)) + b * (rb * math.sin(2 * math.pi * k / n))
            for k in range(n)]


def ellipsoid_rings(center, r, nu, nv):
    """Rings (bottom -> top, excluding poles) + (bottom pole, top pole) of an ellipsoid."""
    c = Vector(center)
    rings = []
    for k in range(1, nv):
        phi = -math.pi / 2 + math.pi * k / nv
        rings.append(ellipse_ring((c.x, c.y, c.z + r[2] * math.sin(phi)),
                                  r[0] * math.cos(phi), r[1] * math.cos(phi), nu))
    return rings, c + Vector((0, 0, -r[2])), c + Vector((0, 0, r[2]))


# --------------------------------------------------------------------------- textures

def write_rgba_png(path, rgba_top_down):
    """Save an (h, w, 4) float array (row 0 = top) as PNG via Blender."""
    h, w, _ = rgba_top_down.shape
    name = os.path.basename(path)
    img = bpy.data.images.get(name)
    if img:
        bpy.data.images.remove(img)
    img = bpy.data.images.new(name, w, h, alpha=True)
    img.colorspace_settings.name = "sRGB"
    img.pixels.foreach_set(np.ascontiguousarray(rgba_top_down[::-1]).astype(np.float32).ravel())
    os.makedirs(os.path.dirname(path), exist_ok=True)
    img.filepath_raw = path
    img.file_format = "PNG"
    img.save()
    bpy.data.images.remove(img)
    img = bpy.data.images.load(path)
    img.name = name
    return img


def hex_rgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i:i + 2], 16) / 255.0 for i in (0, 2, 4))


class BodyAtlas:
    """Flat-colour body atlas: `cells` x `cells` squares of `px` pixels, one colour per
    cell (row-major from the top-left). UVs go to cell centres -> NEAREST safe."""

    def __init__(self, colors, cells=8, px=8):
        self.names = list(colors)
        self.colors = colors
        self.cells, self.px = cells, px
        assert len(colors) <= cells * cells

    def uv(self, name):
        i = self.names.index(name)
        col, row = i % self.cells, i // self.cells
        return ((col + 0.5) / self.cells, 1.0 - (row + 0.5) / self.cells)

    def write(self, path):
        size = self.cells * self.px
        a = np.zeros((size, size, 4), np.float32)
        a[:] = (1, 0, 1, 1)
        for i, n in enumerate(self.names):
            col, row = i % self.cells, i // self.cells
            a[row * self.px:(row + 1) * self.px, col * self.px:(col + 1) * self.px, :3] = hex_rgb(self.colors[n])
        return write_rgba_png(path, a)


# ---- face decal atlas (ART-RIG §6): 4 x 2 cells of 64 px, cell 0 = neutral

FACE_CELL = 64
FACE_SS = 4  # supersampling
EXPRESSIONS = ["neutral", "blink", "happy", "laugh", "talk", "surprised", "thinking", "sad"]


class Canvas:
    """Supersampled RGBA canvas for one 64 x 64 cell; coordinates in cell pixels."""

    def __init__(self):
        n = FACE_CELL * FACE_SS
        self.rgb = np.zeros((n, n, 3), np.float32)
        self.a = np.zeros((n, n), np.float32)
        ys, xs = np.mgrid[0:n, 0:n]
        self.x = (xs + 0.5) / FACE_SS
        self.y = (ys + 0.5) / FACE_SS

    def fill(self, mask, color):
        self.rgb[mask] = color
        self.a[mask] = 1.0

    def ellipse(self, cx, cy, rx, ry, color, rot=0.0):
        m = self.ellipse_mask(cx, cy, rx, ry, rot)
        self.fill(m, color)
        return m

    def ellipse_mask(self, cx, cy, rx, ry, rot=0.0):
        c, s = math.cos(rot), math.sin(rot)
        dx, dy = self.x - cx, self.y - cy
        u = (dx * c + dy * s) / rx
        v = (-dx * s + dy * c) / ry
        return u * u + v * v <= 1.0

    def stroke(self, pts, width, color):
        """Polyline with round caps."""
        m = np.zeros_like(self.a, bool)
        r2 = (width / 2) ** 2
        for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
            dx, dy = x1 - x0, y1 - y0
            L2 = dx * dx + dy * dy or 1e-9
            t = np.clip(((self.x - x0) * dx + (self.y - y0) * dy) / L2, 0, 1)
            px, py = x0 + t * dx - self.x, y0 + t * dy - self.y
            m |= px * px + py * py <= r2
        self.fill(m, color)

    def arc(self, cx, cy, rx, ry, a0, a1, width, color, n=12):
        """Arc of an ellipse, angles in degrees (0 = +x, 90 = down on screen)."""
        pts = [(cx + rx * math.cos(math.radians(a0 + (a1 - a0) * i / n)),
                cy + ry * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]
        self.stroke(pts, width, color)

    def result(self):
        """Downsample to 64 x 64 (premultiplied average), then dilate colours into the
        transparent texels so linear filtering + alpha test gives no dark fringes."""
        n = FACE_CELL
        a = self.a.reshape(n, FACE_SS, n, FACE_SS).mean(axis=(1, 3))
        rgb = (self.rgb * self.a[..., None]).reshape(n, FACE_SS, n, FACE_SS, 3).sum(axis=(1, 3))
        cov = self.a.reshape(n, FACE_SS, n, FACE_SS).sum(axis=(1, 3))
        filled = cov > 0
        rgb[filled] /= cov[filled][:, None]
        for _ in range(6):
            src = rgb.copy()
            done = filled.copy()
            for dy, dx in ((0, 1), (0, -1), (1, 0), (-1, 0)):
                sh_f = np.roll(done, (dy, dx), axis=(0, 1))
                sh_c = np.roll(src, (dy, dx), axis=(0, 1))
                take = sh_f & ~filled
                rgb[take] = sh_c[take]
                filled |= take
        out = np.zeros((n, n, 4), np.float32)
        out[..., :3] = rgb
        out[..., 3] = a
        # >= 4 px transparent padding (ART-RIG §6.2)
        out[:4, :, 3] = out[-4:, :, 3] = out[:, :4, 3] = out[:, -4:, 3] = 0
        return out


def write_face_atlas(path, draw_fn):
    """draw_fn(canvas, expression) draws one cell. Atlas 256 x 128, 4 x 2 cells."""
    atlas = np.zeros((2 * FACE_CELL, 4 * FACE_CELL, 4), np.float32)
    for i, e in enumerate(EXPRESSIONS):
        cv = Canvas()
        draw_fn(cv, e)
        r, c = divmod(i, 4)
        atlas[r * FACE_CELL:(r + 1) * FACE_CELL, c * FACE_CELL:(c + 1) * FACE_CELL] = cv.result()
    return write_rgba_png(path, atlas)


# --------------------------------------------------------------------------- materials

def body_material(img):
    mat = bpy.data.materials.new("body")
    mat.use_backface_culling = True
    mat.use_nodes = True
    nt = mat.node_tree
    bsdf = next(n for n in nt.nodes if n.type == "BSDF_PRINCIPLED")
    bsdf.inputs["Roughness"].default_value = 1.0
    bsdf.inputs["Metallic"].default_value = 0.0
    tex = nt.nodes.new("ShaderNodeTexImage")
    tex.image = img
    tex.interpolation = "Closest"
    nt.links.new(tex.outputs["Color"], bsdf.inputs["Base Color"])
    return mat


def face_material(img):
    """Face decal: base colour + alpha from the atlas, alpha test (glTF MASK, cutoff 0.5
    via a Math:Round node, which the exporter recognises)."""
    mat = bpy.data.materials.new("face")
    mat.use_backface_culling = True
    mat.use_nodes = True
    nt = mat.node_tree
    bsdf = next(n for n in nt.nodes if n.type == "BSDF_PRINCIPLED")
    bsdf.inputs["Roughness"].default_value = 1.0
    bsdf.inputs["Metallic"].default_value = 0.0
    tex = nt.nodes.new("ShaderNodeTexImage")
    tex.image = img
    tex.interpolation = "Linear"
    rnd = nt.nodes.new("ShaderNodeMath")
    rnd.operation = "ROUND"
    nt.links.new(tex.outputs["Color"], bsdf.inputs["Base Color"])
    nt.links.new(tex.outputs["Alpha"], rnd.inputs[0])
    nt.links.new(rnd.outputs[0], bsdf.inputs["Alpha"])
    return mat


# --------------------------------------------------------------------------- posing

def qaxis(axis, deg):
    return Quaternion(Vector(axis).normalized(), math.radians(deg))


def rx(deg):
    """Rotation about +X (lateral). Positive: a hanging limb swings BACK, the torso leans
    FORWARD, a foot's toe goes DOWN."""
    return qaxis((1, 0, 0), deg)


def ry(deg):
    return qaxis((0, 1, 0), deg)


def rz(deg):
    """Yaw about +Z (CCW from above). Positive turns the front (-Y) towards her left (+X)."""
    return qaxis((0, 0, 1), deg)


def rot_between(a, b):
    return a.normalized().rotation_difference(b.normalized())


class Pose:
    """Relative rotations (rest-armature axes) + hips translation; FK on demand.
    Bones must be set parent-first when world-space setters are used."""

    def __init__(self):
        self.rel = {n: Quaternion() for n in JOINT_NAMES}
        self.hips_offset = Vector((0, 0, 0))

    def world(self, name):
        q = Quaternion()
        chain_ = []
        n = name
        while n:
            chain_.append(n)
            n = PARENT[n]
        for n in reversed(chain_):
            q = q @ self.rel[n]
        return q

    def head(self, name):
        p = PARENT[name]
        if p is None:
            return REST_HEAD[name].copy()
        pos = self.head(p) + self.world(p) @ (REST_HEAD[name] - REST_HEAD[p])
        if name == "hips":
            pos += self.hips_offset
        return pos

    def set_world(self, name, w):
        p = PARENT[name]
        self.rel[name] = self.world(p).inverted() @ w if p else w

    # ---- limbs

    def leg_ik(self, side, ankle_target, foot_pitch_deg=0.0, pole=FWD):
        """Place the ankle at `ankle_target` (knee towards `pole`) and the foot at an
        absolute pitch (positive = toe up)."""
        A = self.head(f"upper_leg_{side}")
        K, T = two_bone(A, ankle_target, UPPER_LEG, LOWER_LEG, pole)
        wu = rot_between(DOWN, K - A)
        self.set_world(f"upper_leg_{side}", wu)
        wl = rot_between(K - A, T - K) @ wu
        self.set_world(f"lower_leg_{side}", wl)
        self.set_world(f"foot_{side}", rx(-foot_pitch_deg))

    def arm_ik(self, side, wrist_target, pole, hand_world=None):
        A = self.head(f"upper_arm_{side}")
        K, T = two_bone(A, wrist_target, UPPER_ARM, LOWER_ARM, pole)
        d = arm_dir(side)
        wu = rot_between(d, K - A)
        self.set_world(f"upper_arm_{side}", wu)
        wl = rot_between(K - A, T - K) @ wu
        self.set_world(f"lower_arm_{side}", wl)
        if hand_world is not None:
            self.set_world(f"hand_{side}", hand_world)

    def arm_fk(self, side, down=25.0, swing=0.0, elbow=10.0, out=0.0, hand=0.0):
        """Hanging-arm helper: lower the A-pose arm by `down` deg (towards the body),
        swing forward(-)/back(+) about the lateral axis, bend the elbow forward."""
        s = 1.0 if side == "l" else -1.0
        self.rel[f"upper_arm_{side}"] = rx(swing) @ qaxis((0, 0, 1), -s * out) @ ry(s * down)
        d = arm_dir(side)
        self.rel[f"lower_arm_{side}"] = qaxis(d.cross(FWD), elbow)
        self.rel[f"hand_{side}"] = qaxis(d.cross(FWD), hand)


def two_bone(A, T, l1, l2, pole):
    d = T - A
    dist = d.length
    dist = min(max(dist, abs(l1 - l2) + 1e-4), (l1 + l2) * 0.9995)
    u = d.normalized()
    a = (l1 * l1 - l2 * l2 + dist * dist) / (2 * dist)
    h = math.sqrt(max(l1 * l1 - a * a, 0.0))
    pp = pole - u * pole.dot(u)
    pp = pp.normalized() if pp.length > 1e-6 else Vector((0, -1, 0))
    K = A + u * a + pp * h
    return K, A + u * dist


def apply_pose(arm_obj, pose, frame, prev=None):
    """Write the pose as keyframes (rotation on all bones except root, location on hips)."""
    out = {}
    for n in JOINT_NAMES:
        if n == "root":
            continue
        pb = arm_obj.pose.bones[n]
        R = REST_ROT[n]
        q = R.inverted() @ pose.rel[n] @ R
        q.normalize()
        if prev and n in prev and prev[n].dot(q) < 0:
            q = -q
        pb.rotation_quaternion = q
        pb.keyframe_insert("rotation_quaternion", frame=frame, group=n)
        out[n] = q
    hb = arm_obj.pose.bones["hips"]
    hb.location = REST_ROT["hips"].inverted() @ pose.hips_offset
    hb.keyframe_insert("location", frame=frame, group="hips")
    return out


def bake_clip(arm_obj, name, frames, pose_fn):
    """One action per clip, frames 0..frames (last == first for loops), pushed onto its
    own NLA track so the exporter's ACTIONS mode exports it."""
    bpy.context.preferences.edit.keyframe_new_interpolation_type = "LINEAR"
    ad = arm_obj.animation_data or arm_obj.animation_data_create()
    act = bpy.data.actions.new(name)
    act.use_fake_user = True
    ad.action = act
    prev = None
    for f in range(frames + 1):
        prev = apply_pose(arm_obj, pose_fn(f), f, prev)
    act.use_frame_range = True
    act.frame_start = 0
    act.frame_end = frames
    track = ad.nla_tracks.new()
    track.name = name
    strip = track.strips.new(name, 0, act)
    strip.name = name
    ad.action = None
    return act


def reset_pose(arm_obj):
    for pb in arm_obj.pose.bones:
        pb.rotation_quaternion = Quaternion()
        pb.location = (0, 0, 0)


# --------------------------------------------------------------------------- clips

def ease(x):
    return 0.5 - 0.5 * math.cos(math.pi * min(max(x, 0.0), 1.0))


def envelope(f, a, b, c, d):
    """0 before a, ramps up a->b, 1 until c, ramps down c->d."""
    if f <= a or f >= d:
        return 0.0
    if f < b:
        return ease((f - a) / (b - a))
    if f <= c:
        return 1.0
    return 1.0 - ease((f - c) / (d - c))


REST_ANKLE = {"l": Vector((0.075, 0, ANKLE_Z)), "r": Vector((-0.075, 0, ANKLE_Z))}


def base_stand(p, hips_offset, hips_rot=Quaternion()):
    """Hips transform + planted feet (IK)."""
    p.hips_offset = Vector(hips_offset)
    p.rel["hips"] = hips_rot
    for s in ("l", "r"):
        p.leg_ik(s, REST_ANKLE[s], 0.0)


def clip_idle(f, n=90):
    t = 2 * math.pi * f / n
    p = Pose()
    sway = math.sin(t)
    breath = math.sin(2 * t)
    base_stand(p, (0.012 * sway, 0, -0.012 + 0.004 * breath), ry(1.2 * sway))
    p.rel["spine"] = ry(-1.0 * sway) @ rx(1.0 + 0.8 * breath)
    p.rel["chest"] = rx(-0.8 * breath)
    p.rel["neck"] = rz(4.0 * math.sin(t + 0.6))
    p.rel["head"] = rx(-2.0 * math.sin(2 * t + 1.0)) @ ry(-1.5 * sway)
    for s, k in (("l", 1), ("r", -1)):
        p.rel[f"shoulder_{s}"] = ry(k * 1.5 * breath)
        p.arm_fk(s, down=30.0 + 1.5 * breath, swing=-2.0 + 2.0 * math.sin(t + k), elbow=12.0)
    return p


class Gait:
    """Procedural locomotion cycle with IK feet that move exactly with the ground while
    planted (no foot sliding) — ART-RIG §4.7, RIG-010."""

    def __init__(self, frames, speed, stance, heel_deg, toe_deg, lift, flight_lift=0.0):
        self.n = frames
        self.T = frames / FPS
        self.v = speed
        self.stance = stance
        self.heel = heel_deg
        self.toe = toe_deg
        self.lift = lift
        self.flight_lift = flight_lift
        self.D = speed * self.T * stance       # ground travel while planted
        self.s1, self.s2 = 0.15, 0.55           # heel-roll end, toe-roll start
        # centre the planted range under the hip
        f0, _ = self._stance_ankle(0.0, 0.0)
        f1, _ = self._stance_ankle(1.0, 0.0)
        self.a = -(f0 + f1) / 2

    def _stance_ankle(self, s, a):
        """(forward, height, pitch) of the ankle at stance phase s for heel strike at a."""
        g = self.D * s
        if s < self.s1:
            p = math.radians(self.heel * (1 - s / self.s1))
            heel = a - g
            return heel + 0.05 * math.cos(p) - 0.07 * math.sin(p), 0.05 * math.sin(p) + 0.07 * math.cos(p)
        if s <= self.s2:
            return a - g + 0.05, 0.07
        q = math.radians(self.toe * (s - self.s2) / (1 - self.s2))
        ball = a + 0.13 - g
        return ball - 0.08 * math.cos(q) + 0.07 * math.sin(q), 0.08 * math.sin(q) + 0.07 * math.cos(q)

    def _pitch(self, s):
        if s < self.s1:
            return self.heel * (1 - s / self.s1)
        if s <= self.s2:
            return 0.0
        return -self.toe * (s - self.s2) / (1 - self.s2)

    def foot(self, phase):
        """phase in [0,1): 0 = heel strike. Returns (forward, height, pitch_deg, planted)."""
        if phase < self.stance:
            s = phase / self.stance
            fw, h = self._stance_ankle(s, self.a)
            return fw, h, self._pitch(s), True
        u = (phase - self.stance) / (1 - self.stance)
        f0, h0 = self._stance_ankle(1.0, self.a)
        f1, h1 = self._stance_ankle(0.0, self.a)
        k = ease(u)
        fw = f0 + (f1 - f0) * k
        h = h0 + (h1 - h0) * u + self.lift * math.sin(math.pi * u) ** 1.5
        pitch = -self.toe + (self.heel + self.toe) * ease(min(u * 1.15, 1.0))
        return fw, h, pitch, False

    def hip_heights(self, yaw_fn, sway_fn):
        """Highest hips offset per frame that keeps the planted foot reachable."""
        out = []
        for f in range(self.n):
            best = 0.0
            for side, ph0 in (("l", 0.0), ("r", 0.5)):
                ph = (f / self.n + ph0) % 1.0
                fw, h, _, planted = self.foot(ph)
                s = 1.0 if side == "l" else -1.0
                hip_local = Quaternion((0, 0, 1), math.radians(yaw_fn(f))) @ Vector((s * 0.075, 0, -0.02))
                dx = s * 0.075 - (hip_local.x + sway_fn(f))
                dfw = fw - (-hip_local.y)
                reach = (UPPER_LEG + LOWER_LEG) * (0.985 if planted else 0.96)
                r2 = reach * reach - dfw * dfw - dx * dx
                need = h + math.sqrt(max(r2, 0.0)) - (0.50 - 0.02) if r2 > 0 else -0.2
                best = min(best, need) if (planted or ph < self.stance + 0.2 or ph > 0.9) else best
            out.append(best)
        # smooth (only ever lower): periodic min-filter then average
        n = self.n
        lo = [min(out[(i + k) % n] for k in (-1, 0, 1)) for i in range(n)]
        sm = [(lo[(i - 1) % n] + 2 * lo[i] + lo[(i + 1) % n]) / 4 for i in range(n)]
        return [min(a, b) for a, b in zip(sm, out)]


def make_locomotion(frames, speed, stance, heel, toe, lift, lean, arm_swing, elbow,
                    yaw_deg, bob_extra=0.0, arm_down=28.0, head_bob=2.0):
    g = Gait(frames, speed, stance, heel, toe, lift)

    def yaw(f):
        return -yaw_deg * math.cos(2 * math.pi * f / frames)

    def sway(f):
        return 0.01 * math.cos(2 * math.pi * f / frames + 0.6)

    hz = g.hip_heights(yaw, sway)

    def pose(f):
        fi = f % frames
        t = 2 * math.pi * fi / frames
        p = Pose()
        extra = bob_extra * max(0.0, math.sin(2 * t - 1.2))  # flight phase lift (run)
        p.hips_offset = Vector((sway(fi), 0, hz[fi] + extra))
        p.rel["hips"] = rz(yaw(fi)) @ ry(2.0 * math.cos(t))
        for side, ph0 in (("l", 0.0), ("r", 0.5)):
            fw, h, pitch, _ = g.foot((fi / frames + ph0) % 1.0)
            s = 1.0 if side == "l" else -1.0
            p.leg_ik(side, Vector((s * 0.075, -fw, h)), pitch)
        p.rel["spine"] = rz(-0.6 * yaw(fi)) @ rx(lean * 0.5)
        p.rel["chest"] = rz(-0.6 * yaw(fi)) @ rx(lean * 0.5 + 1.0 * math.cos(2 * t))
        p.rel["neck"] = rx(-lean * 0.7)
        p.rel["head"] = rx(-lean * 0.6 - 2.0 - head_bob * math.cos(2 * t))
        a = arm_swing * math.cos(t)
        p.arm_fk("l", down=arm_down, swing=a, elbow=elbow + max(0.0, -a) * 0.6)
        p.arm_fk("r", down=arm_down, swing=-a, elbow=elbow + max(0.0, a) * 0.6)
        return p

    return pose, g


def clip_walk():
    pose, g = make_locomotion(24, 1.4, stance=0.55, heel=18.0, toe=38.0, lift=0.07,
                              lean=5.0, arm_swing=26.0, elbow=15.0, yaw_deg=7.0)
    return pose


def clip_run():
    pose, g = make_locomotion(16, 3.0, stance=0.38, heel=12.0, toe=45.0, lift=0.16,
                              lean=14.0, arm_swing=40.0, elbow=75.0, yaw_deg=9.0,
                              bob_extra=0.035, arm_down=38.0, head_bob=3.0)
    return pose


def clip_pick_up(f):
    k = envelope(f, 0, 12, 13, 24)
    p = Pose()
    base_stand(p, (0, 0.02 * k, -0.17 * k), rx(22 * k))
    p.rel["spine"] = rx(14 * k)
    p.rel["chest"] = rx(10 * k)
    p.rel["neck"] = rx(-12 * k)
    p.rel["head"] = rx(-8 * k)
    # hands: from hanging to ~0.40 m high, 0.35 m in front (event pick_up_grab at 12)
    for s in ("l", "r"):
        sgn = 1.0 if s == "l" else -1.0
        idle = Pose()
        idle.arm_fk(s, down=30.0, elbow=12.0)
        p.arm_fk(s, down=30.0, elbow=12.0)
        if k > 0:
            target = Vector((sgn * 0.09, -0.35, 0.40)) + Vector((sgn * 0.02, 0, 0)) * (1 - k)
            hang = p.head(f"hand_{s}")
            wt = hang.lerp(target, k)
            p.arm_ik(s, wt, Vector((sgn * 0.6, 0.3, -0.5)))
    return p


def clip_give(f):
    k = envelope(f, 0, 10, 16, 24)
    p = Pose()
    base_stand(p, (0, 0, -0.012), rx(3 * k))
    p.rel["spine"] = rx(4 * k)
    p.rel["chest"] = rx(3 * k)
    p.rel["head"] = rx(4 * k)
    for s in ("l", "r"):
        sgn = 1.0 if s == "l" else -1.0
        p.arm_fk(s, down=30.0, elbow=12.0)
        if k > 0:
            open_ = envelope(f, 13, 16, 24, 25) * 0.05  # hands open a little on release
            target = Vector((sgn * (0.07 + open_), -0.40, 0.50))
            wt = p.head(f"hand_{s}").lerp(target, k)
            p.arm_ik(s, wt, Vector((sgn * 0.7, 0.2, -0.6)))
    return p


def clip_talk(f, n=60):
    t = 2 * math.pi * f / n
    p = Pose()
    base_stand(p, (0.006 * math.sin(t), 0, -0.012), ry(0.6 * math.sin(t)))
    p.rel["spine"] = rx(1.5)
    p.rel["neck"] = rz(3.0 * math.sin(t))
    p.rel["head"] = rx(4.0 * math.sin(2 * t) ** 2 - 2.0)
    p.arm_fk("l", down=32.0, swing=-3.0, elbow=15.0)
    g = math.sin(2 * t)
    p.arm_fk("r", down=18.0 - 4 * g, swing=-22.0 - 6 * g, elbow=70.0 + 15 * math.sin(4 * t),
             out=10.0 + 8 * g)
    return p


def clip_cheer(f):
    p = Pose()
    crouch = envelope(f, 0, 7, 7, 13)
    land = envelope(f, 22, 27, 29, 44)
    jump = envelope(f, 9, 15, 15, 22)
    arms = envelope(f, 5, 13, 26, 44)
    z = -0.10 * crouch - 0.08 * land
    p.hips_offset = Vector((0, 0, z + 0.14 * jump))
    p.rel["hips"] = rx(8 * crouch + 6 * land - 4 * jump)
    air = jump
    for s in ("l", "r"):
        sgn = 1.0 if s == "l" else -1.0
        tgt = REST_ANKLE[s] + Vector((0, 0.03 * air, 0.14 * air - 0.02 * air))
        p.leg_ik(s, tgt, -18.0 * air)
    p.rel["spine"] = rx(4 * crouch - 6 * jump)
    p.rel["chest"] = rx(-4 * jump)
    p.rel["head"] = rx(-12 * arms)
    for s in ("l", "r"):
        sgn = 1.0 if s == "l" else -1.0
        # from hanging to raised high (~50 deg above horizontal)
        p.arm_fk(s, down=30.0 - 125.0 * arms, swing=-12.0 * arms, elbow=12.0 - 4 * arms)
    return p


def clip_wave(f):
    p = Pose()
    k = envelope(f, 0, 8, 32, 40)
    base_stand(p, (-0.008 * k, 0, -0.012), ry(-1.5 * k))
    p.rel["head"] = rz(-6 * k) @ rx(-4 * k)
    p.arm_fk("l", down=30.0, elbow=12.0)
    w = math.sin(2 * math.pi * (f - 8) / 8.0) if 8 <= f <= 32 else 0.0
    # right arm up to the side (above horizontal), forearm up, waving about the forearm
    p.arm_fk("r", down=30.0 - 85.0 * k, swing=-10.0 * k, elbow=0.0)
    # forearm up (about the forward axis in rest space), waving +-22 deg
    p.rel["lower_arm_r"] = qaxis((0, 1, 0), (70.0 + 22.0 * w) * k)
    return p


def clip_carry(f, n=30):
    t = 2 * math.pi * f / n
    p = Pose()
    base_stand(p, (0, 0, -0.012 + 0.003 * math.sin(2 * t)), rx(-2.0))
    p.rel["spine"] = rx(-2.0)
    p.rel["chest"] = rx(-2.0)
    for s in ("l", "r"):
        sgn = 1.0 if s == "l" else -1.0
        tgt = Vector((sgn * 0.10, -0.17, 0.60 + 0.006 * math.sin(2 * t)))
        p.arm_ik(s, tgt, Vector((sgn * 0.8, 0.4, -0.3)))
        p.set_world(f"hand_{s}", p.world(f"lower_arm_{s}") @ qaxis(arm_dir(s).cross(FWD), 10))
    return p


# ---- drive (golf cart, GAME-CART): character space = origin at the cart's `socket_driver`
# (seat top centre of the left seat). Cart wheel hub (0.27, -0.38, 0.88) -> (0, -0.60, 0.38) here.
DRIVE_HUB = Vector((0.0, -0.60, 0.38))
DRIVE_TILT = -20.0   # steering wheel plane tilt about X (golf_cart.py)
DRIVE_RING = 0.17    # grip radius on the ring (ring r 0.15..0.19)
DRIVE_STEER = 30.0   # wheel angle of drive_turn_l / drive_turn_r (deg)


def wheel_grip(side, steer_deg):
    """Hand target on the tilted steering wheel; steer_deg > 0 turns the wheel to the LEFT
    (counter-clockwise seen by the driver: left hand goes down)."""
    sgn = 1.0 if side == "l" else -1.0
    a = math.radians(sgn * 68.0 + steer_deg)  # clock angle from the top, + towards +X
    local = Vector((math.sin(a) * DRIVE_RING, 0.0, math.cos(a) * DRIVE_RING))
    return DRIVE_HUB + Matrix.Rotation(math.radians(DRIVE_TILT), 3, "X") @ local


def clip_drive(f, steer=0.0, n=30):
    """Seated driving: hips on the seat, thighs forward, feet on the floor / pedal, both
    hands on the wheel, tiny breathing + head motion (loop of n frames)."""
    t = 2 * math.pi * f / n
    br = math.sin(t)
    p = Pose()
    p.hips_offset = Vector((0, -0.13, -0.39 + 0.002 * br))
    p.rel["hips"] = rx(-4.0)
    p.rel["spine"] = rx(12.0 + 0.6 * br)
    p.rel["chest"] = rx(8.0 + 0.6 * br)
    p.rel["neck"] = rx(-6.0)
    p.rel["head"] = rx(-12.0) @ rz(0.15 * steer) @ rx(0.8 * math.sin(2 * t))
    # feet: left flat on the floor, right on the pedal (pressed a little, tiny pulse)
    p.leg_ik("l", Vector((0.075, -0.28, -0.105)), 0.0, pole=Vector((0.1, -1.0, 0.6)))
    p.leg_ik("r", Vector((-0.075, -0.31 - 0.004 * br, -0.09)), 12.0, pole=Vector((-0.1, -1.0, 0.6)))
    for s in ("l", "r"):
        sgn = 1.0 if s == "l" else -1.0
        tgt = wheel_grip(s, steer) + Vector((0, 0.004 * math.sin(2 * t + sgn), 0.003 * br))
        p.arm_ik(s, tgt, Vector((sgn * 0.5, 0.35, -0.8)))
        p.set_world(f"hand_{s}", p.world(f"lower_arm_{s}"))
    return p


# (name, frames, loop, pose_fn(frame))
def player_clips():
    walk = clip_walk()
    run = clip_run()
    return [
        ("idle", 90, True, clip_idle),
        ("walk", 24, True, walk),
        ("run", 16, True, run),
        ("pick_up", 24, False, clip_pick_up),
        ("give", 24, False, clip_give),
        ("talk", 60, True, clip_talk),
        ("cheer", 45, False, clip_cheer),
        ("wave", 40, False, clip_wave),
        ("carry", 30, True, clip_carry),
        ("drive", 30, True, clip_drive),
        ("drive_turn_l", 30, True, lambda f: clip_drive(f, DRIVE_STEER)),
        ("drive_turn_r", 30, True, lambda f: clip_drive(f, -DRIVE_STEER)),
    ]


# --------------------------------------------------------------------------- export

def export_character(objs, path):
    """ART-RIG §7 export: armature + skinned mesh + socket empties, one action per clip."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    for o in bpy.context.view_layer.objects:
        o.select_set(o in objs)
    bpy.context.view_layer.objects.active = next(o for o in objs if o.type == "ARMATURE")
    bpy.ops.export_scene.gltf(
        filepath=path,
        export_format="GLB",
        use_selection=True,
        export_yup=True,
        export_apply=True,
        export_extras=False,
        export_draco_mesh_compression_enable=False,
        export_meshopt_compression_enable=False,
        export_vertex_color="NONE",
        export_all_vertex_colors=False,
        export_attributes=False,
        export_materials="EXPORT",
        export_image_format="AUTO",
        export_texcoords=True,
        export_normals=True,
        export_tangents=False,
        export_cameras=False,
        export_lights=False,
        export_skins=True,
        export_def_bones=True,
        export_influence_nb=4,
        export_all_influences=False,
        export_rest_position_armature=True,
        export_leaf_bone=False,
        export_morph=False,
        export_animations=True,
        export_animation_mode="ACTIONS",
        export_nla_strips=True,
        export_force_sampling=True,
        export_frame_step=1,
        export_optimize_animation_size=True,
        export_optimize_animation_keep_anim_armature=False,
        export_anim_slide_to_zero=True,
        export_bake_animation=False,
        export_reset_pose_bones=True,
        export_anim_single_armature=True,
        export_frame_range=False,
    )


# --------------------------------------------------------------------------- preview

def _face_toon(img):
    # no shadow tone on the decal in the preview: EEVEE self-shadows the 2 mm offset patch
    mat = zb._toon_material("pv_face", tex_image=img, shadow=(1.0, 1.0, 1.0))
    nt = mat.node_tree
    tex = next(n for n in nt.nodes if n.type == "TEX_IMAGE")
    tex.interpolation = "Linear"
    emit = next(n for n in nt.nodes if n.type == "EMISSION")
    out = next(n for n in nt.nodes if n.type == "OUTPUT_MATERIAL")
    tr = nt.nodes.new("ShaderNodeBsdfTransparent")
    mix = nt.nodes.new("ShaderNodeMixShader")
    rnd = nt.nodes.new("ShaderNodeMath")
    rnd.operation = "ROUND"
    nt.links.new(tex.outputs["Alpha"], rnd.inputs[0])
    nt.links.new(rnd.outputs[0], mix.inputs[0])
    nt.links.new(tr.outputs[0], mix.inputs[1])
    nt.links.new(emit.outputs[0], mix.inputs[2])
    nt.links.new(mix.outputs[0], out.inputs["Surface"])
    return mat


def _pose_at(arm_obj, action_name, frame):
    ad = arm_obj.animation_data
    ad.use_nla = False
    ad.action = bpy.data.actions[action_name] if action_name else None
    if action_name is None:
        reset_pose(arm_obj)
    bpy.context.scene.frame_set(frame)
    bpy.context.view_layer.update()


def _render(out, res, cam_loc, cam_dir, ortho_scale, ground_rgb):
    scene = bpy.context.scene
    cam = scene.camera
    cam.location = cam_loc
    cam.rotation_euler = cam_dir.to_track_quat("-Z", "Y").to_euler()
    cam.data.ortho_scale = ortho_scale
    scene.render.resolution_x, scene.render.resolution_y = res
    g = bpy.data.materials["pv_ground"]
    ramp = next(n for n in g.node_tree.nodes if n.type == "MIX")
    ramp.inputs["A"].default_value = (*[zb.srgb_to_linear(c) for c in ground_rgb], 1)
    scene.render.filepath = out
    bpy.ops.render.render(write_still=True)
    return out


def setup_preview(mesh_obj, body_img, face_img):
    """Preview-only scene setup (toon materials, ground, camera, sun, Freestyle). Call
    after the .blend is saved and the .glb exported; nothing here is ever exported."""
    scene = bpy.context.scene
    mesh_obj.material_slots[0].link = "OBJECT"
    mesh_obj.material_slots[0].material = zb._toon_material("pv_body", tex_image=body_img)
    mesh_obj.material_slots[1].link = "OBJECT"
    mesh_obj.material_slots[1].material = _face_toon(face_img)

    bpy.ops.mesh.primitive_plane_add(size=60, location=(0, 0, 0))
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
    ls.linestyle.color = tuple(zb.srgb_to_linear(c) for c in hr_outline())
    scene.render.image_settings.file_format = "PNG"
    return ls


def render_preview(arm_obj, out_png, ls):
    """Side by side: 55 deg game camera (walk pose, over grass) | front close-up (rest
    A-pose) | back (rest) | game camera at in-game size (~80 px tall, 2x nearest)."""
    import tempfile
    tmp = tempfile.mkdtemp(prefix="pv_")

    pitch = math.radians(55.0)
    yaw = math.radians(25.0)  # camera looks north-north-east -> sees her front-left
    fwd = Vector((math.sin(yaw), math.cos(yaw), 0))
    view = (fwd * math.cos(pitch) + Vector((0, 0, -math.sin(pitch)))).normalized()
    target = Vector((0, 0, 0.62))
    panels = []
    ls.linestyle.thickness = 3.0
    _pose_at(arm_obj, "walk", 6)
    panels.append(_render(os.path.join(tmp, "a.png"), (700, 900), target - view * 20, view, 2.0,
                          (0.56, 0.75, 0.34)))
    _pose_at(arm_obj, None, 0)
    front = Vector((0, 1, -0.08)).normalized()
    panels.append(_render(os.path.join(tmp, "b.png"), (600, 900), Vector((0, 0, 0.62)) - front * 20,
                          front, 1.62, (0.90, 0.90, 0.91)))
    back = Vector((0, -1, -0.08)).normalized()
    panels.append(_render(os.path.join(tmp, "c.png"), (600, 900), Vector((0, 0, 0.62)) - back * 20,
                          back, 1.62, (0.90, 0.90, 0.91)))
    # in-game size: ~80 px character height on a 2340 px tall phone screen
    ls.linestyle.thickness = 1.0
    _pose_at(arm_obj, "walk", 6)
    small = _render(os.path.join(tmp, "d.png"), (130, 450), target - view * 20, view, 4.5,
                    (0.56, 0.75, 0.34))
    panels.append(small)
    _compose(panels, out_png, heights=900)
    import shutil
    shutil.rmtree(tmp, ignore_errors=True)
    print(f"preview -> {out_png}")


def hr_outline():
    return zb.palette_rgb("outline")


def _compose(paths, out_png, heights):
    arrs = []
    for p in paths:
        img = bpy.data.images.load(p)
        w, h = img.size
        a = np.array(img.pixels[:], np.float32).reshape(h, w, 4)
        bpy.data.images.remove(img)
        if h != heights:  # nearest upscale (in-game size panel)
            k = heights // h
            a = a.repeat(k, axis=0).repeat(k, axis=1)
            pad = heights - a.shape[0]
            if pad > 0:
                a = np.concatenate([np.ones((pad, a.shape[1], 4), np.float32), a], axis=0)
        arrs.append(a)
        arrs.append(np.ones((heights, 6, 4), np.float32))
    full = np.concatenate(arrs[:-1], axis=1)
    h, w, _ = full.shape
    img = bpy.data.images.new("pv_out", w, h, alpha=True)
    img.pixels.foreach_set(full.ravel())
    os.makedirs(os.path.dirname(out_png), exist_ok=True)
    img.filepath_raw = out_png
    img.file_format = "PNG"
    img.save()


def render_debug(arm_obj, out_dir, ls, clips):
    """Extra review renders (not committed): head close-ups, side view, clip strips."""
    os.makedirs(out_dir, exist_ok=True)
    grey = (0.90, 0.90, 0.91)
    ls.linestyle.thickness = 2.0
    _pose_at(arm_obj, None, 0)
    head = Vector((0, 0, 1.0))
    for name, d in (("head_front", Vector((0, 1, -0.05))), ("head_34", Vector((0.7, 0.7, -0.1))),
                    ("head_side", Vector((-1, 0, -0.05))), ("head_top55", Vector((0, 0.57, -0.82)))):
        d = d.normalized()
        _render(os.path.join(out_dir, name + ".png"), (600, 600), head - d * 20, d, 0.55, grey)
    side = Vector((-1, 0, -0.05)).normalized()
    _render(os.path.join(out_dir, "side.png"), (500, 900), Vector((0, 0, 0.62)) - side * 20, side, 1.45, grey)
    q = Vector((0.5, 0.85, -0.25)).normalized()
    for name, frames, _loop, _fn in clips:
        step = max(1, frames // 8)
        for tag, d in (("side", side), ("front", q)):
            paths = []
            for f in range(0, frames + 1, step):
                _pose_at(arm_obj, name, f)
                paths.append(_render(os.path.join(out_dir, f"_{name}_{tag}_{f:02d}.png"), (260, 420),
                                     Vector((0, 0, 0.66)) - d * 20, d, 1.8, grey))
            _compose(paths, os.path.join(out_dir, f"clip_{name}_{tag}.png"), heights=420)
