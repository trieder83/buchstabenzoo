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
