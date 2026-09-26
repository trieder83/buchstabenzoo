"""Review render of the ambient water animals (GAME-AMBIENT): duck, duckling, frog.

Run (after duck.py, duckling.py, frog.py):
    blender -b --factory-startup --python tools/blender/animals/ambient_preview.py

Appends the rigs + meshes + clips from assets/blender/animals/{duck,duckling,frog}.blend
(never modifies them), places posed instances (NLA strips, one clip frame each) and writes
art/props/kit_water/ambient_preview.png:
  row 1  55 deg game camera over water: mother duck swimming with three ducklings in a line,
         a duck dipping (tail up), one flapping, one preening; frogs on lily pads (idle,
         croaking, mid-hop)
  row 2  close-ups (duck, duckling, frog, frog croaking) + the river group at in-game size
Water plane = the water surface (z = 0 = the ducks' origin); lily pads at z = 0.
"""

import math
import os
import shutil
import sys
import tempfile

import bpy
import numpy as np
from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rig_base as rb  # noqa: E402

zb, qr, hr = rb.zb, rb.qr, rb.hr
OUT = zb.repo_path("art", "props", "kit_water", "ambient_preview.png")
SCENE_FRAME = 200
PAD_Z = 0.012


def append(asset):
    path = zb.repo_path("assets", "blender", "animals", f"{asset}.blend")
    with bpy.data.libraries.load(path, link=False) as (src, dst):
        dst.objects = [n for n in src.objects if n in (asset, f"{asset}_rig")]
        dst.actions = list(src.actions)
    objs = {o.name: o for o in dst.objects}
    arm, mesh = objs[f"{asset}_rig"], objs[asset]
    for a in dst.actions:  # appended names may clash between assets: prefix them
        a.name = f"{asset}:{a.name}"
    img = next(n for n in mesh.data.materials[0].node_tree.nodes if n.type == "TEX_IMAGE").image
    return arm, mesh, img


class Kind:
    def __init__(self, asset):
        self.asset = asset
        self.arm, self.mesh, self.img = append(asset)
        self.mat = None
        self.n = 0


def instance(kind, clip, frame, loc, yaw=0.0):
    """Posed copy: an armature object with one NLA strip placed so that SCENE_FRAME shows
    `frame` of `clip`, and a mesh copy deformed by it."""
    kind.n += 1
    arm = kind.arm.copy()
    arm.name = f"{kind.asset}_i{kind.n}"
    bpy.context.scene.collection.objects.link(arm)
    ad = arm.animation_data or arm.animation_data_create()
    for t in list(ad.nla_tracks):
        ad.nla_tracks.remove(t)
    ad.action = None
    act = bpy.data.actions[f"{kind.asset}:{clip}"]
    tr = ad.nla_tracks.new()
    st = tr.strips.new(clip, int(SCENE_FRAME - frame), act)
    st.extrapolation = "HOLD"
    arm.location = loc
    arm.rotation_euler = (0, 0, math.radians(yaw))
    me = kind.mesh.copy()
    me.name = f"{kind.asset}_m{kind.n}"
    bpy.context.scene.collection.objects.link(me)
    me.parent = arm
    me.matrix_parent_inverse.identity()
    me.modifiers["Armature"].object = arm
    me.hide_render = False
    me.material_slots[0].link = "OBJECT"
    me.material_slots[0].material = kind.mat
    return arm


def pad(loc, r=0.2, rot=0.0):
    """Preview lily pad: flat disc with the typical wedge notch, top at PAD_Z."""
    import bmesh
    bm = bmesh.new()
    n = 16
    ring = [bm.verts.new((0.0, 0.0, PAD_Z))]
    for i in range(n + 1):
        a = rot + math.radians(20) + (math.tau - math.radians(40)) * i / n
        ring.append(bm.verts.new((r * math.cos(a), r * math.sin(a), PAD_Z)))
    face = bm.faces.new(ring)
    ext = bmesh.ops.extrude_face_region(bm, geom=[face])
    down = [e for e in ext["geom"] if isinstance(e, bmesh.types.BMVert)]
    bmesh.ops.translate(bm, verts=down, vec=(0, 0, -0.02))
    bmesh.ops.recalc_face_normals(bm, faces=bm.faces[:])
    me = bpy.data.meshes.new("pad")
    bm.to_mesh(me)
    bm.free()
    o = bpy.data.objects.new("pad", me)
    bpy.context.scene.collection.objects.link(o)
    o.location = (loc[0], loc[1], 0)
    me.materials.append(bpy.data.materials["pv_pad"])
    return o


def compose(rows, out_png, gap=6):
    arrs = []
    width = max(sum(p[1] for p in row) + gap * (len(row) - 1) for row in rows)
    for row in rows:
        parts = []
        for path, w, h, up in row:
            img = bpy.data.images.load(path)
            iw, ih = img.size
            a = np.array(img.pixels[:], np.float32).reshape(ih, iw, 4)
            bpy.data.images.remove(img)
            if up > 1:
                a = a.repeat(up, axis=0).repeat(up, axis=1)
            parts += [a, np.ones((a.shape[0], gap, 4), np.float32)]
        r = np.concatenate(parts[:-1], axis=1)
        if r.shape[1] < width:
            r = np.concatenate([r, np.ones((r.shape[0], width - r.shape[1], 4), np.float32)], axis=1)
        arrs += [r, np.ones((gap, width, 4), np.float32)]
    full = np.concatenate(arrs[:-1][::-1], axis=0)  # Blender images are bottom-up
    h, w, _ = full.shape
    img = bpy.data.images.new("amb_out", w, h, alpha=True)
    img.pixels.foreach_set(full.ravel())
    os.makedirs(os.path.dirname(out_png), exist_ok=True)
    img.filepath_raw = out_png
    img.file_format = "PNG"
    img.save()
    print(f"preview -> {out_png}")


def main():
    zb.clean_scene()
    scene = bpy.context.scene
    scene.render.fps = 30
    kinds = {a: Kind(a) for a in ("duck", "duckling", "frog")}
    # preview lighting / outlines / camera (qr.setup_preview needs one mesh: the original
    # duck mesh, which is then hidden)
    first = kinds["duck"].mesh
    scene.collection.objects.link(first)
    ls = qr.setup_preview(first, kinds["duck"].img)
    first.hide_render = True
    for k in kinds.values():
        k.mat = zb._toon_material(f"pv_{k.asset}", tex_image=k.img)
        next(n for n in k.mat.node_tree.nodes if n.type == "TEX_IMAGE").interpolation = "Linear"
    zb._toon_material("pv_pad", flat_rgb=(0.40, 0.68, 0.30))
    D, DL, F = kinds["duck"], kinds["duckling"], kinds["frog"]

    # ---- river group (game view); the ducks swim towards -Y rotated by yaw
    instance(D, "swim", 6, (0.0, 0.0, 0), yaw=-20)
    for i, (x, y, fr) in enumerate(((0.12, 0.42, 12), (0.05, 0.72, 20), (0.16, 1.00, 3))):
        instance(DL, "swim", fr, (x, y, 0), yaw=-20 + 8 * (i % 2))
    instance(D, "dip", 28, (1.0, -0.3, 0), yaw=40)
    instance(D, "flap", 11, (-0.95, 0.35, 0), yaw=25)
    instance(D, "preen", 22, (0.95, 0.85, 0), yaw=150)
    instance(DL, "idle", 30, (1.35, 0.35, 0), yaw=120)
    # ---- pond corner: frogs on lily pads
    for p, rot in (((-0.9, -0.95), 0.5), ((-0.25, -1.25), 2.0), ((-1.35, -0.35), 4.0)):
        pad(p, rot=rot)
    instance(F, "idle", 0, (-0.9, -0.95, PAD_Z), yaw=-30)
    instance(F, "croak", 11, (-0.25, -1.25, PAD_Z), yaw=20)
    instance(F, "hop", 15, (-1.15, -0.62, PAD_Z), yaw=-143)  # mid-air, pad 1 -> pad 3

    # ---- close-up set far away (x = 20)
    cx = 20.0
    instance(D, "idle", 0, (cx, 0, 0), yaw=0)
    instance(DL, "idle", 0, (cx + 3, 0, 0), yaw=0)
    pad((cx + 6, 0), rot=1.0)
    pad((cx + 9, 0), rot=1.0)
    instance(F, "idle", 0, (cx + 6, 0, PAD_Z), yaw=0)
    instance(F, "croak", 11, (cx + 9, 0, PAD_Z), yaw=0)

    tmp = tempfile.mkdtemp(prefix="amb_")
    scene.frame_set(SCENE_FRAME)
    bpy.context.view_layer.update()
    water = rb.WATER

    def shot(name, res, tgt, d, scale, thick):
        ls.linestyle.thickness = thick
        loc, dd = qr._look(tgt, d)
        return hr._render(os.path.join(tmp, name), res, loc, dd, scale, water)

    gv = qr.game_view_dir(-25.0)
    row1 = [(shot("game.png", (2000, 900), (0.0, -0.1, 0.05), gv, 4.4, 2.5), 2000, 900, 1)]
    q = Vector((0.55, 0.75, -0.42))
    row2 = [
        (shot("c1.png", (390, 390), (cx, -0.03, 0.16), q, 0.62, 2.5), 390, 390, 1),
        (shot("c2.png", (390, 390), (cx + 3, -0.02, 0.10), q, 0.34, 2.5), 390, 390, 1),
        (shot("c3.png", (390, 390), (cx + 6, 0, 0.08), q, 0.34, 2.5), 390, 390, 1),
        (shot("c4.png", (390, 390), (cx + 9, 0, 0.08), q, 0.34, 2.5), 390, 390, 1),
        # in-game size: ~ the zoom of the follow camera (about 12 m across 1000 px)
        (shot("g.png", (130, 130), (0.1, 0.45, 0.05), gv, 1.6, 1.0), 390, 390, 3),
    ]
    compose([row1, row2], OUT)
    shutil.rmtree(tmp, ignore_errors=True)


main()
