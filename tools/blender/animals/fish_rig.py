"""Shared `fish` rig for swimming animals (goldfish; later other fish).

Skeleton (13 joints):

    root                          water-surface point above the body centre, never animated
    └─ hips                       body centre; the only joint with translation (rise, jump)
       ├─ head                    front of the body (eyes, mouth) — turns / snaps
       ├─ spine_1 ─ spine_2 ─ spine_3 ─ tail_fin      S-curve chain towards the tail
       │  └─ fin_dorsal (spine_1)                       └─ fin_anal (spine_2)
       ├─ fin_pec_l, fin_pec_r    pectoral fins (sides, behind the head)
       └─ fin_pelvic_l, fin_pelvic_r                    belly fins

Space: Blender Z up, the fish faces -Y (glTF +Z), left = +X (as the quadruped rig).
**Origin convention:** the model origin (root) is the point of the WATER SURFACE directly
above the body centre: in rest pose the body centre (hips) lies `depth` below y = 0, so the
game places the model origin at the water-surface height of the river / pond / bowl and
the fish swims just below it. Clips that leave the water (`happy`) or reach the surface
(`eat`) translate `hips` upwards relative to that surface.

P (Blender metres, left side for paired fins; mirrored for _r):
    hips, head, nose, spine_1, spine_2, spine_3, tail_fin, tail_end     midline points
    fin_dorsal, fin_pec, fin_pelvic, fin_anal: (base, tip)
"""

import os
import sys

from mathutils import Vector

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rig_base as rb  # noqa: E402

FPS = rb.FPS
SIDES = rb.SIDES

JOINTS = [("root", None), ("hips", "root"), ("head", "hips"), ("spine_1", "hips"),
          ("spine_2", "spine_1"), ("spine_3", "spine_2"), ("tail_fin", "spine_3"),
          ("fin_dorsal", "spine_1"), ("fin_pec_l", "hips"), ("fin_pec_r", "hips"),
          ("fin_pelvic_l", "hips"), ("fin_pelvic_r", "hips"), ("fin_anal", "spine_2")]
JOINT_NAMES = [n for n, _ in JOINTS]
PARENT = dict(JOINTS)
SPINE = ("hips", "spine_1", "spine_2", "spine_3", "tail_fin")


class Rig(rb.Skeleton):
    def __init__(self, P):
        self.P = P
        m = rb.mirror
        hips = Vector(P["hips"])
        j = [("root", None, (0, 0, 0), (0, 0, 0.05)),
             ("hips", "root", P["hips"], tuple(hips + Vector((0, -0.05, 0)))),
             ("head", "hips", P["head"], P["nose"]),
             ("spine_1", "hips", P["spine_1"], P["spine_2"]),
             ("spine_2", "spine_1", P["spine_2"], P["spine_3"]),
             ("spine_3", "spine_2", P["spine_3"], P["tail_fin"]),
             ("tail_fin", "spine_3", P["tail_fin"], P["tail_end"]),
             ("fin_dorsal", "spine_1", *P["fin_dorsal"])]
        for s, sg in SIDES:
            b, t = P["fin_pec"]
            j.append((f"fin_pec_{s}", "hips", m(b, sg), m(t, sg)))
        for s, sg in SIDES:
            b, t = P["fin_pelvic"]
            j.append((f"fin_pelvic_{s}", "hips", m(b, sg), m(t, sg)))
        j.append(("fin_anal", "spine_2", *P["fin_anal"]))
        assert [n for n, *_ in j] == JOINT_NAMES
        super().__init__(j)
        self.depth = -hips.z


Pose = rb.Pose
