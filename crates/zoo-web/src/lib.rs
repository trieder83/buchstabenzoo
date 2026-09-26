//! wasm-bindgen entry point (TECH-ARCH): glue between the TypeScript host, `zoo-core`
//! (game logic) and `zoo-render` (WebGL2).
//!
//! The host fetches the files listed by [`required_assets`], hands them over as a
//! `Map<path, Uint8Array>` to [`App::new`] and then calls [`App::frame`] once per animation
//! frame, forwarding input with the `key` / `set_stick` / `drag` / `zoom` methods.

use std::collections::BTreeMap;

use glam::{Vec2, Vec3};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use zoo_assets::Model;
use zoo_core::coords::level_to_world;
use zoo_core::{Content, Game, Language, LevelData};
use zoo_render::renderer::CAPSULE;
use zoo_render::scene::model_placeholder;
use zoo_render::{CameraParams, CharacterDraw, FollowCamera, Instance, LevelScene, Renderer};

/// The player character model (GAME-PLAYER §1; `player_boy` later).
pub const PLAYER_MODEL: &str = "player_girl";
const MARKER: &str = "__marker";
/// Horizontal drag distance (CSS px) for one 45° camera step.
const DRAG_STEP_PX: f32 = 70.0;

/// Asset paths (relative to the served `assets/` folder) needed for a level: palette, every
/// model the level scene places, and the player. Missing models become placeholders.
#[wasm_bindgen]
pub fn required_assets(level_toml: &str) -> Result<Vec<String>, JsError> {
    let data = LevelData::from_toml_str(level_toml).map_err(|e| JsError::new(&e.to_string()))?;
    let scene = LevelScene::build(&data);
    let mut models: Vec<&str> = scene.placements.iter().map(|p| p.model).collect();
    models.sort_unstable();
    models.dedup();
    let mut out = vec!["textures/palette.png".to_owned()];
    out.extend(models.iter().map(|m| format!("models/props/{m}.glb")));
    out.push(format!("models/characters/{PLAYER_MODEL}.glb"));
    Ok(out)
}

#[derive(Debug, Default, Clone, Copy)]
struct Keys {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
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
    time: f64,
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
        renderer
            .add_mesh(MARKER, &zoo_render::renderer::box_mesh(), 1.0)
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        if let Some(png) = files.get("textures/palette.png") {
            renderer
                .set_palette_png(png)
                .map_err(|e| JsError::new(&format!("{e:?}")))?;
        }

        // Static props: one instanced batch per model. Ground tiles get no normal edges.
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
                }
                Err(e) => warn(&format!("{path}: {e} — using a placeholder")),
            }
        }
        let mut player_skinned = false;
        if let Some(bytes) = files.get(&format!("models/characters/{PLAYER_MODEL}.glb")) {
            match Model::from_glb(bytes)
                .map_err(|e| e.to_string())
                .and_then(|m| {
                    renderer
                        .add_skinned(PLAYER_MODEL, &m)
                        .map_err(|e| format!("{e:?}"))
                }) {
                Ok(()) => player_skinned = true,
                Err(e) => warn(&format!("{PLAYER_MODEL}: {e} — using a placeholder")),
            }
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
        if !player_skinned {
            *placeholders.entry(PLAYER_MODEL.to_owned()).or_default() += 1;
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

        // Fluent files: i18n/<lang>/*.ftl (reading panels arrive with M4).
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
            time: 0.0,
        })
    }

    /// Canvas CSS size and device pixel ratio (render resolution is capped at 2×).
    pub fn resize(&mut self, css_width: f64, css_height: f64, device_pixel_ratio: f64) {
        self.renderer
            .resize(css_width, css_height, device_pixel_ratio);
    }

    /// Keyboard input (`KeyboardEvent.code`). Returns true if the key is used by the game.
    pub fn key(&mut self, code: &str, down: bool) -> bool {
        match code {
            "KeyW" | "ArrowUp" => self.keys.up = down,
            "KeyS" | "ArrowDown" => self.keys.down = down,
            "KeyA" | "ArrowLeft" => self.keys.left = down,
            "KeyD" | "ArrowRight" => self.keys.right = down,
            "KeyQ" if down => self.camera.rotate_steps(-1),
            "KeyE" if down => self.camera.rotate_steps(1),
            "Equal" | "NumpadAdd" if down => self.camera.zoom_by(0.85),
            "Minus" | "NumpadSubtract" if down => self.camera.zoom_by(1.0 / 0.85),
            "KeyQ" | "KeyE" | "Equal" | "NumpadAdd" | "Minus" | "NumpadSubtract" => {}
            _ => return false,
        }
        true
    }

    /// Virtual joystick deflection in screen space: `x` right, `y` up, length ≤ 1.
    pub fn set_stick(&mut self, x: f32, y: f32) {
        self.stick = Vec2::new(x, y).clamp_length_max(1.0);
    }

    /// Horizontal mouse/touch drag in CSS pixels; every `DRAG_STEP_PX` rotates the camera
    /// one 45° step (eased, GAME-PLAYER §2).
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

    /// Rotates the camera by whole 45° steps.
    pub fn rotate(&mut self, steps: i32) {
        self.camera.rotate_steps(steps);
    }

    /// Zoom: factor < 1 moves the camera closer (wheel, pinch), clamped to 10–20 m.
    pub fn zoom(&mut self, factor: f32) {
        self.camera.zoom_by(factor);
    }

    /// Advances the game by `dt` seconds and renders one frame.
    pub fn frame(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 0.1);
        self.time += dt as f64;
        let mut stick = self.stick;
        let k = self.keys;
        let kv = Vec2::new(
            f32::from(u8::from(k.right)) - f32::from(u8::from(k.left)),
            f32::from(u8::from(k.up)) - f32::from(u8::from(k.down)),
        );
        if kv != Vec2::ZERO {
            stick = kv.normalize();
        }
        let dir = self.camera.stick_to_level(stick);
        self.game.update(dt, dir);

        let player = level_to_world(self.game.player.pos);
        self.camera.update(dt, player);

        // Character presentation: turn smoothly, blend idle↔walk by speed.
        let target = facing_to_yaw(self.game.player.facing);
        let diff = (target - self.player_yaw + std::f32::consts::PI)
            .rem_euclid(std::f32::consts::TAU)
            - std::f32::consts::PI;
        self.player_yaw += diff * (1.0 - (-14.0 * dt).exp());
        let speed = self.game.player.last_speed;
        let walk_speed = self.game.move_params.walk_speed;
        let want = (speed / (0.5 * walk_speed)).clamp(0.0, 1.0);
        self.walk_blend += (want - self.walk_blend) * (1.0 - (-10.0 * dt).exp());
        self.idle_time += dt;
        self.walk_time += dt * (speed / walk_speed).max(0.4);

        let yaw = self.player_yaw;
        if self.player_skinned {
            let draw = CharacterDraw {
                pos: player,
                yaw,
                idle_time: self.idle_time,
                walk_time: self.walk_time,
                walk_blend: self.walk_blend,
            };
            self.renderer
                .render(&self.camera, player, &[(PLAYER_MODEL, draw)]);
        } else {
            // Placeholder capsule (1.2 m) with a small marker showing the facing.
            let bob = (self.walk_time * 9.0).sin().abs() * 0.05 * self.walk_blend;
            let p = player + Vec3::Y * bob;
            self.dynamic[0] = Instance::flat(p, yaw, Vec3::ONE, [0.35, 0.62, 0.92], false);
            let fwd = glam::Quat::from_rotation_y(yaw) * Vec3::Z;
            self.marker[0] = Instance::flat(
                p + fwd * 0.28 + Vec3::Y * 0.8,
                yaw,
                Vec3::new(0.18, 0.14, 0.12),
                [0.96, 0.78, 0.60],
                false,
            );
            self.renderer.set_dynamic_instances(CAPSULE, &self.dynamic);
            self.renderer.set_dynamic_instances(MARKER, &self.marker);
            self.renderer.render(&self.camera, player, &[]);
        }
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

    pub fn camera_distance(&self) -> f32 {
        self.camera.distance()
    }

    pub fn camera_yaw_deg(&self) -> f32 {
        self.camera.yaw().to_degrees()
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

    /// Placeholders as `name ×count` lines.
    pub fn placeholders(&self) -> String {
        self.placeholders
            .iter()
            .map(|(k, n)| format!("{k} ×{n}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// A Fluent message in the given language, if the i18n files were loaded.
    pub fn text(&self, lang: &str, key: &str) -> Option<String> {
        let lang = Language::from_id(lang)?;
        self.content.as_ref()?.text(lang, key)
    }

    /// Debug/e2e: puts the player at a level position (no collision check) and snaps the
    /// camera there.
    pub fn debug_teleport(&mut self, x: f32, z: f32) {
        self.game.player.pos = Vec2::new(x, z);
        self.camera.snap(level_to_world(self.game.player.pos));
    }

    /// Seconds of game time since start.
    pub fn time(&self) -> f64 {
        self.time
    }
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
    fn required_assets_lists_props_and_player() {
        let toml = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/levels/level-1.toml"
        ))
        .unwrap();
        let list = required_assets(&toml).unwrap();
        assert!(list.contains(&"models/props/grass_tile.glb".to_owned()));
        assert!(list.contains(&"models/props/fence_wood.glb".to_owned()));
        assert!(list.contains(&"models/characters/player_girl.glb".to_owned()));
    }
}
