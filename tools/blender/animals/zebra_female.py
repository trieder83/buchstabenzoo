"""zebra_female — the zebra pair's female (GAME-FAMILY §3; ART-ANIMALS; concept
art/animals/zebra_female/).

Run:  blender -b --factory-startup --python tools/blender/animals/zebra_female.py [-- --no-preview]

Same model, rig (23 joints) and clips as the adult male `zebra` (zebra.py — the male is 10 %
larger, so the female is the male at 1/1.1), built with the same code plus a swept black forelock
over the forehead and lashes (concept, Q-203), a slimmer mane. Writes assets/models/animals/zebra_female.glb, assets/blender/animals/zebra_female.blend,
assets/textures/animals/zebra_female_body.png, art/animals/zebra_female/model_preview.png.
"""

import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import zebra  # noqa: E402

ASSET = "zebra_female"
SCALE = 1 / 1.1  # GAME-FAMILY §3: male ~10 % larger

zebra.ASSET = ASSET
zebra.BLEND = zebra.zb.repo_path("assets", "blender", "animals", f"{ASSET}.blend")
zebra.GLB = zebra.zb.repo_path("assets", "models", "animals", f"{ASSET}.glb")
zebra.BODY_PNG = zebra.zb.repo_path("assets", "textures", "animals", f"{ASSET}_body.png")
zebra.PREVIEW = zebra.zb.repo_path("art", "animals", ASSET, "model_preview.png")
zebra.FORELOCK = True   # swept forelock over the forehead (concept, Q-203)
zebra.LASHES = True     # heavy upper lid + outer lash flick on the eyes (concept, Q-203)
zebra.MANE_W = 0.038    # slimmer mane than the male's
zebra.qr.set_variant(SCALE)

if __name__ == "__main__":
    zebra.main()
