//! Every exported `.glb` loads with the runtime loader (APIPE-004/005 groundwork).

use std::path::PathBuf;

use zoo_assets::{find_files, Model};

/// Models whose origin is the water surface (fish rig, ducks — GAME-AMBIENT §4), not the ground.
const WATER_ORIGIN: &[&str] = &["goldfish", "duck", "duckling"];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn every_glb_loads() {
    let files = find_files(&root().join("assets/models"), &|p| {
        p.extension().is_some_and(|e| e == "glb")
    });
    assert!(!files.is_empty());
    for f in files {
        let bytes = std::fs::read(&f).unwrap();
        let m = Model::from_glb(&bytes).unwrap_or_else(|e| panic!("{}: {e}", f.display()));
        assert!(!m.mesh.indices.is_empty(), "{}", f.display());
        assert_eq!(m.mesh.positions.len(), m.mesh.normals.len());
        assert_eq!(m.mesh.positions.len(), m.mesh.uvs.len());
        let (lo, _) = m.mesh.bounds();
        // Swimming animals (fish rig, ART-ANIMALS) have their origin at the water surface
        // with the body below it; everything else stands on the ground.
        let in_animals = f.parent().is_some_and(|d| d.ends_with("animals"));
        if in_animals
            && f.file_stem()
                .is_some_and(|n| WATER_ORIGIN.contains(&n.to_string_lossy().as_ref()))
        {
            assert!(
                lo.y < 0.0,
                "{}: fish body below the water surface",
                f.display()
            );
        } else {
            assert!(
                lo.y.abs() <= 0.01,
                "{}: origin on the ground (APIPE-004: ±0.01 m), min y = {}",
                f.display(),
                lo.y
            );
        }
        if let Some(s) = &m.skeleton {
            assert!(m.mesh.is_skinned());
            assert_eq!(s.joints.len(), s.inverse_bind.len());
        }
    }
}

#[test]
fn props_share_the_palette_texture() {
    let m =
        Model::from_glb(&std::fs::read(root().join("assets/models/props/grass_tile.glb")).unwrap())
            .unwrap();
    let img = m.image.expect("embedded palette");
    assert_eq!(img.mime, "image/png");
}

fn load(rel: &str) -> Model {
    Model::from_glb(&std::fs::read(root().join(rel)).unwrap()).unwrap()
}

/// ARCH-006: multi-node assets keep their movable / hideable parts with the pivot, their
/// material slots (glow emission, glass blend), empties and text faces.
#[test]
fn arch_006_multi_node_assets_keep_parts_slots_empties_and_faces() {
    // every static model: one node index per vertex, part 0 = the root
    let files = find_files(&root().join("assets/models"), &|p| {
        p.extension().is_some_and(|e| e == "glb")
    });
    for f in files {
        let m = Model::from_glb(&std::fs::read(&f).unwrap()).unwrap();
        if m.skeleton.is_none() {
            assert_eq!(m.mesh.node.len(), m.mesh.positions.len(), "{}", f.display());
            assert!(!m.nodes.is_empty(), "{}", f.display());
            assert!(m.mesh.node.iter().all(|&k| (k as usize) < m.nodes.len()));
        }
    }
    // moon door: two leaves at their hinge pivots, lanterns as light empties, glow slots
    let door = load("assets/models/props/moon_door.glb");
    let leaf_l = door.node_index("leaf_l").expect("leaf_l");
    let leaf_r = door.node_index("leaf_r").expect("leaf_r");
    assert!(door.nodes[leaf_l]
        .pivot
        .abs_diff_eq(glam::Vec3::new(-0.96, 0.0, -0.04), 1e-3));
    assert!(door.nodes[leaf_r]
        .pivot
        .abs_diff_eq(glam::Vec3::new(0.96, 0.0, -0.04), 1e-3));
    for k in [leaf_l, leaf_r] {
        let n = door.mesh.node.iter().filter(|&&x| x as usize == k).count();
        assert!(n > 0, "leaf vertices");
    }
    let light = door.empty("light_l").expect("light_l");
    assert!(light.abs_diff_eq(glam::Vec3::new(-1.35, 3.62, 0.0), 1e-2));
    let glow: Vec<&str> = door
        .materials
        .iter()
        .filter(|m| m.is_glow())
        .map(|m| m.name.as_str())
        .collect();
    assert_eq!(glow, ["moon_glow", "rim_glow", "lamp_glow"]);
    let lamp = door
        .materials
        .iter()
        .find(|m| m.name == "lamp_glow")
        .unwrap();
    // linear emissive → sRGB #FFD66B-ish
    let c = lamp.emissive_srgb();
    assert!(
        (c[0] - 1.0).abs() < 0.02 && (c[1] - 0.84).abs() < 0.03,
        "{c:?}"
    );
    // buildings: roof and walls_upper parts; the night house has a glass part (BLEND)
    for b in ["zookeeper_house", "food_storage", "food_hut", "night_house"] {
        let m = load(&format!("assets/models/buildings/{b}.glb"));
        assert!(m.node_index("roof").is_some(), "{b} roof");
        assert!(m.node_index("walls_upper").is_some(), "{b} walls_upper");
    }
    let nh = load("assets/models/buildings/night_house.glb");
    assert!(nh.node_index("glass").is_some());
    assert!(nh.materials.iter().any(|m| m.is_glass()));
    assert!(nh.empty("light_hall").is_some());
    // text faces: the garden sign's face reads upright towards its front (+Z)
    let sign = load("assets/models/props/garden_sign.glb");
    let face = sign.faces.first().expect("sign_face");
    assert_eq!(face.slot, "sign_face");
    let [tl, tr, br, bl] = face.corners;
    assert!(
        tl.y > bl.y && tr.y > br.y,
        "top above bottom: {:?}",
        face.corners
    );
    assert!(tr.x > tl.x, "left to right: {:?}", face.corners);
    assert!(face.normal.z > 0.5, "faces the front: {}", face.normal);
    let note = load("assets/models/props/note_paper.glb");
    let f = note.faces.first().expect("note_face");
    assert!(f.normal.y > 0.9, "note face up: {}", f.normal);
    assert!(
        f.corners[0].z < f.corners[3].z,
        "top edge away from the reader (−Z)"
    );
    // animals: eye_glow shares the body's atlas image (uploaded once per asset)
    let owl = load("assets/models/animals/owl.glb");
    let body = owl.materials.iter().find(|m| m.name == "body").unwrap();
    let eye = owl.materials.iter().find(|m| m.is_eye_glow()).unwrap();
    assert!(body.image_index.is_some());
    assert_eq!(body.image_index, eye.image_index);
}

// GARD-023: the fruit trees of the level-3 fruit garden load, stay low-poly and grow from stage
// to stage.
#[test]
fn gard_023_fruit_tree_models_exist_load_and_grow() {
    for kind in ["apple", "orange"] {
        let mut heights = Vec::new();
        for stage in ["sprout", "young", "ripe"] {
            let m = load(&format!("assets/models/props/{kind}_tree_{stage}.glb"));
            let tris = m.mesh.indices.len() / 3;
            assert!(
                tris > 0 && tris <= 400,
                "{kind}_tree_{stage}: {tris} triangles"
            );
            let top = m
                .mesh
                .positions
                .iter()
                .map(|p| p[1])
                .fold(f32::MIN, f32::max);
            heights.push(top);
        }
        assert!(
            heights[0] < heights[1] && heights[1] < heights[2],
            "{kind}: sprout < young < ripe, got {heights:?}"
        );
        assert!(
            heights[2] > 1.3 && heights[2] < 1.8,
            "{kind}: ~1.5 m when ripe"
        );
    }
}

// AENV-013: the four `kit_landmarks_play` models (kiosk, carousel, slide, swings) exist, load,
// stay within 600 triangles, fit their level rects and the carousel has its `rotor` node.
#[test]
fn aenv_013_play_landmark_models() {
    // (model, rect w, rect d, max height)
    for (name, w, d, h) in [
        ("ice_cream_kiosk", 4.0, 3.0, 4.2),
        ("carousel", 4.0, 4.0, 4.2),
        ("playground_slide", 2.0, 3.0, 1.9),
        ("playground_swings", 4.0, 2.0, 2.3),
    ] {
        let m = load(&format!("assets/models/props/{name}.glb"));
        let tris = m.mesh.indices.len() / 3;
        assert!(tris > 50 && tris <= 600, "{name}: {tris} triangles");
        let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
        for p in &m.mesh.positions {
            for k in 0..3 {
                lo[k] = lo[k].min(p[k]);
                hi[k] = hi[k].max(p[k]);
            }
        }
        assert!(lo[1].abs() < 0.02, "{name}: origin at the feet");
        assert!(hi[1] <= h, "{name}: {} m high", hi[1]);
        assert!(
            hi[0] - lo[0] <= w + 0.15 && hi[2] - lo[2] <= d + 0.25,
            "{name}: fits {w} x {d} (got {} x {})",
            hi[0] - lo[0],
            hi[2] - lo[2]
        );
    }
    let carousel = load("assets/models/props/carousel.glb");
    let rotor = carousel.node_index("rotor").expect("carousel rotor node");
    assert!(
        carousel.nodes[rotor].pivot.length() < 0.01,
        "rotor turns about the centre"
    );
}

// AENV-016: the `kit_landmarks_l2` models (zoo train at its station, blossom tree) load, stay in
// their triangle budgets, stand on y = 0, fit their rects and keep their animated child nodes.
#[test]
fn aenv_016_l2_landmark_models() {
    // (model, max triangles, rect w, rect d, max height, nodes)
    for (name, max_tris, w, d, h, nodes) in [
        (
            "zoo_train",
            2200,
            8.0,
            2.0,
            2.5,
            &[
                "wheel_e0",
                "wheel_e1",
                "wheel_e2",
                "wheel_w1a",
                "wheel_w1b",
                "wheel_w2a",
                "wheel_w2b",
                "smoke",
            ][..],
        ),
        ("blossom_tree", 1800, 2.0, 2.0, 6.4, &["petals", "bees"][..]),
    ] {
        let m = load(&format!("assets/models/props/{name}.glb"));
        let tris = m.mesh.indices.len() / 3;
        assert!(tris > 300 && tris <= max_tris, "{name}: {tris} triangles");
        let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
        for p in &m.mesh.positions {
            for k in 0..3 {
                lo[k] = lo[k].min(p[k]);
                hi[k] = hi[k].max(p[k]);
            }
        }
        assert!(lo[1].abs() < 0.02, "{name}: origin at the feet");
        assert!(hi[1] <= h, "{name}: {} m high", hi[1]);
        // the tree crown is wider than its 2 x 2 rect (like the other trees); the train fits
        if name == "zoo_train" {
            assert!(
                hi[0] - lo[0] <= w + 0.2 && hi[2] - lo[2] <= d + 0.25,
                "{name}: fits {w} x {d}"
            );
        }
        for n in nodes {
            assert!(m.node_index(n).is_some(), "{name}: node {n}");
        }
    }
}
