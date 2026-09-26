//! Asset manifest and asset/concept checks (ART-PIPELINE). No web dependencies.
//!
//! glTF loading (APIPE-004/005) follows with milestone M2.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;

/// One `[[asset]]` of `assets/manifest.toml` (ART-PIPELINE §8).
#[derive(Debug, Clone, Deserialize)]
pub struct AssetEntry {
    pub id: String,
    pub kind: String,
    pub spec: String,
    pub concept_approved: bool,
    #[serde(default)]
    pub animations: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    #[serde(rename = "asset", default)]
    pub assets: Vec<AssetEntry>,
}

impl Manifest {
    pub fn from_toml_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    pub fn get(&self, id: &str) -> Option<&AssetEntry> {
        self.assets.iter().find(|a| a.id == id)
    }
}

/// All files below `dir` (recursive) whose name satisfies `pred`, sorted.
pub fn find_files(dir: &Path, pred: &dyn Fn(&Path) -> bool) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if pred(&p) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// The `id:` of a Markdown file's YAML frontmatter, if any.
pub fn frontmatter_id(md: &str) -> Option<String> {
    let rest = md.strip_prefix("---\n")?;
    let fm = &rest[..rest.find("\n---")?];
    fm.lines().find_map(|l| {
        let v = l.strip_prefix("id:")?;
        let v = v.split('#').next()?.trim();
        (!v.is_empty()).then(|| v.to_owned())
    })
}

/// Spec ids from the frontmatter of every `*.md` below `specs_dir` (APIPE-006).
pub fn spec_ids(specs_dir: &Path) -> BTreeSet<String> {
    find_files(specs_dir, &|p| p.extension().is_some_and(|e| e == "md"))
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .filter_map(|s| frontmatter_id(&s))
        .collect()
}

/// Item id → status from `art/catalog.js`, parsed loosely: every `status: "…"` belongs to the
/// closest preceding `id: "…"` (section ids have no status).
pub fn catalog_statuses(js: &str) -> BTreeMap<String, String> {
    let re = Regex::new(r#"\b(id|status)\s*:\s*"([^"]*)""#).expect("valid regex");
    let mut out = BTreeMap::new();
    let mut last_id: Option<String> = None;
    for c in re.captures_iter(js) {
        match &c[1] {
            "id" => last_id = Some(c[2].to_owned()),
            _ => {
                if let Some(id) = last_id.take() {
                    out.insert(id, c[2].to_owned());
                }
            }
        }
    }
    out
}

/// ```` ```text ```` blocks of a Markdown file.
pub fn text_blocks(md: &str) -> Vec<String> {
    let re = Regex::new(r"(?s)```text\n(.*?)\n```").expect("valid regex");
    re.captures_iter(md).map(|c| c[1].to_owned()).collect()
}

/// The three style blocks of `art/style/style.md`, in order.
#[derive(Debug, Clone)]
pub struct StyleBlocks {
    pub style: String,
    pub character_sheet: String,
    pub negative: String,
}

impl StyleBlocks {
    pub fn from_style_md(md: &str) -> Option<Self> {
        let b = text_blocks(md);
        if b.len() < 3 {
            return None;
        }
        Some(Self {
            style: b[0].clone(),
            character_sheet: b[1].clone(),
            negative: b[2].clone(),
        })
    }

    /// Checks one prompt block (APIPE-010). `None` = not a prompt (short fragment, e.g. a
    /// replacement camera paragraph), otherwise `Some(ok)`.
    pub fn check_block(&self, b: &str) -> Option<bool> {
        let head: String = b.chars().take(40).collect();
        if b.starts_with("text, letters") || head.to_lowercase().contains("negative") {
            return Some(b.trim_end().ends_with(&self.negative));
        }
        let starts = b.starts_with(&self.style) || b.starts_with(&self.character_sheet);
        if b.chars().count() < 700 && !starts {
            return None;
        }
        Some(starts)
    }
}
