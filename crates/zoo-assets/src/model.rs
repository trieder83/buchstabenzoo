//! glTF 2.0 binary (`.glb`) loading from bytes (ART-PIPELINE, TECH-ARCH decision 4).
//!
//! A [`Model`] is one mesh (all primitives merged into one vertex/index list) plus, for
//! skinned characters and animals, the skeleton and animation clips. Static models get their
//! node transforms baked into the vertices; skinned meshes stay in bind space (glTF skinning
//! rules: the mesh node's own transform is ignored).
//!
//! Multi-node assets (`tools/blender/props/README_night.md`, ARCH-006): every vertex keeps
//! the index of the [`NodePart`] it came from ([`MeshData::node`], 0 = the root / static
//! part), so the renderer can move child nodes (door leaves, sails, lids) about their pivot
//! or hide them (roof, upper walls) without separate meshes. Material slots keep their name,
//! emission and blend mode (`*_glow`, `eye_glow`, `glass`); empties (`light*`, `socket_*`)
//! become [`Model::empties`]; `*_face` primitives become [`Model::faces`] (text quads).

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
    /// Static models only: per vertex the index into [`Model::nodes`] (0 = root part).
    pub node: Vec<u8>,
}

/// A range of [`MeshData::indices`] drawn with one material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubMesh {
    pub first_index: u32,
    pub index_count: u32,
    pub material: Option<usize>,
}

/// A material slot: base colour factor, optional embedded texture, name, emission, blend.
#[derive(Debug, Clone)]
pub struct Material {
    pub base_color: [f32; 4],
    pub image: Option<ImageData>,
    /// glTF image index of the base colour texture (materials sharing the atlas share it:
    /// the renderer uploads it once per asset).
    pub image_index: Option<usize>,
    /// Slot name (`palette`, `lamp_glow`, `eye_glow`, `glass`, `note_face`, …).
    pub name: String,
    /// `emissiveFactor` (linear RGB); zero for non-glowing slots.
    pub emissive: [f32; 3],
    /// `alphaMode BLEND` (see-through panes).
    pub blend: bool,
}

impl Material {
    /// A glowing slot (`*_glow`, `eye_glow`): drawn like the base colour by day, emissive at
    /// night (GAME-NIGHT §10).
    pub fn is_glow(&self) -> bool {
        self.name.ends_with("_glow") && self.emissive.iter().any(|&c| c > 0.0)
    }

    /// The animal eye slot (ART-ANIMALS "Rig conventions" §4).
    pub fn is_eye_glow(&self) -> bool {
        self.name == "eye_glow"
    }

    /// See-through glass (`glass`, `alphaMode BLEND`).
    pub fn is_glass(&self) -> bool {
        self.blend || self.name == "glass"
    }

    /// Emission in sRGB (the renderer's colour space), from the linear `emissiveFactor`.
    pub fn emissive_srgb(&self) -> [f32; 3] {
        self.emissive.map(linear_to_srgb)
    }
}

/// Linear → sRGB transfer function (one channel).
pub fn linear_to_srgb(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// A node of a static model whose mesh can move or hide on its own (door leaf, sails, roof).
/// Index 0 is always the root part (everything baked, pivot at the origin).
#[derive(Debug, Clone, PartialEq)]
pub struct NodePart {
    pub name: String,
    /// The node's origin in model space (translation-only child nodes: the hinge / hub).
    pub pivot: Vec3,
}

/// A mesh-less node (`light`, `light_*` lamp positions, `socket_*` attach points).
#[derive(Debug, Clone, PartialEq)]
pub struct Empty {
    pub name: String,
    pub pos: Vec3,
}

/// A blank text face (`note_face`, `sign_face`): its corners in model space in reading order
/// (top-left, top-right, bottom-right, bottom-left), from the face's 0..1 UVs.
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub slot: String,
    pub corners: [Vec3; 4],
    pub normal: Vec3,
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
    /// Movable / hideable parts of a static model (index 0 = root; [`MeshData::node`]).
    pub nodes: Vec<NodePart>,
    /// Mesh-less nodes (lamp lights, sockets) in model space.
    pub empties: Vec<Empty>,
    /// Text faces (`*_face` slots) in model space.
    pub faces: Vec<Face>,
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
        let any_skin = doc.nodes().any(|n| n.skin().is_some());
        // Parts of a static model: every direct child of a root node that has a mesh in its
        // subtree is a part of its own (ARCH-006); the roots themselves are part 0.
        let mut nodes = vec![NodePart {
            name: String::new(),
            pivot: Vec3::ZERO,
        }];
        let mut part_of = vec![0u8; n];
        if !any_skin {
            for &i in &order {
                if let Some(p) = parents[i] {
                    part_of[i] = if parents[p].is_none() {
                        let has_mesh = subtree_has_mesh(doc, i);
                        if has_mesh && nodes.len() < 255 {
                            nodes.push(NodePart {
                                name: doc
                                    .nodes()
                                    .nth(i)
                                    .and_then(|x| x.name().map(str::to_owned))
                                    .unwrap_or_default(),
                                pivot: world[i].transform_point3(Vec3::ZERO),
                            });
                            (nodes.len() - 1) as u8
                        } else {
                            0
                        }
                    } else {
                        part_of[p]
                    };
                }
            }
            if let Some(root) = doc
                .default_scene()
                .or_else(|| doc.scenes().next())
                .and_then(|s| s.nodes().next())
            {
                nodes[0].name = root.name().unwrap_or("").to_owned();
            }
        }
        let mut empties = Vec::new();
        let mut faces = Vec::new();
        for node in doc.nodes() {
            let Some(m) = node.mesh() else {
                if !any_skin && node.children().next().is_none() && parents[node.index()].is_some()
                {
                    empties.push(Empty {
                        name: node.name().unwrap_or("").to_owned(),
                        pos: world[node.index()].transform_point3(Vec3::ZERO),
                    });
                }
                continue;
            };
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
                if !skinned {
                    mesh.node
                        .extend(std::iter::repeat_n(part_of[node.index()], count));
                }
                let first_index = mesh.indices.len() as u32;
                match reader.read_indices() {
                    Some(ix) => mesh.indices.extend(ix.into_u32().map(|i| i + base)),
                    None => mesh.indices.extend(base..base + count as u32),
                }
                let slot = prim.material().name().unwrap_or("");
                if !skinned && slot.ends_with("_face") {
                    let range = base as usize..mesh.positions.len();
                    if let Some(f) = face_quad(slot, &mesh, range) {
                        faces.push(f);
                    }
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
                    image_index: pbr
                        .base_color_texture()
                        .map(|info| info.texture().source().index()),
                    name: mat.name().unwrap_or("").to_owned(),
                    emissive: mat.emissive_factor(),
                    blend: mat.alpha_mode() == gltf::material::AlphaMode::Blend,
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
                Some(
                    Skeleton::new(parents, rest, order, joints, inverse_bind).with_names(
                        doc.nodes()
                            .map(|n| n.name().unwrap_or("").to_owned())
                            .collect(),
                    ),
                )
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

        if any_skin {
            nodes.clear();
        }
        Ok(Model {
            mesh,
            base_color,
            image,
            materials,
            skeleton,
            clips,
            nodes,
            empties,
            faces,
        })
    }

    /// Position (model space) of an empty (`light`, `socket_note`, …).
    pub fn empty(&self, name: &str) -> Option<Vec3> {
        self.empties.iter().find(|e| e.name == name).map(|e| e.pos)
    }

    /// Index of a movable part by node name.
    pub fn node_index(&self, name: &str) -> Option<usize> {
        self.nodes.iter().position(|p| p.name == name)
    }

    pub fn clip(&self, name: &str) -> Option<&Clip> {
        self.clips.iter().find(|c| c.name == name)
    }
}

/// Whether a node or one of its descendants has a mesh.
fn subtree_has_mesh(doc: &gltf::Document, i: usize) -> bool {
    let Some(node) = doc.nodes().nth(i) else {
        return false;
    };
    node.mesh().is_some() || node.children().any(|c| subtree_has_mesh(doc, c.index()))
}

/// The text quad of a `*_face` primitive (vertices `range` of `mesh`): the vertices nearest
/// to the UV corners. glTF UVs run top-down (the exporter flips Blender's bottom-up v), so
/// UV (0, 0) is the top-left corner as the reader sees it.
fn face_quad(slot: &str, mesh: &MeshData, range: std::ops::Range<usize>) -> Option<Face> {
    if range.is_empty() {
        return None;
    }
    let corner = |u: f32, v: f32| -> Vec3 {
        let k = range
            .clone()
            .min_by(|&a, &b| {
                let da = (mesh.uvs[a][0] - u).powi(2) + (mesh.uvs[a][1] - v).powi(2);
                let db = (mesh.uvs[b][0] - u).powi(2) + (mesh.uvs[b][1] - v).powi(2);
                da.total_cmp(&db)
            })
            .unwrap_or(range.start);
        Vec3::from(mesh.positions[k])
    };
    let corners = [
        corner(0.0, 0.0),
        corner(1.0, 0.0),
        corner(1.0, 1.0),
        corner(0.0, 1.0),
    ];
    // right × up = towards the reader
    let normal = (corners[1] - corners[0])
        .cross(corners[0] - corners[3])
        .normalize_or_zero();
    Some(Face {
        slot: slot.to_owned(),
        corners,
        normal,
    })
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
