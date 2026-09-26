//! Every exported `.glb` loads with the runtime loader (APIPE-004/005 groundwork).

use std::path::PathBuf;

use zoo_assets::{find_files, Model};

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
        assert!(lo.y.abs() < 1e-3, "{}: origin on the ground", f.display());
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
