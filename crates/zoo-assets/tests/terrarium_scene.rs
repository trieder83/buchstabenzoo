//! AENV-019 / LAYOUT-N2-025..027: the terrarium models of `night_2` are placed where the
//! placement table says (case rects, house frame), every placed model loads, the dressing is
//! non-solid and keeps the gates and stand cells free, and the placeholders stay as fallback.

use std::path::PathBuf;

use glam::{IVec2, Vec2};
use zoo_assets::Model;
use zoo_core::coords::world_to_level;
use zoo_core::level::{ElementType, LevelData};
use zoo_core::scene::{case_frame, model_path, LevelScene, Placement};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn night2() -> LevelData {
    let text = std::fs::read_to_string(root().join("assets/levels/night-2.toml")).unwrap();
    LevelData::from_toml_str(&text).unwrap()
}

fn load(model: &str) -> Model {
    let path = root().join("assets").join(model_path(model));
    Model::from_glb(&std::fs::read(&path).unwrap_or_else(|_| panic!("{} exists", path.display())))
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The terrarium models placed by the scene.
fn is_terrarium_model(m: &str) -> bool {
    m.starts_with("terrarium_") || matches!(m, "mist_puff" | "info_board_wall" | "fridge")
}

/// Level-space footprint (min, max) of a placed model's mesh.
fn footprint(p: &Placement, m: &Model) -> (Vec2, Vec2) {
    let (lo, hi) = m.mesh.bounds();
    let (s, c) = p.yaw.sin_cos();
    let at = world_to_level(p.pos);
    let mut min = Vec2::splat(f32::MAX);
    let mut max = Vec2::splat(f32::MIN);
    for x in [lo.x, hi.x] {
        for z in [lo.z, hi.z] {
            // world = pos + R(yaw) * (x, z) with the vertex shader's rotation; level z = -world z
            let w = Vec2::new(c * x + s * z, -s * x + c * z) * p.scale;
            let l = at + Vec2::new(w.x, -w.y);
            min = min.min(l);
            max = max.max(l);
        }
    }
    (min, max)
}

// LAYOUT-N2-025 (= AENV-019): the house model stands at (-84.5, 39.0) yaw 0 and every placed
// terrarium model exists and loads.
#[test]
fn layout_n2_025_house_and_models_load() {
    let data = night2();
    let scene = LevelScene::build(&data);
    let house = scene
        .placements
        .iter()
        .find(|p| p.model == "terrarium_house")
        .expect("terrarium_house placed");
    let at = world_to_level(house.pos);
    assert!((at - Vec2::new(-84.5, 39.0)).length() < 1e-3 && house.yaw == 0.0);
    let mut kinds: Vec<&str> = scene
        .placements
        .iter()
        .map(|p| p.model)
        .filter(|m| is_terrarium_model(m))
        .collect();
    kinds.sort_unstable();
    kinds.dedup();
    for want in [
        "terrarium_floor_snake",
        "terrarium_floor_frog",
        "terrarium_floor_chameleon",
        "terrarium_waterfall",
        "mist_puff",
        "terrarium_tree",
        "terrarium_branch",
        "terrarium_front",
        "terrarium_frame",
        "terrarium_frame_wide",
        "terrarium_lamp",
        "terrarium_lamp_uv",
        "terrarium_lamp_teal",
        "info_board_wall",
        "fridge",
    ] {
        assert!(kinds.contains(&want), "{want} is placed");
    }
    for m in kinds {
        let model = load(m);
        assert!(!model.mesh.indices.is_empty(), "{m}");
    }
    // the house keeps its sockets: the emblem decal sits in front of the plaque
    let model = load("terrarium_house");
    let e = model.empty("socket_emblem").unwrap();
    let d = scene
        .decals
        .iter()
        .find(|d| d.id == "sign:terrarium_house:emblem")
        .expect("emblem decal");
    let want = world_to_level(house.pos) + Vec2::new(e.x, -e.z);
    let got = world_to_level(d.center);
    assert!(
        (got - want).length() < 0.02,
        "emblem decal at socket_emblem: {got} vs {want}"
    );
    assert!(
        (d.center.y - e.y).abs() < 0.01,
        "emblem height {} vs {}",
        d.center.y,
        e.y
    );
}

// LAYOUT-N2-026: the case dressing lies inside its case (plus the thin back wall), the glass
// fronts leave the 2 m gate free, and no prop is solid or covers the gate / hall stand cells.
#[test]
fn layout_n2_026_case_props_inside_cases_gates_free() {
    let data = night2();
    let scene = LevelScene::build(&data);
    let cases: Vec<_> = data
        .elements_of(ElementType::Enclosure)
        .filter(|c| c.indoor && c.terrarium)
        .collect();
    assert_eq!(cases.len(), 3);
    for case in cases {
        let r = case.rect;
        let (lo, hi) = (
            Vec2::new(r.x as f32, r.z as f32),
            Vec2::new((r.x + r.w) as f32, (r.z + r.d) as f32),
        );
        let frame = case_frame(case).expect("case frame");
        let gate = case.gate.expect("gate");
        let (glo, ghi) = (
            Vec2::new(gate.x as f32, gate.z as f32),
            Vec2::new((gate.x + gate.w) as f32, (gate.z + gate.d) as f32),
        );
        let mut props = 0;
        for p in &scene.placements {
            if !is_terrarium_model(p.model) || p.model == "terrarium_house" {
                continue;
            }
            if matches!(p.model, "info_board_wall" | "fridge") {
                continue;
            }
            let model = load(p.model);
            let (min, max) = footprint(p, &model);
            let centre = (min + max) / 2.0;
            if centre.cmplt(lo - 0.5).any() || centre.cmpgt(hi + 0.5).any() {
                continue; // another case
            }
            props += 1;
            let glass = matches!(
                p.model,
                "terrarium_front" | "terrarium_frame" | "terrarium_frame_wide"
            );
            if glass {
                // on the glass line: it never reaches into the gate (2 m free)
                let (fmin, fmax) = (min + 0.02, max - 0.02);
                if p.model == "terrarium_front" {
                    let overlap =
                        fmin.x < ghi.x && fmax.x > glo.x && fmin.y < ghi.y && fmax.y > glo.y;
                    assert!(!overlap, "{} at {centre} stands in the gate", p.model);
                }
                continue;
            }
            // dressing: inside the case, the back wall (0.3 m) and the 0.4 m a leaf may lean over it
            let slack = 0.3 + 0.1;
            assert!(
                min.x >= lo.x - slack
                    && min.y >= lo.y - slack
                    && max.x <= hi.x + slack
                    && max.y <= hi.y + slack,
                "{} {min}..{max} outside case {}",
                p.model,
                case.id
            );
            // never on the hall side of the glass line
            let beyond =
                (centre - frame.floor).dot(frame.out) - (hi - lo).dot(frame.out.abs()) / 2.0;
            assert!(
                beyond < 0.0,
                "{} reaches the hall side of {}",
                p.model,
                case.id
            );
        }
        assert!(props >= 7, "{}: {props} pieces", case.id);
        // no solid footprint inside the case or its gate / stand cells: the dressing never
        // blocks a wander cell
        for s in &scene.box_colliders {
            if let zoo_core::collision::Shape::Box { c, .. } = s {
                assert!(
                    !(c.cmpge(lo).all() && c.cmple(hi).all()),
                    "{}: a solid box at {c} inside the case",
                    case.id
                );
            }
        }
        let _ = IVec2::ZERO;
    }
}

// LAYOUT-N2-027: the fridge stands in the food hut without a collider overlap, the three
// wall boards use `info_board_wall` with their pictogram on its plate, and the placeholder
// boxes of the house, the boards and the fridge stay as the fallback of their models.
#[test]
fn layout_n2_027_fridge_boards_fallbacks() {
    let data = night2();
    let scene = LevelScene::build(&data);
    let hut = data.element("food_storage_n2").unwrap();
    let fridge = scene
        .placements
        .iter()
        .find(|p| p.model == "fridge")
        .expect("fridge");
    let at = world_to_level(fridge.pos);
    assert!(
        hut.interior
            .is_some_and(|i| i.contains(IVec2::new(at.x.floor() as i32, at.y.floor() as i32))),
        "fridge {at} inside the hut"
    );
    for id in ["board_n2_snake", "board_n2_chameleon", "board_n2_frog"] {
        let b = data.element(id).unwrap();
        let animal = data
            .element(b.enclosure.as_deref().unwrap())
            .unwrap()
            .animal
            .clone()
            .unwrap();
        let d = scene
            .decals
            .iter()
            .find(|d| d.id == format!("sign:{id}"))
            .unwrap_or_else(|| panic!("{id}: pictogram decal"));
        assert_eq!(
            d.image,
            zoo_core::scene::DecalImage::Texture(format!("textures/signs/silhouette_{animal}.png"))
        );
        // the decal is on the model's plate: above the panel, near the facade
        let board_model = scene
            .placements
            .iter()
            .find(|p| {
                p.model == "info_board_wall"
                    && (world_to_level(p.pos) - Vec2::new(d.center.x, -d.center.z)).length() < 0.5
            })
            .unwrap_or_else(|| panic!("{id}: info_board_wall under the decal"));
        assert!(
            d.center.y > board_model.pos.y + 0.8,
            "{id}: on the plate above the panel"
        );
    }
    for m in ["terrarium_house", "info_board_wall", "fridge"] {
        assert!(
            scene
                .fallbacks
                .iter()
                .any(|f| f.model == m && !f.boxes.is_empty()),
            "{m}: fallback boxes"
        );
    }
}
