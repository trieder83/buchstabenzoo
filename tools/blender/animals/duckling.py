"""duckling — ambient comic duckling following its mother on the river (GAME-AMBIENT;
`bird` rig, bird_rig.py — same joints and clips as duck.py).

Run:  blender -b --factory-startup --python tools/blender/animals/duckling.py
      [-- --debug <dir>]   extra review renders (not committed)

Writes
  assets/blender/animals/duckling.blend
  assets/models/animals/duckling.glb         1 skin (16 joints), 1 mesh, clips swim idle dip flap preen
  assets/textures/animals/duckling_body.png

Look: kit 5 duck (art/props/kit_water/sheet_v1.jpg) as a chick: yellow and fluffy (three
head feathers, fluffy tail tufts), round body, very big head and eyes, short neck, stubby
wings, small orange bill. ~0.22 m bill tip to tail. Authored in duck units, scaled by S.
Origin: the water surface under the body centre (bird_rig.py).
"""

import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bird_rig as br  # noqa: E402

COLORS = {
    "body": "#FFD84A",
    "wing": "#FFCB2E",
    "wing_line": "#EAA91E",
    "bill": "#F7901E",
    "foot": "#F28A1C",
    "eye_white": "#FFFFFF",
    "pupil": "#1E1E1E",
    "ink": "#2B1B12",
}

DUCKLING = {
    "S": 0.585,
    "body": [(-0.118, 0.090, 0.040, 0.040, 0.046), (-0.100, 0.086, 0.086, 0.078, 0.092),
             (-0.062, 0.082, 0.112, 0.098, 0.120), (-0.005, 0.084, 0.116, 0.100, 0.126),
             (0.050, 0.094, 0.100, 0.084, 0.110), (0.092, 0.112, 0.070, 0.058, 0.076),
             (0.118, 0.140, 0.034, 0.028, 0.036)],
    "body_n": 12,
    "tail_pole": (0.128, 0.160),
    "head": (-0.070, 0.268),
    "head_r": (0.106, 0.102, 0.100),
    "head_rings": 7,
    "neck": [(-0.052, 0.140, 0.072), (-0.062, 0.190, 0.070), (-0.068, 0.225, 0.072)],
    "bill": {"base": (-0.155, 0.248), "len": 0.062, "tilt": 0.06,
             "sections": [(0.0, 0.032, 0.026, 0.0), (0.5, 0.034, 0.019, -0.001),
                          (1.0, 0.030, 0.014, 0.004)],
             "tip_round": 0.35},
    "eye": {"dir": (46.0, 16.0), "r": (0.040, 0.045), "bulge": 0.5, "pupil": 0.52},
    "wing": {"path": [(-0.040, 60, 0.004), (0.000, 57, 0.008), (0.045, 53, 0.009),
                      (0.085, 46, 0.006)],
             "mid": 2,
             "w": [0.030, 0.046, 0.040, 0.016], "thick": 0.018},
    "leg": {"hip": (0.040, 0.020, 0.010), "ankle": (0.044, 0.035, -0.060),
            "toe": (0.048, -0.025, -0.068), "r": 0.012, "foot_w": 0.032},
    # three fluffy head feathers and two tail tufts: (base, tip, radius)
    "tufts": [((0.0, -0.070, 0.350), (0.0, -0.060, 0.405), 0.018),
              ((0.014, -0.058, 0.348), (0.034, -0.040, 0.392), 0.014),
              ((-0.014, -0.058, 0.348), (-0.034, -0.040, 0.392), 0.014)],
    "tail_tufts": [((0.018, 0.112, 0.140), (0.030, 0.150, 0.180), 0.016),
                   ((-0.018, 0.112, 0.140), (-0.030, 0.150, 0.180), 0.016)],
    "joints": {"hips": (0, 0.030, 0.085), "spine": (0, -0.020, 0.085),
               "chest": (0, -0.050, 0.100), "neck_1": (0, -0.056, 0.150),
               "neck_2": (0, -0.062, 0.190), "head": (0, -0.066, 0.225),
               "head_end": (0, -0.225, 0.245), "tail": ((0, 0.085, 0.110), (0, 0.130, 0.165))},
    "body_weights": [(-0.080, "chest"), (-0.050, "chest"), (-0.025, "spine"), (0.000, "spine"),
                     (0.035, "hips"), (0.070, "hips"), (0.105, "tail")],
    "neck_weights": [(0.130, "chest"), (0.150, "neck_1"), (0.170, "neck_1"), (0.190, "neck_2"),
                     (0.205, "neck_2"), (0.230, "head")],
}

br.run("duckling", DUCKLING, COLORS)
