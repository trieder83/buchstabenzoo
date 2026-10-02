//! ART-PIPELINE asset tests (APIPE-006, APIPE-007, APIPE-010).

use std::path::PathBuf;

use zoo_assets::{catalog_statuses, find_files, spec_ids, text_blocks, Manifest, StyleBlocks};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn manifest() -> Manifest {
    Manifest::from_toml_str(&read("assets/manifest.toml")).expect("manifest parses")
}

#[test]
fn manifest_parses_with_unique_ids() {
    let m = manifest();
    assert!(!m.assets.is_empty());
    let mut ids = std::collections::BTreeSet::new();
    for a in &m.assets {
        assert!(ids.insert(&a.id), "duplicate {}", a.id);
        assert!(
            ["characters", "animals", "props", "environment", "audio"].contains(&a.kind.as_str()),
            "{}: kind {}",
            a.id,
            a.kind
        );
    }
}

// APIPE-006
#[test]
fn apipe_006_manifest_spec_ids_exist() {
    let ids = spec_ids(&root().join("specs"));
    assert!(ids.contains("ART-PIPELINE"));
    let missing: Vec<_> = manifest()
        .assets
        .iter()
        .filter(|a| !ids.contains(&a.spec))
        .map(|a| format!("{} -> {}", a.id, a.spec))
        .collect();
    assert!(missing.is_empty(), "unknown spec ids: {missing:?}");
}

// APIPE-007
#[test]
fn apipe_007_catalog_approved_iff_manifest_approved() {
    let cat = catalog_statuses(&read("art/catalog.js"));
    assert_eq!(cat.get("zebra").map(String::as_str), Some("approved"));
    let mut diffs = Vec::new();
    for a in &manifest().assets {
        let status = cat.get(&a.id).map(String::as_str);
        if a.concept_approved != (status == Some("approved")) {
            diffs.push(format!(
                "{}: manifest {} vs catalog {status:?}",
                a.id, a.concept_approved
            ));
        }
    }
    // approved catalog items must have an approved manifest entry
    let m = manifest();
    for (id, status) in &cat {
        if status == "approved" && !m.get(id).is_some_and(|a| a.concept_approved) {
            diffs.push(format!("{id}: approved in catalog, not in manifest"));
        }
    }
    assert!(diffs.is_empty(), "{diffs:#?}");
}

// APIPE-010
#[test]
fn apipe_010_prompts_copy_style_blocks_verbatim() {
    let style = StyleBlocks::from_style_md(&read("art/style/style.md")).expect("3 style blocks");
    // the ad banners (art/ads/) are separate marketing pictures with their own look, not game art
    let briefs = find_files(&root().join("art"), &|p| {
        p.file_name().is_some_and(|n| n == "brief.md")
            && !p.components().any(|c| c.as_os_str() == "ads")
    });
    assert!(!briefs.is_empty());
    let mut checked = 0;
    let mut failures = Vec::new();
    for f in briefs {
        let md = std::fs::read_to_string(&f).unwrap();
        for b in text_blocks(&md) {
            match style.check_block(&b) {
                None => {}
                Some(true) => checked += 1,
                Some(false) => failures.push(format!(
                    "{}: {}",
                    f.strip_prefix(root()).unwrap_or(&f).display(),
                    b.lines()
                        .next()
                        .unwrap_or("")
                        .chars()
                        .take(70)
                        .collect::<String>()
                )),
            }
        }
    }
    assert!(checked > 0);
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn apipe_010_checker_rejects_modified_style() {
    let style = StyleBlocks {
        style: "STYLE A".repeat(120),
        character_sheet: "SHEET B".repeat(120),
        negative: "no text".into(),
    };
    assert_eq!(
        style.check_block(&format!("{} scene", style.style)),
        Some(true)
    );
    assert_eq!(
        style.check_block(&format!("X{} scene", style.style)),
        Some(false)
    );
    assert_eq!(
        style.check_block("Negative prompt: blurry, no text"),
        Some(true)
    );
    assert_eq!(style.check_block("Negative prompt: blurry"), Some(false));
    assert_eq!(style.check_block("short camera fragment"), None);
}
