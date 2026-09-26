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
use zoo_core::coords::level_to_world;
use zoo_core::game::{Interaction, Target};
use zoo_core::level::{cell_center, ElementType};
use zoo_core::nav::Autopilot;
use zoo_core::player::walk_clip_rate;
use zoo_core::{AnimalState, Content, Food, Game, GameEvent, Language, LevelData, ReadingLevel};
use zoo_render::renderer::CAPSULE;
use zoo_render::scene::{model_placeholder, Decal, DecalImage};
use zoo_render::{CameraParams, CharacterDraw, FollowCamera, Instance, LevelScene, Renderer};

/// The player character model (GAME-PLAYER §1; `player_boy` later).
pub const PLAYER_MODEL: &str = "player_girl";
/// Animals drawn in the PoC (PROD-POC: zebra only; hippo/panda are scenery).
pub const SHOWN_ANIMALS: [&str; 1] = ["zebra"];
const MARKER: &str = "__marker";
/// Batch of flat boxes that move (animal placeholders).
const DYN_BOX: &str = "__dyn_box";
/// Second batch of the food box model for the carried box.
const CARRY_BOX: &str = "__carry_food_box";
/// Horizontal mouse drag distance (CSS px) for one 45° camera step.
const DRAG_STEP_PX: f32 = 70.0;

/// Asset path of an animal model.
pub fn animal_model_path(animal: &str) -> String {
    format!("models/animals/{animal}.glb")
}

/// Asset paths (relative to the served `assets/` folder) needed for a level: palette, every
/// model the level scene places, the player and the shown animals. Missing models become
/// placeholders.
#[wasm_bindgen]
pub fn required_assets(level_toml: &str) -> Result<Vec<String>, JsError> {
    let data = LevelData::from_toml_str(level_toml).map_err(|e| JsError::new(&e.to_string()))?;
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
    out.extend(SHOWN_ANIMALS.iter().map(|a| animal_model_path(a)));
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
    /// Where an escaped animal looks (towards the water of its hiding place).
    rest_yaw: f32,
    idle_time: f32,
    walk_time: f32,
    walk_blend: f32,
    /// Current one-shot clip: name, time, duration.
    action: Option<(&'static str, f32, f32)>,
    queue: VecDeque<&'static str>,
    /// Actions wait until the view reached the logic position (walking into the enclosure).
    wait_arrival: bool,
    /// Escaped and standing: rests with `drink` instead of `idle`.
    drinking: bool,
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
    autopilot: Option<Autopilot>,
    outbox: Vec<String>,
    time: f64,
    /// Decals of the level scene (debug getters, AENV-011/012).
    decals: Vec<Decal>,
    /// Text textures the host renders: (texture id, Fluent key, width, height).
    text_textures: Vec<(String, &'static str, u32, u32)>,
    /// Text textures must be (re-)rendered (start, language change).
    text_dirty: bool,
}

#[wasm_bindgen]
impl App {
    /// Builds the level scene and uploads it. `assets` maps paths relative to `assets/`
    /// (e.g. `levels/level-1.toml`, `models/props/hedge.glb`) to their bytes.
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: HtmlCanvasElement,
        level_path: &str,
        assets: &js_sys::Map,
    ) -> Result<App, JsError> {
        console_error_panic_hook::set_once();
        let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        assets.for_each(&mut |v, k| {
            if let (Some(k), Ok(arr)) = (k.as_string(), v.dyn_into::<js_sys::Uint8Array>()) {
                files.insert(k, arr.to_vec());
            }
        });
        let level_bytes = files
            .get(level_path)
            .ok_or_else(|| JsError::new(&format!("missing {level_path}")))?;
        let level_toml = std::str::from_utf8(level_bytes)?;
        let data =
            LevelData::from_toml_str(level_toml).map_err(|e| JsError::new(&e.to_string()))?;
        let scene = LevelScene::build(&data);
        let game = Game::new(data, 1).map_err(|e| JsError::new(&e.to_string()))?;

        let mut renderer = Renderer::new(canvas).map_err(|e| JsError::new(&format!("{e:?}")))?;
        let box_mesh = zoo_render::renderer::box_mesh();
        for name in [MARKER, DYN_BOX] {
            renderer
                .add_mesh(name, &box_mesh, 1.0)
                .map_err(|e| JsError::new(&format!("{e:?}")))?;
        }
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
        let mut animals = Vec::new();
        for id in SHOWN_ANIMALS {
            let Some(a) = game.animal(id) else { continue };
            let skinned = add_skinned(id, &animal_model_path(id));
            let rest_yaw = facing_to_yaw(water_direction(&game, a.pos));
            animals.push(AnimalView {
                id,
                skinned,
                pos: a.pos,
                yaw: rest_yaw,
                rest_yaw,
                idle_time: 0.0,
                walk_time: 0.0,
                walk_blend: 0.0,
                action: None,
                queue: VecDeque::new(),
                wait_arrival: false,
                drinking: true,
            });
        }

        let mut placeholders: BTreeMap<String, usize> = BTreeMap::new();
        for p in &scene.placements {
            if !renderer.add_instance(p.model, p.pos, p.yaw) {
                let (offset, size, color) = model_placeholder(p.model);
                let pos = p.pos + glam::Quat::from_rotation_y(p.yaw) * offset;
                renderer.add_box(pos, size, p.yaw, color, pos.y + size.y > 1.5);
                *placeholders.entry(p.model.to_owned()).or_default() += 1;
            }
        }
        for b in &scene.boxes {
            renderer.add_box(b.pos, b.size, b.yaw, b.color, b.fadeable);
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

        Ok(App {
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
            autopilot: None,
            outbox: Vec::new(),
            time: 0.0,
            decals: scene.decals,
            text_textures,
            text_dirty: true,
        })
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
        for a in &mut s.animals {
            a.yaw = self.animals.iter().find(|v| v.id == a.id).map(|v| v.yaw);
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
        for v in &mut self.animals {
            let Some(a) = self.game.animal(v.id) else {
                continue;
            };
            v.pos = a.pos;
            v.queue.clear();
            v.action = None;
            v.wait_arrival = false;
            v.drinking = a.state == AnimalState::Escaped;
            if let Some(yaw) = state
                .animals
                .iter()
                .find(|s| s.id == v.id)
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
            let n = usize::from(self.game.carry.food().is_some());
            let p = player + fwd * 0.32 + Vec3::Y * 0.45;
            self.carry_box[0] = Instance {
                pos_yaw: [p.x, p.y, p.z, yaw],
                scale_fade: [0.55, 0.55, 0.55, 0.0],
                color: [1.0, 1.0, 1.0, 0.0],
            };
            self.renderer
                .set_dynamic_instances(CARRY_BOX, &self.carry_box[..n]);
        }

        // Animal placeholders (striped boxes) into the dynamic box batch.
        self.dyn_boxes.clear();
        let mut chars: [Option<(&str, CharacterDraw)>; 2] = [None, None];
        for a in &self.animals {
            let pos = level_to_world(a.pos);
            let (action, t) = a.action.map_or((None, 0.0), |(n, t, _)| (Some(n), t));
            if a.skinned {
                chars[1] = Some((
                    a.id,
                    CharacterDraw {
                        pos,
                        yaw: a.yaw,
                        idle_time: a.idle_time,
                        walk_time: a.walk_time,
                        walk_blend: a.walk_blend,
                        idle_clip: if a.drinking { "drink" } else { "idle" },
                        action: action.map(|n| (n, t)),
                        action_blend: if action.is_some() { 1.0 } else { 0.0 },
                    },
                ));
            } else {
                zebra_placeholder(&mut self.dyn_boxes, pos, a, action, t);
            }
        }
        self.renderer
            .set_dynamic_instances(DYN_BOX, &self.dyn_boxes);

        if self.player_skinned {
            chars[0] = Some((
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
        let draws: [(&str, CharacterDraw); 2] = [
            chars[0].unwrap_or((
                "",
                CharacterDraw::locomotion(Vec3::ZERO, 0.0, 0.0, 0.0, 0.0),
            )),
            chars[1].unwrap_or((
                "",
                CharacterDraw::locomotion(Vec3::ZERO, 0.0, 0.0, 0.0, 0.0),
            )),
        ];
        // Unknown names ("") are skipped by the renderer.
        self.renderer.render(&self.camera, player, &draws);
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
                "{{\"kind\":\"info_board\",\"key\":{},\"animal\":{},\"title\":{},\"more\":{},\"facts\":{},\"text\":{},\"picture\":{},\"food\":{},\"food_text\":{},\"level\":{}}}",
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
        if let Some(v) = self.animals.iter_mut().find(|v| v.id == animal) {
            v.queue.clear();
            v.action = None;
            v.queue.extend(clips.iter().copied());
            v.wait_arrival = wait_arrival;
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
                    self.queue(&animal, &["eat", "happy", "happy"], true);
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

    fn update_animals(&mut self, dt: f32) {
        let walk_speed = self.game.move_params.walk_speed;
        for v in &mut self.animals {
            let Some(a) = self.game.animal(v.id) else {
                continue;
            };
            let to = a.pos - v.pos;
            let dist = to.length();
            let moved = if dist > 12.0 {
                v.pos = a.pos;
                0.0
            } else {
                let step = (2.2 * dt).min(dist);
                if step > 1e-5 {
                    v.pos += to / dist * step;
                }
                step
            };
            let speed = if dt > 0.0 { moved / dt } else { 0.0 };
            if speed > 0.1 {
                let target = facing_to_yaw(to);
                v.yaw += angle_diff(target, v.yaw) * (1.0 - (-8.0 * dt).exp());
            } else if a.state == AnimalState::Escaped {
                v.yaw += angle_diff(v.rest_yaw, v.yaw) * (1.0 - (-4.0 * dt).exp());
                if angle_diff(v.rest_yaw, v.yaw).abs() < 0.01 {
                    v.yaw = v.rest_yaw;
                }
            }
            v.drinking = a.state == AnimalState::Escaped && speed < 0.1;
            let want = (speed / (0.5 * walk_speed)).clamp(0.0, 1.0);
            v.walk_blend += (want - v.walk_blend) * (1.0 - (-10.0 * dt).exp());
            v.idle_time += dt;
            v.walk_time += dt * walk_clip_rate(speed);
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

/// Striped placeholder zebra (~1.3 m long) from flat boxes; `happy` hops, `refuse` shakes,
/// `eat` lowers the head (PROD-POC "Placeholders").
fn zebra_placeholder(
    out: &mut Vec<Instance>,
    pos: Vec3,
    a: &AnimalView,
    action: Option<&str>,
    t: f32,
) {
    const WHITE: [f32; 3] = [0.96, 0.95, 0.92];
    const BLACK: [f32; 3] = [0.16, 0.15, 0.18];
    let phase = t * std::f32::consts::TAU;
    let (hop, shake, head_down) = match action {
        Some("happy") => ((phase * 2.0).sin().abs() * 0.18, 0.0, 0.0),
        Some("refuse") => (0.0, (phase * 3.0).sin() * 0.35, 0.0),
        Some("eat") => (0.0, 0.0, 0.45),
        _ => (
            (a.walk_time * 8.0).sin().abs() * 0.04 * a.walk_blend,
            0.0,
            if a.drinking { 0.35 } else { 0.0 },
        ),
    };
    let yaw = a.yaw + shake * 0.3;
    let rot = Quat::from_rotation_y(yaw);
    let base = pos + Vec3::Y * hop;
    let mut push = |local: Vec3, size: Vec3, color: [f32; 3]| {
        out.push(Instance::flat(base + rot * local, yaw, size, color, false));
    };
    // legs
    for (x, z) in [(-0.15, 0.45), (0.15, 0.45), (-0.15, -0.45), (0.15, -0.45)] {
        push(Vec3::new(x, 0.0, z), Vec3::new(0.11, 0.62, 0.11), WHITE);
    }
    // body with stripes
    push(Vec3::new(0.0, 0.6, 0.0), Vec3::new(0.46, 0.5, 1.3), WHITE);
    for z in [-0.45, -0.15, 0.15, 0.45] {
        push(Vec3::new(0.0, 0.58, z), Vec3::new(0.48, 0.54, 0.09), BLACK);
    }
    // neck and head (lowered while drinking/eating)
    let hy = 1.05 - head_down;
    push(
        Vec3::new(0.0, 0.9 - head_down * 0.5, 0.62),
        Vec3::new(0.2, 0.4, 0.22),
        WHITE,
    );
    push(Vec3::new(0.0, hy, 0.82), Vec3::new(0.24, 0.26, 0.42), WHITE);
    push(
        Vec3::new(0.0, hy + 0.2, 0.62),
        Vec3::new(0.08, 0.18, 0.3),
        BLACK,
    ); // mane
    push(Vec3::new(0.0, hy, 1.02), Vec3::new(0.25, 0.2, 0.08), BLACK); // muzzle
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
    }
}

/// Level direction from `pos` to the nearest river/pond cell (where an escaped animal
/// drinks); south if there is none.
fn water_direction(game: &Game, pos: Vec2) -> Vec2 {
    game.level
        .data
        .elements
        .iter()
        .filter(|e| {
            e.ty == ElementType::Landmark && matches!(e.kind.as_deref(), Some("river" | "pond"))
        })
        .flat_map(|e| e.rect.cells())
        .map(|c| cell_center(c) - pos)
        .min_by(|a, b| a.length().total_cmp(&b.length()))
        .map_or(Vec2::NEG_Y, |d| d.normalize_or_zero())
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

    #[test]
    fn required_assets_lists_props_player_and_zebra() {
        let toml = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/levels/level-1.toml"
        ))
        .unwrap();
        let list = required_assets(&toml).unwrap();
        assert!(list.contains(&"models/props/grass_tile.glb".to_owned()));
        assert!(list.contains(&"models/props/fence_wood.glb".to_owned()));
        assert!(list.contains(&"models/props/food_box.glb".to_owned()));
        assert!(list.contains(&"models/characters/player_girl.glb".to_owned()));
        assert!(list.contains(&"models/animals/zebra.glb".to_owned()));
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
    fn zebra_drinks_towards_the_river() {
        let toml = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/levels/level-1.toml"
        ))
        .unwrap();
        let g = Game::new(LevelData::from_toml_str(&toml).unwrap(), 1).unwrap();
        let z = g.animal("zebra").unwrap();
        let d = water_direction(&g, z.pos);
        assert!(d.x > 0.7, "river is east of loc_river's spot: {d}");
    }
}
