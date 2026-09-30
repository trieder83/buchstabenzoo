"""zebra_foal — the zebra pair's baby (GAME-FAMILY §5/§8; ART-ANIMALS; concept
art/animals/zebra_foal/).

Run:  blender -b --factory-startup --python tools/blender/animals/zebra_foal.py [-- --no-preview]

Same rig, joints and clips as the adult `zebra` (zebra.py), the whole model at 45 % (GAME-FAMILY
§8), but the mesh is reshaped in adult space into a baby (concept: very long thin legs, small
round body, short stout neck, BIG head with huge eyes, short muzzle, big ears, fluffy short
mane, fluffy tail tuft, bold broad stripes):
  * torso x0.88 / y0.80 / z0.85 about the barrel centre (legs stay -> they read much longer),
  * neck x0.75 long / x1.3 thick, head x1.38 (x0.95 along the muzzle) sitting on the short neck,
  * legs thinner (x0.72) with big knobbly hooves, eyes x1.45 bigger (EYE_R), fuller mane.
The skeleton is only scaled uniformly (x0.45), so the clips and the walk speed (0.63 m/s) are
unchanged. Stripes are the brief's softer brown-black `#3A3330`. Writes
assets/models/animals/zebra_foal.glb, assets/blender/animals/zebra_foal.blend,
assets/textures/animals/zebra_foal_body.png, art/animals/zebra_foal/model_preview.png.
"""

import os
import sys

from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import zebra  # noqa: E402

ASSET = "zebra_foal"
SCALE = 0.45  # GAME-FAMILY §8
LEG_K = 0.72       # leg thickness
NECK_K = 0.75      # neck length
NECK_W = 1.30      # neck thickness (stout baby neck)
HEAD_LAT = 1.38    # head size across / up
HEAD_ALONG = 0.95  # head length along the muzzle (short muzzle)
EAR_K = 1.15       # extra ear size on top of the head scale
TORSO = (0.88, 0.80, 0.85)
BODY_C = Vector((0.0, 0.0, 0.93))

qr = zebra.qr
HEAD_J = Vector(zebra.P["head"])
NECK_A = zebra.NECK_A
HEAD_D = zebra.HEAD_D
NECK_J = NECK_A + (HEAD_J - NECK_A) * NECK_K  # where the head joint ends up


def neck_f(c):
    d = c - NECK_A
    a = zebra.NECK_AX * d.dot(zebra.NECK_AX)
    return NECK_A + a * NECK_K + (d - a) * NECK_W


def head_f(c):
    d = c - HEAD_J
    a = HEAD_D * d.dot(HEAD_D)
    return NECK_J + a * HEAD_ALONG + (d - a) * HEAD_LAT


def ear_f(c):
    p = head_f(c)
    base = head_f(Vector(zebra.P["ear_base"]))
    return base + (p - base) * EAR_K


def torso_f(c):
    d = c - BODY_C
    return BODY_C + Vector((d.x * TORSO[0], d.y * TORSO[1], d.z * TORSO[2]))


def leg_f(c):
    cx = 0.21 if c.x > 0 else -0.21
    # thin about the leg axis below the belly only (keeps the joins to the torso), big hooves
    k = 1.0 - (1.0 - LEG_K) * min(1.0, max(0.0, (0.80 - c.z) / 0.25))
    hoof = 1.0 + 0.30 * min(1.0, max(0.0, (0.17 - c.z) / 0.08))
    return Vector((cx + (c.x - cx) * k * hoof, c.y, c.z))


def post(arm, mesh):
    legs = set()
    for leg in qr.LEGS:
        legs.update(qr.leg_bones(leg))
    qr.reshape(mesh, legs, leg_f)
    qr.reshape(mesh, {"chest", "spine", "hips", "tail_1", "tail_2"}, torso_f)
    qr.reshape(mesh, {"neck_1", "neck_2"}, neck_f)
    qr.reshape(mesh, {"head"}, head_f)
    qr.reshape(mesh, {"ear_l", "ear_r"}, ear_f)


zebra.ASSET = ASSET
zebra.BLEND = zebra.zb.repo_path("assets", "blender", "animals", f"{ASSET}.blend")
zebra.GLB = zebra.zb.repo_path("assets", "models", "animals", f"{ASSET}.glb")
zebra.BODY_PNG = zebra.zb.repo_path("assets", "textures", "animals", f"{ASSET}_body.png")
zebra.PREVIEW = zebra.zb.repo_path("art", "animals", ASSET, "model_preview.png")
zebra.COLORS["black"] = "#3A3330"
zebra.EYE_R = (0.135, 0.115)  # huge round eyes (~x1.35 the adult's after the head scale)
zebra.MANE_W = 0.075           # fluffy, wide, short mane
zebra.MANE_H = 0.95
zebra.TUFT_K = 1.35            # fluffy tail tuft
zebra.TORSO_STRIPES = [        # fewer, bolder stripes
    (0.17, 0.085, 0.02), (0.29, 0.085, 0.03), (0.41, 0.085, 0.03), (0.53, 0.085, 0.02),
    (0.65, 0.085, -0.01), (0.77, 0.085, -0.04), (0.89, 0.07, -0.04)]
qr.set_variant(SCALE, post)

if __name__ == "__main__":
    zebra.main()
