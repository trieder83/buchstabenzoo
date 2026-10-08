#!/usr/bin/env node
// Idempotent App Store Connect listing setup for Letter Zoo / Buchstabenzoo (spec: TECH-STORE).
//
//   ASC_KEY_ID=... ASC_ISSUER_ID=... ASC_KEY_P8_PATH=~/.appstoreconnect/AuthKey_X.p8 \
//     node tools/store/asc_listing.mjs [--dry-run] [--get /v1/path] [--verify]
//
// The JWT is signed locally; the key and the token are never printed. Nothing is ever submitted for review.
// Sources: store/appstore/<locale>/*.txt, store/appstore/review_notes.txt, store/appstore/README.md.
import { createSign } from 'node:crypto';
import { readFileSync, existsSync } from 'node:fs';
import { homedir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const args = process.argv.slice(2);
const DRY = args.includes('--dry-run');
const getIdx = args.indexOf('--get');
const VERIFY = args.includes('--verify');

const APP_ID = process.env.ASC_APP_ID || '6820559066';
const COPYRIGHT = '2026 Thomas Rieder';
const VERSION = '1.0';
const LOCALES = ['en-US', 'en-GB', 'de-DE'];
const TERRITORIES = ['USA', 'GBR', 'DEU', 'AUT', 'CHE'];
// Fallback names (store/appstore/README.md) tried in order when a name is taken.
const NAME_FALLBACK = {
  'en-US': ['Letter Zoo: Read & Rescue'], 'en-GB': ['Letter Zoo: Read & Rescue'], 'de-DE': ['Buchstabenzoo: Lesespiel'],
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

async function api(path, { method = 'GET', body, soft } = {}) {
  if (method !== 'GET' && DRY) { console.log(`[dry] ${method} ${path} ${JSON.stringify(body)?.slice(0, 300)}`); return null; }
  const res = await fetch(path.startsWith('http') ? path : API + path, {
    method, headers: { Authorization: `Bearer ${TOKEN}`, ...(body ? { 'Content-Type': 'application/json' } : {}) },
    body: body ? JSON.stringify(body) : undefined,
  });
  const text = await res.text();
  if (!res.ok) {
    if (soft) return { error: res.status, text };
    throw new Error(`${method} ${path} -> ${res.status}\n${text.slice(0, 1500)}`);
  }
  return text ? JSON.parse(text) : null;
}
const T = (f) => readFileSync(f, 'utf8').trim();
const txt = (loc, f) => T(join(root, 'store/appstore', loc, f));
const log = (...a) => console.log(...a);

if (getIdx >= 0) { log(JSON.stringify(await api(args[getIdx + 1]), null, 1)); process.exit(0); }


// ---------------------------------------------------------------------------------------------------------------
const rel = (type, id) => ({ data: { type, id } });
const first = async (path) => (await api(path))?.data;
const diff = (cur, want) => Object.fromEntries(Object.entries(want).filter(([k, v]) => cur?.[k] !== v));
const changes = [];

/** PATCH only the attributes that differ. */
async function patch(kind, path, id, cur, want, label) {
  const d = diff(cur, want);
  if (!Object.keys(d).length) { log(`ok      ${label}`); return; }
  log(`change  ${label}: ${Object.keys(d).join(', ')}`);
  changes.push(label);
  await api(`${path}/${id}`, { method: 'PATCH', body: { data: { type: kind, id, attributes: d } } });
}

// 0. app, appInfo, version -----------------------------------------------------------------------------------
const app = await first(`/v1/apps/${APP_ID}`);
if (app.attributes.bundleId !== 'ch.rcms.letterzoo' || app.attributes.primaryLocale !== 'de-DE') throw new Error('unexpected app record');
const appInfo = (await api(`/v1/apps/${APP_ID}/appInfos`)).data[0];
const versions = (await api(`/v1/apps/${APP_ID}/appStoreVersions?filter[versionString]=${VERSION}`)).data;
let version = versions[0];
if (!version) {
  log(`create  appStoreVersion ${VERSION}`);
  version = (await api('/v1/appStoreVersions', { method: 'POST', body: { data: { type: 'appStoreVersions',
    attributes: { platform: 'IOS', versionString: VERSION, releaseType: 'MANUAL', copyright: COPYRIGHT }, relationships: { app: rel('apps', APP_ID) } } } }))?.data;
}
if (!version) throw new Error('no version');

// 1. content rights (CC-BY animal sounds are third-party content we are licensed to use: CREDITS.md) ----------
await patch('apps', '/v1/apps', APP_ID, app.attributes, { contentRightsDeclaration: 'USES_THIRD_PARTY_CONTENT' }, 'app.contentRightsDeclaration');

// 2. appInfo localizations (name, subtitle, privacy URL) ---------------------------------------------------------
const infoLocs = (await api(`/v1/appInfos/${appInfo.id}/appInfoLocalizations`)).data;
for (const loc of LOCALES) {
  const want = { subtitle: txt(loc, 'subtitle.txt'), privacyPolicyUrl: txt(loc, 'privacy_url.txt') };
  let cur = infoLocs.find((l) => l.attributes.locale === loc);
  const all = [txt(loc, 'name.txt'), ...NAME_FALLBACK[loc]];
  // a name that is already set (possibly a fallback chosen earlier) is kept: no re-trying of the taken first choice
  const names = cur && all.includes(cur.attributes.name) ? all.slice(all.indexOf(cur.attributes.name)) : all;
  for (const [i, name] of names.entries()) {
    try {
      if (!cur) {
        log(`create  appInfoLocalization ${loc}`);
        const r = await api('/v1/appInfoLocalizations', { method: 'POST', body: { data: { type: 'appInfoLocalizations',
          attributes: { locale: loc, name, ...want }, relationships: { appInfo: rel('appInfos', appInfo.id) } } } });
        cur = r?.data ?? { id: 'dry', attributes: {} };
        changes.push(`appInfoLocalization ${loc}`);
      } else {
        await patch('appInfoLocalizations', '/v1/appInfoLocalizations', cur.id, cur.attributes, { name, ...want }, `appInfo ${loc}`);
      }
      if (i > 0) log(`NAME FALLBACK USED for ${loc}: "${name}"`);
      break;
    } catch (e) {
      if (i === names.length - 1 || !/name|already|unique|taken|409/i.test(String(e))) throw e;
      log(`name "${name}" rejected for ${loc} (${String(e).slice(0, 200).replace(/\n/g, ' ')}), trying fallback`);
    }
  }
}

// 3. categories + age rating --------------------------------------------------------------------------------------
{
  const want = {
    primaryCategory: 'GAMES', secondaryCategory: 'EDUCATION',
    primarySubcategoryOne: 'GAMES_WORD', primarySubcategoryTwo: 'GAMES_FAMILY',
  };
  const cur = {};
  for (const k of Object.keys(want)) cur[k] = (await api(`/v1/appInfos/${appInfo.id}/${k}`))?.data?.id ?? null;
  if (Object.keys(want).some((k) => cur[k] !== want[k])) {
    log('change  categories', JSON.stringify(want));
    changes.push('categories');
    const relationships = Object.fromEntries(Object.entries(want).map(([k, v]) => [k, rel('appCategories', v)]));
    await api(`/v1/appInfos/${appInfo.id}`, { method: 'PATCH', body: { data: { type: 'appInfos', id: appInfo.id, relationships } } });
  } else log('ok      categories');

  const age = (await api(`/v1/appInfos/${appInfo.id}/ageRatingDeclaration`)).data;
  const none = 'NONE';
  const ageWant = {
    advertising: false, alcoholTobaccoOrDrugUseOrReferences: none, contests: none, gambling: false, gamblingSimulated: none,
    gunsOrOtherWeapons: none, healthOrWellnessTopics: false, kidsAgeBand: 'SIX_TO_EIGHT', lootBox: false,
    medicalOrTreatmentInformation: none, messagingAndChat: false, parentalControls: false, profanityOrCrudeHumor: none,
    ageAssurance: false, sexualContentGraphicAndNudity: none, sexualContentOrNudity: none, socialMedia: false,
    socialMediaAgeRestricted: false, horrorOrFearThemes: none, matureOrSuggestiveThemes: none, unrestrictedWebAccess: false,
    userGeneratedContent: false, violenceCartoonOrFantasy: none, violenceRealisticProlongedGraphicOrSadistic: none, violenceRealistic: none,
  };
  await patch('ageRatingDeclarations', '/v1/ageRatingDeclarations', age.id, age.attributes, ageWant, 'ageRatingDeclaration');
}

// 4. version: copyright, release type, localizations ------------------------------------------------------------------
await patch('appStoreVersions', '/v1/appStoreVersions', version.id, version.attributes, { copyright: COPYRIGHT, releaseType: 'MANUAL' }, `version ${VERSION}`);
const vLocs = (await api(`/v1/appStoreVersions/${version.id}/appStoreVersionLocalizations`)).data;
for (const loc of LOCALES) {
  const want = {
    description: txt(loc, 'description.txt'), keywords: txt(loc, 'keywords.txt'), promotionalText: txt(loc, 'promotional_text.txt'),
    supportUrl: txt(loc, 'support_url.txt'), marketingUrl: txt(loc, 'marketing_url.txt'), // whatsNew is not allowed on a first version
  };
  const cur = vLocs.find((l) => l.attributes.locale === loc);
  if (!cur) {
    log(`create  appStoreVersionLocalization ${loc}`);
    changes.push(`versionLocalization ${loc}`);
    await api('/v1/appStoreVersionLocalizations', { method: 'POST', body: { data: { type: 'appStoreVersionLocalizations',
      attributes: { locale: loc, ...want }, relationships: { appStoreVersion: rel('appStoreVersions', version.id) } } } });
  } else await patch('appStoreVersionLocalizations', '/v1/appStoreVersionLocalizations', cur.id, cur.attributes, want, `versionLocalization ${loc}`);
}

// 5. review details (contact data is reused from the newest ABC Smash version; never printed) -----------------------
{
  const cur = (await api(`/v1/appStoreVersions/${version.id}/appStoreReviewDetail`)).data;
  const notes = T(join(root, 'store/appstore/review_notes.txt'));
  const abc = (await api('/v1/apps?filter[bundleId]=app.abcshooter.ios')).data[0];
  const abcV = (await api(`/v1/apps/${abc.id}/appStoreVersions?limit=1`)).data[0];
  const src = (await api(`/v1/appStoreVersions/${abcV.id}/appStoreReviewDetail`)).data?.attributes ?? {};
  const want = {
    contactFirstName: src.contactFirstName, contactLastName: src.contactLastName, contactPhone: src.contactPhone,
    contactEmail: src.contactEmail, demoAccountRequired: false, notes,
  };
  for (const k of Object.keys(want)) if (want[k] == null) delete want[k];
  if (!cur) {
    log('create  appStoreReviewDetail');
    changes.push('reviewDetail');
    await api('/v1/appStoreReviewDetails', { method: 'POST', body: { data: { type: 'appStoreReviewDetails', attributes: want,
      relationships: { appStoreVersion: rel('appStoreVersions', version.id) } } } });
  } else await patch('appStoreReviewDetails', '/v1/appStoreReviewDetails', cur.id, cur.attributes, want, 'reviewDetail');
}

// 6. price: free (base territory USA) --------------------------------------------------------------------------------
{
  const pts = (await api(`/v1/apps/${APP_ID}/appPricePoints?filter[territory]=USA&limit=5`)).data;
  const free = pts.find((p) => Number(p.attributes.customerPrice) === 0);
  if (!free) throw new Error('no free price point');
  const mp = await api(`/v1/appPriceSchedules/${APP_ID}/manualPrices?include=appPricePoint`, { soft: true });
  const isFree = mp && !mp.error && mp.data.length > 0 && mp.included?.every((i) => Number(i.attributes.customerPrice) === 0);
  if (isFree) log('ok      price free');
  else {
    log('change  price -> free');
    changes.push('price');
    await api('/v1/appPriceSchedules', { method: 'POST', body: {
      data: { type: 'appPriceSchedules', relationships: { app: rel('apps', APP_ID), baseTerritory: rel('territories', 'USA'),
        manualPrices: { data: [{ type: 'appPrices', id: '${p1}' }] } } },
      included: [{ type: 'appPrices', id: '${p1}', attributes: { startDate: null }, relationships: { appPricePoint: rel('appPricePoints', free.id) } }] } });
  }
}

// 7. availability v2: only the five territories ---------------------------------------------------------------------
{
  const cur = await api(`/v1/apps/${APP_ID}/appAvailabilityV2`, { soft: true });
  let ok = false;
  if (cur && !cur.error) {
    const ta = await api(`/v2/appAvailabilities/${cur.data.id}/territoryAvailabilities?limit=200&include=territory`);
    const avail = ta.data.filter((t) => t.attributes.available).map((t) => t.relationships.territory.data.id).sort();
    ok = cur.data.attributes.availableInNewTerritories === false && avail.join() === [...TERRITORIES].sort().join();
  }
  if (ok) log('ok      availability');
  else {
    log('change  availability ->', TERRITORIES.join(','));
    changes.push('availability');
    // every territory must be listed (available true only for ours), otherwise ASC answers 409 per missing territory
    const all = [];
    for (let url = '/v1/territories?limit=200'; url;) {
      const r = await api(url);
      all.push(...r.data.map((t) => t.id));
      url = r.links?.next ?? null;
    }
    await api('/v2/appAvailabilities', { method: 'POST', body: {
      data: { type: 'appAvailabilities', attributes: { availableInNewTerritories: false }, relationships: { app: rel('apps', APP_ID),
        territoryAvailabilities: { data: all.map((_, i) => ({ type: 'territoryAvailabilities', id: `\${t${i}}` })) } } },
      included: all.map((t, i) => ({ type: 'territoryAvailabilities', id: `\${t${i}}`, attributes: { available: TERRITORIES.includes(t) },
        relationships: { territory: rel('territories', t) } })) } });
  }
}

if (VERIFY) await verify();
async function verify() {
  const rows = [];
  const add = (f, v) => rows.push(`${f.padEnd(46)} ${String(v).replace(/\n/g, ' ').slice(0, 90)}`);
  const a = (await api(`/v1/apps/${APP_ID}`)).data.attributes;
  add('app.name / primaryLocale', `${a.name} / ${a.primaryLocale}`); add('app.bundleId / sku', `${a.bundleId} / ${a.sku}`);
  add('app.contentRightsDeclaration', a.contentRightsDeclaration);
  for (const l of (await api(`/v1/appInfos/${appInfo.id}/appInfoLocalizations`)).data) for (const k of ['name', 'subtitle', 'privacyPolicyUrl']) add(`appInfo[${l.attributes.locale}].${k}`, l.attributes[k]);
  for (const k of ['primaryCategory', 'secondaryCategory', 'primarySubcategoryOne', 'primarySubcategoryTwo']) add(`appInfo.${k}`, (await api(`/v1/appInfos/${appInfo.id}/${k}`))?.data?.id);
  const ag = (await api(`/v1/appInfos/${appInfo.id}/ageRatingDeclaration`)).data.attributes;
  add('ageRating.kidsAgeBand', ag.kidsAgeBand);
  add('ageRating (other fields all NONE/false)', Object.entries(ag).filter(([k, v]) => !/Override|grac|developer|kidsAgeBand/.test(k) && v !== 'NONE' && v !== false).map(([k, v]) => `${k}=${v}`).join(',') || 'yes');
  const ai = (await api(`/v1/appInfos/${appInfo.id}`)).data.attributes;
  add('appInfo.appStoreAgeRating', ai.appStoreAgeRating);
  const v = (await api(`/v1/appStoreVersions/${version.id}`)).data.attributes;
  add('version', `${v.versionString} ${v.appStoreState}`); add('version.copyright / releaseType', `${v.copyright} / ${v.releaseType}`);
  for (const l of (await api(`/v1/appStoreVersions/${version.id}/appStoreVersionLocalizations`)).data) {
    const t = l.attributes;
    add(`versionLoc[${t.locale}] description/keywords len`, `${t.description?.length} / ${t.keywords?.length}`);
    add(`versionLoc[${t.locale}] promo`, t.promotionalText); add(`versionLoc[${t.locale}] support / marketing`, `${t.supportUrl} ${t.marketingUrl}`);
  }
  const r = (await api(`/v1/appStoreVersions/${version.id}/appStoreReviewDetail`)).data?.attributes;
  add('review.contact set (name/phone/email)', r && [r.contactFirstName, r.contactPhone, r.contactEmail].every(Boolean));
  add('review.demoAccountRequired', r?.demoAccountRequired); add('review.notes len', r?.notes?.length);
  const mp = await api(`/v1/appPriceSchedules/${APP_ID}/manualPrices?include=appPricePoint`, { soft: true });
  add('price (manual, customerPrice)', mp.error ? mp.error : mp.included?.map((i) => i.attributes.customerPrice).join(',') || 'none');
  const av = await api(`/v1/apps/${APP_ID}/appAvailabilityV2`);
  const ta = await api(`/v2/appAvailabilities/${av.data.id}/territoryAvailabilities?limit=200&include=territory`);
  add('availability.availableInNewTerritories', av.data.attributes.availableInNewTerritories);
  add('availability territories', ta.data.filter((t) => t.attributes.available).map((t) => t.relationships.territory.data.id).join(','));
  log('\n' + rows.join('\n'));
}
log(`\n${DRY ? 'dry run: ' : ''}${changes.length} change(s): ${changes.join('; ') || 'none'}`);
