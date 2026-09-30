# Agent state (read this first)

Short, current snapshot for every agent. Updated by the main session after each commit and
by agents when they finish or stop (append a handoff under "Handoffs", ≤ 15 lines).
Decisions: `.agent/DECISIONS.md` (generated digest of `specs/open-questions.md`).
Work queue: `.agent/TODO.md`.

## Now (2026-09-29, end of week)

- **HEAD:** `6373543` (+ this state update). Working tree clean.
- **Live:** https://letterzoo.web.app = commit `6373543` (deployed 2026-09-29 via the `deploy`
  skill; the user runs the `firebase deploy` command with `!`).
- **Running agents:** none. Performance work is **paused** until the next optimisation round
  (outline AA Q-191 / PERF-R-016: prototypes + next steps in `specs/50-performance/recommendations.md`).
- **Known red tests:** none in the round's specs (344 Rust, 55 web, 46 e2e green on 6373543).
  The full e2e suite (≈ 65 min) was not run in full this round.
- **Next week:** start from `.agent/TODO.md` "Next"; run `spec-manager` first (INDEX, FIX log
  for this round, DECISIONS/CODEMAP regenerate).

## Conventions

- Agents **never commit**; the main session commits after checking.
- Cargo: `~/.cargo/bin/cargo`. Dev server on :5173 (`scripts/start.sh`) — leave it alone; use
  your own port for previews and stop them. Keep ≥ 10 GB disk free; delete your build copies.
- New open questions only in the Q range assigned above; next free number is in
  `DECISIONS.md`. After editing `specs/open-questions.md` run `python3 tools/agent_state.py`.
- **Find code:** `.agent/CODEMAP.md` (generated) — grep it for a file/function/test ID, then Read with offset/limit. Don't list or read whole directories/crates.
- **Save tokens:** don't read big specs whole (`open-questions.md` 120 KB, `levels/level-1.md`
  90 KB, `layout.md` 60 KB, `levels/level-2/3.md`, `night-1.md` 40 KB): `grep -n` for the ID /
  section, then read only those lines. Read `DECISIONS.md` instead of `open-questions.md`
  unless you need a question's full text.
- `cargo fmt` only your own files when others work in parallel (`rustfmt <files>`).
- e2e runs rewrite `art/environment/poc/*.png` review shots — that's expected.

## Testing rules

test only what you changed, by spec ID —
`~/.cargo/bin/cargo test -q -p <crate> --test <file>` (or `-- <name>`), `npm --prefix web test -- <file>`,
and e2e only via `scripts/e2e.sh <spec files | --grep ID>` (one run at a time via a lock, no rebuild —
add `--build` once after Rust/web changes, hard timeout that kills the whole process group, prints only
failures + totals; full log in `.run/e2e-last.log`). Never run the full e2e suite and never
`npm run test:e2e` — the main session runs the full checks once before committing. Run
`cargo clippy -p <crate>` and `rustfmt <your files>` only for crates/files you touched. Read test output
as summaries (`-q`, `| tail`), never whole logs. Screenshots go to `web/test-results/shots/` unless the
task asks for review shots (`UPDATE_SHOTS=1`). A flaky/timeout failure under load: rerun that one test
once, then report it — don't loop.

Main session before a commit: `cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test --workspace`, `npm --prefix web run lint && npm --prefix web test`, then `scripts/e2e.sh --build tests/e2e` (full suite, ≈ 65 min) only for bigger rounds or before a deploy.

## Handoffs

<!-- Agents: append "### <date> <agent/topic>" with done / in progress (file + state) /
next steps / questions. The main session prunes entries once committed. -->

### 2026-09-28 general — Q-181 boxes outside / Q-182 moon street / hints
- Done (uncommitted): labelled food boxes back outside in all 4 storages (door gaps widened for LAYOUT-038/034; night hut: one row south of its door); 6/6/6/2 unlabelled stock crates inside as `[[prop]]` `food_box` (Q-194, FEED-027); `path_moon` under the moon door (LAYOUT-040 incl. moon door); food hint points at the storage door (Q-187); HINT-012 timing-independent; hints `walk_to` tolerates a collider-blocked cell centre (HINT-014).
- Specs: feeding §7, layout, levels 1/2/3/night-1, hints (Q-186/187/189 answered wording), open-questions (Q-181/182 notes, new Q-194).
- Tests: cargo test --workspace green, clippy green, wasm dev build ok, npm lint/test ok; e2e green: LAYOUT-043, doors, feeding, save, mission, m5a, zebra-mission-levels (m5b/hints/level_gates: see report).
- Next: spec-manager run (INDEX); Q-194 open for the user.

### 2026-09-29 performance — Q-193 haze culling on, Q-191 outline AA paused
- Done (uncommitted): `Renderer::haze_cull` default on (PERF-R-018); `Cull::all` replaces the `no_culling` shortcuts; new debug `debug_haze_cull(on)`; PERF-025 turns haze off in both frames; budget 23 names the exception. CAMV-014 green.
- Q-191: the earlier agent's partial work was only scratch shader patches (no AA code was in the tree). Measured box4/tent5/tri3/diag2 + a two-pass R8-mask form; all AA code removed from the renderer again (outline pass = committed look). Prototypes kept in `tools/perf/outline_aa_variants.mjs`; numbers in measurements.md run 2026-09-29, PERF-R-016 entry.
- Next: PERF-R-016 "Next" — review shots of `box4`, cut its cost (≈ +1–1.8 ms/frame on the iGPU) to ≤ +0.5 ms, then implement in `post_fs` + a flicker PERF test.
- e2e: CAMV-014 + PERF-025 (4) green after `--build`. Not run (paused by the user): m5b / hints / level_gates / camera_views (rest) / night / quality / smoke.
- Fixed `scripts/e2e.sh`: the timeout watchdog's `sleep` kept the lock fd open for the whole timeout after every run (blocked other agents up to 45 min) — now `9>&-` + `pkill -P`.

### 2026-09-29 general — Q-194 answered: inside boxes are real food too
- Done (uncommitted): the 6/6/6/2 unlabelled stock crates became real `[[food_box]]` entries (same positions, foods = the level's animals' foods repeated) in `assets/levels/{level-1,level-2,level-3,night-1}.toml`; `scene.rs` elevates a food box to the storage's plank platform only when its centre falls inside the storage rect (no rendering/render-path files touched). `take_food`/`near_own_box` already handled duplicate boxes per food correctly (nearest/any within range) — no code fix needed there.
- Tests: new FEED-028 (unit + e2e) "interact with an inside box → panel → take"; FEED-008/027 and LAYOUT-L1-013/L2-011/L3-011/N1-010 updated for outside-unique-inside-may-repeat; PLAY-020 count 3+16+4; `cargo test --workspace` green, clippy -p zoo-core clean, rustfmt applied; e2e green: `FEED-|LAYOUT-041|LAYOUT-043` (5 tests, incl. rewritten `doors.spec.ts`).
- Specs: feeding §7/test table, layout.md, levels 1/2/3/night-1 (element notes + inside-box tables), open-questions Q-194 "Implemented 2026-09-29".
- Next: spec-manager run (INDEX, glossary); no open questions from me.


### 2026-09-30 general — session specs implemented (commits 07a6d78…15748b5)
- Done: grass 1.45 m/s (PLAY-006, walking tables re-measured), hint rule 4a + step line (HINT-015/016), entrance intro (RESC-029, `?intro=1` for e2e; webdriver skips it), level-start tests (LAYOUT-044/045/046, ANIM-013), garden street + aprons to 4 gates (LAYOUT-L1-044, `loc_river` widened), baby by special food (FAM-008/009; pair flag still off in the data, no baby model → FAM-003/004/005/006/007 open).
- Not done: sound (ART-SOUND: sound-artist agent must name a generator API first, Q-200; no Web Audio host code yet), read-aloud for hints/intro on `kiga` (Q-008), `ice_cream_kiosk` (level 3) has no street within 2 m (decoration, not covered by rule 8), walking-table text of levels 2/3 in the specs still shows old times (tests hold the new numbers).
