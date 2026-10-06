#!/usr/bin/env bash
# Builds the itch.io HTML5 package of Letter Zoo from a COMMITTED version in a clean worktree
# and writes a zip with index.html at its root (itch.io requirement).
#   scripts/itch-build.sh [commit-ish]        -> out/itch/letter-zoo-html5-<sha>.zip (+ latest symlink)
#   ITCH_TARGET=edugamegalaxy/letter-zoo:html5 scripts/itch-build.sh --push   -> also `butler push`
# `--push` needs butler (https://itch.io/docs/butler/) and a one-time `butler login` by the user.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REF="${1:-HEAD}"; PUSH=0
for a in "$@"; do [ "$a" = "--push" ] && PUSH=1; done
[ "$REF" = "--push" ] && REF=HEAD
SHA="$(git -C "$ROOT" rev-parse --short "$REF")"
D="${ITCH_WORKDIR:-${TMPDIR:-/tmp}/zoo-itch-build}"
OUT="$ROOT/out/itch"; mkdir -p "$OUT"
git -C "$ROOT" worktree remove --force "$D" 2>/dev/null || true; rm -rf "$D"
git -C "$ROOT" worktree add -q --detach "$D" "$REF"
( cd "$D/web" && npm ci --silent \
  && PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR="$D/target" CARGO_INCREMENTAL=0 npm run build )
test -f "$D/web/dist/index.html" || { echo "no dist/index.html"; exit 1; }
ZIP="$OUT/letter-zoo-html5-$SHA.zip"; rm -f "$ZIP"
( cd "$D/web/dist" && zip -q -r -9 "$ZIP" . )
ln -sf "$(basename "$ZIP")" "$OUT/letter-zoo-html5-latest.zip"
echo "zip: $ZIP ($(du -h "$ZIP" | cut -f1))"
if [ "$PUSH" = 1 ]; then
  command -v butler >/dev/null || { echo "butler not installed"; exit 1; }
  butler push "$D/web/dist" "${ITCH_TARGET:?set ITCH_TARGET=user/game:channel}" --userversion "$SHA"
fi
git -C "$ROOT" worktree remove --force "$D" || true
rm -rf "$D/target" 2>/dev/null || true
