#!/usr/bin/env node
// Reads the anonymous counters (collection `c` of the Firestore project letterzoo) and prints sums.
//
//   node tools/analytics/report.mjs [--from YYYYMMDD] [--to YYYYMMDD] [--by day|event|param|platform|lang|v ...] [--event NAME] [--csv]
//
// Default: the last 7 days, rows per event + param, summed over days, platforms, languages and versions.
// Examples:
//   node tools/analytics/report.mjs                                 # last 7 days
//   node tools/analytics/report.mjs --from 20261001 --by day,event  # per day and event
//   node tools/analytics/report.mjs --event lock_wrong --by platform
//
// Credentials: your own login, never printed. Needs `gcloud auth login` once (account with access to the project):
// the access token comes from `gcloud auth print-access-token`. Clients (the game) cannot read, only you can.
// Without gcloud: open https://console.firebase.google.com/project/letterzoo/firestore/databases/-default-/data/~2Fc
// (collection `c`, documents <day>_<event>_<param>_<platform>_<lang>_<v> with the field `n`).
import { execFileSync } from 'node:child_process';

const PROJECT = 'letterzoo';
const args = process.argv.slice(2);
const opt = (name, def) => {
  const i = args.indexOf(`--${name}`);
  return i >= 0 ? args[i + 1] : def;
};
const day = (d) => `${d.getUTCFullYear()}${String(d.getUTCMonth() + 1).padStart(2, '0')}${String(d.getUTCDate()).padStart(2, '0')}`;
const to = opt('to', day(new Date()));
const from = opt('from', day(new Date(Date.now() - 6 * 86400_000)));
const by = opt('by', 'event,param').split(',').filter(Boolean);
const only = opt('event', '');
const csv = args.includes('--csv');
for (const b of by) if (!['day', 'event', 'param', 'platform', 'lang', 'v'].includes(b)) throw new Error(`unknown --by ${b}`);

let token;
try {
  token = execFileSync('gcloud', ['auth', 'print-access-token'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
} catch {
  console.error('gcloud is not available or not logged in: run `gcloud auth login`, or read the collection `c` in the Firebase console (see the header of this file).');
  process.exit(2);
}

const url = `https://firestore.googleapis.com/v1/projects/${PROJECT}/databases/(default)/documents:runQuery`;
const filters = [
  { fieldFilter: { field: { fieldPath: 'date' }, op: 'GREATER_THAN_OR_EQUAL', value: { stringValue: from } } },
  { fieldFilter: { field: { fieldPath: 'date' }, op: 'LESS_THAN_OR_EQUAL', value: { stringValue: to } } },
];
const rows = [];
for (let offset = 0; ; ) {
  const res = await fetch(url, {
    method: 'POST',
    headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json', 'X-Goog-User-Project': PROJECT },
    body: JSON.stringify({
      structuredQuery: {
        from: [{ collectionId: 'c' }],
        where: { compositeFilter: { op: 'AND', filters } },
        orderBy: [{ field: { fieldPath: 'date' }, direction: 'ASCENDING' }],
        offset,
        limit: 500,
      },
    }),
  });
  if (!res.ok) {
    console.error(`Firestore ${res.status}: ${(await res.text()).slice(0, 300)}`);
    process.exit(1);
  }
  const part = (await res.json()).filter((r) => r.document);
  for (const { document } of part) {
    const f = document.fields;
    rows.push({
      day: f.date.stringValue,
      event: f.event.stringValue,
      param: f.param.stringValue,
      platform: f.platform.stringValue,
      lang: f.lang.stringValue,
      v: f.v.stringValue,
      n: Number(f.n.integerValue ?? 0),
    });
  }
  offset += part.length;
  if (part.length < 500) break;
}

const sums = new Map();
for (const r of rows) {
  if (only && r.event !== only) continue;
  const key = by.map((b) => r[b]).join('\t');
  sums.set(key, (sums.get(key) ?? 0) + r.n);
}
const out = [...sums.entries()].sort((a, b) => a[0].localeCompare(b[0]));
console.error(`# ${from}..${to} (UTC days), ${rows.length} documents, grouped by ${by.join(', ')}`);
if (csv) console.log([...by, 'n'].join(','));
for (const [k, n] of out) {
  const cols = k.split('\t');
  console.log(csv ? [...cols, n].join(',') : [...cols.map((c) => c.padEnd(28)), String(n).padStart(8)].join(' '));
}
