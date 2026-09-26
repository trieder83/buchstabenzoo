//! TECH-ARCH tests.

mod common;

use std::collections::{BTreeMap, BTreeSet};

// ARCH-001: zoo-core's dependency tree (all targets and platforms, from Cargo.lock) contains
// neither web-sys nor wasm-bindgen.
#[test]
fn arch_001_zoo_core_has_no_web_deps() {
    let lock: toml::Table = toml::from_str(&common::read("Cargo.lock")).unwrap();
    let mut deps: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for p in lock["package"].as_array().unwrap() {
        let name = p["name"].as_str().unwrap().to_owned();
        let ds: Vec<String> = p
            .get("dependencies")
            .and_then(|d| d.as_array())
            .map(|a| {
                a.iter()
                    .map(|d| d.as_str().unwrap().split(' ').next().unwrap().to_owned())
                    .collect()
            })
            .unwrap_or_default();
        deps.entry(name).or_default().extend(ds);
    }
    let mut seen = BTreeSet::new();
    let mut stack = vec!["zoo-core".to_owned()];
    while let Some(n) = stack.pop() {
        if seen.insert(n.clone()) {
            stack.extend(deps.get(&n).cloned().unwrap_or_default());
        }
    }
    for forbidden in ["web-sys", "wasm-bindgen", "js-sys"] {
        assert!(!seen.contains(forbidden), "zoo-core depends on {forbidden}");
    }
    assert!(seen.contains("glam"));
}
