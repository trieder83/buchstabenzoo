"""koala_joey — the koala pair's baby (joey) (GAME-FAMILY; ART-ANIMALS; concept art/animals/koala_joey/).

Run:  blender -b --factory-startup --python tools/blender/animals/koala_joey.py [-- --no-preview]

The adult `koala` (koala.py: same kit, rig, joints and clips) at 45 %% (GAME-FAMILY §8), reshaped
in adult space into a cute baby (concept: round chubby body on short stubby limbs, a huge round
head, big fluffy ears, huge eyes in cream eye patches, slightly lighter coat):
  * torso x1.15 / y1.05 / z1.20 about the barrel centre (rounder, legs show less -> stubby),
  * head x1.38 about the head joint, ears (no extra scale),
  * legs x1.25 thicker (chubby), bigger paws, eyes x1.3 (EYE_R) with cream patches.
The skeleton is only scaled uniformly (x0.45): clips and walk speed are unchanged.
Writes assets/models/animals/koala_joey.glb, assets/blender/animals/koala_joey.blend,
assets/textures/animals/koala_joey_body.png, art/animals/koala_joey/model_preview.png.
"""

import os
import sys

from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import koala  # noqa: E402
import quad_kit as qk  # noqa: E402

qr = qk.qr
ASSET = "koala_joey"
SCALE = 0.45
HEAD_J = Vector(koala.P["head"])
HEAD_K = 1.38
EAR_K = 1.0
LEG_K = 1.25
BODY_C = Vector((0.0, 0.03, 0.36))
TORSO = (1.15, 1.05, 1.20)


def head_f(c):
    return HEAD_J + (c - HEAD_J) * HEAD_K


def ear_f(c):
    p = head_f(c)
    base = head_f(Vector(koala.P["ear_base"]))
    return base + (p - base) * EAR_K


def torso_f(c):
    d = c - BODY_C
    return BODY_C + Vector((d.x * TORSO[0], d.y * TORSO[1], d.z * TORSO[2]))


def leg_f(c):
    cx = 0.12 if c.x > 0 else -0.12
    k = 1.0 + (LEG_K - 1.0) * min(1.0, max(0.0, (0.30 - c.z) / 0.15))  # fat below the fur line
    return Vector((cx + (c.x - cx) * k, c.y, c.z))


def post(arm, mesh):
    legs = set()
    for leg in qr.LEGS:
        legs.update(qr.leg_bones(leg))
    qr.reshape(mesh, legs, leg_f)
    qr.reshape(mesh, {"chest", "spine", "hips", "tail_1", "tail_2"}, torso_f)
    qr.reshape(mesh, {"neck_1", "neck_2", "head"}, head_f)
    qr.reshape(mesh, {"ear_l", "ear_r"}, ear_f)


koala.COLORS["grey"] = "#A6ABB0"  # slightly lighter (brief)
koala.PATCH.update(chest_x=0.13, chest_z=0.51, belly_z=0.24, belly_x=0.13, chin_v=0.32,
                   eye_patch=0.08)
koala.EYE_R = (0.052, 0.060)  # big eyes (x1.3 the adult's before the head scale)
qr.set_variant(SCALE, post)

if __name__ == "__main__":
    kit = qk.Kit(ASSET, koala.COLORS, koala.P)
    qk.run(kit, koala.build, koala.clipset, ["idle", "walk", "eat", "happy", "refuse"],
           dict(center_z=0.45 * SCALE, walk_frame=3, big_scale=1.4 * SCALE, close_scale=1.25 * SCALE,
                head_pt=tuple(Vector((0, -0.32, 0.62)) * SCALE), debug_scale=0.45 * SCALE))
