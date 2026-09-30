"""zebra_foal — the zebra pair's baby (GAME-FAMILY §5/§8; ART-ANIMALS; concept
art/animals/zebra_foal/).

Run:  blender -b --factory-startup --python tools/blender/animals/zebra_foal.py [-- --no-preview]

The adult `zebra` (zebra.py) at 45 % (shoulder ~0.6 m, ears ~1.0 m as in the brief), same rig
and clips, with baby proportions reshaped in adult space before scaling: a bigger head with
bigger ears/eyes (x1.3 about the head joint) and thinner, knobbly-looking legs. Stripes are the
brief's softer brown-black `#3A3330`. Writes assets/models/animals/zebra_foal.glb,
assets/blender/animals/zebra_foal.blend, assets/textures/animals/zebra_foal_body.png,
art/animals/zebra_foal/model_preview.png.
"""

import os
import sys

from mathutils import Vector

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import zebra  # noqa: E402

ASSET = "zebra_foal"
SCALE = 0.45  # GAME-FAMILY §8
HEAD_K = 1.3
LEG_K = 0.78  # leg thickness

qr = zebra.qr
HEAD_J = Vector(zebra.P["head"])


def post(arm, mesh):
    qr.reshape(mesh, {"head", "ear_l", "ear_r"}, lambda c: HEAD_J + (c - HEAD_J) * HEAD_K)
    legs = set()
    for leg in qr.LEGS:
        legs.update(qr.leg_bones(leg))

    def thin(c):
        cx = 0.21 if c.x > 0 else -0.21
        # thin the legs about the leg axis, below the belly only (keeps the joins to the torso)
        k = 1.0 - (1.0 - LEG_K) * min(1.0, max(0.0, (0.80 - c.z) / 0.25))
        return Vector((cx + (c.x - cx) * k, c.y, c.z))
    qr.reshape(mesh, legs, thin)


zebra.ASSET = ASSET
zebra.BLEND = zebra.zb.repo_path("assets", "blender", "animals", f"{ASSET}.blend")
zebra.GLB = zebra.zb.repo_path("assets", "models", "animals", f"{ASSET}.glb")
zebra.BODY_PNG = zebra.zb.repo_path("assets", "textures", "animals", f"{ASSET}_body.png")
zebra.PREVIEW = zebra.zb.repo_path("art", "animals", ASSET, "model_preview.png")
zebra.COLORS["black"] = "#3A3330"
qr.set_variant(SCALE, post)

if __name__ == "__main__":
    zebra.main()
