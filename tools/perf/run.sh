#!/usr/bin/env bash
# Performance run (PERF-BUDGETS, specs/50-performance/measurements.md): one command that
#   1. snapshots the working tree (committed + uncommitted) into $PERF_WORK/tree, so builds
#      never touch crates/zoo-web/pkg, web/dist or a running dev server,
#   2. builds the WASM module --dev and --release (sizes: raw, before/after wasm-opt, gzip,
#      brotli) and the release web bundle (total download, PLAT-001),
#   3. runs the native probe (zoo-core update cost + allocations, animation sampling,
#      per-model triangles/textures),
#   4. runs the fixed browser scenarios (web/tests/e2e/perf/scenarios.perf.ts) against a
#      `vite preview` of the snapshot on port $PERF_PORT (default 4190; killed afterwards),
#   5. writes $PERF_WORK/out/results.json and prints a Markdown summary for measurements.md
#      (values > 10 % worse than the newest tools/perf/baselines/*.json in bold). After a
#      logged run, copy results.json to tools/perf/baselines/<date>.json.
#
# Usage: tools/perf/run.sh [--skip-build] [--only-sizes] [--no-browser]
#   --skip-build reuses the last snapshot and builds (e.g. to re-run the browser part).
# Env:   PERF_WORK (default ~/.cache/buchstabenzoo-perf), PERF_PORT (4190),
#        PERF_ANGLE (swiftshader | gl | vulkan | default — Chromium --use-angle; swiftshader
#        = software rendering, numbers relative only), PERF_FRAMES (frames per sample, 16),
#        PERF_VIEWPORTS (desktop,phone,desktop_half), PERF_SCENARIOS (S01,S09,…),
#        PERF_PREVIOUS (results.json to compare with).
set -euo pipefail

REPO="$(cd "$(dirname "$0")/../.." && pwd)"
WORK="${PERF_WORK:-$HOME/.cache/buchstabenzoo-perf}"
TREE="$WORK/tree"
OUT="$WORK/out"
PORT="${PERF_PORT:-4190}"
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$WORK/target"
SKIP_BUILD=0; ONLY_SIZES=0; NO_BROWSER=0
for a in "$@"; do
  case "$a" in
    --skip-build) SKIP_BUILD=1 ;;
    --only-sizes) ONLY_SIZES=1 ;;
    --no-browser) NO_BROWSER=1 ;;
    *) echo "unknown option $a" >&2; exit 2 ;;
  esac
done
mkdir -p "$WORK" "$OUT"
# keep the previous run for the regression flags (> 10 % worse in bold)
[ -f "$OUT/results.json" ] && cp "$OUT/results.json" "$WORK/previous-results.json"
rm -f "$OUT/browser.json"

# --- 1. snapshot ----------------------------------------------------------------------
cd "$REPO"
COMMIT="$(git rev-parse --short HEAD)"
DIRTY="$(git status --porcelain | wc -l | tr -d ' ')"
DIFF_HASH="$(git diff HEAD | sha1sum | cut -c1-10)"
{
  echo "{\"commit\":\"$COMMIT\",\"dirty_files\":$DIRTY,\"diff_sha1\":\"$DIFF_HASH\","
  echo "\"date\":\"$(date -Iseconds)\",\"host\":\"$(uname -srm)\",\"cpu\":\"$(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | xargs)\","
  echo "\"loadavg\":\"$(cut -d' ' -f1-3 /proc/loadavg)\",\"cpus\":$(nproc),"
  echo "\"rustc\":\"$(rustc --version)\",\"wasm_pack\":\"$(wasm-pack --version)\"}"
} > "$OUT/meta.json"
git status --porcelain > "$OUT/git-status.txt"
if [ "$SKIP_BUILD" = 0 ]; then
  rsync -a --delete \
    --exclude '/.git' --exclude '/target' --exclude '/art' --exclude '/Assets' \
    --exclude '/.claude' --exclude '/qa' --exclude 'node_modules' --exclude '/web/dist' \
    --exclude '/web/test-results' --exclude '/crates/zoo-web/pkg' --exclude '/tools/perf/probe/target' \
    "$REPO/" "$TREE/"
  ln -sfn "$REPO/web/node_modules" "$TREE/web/node_modules"
fi

# --- 2. builds and sizes ------------------------------------------------------------------
build() { # label, dir, command… — full output in $OUT/build.log, tail on failure
  local label="$1" dir="$2"; shift 2
  echo "== $label"
  local t0=$SECONDS
  if ! ( cd "$dir" && "$@" ) >> "$OUT/build.log" 2>&1; then
    tail -40 "$OUT/build.log"; echo "!! $label failed (snapshot of a tree in the middle of an edit? re-run)" >&2; exit 1
  fi
  echo "   $((SECONDS - t0)) s"
}
if [ "$SKIP_BUILD" = 0 ]; then
  : > "$OUT/build.log"
  build "wasm-pack --dev" "$TREE" wasm-pack build crates/zoo-web --target web --dev --out-dir "$WORK/pkg-dev"
  build "wasm-pack --release" "$TREE" wasm-pack build crates/zoo-web --target web --release
  build "tsc + vite build" "$TREE/web" sh -c 'npx tsc --noEmit && npx vite build --logLevel warn'
fi
node "$REPO/tools/perf/sizes.mjs" "$WORK" > "$OUT/sizes.json"
[ "$ONLY_SIZES" = 1 ] && { cat "$OUT/sizes.json"; exit 0; }

# --- 3. native probe ------------------------------------------------------------------------
echo "== native probe"
mkdir -p "$TREE/tools/perf" && rsync -a --exclude target "$REPO/tools/perf/probe" "$TREE/tools/perf/"
cargo run --quiet --release --manifest-path "$TREE/tools/perf/probe/Cargo.toml" -- "$TREE" > "$OUT/probe.json"

# --- 4. browser scenarios -----------------------------------------------------------------
if [ "$NO_BROWSER" = 0 ]; then
  echo "== browser scenarios (port $PORT, angle ${PERF_ANGLE:-swiftshader})"
  cp "$REPO/web/tests/e2e/perf/"* "$TREE/web/tests/e2e/perf/"
  ( cd "$TREE/web" && PERF_OUT="$OUT/browser.json" PERF_PORT="$PORT" \
      npx playwright test -c tests/e2e/perf/perf.config.ts ) || echo "!! browser scenarios failed (see above)"
  # the Playwright webServer is stopped by Playwright; make sure nothing is left on the port
  pkill -f "vite preview --port $PORT" 2>/dev/null || true
fi

# --- 5. report ------------------------------------------------------------------------------
# compare with PERF_PREVIOUS, else the newest committed baseline, else the last local run
PREV="${PERF_PREVIOUS:-$(ls -1 "$REPO"/tools/perf/baselines/*.json 2>/dev/null | tail -1)}"
PREV="${PREV:-$WORK/previous-results.json}"
echo "(regressions vs. $PREV)"
node "$REPO/tools/perf/report.mjs" "$OUT" "$PREV" | tee "$OUT/summary.md"
