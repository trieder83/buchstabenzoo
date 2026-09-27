//! Native performance probe (PERF-BUDGETS). Prints one JSON object to stdout:
//!
//! - `game_update`: `zoo_core::game::Game::update` over the joined zoo (level-1…3 + night-1,
//!   the host's load order), 1200 frames at 60 Hz with a scripted stick input — µs per frame
//!   (p50/p95/max) and heap allocations per frame (PERF-011).
//! - `ambient_update`: `Ambient::update` + `poses` + `butterfly_poses` + `ripples` per frame.
//! - `anim`: per skinned model the cost of one `pose_character` equivalent (sample idle +
//!   walk, blend, joint matrices, copy into the joint texture buffer) in µs, and joints.
//! - `models`: per `.glb` triangles, draw parts (submeshes), texture size, file size.
//!
//! Native x86-64 numbers are a lower bound for the WASM build (typically 1.3–2× slower);
//! use them for relative comparisons between runs and for allocation counts, which are
//! exact and platform independent.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::collections::BTreeMap;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use glam::{Mat4, Vec2};
use serde_json::{json, Value};
use zoo_assets::{Model, Skeleton};
use zoo_core::ambient::Ambient;
use zoo_core::game::Game;
use zoo_core::level::LevelData;
use zoo_core::scene::LevelScene;

struct Counting;
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
/// `PROBE_TRACE=1`: allocation sites (first `zoo_core` frame of the backtrace) of 10 frames.
static TRACE: AtomicBool = AtomicBool::new(false);
static SITES: Mutex<Vec<String>> = Mutex::new(Vec::new());
thread_local! { static IN_TRACE: Cell<bool> = const { Cell::new(false) }; }

fn trace_site() {
    if !TRACE.load(Ordering::Relaxed) || IN_TRACE.with(|g| g.replace(true)) {
        return;
    }
    let bt = std::backtrace::Backtrace::force_capture().to_string();
    // the first frame whose source is in crates/zoo-core (the line after the symbol)
    let lines: Vec<&str> = bt.lines().map(str::trim).collect();
    let site = lines
        .iter()
        .position(|l| l.starts_with("at ") && l.contains("crates/zoo-core/src/"))
        .map(|i| {
            let sym = lines[i - 1].split_once(": ").map_or(lines[i - 1], |x| x.1);
            let at = lines[i].rsplit("crates/zoo-core/src/").next().unwrap_or(lines[i]);
            format!("{sym} ({at})")
        })
        .unwrap_or_else(|| "(outside zoo-core: probe / std)".into());
    SITES.lock().unwrap().push(site);
    IN_TRACE.with(|g| g.set(false));
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(l.size() as u64, Ordering::Relaxed);
        trace_site();
        System.alloc(l)
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        System.dealloc(p, l)
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(n as u64, Ordering::Relaxed);
        trace_site();
        System.realloc(p, l, n)
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

fn allocs() -> (u64, u64) {
    (ALLOCS.load(Ordering::Relaxed), BYTES.load(Ordering::Relaxed))
}

fn stats(mut v: Vec<f64>) -> Value {
    v.sort_by(|a, b| a.total_cmp(b));
    let q = |p: f64| v[((v.len() - 1) as f64 * p).round() as usize];
    json!({ "p50": q(0.5), "p95": q(0.95), "max": q(1.0), "mean": v.iter().sum::<f64>() / v.len() as f64 })
}

const LEVELS: [&str; 4] = [
    "levels/level-1.toml",
    "levels/level-2.toml",
    "levels/level-3.toml",
    "levels/night-1.toml",
];
const FRAMES: usize = 1200;
const DT: f32 = 1.0 / 60.0;

fn joined(assets: &Path) -> LevelData {
    let levels = LEVELS
        .iter()
        .map(|p| {
            let s = std::fs::read_to_string(assets.join(p)).expect(p);
            LevelData::from_toml_str(&s).unwrap_or_else(|e| panic!("{p}: {e}"))
        })
        .collect();
    LevelData::join(levels).expect("join levels")
}

/// Scripted stick: walks, turns slowly, stops now and then (collision, triggers, animals).
fn stick(k: usize) -> Vec2 {
    if (k / 180) % 4 == 3 {
        return Vec2::ZERO;
    }
    let a = k as f32 * 0.01;
    Vec2::new(a.cos(), a.sin())
}

fn game_update(assets: &Path) -> Value {
    let t = Instant::now();
    let data = joined(assets);
    let scene = LevelScene::build(&data);
    let build_ms = t.elapsed().as_secs_f64() * 1e3;
    let mut ambient = Ambient::new(&data, &scene, 7);
    let mut game = Game::new(data, 1).expect("game");
    let (mut poses, mut bflies, mut ripples) = (Vec::new(), Vec::new(), Vec::new());
    let (mut g_us, mut g_al, mut a_us, mut a_al) = (vec![], vec![], vec![], vec![]);
    let mut g_bytes = 0u64;
    let trace = std::env::var_os("PROBE_TRACE").is_some();
    for k in 0..FRAMES {
        TRACE.store(trace && (600..610).contains(&k), Ordering::Relaxed);
        let (a0, b0) = allocs();
        let t = Instant::now();
        game.update(DT, stick(k));
        let us = t.elapsed().as_secs_f64() * 1e6;
        let (a1, b1) = allocs();
        let t = Instant::now();
        ambient.update(DT, game.player.pos);
        ambient.poses(&mut poses);
        ambient.butterfly_poses(&mut bflies);
        ambient.ripples(&mut ripples);
        let aus = t.elapsed().as_secs_f64() * 1e6;
        let (a2, _) = allocs();
        if k >= 60 {
            // skip warm-up (first-frame capacity growth)
            g_us.push(us);
            g_al.push((a1 - a0) as f64);
            g_bytes += b1 - b0;
            a_us.push(aus);
            a_al.push((a2 - a1) as f64);
        }
    }
    TRACE.store(false, Ordering::Relaxed);
    let mut sites: BTreeMap<String, usize> = BTreeMap::new();
    for s in SITES.lock().unwrap().drain(..) {
        *sites.entry(s).or_default() += 1;
    }
    let mut sites: Vec<_> = sites.into_iter().collect();
    sites.sort_by(|a, b| b.1.cmp(&a.1));
    let n = g_al.len() as f64;
    json!({
        "alloc_sites_10_frames": sites.iter().map(|(s, c)| format!("{c} × {s}")).collect::<Vec<_>>(),
        "frames": g_us.len(),
        "scene_build_ms": build_ms,
        "game_update_us": stats(g_us),
        "game_allocs_per_frame": g_al.iter().sum::<f64>() / n,
        "game_alloc_bytes_per_frame": g_bytes as f64 / n,
        "game_allocs_max": g_al.iter().cloned().fold(0.0, f64::max),
        "ambient_us": stats(a_us),
        "ambient_allocs_per_frame": a_al.iter().sum::<f64>() / n,
        "ambient_poses": poses.len(),
        "butterflies": bflies.len(),
    })
}

/// Same work as `zoo_render::renderer::pose_character` (idle + walk sampled, blended, joint
/// matrices, copy into the joint texture buffer).
fn pose_cost(m: &Model) -> Option<Value> {
    let skel: &Skeleton = m.skeleton.as_ref()?;
    let idle = m.clips.iter().position(|c| c.name == "idle").or(Some(0))?;
    let walk = m.clips.iter().position(|c| c.name == "walk").unwrap_or(idle);
    let rest = skel.rest_pose();
    let (mut pi, mut pw, mut p) = (rest.clone(), rest.clone(), rest.clone());
    let mut world = vec![Mat4::IDENTITY; skel.node_count()];
    let mut joints = vec![Mat4::IDENTITY; skel.joint_count()];
    let mut data = vec![0f32; skel.joint_count() * 16];
    let n = 4000;
    let t = Instant::now();
    for k in 0..n {
        let tt = k as f32 * 0.013;
        skel.sample(&m.clips[idle], tt, &mut pi);
        skel.sample(&m.clips[walk], tt, &mut pw);
        Skeleton::blend(&pi, &pw, 0.5, &mut p);
        skel.joint_matrices(&p, &mut world, &mut joints);
        for (j, mm) in joints.iter().enumerate() {
            data[j * 16..j * 16 + 16].copy_from_slice(&mm.to_cols_array());
        }
    }
    let us = t.elapsed().as_secs_f64() * 1e6 / n as f64;
    std::hint::black_box(&data);
    Some(json!({
        "pose_us": us,
        "joints": skel.joint_count(),
        "nodes": skel.node_count(),
        "clips": m.clips.iter().map(|c| c.name.clone()).collect::<Vec<_>>(),
        "channels": m.clips.iter().map(|c| c.channels.len()).sum::<usize>(),
    }))
}

fn png_size(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 24 || &bytes[1..4] != b"PNG" {
        return None;
    }
    let w = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let h = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    Some((w, h))
}

fn glbs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            glbs(&p, out);
        } else if p.extension().is_some_and(|x| x == "glb") {
            out.push(p);
        }
    }
}

fn models(assets: &Path) -> (Value, Value) {
    let mut files = Vec::new();
    glbs(&assets.join("models"), &mut files);
    files.sort();
    let (mut list, mut anim) = (Vec::new(), Vec::new());
    for f in files {
        let rel = f.strip_prefix(assets).unwrap().to_string_lossy().into_owned();
        let bytes = std::fs::read(&f).unwrap();
        let m = match Model::from_glb(&bytes) {
            Ok(m) => m,
            Err(e) => {
                list.push(json!({ "path": rel, "error": e.to_string() }));
                continue;
            }
        };
        let tex = m.image.as_ref().and_then(|i| png_size(&i.bytes));
        list.push(json!({
            "path": rel,
            "kind": rel.split('/').nth(1).unwrap_or(""),
            "bytes": bytes.len(),
            "triangles": m.mesh.indices.len() / 3,
            "vertices": m.mesh.positions.len(),
            "parts": m.mesh.submeshes.len().max(1),
            "materials": m.materials.len(),
            "texture": tex.map(|(w, h)| format!("{w}x{h}")),
            "skinned": m.mesh.is_skinned(),
        }));
        if let Some(mut a) = pose_cost(&m) {
            a["path"] = json!(rel);
            anim.push(a);
        }
    }
    (Value::Array(list), Value::Array(anim))
}

fn main() {
    let root = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| ".".into()));
    let assets = root.join("assets");
    let (models, anim) = models(&assets);
    let out = json!({
        "game": game_update(&assets),
        "anim": anim,
        "models": models,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
