//! Every exported `.glb` loads with the runtime loader (APIPE-004/005 groundwork).

use std::path::PathBuf;

use zoo_assets::{find_files, Model};

/// Models whose origin is the water surface (fish rig), not the ground.
const WATER_ORIGIN: &[&str] = &["goldfish"];

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
        if f.file_stem()
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
