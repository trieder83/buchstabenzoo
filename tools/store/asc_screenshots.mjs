#!/usr/bin/env node
// Idempotent App Store screenshot upload for Letter Zoo / Buchstabenzoo (spec: TECH-STORE, store/appstore/README.md).
//
//   ASC_KEY_ID=... ASC_ISSUER_ID=... ASC_KEY_P8_PATH=~/.appstoreconnect/AuthKey_X.p8 \
//     node tools/store/asc_screenshots.mjs [--dry-run] [--verify] [--locale de-DE]
//
// Reads store/appstore/screenshots/<locale>/<device>/NN_name.png (device dir -> display type below). en-US and en-GB
// use the images of en-US unless an en-GB directory exists. Per set: screenshots whose file checksum matches are kept,
// the others are deleted and re-uploaded, then the order is set by file name. --verify only GETs and prints a table.
// The JWT is signed locally; key and token are never printed. Nothing is ever submitted for review.
import { createSign, createHash } from 'node:crypto';
import { readFileSync, readdirSync, existsSync, statSync } from 'node:fs';
import { homedir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const args = process.argv.slice(2);
const DRY = args.includes('--dry-run');
const VERIFY = args.includes('--verify');
const ONLY = args.includes('--locale') ? args[args.indexOf('--locale') + 1] : null;
const APP_ID = process.env.ASC_APP_ID || '6820559066';
const VERSION = '1.0';
const LOCALES = ['de-DE', 'en-US', 'en-GB'];
const SRC_DIR = { 'de-DE': 'de-DE', 'en-US': 'en-US', 'en-GB': 'en-US' }; // en-GB reuses the en-US images
// device directory -> [displayType, width, height]. The API has no APP_IPHONE_69: 6.9" (1320x2868) goes into the APP_IPHONE_67 slot.
const DEVICES = {
  'iphone-6.9': ['APP_IPHONE_67', 1320, 2868],
  'ipad-13': ['APP_IPAD_PRO_3GEN_129', 2064, 2752],
};

const keyId = process.env.ASC_KEY_ID, issuerId = process.env.ASC_ISSUER_ID;
const p8Path = process.env.ASC_KEY_P8_PATH?.replace(/^~/, homedir());
if (!keyId || !issuerId || !p8Path || !existsSync(p8Path)) {
  console.error('Missing ASC_KEY_ID / ASC_ISSUER_ID / ASC_KEY_P8_PATH (or file not found).');
  process.exit(1);
}
const b64u = (b) => Buffer.from(b).toString('base64url');
function jwt() {
  const iat = Math.floor(Date.now() / 1000);
  const h = b64u(JSON.stringify({ alg: 'ES256', kid: keyId, typ: 'JWT' }));
  const p = b64u(JSON.stringify({ iss: issuerId, iat, exp: iat + 1200, aud: 'appstoreconnect-v1' }));
  const s = createSign('SHA256'); s.update(`${h}.${p}`);
  return `${h}.${p}.${b64u(s.sign({ key: readFileSync(p8Path), dsaEncoding: 'ieee-p1363' }))}`;
}
const TOKEN = jwt();
const API = 'https://api.appstoreconnect.apple.com';

async function api(path, { method = 'GET', body } = {}) {
  if (method !== 'GET' && DRY) { console.log(`[dry] ${method} ${path} ${JSON.stringify(body)?.slice(0, 200)}`); return null; }
  const res = await fetch(API + path, { method, headers: { Authorization: `Bearer ${TOKEN}`, ...(body ? { 'Content-Type': 'application/json' } : {}) }, body: body ? JSON.stringify(body) : undefined });
  const text = await res.text();
  if (!res.ok) throw new Error(`${method} ${path} -> ${res.status}\n${text.slice(0, 1500)}`);
  return text ? JSON.parse(text) : null;
}
const log = (...a) => console.log(...a);
const md5 = (buf) => createHash('md5').update(buf).digest('hex');
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

const version = (await api(`/v1/apps/${APP_ID}/appStoreVersions?filter[versionString]=${VERSION}`)).data[0];
if (!version) throw new Error(`no version ${VERSION}`);
const vlocs = (await api(`/v1/appStoreVersions/${version.id}/appStoreVersionLocalizations`)).data;

async function listShots(setId) {
  return (await api(`/v1/appScreenshotSets/${setId}/appScreenshots?limit=50`)).data;
}

async function upload(setId, file) {
  const buf = readFileSync(file);
  const name = file.split('/').pop();
  const created = (await api('/v1/appScreenshots', { method: 'POST', body: { data: { type: 'appScreenshots',
    attributes: { fileName: name, fileSize: buf.length }, relationships: { appScreenshotSet: { data: { type: 'appScreenshotSets', id: setId } } } } } }))?.data;
  if (!created) return null;
  for (const op of created.attributes.uploadOperations) {
    const headers = Object.fromEntries(op.requestHeaders.map((h) => [h.name, h.value]));
    const res = await fetch(op.url, { method: op.method, headers, body: buf.subarray(op.offset, op.offset + op.length) });
    if (!res.ok) throw new Error(`upload part ${op.offset} -> ${res.status}`);
  }
  await api(`/v1/appScreenshots/${created.id}`, { method: 'PATCH', body: { data: { type: 'appScreenshots', id: created.id, attributes: { uploaded: true, sourceFileChecksum: md5(buf) } } } });
  for (let i = 0; i < 150; i++) {
    const s = (await api(`/v1/appScreenshots/${created.id}`)).data;
    const st = s.attributes.assetDeliveryState?.state;
    if (st === 'COMPLETE') return s;
    if (st === 'FAILED') throw new Error(`${name}: processing FAILED ${JSON.stringify(s.attributes.assetDeliveryState.errors)}`);
    await sleep(2000);
  }
  throw new Error(`${name}: processing timeout`);
}

const rows = [];
for (const loc of LOCALES.filter((l) => !ONLY || l === ONLY)) {
  const vl = vlocs.find((l) => l.attributes.locale === loc);
  if (!vl) { log(`MISSING version localization ${loc}`); continue; }
  const sets = (await api(`/v1/appStoreVersionLocalizations/${vl.id}/appScreenshotSets`)).data;
  for (const [dev, [type, w, h]] of Object.entries(DEVICES)) {
    const dir = join(root, 'store/appstore/screenshots', SRC_DIR[loc], dev, 'portrait');
    const files = existsSync(dir) ? readdirSync(dir).filter((f) => f.endsWith('.png')).sort().map((f) => join(dir, f)) : [];
    let set = sets.find((s) => s.attributes.screenshotDisplayType === type);
    if (VERIFY) {
      const shots = set ? await listShots(set.id) : [];
      for (const s of shots) rows.push([loc, type, s.attributes.fileName, s.attributes.assetDeliveryState?.state, `${s.attributes.imageAsset?.width}x${s.attributes.imageAsset?.height}`]);
      if (!shots.length) rows.push([loc, type, '(none)', '-', '-']);
      continue;
    }
    if (!files.length) { log(`skip ${loc} ${type}: no files in ${dir}`); continue; }
    for (const f of files) {
      const png = readFileSync(f); // PNG IHDR: width/height at bytes 16..24
      if (png.readUInt32BE(16) !== w || png.readUInt32BE(20) !== h) throw new Error(`${f}: not ${w}x${h}`);
    }
    if (!set) {
      log(`create  set ${loc} ${type}`);
      set = (await api('/v1/appScreenshotSets', { method: 'POST', body: { data: { type: 'appScreenshotSets', attributes: { screenshotDisplayType: type },
        relationships: { appStoreVersionLocalization: { data: { type: 'appStoreVersionLocalizations', id: vl.id } } } } } }))?.data ?? { id: 'dry' };
    }
    const have = set.id === 'dry' ? [] : await listShots(set.id);
    const want = new Map(files.map((f) => [md5(readFileSync(f)), f]));
    const keep = new Map();
    for (const s of have) {
      const sum = s.attributes.sourceFileChecksum;
      if (want.has(sum) && s.attributes.assetDeliveryState?.state === 'COMPLETE' && !keep.has(sum)) keep.set(sum, s);
      else { log(`delete  ${loc} ${type} ${s.attributes.fileName}`); await api(`/v1/appScreenshots/${s.id}`, { method: 'DELETE' }); }
    }
    const ids = [];
    for (const [sum, f] of want) {
      let s = keep.get(sum);
      if (s) log(`ok      ${loc} ${type} ${f.split('/').pop()}`);
      else { log(`upload  ${loc} ${type} ${f.split('/').pop()}`); s = await upload(set.id, f); }
      ids.push([f.split('/').pop(), s?.id]);
    }
    ids.sort((a, b) => a[0].localeCompare(b[0]));
    if (!DRY) await api(`/v1/appScreenshotSets/${set.id}/relationships/appScreenshots`, { method: 'PATCH', body: { data: ids.map(([, id]) => ({ type: 'appScreenshots', id })) } });
  }
}
if (VERIFY) { console.log('locale | displayType | file | state | size'); for (const r of rows) console.log(r.join(' | ')); }
