//! Skeletal animation sampling and blending (ART-RIG §4). Pure math, no allocations in the
//! per-frame functions: callers own the [`Pose`] and joint-matrix buffers.

use glam::{Mat4, Quat, Vec3, Vec4};

use crate::model::Clip;

/// Local transform of a node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trs {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Trs {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Linear blend (`t = 0` → `self`, `t = 1` → `other`), shortest-arc rotation.
    pub fn blend(&self, other: &Trs, t: f32) -> Trs {
        let mut r = other.rotation;
        if self.rotation.dot(r) < 0.0 {
            r = -r;
        }
        Trs {
            translation: self.translation.lerp(other.translation, t),
            rotation: self.rotation.lerp(r, t).normalize(),
            scale: self.scale.lerp(other.scale, t),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelKind {
    Translation,
    Rotation,
    Scale,
}

/// Keyframes of one node property. Translation/scale values use `xyz`; rotations are
/// quaternions `xyzw`.
#[derive(Debug, Clone)]
pub struct Channel {
    pub node: usize,
    pub kind: ChannelKind,
    pub times: Vec<f32>,
    pub values: Vec<Vec4>,
    pub step: bool,
}

impl Channel {
    fn sample(&self, t: f32) -> Vec4 {
        let times = &self.times;
        let last = times.len() - 1;
        if t <= times[0] {
            return self.values[0];
        }
        if t >= times[last] {
            return self.values[last];
        }
        // Binary search for the key interval.
        let i = times
            .partition_point(|&k| k <= t)
            .saturating_sub(1)
            .min(last - 1);
        if self.step {
            return self.values[i];
        }
        let span = (times[i + 1] - times[i]).max(1e-6);
        let f = ((t - times[i]) / span).clamp(0.0, 1.0);
        let (a, b) = (self.values[i], self.values[i + 1]);
        match self.kind {
            ChannelKind::Rotation => {
                let qa = Quat::from_vec4(a);
                let mut qb = Quat::from_vec4(b);
                if qa.dot(qb) < 0.0 {
                    qb = -qb;
                }
                Vec4::from(qa.slerp(qb, f))
            }
            _ => a.lerp(b, f),
        }
    }
}

/// Local transforms of every node of a skeleton's document.
pub type Pose = Vec<Trs>;

/// Node hierarchy plus skin of a skinned model.
#[derive(Debug, Clone)]
pub struct Skeleton {
    parents: Vec<Option<usize>>,
    rest: Vec<Trs>,
    order: Vec<usize>,
    /// Node index of each joint (the skin's joint list).
    pub joints: Vec<usize>,
    pub inverse_bind: Vec<Mat4>,
}

impl Skeleton {
    pub fn new(
        parents: Vec<Option<usize>>,
        rest: Vec<Trs>,
        order: Vec<usize>,
        joints: Vec<usize>,
        inverse_bind: Vec<Mat4>,
    ) -> Self {
        Self {
            parents,
            rest,
            order,
            joints,
            inverse_bind,
        }
    }

    pub fn node_count(&self) -> usize {
        self.rest.len()
    }

    pub fn joint_count(&self) -> usize {
        self.joints.len()
    }

    /// The rest pose (node transforms of the file).
    pub fn rest_pose(&self) -> Pose {
        self.rest.clone()
    }

    /// Writes the pose of `clip` at time `t` (looped) into `out` (length = node count).
    pub fn sample(&self, clip: &Clip, t: f32, out: &mut [Trs]) {
        out.copy_from_slice(&self.rest);
        let t = if clip.duration > 0.0 {
            t.rem_euclid(clip.duration)
        } else {
            0.0
        };
        for ch in &clip.channels {
            let Some(node) = out.get_mut(ch.node) else {
                continue;
            };
            let v = ch.sample(t);
            match ch.kind {
                ChannelKind::Translation => node.translation = v.truncate(),
                ChannelKind::Rotation => node.rotation = Quat::from_vec4(v).normalize(),
                ChannelKind::Scale => node.scale = v.truncate(),
            }
        }
    }

    /// `out[i] = a[i]` blended towards `b[i]` by `t`.
    pub fn blend(a: &[Trs], b: &[Trs], t: f32, out: &mut [Trs]) {
        for ((o, x), y) in out.iter_mut().zip(a).zip(b) {
            *o = x.blend(y, t);
        }
    }

    /// Skinning matrices (`world(joint) × inverse_bind`) for `pose`. `world` is scratch space
    /// of node count length; `out` has joint count length.
    pub fn joint_matrices(&self, pose: &[Trs], world: &mut [Mat4], out: &mut [Mat4]) {
        for &i in &self.order {
            let local = pose[i].matrix();
            world[i] = match self.parents[i] {
                Some(p) => world[p] * local,
                None => local,
            };
        }
        for (k, o) in out.iter_mut().enumerate() {
            *o = world[self.joints[k]] * self.inverse_bind[k];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain() -> Skeleton {
        let rest = vec![
            Trs {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            Trs {
                translation: Vec3::Y,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
        ];
        let inv = vec![Mat4::IDENTITY, Mat4::from_translation(-Vec3::Y)];
        Skeleton::new(vec![None, Some(0)], rest, vec![0, 1], vec![0, 1], inv)
    }

    #[test]
    fn rest_pose_gives_identity_skinning() {
        let s = chain();
        let pose = s.rest_pose();
        let mut world = vec![Mat4::IDENTITY; 2];
        let mut out = vec![Mat4::ZERO; 2];
        s.joint_matrices(&pose, &mut world, &mut out);
        for m in out {
            assert!(m.abs_diff_eq(Mat4::IDENTITY, 1e-5));
        }
    }

    #[test]
    fn sample_interpolates_and_loops() {
        let s = chain();
        let clip = Clip {
            name: "walk".into(),
            duration: 1.0,
            channels: vec![Channel {
                node: 1,
                kind: ChannelKind::Translation,
                times: vec![0.0, 1.0],
                values: vec![Vec4::new(0.0, 1.0, 0.0, 0.0), Vec4::new(2.0, 1.0, 0.0, 0.0)],
                step: false,
            }],
        };
        let mut pose = s.rest_pose();
        s.sample(&clip, 0.25, &mut pose);
        assert!((pose[1].translation.x - 0.5).abs() < 1e-5);
        s.sample(&clip, 1.25, &mut pose);
        assert!((pose[1].translation.x - 0.5).abs() < 1e-5);
    }
}
