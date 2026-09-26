# qa/ — gameplay quality

Maintained by the `gameplay-qa` agent (`.claude/agents/gameplay-qa.md`).

- `checklist.md` — what every gameplay QA run checks (movement, collision, interaction,
  camera, touch, mission flow, child-friendliness, performance smoke).
- `reports/YYYY-MM-DD-<topic>.md` — one report per QA run: findings with severity, violated
  spec rule, reproduction, expected vs. actual, screenshot, suggested fix.
- `reports/img/` — small screenshots referenced by the reports.

Regression tests for confirmed bugs live in `crates/zoo-core/tests/` (logic) and
`web/tests/e2e/gameplay/` (input, UI, rendering).
