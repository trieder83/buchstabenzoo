"""Shared `biped_animal` rig for animals drawn upright on two legs (monkey; later others).

Skeleton (23 joints, names follow human_rig where they overlap):

    root                         ground, never animated
    └─ hips                      pelvis; carries the legs and the tail
       ├─ spine ─ chest          chest carries the arms and the neck
       │          ├─ neck ─ head
       │          ├─ upper_arm_l ─ lower_arm_l ─ hand_l
       │          └─ upper_arm_r ─ lower_arm_r ─ hand_r
       ├─ upper_leg_l ─ lower_leg_l ─ foot_l
       ├─ upper_leg_r ─ lower_leg_r ─ foot_r
       └─ tail_1 ─ tail_2 ─ tail_3 ─ tail_4 ─ tail_5

Rest pose: standing, legs vertical, arms hanging down and slightly out (straight), tail
chain along the modelled tail curve. Blender space as quadruped_rig (faces -Y = glTF +Z,
left = +X, origin on the ground between the feet). Joint *positions* come from the
animal's proportions dict P (left side; mirrored for _r):

    hips, spine, chest, neck, head, head_top      midline points
    shoulder, elbow, wrist, hand_end              left arm
    hip, knee, ankle, toe                         left leg (toe on the ground)
    tail: [6 points]                              tail_1..tail_5 heads + tail end
"""

import math
import os
import sys

from mathutils import Vector

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rig_base as rb  # noqa: E402

qr = rb.qr
FPS = rb.FPS
SIDES = rb.SIDES
TAIL_N = 5


def _names():
    t = [("root", None), ("hips", "root"), ("spine", "hips"), ("chest", "spine"),
         ("neck", "chest"), ("head", "neck")]
    for s, _ in SIDES:
        t += [(f"upper_arm_{s}", "chest"), (f"lower_arm_{s}", f"upper_arm_{s}"),
              (f"hand_{s}", f"lower_arm_{s}")]
    for s, _ in SIDES:
        t += [(f"upper_leg_{s}", "hips"), (f"lower_leg_{s}", f"upper_leg_{s}"),
              (f"foot_{s}", f"lower_leg_{s}")]
    t += [("tail_1", "hips")] + [(f"tail_{i}", f"tail_{i - 1}") for i in range(2, TAIL_N + 1)]
    return t


JOINTS = _names()
JOINT_NAMES = [n for n, _ in JOINTS]
PARENT = dict(JOINTS)


def arm_bones(s):
    return f"upper_arm_{s}", f"lower_arm_{s}", f"hand_{s}"


def leg_bones(s):
    return f"upper_leg_{s}", f"lower_leg_{s}", f"foot_{s}"


class Rig(rb.Skeleton):
    def __init__(self, P):
        self.P = P
        m = rb.mirror
        j = [("root", None, (0, 0, 0), (0, 0, 0.10)),
             ("hips", "root", P["hips"], P["spine"]),
             ("spine", "hips", P["spine"], P["chest"]),
             ("chest", "spine", P["chest"], P["neck"]),
             ("neck", "chest", P["neck"], P["head"]),
             ("head", "neck", P["head"], P["head_top"])]
        for s, sg in SIDES:
            j += [(f"upper_arm_{s}", "chest", m(P["shoulder"], sg), m(P["elbow"], sg)),
                  (f"lower_arm_{s}", f"upper_arm_{s}", m(P["elbow"], sg), m(P["wrist"], sg)),
                  (f"hand_{s}", f"lower_arm_{s}", m(P["wrist"], sg), m(P["hand_end"], sg))]
        for s, sg in SIDES:
            j += [(f"upper_leg_{s}", "hips", m(P["hip"], sg), m(P["knee"], sg)),
                  (f"lower_leg_{s}", f"upper_leg_{s}", m(P["knee"], sg), m(P["ankle"], sg)),
                  (f"foot_{s}", f"lower_leg_{s}", m(P["ankle"], sg), m(P["toe"], sg))]
        tp = P["tail"]
        for i in range(TAIL_N):
            j.append((f"tail_{i + 1}", "hips" if i == 0 else f"tail_{i}", tp[i], tp[i + 1]))
        assert [n for n, *_ in j] == JOINT_NAMES
        super().__init__(j)
        self.ankle_z = P["ankle"][2]


class Pose(rb.Pose):
    def leg_ik(self, s, ankle_target, pitch_deg=0.0, pole=rb.FWD, foot_world=None):
        up, lo, ft = leg_bones(s)
        self.limb_ik(up, lo, ankle_target, pole)
        self.set_world(ft, foot_world if foot_world is not None else rb.rx(-pitch_deg))

    def arm_ik(self, s, wrist_target, pole, hand_world=None):
        up, lo, hd = arm_bones(s)
        self.limb_ik(up, lo, wrist_target, pole)
        if hand_world is not None:
            self.set_world(hd, hand_world)

    def rest_ankle(self, s):
        return self.sk.rest_head[leg_bones(s)[2]].copy()

    def stand(self, offset=(0, 0, 0), hips_rot=None, spine=None, chest=None, feet=None):
        """Body transform, then both feet planted at their rest spots (or `feet`
        {side: (target, pitch)})."""
        self.hips_offset = Vector(offset)
        if hips_rot is not None:
            self.rel["hips"] = hips_rot
        if spine is not None:
            self.rel["spine"] = spine
        if chest is not None:
            self.rel["chest"] = chest
        for s, _ in SIDES:
            if feet and s in feet:
                t, pitch = feet[s]
                self.leg_ik(s, t, pitch)
            else:
                self.leg_ik(s, self.rest_ankle(s))


def make_walk(rig, frames, speed, stance, lift, toe_deg, fold_deg, toe_fwd, upper_fn,
              dip=0.01, phases=None, pose_cls=Pose):
    """Biped walk cycle with planted feet (quadruped_rig.QuadGait foot paths). The body is
    lowered per frame just enough that both ankle targets stay reachable (smoothed).
    upper_fn(pose, fi, t) poses spine/arms/head/tail afterwards."""
    phases = phases or {"l": 0.0, "r": 0.5}
    gaits = {s: qr.QuadGait(frames, speed, stance, lift, toe_deg, fold_deg, rig.ankle_z, toe_fwd)
             for s, _ in SIDES}

    def body(fi):
        t = 2 * math.pi * fi / frames
        p = pose_cls(rig)
        p.hips_offset = Vector((0.012 * math.sin(t), 0, -dip - 0.012 * math.cos(2 * t)))
        p.rel["hips"] = rb.rz(6.0 * math.sin(t)) @ rb.ry(-3.0 * math.sin(t))
        return p

    def target(s, fi):
        fw, h, pitch, planted = gaits[s].foot((fi / frames + phases[s]) % 1.0)
        r = rig.rest_head[leg_bones(s)[2]]
        return Vector((r.x, r.y - fw, h)), pitch, planted

    need = []
    for fi in range(frames):
        p = body(fi)
        worst = 0.0
        for s, _ in SIDES:
            up, lo, _ = leg_bones(s)
            A = p.head(up)
            T, _, planted = target(s, fi)
            reach = (rig.length[up] + rig.length[lo]) * (0.985 if planted else 0.97)
            dxy = math.hypot(T.x - A.x, T.y - A.y)
            need_z = (T.z + math.sqrt(max(reach * reach - dxy * dxy, 0.0))) - A.z
            worst = min(worst, need_z)
        need.append(worst)
    n = frames
    lo_ = [min(need[(i + k) % n] for k in (-2, -1, 0, 1, 2)) for i in range(n)]
    sm = [(lo_[(i - 1) % n] + 2 * lo_[i] + lo_[(i + 1) % n]) / 4 for i in range(n)]
    drop = [min(a, b) for a, b in zip(sm, need)]

    def pose(f):
        fi = f % frames
        t = 2 * math.pi * fi / frames
        p = body(fi)
        p.hips_offset.z += drop[fi]
        for s, _ in SIDES:
            T, pitch, _ = target(s, fi)
            p.leg_ik(s, T, pitch)
        upper_fn(p, fi, t)
        return p

    pose.gaits = gaits
    pose.drop = drop
    return pose
