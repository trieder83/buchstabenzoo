//! The round terrarium house of `night_2` (LAYOUT-N2-020, -021, -022; user request 2026-10-06):
//! a half-disc plan with a round hall and three glass cases fanned around it, and the animal
//! pictograms on its boards. (LAYOUT-N2-023 is an e2e screenshot test.)

mod common;

use std::collections::BTreeSet;

use glam::IVec2;
use zoo_core::level::{CellKind, ElementType, Level, Surface};
use zoo_core::nav::flood_fill;
use zoo_core::scene::{DecalImage, LevelScene, OpeningKind};

const HOUSE: &str = "terrarium_house";
const CASES: [(&str, &str); 3] = [
    ("enc_n2_snake", "snake"),
    ("enc_n2_chameleon", "chameleon"),
    ("enc_n2_frog", "poison_dart_frog"),
];

fn cells(set: impl IntoIterator<Item = IVec2>) -> BTreeSet<(i32, i32)> {
    set.into_iter().map(|c| (c.x, c.y)).collect()
}

// LAYOUT-N2-020: the plan is round (a union of rects), not the night house rectangle.
#[test]
fn layout_n2_020_round_plan() {
    let data = common::night2();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let house = data.element(HOUSE).unwrap();
    assert!(
        !house.footprint_extra.is_empty() && !house.interior_extra.is_empty(),
        "a round plan: extra footprint and interior rects"
    );
    let hall = cells(house.interior_cells());
    assert_eq!(hall.len(), 50, "round hall cells");
    let door = house.door_cell().unwrap();
    let footprint = cells(house.footprint_cells());
    let case_cells: BTreeSet<(i32, i32)> = CASES
        .iter()
        .flat_map(|(id, _)| cells(data.element(id).unwrap().rect.cells()))
        .collect();
    // disjoint parts
    assert!(footprint.is_disjoint(&hall), "footprint and hall overlap");
    assert!(
        footprint.is_disjoint(&case_cells),
        "footprint and cases overlap"
    );
    assert!(hall.is_disjoint(&case_cells), "hall and cases overlap");
    // every footprint cell except the door is solid wall; hall and door walk
    for &(x, z) in &footprint {
        let c = IVec2::new(x, z);
        if c == door {
            assert_eq!(grid.kind(c), CellKind::Walkable(Surface::Path), "door");
        } else {
            assert!(
                grid.kind(c) == CellKind::Solid,
                "footprint cell {c} must be solid"
            );
        }
    }
    for &(x, z) in &hall {
        assert_eq!(
            grid.kind(IVec2::new(x, z)),
            CellKind::Walkable(Surface::Path),
            "hall cell"
        );
    }
    // mirror symmetry about the door axis (x -> -170 - x)
    let mut all: BTreeSet<(i32, i32)> = BTreeSet::new();
    all.extend(&footprint);
    all.extend(&hall);
    all.extend(&case_cells);
    let mirrored: BTreeSet<(i32, i32)> = all.iter().map(|&(x, z)| (-170 - x, z)).collect();
    assert_eq!(all, mirrored, "the plan is mirror-symmetric about the door");
    let hall_mirrored: BTreeSet<(i32, i32)> = hall.iter().map(|&(x, z)| (-170 - x, z)).collect();
    assert_eq!(hall, hall_mirrored, "the hall is symmetric");
    // hall rows are >= 3 m wide everywhere
    let rows: BTreeSet<i32> = hall.iter().map(|&(_, z)| z).collect();
    for z in rows {
        let n = hall.iter().filter(|&&(_, hz)| hz == z).count();
        assert!(n >= 3, "hall row {z} only {n} m wide");
    }
    // the cases fan out around the hall: west, north, east - not in one row
    let (hx0, hx1) = (
        hall.iter().map(|c| c.0).min().unwrap(),
        hall.iter().map(|c| c.0).max().unwrap(),
    );
    let hz1 = hall.iter().map(|c| c.1).max().unwrap();
    let snake = data.element("enc_n2_snake").unwrap().rect;
    let chameleon = data.element("enc_n2_chameleon").unwrap().rect;
    let frog = data.element("enc_n2_frog").unwrap().rect;
    assert!(snake.x + snake.w <= hx0 + 1, "snake case west of the hall");
    assert!(frog.x >= hx1, "frog case east of the hall");
    assert!(chameleon.z > hz1, "chameleon case north of the hall");
    // the building rect is the facade row; the model bounds contain everything
    let model = house.model_rect.unwrap();
    for &(x, z) in &all {
        assert!(
            model.contains(IVec2::new(x, z)),
            "{x},{z} outside model_rect"
        );
    }
    assert_eq!(
        ElementType::Building,
        house.ty,
        "a building (LAYOUT-003 / -023 apply to the union)"
    );
    assert!(
        data.solid_overlaps().is_empty(),
        "{:?}",
        data.solid_overlaps()
    );
}

// LAYOUT-N2-021: never stuck - door, hall and gates reachable; the placeholder draws the round
// house (no night-house model), the doors and glass gates stand, the roof cuts away.
#[test]
fn layout_n2_021_reachable_and_drawn_round() {
    let mut level = Level::new(common::zoo_with_night2());
    assert!(level.open_barrier("barrier_n1_garden"));
    let data = level.data.clone();
    let grid = level.grid();
    let reach = flood_fill(grid, IVec2::new(-28, 29), false);
    let house = data.element(HOUSE).unwrap();
    let door = house.door_cell().unwrap();
    let reached = |c: IVec2| grid.index(c).is_some_and(|k| reach[k]);
    assert!(reached(door), "door reachable");
    for c in house.interior_cells() {
        assert!(reached(c), "hall cell {c} reachable over the door");
    }
    for (id, _) in CASES {
        let case = data.element(id).unwrap();
        let gate = case.gate.unwrap();
        let hall = cells(house.interior_cells());
        let next_to_hall = gate.cells().any(|c| {
            [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                .iter()
                .any(|d| hall.contains(&((c + *d).x, (c + *d).y)))
        });
        assert!(next_to_hall, "{id}: gate edge-adjacent to a hall cell");
        // the stand cells in front of the glass are walkable hall cells
        assert!(
            gate.cells()
                .flat_map(|c| [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y].map(|d| c + d))
                .any(|c| hall.contains(&(c.x, c.y)) && reached(c)),
            "{id}: no reachable stand cell"
        );
    }
    let scene = LevelScene::build(&data);
    // the door and the three glass gates (LAYOUT-031)
    assert!(
        scene.openings.iter().any(|o| matches!(&o.kind,
            OpeningKind::BuildingDoor { building, enterable: true } if building == HOUSE)),
        "door leaf"
    );
    for (id, _) in CASES {
        assert!(
            scene.openings.iter().any(|o| matches!(&o.kind,
                OpeningKind::GlassDoor { enclosure } if enclosure == id)),
            "{id}: glass door"
        );
    }
    // drawn by the placeholder, not by the rectangular night house model
    assert!(
        !scene.building_models.iter().any(|b| b.element == HOUSE),
        "the terrarium house must not use the night_house model"
    );
    let roof: usize = scene
        .roof_boxes
        .iter()
        .filter(|(id, _)| id == HOUSE)
        .map(|(_, r)| r.len())
        .sum();
    assert!(roof > 20, "stepped dome roof + upper walls (cut-away)");
}

// LAYOUT-N2-024: the floor inside is wooden planks, never grass.
#[test]
fn layout_n2_024_wooden_floor() {
    let data = common::night2();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let house = data.element(HOUSE).unwrap();
    let scene = LevelScene::build(&data);
    let floor: Vec<_> = scene
        .boxes
        .iter()
        .filter(|b| b.source == HOUSE && b.size.y < 0.2 && b.size.x == 1.0 && b.size.z == 1.0)
        .collect();
    let mut wanted = house.interior_cells();
    wanted.push(house.door_cell().unwrap());
    for c in &wanted {
        assert_eq!(
            grid.kind(*c),
            CellKind::Walkable(Surface::Path),
            "{c} is not grass"
        );
        let centre = zoo_core::level::cell_center(*c);
        let slab = floor
            .iter()
            .find(|b| {
                let p = zoo_core::coords::world_to_level(b.pos);
                (p - centre).length() < 0.01
            })
            .unwrap_or_else(|| panic!("{c}: no plank floor slab"));
        // plank brown (red > green > blue), clearly not grass green
        let [r, g, bl] = slab.color;
        assert!(
            r > g && g > bl && r > 0.6,
            "{c}: floor colour {:?}",
            slab.color
        );
        assert!(slab.size.y > 0.08, "{c}: slab covers the ground tile");
    }
    assert!(floor.len() >= wanted.len(), "a slab for every hall cell");
}

// LAYOUT-N2-022 (= AENV-017): the boards show the pictogram of their case's animal.
#[test]
fn layout_n2_022_sign_pictograms() {
    let data = common::night2();
    let scene = LevelScene::build(&data);
    // the house emblem over the door: the big snake
    assert!(
        scene
            .decals
            .iter()
            .any(|d| d.id == format!("sign:{HOUSE}:emblem")
                && d.image == DecalImage::Texture("textures/signs/silhouette_snake.png".into())
                && d.right.length() > 0.6),
        "big snake plaque over the door"
    );
    for (case, animal) in CASES {
        let path = format!("textures/signs/silhouette_{animal}.png");
        let file = std::fs::read(format!(
            "{}/../../assets/{path}",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap_or_else(|_| panic!("{path} exists"));
        assert!(
            file.len() > 3000 && file.starts_with(&[0x89, b'P', b'N', b'G']),
            "{path} is a real picture"
        );
        let board = data
            .elements
            .iter()
            .find(|b| {
                b.kind.as_deref() == Some("info_board") && b.enclosure.as_deref() == Some(case)
            })
            .unwrap();
        assert!(board.is_wall_board(), "{} is a wall board", board.id);
        for decal_id in [format!("sign:{}", board.id), format!("sign:{case}")] {
            let d = scene
                .decals
                .iter()
                .find(|d| d.id == decal_id)
                .unwrap_or_else(|| panic!("{decal_id}: pictogram decal"));
            assert_eq!(d.image, DecalImage::Texture(path.clone()), "{decal_id}");
        }
        // a cream plate behind each pictogram
        for src in [format!("{}:plate", board.id), format!("{case}:plate")] {
            assert!(
                scene.boxes.iter().any(|b| b.source == src),
                "{src}: cream plate"
            );
        }
    }
}
