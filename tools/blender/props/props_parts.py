"""Extra low-poly parts and the kit runner shared by kit_signs, kit_nature, kit_water and
kit_barriers (Kits 3-6). Builds on tools/blender/lib/zoo_blender.py (import only).

Blender Z-up while modelling. World axes (GAME-LAYOUT "Coordinate spaces", Q-056): east =
+X = Blender +X, up = +Y = Blender +Z, north = world -Z = Blender +Y; glTF = Blender
(x, z, -y). "Front" of a prop = the side the default follow camera sees (camera south of
the player, looking north) = south = world +Z = Blender -Y.
"""

import math
import os
import random
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "lib"))
import bmesh  # noqa: E402
import bpy  # noqa: E402
from mathutils import Matrix, Vector  # noqa: E402

import zoo_blender as zb  # noqa: E402

FRONT = -1.0  # Blender -Y = south (world +Z) = towards the default camera
BACK = 1.0    # Blender +Y = north (world -Z)


def _finish(bm, color):
    bm.normal_update()
    bmesh.ops.recalc_face_normals(bm, faces=bm.faces)
    zb._color_bm(bm, color)
    return zb.Part(bm)


def recolor(part, pred, color):
    """Give faces for which pred(face) is true another palette colour."""
    uvl = part.bm.loops.layers.uv.active
    part.bm.normal_update()
    uv = zb.palette_uv(color)
    for f in part.bm.faces:
        if pred(f):
            for loop in f.loops:
                loop[uvl].uv = uv
    return part


def cyl(r0, z0, z1, sides=8, r1=None, center=(0.0, 0.0), color="wood", top=True, phase=None,
        apex=False):
    """Vertical frustum (cylinder when r1 is None, cone when apex) around `center`."""
    r1 = r0 if r1 is None else r1
    phase = math.pi / sides if phase is None else phase
    bm = zb._new_bm()
    cx, cy = center
    ring0 = [bm.verts.new((cx + r0 * math.cos(phase + 2 * math.pi * i / sides),
                           cy + r0 * math.sin(phase + 2 * math.pi * i / sides), z0)) for i in range(sides)]
    if apex:
        tip = bm.verts.new((cx, cy, z1))
        for i in range(sides):
            bm.faces.new((ring0[i], ring0[(i + 1) % sides], tip))
    else:
        ring1 = [bm.verts.new((cx + r1 * math.cos(phase + 2 * math.pi * i / sides),
                               cy + r1 * math.sin(phase + 2 * math.pi * i / sides), z1)) for i in range(sides)]
        for i in range(sides):
            j = (i + 1) % sides
            bm.faces.new((ring0[i], ring0[j], ring1[j], ring1[i]))
        if top:
            bm.faces.new(ring1)
    return _finish(bm, color)


def beam(p0, p1, w, h=None, color="wood", sides=4, taper=1.0, caps=True):
    """A stick from p0 to p1 (3D points): square (sides=4) or n-gon cross-section, width w.
    taper scales the end at p1. Ends are capped."""
    h = w if h is None else h
    p0, p1 = Vector(p0), Vector(p1)
    d = p1 - p0
    L = d.length
    bm = zb._new_bm()
    rings = []
    for k, (z, s) in enumerate(((0.0, 1.0), (L, taper))):
        ring = []
        for i in range(sides):
            a = math.pi / sides + 2 * math.pi * i / sides
            ring.append(bm.verts.new((w / 2 * s * math.cos(a) * math.sqrt(2) if sides == 4 else w / 2 * s * math.cos(a),
                                      h / 2 * s * math.sin(a) * math.sqrt(2) if sides == 4 else h / 2 * s * math.sin(a),
                                      z)))
        rings.append(ring)
    for i in range(sides):
        j = (i + 1) % sides
        bm.faces.new((rings[0][i], rings[0][j], rings[1][j], rings[1][i]))
    if caps:
        bm.faces.new(list(reversed(rings[0])))
        bm.faces.new(rings[1])
    rot = Vector((0, 0, 1)).rotation_difference(d.normalized()).to_matrix().to_4x4()
    bmesh.ops.transform(bm, matrix=Matrix.Translation(p0) @ rot, verts=bm.verts)
    return _finish(bm, color)


def blob(center, radii, color, subdiv=1, seed=0, amp=0.12, flat_bottom=None):
    """Lumpy icosphere (tree crowns, bushes, rocks). amp = relative radial jitter.
    flat_bottom: z below which vertices are clamped (flat base resting on the ground)."""
    rnd = random.Random(seed)
    bm = zb._new_bm()
    bmesh.ops.create_icosphere(bm, subdivisions=subdiv, radius=1.0)
    for v in bm.verts:
        k = 1.0 + rnd.uniform(-amp, amp)
        v.co = Vector((v.co.x * radii[0] * k, v.co.y * radii[1] * k, v.co.z * radii[2] * k)) + Vector(center)
        if flat_bottom is not None and v.co.z < flat_bottom:
            v.co.z = flat_bottom
    return _finish(bm, color)


def prism_xz(poly, y0, y1, color, front_color=None):
    """Polygon in the X/Z plane [(x, z)], extruded along Blender Y from y0 to y1.
    front_color: colour of the south face (-Y, turned to the default camera)."""
    bm = zb._new_bm()
    a = [bm.verts.new((x, y0, z)) for x, z in poly]
    b = [bm.verts.new((x, y1, z)) for x, z in poly]
    n = len(poly)
    bm.faces.new(a)
    bm.faces.new(b)
    for i in range(n):
        j = (i + 1) % n
        bm.faces.new((a[i], a[j], b[j], b[i]))
    p = _finish(bm, color)
    if front_color:
        recolor(p, lambda f: f.normal.y * FRONT > 0.9, front_color)
    return p


def flat(points, color):
    """A single flat polygon (3D points, CCW seen from the side it should face)."""
    bm = zb._new_bm()
    bm.faces.new([bm.verts.new(p) for p in points])
    bm.normal_update()
    zb._color_bm(bm, color)
    return zb.Part(bm)


def blade(base, tip, width, color, bend=0.0):
    """A leaf/grass blade: thin 3-sided spike from base to tip (3 tris, no base cap)."""
    bm = zb._new_bm()
    bx, by, bz = base
    d = Vector(tip) - Vector(base)
    side = Vector((-d.y, d.x, 0))
    if side.length < 1e-6:
        side = Vector((1, 0, 0))
    side.normalize()
    fwd = side.cross(Vector((0, 0, 1)))
    ring = [bm.verts.new(Vector(base) + side * width / 2),
            bm.verts.new(Vector(base) - side * width / 2),
            bm.verts.new(Vector(base) + fwd * width * 0.45)]
    t = bm.verts.new(Vector(tip) + Vector((0, 0, 0)) + d.normalized() * bend)
    for i in range(3):
        bm.faces.new((ring[i], ring[(i + 1) % 3], t))
    return _finish(bm, color)


def leaf(base, direction, length, width, color, droop=0.0):
    """Flat upward-facing leaf (2 tris, diamond) — single-sided, visible from above."""
    b = Vector(base)
    d = Vector(direction).normalized()
    s = Vector((-d.y, d.x, 0)).normalized() if abs(d.z) < 0.99 else Vector((1, 0, 0))
    mid = b + d * length * 0.45 + Vector((0, 0, -droop * 0.3))
    tip = b + d * length + Vector((0, 0, -droop))
    pts = [b, mid - s * width / 2, tip, mid + s * width / 2]
    p = flat([tuple(v) for v in pts], color)
    p.bm.normal_update()
    p.bm.faces.ensure_lookup_table()
    if p.bm.faces[0].normal.z < 0:
        bmesh.ops.reverse_faces(p.bm, faces=p.bm.faces)
    return p


def star(center, r_out, r_in, points, color, z_up=True, phase=0.0):
    """Flat star/flower polygon facing up."""
    cx, cy, cz = center
    pts = []
    for i in range(points * 2):
        r = r_out if i % 2 == 0 else r_in
        a = phase + math.pi * i / points
        pts.append((cx + r * math.cos(a), cy + r * math.sin(a), cz))
    return flat(pts, color)


def disc(center, r, color, sides=8, rx=None):
    cx, cy, cz = center
    rx = r if rx is None else rx
    return flat([(cx + rx * math.cos(2 * math.pi * i / sides), cy + r * math.sin(2 * math.pi * i / sides), cz)
                 for i in range(sides)], color)


# --------------------------------------------------------------------------- runner

def instance(src, name, loc=(0, 0, 0), rot_deg=0.0):
    o = bpy.data.objects.new(name, src.data)
    o.location = loc
    o.rotation_euler = (0, 0, math.radians(rot_deg))
    bpy.context.scene.collection.objects.link(o)
    return o


def panel_report(obj, colors):
    """Print the world-space (glTF) rectangle of every face coloured with one of `colors`
    (overlay panels): centre, size and normal — for the renderer's text overlay."""
    me = obj.data
    uvl = me.uv_layers.active.data
    want = {c: zb.palette_uv(c) for c in colors}
    out = {}
    for poly in me.polygons:
        uv = uvl[poly.loop_start].uv
        for c, u in want.items():
            if abs(uv[0] - u[0]) < 1e-4 and abs(uv[1] - u[1]) < 1e-4 and poly.normal.z > -0.1:
                out.setdefault(c, []).append(poly)
    for c, polys in out.items():
        best = max(polys, key=lambda p: p.area)
        n = best.normal
        ce = best.center
        print(f"  overlay {obj.name}.{c}: centre world ({ce.x:.3f}, {ce.z:.3f}, {-ce.y:.3f}) "
              f"normal world ({n.x:.3f}, {n.z:.3f}, {-n.y:.3f}) area {best.area:.3f} m2")


def run_kit(kit, builders, cols, spacing, extra=None, overlays=()):
    """Build, report, export, save and preview a kit (same flow as kit_ground/kit_fences).
    extra(objs) -> list of preview-only objects (sample assemblies)."""
    args = zb.script_args()
    zb.clean_scene()
    objs = {name: zb.build_object(name, b()) for name, b in builders.items()}
    ok = zb.report(objs.values())
    for o in objs.values():
        if overlays:
            panel_report(o, overlays)
        zb.export_glb(o, zb.repo_path("assets", "models", "props", o.name + ".glb"))
    order = list(objs.values())
    zb.layout_grid(order, cols=cols, spacing=spacing)
    zb.save_blend(zb.repo_path("assets", "blender", "props", kit + ".blend"))
    if "--no-preview" not in args:
        extra_objs = extra(objs) if extra else []
        zb.render_preview(order, zb.repo_path("art", "props", kit, "model_preview.png"), extra_objs=extra_objs)
    if not ok:
        sys.exit(1)
    return objs
