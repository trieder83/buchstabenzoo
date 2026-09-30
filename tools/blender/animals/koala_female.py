"""koala_female — the koala pair's female (GAME-FAMILY; ART-ANIMALS; concept art/animals/koala_female/).

Run:  blender -b --factory-startup --python tools/blender/animals/koala_female.py [-- --no-preview]

The adult `koala` (koala.py: same kit, rig, joints and clips) at 1/1.1 (male ~10 %% larger, GAME-FAMILY §3) scale,
lighter silvery grey and a larger cream chest/belly patch up to the chin (brief).
Writes assets/models/animals/koala_female.glb, assets/blender/animals/koala_female.blend,
assets/textures/animals/koala_female_body.png, art/animals/koala_female/model_preview.png.
"""

import os
import sys

from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import koala  # noqa: E402
import quad_kit as qk  # noqa: E402

qr = qk.qr
ASSET = "koala_female"
SCALE = 1 / 1.1
koala.COLORS["grey"] = "#AEB3B8"  # lighter, silvery (brief)
koala.PATCH.update(chest_x=0.15, chest_z=0.53, belly_z=0.26, belly_x=0.15, chin_v=0.40)
qr.set_variant(SCALE)

if __name__ == "__main__":
    kit = qk.Kit(ASSET, koala.COLORS, koala.P)
    qk.run(kit, koala.build, koala.clipset, ["idle", "walk", "eat", "happy", "refuse"],
           dict(center_z=0.45 * SCALE, walk_frame=3, big_scale=1.4 * SCALE, close_scale=1.25 * SCALE,
                head_pt=tuple(Vector((0, -0.32, 0.62)) * SCALE), debug_scale=0.45 * SCALE))
