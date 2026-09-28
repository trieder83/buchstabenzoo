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
  - performance: outline anti-aliasing (Q-191), haze culling on (Q-193) → `crates/zoo-render/`,
    render path of `crates/zoo-web/src/lib.rs`, `specs/50-performance/`; Q-196+.
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

## Handoffs

<!-- Agents: append "### <date> <agent/topic>" with done / in progress (file + state) /
next steps / questions. The main session prunes entries once committed. -->
