//! glTF 2.0 binary (`.glb`) loading from bytes (ART-PIPELINE, TECH-ARCH decision 4).
//!
//! A [`Model`] is one mesh (all primitives merged into one vertex/index list) plus, for
//! skinned characters and animals, the skeleton and animation clips. Static models get their
//! node transforms baked into the vertices; skinned meshes stay in bind space (glTF skinning
//! rules: the mesh node's own transform is ignored).

use glam::{Mat3, Mat4, Quat, Vec3, Vec4};
use gltf::animation::util::ReadOutputs;
use gltf::animation::Interpolation;

use crate::anim::{Channel, ChannelKind, Skeleton, Trs};

/// Vertex and index data of one model.
#[derive(Debug, Clone, Default)]
pub struct MeshData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    /// Skinned meshes only: four joint indices (into [`Skeleton::joints`]) per vertex.
    pub joints: Vec<[u16; 4]>,
    /// Skinned meshes only: four joint weights per vertex.
    pub weights: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    /// Index ranges per glTF primitive with their material (index into [`Model::materials`]).
    pub submeshes: Vec<SubMesh>,
}

/// A range of [`MeshData::indices`] drawn with one material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubMesh {
    pub first_index: u32,
    pub index_count: u32,
    pub material: Option<usize>,
}

/// Base colour of a material: factor plus optional embedded texture.
#[derive(Debug, Clone)]
pub struct Material {
    pub base_color: [f32; 4],
    pub image: Option<ImageData>,
}

impl MeshData {
    /// Axis-aligned bounds `(min, max)`; zero for an empty mesh.
    pub fn bounds(&self) -> (Vec3, Vec3) {
        let mut it = self.positions.iter().map(|p| Vec3::from(*p));
        let Some(first) = it.next() else {
            return (Vec3::ZERO, Vec3::ZERO);
        };
        it.fold((first, first), |(lo, hi), p| (lo.min(p), hi.max(p)))
    }

    pub fn is_skinned(&self) -> bool {
        !self.joints.is_empty() && self.joints.len() == self.positions.len()
    }
}

/// An embedded image (base colour texture), still encoded.
#[derive(Debug, Clone)]
pub struct ImageData {
    pub mime: String,
    pub bytes: Vec<u8>,
}

/// One animation clip (ART-RIG §4: named `idle`, `walk`, …).
#[derive(Debug, Clone)]
pub struct Clip {
    pub name: String,
    pub duration: f32,
    pub channels: Vec<Channel>,
}

/// A loaded `.glb`.
#[derive(Debug, Clone)]
pub struct Model {
    pub mesh: MeshData,
    /// Base colour factor of the first material.
    pub base_color: [f32; 4],
    /// Base colour texture of the first material, if embedded.
    pub image: Option<ImageData>,
    /// Every material of the file (for [`SubMesh::material`]).
    pub materials: Vec<Material>,
    pub skeleton: Option<Skeleton>,
    pub clips: Vec<Clip>,
}

#[derive(Debug)]
pub enum LoadError {
    Gltf(gltf::Error),
    Invalid(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Gltf(e) => write!(f, "glTF: {e}"),
            LoadError::Invalid(s) => write!(f, "invalid model: {s}"),
        }
    }
}

impl std::error::Error for LoadError {}

impl Model {
    /// Parses a `.glb` (buffers must be embedded in the binary chunk).
    pub fn from_glb(bytes: &[u8]) -> Result<Self, LoadError> {
        let gltf = gltf::Gltf::from_slice(bytes).map_err(LoadError::Gltf)?;
        let blob = gltf.blob.as_deref();
        let doc = &gltf.document;
        let get_buffer = |b: gltf::Buffer| -> Option<&[u8]> {
            match b.source() {
                gltf::buffer::Source::Bin => blob,
                gltf::buffer::Source::Uri(_) => None,
            }
        };

        // Node hierarchy: parents and world matrices of the default scene.
        let n = doc.nodes().count();
        let mut parents = vec![None; n];
        for node in doc.nodes() {
            for c in node.children() {
                parents[c.index()] = Some(node.index());
            }
        }
        let rest: Vec<Trs> = doc
            .nodes()
            .map(|node| {
                let (t, r, s) = node.transform().decomposed();
                Trs {
                    translation: Vec3::from(t),
                    rotation: Quat::from_array(r),
                    scale: Vec3::from(s),
                }
            })
            .collect();
        let order = parents_first_order(&parents);
        let mut world = vec![Mat4::IDENTITY; n];
        for &i in &order {
            let local = rest[i].matrix();
            world[i] = match parents[i] {
                Some(p) => world[p] * local,
                None => local,
            };
        }

        let mut mesh = MeshData::default();
        let mut skin_index = None;
        for node in doc.nodes() {
            let Some(m) = node.mesh() else { continue };
            let skinned = node.skin().is_some();
            if skinned {
                skin_index = skin_index.or(node.skin().map(|s| s.index()));
            }
            let xf = if skinned {
                Mat4::IDENTITY
            } else {
                world[node.index()]
            };
            let nxf = Mat3::from_mat4(xf).inverse().transpose();
            for prim in m.primitives() {
                let reader = prim.reader(get_buffer);
                let base = mesh.positions.len() as u32;
                let Some(pos) = reader.read_positions() else {
                    continue;
                };
                let count_before = mesh.positions.len();
                mesh.positions
                    .extend(pos.map(|p| xf.transform_point3(Vec3::from(p)).to_array()));
                let count = mesh.positions.len() - count_before;
                match reader.read_normals() {
                    Some(ns) => mesh
                        .normals
                        .extend(ns.map(|v| (nxf * Vec3::from(v)).normalize_or_zero().to_array())),
                    None => mesh
                        .normals
                        .extend(std::iter::repeat_n([0.0, 1.0, 0.0], count)),
                }
                match reader.read_tex_coords(0) {
                    Some(uv) => mesh.uvs.extend(uv.into_f32()),
                    None => mesh.uvs.extend(std::iter::repeat_n([0.5, 0.5], count)),
                }
                if skinned {
                    match (reader.read_joints(0), reader.read_weights(0)) {
                        (Some(j), Some(w)) => {
                            mesh.joints.extend(j.into_u16());
                            mesh.weights.extend(w.into_f32());
                        }
                        _ => {
                            mesh.joints.extend(std::iter::repeat_n([0; 4], count));
                            mesh.weights
                                .extend(std::iter::repeat_n([1.0, 0.0, 0.0, 0.0], count));
                        }
                    }
                }
                let first_index = mesh.indices.len() as u32;
                match reader.read_indices() {
                    Some(ix) => mesh.indices.extend(ix.into_u32().map(|i| i + base)),
                    None => mesh.indices.extend(base..base + count as u32),
                }
                mesh.submeshes.push(SubMesh {
                    first_index,
                    index_count: mesh.indices.len() as u32 - first_index,
                    material: prim.material().index(),
                });
            }
        }
        if mesh.positions.is_empty() {
            return Err(LoadError::Invalid("no mesh".into()));
        }
        if !mesh.joints.is_empty() && mesh.joints.len() != mesh.positions.len() {
            return Err(LoadError::Invalid(
                "mixed skinned and static primitives".into(),
            ));
        }

        // Materials: base colour factor and embedded texture.
        let materials: Vec<Material> = doc
            .materials()
            .map(|mat| {
                let pbr = mat.pbr_metallic_roughness();
                let image = pbr.base_color_texture().and_then(|info| {
                    match info.texture().source().source() {
                        gltf::image::Source::View { view, mime_type } => {
                            let buf = get_buffer(view.buffer())?;
                            let (start, end) = (view.offset(), view.offset() + view.length());
                            (end <= buf.len()).then(|| ImageData {
                                mime: mime_type.to_owned(),
                                bytes: buf[start..end].to_vec(),
                            })
                        }
                        gltf::image::Source::Uri { .. } => None,
                    }
                });
                Material {
                    base_color: pbr.base_color_factor(),
                    image,
                }
            })
            .collect();
        let base_color = materials.first().map_or([1.0; 4], |m| m.base_color);
        let image = materials.first().and_then(|m| m.image.clone());

        // Skeleton and clips.
        let skeleton = match skin_index.and_then(|i| doc.skins().nth(i)) {
            Some(skin) => {
                let reader = skin.reader(get_buffer);
                let joints: Vec<usize> = skin.joints().map(|j| j.index()).collect();
                let inverse_bind: Vec<Mat4> = match reader.read_inverse_bind_matrices() {
                    Some(m) => m.map(|c| Mat4::from_cols_array_2d(&c)).collect(),
                    None => vec![Mat4::IDENTITY; joints.len()],
                };
                Some(Skeleton::new(parents, rest, order, joints, inverse_bind))
            }
            None => None,
        };
        let mut clips = Vec::new();
        if skeleton.is_some() {
            for a in doc.animations() {
                let mut channels = Vec::new();
                let mut duration = 0.0f32;
                for ch in a.channels() {
                    let reader = ch.reader(get_buffer);
                    let (Some(inputs), Some(outputs)) =
                        (reader.read_inputs(), reader.read_outputs())
                    else {
                        continue;
                    };
                    let times: Vec<f32> = inputs.collect();
                    let (kind, mut values): (ChannelKind, Vec<Vec4>) = match outputs {
                        ReadOutputs::Translations(t) => (
                            ChannelKind::Translation,
                            t.map(|v| Vec3::from(v).extend(0.0)).collect(),
                        ),
                        ReadOutputs::Rotations(r) => (
                            ChannelKind::Rotation,
                            r.into_f32().map(Vec4::from).collect(),
                        ),
                        ReadOutputs::Scales(s) => (
                            ChannelKind::Scale,
                            s.map(|v| Vec3::from(v).extend(0.0)).collect(),
                        ),
                        ReadOutputs::MorphTargetWeights(_) => continue,
                    };
                    let interp = ch.sampler().interpolation();
                    if interp == Interpolation::CubicSpline {
                        // Keep the value of each (in-tangent, value, out-tangent) triple.
                        values = values.chunks(3).filter_map(|c| c.get(1).copied()).collect();
                    }
                    if times.is_empty() || times.len() != values.len() {
                        continue;
                    }
                    duration = duration.max(*times.last().unwrap_or(&0.0));
                    channels.push(Channel {
                        node: ch.target().node().index(),
                        kind,
                        times,
                        values,
                        step: interp == Interpolation::Step,
                    });
                }
                clips.push(Clip {
                    name: a.name().unwrap_or("").to_owned(),
                    duration,
                    channels,
                });
            }
        }

        Ok(Model {
            mesh,
            base_color,
            image,
            materials,
            skeleton,
            clips,
        })
    }

    pub fn clip(&self, name: &str) -> Option<&Clip> {
        self.clips.iter().find(|c| c.name == name)
    }
}

/// Node indices ordered so that every parent comes before its children.
fn parents_first_order(parents: &[Option<usize>]) -> Vec<usize> {
    let n = parents.len();
    let mut depth = vec![usize::MAX; n];
    fn d(i: usize, parents: &[Option<usize>], depth: &mut [usize], guard: usize) -> usize {
        if depth[i] != usize::MAX {
            return depth[i];
        }
        let v = match parents[i] {
            Some(p) if guard > 0 => d(p, parents, depth, guard - 1) + 1,
            _ => 0,
        };
        depth[i] = v;
        v
    }
    for i in 0..n {
        d(i, parents, &mut depth, n);
    }
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| depth[i]);
    order
}
