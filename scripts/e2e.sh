#!/usr/bin/env bash
# Run Playwright e2e tests efficiently and safely (see .agent/STATE.md "Testing rules").
#
#   scripts/e2e.sh [--build] [--timeout MIN] <spec files / --grep ID ...>
#
# - One run at a time: a lock queues parallel agents instead of overloading the machine.
# - No rebuild: uses the existing web/dist (build once with --build or `npm --prefix web run build`).
# - Hard timeout that kills the WHOLE process group (playwright, vite preview, browsers).
# - Short output: failures + totals; the full log is in .run/e2e-last.log.
# - Screenshots go to web/test-results/shots (ignored) unless UPDATE_SHOTS=1.
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$ROOT/.run"
LOCK="$ROOT/.run/e2e.lock"
LOG="$ROOT/.run/e2e-last.log"
BUILD=0
TIMEOUT_MIN=45
while [[ $# -gt 0 ]]; do
  case "$1" in
    --build) BUILD=1; shift ;;
    --timeout) TIMEOUT_MIN="$2"; shift 2 ;;
    *) break ;;
  esac
done
if [[ $# -eq 0 ]]; then
  echo "usage: scripts/e2e.sh [--build] [--timeout MIN] <spec files | --grep ID ...>" >&2
  echo "(the full suite is run only by the main session before a commit: scripts/e2e.sh tests/e2e)" >&2
  exit 2
fi

exec 9>"$LOCK"
if ! flock -n 9; then
  echo "e2e: another run is active — waiting for the lock ..."
  flock 9
fi

cd "$ROOT/web"
if [[ $BUILD -eq 1 || ! -f dist/index.html ]]; then
  echo "e2e: building release bundle once ..."
  PATH="$HOME/.cargo/bin:$PATH" npm run build >"$ROOT/.run/e2e-build.log" 2>&1 \
    || { echo "e2e: build failed — see .run/e2e-build.log"; tail -20 "$ROOT/.run/e2e-build.log"; exit 1; }
fi

# own process group so the timeout can kill everything below it
setsid npx playwright test "$@" >"$LOG" 2>&1 &
PID=$!
( sleep $((TIMEOUT_MIN * 60)); echo "e2e: TIMEOUT after ${TIMEOUT_MIN} min — killing the run" >>"$LOG";
  kill -TERM -- -"$PID" 2>/dev/null; sleep 20; kill -KILL -- -"$PID" 2>/dev/null ) 9>&- &
WATCH=$!
wait "$PID"; RC=$?
# the watchdog's `sleep` must die too (it used to keep the lock fd open for the whole timeout)
pkill -P "$WATCH" 2>/dev/null; kill "$WATCH" 2>/dev/null; wait "$WATCH" 2>/dev/null
pkill -f "vite preview --port ${E2E_PORT:-4173}" 2>/dev/null

grep -E "✘|^\s+[0-9]+ (passed|failed|flaky|skipped|did not run)|TIMEOUT|Error:" "$LOG" | head -40
echo "e2e: exit $RC (full log: .run/e2e-last.log)"
exit $RC
