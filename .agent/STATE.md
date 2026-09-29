# Agent state (read this first)

Short, current snapshot for every agent. Updated by the main session after each commit and
by agents when they finish or stop (append a handoff under "Handoffs", ≤ 15 lines).
Decisions: `.agent/DECISIONS.md` (generated digest of `specs/open-questions.md`).
Work queue: `.agent/TODO.md`.

## Now (2026-09-28)

- **HEAD:** `c9787d3` + uncommitted work (hints/🌙 indicator, enterable storages, perf
  R-002a/005/014/015 — finished, waiting for the running agents, then one commit).
- **Live:** https://letterzoo.web.app = commit `3513018` (deploy via the `deploy` skill;
  the `firebase deploy` step is run by the user with `!`).
- **Running agents and what they own:**
  - food boxes back outside + inside crates, street under the moon door, hints tests
    → `assets/levels/`, `crates/zoo-core/tests/hints.rs`, feeding/layout/level specs; Q-194–195.
  - performance (Q-191 outline AA, Q-193 haze culling): **paused** — stopped by the user on
    2026-09-28 before reporting; any partial edits are in `crates/zoo-render/`, the render path of
    `crates/zoo-web/src/lib.rs`, `specs/50-performance/`. Check `git diff` there before resuming.
- **Known red tests:** CAMV-014 (fixed by Q-193, in progress); `hint_002`/`hint_014` (box
  positions in flux).

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

