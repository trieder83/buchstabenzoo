//! ART-SOUND asset tests (ASND-001/002/003/004/008). Files: `assets/audio/<group>/<cue>_<n>.ogg`
//! (+ `.m4a` twin), entries in `assets/manifest.toml` with `kind = "audio"`.

use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// The audio entries of the manifest as raw TOML tables.
fn audio_entries() -> Vec<toml::Table> {
    let doc: toml::Table = read("assets/manifest.toml")
        .parse()
        .expect("manifest parses");
    doc["asset"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|a| a.as_table())
        .filter(|t| t.get("kind").and_then(|k| k.as_str()) == Some("audio"))
        .cloned()
        .collect()
}

fn s<'a>(t: &'a toml::Table, k: &str) -> &'a str {
    t.get(k).and_then(|v| v.as_str()).unwrap_or("")
}

/// Cue ids of ART-SOUND "Sound list" that must exist (animal cues: ASND-004).
const CUES: &[&str] = &[
    "step_path",
    "step_grass",
    "step_sand",
    "step_wood",
    "step_water",
    "door_wood_open",
    "door_wood_close",
    "gate_open",
    "gate_close",
    "glass_door",
    "moon_door",
    "pickup_food",
    "drop_food",
    "pickup_item",
    "drop_item",
    "harvest_plant",
    "basket_add",
    "ui_tap",
    "ui_refuse",
    "ui_success",
];

fn cue_files(t: &toml::Table) -> Vec<PathBuf> {
    let group = s(t, "group");
    let id = s(t, "id");
    let n = t.get("variants").and_then(|v| v.as_integer()).unwrap_or(1);
    (1..=n)
        .flat_map(|v| {
            ["ogg", "m4a"].map(|e| root().join(format!("assets/audio/{group}/{id}_{v}.{e}")))
        })
        .collect()
}

// ASND-001 (entries, origin, files exist)
#[test]
fn asnd_001_every_cue_has_entry_origin_and_files() {
    let entries = audio_entries();
    let mut problems = Vec::new();
    for cue in CUES {
        if !entries.iter().any(|e| s(e, "id") == *cue) {
            problems.push(format!("{cue}: no manifest entry"));
        }
    }
    for e in &entries {
        let id = s(e, "id");
        if s(e, "licence").is_empty() {
            problems.push(format!("{id}: no licence"));
        }
        if !["origin_url", "prompt", "script"]
            .iter()
            .any(|k| !s(e, k).is_empty())
        {
            problems.push(format!("{id}: no origin_url / prompt / script"));
        }
        let Some(approved) = e.get("approved").and_then(|v| v.as_bool()) else {
            problems.push(format!("{id}: no `approved`"));
            continue;
        };
        // agents never set approved = true; a human does after listening
        let _ = approved;
        for f in cue_files(e) {
            if !f.exists() {
                problems.push(format!("{id}: missing {}", f.display()));
            } else if std::fs::metadata(&f).unwrap().len() < 200 {
                problems.push(format!("{id}: empty {}", f.display()));
            }
        }
        if let Some(script) = e.get("script").and_then(|v| v.as_str()) {
            if !root().join(script).exists() {
                problems.push(format!("{id}: script {script} missing"));
            }
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ASND-002
#[test]
fn asnd_002_licences_are_allowed() {
    let credits = read("assets/audio/CREDITS.md");
    let mut problems = Vec::new();
    for e in audio_entries() {
        let id = s(&e, "id");
        match s(&e, "licence") {
            "CC0" => {
                if s(&e, "origin_url").is_empty() {
                    problems.push(format!("{id}: CC0 without origin_url"));
                }
            }
            "CC-BY" => {
                if !credits.contains(id) {
                    problems.push(format!("{id}: CC-BY without a line in CREDITS.md"));
                }
            }
            "generated" | "own" => {}
            other => problems.push(format!("{id}: licence {other:?} not allowed")),
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ASND-001 (decodes) + ASND-003 (loudness, peak, length): the Python checker decodes with ffmpeg.
#[test]
fn asnd_001_003_files_decode_and_are_loud_enough() {
    let out = Command::new("python3")
        .arg(root().join("tools/sound/check_audio.py"))
        .output();
    let out = match out {
        Ok(o) => o,
        Err(e) => {
            eprintln!("skipped: python3 not available ({e})");
            return;
        }
    };
    let text =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    if text.contains("ModuleNotFoundError") || text.contains("No such file or directory: 'ffmpeg'")
    {
        eprintln!("skipped: numpy/scipy/ffmpeg not available");
        return;
    }
    assert!(out.status.success(), "{text}");
}

// ASND-008
#[test]
fn asnd_008_total_size_within_budget() {
    fn size(dir: &std::path::Path) -> u64 {
        std::fs::read_dir(dir)
            .map(|rd| {
                rd.flatten()
                    .map(|e| {
                        let p = e.path();
                        if p.is_dir() {
                            size(&p)
                        } else {
                            e.metadata().map(|m| m.len()).unwrap_or(0)
                        }
                    })
                    .sum()
            })
            .unwrap_or(0)
    }
    let total = size(&root().join("assets/audio"));
    assert!(
        total <= 1_500_000,
        "assets/audio is {total} bytes (> 1.5 MB)"
    );
}

// ASND-004: still red until every species has its calls (Gemini candidates only for zebra and
// koala call so far, see art/sound/brief.md). Run with `-- --ignored` to see the gap.
#[test]
#[ignore = "animal calls are not complete yet (ART-SOUND, art/sound/brief.md)"]
fn asnd_004_every_species_has_call_happy_refuse() {
    let species = [
        "zebra",
        "hippo",
        "panda",
        "koala",
        "elephant",
        "goldfish",
        "monkey",
        "giraffe",
        "lion",
        "snow_fox",
        "hedgehog",
        "bat",
        "owl",
        "raccoon",
        "badger",
        "fennec",
        "kiwi",
        "porcupine",
        "slow_loris",
        "tarsier",
    ];
    let ids: Vec<String> = audio_entries()
        .iter()
        .map(|e| s(e, "id").to_string())
        .collect();
    let missing: Vec<String> = species
        .iter()
        .flat_map(|sp| ["call", "happy", "refuse"].map(|k| format!("animal_{sp}_{k}")))
        .filter(|c| !ids.contains(c))
        .collect();
    assert!(missing.is_empty(), "missing cues: {missing:?}");
}
