"""Shared kit for the night animals (GAME-NIGHT rule 6, ART-ANIMALS "Night animals").

- `GKit`: quad_kit.Kit for any skeleton (bat / owl rigs built on rig_base.Skeleton): tubes,
  painted regions, dome eyes with the `eye_glow` slot, flat ears.
- `cone()`: spine / quill tufts (tube ring + tip pole).
- `frame_rot()`: rotation that maps one (direction, normal) frame onto another (wing spread).
- `run()`: gate, build, atlas, bake, .blend, .glb, preview for quadruped and other rigs.
- `render_preview()`: 55 deg game view | front | side | in-game size | **night panel** (blue
  night light, the `eye_glow` slot emissive #E6F7A0 as inside the lantern radius).

Eyeshine (art/night/README.md, NIGHT-006): the front cap of every eye dome (pupil, inner
iris and both painted highlights) is a separate material `eye_glow` that samples the same
body atlas, so by day it looks exactly like `body`.
"""

import math
import os
import shutil
import sys
import tempfile

import bpy
from mathutils import Matrix, Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import quad_kit as qk  # noqa: E402
import rig_base as rb  # noqa: E402

qr = qk.qr
hr = qr.hr
zb = qr.zb
V = qk.V3
X, Y, Z = qk.X, qk.Y, qk.Z
FWD = qr.FWD
Tube = qk.Tube
ellipsoid = qk.ellipsoid
TAU = qk.TAU
TRI_BUDGET = 3000

EYE_GLOW = "#E6F7A0"
NIGHT_GROUND = (0.20, 0.26, 0.52)
NIGHT_LIT = (0.50, 0.58, 1.00)       # multiplies the atlas (blue moonlight)
NIGHT_SHADOW = (0.32, 0.38, 0.78)


class GKit(qk.Kit):
    """quad_kit.Kit on a generic skeleton (rig_base.Skeleton subclass)."""

    def __init__(self, asset, colors, rig, regions=None):
        self.asset = asset
        self.P = getattr(rig, "P", None)
        self.rig = rig
        self.pal = qk.Palette(colors)
        self.atlas = qr.Atlas(256, colors, regions or qk.DEFAULT_REGIONS)
        self.mb = rb.MeshBuilder(rig)
        self.painters = {}
        self.tubes = {}


def eye_at(k, S, f, ex, r, depth, bone="head", glow=True):
    """One dome eye at surface point S with outward normal f; ex = in-plane direction of the
    swatch's +U (pupil 'look' side). Same geometry / UVs / eye_glow cap as Kit.eyes."""
    at = k.atlas
    f = V(f).normalized()
    ex = (V(ex) - f * f.dot(V(ex))).normalized()
    ey = f.cross(ex).normalized()
    if ey.dot(Z) < 0:
        ey = -ey
    c = V(S) - f * (0.1 * depth)
    rings = [qr.ring(c + f * off, ey, ex, r[1] * sc, r[0] * sc, 12)
             for off, sc in ((-0.9 * depth, 0.72), (-0.35 * depth, 1.0), (0.3 * depth, 0.86),
                             (0.65 * depth, 0.52))]

    def uv(i, kk, co):
        dd = co - c
        return at.map_uv("eye", 0.5 + 0.5 * dd.dot(ex) / r[0], 0.5 + 0.5 * dd.dot(ey) / r[1])

    _, faces = k.mb.loft(rings, uv, qr.rigid(bone), pole_start=c - f * (1.3 * depth),
                         pole_end=c + f * (0.9 * depth))
    if glow:
        k.mb.mark_glow(faces[-12:])


def cone(base, direction, r, length, n=6, up=None, squash=1.0):
    """Tuft cone from `base` along `direction` (base ring slightly rounded, tip pole)."""
    d = V(direction).normalized()
    b = V(base)
    u = up if up is not None else (Z if abs(d.dot(Z)) < 0.9 else Y)
    pts = [b, b + d * (0.35 * length)]
    radii = [(r, r * squash, r * squash), (0.72 * r, 0.72 * r * squash, 0.72 * r * squash)]
    return Tube(pts, radii, n, up=u, cap=(None, 0.65 * length))


def frame_rot(d0, n0, d1, n1):
    """Quaternion R with R @ d0 = d1 and R @ n0 = n1 (both frames orthonormalised)."""
    def basis(d, n):
        d = V(d).normalized()
        n = (V(n) - d * d.dot(V(n))).normalized()
        return Matrix((d, n, d.cross(n))).transposed()
    return (basis(d1, n1) @ basis(d0, n0).transposed()).to_quaternion()


# --------------------------------------------------------------------------- preview

def _night_materials(mesh_obj, img):
    body = zb._toon_material("pv_body_night", tex_image=img, shadow=NIGHT_SHADOW)
    ramp = next(n for n in body.node_tree.nodes if n.type == "VALTORGB")
    ramp.color_ramp.elements[1].color = (*NIGHT_LIT, 1)
    next(n for n in body.node_tree.nodes if n.type == "TEX_IMAGE").interpolation = "Linear"
    glow = bpy.data.materials.new("pv_eye_glow_night")
    glow.use_nodes = True
    nt = glow.node_tree
    for n in list(nt.nodes):
        nt.nodes.remove(n)
    out = nt.nodes.new("ShaderNodeOutputMaterial")
    tex = nt.nodes.new("ShaderNodeTexImage")
    tex.image = img
    tex.interpolation = "Linear"
    # emissive #E6F7A0 x (0.25 + 0.75 luminance): highlights shine, the pupil stays dark
    bw = nt.nodes.new("ShaderNodeRGBToBW")
    mr = nt.nodes.new("ShaderNodeMapRange")
    mr.inputs["To Min"].default_value = 0.25
    mul = nt.nodes.new("ShaderNodeMix")
    mul.data_type = "RGBA"
    mul.blend_type = "MULTIPLY"
    mul.inputs["Factor"].default_value = 1.0
    gc = tuple(zb.srgb_to_linear(c) for c in hr.hex_rgb(EYE_GLOW))
    col_in = lambda node, nm: next(x for x in node.inputs if x.name == nm and x.type == "RGBA")  # noqa: E731
    col_in(mul, "B").default_value = (*gc, 1)
    emit = nt.nodes.new("ShaderNodeEmission")
    emit.inputs["Strength"].default_value = 1.4
    nt.links.new(tex.outputs["Color"], bw.inputs[0])
    nt.links.new(bw.outputs[0], mr.inputs["Value"])
    nt.links.new(mr.outputs[0], col_in(mul, "A"))
    nt.links.new(mul.outputs["Result"], emit.inputs["Color"])
    nt.links.new(emit.outputs[0], out.inputs["Surface"])
    day = [s.material for s in mesh_obj.material_slots]
    night = [body] + [glow] * (len(day) - 1)
    return day, night


def render_preview(arm_obj, mesh_obj, img, out_png, ls, loco, center_z, height_m,
                   game_yaw=-60.0, big_scale=2.0, close_scale=1.6, small_px=64,
                   night_pose=("idle", 30), night_scale=None, side_dir=(-1, 0, -0.06),
                   loco_z=None):
    """55 deg game camera (loco = (clip, frame)) | front (rest) | left side (rest) | game
    camera at in-game size | night: 55 deg game view, blue night light, eye glow on."""
    tmp = tempfile.mkdtemp(prefix="pv_")
    grass, grey = rb.GRASS, rb.GREY
    gv = qr.game_view_dir(game_yaw)
    tgt = (0, 0, center_z)
    ltgt = (0, 0, center_z if loco_z is None else loco_z)
    panels = []
    ls.linestyle.thickness = 3.0
    hr._pose_at(arm_obj, *loco)
    loc, d = qr._look(ltgt, gv)
    panels.append(hr._render(os.path.join(tmp, "a.png"), (760, 900), loc, d, big_scale, grass))
    hr._pose_at(arm_obj, None, 0)
    loc, d = qr._look(tgt, (0, 1, -0.10))
    panels.append(hr._render(os.path.join(tmp, "b.png"), (620, 900), loc, d, close_scale, grey))
    loc, d = qr._look(tgt, side_dir)
    panels.append(hr._render(os.path.join(tmp, "c.png"), (860, 900), loc, d, close_scale, grey))
    ls.linestyle.thickness = 1.0
    hr._pose_at(arm_obj, *loco)
    res = 150
    loc, d = qr._look(ltgt, gv)
    panels.append(hr._render(os.path.join(tmp, "d.png"), (res, res), loc, d,
                             height_m * res / small_px, grass))
    # night panel
    day, night = _night_materials(mesh_obj, img)
    for s, m in zip(mesh_obj.material_slots, night):
        s.material = m
    ls.linestyle.thickness = 3.0
    hr._pose_at(arm_obj, *night_pose)
    loc, d = qr._look(tgt, qr.game_view_dir(game_yaw + 40))
    panels.append(hr._render(os.path.join(tmp, "e.png"), (760, 900), loc, d,
                             night_scale or big_scale, NIGHT_GROUND))
    for s, m in zip(mesh_obj.material_slots, day):
        s.material = m
    hr._compose(panels, out_png, heights=900)
    shutil.rmtree(tmp, ignore_errors=True)
    print(f"preview -> {out_png}")


# --------------------------------------------------------------------------- run

def run(kit, build, clips_fn, preview, debug=None):
    """build(kit) adds all parts; clips_fn(kit) -> [Clip]; preview: kwargs of
    render_preview (loco, center_z, height_m, ...); debug: kwargs of rb.render_debug
    (center_z, head_pt, head_scale, strip_scale)."""
    asset = kit.asset
    args = zb.script_args()
    qk.check_gate(asset)
    zb.clean_scene()
    bpy.context.scene.render.fps = qr.FPS
    arm = kit.rig.build_armature(f"{asset}_rig")
    build(kit)
    mesh, img = kit.finish(arm)
    clips = clips_fn(kit)
    for c in clips:
        rb.bake_clip(kit.rig, arm, c)
    hr.reset_pose(arm)
    bpy.context.scene.frame_set(0)

    tris = zb.tri_count(mesh)
    mn, mx = zb.bounds(mesh)
    glow = sum(1 for p in mesh.data.polygons if p.material_index == 1)
    print(f"{asset}: {tris} tris ({glow} eye_glow), {len(kit.rig.names)} joints, bounds "
          f"x {mn.x:.3f}..{mx.x:.3f}  y {mn.y:.3f}..{mx.y:.3f}  z {mn.z:.3f}..{mx.z:.3f}")
    blend = zb.repo_path("assets", "blender", "animals", f"{asset}.blend")
    glb = zb.repo_path("assets", "models", "animals", f"{asset}.glb")
    zb.save_blend(blend)
    qr.export_animal([arm, mesh], glb)
    print(f"exported {glb} ({os.path.getsize(glb) / 1024:.1f} KB)")
    if "--no-preview" not in args:
        ls = qr.setup_preview(mesh, img)
        out = zb.repo_path("art", "animals", asset, "model_preview.png")
        render_preview(arm, mesh, img, out, ls, **preview)
        if "--debug" in args and debug:
            rb.render_debug(arm, args[args.index("--debug") + 1], ls, clips, **debug)
    if tris > TRI_BUDGET:
        print("OVER BUDGET")
        sys.exit(1)
    return arm, mesh


class Wings:
    """Folded <-> spread wing chains. bones: {side: [bone, ...]} (root first). The folded
    rest frame (span d0 = first bone head -> last bone tail, outward panel normal n0) maps onto
    the spread frame (span d1, normal n1 = up); the edge d0 x n0 (front-lower in the fold)
    becomes the leading edge."""

    def __init__(self, sk, bones, n0, d1, n1=Z):
        from mathutils import Quaternion
        self.Q = Quaternion
        self.sk, self.bones = sk, bones
        self.R, self.lead = {}, {}
        for s, sg in rb.SIDES:
            bs = bones[s]
            d0 = (sk.rest_tail[bs[-1]] - sk.rest_head[bs[0]]).normalized()
            nn = V((sg * n0[0], n0[1], n0[2]))
            self.R[s] = frame_rot(d0, nn, V((sg * d1[0], d1[1], d1[2])), n1)
            nn = (nn - d0 * d0.dot(nn)).normalized()
            self.lead[s] = d0.cross(nn).normalized()

    def apply(self, p, spread=0.0, flap=0.0, lag=0.0, lift=0.0, wrap=0.0, world=False):
        """spread 0..1; flap/lift: + = tips up (deg, about the body's forward axis); lag: + =
        outer bones bend down (deg per bone); wrap: + = folded wings turned in round the body (trailing edge towards the back);
        world: the spread frame is in world space (flight with a pitched body)."""
        Q, sk = self.Q, self.sk
        for s, sg in rb.SIDES:
            bs = self.bones[s]
            base = Q() if world else p.world(sk.parent[bs[0]])
            W0 = base @ rb.ry(-(flap + lift) * sg) @ rb.rz(wrap * sg) @ Q().slerp(self.R[s], spread)
            p.set_world(bs[0], W0)
            for i, b in enumerate(bs[1:], 1):
                st = rb.rot_between(sk.rest_dir[b], sk.rest_dir[bs[0]])
                p.set_world(b, W0 @ rb.qaxis(self.lead[s], lag * i * sg) @ Q().slerp(st, spread))


class QuadClips(qk.ClipSet):
    """quad_kit.ClipSet + a generic curled `sleep` (loop) for quadrupeds: body lowered onto
    the ground, legs folded under, spine curled to one side, head resting on the ground
    towards the tail, tail wrapped round, ears back. cfg["sleep"] = dict(drop, curl, nose_z)."""

    def sleep(self, f, n=90):
        c = self.c.get("sleep", {})
        t = TAU * f / n
        b = math.sin(t)
        drop = c.get("drop", 0.2)
        curl = c.get("curl", 16.0)
        p = qr.Pose(self.rig)
        p.hips_offset = Vector((0, 0, -drop + 0.004 * self.sc * b))
        p.rel["hips"] = rb.rz(-0.5 * curl)
        p.rel["spine"] = rb.rz(curl) @ rb.rx(1.0 * b)
        p.rel["chest"] = rb.rz(curl) @ rb.rx(0.8 * b)
        rig = self.rig
        for leg in qr.LEGS:
            up, lo, _ = qr.leg_bones(leg)
            A = p.head(up)
            L = rig.length[up] + rig.length[lo]
            fr = leg.startswith("front")
            fz = rig.fet_z[leg]
            tgt = A + Vector((0, (-0.45 if fr else 0.1) * L, 0))
            tgt.z = fz
            p.leg_ik(leg, tgt, -75.0 if fr else 60.0, pole=(FWD if fr else -FWD))
        p.rel["neck_1"] = rb.rz(1.3 * curl) @ rb.rx(c.get("neck", 18.0))
        p.rel["neck_2"] = rb.rz(1.3 * curl) @ rb.rx(c.get("neck", 18.0))
        p.rel["head"] = rb.rz(0.8 * curl) @ rb.rx(c.get("head", 12.0) + 1.0 * math.sin(t + 0.7))
        for i, j in enumerate(self.tail):
            p.rel[j] = rb.rz(c.get("tail_wrap", 35.0) * (1 + 0.3 * i)) @ rb.rx(-c.get("tail_down", 20.0) if i == 0 else 0.0)
        self.ears(p, back=28.0)
        self.extra(p, "sleep", f, n)
        return p

    def clips(self, names):
        out = []
        for nm in names:
            if nm == "sleep":
                out.append(qr.Clip("sleep", 90, True, self.sleep))
            else:
                out += super().clips([nm])
        return out
