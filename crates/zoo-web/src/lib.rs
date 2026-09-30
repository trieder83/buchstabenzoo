//! wasm-bindgen entry point (TECH-ARCH): glue between the TypeScript host, `zoo-core`
//! (game logic) and `zoo-render` (WebGL2).
//!
//! The host fetches the files listed by [`required_assets`], hands them over as a
//! `Map<path, Uint8Array>` to [`App::new`] and then calls [`App::frame`] once per animation
//! frame, forwarding input with the `key` / `set_stick` / `drag` / `rotate` / `zoom` /
//! `interact` / `take_food` methods. Everything the host shows (interact button, text panel,
//! HUD, feedback) is decided here from `zoo-core`; texts come from Fluent.

use std::collections::{BTreeMap, VecDeque};

use glam::{Quat, Vec2, Vec3};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use zoo_assets::Model;
use zoo_core::ambient::{Ambient, AmbientPose, ButterflyPose, Ripple};
use zoo_core::animals::{swim_sink_m, AnimTable};
use zoo_core::coords::level_to_world;
use zoo_core::game::{Interaction, Target};
use zoo_core::level::ElementType;
use zoo_core::nav::Autopilot;
use zoo_core::night_scene::NightScene;
use zoo_core::player::walk_clip_rate;
use zoo_core::quality::{QualityGovernor, QualityMode, QualityTier};
use zoo_core::scene::{model_path, OpeningKind};
use zoo_core::view::{self as views, ViewMode};
use zoo_core::{AnimalState, Content, Food, Game, GameEvent, Language, LevelData, ReadingLevel};
use zoo_render::night::{self as nightfx, DayLight, PointLight};
use zoo_render::renderer::{
    butterfly_mesh, cylinder_mesh, InstanceHandle, CAPSULE, HIDE_ROOF, HIDE_WALLS_UPPER,
};
use zoo_render::scene::{model_placeholder, Decal, DecalImage};
use zoo_render::{CameraParams, CharacterDraw, FollowCamera, Instance, LevelScene, Renderer};

/// The player character model (GAME-PLAYER §1; `player_boy` later).
pub const PLAYER_MODEL: &str = "player_girl";
/// Clip data of the animal models (ART-ANIMALS "Clips").
pub const ANIMAL_ANIMS: &str = "models/animals/animal_anims.toml";
const MARKER: &str = "__marker";
/// Batch of flat boxes that move (animal placeholders).
const DYN_BOX: &str = "__dyn_box";
/// Second batch of the food box model for the carried box.
const CARRY_BOX: &str = "__carry_food_box";
/// Horizontal mouse drag distance (CSS px) for one 45° camera step.
const DRAG_STEP_PX: f32 = 70.0;
/// Glass fish bowl and its water (placeholder meshes built here, GAME-RESCUE "goldfish bowl").
const BOWL: &str = "__bowl_glass";
const BOWL_WATER: &str = "__bowl_water";
/// Bowl size (m): radius and height of the glass.
const BOWL_RADIUS_M: f32 = 0.3;
/// The procedural water-wheel model (LAYOUT-L3-017).
const WATER_WHEEL: &str = "__water_wheel";
/// Lying foods show their icon above them within this distance of the player (GAME-FEED §10).
const LYING_ICON_M: f32 = 5.0;
const BOWL_HEIGHT_M: f32 = 0.36;
/// Glass colour; alpha 0.5 = screen-door transparency in the cel shader.
const GLASS: [f32; 4] = [0.78, 0.92, 0.98, 0.5];
const BOWL_WATER_COLOR: [f32; 4] = [0.36, 0.66, 0.90, 0.5];
/// Descending speed from a perch when the animal has no `climb_speed` (m/s).
const DESCEND_SPEED: f32 = 1.8;
/// Vertical speed of a flyer between its perch and its flying height (m/s).
const FLY_CLIMB_SPEED: f32 = 1.2;
/// Duration of the goldfish's leap into / out of the bowl (s).
const LEAP_S: f32 = 0.9;
/// Ambient animal models (GAME-AMBIENT), drawn as instanced skinned crowds.
pub const AMBIENT_MODELS: [&str; 3] = ["duck", "duckling", "frog"];
/// Seed of the ambient animals (not saved; restart naturally, GAME-AMBIENT 9).
const AMBIENT_SEED: u64 = 7;
/// Built-in butterfly mesh (dynamic instances, one draw call).
const BUTTERFLY: &str = "__butterfly";
/// Dynamic emissive boxes (eyeshine, fireflies; GAME-NIGHT §10), one draw call.
const DYN_GLOW: &str = "__dyn_glow";
/// Fireflies per firefly area (Q-115).
const FIREFLIES_PER_AREA: usize = 6;
/// The hand lantern the player carries at night (`kit_night`, GAME-NIGHT rule 1).
const HAND_LANTERN: &str = "hand_lantern";
/// `hand_lantern.socket_handle` (glTF): the grip that coincides with her hand.
const HAND_LANTERN_GRIP: Vec3 = Vec3::new(0.0, 0.385, 0.0);
/// Garden plant models per growth stage (`kit_garden`; `empty` = no model).
const PLANT_MODELS: [[&str; 3]; 2] = [
    [
        "carrot_plant_sprout",
        "carrot_plant_young",
        "carrot_plant_ripe",
    ],
    [
        "potato_plant_sprout",
        "potato_plant_young",
        "potato_plant_ripe",
    ],
];
/// Models the presentation places itself (not in the level scene).
const EXTRA_MODELS: [&str; 7] = [
    HAND_LANTERN,
    "carrot_plant_sprout",
    "carrot_plant_young",
    "carrot_plant_ripe",
    "potato_plant_sprout",
    "potato_plant_young",
    "potato_plant_ripe",
];
/// Gates and doors: opening / closing speed (open amount per second) and how long an
/// enclosure gate stays open behind the animals (s).
const OPEN_RATE: f32 = 2.2;
const CLOSE_RATE: f32 = 1.2;
const GATE_HOLD_S: f32 = 1.5;

/// Asset path of an animal model.
pub fn animal_model_path(animal: &str) -> String {
    format!("models/animals/{animal}.glb")
}

/// Animals of the (joined) levels: the animal of every enclosure in scope of its level
/// (`[level] missions`, Q-069).
pub fn level_animals(data: &LevelData) -> Vec<String> {
    data.elements_of(ElementType::Enclosure)
        .filter_map(|e| {
            let a = e.animal.clone()?;
            let scope = data.parts.get(e.part).map(|p| &p.missions);
            scope.is_none_or(|m| m.contains(&a)).then_some(a)
        })
        .collect()
}

/// Joins the level files (GAME-LAYOUT "Joining levels", proposal Q-088).
pub fn join_levels(tomls: &[String]) -> Result<LevelData, String> {
    let mut levels = Vec::new();
    for t in tomls {
        levels.push(LevelData::from_toml_str(t).map_err(|e| e.to_string())?);
    }
    LevelData::join(levels).map_err(|e| e.to_string())
}

/// Asset paths (relative to the served `assets/` folder) needed for the levels (TOML texts,
/// joined in this order): palette, every model the scene places, the player and every
/// animal. Missing models become placeholders.
#[wasm_bindgen]
pub fn required_assets(level_tomls: Vec<String>) -> Result<Vec<String>, JsError> {
    let data = join_levels(&level_tomls).map_err(|e| JsError::new(&e))?;
    let scene = LevelScene::build(&data);
    let night = NightScene::build(&data);
    let mut models: Vec<&str> = scene
        .placements
        .iter()
        .chain(&night.placements)
        .map(|p| p.model)
        .chain(EXTRA_MODELS)
        .collect();
    models.sort_unstable();
    models.dedup();
    let mut out = vec!["textures/palette.png".to_owned()];
    out.extend(models.iter().map(|m| model_path(m)));
    // decal images (enclosure sign silhouettes); missing ones leave the panel blank
    for d in &scene.decals {
        if let DecalImage::Texture(path) = &d.image {
            if !out.contains(path) {
                out.push(path.clone());
            }
        }
    }
    out.push(format!("models/characters/{PLAYER_MODEL}.glb"));
    out.extend(level_animals(&data).iter().map(|a| animal_model_path(a)));
    // pairs (GAME-FAMILY): the female and the baby model of each pair enclosure
    for e in data.elements_of(ElementType::Enclosure).filter(|e| e.pair) {
        if let Some(a) = e.animal.as_deref() {
            for m in [
                zoo_core::animals::female_model(a),
                zoo_core::animals::baby_model(a),
            ]
            .into_iter()
            .flatten()
            {
                let path = animal_model_path(m);
                if !out.contains(&path) {
                    out.push(path);
                }
            }
        }
    }
    out.extend(AMBIENT_MODELS.iter().map(|a| animal_model_path(a)));
    out.push(ANIMAL_ANIMS.to_owned());
    Ok(out)
}

#[derive(Debug, Default, Clone, Copy)]
struct Keys {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    /// Arrow keys ← → (walk sideways in the zoo view, turn in first person).
    arrow_left: bool,
    arrow_right: bool,
}

/// Presentation state of one animal (GAME-RESCUE §11): smoothed position, facing, clips.
struct AnimalView {
    id: &'static str,
    /// Renderer model / clip table id: the species, or its female / baby model (GAME-FAMILY).
    model: &'static str,
    skinned: bool,
    pos: Vec2,
    yaw: f32,
    idle_time: f32,
    walk_time: f32,
    walk_blend: f32,
    /// Current one-shot clip: name, time, duration.
    action: Option<(&'static str, f32, f32)>,
    queue: VecDeque<&'static str>,
    /// Actions wait until the view reached the logic position (walking into the enclosure).
    wait_arrival: bool,
    /// Resting clip (hiding-place pose, `swim` in water, else `idle`; falls back to `idle`).
    rest: &'static str,
    /// Locomotion clip (`swim` in water, else `walk`).
    locomotion: &'static str,
    /// Current depth below the land pose (m), eased (swimmers in water).
    sink: f32,
    /// Height above the ground (m): up in a perch, eased down when it follows (Q-094).
    lift: f32,
    /// Ground height under it (m, GAME-PLAYER 8), eased (0 while perched: the perch height
    /// is absolute).
    ground: f32,
    /// Leap arc (goldfish into / out of the bowl): start, time, whether into the bowl.
    leap: Option<(Vec3, f32, bool)>,
    /// Drawn at all (its level is unlocked).
    visible: bool,
    /// Drawn through the water surface (the goldfish in the stream / pond).
    under_water: bool,
    /// Drawn here instead (world): the fish in the bowl.
    anchor: Option<Vec3>,
    /// Yaw of a perched animal (towards its tree / mast).
    perch_yaw: Option<f32>,
}

impl AnimalView {
    fn new(id: &'static str, skinned: bool) -> Self {
        Self {
            id,
            model: id,
            skinned,
            pos: Vec2::ZERO,
            yaw: 0.0,
            idle_time: 0.0,
            walk_time: 0.0,
            walk_blend: 0.0,
            action: None,
            queue: VecDeque::new(),
            wait_arrival: false,
            rest: "idle",
            locomotion: "walk",
            sink: 0.0,
            lift: 0.0,
            ground: 0.0,
            leap: None,
            visible: true,
            under_water: false,
            anchor: None,
            perch_yaw: None,
        }
    }
}

/// A gate / door model in an opening (LAYOUT-031): its instance as placed, how it swings,
/// the eased open amount and how long it stays open behind the animals.
struct OpeningView {
    handle: Option<InstanceHandle>,
    base: Instance,
    /// Yaw at full open (whole-model swing), or 0 = its leaf parts open.
    swing: f32,
    open: f32,
    hold: f32,
    enclosure_gate: bool,
}

/// A garden plant spot: its model per growth stage (sprout, young, ripe) and the one shown.
struct PlantView {
    spot: String,
    stages: [Option<InstanceHandle>; 3],
    base: [Instance; 3],
    shown: Option<usize>,
}

/// Maps a clip name from the game data to a static name the renderer knows (unknown → idle).
fn static_clip(name: &str) -> &'static str {
    const CLIPS: [&str; 15] = [
        "idle", "walk", "drink", "eat", "sleep", "swim", "happy", "refuse", "climb", "roll",
        "perch", "hang", "fly", "look", "hop",
    ];
    CLIPS.iter().copied().find(|c| *c == name).unwrap_or("idle")
}

/// A running game with its renderer, owned by the host page.
#[wasm_bindgen]
pub struct App {
    game: Game,
    renderer: Renderer,
    camera: FollowCamera,
    keys: Keys,
    stick: Vec2,
    drag_acc: f32,
    content: Option<Content>,
    placeholders: BTreeMap<String, usize>,
    player_skinned: bool,
    player_yaw: f32,
    idle_time: f32,
    walk_time: f32,
    walk_blend: f32,
    dynamic: [Instance; 1],
    marker: [Instance; 1],
    dyn_boxes: Vec<Instance>,
    /// The carried food box (first) and the lying foods on the ground (GAME-FEED §10).
    carry_box: Vec<Instance>,
    has_carry_model: bool,
    animals: Vec<AnimalView>,
    /// Animal clip data (authored walk speeds).
    anims: AnimTable,
    /// Characters drawn this frame (reused, no per-frame allocation once grown).
    draws: Vec<(&'static str, CharacterDraw)>,
    /// Baby models that loaded (GAME-FAMILY §5).
    baby_models: Vec<&'static str>,
    autopilot: Option<Autopilot>,
    /// Next-target hint and idle nudge (GAME-HINT, `zoo_core::hints`).
    hints: zoo_core::hints::HintTracker,
    outbox: Vec<String>,
    time: f64,
    /// Decals of the level scene (debug getters, AENV-011/012).
    decals: Vec<Decal>,
    /// Text textures the host renders: (texture id, Fluent key, width, height).
    text_textures: Vec<(String, String, u32, u32)>,
    /// Text textures must be (re-)rendered (start, language change).
    text_dirty: bool,
    /// Render regions of barriers (hidden once open) and roofs (hidden while inside).
    barrier_regions: Vec<(String, u16)>,
    roof_regions: Vec<(String, u16)>,
    /// Fish bowl instances (glass, water).
    bowl_glass: [Instance; 1],
    bowl_water: [Instance; 1],
    /// Building the player is inside (roof hidden), if any.
    inside: Option<String>,
    /// Ambient ducks, ducklings and frogs (GAME-AMBIENT) and their per-frame buffers.
    ambient: Ambient,
    ambient_poses: Vec<AmbientPose>,
    ripples: Vec<Ripple>,
    crowd: Vec<(&'static str, CharacterDraw)>,
    butterfly_poses: Vec<ButterflyPose>,
    butterflies: Vec<Instance>,
    /// Debug/e2e: the clock stands still (frames render, `debug_step` advances time).
    paused: bool,
    /// Debug/e2e: the camera looks at this level point instead of the player.
    look_at: Option<Vec2>,
    /// Debug/e2e: ambient animals simulated and drawn (AMB-007 A/B measurement).
    ambient_on: bool,
    /// Night (GAME-NIGHT §10): night-only render regions (lamp props, glowing windows) per
    /// level part, every lamp's light, per-frame light scratch, emissive dynamic boxes.
    night_regions: Vec<u16>,
    /// Turning water wheels (LAYOUT-L3-017), for the debug getter.
    water_wheels: Vec<zoo_core::scene::WaterWheel>,
    lamps: Vec<PointLight>,
    lamp_order: Vec<(f32, usize)>,
    /// Automatic quality tier (PERF-BUDGETS rule 5, PERF-R-005).
    quality: QualityGovernor,
    frame_lights: Vec<PointLight>,
    frame_pools: Vec<PointLight>,
    glows: Vec<Instance>,
    /// Fireflies: centre of the dance (world), phase.
    fireflies: Vec<(Vec3, f32)>,
    /// Gates and doors (index = `Level::openings`).
    openings: Vec<OpeningView>,
    /// Buildings drawn by their model: element id, instance (roof / upper walls hide).
    building_models: Vec<(String, InstanceHandle, Instance)>,
    /// Garden plants.
    plants: Vec<PlantView>,
    /// The hand lantern at night (dynamic instance).
    hand_lantern: [Instance; 1],
}

#[wasm_bindgen]
impl App {
    /// Builds the level scene and uploads it. `assets` maps paths relative to `assets/`
    /// (e.g. `levels/level-1.toml`, `models/props/hedge.glb`) to their bytes.
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        level_paths: Vec<String>,
        assets: &js_sys::Map,
    ) -> Result<App, JsError> {
        console_error_panic_hook::set_once();
        let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        assets.for_each(&mut |v, k| {
            if let (Some(k), Ok(arr)) = (k.as_string(), v.dyn_into::<js_sys::Uint8Array>()) {
                files.insert(k, arr.to_vec());
            }
        });
        let mut tomls = Vec::new();
        for path in &level_paths {
            let bytes = files
                .get(path)
                .ok_or_else(|| JsError::new(&format!("missing {path}")))?;
            tomls.push(String::from_utf8_lossy(bytes).into_owned());
        }
        let data = join_levels(&tomls).map_err(|e| JsError::new(&e))?;
        let scene = LevelScene::build(&data);
        let ambient = Ambient::new(&data, &scene, AMBIENT_SEED);
        let game = Game::new(data, 1).map_err(|e| JsError::new(&e.to_string()))?;

        let mut renderer = Renderer::new(canvas).map_err(|e| JsError::new(&format!("{e:?}")))?;
        let box_mesh = zoo_render::renderer::box_mesh();
        for name in [MARKER, DYN_BOX] {
            renderer
                .add_mesh(name, &box_mesh, 1.0)
                .map_err(|e| JsError::new(&format!("{e:?}")))?;
        }
        renderer
            .add_mesh(BOWL, &cylinder_mesh(12, false, true), 1.0)
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        renderer
            .add_mesh(BUTTERFLY, &butterfly_mesh(), 1.0)
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        renderer
            .add_mesh(DYN_GLOW, &box_mesh, 1.0)
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        renderer
            .add_mesh(BOWL_WATER, &cylinder_mesh(12, true, false), 1.0)
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        if let Some(png) = files.get("textures/palette.png") {
            renderer
                .set_palette_png(png)
                .map_err(|e| JsError::new(&format!("{e:?}")))?;
        }
        // Living water (TECH-WATER): the baked field and the foam obstacles (Q-068).
        renderer
            .set_water_field(&zoo_core::water::WaterField::bake(&scene.water))
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        let obstacles: Vec<(Vec2, f32, f32, f32, bool)> = scene
            .water
            .obstacles
            .iter()
            .map(|o| {
                let (s, c) = scene
                    .water
                    .river_of(zoo_core::level::cell_of(o.pos))
                    .map_or((0.0, 0.0), |r| scene.water.rivers[r].coords(o.pos));
                let w = level_to_world(o.pos);
                (Vec2::new(w.x, w.z), o.radius, s, c, o.river)
            })
            .collect();
        renderer.set_water_obstacles(&obstacles);

        // Static props and buildings: one instanced batch per model (its moving parts, glow
        // slots and glass included, ARCH-006/007). Ground tiles get no normal edges.
        let mut has_carry_model = false;
        let mut faces: BTreeMap<String, Vec<zoo_assets::Face>> = BTreeMap::new();
        for (path, bytes) in &files {
            let Some(name) = path
                .strip_prefix("models/props/")
                .or_else(|| path.strip_prefix("models/buildings/"))
                .and_then(|s| s.strip_suffix(".glb"))
            else {
                continue;
            };
            match Model::from_glb(bytes) {
                Ok(m) => {
                    if !m.faces.is_empty() {
                        faces.insert(name.to_owned(), m.faces.clone());
                    }
                    let ground = name.ends_with("_tile") || name == "path_edge";
                    renderer
                        .add_model(name, &m, if ground { 0.0 } else { 1.0 })
                        .map_err(|e| JsError::new(&format!("{e:?}")))?;
                    if name == "food_box" {
                        renderer
                            .add_model(CARRY_BOX, &m, 1.0)
                            .map_err(|e| JsError::new(&format!("{e:?}")))?;
                        has_carry_model = true;
                    }
                }
                Err(e) => warn(&format!("{path}: {e} — using a placeholder")),
            }
        }
        let mut add_skinned = |name: &str, path: &str| -> bool {
            let Some(bytes) = files.get(path) else {
                return false;
            };
            match Model::from_glb(bytes)
                .map_err(|e| e.to_string())
                .and_then(|m| renderer.add_skinned(name, &m).map_err(|e| format!("{e:?}")))
            {
                Ok(()) => true,
                Err(e) => {
                    warn(&format!("{name}: {e} — using a placeholder"));
                    false
                }
            }
        };
        let player_skinned = add_skinned(
            PLAYER_MODEL,
            &format!("models/characters/{PLAYER_MODEL}.glb"),
        );
        // every animal of the level: its skinned model, else a placeholder (GAME-RESCUE §11)
        let mut animals = Vec::new();
        for a in &game.animals {
            let id = a.id();
            // the female of a pair has her own model (GAME-FAMILY §3), else the species'
            let model = match (a.member, zoo_core::animals::female_model(id)) {
                (1, Some(f)) => f,
                _ => id,
            };
            let mut load = |m: &'static str, animals: &Vec<AnimalView>| match animals
                .iter()
                .find(|v: &&AnimalView| v.model == m)
            {
                Some(v) => v.skinned,
                None => add_skinned(m, &animal_model_path(m)),
            };
            let mut skinned = load(model, &animals);
            let mut model = model;
            if !skinned && model != id {
                // no female model in the bundle: she shares the male's
                model = id;
                skinned = load(id, &animals);
            }
            let mut view = AnimalView::new(id, skinned);
            view.model = model;
            animals.push(view);
        }
        // babies (GAME-FAMILY §5): the model of every pair species
        let mut baby_models = Vec::new();
        for a in game.animals.iter().filter(|a| a.member == 1) {
            if let Some(b) = zoo_core::animals::baby_model(a.id()) {
                if !baby_models.contains(&b) && add_skinned(b, &animal_model_path(b)) {
                    baby_models.push(b);
                }
            }
        }
        for m in AMBIENT_MODELS {
            add_skinned(m, &animal_model_path(m));
        }
        let anims = files
            .get(ANIMAL_ANIMS)
            .and_then(|b| AnimTable::from_toml_str(&String::from_utf8_lossy(b)).ok())
            .unwrap_or_default();

        // Render regions (QA F12): one per level part, one per barrier (hidden once open),
        // one per enterable building's roof (hidden while the player is inside).
        let part_regions: Vec<u16> = (0..game.level.data.parts.len())
            .map(|_| renderer.add_region())
            .collect();
        let barrier_regions: Vec<(String, u16)> = scene
            .barrier_parts
            .iter()
            .map(|(id, _)| (id.clone(), renderer.add_region()))
            .collect();
        let roof_regions: Vec<(String, u16)> = scene
            .roof_boxes
            .iter()
            .map(|(id, _)| (id.clone(), renderer.add_region()))
            .collect();
        let placement_region = |i: usize, part: u8| -> u16 {
            scene
                .barrier_parts
                .iter()
                .zip(&barrier_regions)
                .find(|((_, r), _)| r.contains(&i))
                .map_or(part_regions[part as usize], |(_, (_, reg))| *reg)
        };
        let box_region = |i: usize, part: u8| -> u16 {
            if let Some((_, (_, reg))) = scene
                .barrier_boxes
                .iter()
                .zip(&barrier_regions)
                .find(|((_, r), _)| r.contains(&i))
            {
                return *reg;
            }
            if let Some((_, (_, reg))) = scene
                .roof_boxes
                .iter()
                .zip(&roof_regions)
                .find(|((_, r), _)| r.contains(&i))
            {
                return *reg;
            }
            part_regions[part as usize]
        };
        let mut placeholders: BTreeMap<String, usize> = BTreeMap::new();
        // static batching (ARCH-008): groups that never move become one mesh each
        let mut baked = vec![false; scene.placements.len()];
        for g in &scene.bake_groups {
            let members: Vec<usize> = g
                .placements
                .iter()
                .copied()
                .filter(|&i| renderer.can_bake(scene.placements[i].model))
                .collect();
            if members.len() < 2 {
                continue;
            }
            let items: Vec<(&str, Vec3, f32, Vec3)> = members
                .iter()
                .map(|&i| {
                    let p = &scene.placements[i];
                    let sc = Vec3::new(p.scale * p.stretch, p.scale, p.scale);
                    (p.model, p.pos, p.yaw, sc)
                })
                .collect();
            let region = placement_region(members[0], g.part);
            if renderer
                .bake(&g.name, region, &items)
                .map_err(|e| JsError::new(&format!("{e:?}")))?
            {
                for i in members {
                    baked[i] = true;
                }
            }
        }
        let mut handles: Vec<Option<InstanceHandle>> = Vec::with_capacity(scene.placements.len());
        for (i, p) in scene.placements.iter().enumerate() {
            let region = placement_region(i, p.part);
            if baked[i] {
                handles.push(None);
                continue;
            }
            let h = renderer.add_instance_handle(p.model, region, p.pos, p.yaw, p.scale);
            handles.push(h);
            if let Some(h) = h {
                stretch(&mut renderer, h, p.stretch);
            } else {
                *placeholders.entry(p.model.to_owned()).or_default() += 1;
                if scene.fallbacks.iter().any(|f| f.model == p.model) {
                    continue; // drawn by its fallback geometry below
                }
                let (offset, size, color) = model_placeholder(p.model);
                let size = size * p.scale;
                let pos = p.pos + glam::Quat::from_rotation_y(p.yaw) * offset * p.scale;
                renderer.add_box_in(region, pos, size, p.yaw, color, pos.y + size.y > 1.5);
            }
        }
        // gates and doors (LAYOUT-031), building models (roof hiding), garden plants
        let openings: Vec<OpeningView> = scene
            .openings
            .iter()
            .map(|o| {
                let handle = handles[o.placement];
                OpeningView {
                    handle,
                    base: handle
                        .and_then(|h| renderer.instance(h))
                        .unwrap_or_else(|| Instance::model(Vec3::ZERO, 0.0, false)),
                    swing: o.swing,
                    open: 0.0,
                    hold: 0.0,
                    enclosure_gate: matches!(
                        o.kind,
                        OpeningKind::EnclosureGate { .. } | OpeningKind::GlassDoor { .. }
                    ),
                }
            })
            .collect();
        let building_models: Vec<(String, InstanceHandle, Instance)> = scene
            .building_models
            .iter()
            .filter_map(|b| {
                let h = handles[b.placement]?;
                Some((b.element.clone(), h, renderer.instance(h)?))
            })
            .collect();
        let mut plants = Vec::new();
        for pl in &scene.plants {
            let kind = usize::from(pl.kind == "potato");
            let region = part_regions[(pl.part as usize).min(part_regions.len() - 1)];
            let mut stages = [None; 3];
            let mut base = [Instance::model(pl.pos, 0.0, false); 3];
            for (k, m) in PLANT_MODELS[kind].iter().enumerate() {
                // a little turn per spot so the rows do not look copied
                let yaw = (pl.pos.x * 7.3 + pl.pos.z * 3.1).sin() * 0.6;
                stages[k] = renderer.add_instance_handle(m, region, pl.pos, yaw, 1.0);
                if let Some(h) = stages[k] {
                    base[k] = renderer.instance(h).unwrap_or(base[k]);
                    let mut hidden = base[k];
                    hidden.scale_fade = [0.0; 4];
                    renderer.set_instance(h, hidden);
                }
            }
            plants.push(PlantView {
                spot: pl.spot.clone(),
                stages,
                base,
                shown: None,
            });
        }
        // placeholder geometry of missing models (e.g. the tiled pool rim, `pool_tiled`)
        for f in &scene.fallbacks {
            if renderer.has_model(f.model) {
                continue;
            }
            for b in &f.boxes {
                let region = part_regions[b.part as usize];
                renderer.add_box_in(region, b.pos, b.size, b.yaw, b.color, b.fadeable);
            }
            for p in &f.placements {
                let region = part_regions[p.part as usize];
                renderer.add_instance_in(p.model, region, p.pos, p.yaw, p.scale);
            }
        }
        for (i, b) in scene.boxes.iter().enumerate() {
            let region = box_region(i, b.part);
            renderer.add_box_in(region, b.pos, b.size, b.yaw, b.color, b.fadeable);
            *placeholders
                .entry(format!("element:{}", b.source))
                .or_default() += 1;
        }
        // Text faces of placed models (entrance board, garden signs): text decals on them.
        let mut decals = scene.decals.clone();
        for tf in &scene.text_faces {
            let p = &scene.placements[tf.placement];
            let Some(f) = faces
                .get(p.model)
                .and_then(|v| v.iter().find(|f| f.slot == tf.slot))
            else {
                continue;
            };
            let rot = Quat::from_rotation_y(p.yaw);
            let w = |c: Vec3| p.pos + rot * (c * p.scale);
            let [tl, tr, br, bl] = f.corners.map(w);
            let n = rot * f.normal;
            let center = (tl + tr + br + bl) / 4.0 + n * zoo_core::scene::DECAL_LIFT_M;
            let right = (tr - tl) / 2.0;
            let up = (tl - bl) / 2.0;
            let aspect = up.length() / right.length().max(1e-3);
            decals.push(Decal {
                id: tf.id.clone(),
                image: DecalImage::Text {
                    key: tf.key.clone(),
                    width_px: 512,
                    height_px: ((512.0 * aspect).round() as u32).clamp(32, 512),
                },
                center,
                right,
                up,
            });
        }
        // Decals: sign silhouettes (image files) and sign texts (rendered by the host).
        let mut text_textures: Vec<(String, String, u32, u32)> = Vec::new();
        for d in &decals {
            let texture = match &d.image {
                DecalImage::Texture(path) => {
                    if !renderer.has_decal_texture(path) {
                        let Some(png) = files.get(path) else {
                            continue; // no silhouette yet: the panel stays blank
                        };
                        if let Err(e) = renderer.set_decal_texture_png(path, png) {
                            warn(&format!("{path}: {e:?}"));
                            continue;
                        }
                    }
                    path.clone()
                }
                DecalImage::Text {
                    key,
                    width_px,
                    height_px,
                } => {
                    let id = text_texture_id(key);
                    if !text_textures.iter().any(|(t, ..)| *t == id) {
                        text_textures.push((id.clone(), key.clone(), *width_px, *height_px));
                    }
                    id
                }
            };
            // a building's name board hides with its roof (inside, zoo view)
            let region = roof_regions
                .iter()
                .find(|(id, _)| d.id.strip_prefix("sign:") == Some(id.as_str()))
                .map_or(zoo_render::renderer::REGION_ALWAYS, |(_, r)| *r);
            renderer.add_decal_in(&texture, d.corners(), d.normal(), region);
        }

        // Water wheels turning in the water (LAYOUT-L3-017): a procedural placeholder model
        // with a spinning `wheel` part until `mill_hut_wheel` has an approved concept.
        for w in &scene.water_wheels {
            if !renderer.has_model(WATER_WHEEL) {
                renderer
                    .add_model(WATER_WHEEL, &water_wheel_model(w), 1.0)
                    .map_err(|e| JsError::new(&format!("{e:?}")))?;
            }
            let region = part_regions[w.part.min(part_regions.len() - 1)];
            let pos = level_to_world(w.center) + Vec3::Y * zoo_core::ground::WATER_TOP_M;
            renderer.add_instance_handle(WATER_WHEEL, region, pos, 0.0, 1.0);
        }

        // Night-only parts (GAME-NIGHT §10): lamp props (placeholders until `kit_night`),
        // glowing windows, board lamps, the moon sign — hidden by day.
        let night = NightScene::build(&game.level.data);
        let night_regions: Vec<u16> = (0..game.level.data.parts.len())
            .map(|_| renderer.add_region())
            .collect();
        for b in &night.boxes {
            let region = night_regions[(b.part as usize).min(night_regions.len() - 1)];
            renderer.add_box_in(region, b.pos, b.size, b.yaw, b.color, false);
        }
        for b in &night.glows {
            let region = night_regions[(b.part as usize).min(night_regions.len() - 1)];
            renderer.add_glow_box_in(region, b.pos, b.size, b.yaw, b.color);
        }
        // lamp models (`kit_night`): lantern posts, string lights, wall and board lamps
        for p in &night.placements {
            let region = night_regions[(p.part as usize).min(night_regions.len() - 1)];
            match renderer.add_instance_handle(p.model, region, p.pos, p.yaw, p.scale) {
                Some(h) => stretch(&mut renderer, h, p.stretch),
                None => *placeholders.entry(p.model.to_owned()).or_default() += 1,
            }
        }
        for r in &night_regions {
            renderer.set_region_hidden(*r, true);
        }
        let lamps: Vec<PointLight> = night
            .lamps
            .iter()
            .map(|l| PointLight {
                pos: l.light,
                radius: l.radius,
                color: Vec3::from(l.color),
                strength: 1.0,
                tinted: l.kind == "indoor_colored",
            })
            .collect();
        let mut fireflies = Vec::new();
        let mut frng = zoo_core::rng::Pcg32::new(0xF1F1);
        for (r, _) in &night.fireflies {
            for _ in 0..FIREFLIES_PER_AREA {
                let u = frng.next_u32() as f32 / 4_294_967_296.0;
                let v = frng.next_u32() as f32 / 4_294_967_296.0;
                let h = frng.next_u32() as f32 / 4_294_967_296.0;
                let p = Vec2::new(r.x as f32 + u * r.w as f32, r.z as f32 + v * r.d as f32);
                fireflies.push((zoo_core::coords::level_to_world_at(p, 0.5 + h), h * 10.0));
            }
        }

        if !player_skinned {
            *placeholders.entry(PLAYER_MODEL.to_owned()).or_default() += 1;
        }
        for a in animals.iter().filter(|a| !a.skinned) {
            *placeholders.entry(a.id.to_owned()).or_default() += 1;
        }
        let models: Vec<String> = placeholders
            .iter()
            .filter(|(k, _)| !k.starts_with("element:"))
            .map(|(k, n)| format!("{k} ×{n}"))
            .collect();
        let elements = placeholders
            .keys()
            .filter(|k| k.starts_with("element:"))
            .count();
        warn(&format!(
            "placeholders (PROD-POC): {elements} level elements as boxes; missing models: {}",
            if models.is_empty() {
                "none".to_owned()
            } else {
                models.join(", ")
            }
        ));

        // Fluent files: i18n/<lang>/*.ftl.
        let mut per_lang: BTreeMap<&str, String> = BTreeMap::new();
        for (path, bytes) in &files {
            let mut parts = path.split('/');
            if let (Some("i18n"), Some(lang), Some(file)) =
                (parts.next(), parts.next(), parts.next())
            {
                if file.ends_with(".ftl") {
                    let text = per_lang.entry(lang).or_default();
                    text.push_str(&String::from_utf8_lossy(bytes));
                    text.push('\n');
                }
            }
        }
        let sources: Vec<(Language, &str)> = per_lang
            .iter()
            .filter_map(|(l, t)| Language::from_id(l).map(|l| (l, t.as_str())))
            .collect();
        let content = if sources.is_empty() {
            None
        } else {
            match Content::from_sources(&sources) {
                Ok(c) => Some(c),
                Err(e) => {
                    warn(&format!("i18n: {e}"));
                    None
                }
            }
        };

        // Level 1 starts at maximum zoom-out (LAYOUT-L1-011), camera south looking north.
        let params = CameraParams::default();
        let mut camera = FollowCamera::new(params, params.max_distance_m);
        let start = level_to_world(game.player.pos);
        camera.snap(start);
        let player_yaw = facing_to_yaw(game.player.facing);

        let mut app = App {
            game,
            renderer,
            camera,
            keys: Keys::default(),
            stick: Vec2::ZERO,
            drag_acc: 0.0,
            content,
            placeholders,
            player_skinned,
            player_yaw,
            idle_time: 0.0,
            walk_time: 0.0,
            walk_blend: 0.0,
            dynamic: [Instance::model(Vec3::ZERO, 0.0, false)],
            marker: [Instance::model(Vec3::ZERO, 0.0, false)],
            dyn_boxes: Vec::with_capacity(16),
            carry_box: Vec::with_capacity(1 + zoo_core::carrying::MAX_LYING),
            has_carry_model,
            animals,
            anims,
            draws: Vec::with_capacity(8),
            baby_models,
            autopilot: None,
            hints: Default::default(),
            outbox: Vec::new(),
            time: 0.0,
            decals,
            text_textures,
            text_dirty: true,
            barrier_regions,
            roof_regions,
            bowl_glass: [Instance::model(Vec3::ZERO, 0.0, false)],
            bowl_water: [Instance::model(Vec3::ZERO, 0.0, false)],
            inside: None,
            ambient,
            ambient_poses: Vec::with_capacity(16),
            ripples: Vec::with_capacity(16),
            crowd: Vec::with_capacity(16),
            butterfly_poses: Vec::with_capacity(8),
            butterflies: Vec::with_capacity(8),
            paused: false,
            look_at: None,
            ambient_on: true,
            night_regions,
            water_wheels: scene.water_wheels.clone(),
            lamps,
            lamp_order: Vec::with_capacity(128),
            quality: QualityGovernor::default(),
            frame_lights: Vec::with_capacity(nightfx::MAX_POINT_LIGHTS),
            frame_pools: Vec::with_capacity(nightfx::MAX_LIGHT_POOLS),
            glows: Vec::with_capacity(64),
            fireflies,
            openings,
            building_models,
            plants,
            hand_lantern: [Instance::model(Vec3::ZERO, 0.0, false)],
        };
        app.reset_views();
        Ok(app)
    }

    /// Starts a new game with a seed (the host passes a random one, or `?seed=` for tests)
    /// and the hiding places of the previous game to avoid (`{"zebra":"loc_river",…}` or
    /// empty; Q-082). Keeps the settings. Returns false if the game cannot start.
    pub fn new_game(&mut self, seed: f64, avoid_json: &str) -> bool {
        let seed = if seed.is_finite() && seed >= 0.0 {
            seed as u64
        } else {
            1
        };
        let avoid: BTreeMap<String, String> = serde_json::from_str(avoid_json).unwrap_or_default();
        let Ok(mut game) = Game::new_avoiding(self.game.level.data.clone(), seed, &avoid) else {
            return false;
        };
        game.settings = self.game.settings;
        self.game = game;
        self.camera.snap(player_feet(&self.game));
        self.player_yaw = facing_to_yaw(self.game.player.facing);
        self.autopilot = None;
        self.hints = Default::default();
        self.outbox.clear();
        self.reset_views();
        true
    }

    /// Seed of the running playthrough.
    pub fn seed(&self) -> f64 {
        self.game.to_save().seed as f64
    }

    /// Chosen hiding place per animal as JSON (`{"zebra":"loc_river",…}`); the host keeps it
    /// so the next new game avoids these places (Q-082).
    pub fn picks_json(&self) -> String {
        let m: BTreeMap<&str, &str> = self
            .game
            .animals
            .iter()
            .map(|a| (a.id(), a.hiding_place.as_str()))
            .collect();
        serde_json::to_string(&m).unwrap_or_default()
    }

    /// Quality tier (PERF-BUDGETS rule 5, PERF-R-005): `auto` (default; steps down by
    /// itself on slow frames, never back up) or a fixed tier `high` | `low1` (pixel ratio
    /// 1.5) | `low` (pixel ratio 1.5, lantern + 4 lamps, no clouds) — the debug override
    /// `?quality=…` of the host. Returns `false` for an unknown id.
    pub fn set_quality(&mut self, id: &str) -> bool {
        let mode = match id {
            "auto" => QualityMode::Auto,
            _ => match QualityTier::from_id(id) {
                Some(t) => QualityMode::Fixed(t),
                None => return false,
            },
        };
        self.quality.set_mode(mode);
        self.apply_quality();
        true
    }

    /// Current quality tier id (`high` | `low1` | `low`).
    pub fn quality(&self) -> String {
        self.quality.tier().id().to_owned()
    }

    /// `auto` or `fixed`.
    pub fn quality_mode(&self) -> String {
        match self.quality.mode() {
            QualityMode::Auto => "auto",
            QualityMode::Fixed(_) => "fixed",
        }
        .to_owned()
    }

    /// Drawing-buffer pixels per CSS pixel (capped at 2, or 1.5 in the low tier).
    pub fn pixel_ratio(&self) -> f32 {
        self.renderer.pixel_ratio()
    }

    fn apply_quality(&mut self) {
        let tier = self.quality.tier();
        self.renderer.set_max_pixel_ratio(tier.max_pixel_ratio());
        self.renderer.clouds = tier.clouds();
    }

    /// Canvas CSS size and device pixel ratio (render resolution is capped at 2×).
    pub fn resize(&mut self, css_width: f64, css_height: f64, device_pixel_ratio: f64) {
        self.renderer
            .resize(css_width, css_height, device_pixel_ratio);
    }

    /// Keyboard input (`KeyboardEvent.code`). Returns true if the key is used by the game.
    /// Interact keys (`E`, Space, Enter) are handled by the host, which calls [`App::interact`].
    pub fn key(&mut self, code: &str, down: bool) -> bool {
        match code {
            "KeyW" | "ArrowUp" => self.keys.up = down,
            "KeyS" | "ArrowDown" => self.keys.down = down,
            "KeyA" => self.keys.left = down,
            "KeyD" => self.keys.right = down,
            "ArrowLeft" => self.keys.arrow_left = down,
            "ArrowRight" => self.keys.arrow_right = down,
            // GAME-CAMERA-VIEWS 2/3 (user decision 2026-09-27): hold F to look around, V
            // toggles first person
            "KeyF" => {
                self.look_hold(down);
            }
            "KeyV" if down => {
                self.toggle_first_person();
            }
            "KeyV" => {}
            // GAME-FEED §8: G puts the item in the hands down (nothing happens with empty hands)
            "KeyG" if down => {
                self.put_down();
            }
            "KeyG" => {}
            // GAME-HINT rule 1: H = the 🧭 hint button
            "KeyH" if down => {
                self.hint_press();
            }
            "KeyH" => {}
            // FIX-024: E interacts, so rotation is Q (left) / R (right).
            "KeyQ" if down => self.camera.rotate_steps(-1),
            "KeyR" if down => self.camera.rotate_steps(1),
            "Equal" | "NumpadAdd" if down => self.camera.zoom_by(0.85),
            "Minus" | "NumpadSubtract" if down => self.camera.zoom_by(1.0 / 0.85),
            "KeyQ" | "KeyR" | "Equal" | "NumpadAdd" | "Minus" | "NumpadSubtract" => {}
            _ => return false,
        }
        true
    }

    /// Virtual joystick deflection in screen space: `x` right, `y` up, length ≤ 1.
    pub fn set_stick(&mut self, x: f32, y: f32) {
        self.stick = Vec2::new(x, y).clamp_length_max(1.0);
    }

    /// Horizontal mouse drag in CSS pixels; every `DRAG_STEP_PX` rotates the camera one 45°
    /// step (eased, GAME-PLAYER §2).
    pub fn drag(&mut self, dx: f32) {
        if self.camera.mode().is_close() {
            return; // close views turn smoothly with `look_drag` (GAME-CAMERA-VIEWS 2/3)
        }
        self.drag_acc += dx;
        while self.drag_acc >= DRAG_STEP_PX {
            self.camera.rotate_steps(1);
            self.drag_acc -= DRAG_STEP_PX;
        }
        while self.drag_acc <= -DRAG_STEP_PX {
            self.camera.rotate_steps(-1);
            self.drag_acc += DRAG_STEP_PX;
        }
    }

    pub fn drag_end(&mut self) {
        self.drag_acc = 0.0;
    }

    /// Rotates the camera by whole 45° steps (touch swipe, GAME-PLAYER §3).
    pub fn rotate(&mut self, steps: i32) {
        if !self.camera.mode().is_close() {
            self.camera.rotate_steps(steps);
        }
    }

    // ------------------------------------------------------------------ camera views

    /// Look-around held (`true`) or released (GAME-CAMERA-VIEWS 2): eye button, `F`, right
    /// mouse button. Only from the zoo view; returns whether look-around is now active.
    pub fn look_hold(&mut self, on: bool) -> bool {
        let facing = views::level_to_yaw(self.game.player.facing);
        match (on, self.camera.mode()) {
            (true, ViewMode::Zoo) => {
                self.camera.set_view(ViewMode::LookAround, facing);
            }
            (false, ViewMode::LookAround) => {
                self.camera.set_view(ViewMode::Zoo, facing);
            }
            _ => {}
        }
        self.camera.mode() == ViewMode::LookAround
    }

    /// Toggles first person (GAME-CAMERA-VIEWS 3; `V`, 👓 button). Returns the new view id.
    pub fn toggle_first_person(&mut self) -> String {
        let next = if self.camera.mode() == ViewMode::FirstPerson {
            ViewMode::Zoo
        } else {
            ViewMode::FirstPerson
        };
        self.set_view(next);
        self.view_mode()
    }

    /// Sets the view by id (`zoo`, `first_person`; restored from the settings at start, no
    /// glide). Returns `false` for an unknown id.
    pub fn set_view_mode(&mut self, id: &str) -> bool {
        let Some(mode) = ViewMode::from_id(id) else {
            return false;
        };
        self.set_view(mode.saved());
        self.camera.snap(player_feet(&self.game));
        true
    }

    /// The requested view id (`zoo`, `look_around`, `first_person`).
    pub fn view_mode(&self) -> String {
        self.camera.mode().id().to_owned()
    }

    /// The view to store in the settings (look-around is never stored, behaviour 9).
    pub fn saved_view_mode(&self) -> String {
        self.camera.mode().saved().id().to_owned()
    }

    /// Continuous turn of a close view by a drag in CSS px (mouse, right thumb, eye button;
    /// `dx` right = turn right, `dy` up (negative) = look up). Ignored in the zoo view.
    pub fn look_drag(&mut self, dx: f32, dy: f32) {
        let k = views::TURN_DEG_PER_PX.to_radians();
        self.camera.turn(-dx * k, -dy * k);
    }

    /// Zoom: factor < 1 moves the camera closer (wheel, pinch), clamped to 10–20 m.
    pub fn zoom(&mut self, factor: f32) {
        self.camera.zoom_by(factor);
    }

    // ------------------------------------------------------------------ settings

    /// Sets the language (`de`, `en`); returns false for unsupported ids.
    pub fn set_language(&mut self, id: &str) -> bool {
        match Language::from_id(id) {
            Some(l) => {
                if self.game.settings.language != l {
                    self.text_dirty = true;
                }
                self.game.settings.language = l;
                true
            }
            None => false,
        }
    }

    pub fn language(&self) -> String {
        self.game.settings.language.id().to_owned()
    }

    /// Sets the reading level (`kiga` … `klasse3`); returns false for unknown ids.
    pub fn set_reading_level(&mut self, id: &str) -> bool {
        match ReadingLevel::from_id(id) {
            Some(l) => {
                self.game.settings.reading_level = l;
                true
            }
            None => false,
        }
    }

    pub fn reading_level(&self) -> String {
        self.game.settings.reading_level.id().to_owned()
    }

    /// Default language for a browser language tag (CONT-L10N §5).
    pub fn default_language(tag: &str) -> String {
        zoo_core::content::default_language(tag).id().to_owned()
    }

    /// A Fluent message in the current language (the key itself if missing).
    pub fn t(&self, key: &str) -> String {
        self.text_now(key)
    }

    // ------------------------------------------------------------------ save (GAME-SAVE)

    /// The full game state as JSON (versioned), incl. camera and animal facing. The host
    /// stores it (`localStorage`) on `visibilitychange` / `pagehide` and when
    /// [`App::take_save`] returns one.
    pub fn save(&mut self) -> String {
        let mut s = self.game.to_save();
        s.camera = Some(zoo_core::save::CameraSave {
            yaw_steps: self.camera.yaw_steps(),
            distance_m: self.camera.target_distance(),
        });
        for (a, v) in s.animals.iter_mut().zip(&self.animals) {
            a.yaw = Some(v.yaw);
        }
        self.game.mark_saved();
        s.to_json()
    }

    /// A save when one is due (progress event, or 5 s of walking; GAME-SAVE §3), else empty.
    pub fn take_save(&mut self) -> String {
        if self.game.save_due() {
            self.save()
        } else {
            String::new()
        }
    }

    /// Restores a save before the first frame (GAME-SAVE §4). Returns false (and keeps the
    /// new game) for an unknown version, another level or broken data (§5) — silently.
    pub fn restore(&mut self, json: &str) -> bool {
        let Ok(state) = zoo_core::save::SaveState::from_json(json) else {
            return false;
        };
        let Ok(mut game) = Game::from_save(self.game.level.data.clone(), &state) else {
            return false;
        };
        game.settings = self.game.settings;
        self.game = game;
        if let Some(c) = state.camera {
            self.camera.set_state(c.yaw_steps, c.distance_m);
        }
        self.camera.snap(player_feet(&self.game));
        self.player_yaw = facing_to_yaw(self.game.player.facing);
        self.reset_views();
        for (i, v) in self.animals.iter_mut().enumerate() {
            let Some(a) = self.game.animals.get(i) else {
                continue;
            };
            if let Some(yaw) = state
                .animals
                .iter()
                .find(|s| s.id == v.id && s.member == a.member)
                .and_then(|s| s.yaw)
                .filter(|y| y.is_finite())
            {
                v.yaw = yaw;
            } else if a.state == AnimalState::Following {
                v.yaw = facing_to_yaw(self.game.player.pos - a.pos);
            }
        }
        self.autopilot = None;
        self.hints = Default::default();
        self.outbox.clear();
        true
    }

    // ------------------------------------------------------------------ text textures

    /// Whether the host must (re-)render the text textures (start, language change).
    pub fn text_textures_dirty(&self) -> bool {
        self.text_dirty
    }

    /// Text textures to render as a JSON array `[{"id", "key", "text", "width", "height"}]`
    /// (texts in the current language). Clears the dirty flag. The host draws each text into
    /// a `width` × `height` RGBA image and hands it back with [`App::set_text_texture`].
    pub fn text_textures(&mut self) -> String {
        self.text_dirty = false;
        let items: Vec<String> = self
            .text_textures
            .iter()
            .map(|(id, key, w, h)| {
                format!(
                    "{{\"id\":{},\"key\":{},\"text\":{},\"width\":{w},\"height\":{h}}}",
                    js(id),
                    js(key),
                    js(&self.text_now(key))
                )
            })
            .collect();
        format!("[{}]", items.join(","))
    }

    /// Uploads a rendered text texture (RGBA8, top row first). Returns false for unknown ids
    /// or a wrong size.
    pub fn set_text_texture(&mut self, id: &str, width: u32, height: u32, rgba: &[u8]) -> bool {
        let known = self
            .text_textures
            .iter()
            .any(|(t, _, w, h)| t == id && *w == width && *h == height);
        known
            && self
                .renderer
                .set_decal_texture_rgba(id, width, height, rgba)
                .is_ok()
    }

    // ------------------------------------------------------------------ interaction

    /// Kind of the available interactable (`info_board`, `food_box`, `animal`, `gate`) or
    /// empty (GAME-PLAYER §5). Drives the interact button / key hint.
    pub fn target_kind(&self) -> String {
        self.game
            .available_target()
            .map(|t| t.kind().to_owned())
            .unwrap_or_default()
    }

    /// Debug (e2e, LAYOUT-041 / Q-181): every food box as JSON `[{food, x, z, fx, fz}]` —
    /// box centre and label facing (level).
    pub fn food_boxes_json(&self) -> String {
        let rows: Vec<String> = self
            .game
            .food_boxes
            .iter()
            .map(|(f, p, d)| {
                format!(
                    "{{\"food\":\"{}\",\"x\":{:.2},\"z\":{:.2},\"fx\":{:.2},\"fz\":{:.2}}}",
                    f.id(),
                    p.x,
                    p.y,
                    d.x,
                    d.y
                )
            })
            .collect();
        format!("[{}]", rows.join(","))
    }

    /// Stable key of the available interactable (e.g. `food_box:grass`) or empty.
    pub fn target_key(&self) -> String {
        self.game
            .available_target()
            .map(|t| target_key(&t))
            .unwrap_or_default()
    }

    /// Interacts with the available target. Returns a JSON object for the host (text panel
    /// content) or an empty string when nothing is available.
    pub fn interact(&mut self) -> String {
        let Some(result) = self.game.interact() else {
            return String::new();
        };
        let json = self.interaction_json(&result);
        self.handle_events();
        json
    }

    /// Closes the reading panel by hand (✖ / Esc / after taking food): it stays closed until
    /// the player leaves and re-enters the range (GAME-PLAYER §4, PLAY-026).
    pub fn close_panel(&mut self) {
        self.game.close_panel();
    }

    /// Key of the target whose reading panel is open (e.g. `food_box:grass`) or empty.
    pub fn panel_key(&self) -> String {
        self.game
            .panel
            .open
            .as_ref()
            .map(target_key)
            .unwrap_or_default()
    }

    /// Content of the open reading panel as JSON (same form as [`App::interact`]) in the
    /// current language and reading level, or empty (host re-renders after settings changes).
    pub fn panel_json(&self) -> String {
        self.game
            .panel
            .open
            .clone()
            .and_then(|t| self.panel_interaction(&t))
            .map(|i| self.interaction_json(&i))
            .unwrap_or_default()
    }

    /// Takes the food of a box (GAME-FEED §6). Returns true when the player now carries it.
    pub fn take_food(&mut self, food_id: &str) -> bool {
        let Some(food) = Food::from_id(food_id) else {
            return false;
        };
        let ok = self.game.take_food(food).is_ok();
        self.handle_events();
        ok
    }

    /// The treat basket (GAME-GARDEN §4) as JSON `{"carrot": n, "potato": n, "capacity": 6,
    /// "offered": "carrot"|""}` for the HUD (icons + numbers).
    pub fn basket_json(&self) -> String {
        let b = self.game.garden.basket;
        format!(
            "{{\"carrot\":{},\"potato\":{},\"capacity\":{},\"offered\":{}}}",
            b.carrots,
            b.potatoes,
            zoo_core::garden::BASKET_CAPACITY,
            js(self.game.offered_treat().map_or("", |t| t.id()))
        )
    }

    /// Chooses the treat offered at a fence (`carrot`, `potato`); false if unknown.
    pub fn select_treat(&mut self, id: &str) -> bool {
        match zoo_core::garden::Treat::from_id(id) {
            Some(t) => {
                self.game.treat_choice = Some(t);
                true
            }
            None => false,
        }
    }

    /// Debug/e2e: the gates and doors as JSON `[{"model", "x", "z", "open"}]` (open amount
    /// 0…1 as drawn, LAYOUT-031).
    pub fn openings_json(&self) -> String {
        let scene_models: Vec<String> = self
            .game
            .level
            .openings()
            .iter()
            .zip(&self.openings)
            .map(|(o, v)| {
                format!(
                    "{{\"kind\":{},\"x\":{:.2},\"z\":{:.2},\"open\":{:.3},\"drawn\":{}}}",
                    js(match o.kind {
                        OpeningKind::BuildingDoor { .. } => "door",
                        OpeningKind::EnclosureGate { .. } => "gate",
                        OpeningKind::GlassDoor { .. } => "glass_door",
                        OpeningKind::GardenGate { .. } => "garden_gate",
                        OpeningKind::MoonDoor { .. } => "moon_door",
                        OpeningKind::LevelGate { .. } => "level_gate",
                    }),
                    o.center.x,
                    o.center.y,
                    v.open,
                    v.handle.is_some()
                )
            })
            .collect();
        format!("[{}]", scene_models.join(","))
    }

    /// Debug/e2e: a garden plant's growth stage (`empty` … `ripe`) or empty if unknown.
    pub fn plant_stage(&self, spot: &str) -> String {
        self.game
            .garden
            .plant(spot)
            .map(|p| p.stage().id().to_owned())
            .unwrap_or_default()
    }

    /// Whether something droppable is in the hands: the put-down button is shown (GAME-FEED
    /// §8, FEED-016).
    pub fn can_put_down(&self) -> bool {
        self.game.can_put_down()
    }

    /// Puts the item in the hands down (put-down button / `G`, GAME-FEED §8–10). False when
    /// nothing was put down (empty hands, or no free spot: the host shakes the button).
    pub fn put_down(&mut self) -> bool {
        if !self.game.can_put_down() {
            return false;
        }
        let ok = match self.game.put_down() {
            Ok(_) => true,
            Err(_) => {
                self.outbox
                    .push("{\"type\":\"put_down_refused\"}".to_owned());
                false
            }
        };
        self.handle_events();
        ok
    }

    /// Debug/e2e: the foods lying on the ground as JSON `[{"uid", "food", "x", "z", "y"}]`
    /// (oldest first) and whether the bowl lies somewhere (GAME-FEED §10).
    pub fn lying_json(&self) -> String {
        let foods: Vec<String> = self
            .game
            .lying
            .foods
            .iter()
            .map(|f| {
                format!(
                    "{{\"uid\":{},\"food\":{},\"x\":{:.3},\"z\":{:.3},\"y\":{:.3}}}",
                    f.uid,
                    js(f.food.id()),
                    f.pos.x,
                    f.pos.y,
                    f.y
                )
            })
            .collect();
        format!(
            "{{\"foods\":[{}],\"bowl\":{}}}",
            foods.join(","),
            self.game.bowl_lying()
        )
    }

    /// Lying foods near the player (≤ 5 m) with their screen position (CSS px) for the
    /// readable icon above them (GAME-FEED §10): `[{"food", "x", "y"}]`.
    pub fn lying_icons_json(&self) -> String {
        let p = self.game.player.pos;
        let items: Vec<String> = self
            .game
            .lying
            .foods
            .iter()
            .filter(|f| f.pos.distance(p) <= LYING_ICON_M)
            .filter_map(|f| {
                let s = self.screen_point(f.pos.x, f.pos.y, f.y + 0.75);
                (s.len() == 2).then(|| {
                    format!(
                        "{{\"food\":{},\"x\":{:.1},\"y\":{:.1}}}",
                        js(f.food.id()),
                        s[0],
                        s[1]
                    )
                })
            })
            .collect();
        format!("[{}]", items.join(","))
    }

    /// Debug/e2e: the bamboo cut spots as JSON `[{"id", "x", "z", "stage", "regrow_s"}]`
    /// (GAME-FEED §14–15).
    pub fn cut_spots_json(&self) -> String {
        let spots: Vec<String> = self
            .game
            .level
            .data
            .cut_spots
            .iter()
            .enumerate()
            .map(|(i, c)| {
                format!(
                    "{{\"id\":{},\"x\":{:.2},\"z\":{:.2},\"sx\":{:.2},\"sz\":{:.2},\"stage\":{},\"regrow_s\":{:.2}}}",
                    js(&c.id),
                    c.pos[0],
                    c.pos[1],
                    c.stand[0],
                    c.stand[1],
                    js(self.game.bamboo.stage(i).map_or("", |s| s.id())),
                    self.game.bamboo.regrow_s.get(i).copied().unwrap_or(0.0)
                )
            })
            .collect();
        format!("[{}]", spots.join(","))
    }

    /// Debug/e2e: the water wheels as JSON `[{"x", "z", "axle_y", "radius", "angle_deg"}]`
    /// with the angle drawn now (the renderer turns the `wheel` part with the same clock,
    /// LAYOUT-L3-017/018).
    pub fn water_wheels_json(&self) -> String {
        let items: Vec<String> = self
            .water_wheels
            .iter()
            .map(|w| {
                format!(
                    "{{\"x\":{:.2},\"z\":{:.2},\"axle_y\":{:.2},\"radius\":{:.2},\"angle_deg\":{:.3}}}",
                    w.center.x,
                    w.center.y,
                    w.axle_y,
                    w.radius,
                    w.angle_at(self.time).to_degrees()
                )
            })
            .collect();
        format!("[{}]", items.join(","))
    }

    /// Id of the carried food or empty.
    pub fn carry_food(&self) -> String {
        self.game
            .carry
            .food()
            .map(|f| f.id().to_owned())
            .unwrap_or_default()
    }

    /// Word of the carried food in the current language, or empty.
    pub fn carry_text(&self) -> String {
        self.game
            .carry
            .food()
            .map(|f| self.text_now(&f.label_key()))
            .unwrap_or_default()
    }

    /// Feedback events since the last call as a JSON array: `{"type": "say" | "mission_complete"
    /// | "food_taken", "animal", "text"}`.
    pub fn poll_events(&mut self) -> String {
        let out = format!("[{}]", self.outbox.join(","));
        self.outbox.clear();
        out
    }

    // ------------------------------------------------------------------ frame

    /// Advances the game by `dt` seconds and renders one frame. `dt` is the real interval
    /// since the last frame (it also drives the automatic quality tier, PERF-R-005).
    pub fn frame(&mut self, dt: f32) {
        if self.quality.sample(dt) {
            self.apply_quality();
        }
        let dt = if self.paused { 0.0 } else { dt.clamp(0.0, 0.1) };
        self.time += dt as f64;
        self.simulate(dt);

        let player = player_feet(&self.game);
        self.camera.update(dt, player);
        if let Some(p) = self.look_at {
            self.camera.snap(level_to_world(p));
        }

        // Character presentation: turn smoothly, blend idle↔walk by speed.
        let target = facing_to_yaw(self.game.player.facing);
        self.player_yaw += angle_diff(target, self.player_yaw) * (1.0 - (-14.0 * dt).exp());
        if self.camera.mode() == ViewMode::FirstPerson {
            self.player_yaw = target; // carried items stay in front of the eye
        }
        let speed = self.game.player.last_speed;
        let walk_speed = self.game.move_params.walk_speed;
        let want = (speed / (0.5 * walk_speed)).clamp(0.0, 1.0);
        self.walk_blend += (want - self.walk_blend) * (1.0 - (-10.0 * dt).exp());
        self.idle_time += dt;
        // walk clip playback = speed ÷ 1.4, clamped (GAME-PLAYER §6, ART-RIG §4.7)
        self.walk_time += dt * walk_clip_rate(speed);
        let yaw = self.player_yaw;

        // Carried food box in front of the chest (socket_carry approximation).
        let fwd = Quat::from_rotation_y(yaw) * Vec3::Z;
        if self.has_carry_model {
            // with the bowl in both hands the food is in the pocket (Q-084): not drawn
            let bowl_carried = self.game.bowl.as_ref().is_some_and(|b| b.carried);
            self.carry_box.clear();
            if self.game.carry.food().is_some() && !bowl_carried {
                let p = player + fwd * 0.32 + Vec3::Y * 0.45;
                self.carry_box.push(Instance {
                    pos_yaw: [p.x, p.y, p.z, yaw],
                    scale_fade: [0.55, 0.55, 0.55, 0.0],
                    color: [1.0, 1.0, 1.0, 0.0],
                    node: [0.0; 4],
                });
            }
            // lying foods: the same small closed box standing on the surface (GAME-FEED §10;
            // per-food models are missing art)
            for f in &self.game.lying.foods {
                let p = level_to_world(f.pos) + Vec3::Y * f.y;
                let yaw = (f.uid as f32 * 2.399).rem_euclid(std::f32::consts::TAU);
                self.carry_box.push(Instance {
                    pos_yaw: [p.x, p.y, p.z, yaw],
                    scale_fade: [0.55, 0.55, 0.55, 0.0],
                    color: [1.0, 1.0, 1.0, 0.0],
                    node: [0.0; 4],
                });
            }
            self.renderer
                .set_dynamic_instances(CARRY_BOX, &self.carry_box);
        }

        // Barriers that opened disappear; the roof of the building the player is in is hidden
        // (GAME-PLAYER §2, PLAY-028).
        for (id, region) in &self.barrier_regions {
            let open = self.game.level.is_barrier_open(id);
            if open != self.renderer.region_hidden(*region) {
                self.renderer.set_region_hidden(*region, open);
            }
        }
        self.inside = self.building_inside();
        let mode = self.camera.mode();
        for (id, region) in &self.roof_regions {
            // first person keeps the roof and its ceiling (CAMV-022)
            let hide = views::roof_hidden(self.inside.as_deref() == Some(id.as_str()), mode);
            if hide != self.renderer.region_hidden(*region) {
                self.renderer.set_region_hidden(*region, hide);
            }
        }
        for (id, h, base) in &self.building_models {
            let hide = views::roof_hidden(self.inside.as_deref() == Some(id.as_str()), mode);
            let mut i = *base;
            i.node[1] = if hide {
                (HIDE_ROOF | HIDE_WALLS_UPPER) as f32
            } else {
                0.0
            };
            self.renderer.set_instance(*h, i);
        }
        self.update_openings(dt);
        self.update_plants();

        // The fish bowl: glass + water (+ the fish inside, drawn with the animals).
        let bowl_base = self.bowl_base(fwd, player);
        let (mut n_glass, mut n_water) = (0, 0);
        if let (Some(base), Some(b)) = (bowl_base, &self.game.bowl) {
            let r = BOWL_RADIUS_M * 2.0;
            self.bowl_glass[0] = Instance {
                pos_yaw: [base.x, base.y, base.z, 0.0],
                scale_fade: [r, BOWL_HEIGHT_M, r, 0.0],
                color: GLASS,
                node: [0.0; 4],
            };
            n_glass = 1;
            if b.water {
                let w = r - 0.04;
                self.bowl_water[0] = Instance {
                    pos_yaw: [base.x, base.y + 0.02, base.z, 0.0],
                    scale_fade: [w, BOWL_HEIGHT_M * 0.72, w, 0.0],
                    color: BOWL_WATER_COLOR,
                    node: [0.0; 4],
                };
                n_water = 1;
            }
        }
        self.renderer
            .set_dynamic_instances(BOWL, &self.bowl_glass[..n_glass]);
        self.renderer
            .set_dynamic_instances(BOWL_WATER, &self.bowl_water[..n_water]);

        // Animals: skinned models, else placeholder boxes in the dynamic box batch.
        self.dyn_boxes.clear();
        bamboo_stalks(&mut self.dyn_boxes, &self.game);
        self.draws.clear();
        self.glows.clear();
        for (i, a) in self.animals.iter().enumerate() {
            if !a.visible {
                continue; // its level is still locked
            }
            // eyeshine inside the lantern light (NIGHT-006)
            let shine = self
                .game
                .animals
                .get(i)
                .is_some_and(|ga| self.game.eyes_shine(ga));
            let pos = a
                .anchor
                .unwrap_or_else(|| level_to_world(a.pos) + Vec3::Y * (a.ground - a.sink + a.lift));
            let pos = match a.leap {
                Some((from, t, _)) => {
                    let k = (t / LEAP_S).clamp(0.0, 1.0);
                    from.lerp(pos, k) + Vec3::Y * (4.0 * k * (1.0 - k)) * 0.9
                }
                None => pos,
            };
            let (action, t) = a.action.map_or((None, 0.0), |(n, t, _)| (Some(n), t));
            if a.skinned {
                // the baby (GAME-FAMILY §5, flag `game.babies`) stays at the female's side and
                // copies her clips, a step behind (own model, own size)
                let baby = self
                    .game
                    .animals
                    .get(i)
                    .filter(|ga| ga.member == 1)
                    .and_then(|_| {
                        let m = zoo_core::animals::baby_model(a.id)?;
                        (self.game.babies.iter().any(|b| b == a.id)
                            && self.baby_models.contains(&m))
                        .then_some(m)
                    });
                if let Some(m) = baby {
                    self.draws.push((
                        m,
                        CharacterDraw {
                            pos: pos + Vec3::new(0.75, 0.0, 0.45),
                            yaw: a.yaw,
                            idle_time: a.idle_time + 0.7,
                            walk_time: a.walk_time + 0.3,
                            walk_blend: a.walk_blend,
                            idle_clip: a.rest,
                            walk_clip: a.locomotion,
                            action: None,
                            action_blend: 0.0,
                            under_water: a.under_water,
                            tilt: Quat::IDENTITY,
                            eye_glow: false,
                        },
                    ));
                }
                self.draws.push((
                    a.model,
                    CharacterDraw {
                        pos,
                        yaw: a.yaw,
                        idle_time: a.idle_time,
                        walk_time: a.walk_time,
                        walk_blend: a.walk_blend,
                        idle_clip: a.rest,
                        walk_clip: a.locomotion,
                        action: action.map(|n| (n, t)),
                        action_blend: if action.is_some() { 1.0 } else { 0.0 },
                        under_water: a.under_water,
                        tilt: Quat::IDENTITY,
                        // eye_glow inside the lantern light, never while asleep (Q-146)
                        eye_glow: zoo_core::night::eye_glow(shine, a.rest, action),
                    },
                ));
            } else if is_night_placeholder(a.id) {
                night_animal_placeholder(
                    &mut self.dyn_boxes,
                    &mut self.glows,
                    pos,
                    a,
                    action,
                    t,
                    shine,
                );
            } else {
                animal_placeholder(&mut self.dyn_boxes, pos, a, action, t);
            }
        }
        self.renderer
            .set_dynamic_instances(DYN_BOX, &self.dyn_boxes);

        let hide_player = self.camera.hides_player(); // first person (GAME-CAMERA-VIEWS 3)
        if hide_player {
            self.renderer.set_dynamic_instances(CAPSULE, &[]);
            self.renderer.set_dynamic_instances(MARKER, &[]);
        } else if self.player_skinned {
            self.draws.push((
                PLAYER_MODEL,
                CharacterDraw::locomotion(
                    player,
                    yaw,
                    self.idle_time,
                    self.walk_time,
                    self.walk_blend,
                ),
            ));
        } else {
            // Placeholder capsule (1.2 m) with a small marker showing the facing.
            let bob = (self.walk_time * 9.0).sin().abs() * 0.05 * self.walk_blend;
            let p = player + Vec3::Y * bob;
            self.dynamic[0] = Instance::flat(p, yaw, Vec3::ONE, [0.35, 0.62, 0.92], false);
            self.marker[0] = Instance::flat(
                p + fwd * 0.28 + Vec3::Y * 0.8,
                yaw,
                Vec3::new(0.18, 0.14, 0.12),
                [0.96, 0.78, 0.60],
                false,
            );
            self.renderer.set_dynamic_instances(CAPSULE, &self.dynamic);
            self.renderer.set_dynamic_instances(MARKER, &self.marker);
        }
        // Ambient animals (GAME-AMBIENT): instanced skinned crowds + their water ripples.
        self.ambient.poses(&mut self.ambient_poses);
        if !self.ambient_on {
            self.ambient_poses.clear();
        }
        self.crowd.clear();
        for p in &self.ambient_poses {
            if !self.renderer.has_skinned(p.model) {
                continue;
            }
            self.crowd.push((
                p.model,
                CharacterDraw {
                    pos: p.pos,
                    yaw: p.yaw,
                    idle_time: p.idle_time,
                    walk_time: p.walk_time,
                    walk_blend: p.walk_blend,
                    idle_clip: p.idle_clip,
                    walk_clip: p.walk_clip,
                    action: p.action,
                    action_blend: p.action_blend,
                    under_water: false,
                    tilt: p.tilt,
                    eye_glow: false,
                },
            ));
        }
        self.ambient.butterfly_poses(&mut self.butterfly_poses);
        self.butterflies.clear();
        if self.ambient_on {
            for b in &self.butterfly_poses {
                self.butterflies.push(Instance::flat(
                    b.pos,
                    b.yaw,
                    Vec3::new(b.open, 1.0, 1.0),
                    b.color,
                    false,
                ));
            }
        }
        self.renderer
            .set_dynamic_instances(BUTTERFLY, &self.butterflies);
        self.ambient.ripples(&mut self.ripples);
        if !self.ambient_on {
            self.ripples.clear();
        }
        let pl = self.game.player.pos;
        self.ripples.sort_unstable_by(|a, b| {
            a.pos
                .distance_squared(pl)
                .total_cmp(&b.pos.distance_squared(pl))
        });
        self.renderer
            .set_water_ripples(self.ripples.iter().map(|r| {
                (
                    Vec2::new(r.pos.x, -r.pos.y),
                    Vec2::new(r.heading.x, -r.heading.y),
                    r.wake,
                    r.ring,
                )
            }));
        self.night_frame(player);
        self.renderer.set_time(self.time);
        self.renderer
            .render(&self.camera, player, &self.draws, &self.crowd);
    }

    // ------------------------------------------------------------------ debug getters

    /// Player position in level coordinates (x east).
    pub fn player_x(&self) -> f32 {
        self.game.player.pos.x
    }

    /// Player position in level coordinates (z north).
    pub fn player_z(&self) -> f32 {
        self.game.player.pos.y
    }

    /// Player facing in level coordinates (x east).
    pub fn player_facing_x(&self) -> f32 {
        self.game.player.facing.x
    }

    /// Player facing in level coordinates (z north).
    pub fn player_facing_z(&self) -> f32 {
        self.game.player.facing.y
    }

    /// Speed moved in the last update (m/s).
    pub fn player_speed(&self) -> f32 {
        self.game.player.last_speed
    }

    /// Current surface speed at full deflection (m/s).
    pub fn surface_speed(&self) -> f32 {
        self.game.player.surface_speed()
    }

    /// Debug/e2e: world position of the camera eye `[x, y, z]` (y up; level z = −world z).
    pub fn camera_eye(&self) -> Vec<f32> {
        self.camera.eye().to_array().to_vec()
    }

    /// Debug/e2e: pitch of the drawn view in degrees (negative = down).
    pub fn camera_pitch_deg(&self) -> f32 {
        self.camera.pose().pitch.to_degrees()
    }

    /// Debug/e2e: yaw of the drawn view in degrees (zoo, close or the glide between).
    pub fn camera_view_yaw_deg(&self) -> f32 {
        self.camera.view_yaw().to_degrees()
    }

    /// Debug/e2e: glide progress 0 (zoo view) … 1 (close view).
    pub fn camera_blend(&self) -> f32 {
        self.camera.blend()
    }

    /// Debug/e2e: fog end distance of this frame (m) and fog amount.
    pub fn camera_fog(&self) -> Vec<f32> {
        let f = self.camera.fog();
        vec![f.start, f.end, f.amount]
    }

    /// Debug/e2e: the player's body is drawn (not in first person).
    pub fn player_drawn(&self) -> bool {
        !self.camera.hides_player()
    }

    pub fn camera_distance(&self) -> f32 {
        self.camera.distance()
    }

    pub fn camera_yaw_deg(&self) -> f32 {
        self.camera.yaw().to_degrees()
    }

    /// Yaw the camera eases towards (a multiple of 45°).
    pub fn camera_target_yaw_deg(&self) -> f32 {
        self.camera.target_yaw().to_degrees()
    }

    pub fn draw_calls(&self) -> u32 {
        self.renderer.stats.draw_calls
    }

    /// Debug: renders one frame and returns the static batches it drew (`model@region`, one
    /// per line) — which meshes cost the draw calls (CAMV-014, Q-104).
    pub fn debug_draw_list(&mut self) -> String {
        self.renderer.debug_draws = Some(Vec::new());
        self.frame(0.0);
        let list = self.renderer.debug_draws.take().unwrap_or_default();
        list.join("\n")
    }

    pub fn instances(&self) -> u32 {
        self.renderer.stats.instances
    }

    pub fn triangles(&self) -> u32 {
        self.renderer.stats.triangles
    }

    /// Number of placeholder boxes (missing models + unmodelled level elements, POC-004).
    pub fn placeholder_count(&self) -> usize {
        self.placeholders.values().sum()
    }

    /// Whether the player is drawn with the skinned `player_girl.glb`.
    pub fn player_is_model(&self) -> bool {
        self.player_skinned
    }

    /// Whether an animal is drawn with its skinned model (else a placeholder).
    pub fn animal_is_model(&self, id: &str) -> bool {
        self.animals.iter().any(|a| a.id == id && a.skinned)
    }

    /// Placeholders as `name ×count` lines.
    pub fn placeholders(&self) -> String {
        self.placeholders
            .iter()
            .map(|(k, n)| format!("{k} ×{n}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Decal ids of the level (e.g. `sign:enc_zebra`, `sign:food_storage`), one per line.
    pub fn decal_ids(&self) -> String {
        self.decals
            .iter()
            .map(|d| d.id.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Whether a decal is drawn (its texture exists), AENV-011/012.
    pub fn decal_drawn(&self, id: &str) -> bool {
        self.decals.iter().any(|d| {
            d.id == id
                && self.renderer.has_decal_texture(&match &d.image {
                    DecalImage::Texture(p) => p.clone(),
                    DecalImage::Text { key, .. } => text_texture_id(key),
                })
        })
    }

    /// Screen rectangle of a decal in CSS px `[min_x, min_y, max_x, max_y]` (y down) with the
    /// current camera, or empty if unknown / behind the camera.
    pub fn decal_screen_rect(&self, id: &str) -> Vec<f32> {
        let Some(d) = self.decals.iter().find(|d| d.id == id) else {
            return Vec::new();
        };
        let (w, h) = self.renderer.size();
        let ratio = self.renderer.pixel_ratio();
        let vp = self.camera.view_proj(self.renderer.aspect());
        let mut r = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
        for c in d.corners() {
            let Some(ndc) = zoo_render::camera::project(vp, c) else {
                return Vec::new();
            };
            let x = (ndc.x * 0.5 + 0.5) * w as f32 / ratio;
            let y = (0.5 - ndc.y * 0.5) * h as f32 / ratio;
            r = [r[0].min(x), r[1].min(y), r[2].max(x), r[3].max(y)];
        }
        r.to_vec()
    }

    /// Screen rectangle of the player (feet to 1.3 m, radius 0.3 m) in CSS px
    /// `[min_x, min_y, max_x, max_y]` with the current camera (PLAY-025: panels never cover it).
    pub fn player_screen_rect(&self) -> Vec<f32> {
        let (w, h) = self.renderer.size();
        let ratio = self.renderer.pixel_ratio();
        let vp = self.camera.view_proj(self.renderer.aspect());
        let p = player_feet(&self.game);
        let mut r = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
        for dy in [0.0, 1.3] {
            for (dx, dz) in [(-0.3, 0.0), (0.3, 0.0), (0.0, -0.3), (0.0, 0.3)] {
                let Some(ndc) = zoo_render::camera::project(vp, p + Vec3::new(dx, dy, dz)) else {
                    return Vec::new();
                };
                let x = (ndc.x * 0.5 + 0.5) * w as f32 / ratio;
                let y = (0.5 - ndc.y * 0.5) * h as f32 / ratio;
                r = [r[0].min(x), r[1].min(y), r[2].max(x), r[3].max(y)];
            }
        }
        r.to_vec()
    }

    /// Decals drawn in the last frame.
    pub fn decals_drawn(&self) -> u32 {
        self.renderer.decals_drawn()
    }

    /// A Fluent message in the given language, if the i18n files were loaded.
    pub fn text(&self, lang: &str, key: &str) -> Option<String> {
        let lang = Language::from_id(lang)?;
        self.content.as_ref()?.text(lang, key)
    }

    /// Animal state: `escaped`, `following`, `in_enclosure` (empty if unknown).
    pub fn animal_state(&self, id: &str) -> String {
        self.game
            .animal(id)
            .map(|a| match a.state {
                AnimalState::Escaped => "escaped",
                AnimalState::Following => "following",
                AnimalState::InEnclosure => "in_enclosure",
                AnimalState::InBowl => "in_bowl",
            })
            .unwrap_or_default()
            .to_owned()
    }

    pub fn animal_x(&self, id: &str) -> f32 {
        self.game.animal(id).map_or(f32::NAN, |a| a.pos.x)
    }

    pub fn animal_z(&self, id: &str) -> f32 {
        self.game.animal(id).map_or(f32::NAN, |a| a.pos.y)
    }

    /// Chosen hiding place of an animal (discovery, GAME-RESCUE §1) or empty.
    pub fn animal_hiding_place(&self, id: &str) -> String {
        self.game
            .animal(id)
            .map(|a| a.hiding_place.clone())
            .unwrap_or_default()
    }

    /// Ids of the animals of the level, one per line.
    pub fn animal_ids(&self) -> String {
        self.game
            .animals
            .iter()
            .map(|a| a.id())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Whether an animal is walking around (wandering) right now.
    pub fn animal_wandering(&self, id: &str) -> bool {
        self.game.animal(id).is_some_and(|a| a.is_wandering())
    }

    /// Resting / locomotion clip the animal plays and how deep it is sunk (m), for tests.
    pub fn animal_clip(&self, id: &str) -> String {
        self.animals
            .iter()
            .find(|v| v.id == id)
            .map(|v| format!("{}|{}|{:.2}", v.rest, v.locomotion, v.sink))
            .unwrap_or_default()
    }

    /// Debug/e2e: a free standing point `[x, z]` next to an animal (the walkable cell centre
    /// closest to it, at least 0.9 m away) for a scripted walk; empty if unknown.
    pub fn debug_stand_near(&self, id: &str) -> Vec<f32> {
        let Some(a) = self.game.animal(id) else {
            return Vec::new();
        };
        let grid = self.game.level.grid();
        let best = grid
            .bounds()
            .cells()
            .filter(|&c| grid.is_passable(c, false))
            .map(zoo_core::level::cell_center)
            .filter(|p| p.distance(a.pos) >= 0.9)
            .min_by(|x, y| x.distance(a.pos).total_cmp(&y.distance(a.pos)));
        best.map(|p| vec![p.x, p.y]).unwrap_or_default()
    }

    /// Debug/e2e: turns the player towards an animal (as a short joystick tap would).
    pub fn debug_face_animal(&mut self, id: &str) -> bool {
        let Some(a) = self.game.animal(id) else {
            return false;
        };
        let to = a.pos - self.game.player.pos;
        if to.length() < 1e-3 {
            return false;
        }
        self.game.player.facing = to.normalize();
        true
    }

    /// Debug/e2e: the animal walks into its own enclosure as if led home (GAME-RESCUE §8).
    pub fn debug_send_home(&mut self, id: &str) -> bool {
        let ok = self.game.debug_send_home(id);
        self.handle_events();
        ok
    }

    pub fn mission_started(&self, id: &str) -> bool {
        self.game.mission(id).is_some_and(|m| m.started)
    }

    pub fn mission_complete(&self, id: &str) -> bool {
        self.game.mission(id).is_some_and(|m| m.complete)
    }

    /// Debug/e2e: puts the player at a level position (no collision check) and snaps the
    /// camera there.
    pub fn debug_teleport(&mut self, x: f32, z: f32) {
        self.game.player.pos = Vec2::new(x, z);
        self.game.player.y = self.game.level.ground_height(self.game.player.pos);
        self.camera.snap(player_feet(&self.game));
    }

    /// Debug/e2e scripted player: walks to a level position along a grid path with the
    /// normal movement and collision (PROD-POC M4). Cleared on arrival.
    pub fn debug_goto(&mut self, x: f32, z: f32) {
        self.autopilot = Some(Autopilot::new(Vec2::new(x, z)));
    }

    /// Debug/e2e: runs the simulation (autopilot, game, animals) for up to `seconds` of game
    /// time in 1/60 s steps without rendering, stopping early when a scripted walk arrives.
    /// Returns whether no scripted walk is pending. Keeps e2e runs independent of the frame
    /// rate of software WebGL.
    pub fn debug_step(&mut self, seconds: f32) -> bool {
        let dt = 1.0 / 60.0;
        let n = (seconds / dt).round() as usize;
        let walking = self.autopilot.is_some();
        for _ in 0..n {
            self.time += dt as f64;
            self.simulate(dt);
            if walking && self.autopilot.is_none() {
                break;
            }
        }
        self.camera.snap(player_feet(&self.game));
        self.autopilot.is_none()
    }

    /// Debug/e2e (PLAY-036): height of the player's feet as drawn (m, world Y).
    pub fn player_foot_y(&self) -> f32 {
        self.game.player.y
    }

    /// Debug/e2e (PLAY-035/036): `ground_height` at a level position (m).
    pub fn ground_height(&self, x: f32, z: f32) -> f32 {
        self.game.level.ground_height(Vec2::new(x, z))
    }

    /// Debug/e2e: height of an animal's origin as drawn (m, world Y).
    pub fn animal_foot_y(&self, id: &str) -> f32 {
        self.animals
            .iter()
            .find(|v| v.id == id)
            .map_or(f32::NAN, |v| v.ground - v.sink + v.lift)
    }

    /// Whether the scripted walk of [`App::debug_goto`] has ended.
    pub fn debug_arrived(&self) -> bool {
        self.autopilot.is_none()
    }

    /// Seconds of game time since start.
    pub fn time(&self) -> f64 {
        self.time
    }

    // ------------------------------------------------------------------ living water, ambient

    /// Debug/e2e: stops the clock (frames still render; `debug_step` advances time), so
    /// screenshots at fixed times are reproducible (WATER-006/007, the GIF).
    pub fn debug_pause(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// Debug/e2e: sets the game clock (water animation time) in seconds.
    pub fn debug_set_time(&mut self, t: f64) {
        self.time = t;
    }

    /// Debug/e2e: water animation on/off (off = flat water tiles, no bobbing; WATER-008).
    pub fn set_water_animation(&mut self, on: bool) {
        self.renderer.water_animation = on;
    }

    /// Debug/e2e: ambient animals on/off (AMB-007 A/B measurement).
    pub fn set_ambient(&mut self, on: bool) {
        self.ambient_on = on;
    }

    /// Water clock of the last frame (s in [0, 16)).
    pub fn water_clock(&self) -> f32 {
        self.renderer.water_clock()
    }

    /// Draw calls of the ambient animals in the last frame (AMB-007).
    pub fn ambient_draw_calls(&self) -> u32 {
        self.renderer.stats.crowd_draw_calls + u32::from(!self.butterflies.is_empty())
    }

    /// Ambient animals as JSON `[{kind, x, z, clip, water}]` (level coordinates).
    pub fn ambient_json(&self) -> String {
        let items: Vec<String> = self
            .ambient
            .animals
            .iter()
            .map(|a| {
                format!(
                    r#"{{"kind":"{}","x":{:.3},"z":{:.3},"clip":"{}","water":{}}}"#,
                    a.kind.model(),
                    a.pos.x,
                    a.pos.y,
                    a.action.map_or("", |x| x.clip),
                    a.in_water
                )
            })
            .collect();
        format!("[{}]", items.join(","))
    }

    /// Screen position (CSS px, y down) of a level point at a height, or empty if behind the
    /// camera.
    pub fn screen_point(&self, x: f32, z: f32, y: f32) -> Vec<f32> {
        let (w, h) = self.renderer.size();
        let ratio = self.renderer.pixel_ratio();
        let vp = self.camera.view_proj(self.renderer.aspect());
        let p = level_to_world(Vec2::new(x, z)) + Vec3::Y * y;
        match zoo_render::camera::project(vp, p) {
            Some(ndc) => vec![
                (ndc.x * 0.5 + 0.5) * w as f32 / ratio,
                (0.5 - ndc.y * 0.5) * h as f32 / ratio,
            ],
            None => Vec::new(),
        }
    }

    /// Debug/e2e: places the camera target (look at a level point) without moving the
    /// player, e.g. to frame the river and the pond for screenshots.
    pub fn debug_look_at(&mut self, x: f32, z: f32) {
        self.look_at = Some(Vec2::new(x, z));
        self.camera.snap(level_to_world(Vec2::new(x, z)));
    }

    /// Debug/e2e: the camera follows the player again.
    pub fn debug_look_at_player(&mut self) {
        self.look_at = None;
    }

    // ------------------------------------------------------------------ M5b: levels, bowl

    /// Whether a level (`level_1` …) is unlocked (its entry barrier is open).
    pub fn level_unlocked(&self, id: &str) -> bool {
        self.game.level_unlocked(id)
    }

    /// Ids of the joined levels, one per line.
    pub fn level_ids(&self) -> String {
        self.game
            .level
            .data
            .parts
            .iter()
            .map(|p| p.id.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Spawn point `[x, z]` (cell centre) of a level, empty if unknown.
    pub fn level_spawn(&self, id: &str) -> Vec<f32> {
        let data = &self.game.level.data;
        data.part_index(id)
            .map(|k| {
                let c = zoo_core::level::cell_center(data.parts[k].spawn.cell());
                vec![c.x, c.y]
            })
            .unwrap_or_default()
    }

    /// Whether a barrier is open (removed).
    pub fn barrier_open(&self, id: &str) -> bool {
        self.game.level.is_barrier_open(id)
    }

    /// Whether the renderer hides a barrier's models (opened) or a building's roof (inside).
    pub fn region_hidden(&self, id: &str) -> bool {
        if let Some((_, h, _)) = self.building_models.iter().find(|(k, ..)| k == id) {
            // a building model: its roof part is hidden by the instance's hide mask
            return self
                .renderer
                .instance(*h)
                .is_some_and(|i| (i.node[1] as u32) & HIDE_ROOF != 0);
        }
        self.barrier_regions
            .iter()
            .chain(&self.roof_regions)
            .find(|(k, _)| k == id)
            .is_some_and(|(_, r)| self.renderer.region_hidden(*r))
    }

    /// The enterable building the player is inside (its roof is hidden), or empty.
    pub fn player_inside(&self) -> String {
        self.building_inside().unwrap_or_default()
    }

    /// Carried bowl for the HUD: empty (not carried), `empty`, `water` or `fish` (RESC-020).
    pub fn carry_bowl(&self) -> String {
        match &self.game.bowl {
            Some(b) if b.carried => {
                if b.fish {
                    "fish"
                } else if b.water {
                    "water"
                } else {
                    "empty"
                }
            }
            _ => "",
        }
        .to_owned()
    }

    /// The fish bowl as JSON `{"x","z","carried","water","fish"}` or empty (tests).
    pub fn bowl_json(&self) -> String {
        self.game
            .bowl
            .as_ref()
            .map(|b| {
                format!(
                    "{{\"x\":{},\"z\":{},\"carried\":{},\"water\":{},\"fish\":{}}}",
                    b.pos.x, b.pos.y, b.carried, b.water, b.fish
                )
            })
            .unwrap_or_default()
    }

    /// Height of an animal above the ground (perch, Q-094) as drawn.
    pub fn animal_lift(&self, id: &str) -> f32 {
        self.animals
            .iter()
            .find(|v| v.id == id)
            .map_or(0.0, |v| v.lift)
    }

    /// Whether an animal is drawn (its level is unlocked).
    pub fn animal_visible(&self, id: &str) -> bool {
        self.animals.iter().any(|v| v.id == id && v.visible)
    }

    /// Static batches skipped by region culling in the last frame (QA F12).
    pub fn culled_batches(&self) -> u32 {
        self.renderer.stats.culled_batches
    }

    /// Debug/e2e: a free standing point `[x, z]` near a level point (the passable cell centre
    /// closest to it, at least `min_m` away) — e.g. next to the fish bowl or at a bank.
    pub fn debug_stand_near_point(&self, x: f32, z: f32, min_m: f32) -> Vec<f32> {
        let p = Vec2::new(x, z);
        let grid = self.game.level.grid();
        let r = zoo_core::level::Rect::new(x as i32 - 4, z as i32 - 4, 9, 9);
        r.cells()
            .filter(|&c| grid.is_passable(c, false))
            .map(zoo_core::level::cell_center)
            .filter(|q| q.distance(p) >= min_m)
            .min_by(|a, b| a.distance(p).total_cmp(&b.distance(p)))
            .map(|q| vec![q.x, q.y])
            .unwrap_or_default()
    }

    // ------------------------------------------------------------------ night (GAME-NIGHT)

    /// Time of day: `day`, `dusk`, `night`, `sleeping`, `morning`.
    pub fn daytime(&self) -> String {
        self.game.daytime.phase.id().to_owned()
    }

    /// Night light of the time of day `[night 0…1, warm 0…1]`.
    pub fn daylight(&self) -> Vec<f32> {
        let l = self.game.daytime.light();
        vec![l.night, l.warm]
    }

    /// Dream fade of the sleep (0 … 1) for the host overlay (NIGHT-003).
    pub fn sleep_fade(&self) -> f32 {
        self.game.daytime.sleep_fade()
    }

    /// Debug/e2e: jumps to a time of day (`day`, `dusk`, `night`, `sleeping`, `morning`;
    /// `night` opens the moon door, `day` / `morning` apply the morning). False if unknown.
    pub fn debug_set_daytime(&mut self, id: &str) -> bool {
        let ok = self.game.debug_set_daytime(id);
        self.handle_events();
        self.camera.snap(player_feet(&self.game));
        ok
    }

    /// Debug/e2e: the rest of the night at once (night → the next morning, day): pending
    /// barriers open; the player stays where she is (unless she was in the night zoo).
    pub fn debug_next_morning(&mut self) {
        let _ = self.game.debug_set_daytime("night");
        let _ = self.game.debug_set_daytime("day");
        self.handle_events();
        self.camera.snap(player_feet(&self.game));
    }

    /// Whether an animal's eyes shine now (lantern light, NIGHT-006).
    pub fn eyes_shine(&self, id: &str) -> bool {
        self.game
            .animal(id)
            .is_some_and(|a| self.game.eyes_shine(a))
    }

    /// Point lights and light pools of the last frame `[lights, pools]` and the number of
    /// lamps of the zoo (Q-114, performance report).
    pub fn light_stats(&self) -> Vec<u32> {
        let (l, p) = self.renderer.light_counts();
        vec![l, p, self.lamps.len() as u32]
    }

    /// Debug (PERF-025): `true` draws all static geometry without frustum culling (regions,
    /// chunks, chunk ranges), to check that culling never changes the picture (no popping).
    pub fn debug_no_culling(&mut self, on: bool) {
        self.renderer.no_culling = on;
    }

    /// Debug (PERF-025, PERF-R-018): `false` switches the haze culling of the close views off
    /// (default on, Q-193), so the frustum culling can be compared on its own.
    pub fn debug_haze_cull(&mut self, on: bool) {
        self.renderer.haze_cull = on;
    }

    /// Debug (PERF-017): `true` gives every draw every light (no per-draw light masks), to
    /// compare the masked frame with the unmasked one pixel by pixel.
    pub fn debug_full_light_masks(&mut self, on: bool) {
        self.renderer.full_light_masks = on;
    }

    /// Ids of the moon doors, one per line.
    pub fn moon_doors(&self) -> String {
        self.game.moon_doors().join("\n")
    }

    /// Where the bed is `[x, z]` (level), empty if the zoo has none.
    pub fn bed_point(&self) -> Vec<f32> {
        self.game
            .bed()
            .map(|(p, _)| vec![p.x, p.y])
            .unwrap_or_default()
    }

    /// Where the child stands to use the nearest bed `[x, z]` (level), empty if none
    /// (NIGHT-016).
    pub fn bed_stand(&self) -> Vec<f32> {
        self.game
            .bed_stand(self.game.player.pos)
            .map(|p| vec![p.x, p.y])
            .unwrap_or_default()
    }

    /// Whether the player is in a night level.
    pub fn player_in_night_zoo(&self) -> bool {
        self.game.player_in_night_zoo()
    }

    /// Debug/e2e: turns the player towards a level point.
    pub fn debug_face_point(&mut self, x: f32, z: f32) -> bool {
        let to = Vec2::new(x, z) - self.game.player.pos;
        if to.length() < 1e-3 {
            return false;
        }
        self.game.player.facing = to.normalize();
        true
    }
}

impl App {
    /// Gates and doors open and close visibly (LAYOUT-031): eased towards what the game
    /// wants; an enclosure gate stays open a moment behind the animals.
    fn update_openings(&mut self, dt: f32) {
        let openings = self.game.level.openings();
        for (o, v) in openings.iter().zip(self.openings.iter_mut()) {
            let Some(h) = v.handle else { continue };
            let want = self.game.opening_open(o);
            if want {
                v.hold = if v.enclosure_gate { GATE_HOLD_S } else { 0.0 };
            } else {
                v.hold = (v.hold - dt).max(0.0);
            }
            let target = if want || v.hold > 0.0 { 1.0 } else { 0.0 };
            let before = v.open;
            if target > v.open {
                v.open = (v.open + OPEN_RATE * dt).min(1.0);
            } else if target < v.open {
                v.open = (v.open - CLOSE_RATE * dt).max(0.0);
            }
            if v.open == before && dt > 0.0 {
                continue;
            }
            // ease in and out
            let k = v.open * v.open * (3.0 - 2.0 * v.open);
            let mut i = v.base;
            if v.swing != 0.0 {
                i.pos_yaw[3] = v.base.pos_yaw[3] + v.swing * k;
            } else {
                i.node[0] = k;
            }
            self.renderer.set_instance(h, i);
        }
    }

    /// Garden plants show the model of their growth stage (GAME-GARDEN §3).
    fn update_plants(&mut self) {
        for v in &mut self.plants {
            let stage = self.game.garden.plant(&v.spot).map(|p| p.stage());
            let shown = match stage {
                Some(zoo_core::garden::Stage::Sprout) => Some(0),
                Some(zoo_core::garden::Stage::Young) => Some(1),
                Some(zoo_core::garden::Stage::Ripe) => Some(2),
                _ => None,
            };
            if shown == v.shown {
                continue;
            }
            for k in 0..3 {
                if let Some(h) = v.stages[k] {
                    let mut i = v.base[k];
                    if Some(k) != shown {
                        i.scale_fade = [0.0; 4];
                    }
                    self.renderer.set_instance(h, i);
                }
            }
            v.shown = shown;
        }
    }

    /// Night presentation of a frame (GAME-NIGHT §10): global light, night-only props,
    /// the player's lantern + the nearest lamps as point lights, light pools for the rest,
    /// eyeshine and fireflies (emissive dynamic boxes).
    fn night_frame(&mut self, player: Vec3) {
        let l = self.game.daytime.light();
        self.renderer.set_daylight(DayLight {
            night: l.night,
            warm: l.warm,
        });
        // lamps switch on during dusk (rule 1); their glow slots with them
        let lamps_on = self.game.daytime.lamps_on();
        self.renderer.set_glow(lamps_on);
        for r in &self.night_regions {
            if self.renderer.region_hidden(*r) == lamps_on {
                self.renderer.set_region_hidden(*r, !lamps_on);
            }
        }
        self.frame_lights.clear();
        self.frame_pools.clear();
        let mut lantern = 0;
        if lamps_on {
            // the player's hand lantern: a light circle around her, always (NIGHT-005)
            // (centre above her hands: she is lit from head to toe, her pool on the ground
            // ≈ 1.8 m around her; eyes shine within LANTERN_RADIUS_M, NIGHT-006)
            self.frame_lights.push(PointLight {
                pos: player + Vec3::Y * zoo_core::night::LANTERN_LIGHT_Y_M,
                // ground pool ≈ the 2.5 m eyeshine radius (Q-142)
                radius: zoo_core::night::lantern_light_radius(),
                color: Vec3::new(1.0, 0.95, 0.84),
                strength: 1.0,
                // (a lamp: flat cream pool on the ground; she keeps her colours, NIGHT-005)
                tinted: false,
            });
            // the hand lantern in her left hand (`hand_lantern`: its grip on the hand)
            if !self.camera.hides_player() {
                let yaw = self.player_yaw;
                let left = Quat::from_rotation_y(yaw) * Vec3::X;
                let fwd = Quat::from_rotation_y(yaw) * Vec3::Z;
                let hand = player + left * 0.3 + fwd * 0.08 + Vec3::Y * 0.42;
                if self.renderer.has_model(HAND_LANTERN) {
                    self.hand_lantern[0] = Instance::model(hand - HAND_LANTERN_GRIP, yaw, false);
                    lantern = 1;
                } else {
                    self.glows.push(Instance::glow(
                        hand,
                        yaw,
                        Vec3::new(0.16, 0.2, 0.16),
                        nightfx::LAMP_GLOW.to_array(),
                    ));
                }
            }
            nightfx::pick_lamps(
                &self.lamps,
                self.camera.target,
                nightfx::LAMP_CULL_M,
                // the lantern + the tier's lamps as point lights (PERF-BUDGETS rule 5)
                nightfx::MAX_POINT_LIGHTS - self.quality.tier().lamp_lights(),
                &mut self.lamp_order,
                &mut self.frame_lights,
                &mut self.frame_pools,
            );
            // fireflies: tiny blinking dots (Q-115)
            let t = self.time as f32;
            for (c, ph) in &self.fireflies {
                let blink = (t * 1.3 + ph).sin();
                if blink < -0.2 {
                    continue;
                }
                let p = *c
                    + Vec3::new(
                        (t * 0.7 + ph).sin() * 0.8,
                        (t * 1.1 + ph * 2.0).sin() * 0.25,
                        (t * 0.5 + ph * 3.0).cos() * 0.8,
                    );
                self.glows.push(Instance::glow(
                    p,
                    0.0,
                    Vec3::splat(0.07),
                    nightfx::FIREFLY_GLOW.to_array(),
                ));
            }
        }
        self.renderer.set_point_lights(&self.frame_lights);
        self.renderer.set_light_pools(&self.frame_pools);
        self.renderer.set_dynamic_instances(DYN_GLOW, &self.glows);
        self.renderer
            .set_dynamic_instances(HAND_LANTERN, &self.hand_lantern[..lantern]);
        self.ambient.night = self.game.daytime.is_dark();
    }

    /// Switches the camera view; leaving first person unlocks the facing (GAME-CAMERA-VIEWS).
    fn set_view(&mut self, mode: ViewMode) {
        let facing = views::level_to_yaw(self.game.player.facing);
        self.camera.set_view(mode, facing);
        if mode != ViewMode::FirstPerson {
            self.game.player.lock_facing(None);
        }
    }

    /// Input → game update → events → animal presentation.
    fn simulate(&mut self, dt: f32) {
        let mut stick = self.stick;
        let k = self.keys;
        let first_person = self.camera.mode() == ViewMode::FirstPerson;
        // arrow keys ← → turn in first person (GAME-CAMERA-VIEWS 3), else walk sideways
        let arrows = f32::from(u8::from(k.arrow_right)) - f32::from(u8::from(k.arrow_left));
        let (side, turn) = if first_person {
            (0.0, arrows)
        } else {
            (arrows, 0.0)
        };
        if turn != 0.0 {
            let a = views::KEY_TURN_DEG_S.to_radians() * dt * turn;
            self.camera.turn(-a, 0.0);
        }
        let kv = Vec2::new(
            (f32::from(u8::from(k.right)) - f32::from(u8::from(k.left)) + side).clamp(-1.0, 1.0),
            f32::from(u8::from(k.up)) - f32::from(u8::from(k.down)),
        );
        // first person: the facing is the view direction (CAMV-006)
        self.game
            .player
            .lock_facing(first_person.then(|| self.camera.look_level()));
        if kv != Vec2::ZERO {
            stick = kv.normalize();
        }
        let mut dir = self.camera.stick_to_level(stick);
        if let Some(ap) = &mut self.autopilot {
            match ap.input(
                self.game.level.grid(),
                self.game.player.pos,
                self.game.is_leading(),
            ) {
                Some(d) => dir = d,
                None => self.autopilot = None,
            }
        }
        self.game.update(dt, dir);
        self.handle_events();
        self.hints.update(&self.game, dt);
        self.update_animals(dt);
        if self.ambient_on {
            self.ambient.update(dt, self.game.player.pos);
        }
    }

    /// What the reading panel of a target shows.
    fn panel_interaction(&self, t: &Target) -> Option<Interaction> {
        match t {
            Target::InfoBoard { animal } => {
                self.game.info_board(animal).map(Interaction::InfoBoard)
            }
            Target::FoodBox { food } => Some(Interaction::FoodBox {
                food: *food,
                label: zoo_core::FoodBox { food: *food }.label(self.game.settings.reading_level),
            }),
            Target::GardenSign { bed } => {
                let b = self
                    .game
                    .level
                    .data
                    .garden_beds
                    .iter()
                    .find(|b| &b.id == bed)?;
                Some(Interaction::GardenSign {
                    bed: b.id.clone(),
                    key: b.sign_key.clone(),
                })
            }
            _ => None,
        }
    }

    /// JSON for the host (text panel content) of an interaction result.
    fn interaction_json(&self, result: &Interaction) -> String {
        match result {
            Interaction::InfoBoard(b) => format!(
                "{{\"kind\":\"info_board\",\"key\":{},\"animal\":{},\"title\":{},\"more\":{},\"facts\":{},\"text\":{},\"picture\":{},\"food\":{},\"food_text\":{},\"level\":{},\"hint\":{}}}",
                js(&format!("info_board:{}", b.animal)),
                js(b.animal),
                js(&self.text_now(&b.name_key)),
                js(&self.text_now(&b.more_key)),
                js(&self.text_now(&b.facts_key)),
                js(&self.text_now(&b.riddle_key)),
                b.picture.as_deref().map_or("null".to_owned(), js),
                js(b.food_key.trim_start_matches("food-")),
                js(&self.text_now(&b.food_key)),
                js(self.game.settings.reading_level.id()),
                // animals that travel in a container: the board says a bowl is needed
                if self.game.needs_container(b.animal) {
                    js(&self.text_now(&format!(
                        "mission-{}-bowl-hint-{}",
                        b.animal,
                        self.game.settings.reading_level.id()
                    )))
                } else {
                    "null".to_owned()
                },
            ),
            Interaction::FoodBox { food, label } => format!(
                "{{\"kind\":\"food_box\",\"key\":{},\"food\":{},\"text\":{},\"picture\":{},\"take\":{}}}",
                js(&format!("food_box:{}", food.id())),
                js(food.id()),
                js(&self.text_now(&label.word_key)),
                label.picture,
                js(&self.text_now("ui-take")),
            ),
            Interaction::ShowFood {
                animal, accepted, ..
            } => format!(
                "{{\"kind\":\"show_food\",\"animal\":{},\"accepted\":{accepted}}}",
                js(animal)
            ),
            Interaction::Gate { enclosure, entered } => format!(
                "{{\"kind\":\"gate\",\"enclosure\":{},\"entered\":{entered}}}",
                js(enclosure)
            ),
            Interaction::Item { id, action } => format!(
                "{{\"kind\":\"item\",\"id\":{},\"action\":{}}}",
                js(id),
                js(action)
            ),
            Interaction::Sleep => "{\"kind\":\"sleep\"}".to_owned(),
            Interaction::Harvest { spot, treat, count } => format!(
                "{{\"kind\":\"harvest\",\"spot\":{},\"treat\":{},\"count\":{count}}}",
                js(spot),
                js(treat.id())
            ),
            Interaction::GardenSign { bed, key } => {
                // the word (GARD-009) and, from klasse1 on, a sentence (proposal Q-103)
                let level = self.game.settings.reading_level.id();
                let sentence = format!("{key}-{level}");
                let text = self
                    .content
                    .as_ref()
                    .and_then(|c| c.text(self.game.settings.language, &sentence))
                    .unwrap_or_default();
                format!(
                    "{{\"kind\":\"garden_sign\",\"key\":{},\"food\":{},\"title\":{},\"text\":{},\"picture\":true}}",
                    js(&format!("garden_sign:{bed}")),
                    js(key.trim_start_matches("garden-")),
                    js(&self.text_now(key)),
                    js(&text)
                )
            }
            Interaction::Treat {
                animal,
                treat,
                accepted,
            } => format!(
                "{{\"kind\":\"treat\",\"animal\":{},\"treat\":{},\"accepted\":{accepted}}}",
                js(animal),
                js(treat.id())
            ),
            Interaction::MoonDoor { into_night_zoo } => {
                format!("{{\"kind\":\"moon_door\",\"into_night_zoo\":{into_night_zoo}}}")
            }
        }
    }

    fn text_now(&self, key: &str) -> String {
        self.content
            .as_ref()
            .and_then(|c| c.text(self.game.settings.language, key))
            .unwrap_or_else(|| key.to_owned())
    }

    fn say(&mut self, animal: &str, key: &str) {
        let text = self.text_now(key);
        self.outbox.push(format!(
            "{{\"type\":\"say\",\"animal\":{},\"key\":{},\"text\":{}}}",
            js(animal),
            js(key),
            js(&text)
        ));
    }

    fn queue(&mut self, animal: &str, clips: &[&'static str], wait_arrival: bool) {
        for v in self.animals.iter_mut().filter(|v| v.id == animal) {
            v.queue.clear();
            v.action = None;
            v.queue.extend(clips.iter().copied());
            v.wait_arrival = wait_arrival;
        }
    }

    /// Starts the leap arc of an animal from where it is drawn now (the goldfish jumping into
    /// the bowl or out of it into its pond).
    fn leap(&mut self, animal: &str, into_bowl: bool) {
        let fwd = Quat::from_rotation_y(self.player_yaw) * Vec3::Z;
        let player = player_feet(&self.game);
        let bowl = self.bowl_base(fwd, player);
        for v in self.animals.iter_mut().filter(|v| v.id == animal) {
            let from = if into_bowl {
                level_to_world(v.pos) + Vec3::Y * (v.ground - v.sink)
            } else {
                // out of the bowl on the step: from the bowl's water surface
                bowl.map_or(level_to_world(v.pos), |b| {
                    b + Vec3::Y * (BOWL_HEIGHT_M * 0.74)
                })
            };
            v.leap = Some((from, 0.0, into_bowl));
        }
    }

    /// Game events → animation clips and host feedback (GAME-RESCUE §11).
    fn handle_events(&mut self) {
        for e in self.game.drain_events() {
            self.hints.observe(&e);
            match e {
                GameEvent::StartedFollowing { animal } => {
                    self.queue(&animal, &["happy"], false);
                    self.say(&animal, "ui-following");
                }
                GameEvent::NotInterested { animal } => {
                    self.queue(&animal, &["refuse"], false);
                    let key = if self.game.carry.food().is_some() {
                        "ui-not-interested"
                    } else {
                        "ui-no-food"
                    };
                    self.say(&animal, key);
                }
                GameEvent::Refuse { animal, .. } => {
                    self.queue(&animal, &["refuse"], false);
                    self.say(&animal, "ui-refuse");
                }
                GameEvent::InEnclosure { animal } => {
                    if self.game.needs_container(&animal) {
                        // from the bowl on the stone step into its pond
                        self.leap(&animal, false);
                        self.queue(&animal, &["happy", "eat"], false);
                    } else {
                        self.queue(&animal, &["eat", "happy", "happy"], true);
                    }
                }
                GameEvent::NeedsContainer { animal } => {
                    self.queue(&animal, &["happy"], false);
                    self.say(&animal.clone(), &format!("mission-{animal}-needs-bowl"));
                }
                GameEvent::ContainerEmpty { animal } => {
                    self.queue(&animal, &["refuse"], false);
                    self.say(&animal.clone(), &format!("mission-{animal}-bowl-empty"));
                }
                GameEvent::ContainerFilled { animal, .. } => {
                    self.say(&animal.clone(), &format!("mission-{animal}-bowl-filled"));
                }
                GameEvent::InContainer { animal } => {
                    self.leap(&animal, true);
                    self.queue(&animal, &["happy"], false);
                    self.say(&animal.clone(), &format!("mission-{animal}-in-bowl"));
                }
                GameEvent::BarrierOpened { id } => {
                    self.outbox.push(format!(
                        "{{\"type\":\"barrier_opened\",\"id\":{}}}",
                        js(&id)
                    ));
                }
                GameEvent::LevelComplete { level } => {
                    // the night zoo is complete: a gentle "time to sleep" text (GAME-NIGHT 7)
                    let night = self
                        .game
                        .level
                        .data
                        .part_index(&level)
                        .is_some_and(|k| self.game.level.data.is_night_part(k));
                    let key = format!("night-complete-{}", self.game.settings.reading_level.id());
                    let text = if night {
                        self.text_now(&key)
                    } else {
                        String::new()
                    };
                    self.outbox.push(format!(
                        "{{\"type\":\"level_complete\",\"level\":{},\"key\":{},\"text\":{}}}",
                        js(&level),
                        js(&key),
                        js(&text)
                    ));
                }
                GameEvent::MissionComplete { animal } => {
                    let key = format!("mission-{animal}-home");
                    let text = self.text_now(&key);
                    self.outbox.push(format!(
                        "{{\"type\":\"mission_complete\",\"animal\":{},\"key\":{},\"text\":{}}}",
                        js(&animal),
                        js(&key),
                        js(&text)
                    ));
                }
                GameEvent::PanelOpened { target } => {
                    if let Some(i) = self.panel_interaction(&target) {
                        let panel = self.interaction_json(&i);
                        self.outbox
                            .push(format!("{{\"type\":\"panel_open\",\"panel\":{panel}}}"));
                    }
                }
                GameEvent::PanelClosed { target } => {
                    self.outbox.push(format!(
                        "{{\"type\":\"panel_close\",\"key\":{}}}",
                        js(&target_key(&target))
                    ));
                }
                GameEvent::DuskStarted => {
                    // a gentle cut-in text per reading level (GAME-NIGHT rule 2, Fluent)
                    let key = format!("night-dusk-{}", self.game.settings.reading_level.id());
                    let text = self.text_now(&key);
                    self.outbox.push(format!(
                        "{{\"type\":\"dusk\",\"key\":{},\"text\":{}}}",
                        js(&key),
                        js(&text)
                    ));
                }
                GameEvent::NightFell => self.outbox.push("{\"type\":\"night\"}".to_owned()),
                GameEvent::PutDownRefused => self
                    .outbox
                    .push("{\"type\":\"put_down_refused\"}".to_owned()),
                GameEvent::FoodPutBack { food } => self.outbox.push(format!(
                    "{{\"type\":\"food_put_back\",\"food\":{}}}",
                    js(food.id())
                )),
                GameEvent::BambooCut { spot } => self.outbox.push(format!(
                    "{{\"type\":\"bamboo_cut\",\"spot\":{}}}",
                    js(&spot)
                )),
                GameEvent::SleepStarted => self.outbox.push("{\"type\":\"sleep\"}".to_owned()),
                GameEvent::Morning => {
                    self.camera.snap(player_feet(&self.game));
                    let key = format!("night-morning-{}", self.game.settings.reading_level.id());
                    let text = self.text_now(&key);
                    self.outbox.push(format!(
                        "{{\"type\":\"morning\",\"key\":{},\"text\":{}}}",
                        js(&key),
                        js(&text)
                    ));
                }
                GameEvent::DayStarted => self.outbox.push("{\"type\":\"day\"}".to_owned()),
                GameEvent::MoonDoor { into_night_zoo } => {
                    self.camera.snap(player_feet(&self.game));
                    self.player_yaw = facing_to_yaw(self.game.player.facing);
                    let key = format!("night-welcome-{}", self.game.settings.reading_level.id());
                    let text = if into_night_zoo {
                        self.text_now(&key)
                    } else {
                        String::new()
                    };
                    self.outbox.push(format!(
                        "{{\"type\":\"moon_door\",\"into_night_zoo\":{into_night_zoo},\"key\":{},\"text\":{}}}",
                        js(&key),
                        js(&text)
                    ));
                }
                GameEvent::Harvested { treat, count, .. } => {
                    self.outbox.push(format!(
                        "{{\"type\":\"harvest\",\"treat\":{},\"count\":{count},\"text\":{}}}",
                        js(treat.id()),
                        js(&self.text_now(treat.label_key()))
                    ));
                }
                GameEvent::BasketFull => {
                    let text = self.text_now("garden-basket-full");
                    self.outbox.push(format!(
                        "{{\"type\":\"say\",\"animal\":\"\",\"key\":\"garden-basket-full\",\"text\":{}}}",
                        js(&text)
                    ));
                }
                GameEvent::TreatEaten { animal, .. } => {
                    self.queue(&animal, &["eat", "happy"], false);
                    self.say(&animal, "garden-treat-yum");
                }
                GameEvent::BabyBorn { animal } => {
                    // no baby model yet (FAM-007, ART-ANIMALS): the celebration is the message
                    self.queue(&animal, &["happy"], false);
                    self.say(&animal, "family-baby");
                }
                GameEvent::TreatRefused { animal, .. } => {
                    self.queue(&animal, &["refuse"], false);
                    self.say(&animal, "ui-not-interested");
                }
                GameEvent::FoodTaken { food, .. } => {
                    self.outbox.push(format!(
                        "{{\"type\":\"food_taken\",\"food\":{},\"text\":{}}}",
                        js(food.id()),
                        js(&self.text_now(&food.label_key()))
                    ));
                }
                _ => {}
            }
        }
    }

    /// Snaps every animal view to its logic state (start, new game, restore).
    fn reset_views(&mut self) {
        for (i, v) in self.animals.iter_mut().enumerate() {
            let Some(a) = self.game.animals.get(i) else {
                continue;
            };
            v.pos = a.pos;
            v.yaw = facing_to_yaw(a.facing);
            v.queue.clear();
            v.action = None;
            v.wait_arrival = false;
            v.walk_blend = 0.0;
            v.leap = None;
            let depth = self.game.water_depth(a);
            v.sink = swim_sink_m(v.id) * depth;
            v.lift = self.game.perch(a).map_or(0.0, |(_, h)| h);
            v.ground = if self.game.perch(a).is_some() {
                0.0
            } else {
                self.game.level.ground_height(a.pos)
            };
            v.rest = static_clip(self.game.rest_clip(a));
            v.locomotion = if depth > 0.5 { "swim" } else { "walk" };
            v.visible = self.game.in_scope(a);
        }
    }

    /// Where the bowl stands (world, bottom centre): in front of the player's chest when
    /// carried (both hands, `socket_carry` approximation), else on its table / step / ground.
    fn bowl_base(&self, fwd: Vec3, player: Vec3) -> Option<Vec3> {
        let b = self.game.bowl.as_ref()?;
        if b.carried {
            return Some(player + fwd * 0.42 + Vec3::Y * 0.42);
        }
        let part = self
            .game
            .level
            .data
            .part_at(zoo_core::level::cell_of(b.pos))
            .unwrap_or(0);
        self.game
            .part_unlocked(part)
            .then(|| zoo_core::coords::level_to_world_at(b.pos, b.lift_m))
    }

    /// The enterable building whose interior or door cell the player stands on (PLAY-028).
    fn building_inside(&self) -> Option<String> {
        let c = zoo_core::level::cell_of(self.game.player.pos);
        self.game
            .level
            .data
            .elements
            .iter()
            .find(|e| e.is_enterable() && e.is_open_cell(c))
            .map(|e| e.id.clone())
    }

    fn update_animals(&mut self, dt: f32) {
        let walk_speed = self.game.move_params.walk_speed;
        let fwd = Quat::from_rotation_y(self.player_yaw) * Vec3::Z;
        let player = player_feet(&self.game);
        let bowl = self.bowl_base(fwd, player);
        for (i, v) in self.animals.iter_mut().enumerate() {
            let Some(a) = self.game.animals.get(i) else {
                continue;
            };
            v.visible = self.game.in_scope(a);
            // in the bowl: at its water surface (origin = water surface, fish rig)
            v.anchor = match (a.state, bowl) {
                (AnimalState::InBowl, Some(base)) => Some(base + Vec3::Y * (BOWL_HEIGHT_M * 0.74)),
                _ => None,
            };
            if let Some((from, t, into)) = v.leap {
                let t = t + dt;
                v.leap = (t < LEAP_S).then_some((from, t, into));
            }
            let to = a.pos - v.pos;
            let dist = to.length();
            // bat / owl fly (`fly_height`, ART-ANIMALS "Night animals", NIGHT-017)
            let fly_height = if v.skinned {
                self.anims.fly_height(v.id)
            } else {
                None
            };
            let moved = if dist > 12.0 || v.anchor.is_some() {
                v.pos = a.pos;
                0.0
            } else if v.lift > 0.3 && a.state == AnimalState::Following && fly_height.is_none() {
                0.0 // still coming down from its perch
            } else {
                let step = (2.2 * dt).min(dist);
                if step > 1e-5 {
                    v.pos += to / dist * step;
                }
                step
            };
            let speed = if dt > 0.0 { moved / dt } else { 0.0 };
            // perch (Q-094): up there while escaped, climbs down when it follows
            let perch = self.game.perch(a);
            let flyer = fly_height.map(|h| {
                let pose = self
                    .game
                    .level
                    .data
                    .hiding_place(&a.hiding_place)
                    .and_then(|p| p.pose.as_deref())
                    .unwrap_or("idle");
                zoo_core::night::flyer_pose(a.state, perch.map(|(_, h)| h), pose, h)
            });
            let want_lift = flyer.map_or(perch.map_or(0.0, |(_, h)| h), |f| f.lift);
            let mut climbing = false;
            if (v.lift - want_lift).abs() > 1e-3 {
                let rate = if flyer.is_some() {
                    FLY_CLIMB_SPEED
                } else {
                    self.anims.climb_speed(v.id).unwrap_or(DESCEND_SPEED)
                };
                let step = rate * dt;
                v.lift += (want_lift - v.lift).clamp(-step, step);
                climbing = flyer.is_none();
            }
            if let Some((p, _)) = perch {
                if v.leap.is_none() {
                    v.pos = p;
                }
            }
            v.perch_yaw = perch.and_then(|_| {
                let place = self.game.level.data.hiding_place(&a.hiding_place)?;
                let e = zoo_core::scene::perch_scenery(place, &self.game.level.data)?;
                let r = e.rect;
                let c = Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0);
                Some(facing_to_yaw(c - v.pos))
            });
            // turn towards the walking direction, else to the logic facing (e.g. the player,
            // ANIM-009, or the water it drinks from)
            let target = if speed > 0.1 {
                facing_to_yaw(to)
            } else if let (Some(y), "climb") = (v.perch_yaw, self.game.rest_clip(a)) {
                y // clings to the mast / trunk
            } else {
                facing_to_yaw(a.facing)
            };
            let rate = if speed > 0.1 { 8.0 } else { 4.0 };
            v.yaw += angle_diff(target, v.yaw) * (1.0 - (-rate * dt).exp());
            // swimmers sink in water (only eyes, ears and back show); clips per GAME-RESCUE §11
            let depth = self.game.water_depth(a);
            let want_sink = swim_sink_m(v.id) * depth;
            v.sink += (want_sink - v.sink) * (1.0 - (-3.0 * dt).exp());
            // feet on the surface under it (GAME-PLAYER 8); a perch height is absolute
            let want_ground = if perch.is_some() {
                0.0
            } else {
                self.game.level.ground_height(v.pos)
            };
            v.ground = zoo_core::ground::follow(v.ground, want_ground, dt);
            let in_water = depth > 0.5 && a.state != AnimalState::Following;
            v.under_water =
                in_water && v.id == "goldfish" && v.leap.is_none() && v.anchor.is_none();
            v.rest = static_clip(self.game.rest_clip(a));
            v.locomotion = if climbing {
                "climb"
            } else if in_water || a.state == AnimalState::InBowl {
                "swim"
            } else {
                "walk"
            };
            if let Some(f) = flyer {
                // perched / hanging at its place, flying while it follows; asleep at night
                // stays asleep
                if v.rest != "sleep" {
                    v.rest = f.rest;
                }
                v.locomotion = f.locomotion;
            }
            if v.skinned {
                if !self.renderer.has_clip(v.model, v.rest) {
                    v.rest = "idle";
                }
                if !self.renderer.has_clip(v.model, v.locomotion) {
                    v.locomotion = if self.renderer.has_clip(v.model, "walk") {
                        "walk"
                    } else {
                        "swim"
                    };
                }
            }
            let moving = if climbing { DESCEND_SPEED } else { speed };
            let want = (moving / (0.5 * walk_speed)).clamp(0.0, 1.0);
            v.walk_blend += (want - v.walk_blend) * (1.0 - (-10.0 * dt).exp());
            v.idle_time += dt;
            // walk clip playback = speed ÷ authored speed (animal_anims.toml), clamped
            let authored = if climbing {
                self.anims.climb_speed(v.id).unwrap_or(DESCEND_SPEED)
            } else {
                self.anims.walk_speed(v.model)
            };
            let actual = if climbing {
                self.anims.climb_speed(v.id).unwrap_or(DESCEND_SPEED)
            } else {
                speed
            };
            v.walk_time += dt * walk_clip_rate(actual * 1.4 / authored);
            let arrived = dist < 0.05;
            if v.wait_arrival && arrived {
                v.wait_arrival = false;
            }
            if let Some((name, t, dur)) = v.action {
                let t = t + dt;
                v.action = (t < dur).then_some((name, t, dur));
            }
            if v.action.is_none() && !v.wait_arrival {
                if let Some(next) = v.queue.pop_front() {
                    // missing one-shot clips fall back: refuse → idle shake is skipped
                    let dur = if v.skinned {
                        self.renderer.clip_duration(v.model, next).unwrap_or(0.0)
                    } else {
                        1.0
                    };
                    if dur > 0.0 {
                        v.action = Some((next, 0.0, dur));
                    }
                }
            }
        }
    }
}

/// The player's feet in world space: level position at the ground height (GAME-PLAYER 8).
fn player_feet(game: &Game) -> Vec3 {
    level_to_world(game.player.pos) + Vec3::Y * game.player.y
}

/// Placeholder animal (~1.3 m long) from flat boxes in the animal's colours; `happy` hops,
/// `refuse` shakes, `eat`/`drink` lower the head (PROD-POC "Placeholders").
/// Procedural water wheel (placeholder for `mill_hut_wheel`, LAYOUT-L3-017): root = the
/// axle into the hut wall; child part `wheel` (pivot on the axle) = hub, two rims, spokes and
/// the paddles, turned by the renderer with the clock (`NodeBehaviour` "wheel"). Model space:
/// glTF (x along the axle = level x, y up, z = level south), origin on the water surface under
/// the wheel centre. Paddle `k` rests at the angle `k · 45°` like
/// [`zoo_core::scene::WaterWheel::paddle_tip`].
fn water_wheel_model(w: &zoo_core::scene::WaterWheel) -> Model {
    use zoo_assets::NodePart;
    // palette cells (tools/blender/palette.toml, 16 × 16 cells): wood_dark 18, wood_light 17
    const UV: [[f32; 2]; 2] = [[2.5 / 16.0, 1.5 / 16.0], [1.5 / 16.0, 1.5 / 16.0]];
    let mut mesh = zoo_assets::MeshData::default();
    let pivot = Vec3::new(0.0, w.axle_y, 0.0);
    // oriented box: centre, unit axes, half extents, part, material
    let mut cube = |c: Vec3, ax: [Vec3; 3], h: Vec3, part: u8, mat: usize| {
        let he = [ax[0] * h.x, ax[1] * h.y, ax[2] * h.z];
        for (i, s) in [
            (0usize, 1.0f32),
            (0, -1.0),
            (1, 1.0),
            (1, -1.0),
            (2, 1.0),
            (2, -1.0),
        ] {
            let n = ax[i] * s;
            let (j, k) = ((i + 1) % 3, (i + 2) % 3);
            let base = mesh.positions.len() as u32;
            for (a, b) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                let p = c + he[i] * s + he[j] * a + he[k] * b;
                mesh.positions.push(p.to_array());
                mesh.normals.push(n.to_array());
                mesh.uvs.push(UV[mat]);
                mesh.node.push(part);
            }
            let quad = if s > 0.0 {
                [0, 1, 2, 0, 2, 3]
            } else {
                [0, 2, 1, 0, 3, 2]
            };
            mesh.indices.extend(quad.iter().map(|q| base + q));
        }
    };
    let radial = |a: f32| Vec3::new(0.0, a.cos(), a.sin());
    let tangent = |a: f32| Vec3::new(0.0, -a.sin(), a.cos());
    // root: the axle from the wheel into the hut wall
    let east = w.axle_end_x - w.center.x;
    cube(
        Vec3::new((east - 0.4) / 2.0, w.axle_y, 0.0),
        [Vec3::X, Vec3::Y, Vec3::Z],
        Vec3::new((east + 0.4) / 2.0, 0.07, 0.07),
        0,
        0,
    );
    // wheel: hub, rims, spokes (two sides of different section: no coplanar faces), paddles
    cube(
        pivot,
        [Vec3::X, Vec3::Y, Vec3::Z],
        Vec3::new(w.width / 2.0 + 0.08, 0.17, 0.17),
        1,
        0,
    );
    let rim = w.radius - WATER_WHEEL_PADDLE_IN;
    let n = zoo_core::scene::WATER_WHEEL_PADDLES;
    for (side, t) in [(-1.0f32, 0.035f32), (1.0, 0.04)] {
        let x = side * (w.width / 2.0 - 0.05);
        for k in 0..16 {
            let a = (k as f32 + 0.5) * std::f32::consts::TAU / 16.0;
            let seg = rim * (std::f32::consts::PI / 16.0).sin() + 0.03;
            cube(
                pivot + Vec3::X * x + radial(a) * rim,
                [Vec3::X, radial(a), tangent(a)],
                Vec3::new(t, 0.05, seg),
                1,
                1,
            );
        }
        for k in 0..n / 2 {
            let a = (k as f32 + 0.5) * std::f32::consts::TAU / n as f32;
            cube(
                pivot + Vec3::X * x,
                [Vec3::X, radial(a), tangent(a)],
                Vec3::new(t + 0.005, rim, 0.04),
                1,
                1,
            );
        }
    }
    for k in 0..n {
        let a = k as f32 * std::f32::consts::TAU / n as f32;
        let r = w.radius - zoo_core::scene::WATER_WHEEL_PADDLE_M / 2.0;
        cube(
            pivot + radial(a) * r,
            [Vec3::X, radial(a), tangent(a)],
            Vec3::new(
                w.width / 2.0,
                zoo_core::scene::WATER_WHEEL_PADDLE_M / 2.0,
                0.03,
            ),
            1,
            0,
        );
    }
    Model {
        mesh,
        base_color: [1.0; 4],
        image: None,
        materials: Vec::new(),
        skeleton: None,
        clips: Vec::new(),
        nodes: vec![
            NodePart {
                name: WATER_WHEEL.to_owned(),
                pivot: Vec3::ZERO,
            },
            NodePart {
                name: "wheel".to_owned(),
                pivot,
            },
        ],
        empties: Vec::new(),
        faces: Vec::new(),
    }
}

/// Paddles start this far inside the tip radius: the rims run along their inner edge.
const WATER_WHEEL_PADDLE_IN: f32 = 0.3;

/// Bamboo cut spots (GAME-FEED §15): full stalk, young shoot or stump as placeholder boxes
/// until the bamboo model has `stalk_full` / `stalk_young` / `stump` nodes (§17, missing art).
fn bamboo_stalks(out: &mut Vec<Instance>, game: &Game) {
    use zoo_core::carrying::StalkStage;
    // fresh yellow-green stalks stand out against the darker thicket
    const STALK: [f32; 3] = [0.66, 0.82, 0.30];
    const LEAF: [f32; 3] = [0.48, 0.72, 0.24];
    const CUT: [f32; 3] = [0.95, 0.90, 0.66];
    for (i, c) in game.level.data.cut_spots.iter().enumerate() {
        if !game.part_unlocked(c.part) {
            continue;
        }
        let Some(stage) = game.bamboo.stage(i) else {
            continue;
        };
        let base = level_to_world(c.pos()) + Vec3::Y * game.level.ground_height(c.pos());
        let yaw = i as f32 * 1.3;
        let mut push = |y: f32, size: Vec3, color: [f32; 3]| {
            out.push(Instance::flat(base + Vec3::Y * y, yaw, size, color, false));
        };
        match stage {
            StalkStage::Full => {
                // two tall stalks with nodes and a leaf tuft
                push(0.0, Vec3::new(0.12, 2.9, 0.12), STALK);
                for y in [0.7, 1.4, 2.1] {
                    push(y, Vec3::new(0.12, 0.05, 0.12), LEAF);
                }
                push(2.5, Vec3::new(0.55, 0.35, 0.25), LEAF);
            }
            StalkStage::Young => {
                push(0.0, Vec3::new(0.1, 1.2, 0.1), STALK);
                push(0.55, Vec3::new(0.11, 0.05, 0.11), LEAF);
                push(1.0, Vec3::new(0.3, 0.22, 0.16), LEAF);
            }
            StalkStage::Stump => {
                push(0.0, Vec3::new(0.13, 0.3, 0.13), STALK);
                push(0.3, Vec3::new(0.11, 0.03, 0.11), CUT);
            }
        }
    }
}

fn animal_placeholder(
    out: &mut Vec<Instance>,
    pos: Vec3,
    a: &AnimalView,
    action: Option<&str>,
    t: f32,
) {
    const WHITE: [f32; 3] = [0.96, 0.95, 0.92];
    const BLACK: [f32; 3] = [0.16, 0.15, 0.18];
    let (body, stripe) = match a.id {
        "zebra" => (WHITE, Some(BLACK)),
        "hippo" => ([0.62, 0.55, 0.66], None),
        "panda" => (WHITE, Some(BLACK)),
        _ => ([0.80, 0.62, 0.40], None),
    };
    let phase = t * std::f32::consts::TAU;
    let head_low = matches!(a.rest, "drink" | "eat");
    let (hop, shake, head_down) = match action {
        Some("happy") => ((phase * 2.0).sin().abs() * 0.18, 0.0, 0.0),
        Some("refuse") => (0.0, (phase * 3.0).sin() * 0.35, 0.0),
        Some("eat") => (0.0, 0.0, 0.45),
        _ => (
            (a.walk_time * 8.0).sin().abs() * 0.04 * a.walk_blend,
            0.0,
            if head_low { 0.35 } else { 0.0 },
        ),
    };
    let yaw = a.yaw + shake * 0.3;
    let rot = Quat::from_rotation_y(yaw);
    let base = pos + Vec3::Y * hop;
    let mut push = |local: Vec3, size: Vec3, color: [f32; 3]| {
        out.push(Instance::flat(base + rot * local, yaw, size, color, false));
    };
    let dark = stripe.unwrap_or(body);
    for (x, z) in [(-0.15, 0.45), (0.15, 0.45), (-0.15, -0.45), (0.15, -0.45)] {
        push(Vec3::new(x, 0.0, z), Vec3::new(0.11, 0.62, 0.11), dark);
    }
    push(Vec3::new(0.0, 0.6, 0.0), Vec3::new(0.46, 0.5, 1.3), body);
    if a.id == "zebra" {
        for z in [-0.45, -0.15, 0.15, 0.45] {
            push(Vec3::new(0.0, 0.58, z), Vec3::new(0.48, 0.54, 0.09), BLACK);
        }
    }
    let hy = 1.05 - head_down;
    push(
        Vec3::new(0.0, 0.9 - head_down * 0.5, 0.62),
        Vec3::new(0.2, 0.4, 0.22),
        body,
    );
    push(Vec3::new(0.0, hy, 0.82), Vec3::new(0.24, 0.26, 0.42), body);
    push(Vec3::new(0.0, hy, 1.02), Vec3::new(0.25, 0.2, 0.08), dark); // muzzle
}

/// Night animals without a model yet (GAME-NIGHT rule 6): coloured placeholder shapes.
fn is_night_placeholder(id: &str) -> bool {
    matches!(id, "hedgehog" | "bat" | "owl")
}

/// Placeholder night animal (no 3D model yet): a small coloured body in the animal's colour
/// with big round eyes — dark by default, shining `#E6F7A0` inside the lantern light
/// (NIGHT-006). `happy` hops, `refuse` shakes.
fn night_animal_placeholder(
    out: &mut Vec<Instance>,
    glows: &mut Vec<Instance>,
    pos: Vec3,
    a: &AnimalView,
    action: Option<&str>,
    t: f32,
    shine: bool,
) {
    let phase = t * std::f32::consts::TAU;
    let (hop, shake) = match action {
        Some("happy") => ((phase * 2.0).sin().abs() * 0.15, 0.0),
        Some("refuse") => (0.0, (phase * 3.0).sin() * 0.35),
        _ => ((a.walk_time * 10.0).sin().abs() * 0.03 * a.walk_blend, 0.0),
    };
    let yaw = a.yaw + shake * 0.3;
    let rot = Quat::from_rotation_y(yaw);
    let base = pos + Vec3::Y * hop;
    let mut push = |local: Vec3, size: Vec3, color: [f32; 3]| {
        out.push(Instance::flat(base + rot * local, yaw, size, color, false));
    };
    // (body colour, face colour, eye height, eye spacing, eye forward, eye size)
    // (eyes big and a little proud of the face so they read from the 55° zoo camera)
    let (eye_y, eye_dx, eye_z, eye_s) = match a.id {
        "hedgehog" => {
            let spikes = [0.36, 0.27, 0.20];
            push(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.46, 0.30, 0.56),
                [0.66, 0.50, 0.36],
            );
            push(
                Vec3::new(0.0, 0.2, -0.04),
                Vec3::new(0.52, 0.18, 0.56),
                spikes,
            );
            push(
                Vec3::new(0.0, 0.08, 0.34),
                Vec3::new(0.24, 0.18, 0.2),
                [0.86, 0.72, 0.56],
            );
            push(
                Vec3::new(0.0, 0.12, 0.46),
                Vec3::new(0.07, 0.06, 0.06),
                [0.17, 0.12, 0.10],
            );
            (0.24, 0.09, 0.44, 0.1)
        }
        "bat" => {
            let fur = [0.42, 0.34, 0.44];
            let wing = [0.30, 0.24, 0.34];
            push(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.26, 0.36, 0.22), fur);
            push(
                Vec3::new(0.0, 0.36, 0.0),
                Vec3::new(0.24, 0.2, 0.2),
                [0.56, 0.44, 0.50],
            );
            for sx in [-1.0f32, 1.0] {
                push(
                    Vec3::new(sx * 0.3, 0.08, -0.02),
                    Vec3::new(0.36, 0.28, 0.04),
                    wing,
                );
                push(
                    Vec3::new(sx * 0.08, 0.56, 0.0),
                    Vec3::new(0.07, 0.12, 0.04),
                    fur,
                );
            }
            (0.47, 0.065, 0.11, 0.085)
        }
        _ => {
            // owl
            let body = [0.66, 0.50, 0.34];
            push(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.40, 0.56, 0.34), body);
            push(
                Vec3::new(0.0, 0.08, 0.14),
                Vec3::new(0.26, 0.3, 0.08),
                [0.90, 0.80, 0.62],
            );
            push(Vec3::new(0.0, 0.56, 0.0), Vec3::new(0.40, 0.28, 0.32), body);
            push(
                Vec3::new(0.0, 0.58, 0.14),
                Vec3::new(0.34, 0.22, 0.06),
                [0.92, 0.84, 0.70],
            );
            for sx in [-1.0f32, 1.0] {
                push(
                    Vec3::new(sx * 0.13, 0.82, 0.0),
                    Vec3::new(0.08, 0.1, 0.08),
                    body,
                );
            }
            push(
                Vec3::new(0.0, 0.62, 0.18),
                Vec3::new(0.05, 0.07, 0.05),
                [0.95, 0.72, 0.30],
            );
            (0.72, 0.09, 0.19, 0.11)
        }
    };
    for sx in [-1.0f32, 1.0] {
        let p = base + rot * Vec3::new(sx * eye_dx, eye_y - eye_s / 2.0, eye_z);
        let size = Vec3::new(eye_s, eye_s, 0.06);
        if shine {
            glows.push(Instance::glow(p, yaw, size, nightfx::EYE_GLOW.to_array()));
        } else {
            out.push(Instance::flat(p, yaw, size, [0.12, 0.10, 0.12], false));
        }
    }
}

/// Scales an instance along its model's x axis (string-light spans, proposal Q-147).
fn stretch(r: &mut Renderer, h: InstanceHandle, k: f32) {
    if (k - 1.0).abs() < 1e-4 {
        return;
    }
    if let Some(mut i) = r.instance(h) {
        i.scale_fade[0] *= k;
        r.set_instance(h, i);
    }
}

/// Texture id of a text decal (`text:<fluent key>`).
fn text_texture_id(key: &str) -> String {
    format!("text:{key}")
}

#[wasm_bindgen]
impl App {
    // -------------------------------------------------------------- intro (GAME-RESCUE)

    /// Whether the intro at the entrance gate still has to be shown (RESC-029): a new game
    /// that has not seen it; a loaded game never repeats it.
    pub fn intro_pending(&self) -> bool {
        !self.game.intro_seen
    }

    /// The intro was shown or skipped: remember it (saved with the game) and point the
    /// child at the first step with the hint (RESC-029).
    pub fn intro_done(&mut self) {
        self.game.intro_seen = true;
        self.hint_press();
    }

    // -------------------------------------------------------------- hints (GAME-HINT)

    /// The 🧭 hint button / `H` / tapping the 🌙 progress (GAME-HINT rule 2/3, rule 8):
    /// shows the best next target, pressed again within 12 s the next of the top 3.
    /// Returns the target kind (`board`, `food`, …) or empty.
    pub fn hint_press(&mut self) -> String {
        self.hints
            .press(&self.game)
            .map(|h| h.kind.id().to_owned())
            .unwrap_or_default()
    }

    /// The shown hint for the overlay, or empty when none is shown: JSON `{"id", "kind",
    /// "on" (on screen), "x", "y" (CSS px: above the target, or the edge-arrow position),
    /// "angle" (edge arrow, degrees, 0 = right, 90 = down), "dots" (1…5 walking distance),
    /// "left" (s), "lx", "lz" (target, level), "sx", "sz" (stand point), "animal"}`.
    pub fn hint_json(&self) -> String {
        let Some(h) = self.hints.shown() else {
            return String::new();
        };
        let (w, hgt) = self.renderer.size();
        let ratio = self.renderer.pixel_ratio().max(1e-3);
        let (w, hgt) = (w as f32 / ratio, hgt as f32 / ratio);
        let vp = self.camera.view_proj(self.renderer.aspect());
        let ground = self.game.level.ground_height(h.pos);
        let p = level_to_world(h.pos) + Vec3::Y * (ground + h.height);
        let clip = vp * p.extend(1.0);
        let place = zoo_core::hints::screen_place(clip, w, hgt, HINT_EDGE_MARGIN_PX);
        format!(
            "{{\"id\":{},\"kind\":{},\"on\":{},\"x\":{:.1},\"y\":{:.1},\"angle\":{:.1},\"dots\":{},\"left\":{:.2},\"lx\":{:.2},\"lz\":{:.2},\"sx\":{:.2},\"sz\":{:.2},\"animal\":{},\"step\":{}}}",
            js(&h.id),
            js(h.kind.id()),
            place.on_screen,
            place.x,
            place.y,
            place.angle_deg,
            self.hints.dots(),
            self.hints.time_left(),
            h.pos.x,
            h.pos.y,
            h.stand.x,
            h.stand.y,
            js(h.animal.unwrap_or("")),
            js(h.kind.step_key())
        )
    }

    /// How often the 🧭 button should have pulsed so far (idle nudge, GAME-HINT rule 6).
    pub fn hint_pulses(&self) -> u32 {
        self.hints.pulses()
    }

    /// The 🌙 night progress (GAME-NIGHT rule 11): JSON `{"state": "hidden" | "missing" |
    /// "night_coming" | "night" | "sleep", "level", "animals": [{"id", "home"}]}`.
    pub fn night_progress_json(&self) -> String {
        let p = zoo_core::hints::night_progress(&self.game);
        let animals: Vec<String> = p
            .animals
            .iter()
            .map(|(id, home)| format!("{{\"id\":{},\"home\":{}}}", js(id), home))
            .collect();
        format!(
            "{{\"state\":{},\"level\":{},\"animals\":[{}]}}",
            js(p.state.id()),
            js(&p.level),
            animals.join(",")
        )
    }

    /// Debug/e2e: the food box of a food nearest to the player `[x, z, facing_x, facing_z]`
    /// (level; read from the level data), empty if there is none.
    pub fn debug_food_box(&self, food_id: &str) -> Vec<f32> {
        let Some(food) = Food::from_id(food_id) else {
            return Vec::new();
        };
        let p = self.game.player.pos;
        self.game
            .food_boxes
            .iter()
            .filter(|b| b.0 == food)
            .min_by(|a, b| a.1.distance(p).total_cmp(&b.1.distance(p)))
            .map(|b| vec![b.1.x, b.1.y, b.2.x, b.2.y])
            .unwrap_or_default()
    }

    /// Debug/e2e: the food an animal eats (what its board says).
    pub fn debug_animal_food(&self, id: &str) -> String {
        self.game
            .animal(id)
            .and_then(|a| a.info.foods.first())
            .map(|f| f.id().to_owned())
            .unwrap_or_default()
    }

    /// Debug/e2e: the gate cell of an animal's enclosure `[x, z]` (cell centre), empty if none.
    pub fn debug_gate_point(&self, id: &str) -> Vec<f32> {
        self.game
            .animal(id)
            .and_then(|a| self.game.level.data.elements[a.enclosure].gate)
            .map(|g| vec![g.x as f32 + 0.5, g.z as f32 + 0.5])
            .unwrap_or_default()
    }
}

/// The edge arrow of the hint stays this far inside the screen border (CSS px).
const HINT_EDGE_MARGIN_PX: f32 = 56.0;

fn target_key(t: &Target) -> String {
    match t {
        Target::InfoBoard { animal } => format!("info_board:{animal}"),
        Target::FoodBox { food } => format!("food_box:{}", food.id()),
        Target::Animal { animal } => format!("animal:{animal}"),
        Target::Gate { enclosure } => format!("gate:{enclosure}"),
        Target::Item { id } => format!("item:{id}"),
        Target::LyingFood { uid, food } => format!("lying_food:{}:{uid}", food.id()),
        Target::CutSpot { spot } => format!("bamboo:{spot}"),
        Target::Water { source } => format!("water:{source}"),
        Target::PutDown => "put_down".to_owned(),
        Target::Bed => "bed".to_owned(),
        Target::MoonDoor { id } => format!("moon_door:{id}"),
        Target::Plant { spot } => format!("plant:{spot}"),
        Target::GardenSign { bed } => format!("garden_sign:{bed}"),
        Target::Treat { animal } => format!("treat:{animal}"),
    }
}

/// Minimal JSON string literal.
fn js(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Signed smallest angle from `from` to `to` (radians).
fn angle_diff(to: f32, from: f32) -> f32 {
    (to - from + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}

/// Yaw about +Y that turns a model facing +Z (level south at yaw 0, Q-061) towards the
/// level direction `facing`.
fn facing_to_yaw(facing: Vec2) -> f32 {
    let f = level_to_world(facing);
    f.x.atan2(f.z)
}

fn warn(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::warn_1(&JsValue::from_str(msg));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{msg}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facing_north_turns_models_around() {
        // Level north = world -Z; a model facing +Z must turn 180°.
        assert!((facing_to_yaw(Vec2::Y).abs() - std::f32::consts::PI).abs() < 1e-5);
        // Level east = world +X: +90° (counter-clockwise from south to east).
        assert!((facing_to_yaw(Vec2::X) - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
    }

    fn level_tomls() -> Vec<String> {
        ["level-1", "level-2", "level-3", "night-1"]
            .iter()
            .map(|n| {
                std::fs::read_to_string(format!(
                    "{}/../../assets/levels/{n}.toml",
                    env!("CARGO_MANIFEST_DIR")
                ))
                .unwrap()
            })
            .collect()
    }

    #[test]
    fn required_assets_lists_props_player_and_every_animal() {
        let list = required_assets(level_tomls()).unwrap();
        assert!(list.contains(&"models/props/grass_tile.glb".to_owned()));
        assert!(list.contains(&"models/props/fence_wood.glb".to_owned()));
        assert!(list.contains(&"models/props/food_box.glb".to_owned()));
        assert!(list.contains(&"models/characters/player_girl.glb".to_owned()));
        for a in zoo_core::ANIMALS.iter().map(|a| a.id) {
            assert!(list.contains(&format!("models/animals/{a}.glb")), "{a}");
        }
        // AENV-011 for the new enclosures, placeholder props of levels 2-3
        assert!(list.contains(&"textures/signs/silhouette_goldfish.png".to_owned()));
        assert!(list.contains(&"models/props/tree_eucalyptus.glb".to_owned()));
        assert!(list.contains(&ANIMAL_ANIMS.to_owned()));
        // the hippo pool's model (placeholder rim when missing)
        assert!(list.contains(&"models/props/pool_tiled.glb".to_owned()));
        // AENV-011: the zebra sign silhouette is fetched (hippo/panda once they exist)
        assert!(list.contains(&"textures/signs/silhouette_zebra.png".to_owned()));
        assert!(std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/textures/signs/silhouette_zebra.png"
        ))
        .exists());
    }

    #[test]
    fn json_strings_are_escaped() {
        assert_eq!(js("a\"b\\c\nd"), "\"a\\\"b\\\\c\\nd\"");
        assert_eq!(js("Gras"), "\"Gras\"");
    }

    #[test]
    fn zebra_at_the_river_drinks_towards_the_water() {
        let toml = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/levels/level-1.toml"
        ))
        .unwrap();
        let data = LevelData::from_toml_str(&toml).unwrap();
        // a seed that puts the zebra at loc_river (pose `drink`, facing the river east)
        let g = (0..100)
            .map(|s| Game::new(data.clone(), s).unwrap())
            .find(|g| g.animal("zebra").unwrap().hiding_place == "loc_river")
            .unwrap();
        let z = g.animal("zebra").unwrap();
        assert!(
            z.facing.x > 0.7,
            "river is east of loc_river's spot: {}",
            z.facing
        );
        assert_eq!(g.rest_clip(z), "drink");
        assert_eq!(static_clip(g.rest_clip(z)), "drink");
        assert_eq!(static_clip("dance"), "idle");
    }
}
