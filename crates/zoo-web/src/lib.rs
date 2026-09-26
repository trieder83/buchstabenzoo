//! wasm-bindgen entry point (TECH-ARCH): glue between the TypeScript host, `zoo-core` and
//! `zoo-render`. Minimal for M1 — the host passes the level data and drives `update`.

use glam::Vec2;
use wasm_bindgen::prelude::*;
use zoo_core::{Game, LevelData};

/// A running game, owned by the host page.
#[wasm_bindgen]
pub struct WebGame {
    game: Game,
}

#[wasm_bindgen]
impl WebGame {
    /// Creates a game from the level toml text (fetched by the host) and a seed.
    #[wasm_bindgen(constructor)]
    pub fn new(level_toml: &str, seed: u64) -> Result<WebGame, JsError> {
        let data =
            LevelData::from_toml_str(level_toml).map_err(|e| JsError::new(&e.to_string()))?;
        let game = Game::new(data, seed).map_err(|e| JsError::new(&e.to_string()))?;
        Ok(WebGame { game })
    }

    /// Advances the game by `dt` seconds with joystick input in world `(x, z)`.
    pub fn update(&mut self, dt: f32, input_x: f32, input_z: f32) {
        self.game.update(dt, Vec2::new(input_x, input_z));
    }

    pub fn player_x(&self) -> f32 {
        self.game.player.pos.x
    }

    pub fn player_z(&self) -> f32 {
        self.game.player.pos.y
    }

    /// Default camera distance, to prove `zoo-render` links.
    pub fn camera_distance(&self) -> f32 {
        zoo_render::CameraParams::default().distance_m
    }
}
