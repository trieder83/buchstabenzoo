//! GAME-LAYOUT "Modular edges" (Q-057): 2 m / 1 m segment fill rule and level-1 edges.

mod common;

use glam::Vec2;
use zoo_core::level::{band_run, enclosure_fence, segment_run, RunAxis};
use zoo_core::ElementType;

fn check_fill(length: u32) {
    let segs = segment_run(length);
    let mut offset = 0;
    for (i, s) in segs.iter().enumerate() {
        assert_eq!(s.offset_m, offset, "L = {length}: gap before segment {i}");
        let last = i + 1 == segs.len();
        let want = if last && length % 2 == 1 { 1 } else { 2 };
        assert_eq!(s.length_m, want, "L = {length}: segment {i}");
        offset += s.length_m;
    }
    assert_eq!(offset, length, "L = {length}: pieces do not sum up");
    assert_eq!(segs.len() as u32, length.div_ceil(2));
    assert_eq!(
        segs.iter().filter(|s| s.length_m == 1).count() as u32,
        length % 2
    );
}

// LAYOUT-012
#[test]
fn layout_012_segment_run_fills_every_length() {
    // level-1 odd lengths first, then even ones, then everything up to 60
    for l in [3, 9, 13, 17, 21] {
        check_fill(l);
        let s = segment_run(l);
        assert_eq!(
            s.last().unwrap().length_m,
            1,
            "L = {l}: 1 m piece must be last"
        );
        assert_eq!(s.last().unwrap().offset_m, l - 1);
    }
    for l in [2, 4, 6, 8, 10, 12, 16, 48] {
        check_fill(l);
        assert!(segment_run(l).iter().all(|s| s.length_m == 2));
    }
    for l in 1..=60 {
        check_fill(l);
    }
}

// LAYOUT-013
#[test]
fn layout_013_level1_edges_fill_exactly() {
    let data = common::level1();
    let bounds = data.level.bounds;
    let mut bands = 0;
    for e in &data.elements {
        match e.kind.as_deref() {
            Some("hedge") | Some("zoo_wall") => {
                let run = band_run(e.rect, bounds)
                    .unwrap_or_else(|| panic!("{}: no unique band run for {:?}", e.id, e.rect));
                let (len, depth) = match run.axis {
                    RunAxis::X => (e.rect.w, e.rect.d),
                    RunAxis::Z => (e.rect.d, e.rect.w),
                };
                assert!(depth <= 2 && run.length_m as i32 == len, "{}", e.id);
                // centre line lies in the middle of the band
                let across = match run.axis {
                    RunAxis::X => run.start.y - e.rect.z as f32,
                    RunAxis::Z => run.start.x - e.rect.x as f32,
                };
                assert_eq!(across, depth as f32 / 2.0, "{}", e.id);
                let total: u32 = run.segments().iter().map(|s| s.length_m).sum();
                assert_eq!(total, run.length_m, "{}", e.id);
                bands += 1;
            }
            _ => {}
        }
    }
    assert!(bands >= 12, "only {bands} hedge/wall bands found");

    // bands along the level border run parallel to it
    let run = |id: &str| band_run(data.element(id).unwrap().rect, bounds).unwrap();
    assert_eq!(run("hedge_east_c").axis, RunAxis::Z);
    assert_eq!(run("hedge_east_b").axis, RunAxis::Z);
    assert_eq!(run("hedge_east_b").start, Vec2::new(23.0, 11.0));
    assert_eq!(run("wall_south_w").axis, RunAxis::X);
    assert_eq!(run("wall_west").axis, RunAxis::Z);
    assert_eq!(run("hedge_north_a").axis, RunAxis::X);
    assert_eq!(run("hedge_center_w").axis, RunAxis::Z);

    let mut enclosures = 0;
    for e in data
        .elements
        .iter()
        .filter(|e| e.ty == ElementType::Enclosure)
    {
        let f = enclosure_fence(e.rect, e.gate).unwrap_or_else(|err| panic!("{}: {err}", e.id));
        let gate = f.gate.unwrap_or_else(|| panic!("{}: no gate", e.id));
        assert_eq!(gate.length_m, 2, "{}", e.id);
        let perimeter = 2 * (e.rect.w + e.rect.d) as u32;
        let runs: u32 = f.runs.iter().map(|r| r.length_m).sum();
        assert_eq!(runs + 8 + 2, perimeter, "{}: fence does not close", e.id);
        for r in &f.runs {
            let total: u32 = r.segments().iter().map(|s| s.length_m).sum();
            assert_eq!(total, r.length_m, "{}", e.id);
        }
        enclosures += 1;
    }
    assert_eq!(enclosures, 3);
}
