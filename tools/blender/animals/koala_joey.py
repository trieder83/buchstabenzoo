"""koala_joey — the koala pair's baby (joey) (GAME-FAMILY; ART-ANIMALS; concept art/animals/koala_joey/).

Run:  blender -b --factory-startup --python tools/blender/animals/koala_joey.py [-- --no-preview]

The adult `koala` (koala.py: same kit, rig, joints and clips) at 45 %% (GAME-FAMILY §8) scale,
with a bigger head/ears (x1.25) and a slightly lighter coat (brief).
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
HEAD_K = 1.25  # even bigger head and ears; body stays tiny (x0.45)


def post(arm, mesh):
    qr.reshape(mesh, {"head", "ear_l", "ear_r"}, lambda c: HEAD_J + (c - HEAD_J) * HEAD_K)


koala.COLORS["grey"] = "#A6ABB0"  # slightly lighter (brief)
koala.PATCH.update(chest_x=0.13, chest_z=0.51, belly_z=0.24, belly_x=0.13, chin_v=0.32)
qr.set_variant(SCALE, post)

if __name__ == "__main__":
    kit = qk.Kit(ASSET, koala.COLORS, koala.P)
    qk.run(kit, koala.build, koala.clipset, ["idle", "walk", "eat", "happy", "refuse"],
           dict(center_z=0.45 * SCALE, walk_frame=3, big_scale=1.4 * SCALE, close_scale=1.25 * SCALE,
                head_pt=tuple(Vector((0, -0.32, 0.62)) * SCALE), debug_scale=0.45 * SCALE))
