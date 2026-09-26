"""duck — ambient comic duck on the river (GAME-AMBIENT; `bird` rig, bird_rig.py).

Run:  blender -b --factory-startup --python tools/blender/animals/duck.py
      [-- --debug <dir>]   extra review renders (close-ups, clip strips; not committed)

Writes
  assets/blender/animals/duck.blend          source (never hand-edited)
  assets/models/animals/duck.glb             1 skin (16 joints), 1 mesh, clips swim idle dip flap preen
  assets/textures/animals/duck_body.png      256 x 256 atlas (flat cells + eye + wing feathers)

Look: art/props/kit_water/sheet_v1.jpg (approved kit 5 duck): cream-white chunky body with
an upturned tail, big round head, big friendly eyes, orange bill and feet, pale-yellow wing
with scalloped feather rows. ~0.45 m bill tip to tail, head top ~0.37 m above the water.
Origin: the water surface under the body centre (bird_rig.py). Combined preview:
tools/blender/animals/ambient_preview.py.
"""

import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bird_rig as br  # noqa: E402

COLORS = {
    "body": "#FFF4D8",
    "wing": "#FBE6AE",
    "wing_line": "#EBC572",
    "bill": "#F7961E",
    "foot": "#F28A1C",
    "eye_white": "#FFFFFF",
    "pupil": "#1E1E1E",
    "ink": "#2B1B12",
}

# duck units = metres for the adult duck (S = 1)
DUCK = {
    "S": 1.0,
    # (y, z_centre, r_up, r_down, r_lat): chest front (-Y) -> tail tip
    "body": [(-0.150, 0.080, 0.040, 0.040, 0.050), (-0.132, 0.076, 0.078, 0.072, 0.086),
             (-0.098, 0.070, 0.100, 0.090, 0.110), (-0.045, 0.067, 0.110, 0.096, 0.124),
             (0.015, 0.068, 0.108, 0.093, 0.122), (0.075, 0.080, 0.094, 0.080, 0.105),
             (0.122, 0.108, 0.072, 0.056, 0.078), (0.150, 0.146, 0.046, 0.034, 0.046),
             (0.166, 0.180, 0.024, 0.018, 0.024)],
    "tail_pole": (0.174, 0.210),
    "head": (-0.110, 0.262),
    "head_r": (0.084, 0.080, 0.077),       # forward, up, lateral
    "neck": [(-0.078, 0.105, 0.066), (-0.094, 0.160, 0.060), (-0.104, 0.210, 0.062)],
    "bill": {"base": (-0.168, 0.240), "len": 0.088, "tilt": 0.10,
             # (t along the bill, half width, half height, lift)
             "sections": [(0.0, 0.034, 0.029, 0.0), (0.35, 0.037, 0.022, -0.002),
                          (0.75, 0.040, 0.018, 0.000), (1.0, 0.034, 0.015, 0.006)],
             "tip_round": 0.35},
    "eye": {"dir": (50.0, 18.0), "r": (0.033, 0.037), "bulge": 0.55, "pupil": 0.46},
    # wing centre line on the body side: (y, ring angle from the top in deg, out)
    "wing": {"path": [(-0.080, 62, 0.004), (-0.035, 58, 0.008), (0.025, 55, 0.010),
                      (0.085, 50, 0.010), (0.135, 42, 0.008)],
             "mid": 2,
             "w": [0.026, 0.046, 0.050, 0.040, 0.014], "thick": 0.016},
    "leg": {"hip": (0.042, 0.020, -0.005), "ankle": (0.046, 0.040, -0.070),
            "toe": (0.050, -0.030, -0.078), "r": 0.009, "foot_w": 0.030},
    "joints": {"hips": (0, 0.030, 0.070), "spine": (0, -0.030, 0.070),
               "chest": (0, -0.080, 0.090), "neck_1": (0, -0.086, 0.130),
               "neck_2": (0, -0.098, 0.175), "head": (0, -0.106, 0.215),
               "head_end": (0, -0.262, 0.238), "tail": ((0, 0.110, 0.100), (0, 0.178, 0.205))},
    "body_weights": [(-0.115, "chest"), (-0.075, "chest"), (-0.040, "spine"), (-0.005, "spine"),
                     (0.040, "hips"), (0.095, "hips"), (0.140, "tail")],
    "neck_weights": [(0.120, "chest"), (0.140, "neck_1"), (0.160, "neck_1"), (0.182, "neck_2"),
                     (0.200, "neck_2"), (0.225, "head")],
}

br.run("duck", DUCK, COLORS)
