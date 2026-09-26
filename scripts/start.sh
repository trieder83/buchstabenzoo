#!/usr/bin/env bash
# Control the Buchstabenzoo dev server (wasm-pack --dev build + Vite, reachable on the LAN).
#
# Usage: scripts/start.sh {start|stop|restart|status}
#   PORT=5173 scripts/start.sh start     # port can be overridden (default 5173)
#
# The server runs in the background in its own process group; PID and log live in .run/.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_DIR="$ROOT/.run"
PID_FILE="$RUN_DIR/dev.pid"
LOG_FILE="$RUN_DIR/dev.log"
PORT="${PORT:-5173}"
export PATH="$HOME/.cargo/bin:$PATH"

is_running() {
  [[ -f "$PID_FILE" ]] && kill -0 "$(cat "$PID_FILE")" 2>/dev/null
}

port_open() {
  (exec 3<>"/dev/tcp/127.0.0.1/$PORT") 2>/dev/null
}

lan_ip() {
  hostname -I 2>/dev/null | awk '{print $1}'
}

start() {
  if is_running; then
    echo "already running (pid $(cat "$PID_FILE"))"
    status
    return 0
  fi
  if port_open; then
    echo "port $PORT is already in use by another process — set PORT=... or free it" >&2
    exit 1
  fi
  mkdir -p "$RUN_DIR"
  [[ -d "$ROOT/web/node_modules" ]] || npm --prefix "$ROOT/web" install
  echo "building WASM (dev) and starting Vite on port $PORT … (log: $LOG_FILE)"
  # setsid: own process group, so stop can end npm, wasm-pack and vite together
  setsid nohup npm --prefix "$ROOT/web" run dev -- --port "$PORT" --strictPort \
    >"$LOG_FILE" 2>&1 < /dev/null &
  echo $! >"$PID_FILE"
  for _ in $(seq 1 300); do            # wasm-pack build can take a while
    if port_open; then
      status
      return 0
    fi
    if ! is_running; then
      echo "dev server exited during startup — last log lines:" >&2
      tail -n 20 "$LOG_FILE" >&2
      rm -f "$PID_FILE"
      exit 1
    fi
    sleep 1
  done
  echo "still starting after 300 s — check $LOG_FILE" >&2
}

stop() {
  if ! is_running; then
    echo "not running"
    rm -f "$PID_FILE"
    return 0
  fi
  local pid
  pid="$(cat "$PID_FILE")"
  kill -TERM -- "-$pid" 2>/dev/null || kill -TERM "$pid" 2>/dev/null || true
  for _ in $(seq 1 20); do
    kill -0 "$pid" 2>/dev/null || break
    sleep 0.5
  done
  kill -0 "$pid" 2>/dev/null && kill -KILL -- "-$pid" 2>/dev/null || true
  rm -f "$PID_FILE"
  echo "stopped"
}

status() {
  if is_running; then
    local state="starting"
    port_open && state="serving"
    echo "running (pid $(cat "$PID_FILE"), $state)"
    echo "  local:  http://localhost:$PORT/"
    local ip
    ip="$(lan_ip)"
    [[ -n "$ip" ]] && echo "  LAN:    http://$ip:$PORT/   (phone on the same network)"
    echo "  log:    $LOG_FILE"
  else
    echo "not running"
    return 3
  fi
}

case "${1:-}" in
  start) start ;;
  stop) stop ;;
  restart) stop; start ;;
  status) status ;;
  *)
    echo "usage: $(basename "$0") {start|stop|restart|status}" >&2
    exit 2
    ;;
esac
