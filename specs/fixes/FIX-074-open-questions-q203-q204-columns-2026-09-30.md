# FIX-074 Q-203/Q-204 rows had missing table columns

- **Found:** 2026-09-30 by spec-manager.
- **Problem:** Q-203 and Q-204 in `specs/open-questions.md` ended with `| open |` (specs / blocks columns missing), so the digest could mis-parse them.
- **Fix:** added `GAME-FAMILY, ART-ANIMALS | no | open | |`; regenerated DECISIONS.md and CODEMAP.md.
