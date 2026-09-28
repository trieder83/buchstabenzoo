#!/usr/bin/env bash
# Build the release game and deploy it to a Firebase Hosting preview channel
# (PLAT-003..PLAT-007, docs/deploy-preview.md). Needs firebase-tools, `firebase login`
# and a .firebaserc (copy .firebaserc.example). Usage:
#   scripts/deploy-preview.sh [channel] [expires]     # defaults: playtest 30d
set -euo pipefail
cd "$(dirname "$0")/.."

channel="${1:-playtest}"
expires="${2:-30d}"

if [ ! -f .firebaserc ]; then
  echo "Missing .firebaserc — run: cp .firebaserc.example .firebaserc and set your project id." >&2
  exit 1
fi
command -v firebase >/dev/null || { echo "firebase CLI missing — run: npm i -g firebase-tools" >&2; exit 1; }

npm --prefix web run build

# PLAT-007: the release build must stay under the 30 MB download budget (PLAT-001).
bytes=$(du -sb web/dist | cut -f1)
limit=$((30 * 1024 * 1024))
if [ "$bytes" -gt "$limit" ]; then
  echo "web/dist is $bytes bytes (> 30 MB, PLAT-001) — not deploying." >&2
  exit 1
fi
echo "web/dist: $((bytes / 1024 / 1024)) MB"

# Prints the preview URL (https://<project>--<channel>-<hash>.web.app) when done.
firebase hosting:channel:deploy "$channel" --expires "$expires"
