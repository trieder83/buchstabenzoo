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

/// Every Fluent file of a language (`assets/i18n/<lang>/*.ftl`, sorted), as the host loads them.
pub fn ftl_text(lang: &str) -> String {
    let dir = repo_root().join("assets/i18n").join(lang);
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "ftl"))
        .collect();
    files.sort();
    files
        .iter()
        .map(|p| std::fs::read_to_string(p).unwrap() + "\n")
        .collect()
}

pub fn content() -> Content {
    let de = ftl_text("de");
    let en = ftl_text("en");
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

pub fn night1() -> LevelData {
    LevelData::from_toml_str(&read("assets/levels/night-1.toml")).expect("night-1.toml parses")
}

/// The day levels 1–3 and the night zoo `night_1` joined (GAME-NIGHT rule 4, NIGHT-004).
pub fn zoo_with_night() -> LevelData {
    LevelData::join(vec![level1(), level2(), level3(), night1()]).expect("levels join")
}

pub fn night2() -> LevelData {
    LevelData::from_toml_str(&read("assets/levels/night-2.toml")).expect("night-2.toml parses")
}

/// The day levels 1–3, `night_1` and the terrarium garden `night_2` joined (the whole game,
/// GAME-LEVEL-NIGHT-2).
pub fn zoo_with_night2() -> LevelData {
    LevelData::join(vec![level1(), level2(), level3(), night1(), night2()]).expect("levels join")
}

pub fn night2_game(seed: u64) -> Game {
    Game::new(zoo_with_night2(), seed).expect("zoo with night_2 starts")
}

pub fn night_game(seed: u64) -> Game {
    Game::new(zoo_with_night(), seed).expect("zoo with night starts")
}
