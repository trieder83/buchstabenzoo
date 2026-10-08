#!/usr/bin/env node
// Live check of firestore.rules against the REAL project (PLAT-050): a valid increment must pass, every
// invalid write / read / delete must be refused with 403. The only document it can create is the marked one
// c/20000101_session_start__web_de_rulecheck (day 2000-01-01, version "rulecheck"): it never touches a real
// counter; every run adds 2 to its n (delete it in the console if you like). Run: node --experimental-strip-types tools/analytics/rules_check.mjs
import { COUNTER_API_KEY, COUNTER_PROJECT } from '../../web/src/counter-config.ts';

const base = `projects/${COUNTER_PROJECT}/databases/(default)/documents`;
const api = `https://firestore.googleapis.com/v1/${base}`;
const key = `?key=${COUNTER_API_KEY}`;
const f = (o) => Object.fromEntries(Object.entries(o).map(([k, v]) => [k, { stringValue: v }]));

function write(id, fields, n = '1') {
  const keys = Object.keys(fields);
  return {
    writes: [
      {
        update: { name: `${base}/c/${id}`, fields: f(fields) },
        updateMask: { fieldPaths: keys },
        updateTransforms: [{ fieldPath: 'n', increment: { integerValue: n } }],
      },
    ],
  };
}
const good = { date: '20000101', event: 'session_start', param: '', platform: 'web', lang: 'de', v: 'rulecheck' };
const goodId = '20000101_session_start__web_de_rulecheck';
const post = async (body) => (await fetch(`${api}:commit${key}`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) })).status;

const cases = [
  ['valid increment (create or +1)', 200, () => post(write(goodId, good))],
  ['valid increment again (+1)', 200, () => post(write(goodId, good))],
  ['extra field', 403, () => post(write(goodId, { ...good, user: 'x' }))],
  ['bad event', 403, () => post(write('20000101_test_ping__web_de_rulecheck', { ...good, event: 'test_ping' }))],
  ['n = 5', 403, () => post(write(goodId, good, '5'))],
  ['bad platform', 403, () => post(write('20000101_session_start__tv_de_rulecheck', { ...good, platform: 'tv' }))],
  ['bad lang', 403, () => post(write('20000101_session_start__web_fr_rulecheck', { ...good, lang: 'fr' }))],
  ['param with space', 403, () => post(write('20000101_session_start_a b_web_de_rulecheck', { ...good, param: 'a b' }))],
  ['id does not match fields', 403, () => post(write('foo', good))],
  ['other collection', 403, () => post({ writes: [{ update: { name: `${base}/x/a`, fields: f({ a: 'b' }) } }] })],
  ['read document', 403, async () => (await fetch(`${api}/c/${goodId}${key}`)).status],
  ['list collection', 403, async () => (await fetch(`${api}/c${key}`)).status],
  ['delete', 403, () => post({ writes: [{ delete: `${base}/c/${goodId}` }] })],
];
let bad = 0;
for (const [name, want, run] of cases) {
  const got = await run();
  const ok = got === want;
  if (!ok) bad++;
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${got} (want ${want})  ${name}`);
}
process.exit(bad ? 1 : 0);
