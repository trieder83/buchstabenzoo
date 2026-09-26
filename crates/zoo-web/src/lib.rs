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
use zoo_core::animals::{swim_sink_m, AnimTable};
use zoo_core::coords::level_to_world;
use zoo_core::game::{Interaction, Target};
use zoo_core::level::ElementType;
use zoo_core::nav::Autopilot;
use zoo_core::player::walk_clip_rate;
use zoo_core::{AnimalState, Content, Food, Game, GameEvent, Language, LevelData, ReadingLevel};
use zoo_render::renderer::{cylinder_mesh, CAPSULE};
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
const BOWL_HEIGHT_M: f32 = 0.36;
/// Glass colour; alpha 0.5 = screen-door transparency in the cel shader.
const GLASS: [f32; 4] = [0.78, 0.92, 0.98, 0.5];
const BOWL_WATER_COLOR: [f32; 4] = [0.36, 0.66, 0.90, 0.5];
/// Descending speed from a perch when the animal has no `climb_speed` (m/s).
const DESCEND_SPEED: f32 = 1.8;
/// Duration of the goldfish's leap into / out of the bowl (s).
const LEAP_S: f32 = 0.9;

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
    let mut models: Vec<&str> = scene.placements.iter().map(|p| p.model).collect();
    models.sort_unstable();
    models.dedup();
    let mut out = vec!["textures/palette.png".to_owned()];
    out.extend(models.iter().map(|m| format!("models/props/{m}.glb")));
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
    out.push(ANIMAL_ANIMS.to_owned());
    Ok(out)
}

#[derive(Debug, Default, Clone, Copy)]
struct Keys {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

/// Presentation state of one animal (GAME-RESCUE §11): smoothed position, facing, clips.
struct AnimalView {
    id: &'static str,
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
            leap: None,
            visible: true,
            under_water: false,
            anchor: None,
            perch_yaw: None,
        }
    }
}

/// Maps a clip name from the game data to a static name the renderer knows (unknown → idle).
fn static_clip(name: &str) -> &'static str {
    const CLIPS: [&str; 10] = [
        "idle", "walk", "drink", "eat", "sleep", "swim", "happy", "refuse", "climb", "roll",
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
    carry_box: [Instance; 1],
    has_carry_model: bool,
    animals: Vec<AnimalView>,
    /// Animal clip data (authored walk speeds).
    anims: AnimTable,
    /// Characters drawn this frame (reused, no per-frame allocation once grown).
    draws: Vec<(&'static str, CharacterDraw)>,
    autopilot: Option<Autopilot>,
    outbox: Vec<String>,
    time: f64,
    /// Decals of the level scene (debug getters, AENV-011/012).
    decals: Vec<Decal>,
    /// Text textures the host renders: (texture id, Fluent key, width, height).
    text_textures: Vec<(String, &'static str, u32, u32)>,
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
            .add_mesh(BOWL_WATER, &cylinder_mesh(12, true, false), 1.0)
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        if let Some(png) = files.get("textures/palette.png") {
            renderer
                .set_palette_png(png)
                .map_err(|e| JsError::new(&format!("{e:?}")))?;
        }

        // Static props: one instanced batch per model. Ground tiles get no normal edges.
        let mut has_carry_model = false;
        for (path, bytes) in &files {
            let Some(name) = path
                .strip_prefix("models/props/")
                .and_then(|s| s.strip_suffix(".glb"))
            else {
                continue;
            };
            match Model::from_glb(bytes) {
                Ok(m) => {
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
            // the second animal of a pair shares the model (GAME-FAMILY)
            let skinned = match animals.iter().find(|v: &&AnimalView| v.id == id) {
                Some(v) => v.skinned,
                None => add_skinned(id, &animal_model_path(id)),
            };
            animals.push(AnimalView::new(id, skinned));
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
        for (i, p) in scene.placements.iter().enumerate() {
            let region = placement_region(i, p.part);
            if !renderer.add_instance_in(p.model, region, p.pos, p.yaw, p.scale) {
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
        // Decals: sign silhouettes (image files) and sign texts (rendered by the host).
        let mut text_textures: Vec<(String, &'static str, u32, u32)> = Vec::new();
        for d in &scene.decals {
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
                        text_textures.push((id.clone(), *key, *width_px, *height_px));
                    }
                    id
                }
            };
            renderer.add_decal(&texture, d.corners(), d.normal());
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
            carry_box: [Instance::model(Vec3::ZERO, 0.0, false)],
            has_carry_model,
            animals,
            anims,
            draws: Vec::with_capacity(8),
            autopilot: None,
            outbox: Vec::new(),
            time: 0.0,
            decals: scene.decals,
            text_textures,
            text_dirty: true,
            barrier_regions,
            roof_regions,
            bowl_glass: [Instance::model(Vec3::ZERO, 0.0, false)],
            bowl_water: [Instance::model(Vec3::ZERO, 0.0, false)],
            inside: None,
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
        self.camera.snap(level_to_world(self.game.player.pos));
        self.player_yaw = facing_to_yaw(self.game.player.facing);
        self.autopilot = None;
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
            "KeyA" | "ArrowLeft" => self.keys.left = down,
            "KeyD" | "ArrowRight" => self.keys.right = down,
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
        self.camera.rotate_steps(steps);
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
        self.camera.snap(level_to_world(self.game.player.pos));
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

    /// Advances the game by `dt` seconds and renders one frame.
    pub fn frame(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 0.1);
        self.time += dt as f64;
        self.simulate(dt);

        let player = level_to_world(self.game.player.pos);
        self.camera.update(dt, player);

        // Character presentation: turn smoothly, blend idle↔walk by speed.
        let target = facing_to_yaw(self.game.player.facing);
        self.player_yaw += angle_diff(target, self.player_yaw) * (1.0 - (-14.0 * dt).exp());
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
            let n = usize::from(self.game.carry.food().is_some() && !bowl_carried);
            let p = player + fwd * 0.32 + Vec3::Y * 0.45;
            self.carry_box[0] = Instance {
                pos_yaw: [p.x, p.y, p.z, yaw],
                scale_fade: [0.55, 0.55, 0.55, 0.0],
                color: [1.0, 1.0, 1.0, 0.0],
            };
            self.renderer
                .set_dynamic_instances(CARRY_BOX, &self.carry_box[..n]);
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
        for (id, region) in &self.roof_regions {
            let hide = self.inside.as_deref() == Some(id.as_str());
            if hide != self.renderer.region_hidden(*region) {
                self.renderer.set_region_hidden(*region, hide);
            }
        }

        // The fish bowl: glass + water (+ the fish inside, drawn with the animals).
        let bowl_base = self.bowl_base(fwd, player);
        let (mut n_glass, mut n_water) = (0, 0);
        if let (Some(base), Some(b)) = (bowl_base, &self.game.bowl) {
            let r = BOWL_RADIUS_M * 2.0;
            self.bowl_glass[0] = Instance {
                pos_yaw: [base.x, base.y, base.z, 0.0],
                scale_fade: [r, BOWL_HEIGHT_M, r, 0.0],
                color: GLASS,
            };
            n_glass = 1;
            if b.water {
                let w = r - 0.04;
                self.bowl_water[0] = Instance {
                    pos_yaw: [base.x, base.y + 0.02, base.z, 0.0],
                    scale_fade: [w, BOWL_HEIGHT_M * 0.72, w, 0.0],
                    color: BOWL_WATER_COLOR,
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
        self.draws.clear();
        for a in &self.animals {
            if !a.visible {
                continue; // its level is still locked
            }
            let pos = a
                .anchor
                .unwrap_or_else(|| level_to_world(a.pos) - Vec3::Y * a.sink + Vec3::Y * a.lift);
            let pos = match a.leap {
                Some((from, t, _)) => {
                    let k = (t / LEAP_S).clamp(0.0, 1.0);
                    from.lerp(pos, k) + Vec3::Y * (4.0 * k * (1.0 - k)) * 0.9
                }
                None => pos,
            };
            let (action, t) = a.action.map_or((None, 0.0), |(n, t, _)| (Some(n), t));
            if a.skinned {
                self.draws.push((
                    a.id,
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
                    },
                ));
            } else {
                animal_placeholder(&mut self.dyn_boxes, pos, a, action, t);
            }
        }
        self.renderer
            .set_dynamic_instances(DYN_BOX, &self.dyn_boxes);

        if self.player_skinned {
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
        self.renderer.render(&self.camera, player, &self.draws);
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
        let p = level_to_world(self.game.player.pos);
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
        self.camera.snap(level_to_world(self.game.player.pos));
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
        self.camera.snap(level_to_world(self.game.player.pos));
        self.autopilot.is_none()
    }

    /// Whether the scripted walk of [`App::debug_goto`] has ended.
    pub fn debug_arrived(&self) -> bool {
        self.autopilot.is_none()
    }

    /// Seconds of game time since start.
    pub fn time(&self) -> f64 {
        self.time
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
    /// Input → game update → events → animal presentation.
    fn simulate(&mut self, dt: f32) {
        let mut stick = self.stick;
        let k = self.keys;
        let kv = Vec2::new(
            f32::from(u8::from(k.right)) - f32::from(u8::from(k.left)),
            f32::from(u8::from(k.up)) - f32::from(u8::from(k.down)),
        );
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
        self.update_animals(dt);
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
        let player = level_to_world(self.game.player.pos);
        let bowl = self.bowl_base(fwd, player);
        for v in self.animals.iter_mut().filter(|v| v.id == animal) {
            let from = if into_bowl {
                level_to_world(v.pos) - Vec3::Y * v.sink
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
                    self.outbox.push(format!(
                        "{{\"type\":\"level_complete\",\"level\":{}}}",
                        js(&level)
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
        let player = level_to_world(self.game.player.pos);
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
            let moved = if dist > 12.0 || v.anchor.is_some() {
                v.pos = a.pos;
                0.0
            } else if v.lift > 0.3 && a.state == AnimalState::Following {
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
            let want_lift = perch.map_or(0.0, |(_, h)| h);
            let mut climbing = false;
            if (v.lift - want_lift).abs() > 1e-3 {
                let rate = self.anims.climb_speed(v.id).unwrap_or(DESCEND_SPEED);
                let step = rate * dt;
                v.lift += (want_lift - v.lift).clamp(-step, step);
                climbing = true;
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
            if v.skinned {
                if !self.renderer.has_clip(v.id, v.rest) {
                    v.rest = "idle";
                }
                if !self.renderer.has_clip(v.id, v.locomotion) {
                    v.locomotion = if self.renderer.has_clip(v.id, "walk") {
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
                self.anims.walk_speed(v.id)
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
                        self.renderer.clip_duration(v.id, next).unwrap_or(0.0)
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

/// Placeholder animal (~1.3 m long) from flat boxes in the animal's colours; `happy` hops,
/// `refuse` shakes, `eat`/`drink` lower the head (PROD-POC "Placeholders").
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

/// Texture id of a text decal (`text:<fluent key>`).
fn text_texture_id(key: &str) -> String {
    format!("text:{key}")
}

fn target_key(t: &Target) -> String {
    match t {
        Target::InfoBoard { animal } => format!("info_board:{animal}"),
        Target::FoodBox { food } => format!("food_box:{}", food.id()),
        Target::Animal { animal } => format!("animal:{animal}"),
        Target::Gate { enclosure } => format!("gate:{enclosure}"),
        Target::Item { id } => format!("item:{id}"),
        Target::Water { source } => format!("water:{source}"),
        Target::PutDown => "put_down".to_owned(),
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
        (1..=3)
            .map(|n| {
                std::fs::read_to_string(format!(
                    "{}/../../assets/levels/level-{n}.toml",
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
