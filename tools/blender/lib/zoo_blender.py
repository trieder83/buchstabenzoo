"""Shared helpers for the headless Blender asset scripts (ART-PIPELINE stage 3).

Run kit scripts with:  blender -b --python tools/blender/props/<kit>.py -- [--no-preview]

Conventions (see tools/blender/README.md):
- 1 Blender unit = 1 m. Scripts model in Blender's Z-up space; the glTF exporter converts
  to Y-up. Game axes: +X east = Blender +X, +Y up = Blender +Z, +Z north = Blender -Y.
- Every asset is ONE mesh object with ONE material ("palette") that samples the shared
  palette atlas assets/textures/palette.png; colour = UV position (palette_uv).
- Smooth normals with sharp edges from angle; no outline geometry (the renderer draws
  outlines and cel shading, art/style/style.md).
- Origin at ground contact: mesh min Z (= glTF min Y) is exactly 0.
- Faces pointing straight down are deleted (never visible from the high game camera).
"""

import math
import os
import struct
import sys
import zlib

import bmesh
import bpy
from mathutils import Matrix, Vector

try:
    import tomllib
except ImportError:  # Blender < 4.0 ships Python 3.10
    tomllib = None

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
PALETTE_TOML = os.path.join(REPO, "tools", "blender", "palette.toml")
PALETTE_PNG = os.path.join(REPO, "assets", "textures", "palette.png")

SMOOTH_ANGLE_DEG = 50.0  # edges sharper than this stay hard (chamfers < 50 deg read round)


# --------------------------------------------------------------------------- args / paths

def script_args():
    """Arguments after '--' on the blender command line."""
    argv = sys.argv
    return argv[argv.index("--") + 1:] if "--" in argv else []


def repo_path(*parts):
    return os.path.join(REPO, *parts)


# --------------------------------------------------------------------------- palette

_PALETTE = None


def load_palette():
    """Return (cells, cell_px, {name: (index, (r, g, b) 0..255)})."""
    global _PALETTE
    if _PALETTE is None:
        with open(PALETTE_TOML, "rb") as f:
            data = tomllib.load(f)
        colors = {}
        used = {}
        for name, entry in data["colors"].items():
            idx = int(entry["index"])
            if idx in used:
                raise ValueError(f"palette index {idx} used by {used[idx]} and {name}")
            used[idx] = name
            h = entry["hex"].lstrip("#")
            colors[name] = (idx, tuple(int(h[i:i + 2], 16) for i in (0, 2, 4)))
        _PALETTE = (int(data["cells"]), int(data["cell_px"]), colors)
    return _PALETTE


def write_palette_png(path=PALETTE_PNG):
    """Generate the palette atlas PNG (no dependencies; unused cells are magenta)."""
    cells, px, colors = load_palette()
    by_index = {idx: rgb for idx, rgb in colors.values()}
    size = cells * px
    rows = []
    for y in range(size):
        row = bytearray([0])  # filter type 0
        cy = y // px
        for cx in range(cells):
            rgb = by_index.get(cy * cells + cx, (255, 0, 255))
            row += bytes(rgb) * px
        rows.append(bytes(row))
    raw = b"".join(rows)

    def chunk(tag, data):
        c = struct.pack(">I", len(data)) + tag + data
        return c + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)

    png = b"\x89PNG\r\n\x1a\n"
    png += chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 2, 0, 0, 0))
    png += chunk(b"IDAT", zlib.compress(raw, 9))
    png += chunk(b"IEND", b"")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    old = open(path, "rb").read() if os.path.exists(path) else None
    if old != png:
        with open(path, "wb") as f:
            f.write(png)
    return path


def palette_uv(name):
    """UV (Blender convention, v up) of the centre of a palette colour's cell."""
    cells, _, colors = load_palette()
    if name not in colors:
        raise KeyError(f"colour '{name}' not in {PALETTE_TOML}")
    idx = colors[name][0]
    col, row = idx % cells, idx // cells
    return ((col + 0.5) / cells, 1.0 - (row + 0.5) / cells)


def palette_rgb(name):
    return tuple(c / 255.0 for c in load_palette()[2][name][1])


def srgb_to_linear(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


# --------------------------------------------------------------------------- scene

def clean_scene():
    bpy.ops.wm.read_factory_settings(use_empty=True)
    scene = bpy.context.scene
    scene.unit_settings.system = "METRIC"
    scene.unit_settings.scale_length = 1.0
    scene.unit_settings.length_unit = "METERS"
    global _MATERIAL
    _MATERIAL = None
    write_palette_png()


_MATERIAL = None


def palette_material():
    """The single shared material: base colour = palette texture (nearest), matte."""
    global _MATERIAL
    if _MATERIAL is not None:
        return _MATERIAL
    img = bpy.data.images.get("palette.png") or bpy.data.images.load(PALETTE_PNG)
    img.name = "palette.png"
    mat = bpy.data.materials.new("palette")
    mat.use_backface_culling = True  # exported as doubleSided = false
    mat.use_nodes = True
    nt = mat.node_tree
    bsdf = nt.nodes.get("Principled BSDF")
    bsdf.inputs["Roughness"].default_value = 1.0
    bsdf.inputs["Metallic"].default_value = 0.0
    tex = nt.nodes.new("ShaderNodeTexImage")
    tex.image = img
    tex.interpolation = "Closest"
    tex.location = (-400, 200)
    nt.links.new(tex.outputs["Color"], bsdf.inputs["Base Color"])
    _MATERIAL = mat
    return mat


# --------------------------------------------------------------------------- mesh building

def _new_bm():
    bm = bmesh.new()
    bm.loops.layers.uv.new("UVMap")
    return bm


def _color_bm(bm, color, top_color=None, side_color=None):
    """Put every face's UVs on its palette cell. top/side override by face normal."""
    uvl = bm.loops.layers.uv.active
    bm.normal_update()
    for f in bm.faces:
        c = color
        if top_color and f.normal.z > 0.7:
            c = top_color
        elif side_color and abs(f.normal.z) <= 0.7:
            c = side_color
        uv = palette_uv(c)
        for loop in f.loops:
            loop[uvl].uv = uv


class Part:
    """A coloured bmesh primitive; merged into an asset by build_object()."""

    def __init__(self, bm):
        self.bm = bm

    def transform(self, matrix):
        bmesh.ops.transform(self.bm, matrix=matrix, verts=self.bm.verts)
        return self

    def move(self, x=0.0, y=0.0, z=0.0):
        return self.transform(Matrix.Translation((x, y, z)))

    def rotate_z(self, deg, pivot=(0, 0, 0)):
        p = Vector(pivot)
        m = Matrix.Translation(p) @ Matrix.Rotation(math.radians(deg), 4, "Z") @ Matrix.Translation(-p)
        return self.transform(m)

    def rotate(self, deg, axis):
        return self.transform(Matrix.Rotation(math.radians(deg), 4, axis))

    def delete_faces(self, pred):
        """Delete faces for which pred(face) is true (e.g. faces hidden inside a wall)."""
        self.bm.normal_update()
        doomed = [f for f in self.bm.faces if pred(f)]
        bmesh.ops.delete(self.bm, geom=doomed, context="FACES_ONLY")
        loose = [v for v in self.bm.verts if not v.link_faces]
        bmesh.ops.delete(self.bm, geom=loose, context="VERTS")
        return self


def box(size, center=(0, 0, 0), color="wood", bevel=0.0, segments=1,
        top_color=None, side_color=None, bevel_edges="all"):
    """Box sitting around `center` (z = centre too). bevel_edges: 'all' | 'vertical' |
    'top' | 'long' (edges parallel to X)."""
    bm = _new_bm()
    bmesh.ops.create_cube(bm, size=1.0)
    bmesh.ops.scale(bm, vec=Vector(size), verts=bm.verts)
    bmesh.ops.translate(bm, vec=Vector(center), verts=bm.verts)
    if bevel > 0:
        edges = []
        top = max(v.co.z for v in bm.verts)
        for e in bm.edges:
            d = (e.verts[1].co - e.verts[0].co).normalized()
            if bevel_edges == "all":
                edges.append(e)
            elif bevel_edges == "vertical" and abs(d.z) > 0.9:
                edges.append(e)
            elif bevel_edges == "top" and all(abs(v.co.z - top) < 1e-6 for v in e.verts):
                edges.append(e)
            elif bevel_edges == "top+vertical" and (
                    abs(d.z) > 0.9 or all(abs(v.co.z - top) < 1e-6 for v in e.verts)):
                edges.append(e)
            elif bevel_edges == "long" and abs(d.x) > 0.9:
                edges.append(e)
        bmesh.ops.bevel(bm, geom=edges, offset=bevel, offset_type="OFFSET",
                        segments=segments, profile=0.5, affect="EDGES", clamp_overlap=True)
    _color_bm(bm, color, top_color, side_color)
    return Part(bm)


def slab(polygon, z0, z1, color="grass", chamfer=0.0, top_color=None, side_color=None):
    """Vertical prism from a CCW 2D polygon (x, y), bottom z0, top z1. chamfer > 0 insets
    the top ring (rounded look with smooth normals: a low 'pebble' / paving stone)."""
    bm = _new_bm()
    verts = [bm.verts.new((x, y, z0)) for x, y in polygon]
    base = bm.faces.new(verts)
    base.normal_update()
    if base.normal.z > 0:
        base.normal_flip()
    h = (z1 - z0) - chamfer
    top = base
    if h > 1e-6:  # vertical side wall (skipped when the chamfer spans the full height)
        res = bmesh.ops.extrude_face_region(bm, geom=[base])
        new_verts = [e for e in res["geom"] if isinstance(e, bmesh.types.BMVert)]
        bmesh.ops.translate(bm, vec=(0, 0, h), verts=new_verts)
        top = [e for e in res["geom"] if isinstance(e, bmesh.types.BMFace)][0]
    if chamfer > 0:
        res2 = bmesh.ops.extrude_face_region(bm, geom=[top])
        nv = [e for e in res2["geom"] if isinstance(e, bmesh.types.BMVert)]
        c = sum((v.co for v in nv), Vector()) / len(nv)
        for v in nv:
            d = Vector((v.co.x - c.x, v.co.y - c.y, 0))
            L = d.length
            if L > 1e-9:
                v.co -= d / L * min(chamfer, L * 0.6)
            v.co.z += chamfer
    bmesh.ops.recalc_face_normals(bm, faces=bm.faces)
    _color_bm(bm, color, top_color, side_color)
    return Part(bm)


def ellipse(cx, cy, rx, ry, sides=8, phase=0.0):
    return [(cx + rx * math.cos(phase + 2 * math.pi * i / sides),
             cy + ry * math.sin(phase + 2 * math.pi * i / sides)) for i in range(sides)]


def rounded_rect(x0, y0, x1, y1, r, seg=1):
    """CCW rounded rectangle polygon; seg = points per corner arc minus one."""
    r = min(r, (x1 - x0) / 2, (y1 - y0) / 2)
    pts = []
    corners = [(x1 - r, y0 + r, -90), (x1 - r, y1 - r, 0), (x0 + r, y1 - r, 90), (x0 + r, y0 + r, 180)]
    for cx, cy, a0 in corners:
        for i in range(seg + 1):
            a = math.radians(a0 + 90 * i / seg)
            pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return pts


def sweep(profile, path, color="hedge", caps=True, top_color=None, side_color=None,
          jitter=None):
    """Sweep a 2D profile [(lateral, z)] along a horizontal polyline [(x, y)] with mitred
    joints. lateral > 0 is to the LEFT of the walking direction. `jitter(p, i_path, i_prof)`
    may return an offset Vector per vertex (for fluffy hedges)."""
    bm = _new_bm()
    rings = []
    n = len(path)
    for i, (px, py) in enumerate(path):
        p = Vector((px, py))
        d_in = (p - Vector(path[i - 1])).normalized() if i > 0 else None
        d_out = (Vector(path[i + 1]) - p).normalized() if i < n - 1 else None
        if d_in is None:
            d_in = d_out
        if d_out is None:
            d_out = d_in
        left_in = Vector((-d_in.y, d_in.x))
        left_out = Vector((-d_out.y, d_out.x))
        miter = (left_in + left_out).normalized()
        scale = 1.0 / max(miter.dot(left_in), 1e-3)
        ring = []
        for j, (lat, z) in enumerate(profile):
            q = p + miter * lat * scale
            co = Vector((q.x, q.y, z))
            if jitter:
                co += jitter(co, i, j)
            ring.append(bm.verts.new(co))
        rings.append(ring)
    m = len(profile)
    for i in range(n - 1):
        for j in range(m - 1):
            bm.faces.new((rings[i][j], rings[i + 1][j], rings[i + 1][j + 1], rings[i][j + 1]))
    if caps:
        bm.faces.new(list(reversed(rings[0])))
        bm.faces.new(rings[-1])
    bm.normal_update()
    bmesh.ops.recalc_face_normals(bm, faces=bm.faces)
    _color_bm(bm, color, top_color, side_color)
    return Part(bm)


def knob(radius, center, color="wood", u=8, v=5, squash=1.0):
    bm = _new_bm()
    bmesh.ops.create_uvsphere(bm, u_segments=u, v_segments=v, radius=radius)
    bmesh.ops.scale(bm, vec=(1, 1, squash), verts=bm.verts)
    bmesh.ops.translate(bm, vec=Vector(center), verts=bm.verts)
    _color_bm(bm, color)
    return Part(bm)


def tuft(center, height=0.1, color="grass_dark", blades=3, seed=0):
    """A small grass tuft: a few thin 3-sided spikes (3 tris each, no base)."""
    import random
    rnd = random.Random(seed)
    bm = _new_bm()
    cx, cy, cz = center
    for b in range(blades):
        a = 2 * math.pi * b / blades + rnd.uniform(-0.3, 0.3)
        ox, oy = cx + 0.025 * math.cos(a), cy + 0.025 * math.sin(a)
        r = 0.022
        base = [bm.verts.new((ox + r * math.cos(a + k * 2.094), oy + r * math.sin(a + k * 2.094), cz))
                for k in range(3)]
        tip = bm.verts.new((ox + 0.05 * math.cos(a), oy + 0.05 * math.sin(a),
                            cz + height * rnd.uniform(0.75, 1.1)))
        for k in range(3):
            bm.faces.new((base[k], base[(k + 1) % 3], tip))
    bm.normal_update()
    _color_bm(bm, color)
    return Part(bm)


def build_object(name, parts, delete_down_faces=True, smooth_angle=SMOOTH_ANGLE_DEG):
    """Merge parts into one mesh object with the palette material; origin at ground."""
    bm = _new_bm()
    tmp = bpy.data.meshes.new("_tmp")
    for part in parts:
        part.bm.to_mesh(tmp)
        bm.from_mesh(tmp)
        part.bm.free()
    bpy.data.meshes.remove(tmp)
    bm.normal_update()
    if delete_down_faces:
        down = [f for f in bm.faces if f.normal.z < -0.7]
        bmesh.ops.delete(bm, geom=down, context="FACES_ONLY")
        loose = [v for v in bm.verts if not v.link_faces]
        bmesh.ops.delete(bm, geom=loose, context="VERTS")
    minz = min(v.co.z for v in bm.verts)
    bmesh.ops.translate(bm, vec=(0, 0, -minz), verts=bm.verts)
    me = bpy.data.meshes.new(name)
    bm.to_mesh(me)
    bm.free()
    me.materials.append(palette_material())
    me.shade_smooth()
    me.set_sharp_from_angle(angle=math.radians(smooth_angle))
    obj = bpy.data.objects.new(name, me)
    bpy.context.scene.collection.objects.link(obj)
    return obj


def tri_count(obj):
    me = obj.data
    me.calc_loop_triangles()
    return len(me.loop_triangles)


def bounds(obj):
    """(min, max) of the mesh in Blender space."""
    xs = [v.co for v in obj.data.vertices]
    mn = Vector((min(c.x for c in xs), min(c.y for c in xs), min(c.z for c in xs)))
    mx = Vector((max(c.x for c in xs), max(c.y for c in xs), max(c.z for c in xs)))
    return mn, mx


# --------------------------------------------------------------------------- export

def export_glb(obj, path):
    """Export one object as .glb (glTF 2.0 binary) with the ART-PIPELINE §9 rules:
    Y-up, transforms applied, no Draco, no vertex colours, no extras/custom props."""
    os.makedirs(os.path.dirname(path), exist_ok=True)
    saved = obj.location.copy()
    obj.location = (0, 0, 0)
    for o in bpy.context.scene.objects:
        o.select_set(o == obj)
    bpy.context.view_layer.objects.active = obj
    bpy.ops.export_scene.gltf(
        filepath=path,
        export_format="GLB",
        use_selection=True,
        export_yup=True,
        export_apply=True,
        export_extras=False,
        export_draco_mesh_compression_enable=False,
        export_vertex_color="NONE",
        export_all_vertex_colors=False,
        export_attributes=False,
        export_materials="EXPORT",
        export_image_format="AUTO",
        export_unused_images=False,
        export_unused_textures=False,
        export_texcoords=True,
        export_normals=True,
        export_tangents=False,
        export_animations=False,
        export_skins=False,
        export_morph=False,
        export_cameras=False,
        export_lights=False,
    )
    obj.location = saved
    obj.select_set(False)


def save_blend(path):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    bpy.context.preferences.filepaths.save_version = 0  # no .blend1 backups
    bpy.ops.wm.save_as_mainfile(filepath=path, relative_remap=True, compress=True)


# --------------------------------------------------------------------------- preview

GAME_PITCH_DEG = 55.0
# Preview yaw: camera looks north-west, so X-aligned pieces run lower-left -> upper-right
# like on the concept sheets, and the south (+Y Blender) faces the in-game camera sees
# are visible.
PREVIEW_YAW_DEG = -45.0


def camera_basis(yaw_deg, pitch_deg=GAME_PITCH_DEG):
    """Horizontal forward (towards the scene) and right vectors for a camera whose yaw is
    measured from game north (+Z game = -Y Blender) clockwise towards east."""
    y = math.radians(yaw_deg)
    fwd = Vector((math.sin(y), -math.cos(y), 0.0))
    right = Vector((fwd.y, -fwd.x, 0.0))  # fwd x up: right-hand side when looking along fwd
    return fwd, right


def layout_grid(objs, cols, spacing, yaw_deg=PREVIEW_YAW_DEG):
    """Place objects in a screen-aligned grid (rows go away from the camera)."""
    fwd, right = camera_basis(yaw_deg)
    sx, sy = spacing if isinstance(spacing, (tuple, list)) else (spacing, spacing)
    for i, o in enumerate(objs):
        r, c = divmod(i, cols)
        o.location = right * (c * sx) - fwd * (r * sy)


def _toon_material(name, tex_image=None, flat_rgb=None, shadow=(0.64, 0.62, 0.78)):
    """Preview-only 2-tone cel material (NOT exported): colour x (lit ? 1 : shadow tint)."""
    mat = bpy.data.materials.new(name)
    mat.use_nodes = True
    nt = mat.node_tree
    for n in list(nt.nodes):
        nt.nodes.remove(n)
    out = nt.nodes.new("ShaderNodeOutputMaterial")
    diff = nt.nodes.new("ShaderNodeBsdfDiffuse")
    s2r = nt.nodes.new("ShaderNodeShaderToRGB")
    ramp = nt.nodes.new("ShaderNodeValToRGB")
    ramp.color_ramp.interpolation = "CONSTANT"
    ramp.color_ramp.elements[0].position = 0.0
    ramp.color_ramp.elements[0].color = (*shadow, 1)
    ramp.color_ramp.elements[1].position = 0.2
    ramp.color_ramp.elements[1].color = (1, 1, 1, 1)
    mix = nt.nodes.new("ShaderNodeMix")
    mix.data_type = "RGBA"
    mix.blend_type = "MULTIPLY"
    mix.inputs["Factor"].default_value = 1.0
    emit = nt.nodes.new("ShaderNodeEmission")
    nt.links.new(diff.outputs[0], s2r.inputs[0])
    nt.links.new(s2r.outputs["Color"], ramp.inputs["Fac"])
    if tex_image is not None:
        tex = nt.nodes.new("ShaderNodeTexImage")
        tex.image = tex_image
        tex.interpolation = "Closest"
        nt.links.new(tex.outputs["Color"], mix.inputs["A"])
    else:
        mix.inputs["A"].default_value = (*[srgb_to_linear(c) for c in flat_rgb], 1)
    nt.links.new(ramp.outputs["Color"], mix.inputs["B"])
    nt.links.new(mix.outputs["Result"], emit.inputs["Color"])
    nt.links.new(emit.outputs[0], out.inputs["Surface"])
    return mat


def add_label(text, location, yaw_deg, size=0.22):
    fwd, right = camera_basis(yaw_deg)
    cu = bpy.data.curves.new("lbl_" + text, "FONT")
    cu.body = text
    cu.size = size
    cu.align_x = "CENTER"
    ob = bpy.data.objects.new("lbl_" + text, cu)
    ob.location = location
    # lie flat on the ground, reading left-to-right along camera right
    ob.rotation_euler = (0, 0, math.atan2(right.y, right.x))
    ob["preview_only"] = True
    bpy.context.scene.collection.objects.link(ob)
    return ob


def render_preview(objs, out_png, yaw_deg=PREVIEW_YAW_DEG, pitch_deg=GAME_PITCH_DEG, res=(1600, 900),
                   labels=True, ortho=True, fov_deg=35.0, extra_objs=()):
    """Render the given (already laid out) objects from the game camera (55 deg pitch,
    orthographic by default, or perspective with a 35 deg vertical FOV) with a simple
    2-tone toon material, one sun with hard shadows and Freestyle outlines — a rough
    stand-in for the in-game cel shader, only to compare against the concept sheets."""
    scene = bpy.context.scene
    fwd, right = camera_basis(yaw_deg)
    pitch = math.radians(pitch_deg)
    view_dir = (fwd * math.cos(pitch) + Vector((0, 0, -math.sin(pitch)))).normalized()
    cam_up = right.cross(view_dir).normalized()

    # preview materials (object-linked so the mesh data keeps the export material)
    toon = _toon_material("preview_toon", tex_image=palette_material().node_tree.nodes["Image Texture"].image)
    all_objs = list(objs) + list(extra_objs)
    for o in all_objs:
        if o.type == "MESH":
            o.material_slots[0].link = "OBJECT"
            o.material_slots[0].material = toon

    # labels under each object
    lbl_mat = _toon_material("preview_label", flat_rgb=(0.29, 0.16, 0.13))
    lbls = []
    if labels:
        for o in objs:
            mn, mx = bounds(o)
            corners = [o.location + Vector((x, y, 0)) for x in (mn.x, mx.x) for y in (mn.y, mx.y)]
            near = min(c.dot(fwd) for c in corners)
            centre = o.location + Vector(((mn.x + mx.x) / 2, (mn.y + mx.y) / 2, 0))
            loc = centre + fwd * (near - centre.dot(fwd) - 0.35) + Vector((0, 0, 0.005))
            lb = add_label(o.name, loc, yaw_deg)
            lb.data.materials.append(lbl_mat)
            lbls.append(lb)

    # fit an orthographic frame around everything
    bpy.context.view_layer.update()
    pts = []
    for o in all_objs + lbls:
        for c in o.bound_box:
            pts.append(o.matrix_world @ Vector(c))
    xs = [p.dot(right) for p in pts]
    ys = [p.dot(cam_up) for p in pts]
    cx, cy = (min(xs) + max(xs)) / 2, (min(ys) + max(ys)) / 2
    w, h = max(xs) - min(xs), max(ys) - min(ys)
    depth_c = sum(p.dot(view_dir) for p in pts) / len(pts)
    target = right * cx + cam_up * cy + view_dir * depth_c
    aspect = res[0] / res[1]

    cam_data = bpy.data.cameras.new("preview_cam")
    cam = bpy.data.objects.new("preview_cam", cam_data)
    scene.collection.objects.link(cam)
    if ortho:
        cam_data.type = "ORTHO"
        cam_data.ortho_scale = max(w, h * aspect) * 1.06
        dist = 60.0
    else:
        cam_data.type = "PERSP"
        cam_data.sensor_fit = "VERTICAL"
        cam_data.angle_y = math.radians(fov_deg)
        dist = (max(h, w / aspect) * 0.53) / math.tan(math.radians(fov_deg) / 2) + 2
    cam_data.clip_end = 500
    cam.location = target - view_dir * dist
    cam.rotation_euler = view_dir.to_track_quat("-Z", "Y").to_euler()
    scene.camera = cam

    # ground plane (light grey like the sheets)
    bpy.ops.mesh.primitive_plane_add(size=400, location=(target.x, target.y, -0.001))
    ground = bpy.context.active_object
    ground.name = "preview_ground"
    ground.data.materials.append(_toon_material("preview_ground", flat_rgb=(0.90, 0.90, 0.91),
                                                shadow=(0.80, 0.78, 0.88)))

    # sun from the upper left of the screen, hard shadows
    sun_data = bpy.data.lights.new("preview_sun", "SUN")
    sun_data.energy = 3.0
    sun_data.angle = math.radians(0.5)
    sun_data.use_shadow = True
    sun = bpy.data.objects.new("preview_sun", sun_data)
    scene.collection.objects.link(sun)
    # direction the light travels: from the upper left of the screen towards the lower right
    light_dir = (right * 1.0 - fwd * 0.3 + Vector((0, 0, -1.1))).normalized()
    sun.rotation_euler = light_dir.to_track_quat("-Z", "Y").to_euler()

    world = bpy.data.worlds.new("preview_world")
    world.use_nodes = True
    # no ambient light: the toon ramp must see 0 on faces turned away from the sun
    # (the ground plane fills the whole orthographic frame, so the sky is never seen)
    world.node_tree.nodes["Background"].inputs["Color"].default_value = (0, 0, 0, 1)
    scene.world = world

    scene.render.engine = "BLENDER_EEVEE"
    for attr, val in (("taa_render_samples", 16), ("use_shadows", True)):
        if hasattr(scene.eevee, attr):
            setattr(scene.eevee, attr, val)
    scene.render.resolution_x, scene.render.resolution_y = res
    scene.render.resolution_percentage = 100
    scene.view_settings.view_transform = "Standard"
    scene.view_settings.look = "None"
    scene.render.film_transparent = False

    # Freestyle outlines (dark brown), stand-in for the renderer's outline pass
    scene.render.use_freestyle = True
    scene.render.line_thickness_mode = "ABSOLUTE"
    scene.render.line_thickness = 2.0
    vl = scene.view_layers[0]
    vl.use_freestyle = True
    fs = vl.freestyle_settings
    fs.crease_angle = math.radians(115)
    ls = fs.linesets[0] if len(fs.linesets) else fs.linesets.new("outline")
    ls.select_by_visibility = True
    ls.select_by_edge_types = True
    ls.select_silhouette = True
    ls.select_border = False  # open mesh borders (tile edges) are not outlines
    ls.select_crease = True
    ls.select_by_collection = False
    if ls.linestyle is None:
        ls.linestyle = bpy.data.linestyles.new("outline")
    ls.linestyle.color = tuple(srgb_to_linear(c) for c in palette_rgb("outline"))
    ls.linestyle.thickness = 2.0

    os.makedirs(os.path.dirname(out_png), exist_ok=True)
    scene.render.image_settings.file_format = "PNG"
    scene.render.filepath = out_png
    bpy.ops.render.render(write_still=True)
    return out_png


def report(objs, budget=500):
    """Print tri counts and sizes (game axes: X, height Y, depth Z); fail over budget."""
    ok = True
    print(f"{'asset':24s} {'tris':>5s}  size x*y*z (m, game Y-up)")
    for o in objs:
        t = tri_count(o)
        mn, mx = bounds(o)
        sz = mx - mn
        flag = "" if t <= budget else "  OVER BUDGET"
        ok &= t <= budget
        print(f"{o.name:24s} {t:5d}  {sz.x:.2f} x {sz.z:.2f} x {sz.y:.2f}{flag}")
    return ok
