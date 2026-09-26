#![allow(dead_code)]
//! Shared test helpers: load the real project data files.

use std::path::PathBuf;

use zoo_core::{Content, Game, Language, LevelData};

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn read(rel: &str) -> String {
    let p = repo_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

pub fn level1() -> LevelData {
    LevelData::from_toml_str(&read("assets/levels/level-1.toml")).expect("level-1.toml parses")
}

pub fn content() -> Content {
    let de = read("assets/i18n/de/missions.ftl");
    let en = read("assets/i18n/en/missions.ftl");
    Content::from_sources(&[(Language::De, &de), (Language::En, &en)]).expect("ftl parses")
}

pub fn game(seed: u64) -> Game {
    Game::new(level1(), seed).expect("game starts")
}

pub fn level2() -> LevelData {
    LevelData::from_toml_str(&read("assets/levels/level-2.toml")).expect("level-2.toml parses")
}

pub fn level3() -> LevelData {
    LevelData::from_toml_str(&read("assets/levels/level-3.toml")).expect("level-3.toml parses")
}

/// Levels 1–3 joined into one zoo (GAME-LAYOUT "Joining levels", proposal Q-088).
pub fn zoo() -> LevelData {
    LevelData::join(vec![level1(), level2(), level3()]).expect("levels join")
}

pub fn zoo_game(seed: u64) -> Game {
    Game::new(zoo(), seed).expect("zoo game starts")
}
