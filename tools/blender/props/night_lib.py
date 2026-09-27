"""Multi-node assets for the night / building / gate kits (kit_night, kit_bedroom, kit_gates,
kit_buildings, kit_landmarks, kit_garden). Builds on props_parts.py and lib/zoo_blender.py.

The older kits export ONE mesh with ONE material. The kits built with this module need more:

- **Material slots.** `palette` (the shared palette atlas, as before) plus
  - `*_glow`  — glowing parts (lamp glass, bulbs, lit windows, moon sign). Base colour = the
    palette atlas (UVs on the part's DAY colour cell, e.g. `lamp_glass`), and the glTF
    `emissiveFactor` = the NIGHT glow colour (art/night/README.md). The renderer draws the
    slot like `palette` by day and adds the emission only in night mode (GAME-NIGHT §10).
  - `glass`   — see-through panes (night-house glass fronts, glass doors): flat base colour
    factor `glass_tint`, alpha 0.35, alphaMode BLEND, no texture. Always its own node
    `glass` or inside a moving leaf, so the renderer can draw it last or skip it.
  - `*_face`  — blank text faces the game draws on (note paper, sign boards): flat cream
    base colour factor, no texture, and UVs 0..1 across the face (u = left -> right,
    v = bottom -> top when looking at the face), so a text texture maps onto it directly.
- **Nodes.** The root node carries the asset id and no transform. Moving / hideable parts are
  child nodes with a **translation only** (their pivot: hinge axis foot, hub centre, roof
  origin ...); rotation and scale are never set (applied). Empties (no mesh) mark points:
  `light` / `light_*` = point-light position of a lamp, `socket_*` = attachment points.
- Origin on the ground: min Y of all meshes (in asset space) = 0.
"""

import math
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bmesh  # noqa: E402
import bpy  # noqa: E402
from mathutils import Vector  # noqa: E402

import props_parts as pp  # noqa: E402
from props_parts import zb  # noqa: E402

FRONT = pp.FRONT  # Blender -Y = south = glTF +Z = the side the default camera sees
BACK = pp.BACK

# night emission colours of the *_glow slots (art/night/README.md "Night colours")
GLOW = {
    "lamp_glow": "#FFD66B",
    "bulb_glow": "#FFD66B",
    "bulb_orange_glow": "#FFB547",
    "bulb_cream_glow": "#FFF1C9",
    "window_glow": "#FFC857",
    "door_glow": "#FFC857",
    "window_blue_glow": "#8FB8FF",
    "window_red_glow": "#E8735A",
    "moon_glow": "#FFF4C9",
    "rim_glow": "#8FB8FF",
}
FACE_COLOR = {"note_face": "#FBF8EF", "sign_face": "#F3E6C8"}
GLASS_ALPHA = 0.35


def hex_rgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i:i + 2], 16) / 255.0 for i in (0, 2, 4))


def lin(rgb):
    return tuple(zb.srgb_to_linear(c) for c in rgb)


# --------------------------------------------------------------------------- materials

_MATS = {}


def reset():
    zb.clean_scene()
    _MATS.clear()


def _bsdf(mat):
    return next(n for n in mat.node_tree.nodes if n.type == "BSDF_PRINCIPLED")


def material(name):
    if name in _MATS:
        return _MATS[name]
    if name == "palette":
        mat = zb.palette_material()
    elif name.endswith("_glow"):
        mat = bpy.data.materials.new(name)
        mat.use_backface_culling = True
        mat.use_nodes = True
        nt = mat.node_tree
        b = _bsdf(mat)
        b.inputs["Roughness"].default_value = 1.0
        b.inputs["Metallic"].default_value = 0.0
        tex = nt.nodes.new("ShaderNodeTexImage")
        tex.image = zb.palette_material().node_tree.nodes["Image Texture"].image
        tex.interpolation = "Closest"
        nt.links.new(tex.outputs["Color"], b.inputs["Base Color"])
        b.inputs["Emission Color"].default_value = (*lin(hex_rgb(GLOW[name])), 1.0)
        b.inputs["Emission Strength"].default_value = 1.0
    elif name == "glass":
        mat = bpy.data.materials.new(name)
        mat.use_backface_culling = True
        mat.use_nodes = True
        b = _bsdf(mat)
        b.inputs["Base Color"].default_value = (*lin(zb.palette_rgb("glass_tint")), 1.0)
        b.inputs["Alpha"].default_value = GLASS_ALPHA
        b.inputs["Roughness"].default_value = 1.0
        try:
            mat.surface_render_method = "BLENDED"
        except (AttributeError, TypeError):
            mat.blend_method = "BLEND"
    elif name.endswith("_face"):
        mat = bpy.data.materials.new(name)
        mat.use_backface_culling = True
        mat.use_nodes = True
        b = _bsdf(mat)
        b.inputs["Base Color"].default_value = (*lin(hex_rgb(FACE_COLOR[name])), 1.0)
        b.inputs["Roughness"].default_value = 1.0
    else:
        raise KeyError(f"unknown material slot {name}")
    _MATS[name] = mat
    return mat


# --------------------------------------------------------------------------- parts

def face_quad(corners, name="note_face"):
    """A flat quad [bl, br, tr, tl] (3D, CCW seen from the front) with UVs 0..1 — the blank
    text face of a `*_face` slot. Returns a Part (add it with mat=name)."""
    bm = zb._new_bm()
    uvl = bm.loops.layers.uv.active
    vs = [bm.verts.new(c) for c in corners]
    f = bm.faces.new(vs)
    for loop, uv in zip(f.loops, ((0, 0), (1, 0), (1, 1), (0, 1))):
        loop[uvl].uv = uv
    bm.normal_update()
    return zb.Part(bm)


def ring_xz(cx, cz, r, sides, y0, y1, color, r_in=None):
    """Round disc (or ring with r_in) facing -Y (front), extruded from y0 to y1."""
    if r_in is None:
        poly = [(cx + r * math.cos(2 * math.pi * i / sides + math.pi / sides),
                 cz + r * math.sin(2 * math.pi * i / sides + math.pi / sides)) for i in range(sides)]
        return pp.prism_xz(poly, y0, y1, color)
    bm = zb._new_bm()
    rings = []
    for y in (y0, y1):
        for rr in (r, r_in):
            rings.append([bm.verts.new((cx + rr * math.cos(2 * math.pi * i / sides + math.pi / sides), y,
                                        cz + rr * math.sin(2 * math.pi * i / sides + math.pi / sides)))
                          for i in range(sides)])
    o0, i0, o1, i1 = rings
    for i in range(sides):
        j = (i + 1) % sides
        bm.faces.new((o0[i], o0[j], i0[j], i0[i]))
        bm.faces.new((o1[i], i1[i], i1[j], o1[j]))
        bm.faces.new((o0[i], o1[i], o1[j], o0[j]))
        bm.faces.new((i0[i], i0[j], i1[j], i1[i]))
    return pp._finish(bm, color)


def arc_points(x0, x1, z_edge, z_top, n):
    """Points of an elliptic arc from (x0, z_edge) over (mid, z_top) to (x1, z_edge)."""
    cx = (x0 + x1) / 2
    a = (x1 - x0) / 2
    pts = []
    for i in range(n + 1):
        t = math.pi - math.pi * i / n
        pts.append((cx + a * math.cos(t), z_edge + (z_top - z_edge) * math.sin(t)))
    return pts


def wall_boxes(axis, a0, a1, c, t, z0, z1, color, holes=(), side_color=None):
    """A wall slab along `axis` ('x' or 'y') from a0 to a1, centred on the line c (other
    axis), thickness t, from z0 to z1, with rectangular holes [(h0, h1, hz0, hz1)] — built as
    boxes split at the hole edges (no coplanar same-facing overlaps)."""
    cuts = sorted({a0, a1} | {h[0] for h in holes} | {h[1] for h in holes})
    cuts = [x for x in cuts if a0 <= x <= a1]
    parts = []
    for u0, u1 in zip(cuts, cuts[1:]):
        if u1 - u0 < 1e-4:
            continue
        mid = (u0 + u1) / 2
        spans = [(z0, z1)]
        for h0, h1, hz0, hz1 in holes:
            if h0 <= mid <= h1:
                new = []
                for s0, s1 in spans:
                    if hz0 > s0:
                        new.append((s0, min(s1, hz0)))
                    if hz1 < s1:
                        new.append((max(s0, hz1), s1))
                spans = new
        for s0, s1 in spans:
            if s1 - s0 < 1e-4:
                continue
            if axis == "x":
                size, cen = (u1 - u0, t, s1 - s0), (mid, c, (s0 + s1) / 2)
            else:
                size, cen = (t, u1 - u0, s1 - s0), (c, mid, (s0 + s1) / 2)
            parts.append(zb.box(size, cen, color=color, side_color=side_color))
    return parts


# --------------------------------------------------------------------------- asset

class Asset:
    """An asset = a root node (named after the asset) + optional child nodes + empties.
    Parts are given in ASSET coordinates (Blender Z-up, origin on the ground); a child node's
    mesh is stored relative to its pivot."""

    def __init__(self, name):
        self.name = name
        self.nodes = {name: {"pivot": Vector((0, 0, 0)), "parent": None, "groups": []}}
        self.order = [name]
        self.empties = []   # (name, loc, parent)
        self.lights = {}    # empty name -> (radius m, hex) — preview + README only
        self.notes = []

    def node(self, name, pivot=(0, 0, 0), parent=None):
        self.nodes[name] = {"pivot": Vector(pivot), "parent": parent or self.name, "groups": []}
        self.order.append(name)
        return name

    def add(self, parts, mat="palette", node=None, keep_down=None):
        if isinstance(parts, zb.Part):
            parts = [parts]
        if keep_down is None:
            keep_down = mat != "palette"
        self.nodes[node or self.name]["groups"].append((mat, list(parts), keep_down))
        return self

    def empty(self, name, loc, parent=None, light=None):
        self.empties.append((name, Vector(loc), parent or self.name))
        if light:
            self.lights[name] = light
        return self


_ASSETS = {}  # root object name -> Asset (python side only, never exported)


def asset_of(ob):
    while ob.parent is not None:
        ob = ob.parent
    return _ASSETS.get(ob.name)


def build(asset, smooth_angle=zb.SMOOTH_ANGLE_DEG):
    """Create the Blender objects of an asset; returns the root object. Node pivots are in
    asset coordinates; a child's mesh is stored relative to its pivot."""
    minz = float("inf")
    for nd in asset.nodes.values():
        for mat, parts, keep_down in nd["groups"]:
            for p in parts:
                p.bm.normal_update()
                if not keep_down:
                    p.delete_faces(lambda f: f.normal.z < -0.7)
                for v in p.bm.verts:
                    minz = min(minz, v.co.z)
    shift = Vector((0, 0, -minz))

    def wpos(name):  # world position of a node after the ground shift
        return Vector((0, 0, 0)) if name == asset.name else asset.nodes[name]["pivot"] + shift

    objs = {}
    for name in asset.order:
        nd = asset.nodes[name]
        mats = []
        bm = zb._new_bm()
        tmp = bpy.data.meshes.new("_tmp")
        off = shift - wpos(name)
        for mat, parts, _keep in nd["groups"]:
            if mat not in mats:
                mats.append(mat)
            idx = mats.index(mat)
            for p in parts:
                for f in p.bm.faces:
                    f.material_index = idx
                bmesh.ops.translate(p.bm, vec=off, verts=p.bm.verts)
                p.bm.to_mesh(tmp)
                bm.from_mesh(tmp)
                p.bm.free()
        bpy.data.meshes.remove(tmp)
        if len(bm.faces):
            me = bpy.data.meshes.new(name)
            bm.to_mesh(me)
            for m in mats:
                me.materials.append(material(m))
            me.shade_smooth()
            me.set_sharp_from_angle(angle=math.radians(smooth_angle))
            ob = bpy.data.objects.new(name, me)
        else:
            ob = bpy.data.objects.new(name, None)
            ob.empty_display_size = 0.2
        bm.free()
        bpy.context.scene.collection.objects.link(ob)
        if nd["parent"] is not None:
            ob.parent = objs[nd["parent"]]
            ob.location = wpos(name) - wpos(nd["parent"])
        objs[name] = ob
    for name, loc, parent in asset.empties:
        e = bpy.data.objects.new(name, None)
        e.empty_display_type = "SPHERE" if name.startswith("light") else "ARROWS"
        e.empty_display_size = 0.1
        bpy.context.scene.collection.objects.link(e)
        e.parent = objs[parent]
        e.location = loc + shift - wpos(parent)
        objs[name] = e
    root = objs[asset.name]
    _ASSETS[root.name] = asset
    return root


def descendants(ob):
    out = [ob]
    for c in ob.children:
        out += descendants(c)
    return out


def mesh_objs(root):
    return [o for o in descendants(root) if o.type == "MESH"]


def world_verts(root):
    bpy.context.view_layer.update()
    pts = []
    for o in mesh_objs(root):
        m = o.matrix_world
        pts += [m @ v.co for v in o.data.vertices]
    return pts


def tris(root):
    return sum(zb.tri_count(o) for o in mesh_objs(root))


def size(root):
    saved = root.location.copy()
    root.location = (0, 0, 0)
    pts = world_verts(root)
    root.location = saved
    bpy.context.view_layer.update()
    mn = Vector((min(p.x for p in pts), min(p.y for p in pts), min(p.z for p in pts)))
    mx = Vector((max(p.x for p in pts), max(p.y for p in pts), max(p.z for p in pts)))
    return mn, mx


def export(root, path):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    saved = root.location.copy()
    root.location = (0, 0, 0)
    objs = descendants(root)
    for o in bpy.context.scene.objects:
        o.select_set(o in objs)
    bpy.context.view_layer.objects.active = root
    bpy.ops.export_scene.gltf(
        filepath=path, export_format="GLB", use_selection=True, export_yup=True,
        export_apply=True, export_extras=False, export_draco_mesh_compression_enable=False,
        export_vertex_color="NONE", export_all_vertex_colors=False, export_attributes=False,
        export_materials="EXPORT", export_image_format="AUTO", export_unused_images=False,
        export_unused_textures=False, export_texcoords=True, export_normals=True,
        export_tangents=False, export_animations=False, export_skins=False, export_morph=False,
        export_cameras=False, export_lights=False)
    root.location = saved
    for o in objs:
        o.select_set(False)


def describe(root):
    """Node / slot / empty summary in glTF space (x, y up, z = -Blender y)."""
    lines = []
    for o in descendants(root)[1:]:
        loc = o.matrix_world @ Vector((0, 0, 0)) - root.location
        g = (loc.x, loc.z, -loc.y)
        kind = "mesh" if o.type == "MESH" else "empty"
        lines.append(f"    {kind:5s} {o.name:14s} at ({g[0]:.3f}, {g[1]:.3f}, {g[2]:.3f})")
    slots = sorted({m.name for o in mesh_objs(root) for m in o.data.materials})
    lines.append("    slots: " + ", ".join(slots))
    return lines


def report(roots, budgets):
    ok = True
    print(f"{'asset':22s} {'tris':>5s} {'budget':>6s}  size x*y*z (m, glTF Y-up)")
    for r in roots:
        t = tris(r)
        b = budgets.get(r.name, budgets.get("*", 500))
        mn, mx = size(r)
        s = mx - mn
        flag = "" if t <= b else "  OVER BUDGET"
        ok &= t <= b
        print(f"{r.name:22s} {t:5d} {b:6d}  {s.x:.2f} x {s.z:.2f} x {s.y:.2f}{flag}")
        for line in describe(r):
            print(line)
    return ok


# --------------------------------------------------------------------------- preview

def _toon(name, tex=None, flat=None, stops=None):
    """Preview-only cel material: colour x ramp(light). stops = [(pos, rgb multiplier)]."""
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
    els = ramp.color_ramp.elements
    while len(els) < len(stops):
        els.new(0.5)
    for el, (pos, rgb) in zip(els, stops):
        el.position = pos
        el.color = (*rgb, 1)
    mix = nt.nodes.new("ShaderNodeMix")
    mix.data_type = "RGBA"
    mix.blend_type = "MULTIPLY"
    mix.inputs["Factor"].default_value = 1.0
    emit = nt.nodes.new("ShaderNodeEmission")
    nt.links.new(diff.outputs[0], s2r.inputs[0])
    nt.links.new(s2r.outputs["Color"], ramp.inputs["Fac"])
    if tex is not None:
        t = nt.nodes.new("ShaderNodeTexImage")
        t.image = tex
        t.interpolation = "Closest"
        nt.links.new(t.outputs["Color"], mix.inputs["A"])
    else:
        mix.inputs["A"].default_value = (*lin(flat), 1)
    nt.links.new(ramp.outputs["Color"], mix.inputs["B"])
    nt.links.new(mix.outputs["Result"], emit.inputs["Color"])
    nt.links.new(emit.outputs[0], out.inputs["Surface"])
    return mat


def _flat_emit(name, rgb, strength=1.0):
    mat = bpy.data.materials.new(name)
    mat.use_nodes = True
    nt = mat.node_tree
    for n in list(nt.nodes):
        nt.nodes.remove(n)
    out = nt.nodes.new("ShaderNodeOutputMaterial")
    emit = nt.nodes.new("ShaderNodeEmission")
    emit.inputs["Color"].default_value = (*lin(rgb), 1)
    emit.inputs["Strength"].default_value = strength
    nt.links.new(emit.outputs[0], out.inputs["Surface"])
    return mat


DAY_STOPS = [(0.0, (0.64, 0.62, 0.78)), (0.2, (1, 1, 1))]
NIGHT_STOPS = [(0.0, (0.26, 0.31, 0.58)), (0.06, (0.45, 0.53, 0.90)), (0.55, (1.0, 0.86, 0.62))]


def _assign(objs, night):
    img = zb.palette_material().node_tree.nodes["Image Texture"].image
    stops = NIGHT_STOPS if night else DAY_STOPS
    tag = "n" if night else "d"
    cache = {}

    def get(key, make):
        if key not in cache:
            cache[key] = make()
        return cache[key]

    for o in objs:
        if o.type != "MESH":
            continue
        for i, slot in enumerate(o.material_slots):
            src = o.data.materials[i]
            n = src.name.split(".")[0]
            if n == "palette" or (n.endswith("_glow") and not night):
                m = get("pal", lambda: _toon("pv_pal_" + tag, tex=img, stops=stops))
            elif n.endswith("_glow"):
                m = get(n, lambda n=n: _flat_emit("pv_" + n, hex_rgb(GLOW[n])))
            elif n == "glass":
                m = get("glass", lambda: _toon("pv_glass_" + tag, flat=zb.palette_rgb("glass_tint"), stops=stops))
            elif n.endswith("_face"):
                m = get(n, lambda n=n: _toon("pv_" + n + tag, flat=hex_rgb(FACE_COLOR[n]), stops=stops))
            else:
                m = get("pal", lambda: _toon("pv_pal_" + tag, tex=img, stops=stops))
            slot.link = "OBJECT"
            slot.material = m


def _render(out_png, res):
    scene = bpy.context.scene
    scene.render.filepath = out_png
    scene.render.resolution_x, scene.render.resolution_y = res
    bpy.ops.render.render(write_still=True)


def render_preview(roots, out_png, extra=(), res=(1600, 900), yaw_deg=zb.PREVIEW_YAW_DEG,
                   pitch_deg=zb.GAME_PITCH_DEG, labels=True, night=True):
    """Day render (top) + night-tinted render (bottom: moonlight ramp, *_glow slots emissive,
    point lights at the `light*` empties) from the 55 deg game camera."""
    scene = bpy.context.scene
    fwd, right = zb.camera_basis(yaw_deg)
    pitch = math.radians(pitch_deg)
    view_dir = (fwd * math.cos(pitch) + Vector((0, 0, -math.sin(pitch)))).normalized()
    cam_up = right.cross(view_dir).normalized()
    allroots = list(roots) + list(extra)
    objs = [o for r in allroots for o in descendants(r)]
    bpy.context.view_layer.update()

    lbl_mat = _toon("pv_label", flat=(0.29, 0.16, 0.13), stops=DAY_STOPS)
    lbls = []
    if labels:
        for r in roots:
            pts = world_verts(r)
            near = min(p.dot(fwd) for p in pts)
            cx = sum(p.x for p in pts) / len(pts)
            cy = sum(p.y for p in pts) / len(pts)
            c = Vector((cx, cy, 0))
            loc = c + fwd * (near - c.dot(fwd) - 0.35) + Vector((0, 0, 0.005))
            lb = zb.add_label(r.name, loc, yaw_deg, size=0.22)
            lb.data.materials.append(lbl_mat)
            lbls.append(lb)

    pts = []
    for o in objs:
        if o.type == "MESH":
            pts += [o.matrix_world @ v.co for v in o.data.vertices]
    for lb in lbls:
        pts += [lb.matrix_world @ Vector(c) for c in lb.bound_box]
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
    cam_data.type = "ORTHO"
    cam_data.ortho_scale = max(w, h * aspect) * 1.06
    cam_data.clip_end = 500
    cam.location = target - view_dir * 80
    cam.rotation_euler = view_dir.to_track_quat("-Z", "Y").to_euler()
    scene.camera = cam

    bpy.ops.mesh.primitive_plane_add(size=600, location=(target.x, target.y, -0.001))
    ground = bpy.context.active_object
    ground.name = "preview_ground"
    g_day = _toon("pv_ground_d", flat=(0.90, 0.90, 0.91), stops=[(0.0, (0.80, 0.78, 0.88)), (0.2, (1, 1, 1))])
    g_night = _toon("pv_ground_n", flat=(0.80, 0.80, 0.80), stops=NIGHT_STOPS)
    ground.data.materials.append(g_day)

    sun_data = bpy.data.lights.new("preview_sun", "SUN")
    sun_data.energy = 3.0
    sun_data.angle = math.radians(0.5)
    sun = bpy.data.objects.new("preview_sun", sun_data)
    scene.collection.objects.link(sun)
    light_dir = (right * 1.0 - fwd * 0.3 + Vector((0, 0, -1.1))).normalized()
    sun.rotation_euler = light_dir.to_track_quat("-Z", "Y").to_euler()

    world = bpy.data.worlds.new("preview_world")
    world.use_nodes = True
    world.node_tree.nodes["Background"].inputs["Color"].default_value = (0, 0, 0, 1)
    scene.world = world
    scene.render.engine = "BLENDER_EEVEE"
    for attr, val in (("taa_render_samples", 16), ("use_shadows", True)):
        if hasattr(scene.eevee, attr):
            setattr(scene.eevee, attr, val)
    scene.render.resolution_percentage = 100
    scene.view_settings.view_transform = "Standard"
    scene.view_settings.look = "None"
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
    ls.select_border = False
    ls.select_crease = True
    if ls.linestyle is None:
        ls.linestyle = bpy.data.linestyles.new("outline")
    ls.linestyle.color = tuple(zb.srgb_to_linear(c) for c in zb.palette_rgb("outline"))
    ls.linestyle.thickness = 2.0
    scene.render.image_settings.file_format = "PNG"

    os.makedirs(os.path.dirname(out_png), exist_ok=True)
    tmp_day = out_png[:-4] + "_tmp_day.png"
    tmp_night = out_png[:-4] + "_tmp_night.png"
    _assign(objs, night=False)
    day_hidden = [o for o in objs if o.name.endswith("night_sky")]  # night-only nodes
    for o in day_hidden:
        o.hide_render = True
    _render(tmp_day, res)
    for o in day_hidden:
        o.hide_render = False
    if not night:
        os.replace(tmp_day, out_png)
        return out_png

    # night: weak moonlight, lamp point lights with a hard cut-off radius
    _assign(objs, night=True)
    ground.material_slots[0].material = g_night
    sun_data.energy = 0.35
    for o in objs:
        if o.type != "EMPTY":
            continue
        a = asset_of(o)
        key = None
        if a is not None:
            key = next((k for k in a.lights if o.name == k or o.name.endswith("_" + k)), None)
        if key is None:
            continue
        radius = a.lights[key][0]
        ld = bpy.data.lights.new("pv_" + o.name, "POINT")
        ld.energy = 60.0 * radius * radius
        ld.color = hex_rgb("#FFC46E")
        ld.shadow_soft_size = 0.05
        ld.use_shadow = False
        if hasattr(ld, "use_custom_distance"):
            ld.use_custom_distance = True
            ld.cutoff_distance = radius
        lo = bpy.data.objects.new("pv_" + o.name, ld)
        scene.collection.objects.link(lo)
        lo.location = o.matrix_world.translation
    _render(tmp_night, res)

    a = bpy.data.images.load(tmp_day)
    b = bpy.data.images.load(tmp_night)
    W, H = res
    out = bpy.data.images.new("preview_out", W, 2 * H, alpha=False)
    pa = list(a.pixels[:])
    pb = list(b.pixels[:])
    out.pixels[:] = pb + pa  # Blender images are bottom-up: night at the bottom
    out.filepath_raw = out_png
    out.file_format = "PNG"
    out.save()
    os.remove(tmp_day)
    os.remove(tmp_night)
    return out_png


# --------------------------------------------------------------------------- runner

KIT_PREFIX = "focus_"


def run(kit, builders, cols, spacing, folder="props", budgets=None, extra=None, preview_dir=None,
        variants=None):
    """Build -> report -> export -> save .blend -> preview (day + night).
    builders: {name: fn() -> Asset}. folder: assets/models/<folder>/.
    extra(roots) -> preview-only objects. variants: {name: fn() -> Asset} exported but
    not shown in the preview grid."""
    args = zb.script_args()
    only = [a for a in args if not a.startswith("--")]
    reset()
    roots = {}
    ok = True
    budgets = budgets or {"*": 500}
    todo = list(builders.items()) + list((variants or {}).items())
    for name, fn in todo:
        if only and name not in only:
            continue
        r = build(fn())
        ok &= report([r], budgets)
        export(r, zb.repo_path("assets", "models", folder, r.name + ".glb"))
        # unique Blender names for the next asset (glTF node names come from the export above)
        for o in descendants(r)[1:]:
            o.name = r.name + "_" + o.name
            if o.type == "MESH":
                o.data.name = o.name
        roots[name] = r
    grid = [r for n, r in roots.items() if n in builders]
    hidden = [r for n, r in roots.items() if n not in builders]
    zb.layout_grid(grid, cols=cols, spacing=spacing)
    for i, r in enumerate(hidden):
        r.location = (0, -1000 - 50 * i, 0)
    if not only:
        zb.save_blend(zb.repo_path("assets", "blender", folder, kit + ".blend"))
    if "--no-preview" not in args:
        for r in hidden:
            for o in descendants(r):
                bpy.data.objects.remove(o, do_unlink=True)
        ex = extra(roots) if extra and not only else []
        out = zb.repo_path("art", "props", preview_dir or kit, "model_preview.png")
        if only:  # focused preview of a few assets: never overwrite the kit preview
            out = os.path.join(os.environ.get("PREVIEW_DIR", os.path.dirname(out)),
                               KIT_PREFIX + kit + "_" + "_".join(only) + ".png")
        render_preview(grid, out, extra=ex)
    if not ok:
        sys.exit(1)
    return roots


def instance_tree(root, name, loc=(0, 0, 0), rot_deg=0.0, node_rot=None):
    """Preview-only copy of an asset (shares mesh data). node_rot = {node: deg about Z}."""
    def copy(o, parent):
        c = bpy.data.objects.new(name + "_" + o.name if parent else name, o.data)
        bpy.context.scene.collection.objects.link(c)
        if parent is not None:
            c.parent = parent
            c.location = o.location
            for k, deg in (node_rot or {}).items():
                if o.name == k or o.name.endswith("_" + k):
                    c.rotation_euler = (0, 0, math.radians(deg))
        for ch in o.children:
            copy(ch, c)
        return c
    top = copy(root, None)
    top.location = loc
    top.rotation_euler = (0, 0, math.radians(rot_deg))
    if root.name in _ASSETS:
        _ASSETS[top.name] = _ASSETS[root.name]
    return top
