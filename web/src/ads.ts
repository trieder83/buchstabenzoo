// External, signed ad content (GAME-ADS "External content", ADS-007…ADS-019, Q-240).
//
// The game accepts ONLY campaigns signed by us: `boards/index.json` + detached Ed25519
// signature `boards/index.sig` (over DOMAIN + the exact manifest bytes), public key compiled
// into the game (ad-keys.ts). Before anything is shown we check: signature, format, version
// (never lower than the last seen), validity window, per campaign: known id + slot + link host
// (hard-coded allowlist, https only), plain-text tagline, and per image: path, size, SHA-256,
// magic bytes and dimensions. ANY failure → that campaign (or the whole manifest) is dropped
// and the board keeps its local placeholder — never anything unsigned. Pure functions, no DOM.
import * as ed from '@noble/ed25519';
import { sha256 as nobleSha256, sha512 as nobleSha512 } from '@noble/hashes/sha2.js';
import { AD_PUBLIC_KEYS } from './ad-keys';
import { adsTelemetry, logAdError, logAdEvent, logBlocked } from './ads-debug';

/** Build-time switch (vite `define`): true only in the e2e test build (`VITE_AD_TEST=1`). */
declare const __AD_TEST__: boolean;

export const AD_FORMAT = 'buchstabenzoo-ads/1';
const DOMAIN = new TextEncoder().encode(`${AD_FORMAT}\n`);
export const MAX_IMAGE_BYTES = 512 * 1024;
export const MAX_DIM = 2048;
export const MIN_DIM = 64;
export const MAX_TAGLINE = 90;
export const MAX_CAMPAIGNS = 3;
/** Manifest + signature (2.7 KB): generous, because a phone on a weak mobile network needs seconds for the TLS handshake alone (ADS-039). */
export const FETCH_TIMEOUT_MS = 10000;
/** The images get longer: up to 3 x 512 KB over mobile data (ADS-029, user report 2026-10-03). */
export const IMAGE_TIMEOUT_MS = 15000;
/** Clock skew tolerated for `issued` (ms). */
const SKEW_MS = 24 * 3600 * 1000;
export const VERSION_KEY = 'zoo.ads.version';
const SCHEME = 'https:';

/** The campaigns compiled into the game (Q-241): id → slot and the ONLY host its link may use. */
export const KNOWN_CAMPAIGNS: Readonly<Record<string, { slot: number; host: string; path?: string; query?: string; noLink?: true }>> = {
  mathfighter: { slot: 1, host: 'mathfighter.rcms.ch' },
  abcsmash: { slot: 2, host: 'abcsmash.rcms.ch' },
  edugamegalaxy: { slot: 3, host: 'edugamegalaxy.rcms.ch' },
  // native app build only (boards-native/, PLAT-038): the developer's own App Store pages, id fixed here
  'mathfighter-ios': { slot: 1, host: 'apps.apple.com', path: '/app/id6760628828' },
  'abcsmash-ios': { slot: 2, host: 'apps.apple.com', path: '/app/id6790508038' },
  'mathfighter-ios-b': { slot: 3, host: 'apps.apple.com', path: '/app/id6760628828' },
  // Android native build only (boards-native-android/, PLAT-054): the developer's own Google Play pages, package fixed here
  'mathfighter-android': { slot: 1, host: 'play.google.com', path: '/store/apps/details', query: 'id=com.mathfighter.app' },
  'abcsmash-android': { slot: 2, host: 'play.google.com', path: '/store/apps/details', query: 'id=app.abcshooter.twa' },
  // credit poster: NO link at all (no click, no gate); the manifest entry must not carry `link` / `links`
  'credit-android': { slot: 3, host: '', noLink: true },
};

export type AdErrorCode =
  | 'no-keys'
  | 'signature'
  | 'format'
  | 'rollback'
  | 'expired'
  | 'not-yet-valid'
  | 'campaign'
  | 'link'
  | 'text'
  | 'image-size'
  | 'image-type'
  | 'image-hash'
  | 'image-dims'
  | 'network';

export class AdError extends Error {
  constructor(
    readonly code: AdErrorCode,
    detail = '',
  ) {
    super(detail ? `${code}: ${detail}` : code);
  }
}

export interface AdImage {
  lang: string;
  path: string;
  mime: string;
  bytes: number;
  width: number;
  height: number;
  sha256: string;
}

export interface AdCampaign {
  id: string;
  slot: number;
  active: boolean;
  /** '' for a link-less credit poster ({@link KNOWN_CAMPAIGNS} `noLink`). Canonical link (scheme + allowlisted host + "/"), built by us, not copied from the manifest. */
  link: string;
  /** Store links per platform (ADS rule 16); a missing entry falls back to {@link link}. */
  links: { ios?: string; android?: string };
  tagline: { de: string; en: string };
  images: AdImage[];
}

export interface AdManifest {
  version: number;
  issued: number;
  validUntil: number;
  campaigns: AdCampaign[];
  /** Campaigns dropped during validation: `[id, reason]` (tests, diagnostics). */
  dropped: [string, AdErrorCode][];
}

export interface VerifiedImage extends AdImage {
  data: Uint8Array;
}
export interface VerifiedCampaign extends Omit<AdCampaign, 'images'> {
  images: VerifiedImage[];
}
export interface AdContent {
  version: number;
  /** Verified campaigns by slot (1…3); a missing slot shows its placeholder. */
  bySlot: Map<number, VerifiedCampaign>;
  /** True if an active campaign of the manifest did not arrive (timeout / hash) — the host retries (ADS-039). */
  incomplete?: boolean;
}

// ------------------------------------------------------------------ small helpers

export function b64decode(s: string): Uint8Array | null {
  const t = s.trim();
  if (!/^[A-Za-z0-9+/]+={0,2}$/.test(t) || t.length % 4 !== 0) return null;
  try {
    const bin = atob(t);
    const out = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
    return out;
  } catch {
    return null;
  }
}

function concat(a: Uint8Array, b: Uint8Array): Uint8Array {
  const out = new Uint8Array(a.length + b.length);
  out.set(a, 0);
  out.set(b, a.length);
  return out;
}

export async function sha256Hex(data: Uint8Array): Promise<string> {
  const subtle = globalThis.crypto?.subtle;
  // Web Crypto only exists in secure contexts (https, localhost); on plain http (LAN test
  // server, phone on the Wi-Fi) the pure-JS SHA-256 gives the same result.
  let d: Uint8Array | null = null;
  if (subtle) {
    try {
      d = new Uint8Array(await subtle.digest('SHA-256', data as BufferSource));
    } catch (e) {
      logAdError(`subtle.digest failed, pure-JS SHA-256 used: ${String((e as Error)?.message ?? e)}`);
    }
  }
  if (!d) d = nobleSha256(data);
  return [...d].map((x) => x.toString(16).padStart(2, '0')).join('');
}

/** Strict ISO time `YYYY-MM-DDTHH:MM:SSZ` → ms since epoch, NaN if malformed. */
export function parseIso(s: unknown): number {
  if (typeof s !== 'string' || !/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$/.test(s)) return NaN;
  return Date.parse(s);
}

/**
 * Type, width and height from the magic bytes / header (PNG, WebP, JPEG); null for anything
 * else. Independent of the browser's decoder, so the declared metadata can be checked first.
 */
export function imageInfo(d: Uint8Array): { mime: string; width: number; height: number } | null {
  const u16be = (i: number) => (d[i] << 8) | d[i + 1];
  const u32be = (i: number) => ((d[i] << 24) | (d[i + 1] << 16) | (d[i + 2] << 8) | d[i + 3]) >>> 0;
  const ascii = (i: number, n: number) => String.fromCharCode(...d.subarray(i, i + n));
  if (d.length >= 24 && u32be(0) === 0x89504e47 && u32be(4) === 0x0d0a1a0a && ascii(12, 4) === 'IHDR') {
    return { mime: 'image/png', width: u32be(16), height: u32be(20) };
  }
  if (d.length >= 30 && ascii(0, 4) === 'RIFF' && ascii(8, 4) === 'WEBP') {
    const kind = ascii(12, 4);
    if (kind === 'VP8X') {
      const w = 1 + (d[24] | (d[25] << 8) | (d[26] << 16));
      const h = 1 + (d[27] | (d[28] << 8) | (d[29] << 16));
      return { mime: 'image/webp', width: w, height: h };
    }
    if (kind === 'VP8L' && d[20] === 0x2f) {
      const bits = (d[21] | (d[22] << 8) | (d[23] << 16) | (d[24] << 24)) >>> 0;
      return { mime: 'image/webp', width: (bits & 0x3fff) + 1, height: ((bits >>> 14) & 0x3fff) + 1 };
    }
    if (kind === 'VP8 ' && d[23] === 0x9d && d[24] === 0x01 && d[25] === 0x2a) {
      return { mime: 'image/webp', width: (d[26] | (d[27] << 8)) & 0x3fff, height: (d[28] | (d[29] << 8)) & 0x3fff };
    }
    return null;
  }
  if (d.length > 4 && d[0] === 0xff && d[1] === 0xd8) {
    let i = 2;
    while (i + 9 < d.length) {
      if (d[i] !== 0xff) {
        i += 1;
        continue;
      }
      const m = d[i + 1];
      if (m === 0xc0 || m === 0xc1 || m === 0xc2) return { mime: 'image/jpeg', width: u16be(i + 7), height: u16be(i + 5) };
      i += 2 + u16be(i + 2);
    }
  }
  return null;
}

// ------------------------------------------------------------------ rules per item

/**
 * The link of a known campaign: https only, exactly the allowlisted host, no user info, no
 * port, no query, no fragment, empty path. Returns the canonical URL we open (never the raw
 * manifest string) or null.
 */
export function checkLink(campaignId: string, link: unknown): string | null {
  const known = KNOWN_CAMPAIGNS[campaignId];
  if (!known || typeof link !== 'string' || link.length > 100) return null;
  let u: URL;
  try {
    u = new URL(link);
  } catch {
    return null;
  }
  const ok =
    u.protocol === SCHEME &&
    u.hostname === known.host &&
    u.username === '' &&
    u.password === '' &&
    u.port === '' &&
    (known.query ? u.search === `?${known.query}` : u.search === '') &&
    u.hash === '' &&
    (known.path ? u.pathname === known.path : u.pathname === '/' || u.pathname === '') &&
    (known.query ? link.indexOf('?') === link.lastIndexOf('?') : !link.includes('?')) &&
    !link.includes('#') &&
    !link.includes('@');
  return ok ? `${SCHEME}//${known.host}${known.path ?? '/'}${known.query ? `?${known.query}` : ''}` : null;
}

export const IOS_HOSTS: readonly string[] = ['apps.apple.com', 'itunes.apple.com'];
export const ANDROID_HOST = 'play.google.com';

/**
 * A store link (ADS rule 16): https, no user info / port / fragment, and
 * - `ios`: host `apps.apple.com` or `itunes.apple.com`, path `/app/id123…` or `/<cc>/app/<slug>/id123…`, no query;
 * - `android`: host `play.google.com`, path `/store/apps/details`, query exactly `id=<package name>`.
 * Returns the canonical URL we open (rebuilt, never the raw manifest string) or null.
 */
export function checkStoreLink(platform: 'ios' | 'android', link: unknown): string | null {
  if (typeof link !== 'string' || link.length > 160 || link.includes('@') || link.includes('#') || /[\s\\]/.test(link)) return null;
  let u: URL;
  try {
    u = new URL(link);
  } catch {
    return null;
  }
  if (u.protocol !== SCHEME || u.username !== '' || u.password !== '' || u.port !== '' || u.hash !== '') return null;
  if (platform === 'ios') {
    if (!IOS_HOSTS.includes(u.hostname) || u.search !== '' || link.includes('?')) return null;
    if (!/^\/(?:[a-z]{2}\/)?app\/(?:[a-z0-9-]{1,60}\/)?id\d{6,12}$/.test(u.pathname)) return null;
    return `${SCHEME}//${u.hostname}${u.pathname}`;
  }
  if (u.hostname !== ANDROID_HOST || u.pathname !== '/store/apps/details') return null;
  const m = /^\?id=([A-Za-z][A-Za-z0-9_]*(?:\.[A-Za-z0-9_]+)+)$/.exec(u.search);
  return m ? `${SCHEME}//${ANDROID_HOST}/store/apps/details?id=${m[1]}` : null;
}

/** The link to open for a platform: its store link, else the web link. */
export function linkFor(c: { link: string; links: { ios?: string; android?: string } }, platform: 'ios' | 'android' | 'web'): string {
  return (platform === 'web' ? undefined : c.links[platform]) ?? c.link;
}

/** Plain text of 1…MAX_TAGLINE characters without markup characters or control characters. */
export function checkTagline(text: unknown): string | null {
  if (typeof text !== 'string') return null;
  const t = text.trim();
  if (t.length < 1 || t.length > MAX_TAGLINE) return null;
  if (/[<>&"\\\u0000-\u001f\u007f-\u009f\u2028\u2029]/.test(t)) return null;
  return t;
}

const IMAGE_PATH = /^img\/[a-z0-9][a-z0-9._-]{0,63}$/;

function checkImageMeta(im: unknown): AdImage | null {
  if (typeof im !== 'object' || im === null) return null;
  const o = im as Record<string, unknown>;
  const { lang, path, mime, bytes, width, height, sha256 } = o;
  if (lang !== '*' && lang !== 'de' && lang !== 'en') return null;
  if (typeof path !== 'string' || !IMAGE_PATH.test(path) || path.includes('..')) return null;
  if (mime !== 'image/png' && mime !== 'image/webp' && mime !== 'image/jpeg') return null;
  if (!Number.isInteger(bytes) || (bytes as number) < 1 || (bytes as number) > MAX_IMAGE_BYTES) return null;
  for (const n of [width, height]) {
    if (!Number.isInteger(n) || (n as number) < MIN_DIM || (n as number) > MAX_DIM) return null;
  }
  if (typeof sha256 !== 'string' || !/^[0-9a-f]{64}$/.test(sha256)) return null;
  return { lang, path, mime, bytes: bytes as number, width: width as number, height: height as number, sha256 };
}

// ------------------------------------------------------------------ manifest

/** Verifies the detached signature against any of the trusted keys. */
export async function verifySignature(body: Uint8Array, sigB64: string, keys: readonly Uint8Array[]): Promise<boolean> {
  const sig = b64decode(sigB64);
  if (!sig || sig.length !== 64) return false;
  const msg = concat(DOMAIN, body);
  const secure = !!globalThis.crypto?.subtle;
  adsTelemetry.sig.verifier = secure ? 'subtle' : 'js';
  const tries: string[] = [];
  adsTelemetry.sig.tries = tries;
  ed.hashes.sha512 = nobleSha512; // the pure-JS path is always available (no Web Crypto, or Web Crypto misbehaves)
  for (const key of keys) {
    // 1st: Web Crypto SHA-512 (fast). If it throws or says "no" (older / odd Chromium, e.g. Samsung Internet), 2nd: pure JS.
    if (secure) {
      try {
        if (await ed.verifyAsync(sig, msg, key, { zip215: false })) {
          tries.push('subtle: ok');
          return true;
        }
        tries.push('subtle: false');
      } catch (e) {
        tries.push(`subtle: threw ${String((e as Error)?.message ?? e).slice(0, 80)}`);
      }
    }
    try {
      if (ed.verify(sig, msg, key, { zip215: false })) {
        tries.push('js: ok');
        adsTelemetry.sig.verifier = 'js';
        return true;
      }
      tries.push('js: false');
    } catch (e) {
      tries.push(`js: threw ${String((e as Error)?.message ?? e).slice(0, 80)}`);
    }
  }
  return false;
}

/**
 * Parses and validates an already signature-checked manifest. Top-level problems throw an
 * {@link AdError}; a bad campaign is dropped (listed in `dropped`).
 */
export function parseManifest(body: Uint8Array, nowMs: number, lastVersion: number): AdManifest {
  let json: unknown;
  try {
    json = JSON.parse(new TextDecoder('utf-8', { fatal: true }).decode(body));
  } catch {
    throw new AdError('format', 'not JSON');
  }
  if (typeof json !== 'object' || json === null) throw new AdError('format');
  const m = json as Record<string, unknown>;
  if (m.format !== AD_FORMAT) throw new AdError('format', 'format');
  const version = m.version;
  if (!Number.isInteger(version) || (version as number) < 1) throw new AdError('format', 'version');
  if ((version as number) < lastVersion) throw new AdError('rollback', `${String(version)} < ${lastVersion}`);
  const issued = parseIso(m.issued);
  const validUntil = parseIso(m.valid_until);
  if (Number.isNaN(issued) || Number.isNaN(validUntil) || validUntil <= issued) throw new AdError('format', 'dates');
  if (nowMs > validUntil) throw new AdError('expired');
  if (nowMs + SKEW_MS < issued) throw new AdError('not-yet-valid');
  if (!Array.isArray(m.campaigns) || m.campaigns.length > MAX_CAMPAIGNS) throw new AdError('format', 'campaigns');

  const out: AdManifest = { version: version as number, issued, validUntil, campaigns: [], dropped: [] };
  const slots = new Set<number>();
  for (const raw of m.campaigns as unknown[]) {
    const c = (typeof raw === 'object' && raw !== null ? raw : {}) as Record<string, unknown>;
    const id = typeof c.id === 'string' ? c.id : '?';
    const drop = (code: AdErrorCode) => out.dropped.push([id, code]);
    const known = KNOWN_CAMPAIGNS[id];
    if (!known || c.slot !== known.slot || slots.has(known.slot) || typeof c.active !== 'boolean') {
      drop('campaign');
      continue;
    }
    // a link-less credit poster must carry neither `link` nor `links`; every other campaign needs its exact link
    const link = known.noLink ? (c.link === undefined && c.links === undefined ? '' : null) : checkLink(id, c.link);
    if (link === null) {
      drop('link');
      continue;
    }
    // optional store links; an invalid one is dropped (that platform then opens the web link)
    const rawLinks = (typeof c.links === 'object' && c.links !== null ? c.links : {}) as Record<string, unknown>;
    const links: { ios?: string; android?: string } = {};
    const ios = rawLinks.ios === undefined ? null : checkStoreLink('ios', rawLinks.ios);
    const android = rawLinks.android === undefined ? null : checkStoreLink('android', rawLinks.android);
    if (ios) links.ios = ios;
    if (android) links.android = android;
    const tl = (c.tagline ?? {}) as Record<string, unknown>;
    const de = checkTagline(tl.de);
    const en = checkTagline(tl.en);
    if (!de || !en) {
      drop('text');
      continue;
    }
    const imgs = Array.isArray(c.images) ? c.images.map(checkImageMeta) : [];
    if (imgs.length < 1 || imgs.length > 4 || imgs.some((i) => i === null)) {
      drop('image-size');
      continue;
    }
    slots.add(known.slot);
    if (c.active) out.campaigns.push({ id, slot: known.slot, active: true, link, links, tagline: { de, en }, images: imgs as AdImage[] });
  }
  return out;
}

/** Checks the fetched bytes of one image against its signed metadata. */
export async function checkImageBytes(im: AdImage, data: Uint8Array): Promise<void> {
  if (data.length > MAX_IMAGE_BYTES || data.length !== im.bytes) throw new AdError('image-size', im.path);
  const info = imageInfo(data);
  if (!info || info.mime !== im.mime) throw new AdError('image-type', im.path);
  if (info.width !== im.width || info.height !== im.height) throw new AdError('image-dims', im.path);
  if ((await sha256Hex(data)) !== im.sha256) throw new AdError('image-hash', im.path);
}

// ------------------------------------------------------------------ keys

/** True only in the dev server and the e2e test build (vite `define`, Q-245). */
export const AD_TEST_BUILD: boolean = typeof __AD_TEST__ !== 'undefined' && __AD_TEST__;

/** The test public key given in the URL (base64), or null. Only ever called in a test build. */
export function testKeyParam(search: string): string | null {
  return new URLSearchParams(search).get('adkey');
}

/** The trusted public keys: the compiled ones, or (test build only) the given test key instead. */
export function resolveKeys(testKey: string | null = null, compiled: readonly string[] = AD_PUBLIC_KEYS): Uint8Array[] {
  const list: readonly string[] = testKey ? [testKey] : compiled;
  return list.map((k) => b64decode(k)).filter((k): k is Uint8Array => k !== null && k.length === 32);
}

// ------------------------------------------------------------------ loading

export interface KeyValue {
  getItem(k: string): string | null;
  setItem(k: string, v: string): void;
}

export interface LoadOptions {
  /** URL prefix of the ads directory, ending with `/` (same origin, e.g. `boards/`). */
  base: string;
  keys: readonly Uint8Array[];
  fetchFn: (url: string, init: { signal: AbortSignal; cache?: RequestCache; credentials?: RequestCredentials; referrerPolicy?: ReferrerPolicy }) => Promise<Response>;
  now: number;
  store: KeyValue | null;
  timeoutMs?: number;
}

async function fetchBytes(o: LoadOptions, path: string, signal: AbortSignal, max: number): Promise<Uint8Array> {
  let res: Response;
  try {
    res = await o.fetchFn(o.base + path, { signal, cache: 'no-cache', credentials: 'omit', referrerPolicy: 'no-referrer' });
  } catch (e) {
    // a TypeError ("Failed to fetch", net::ERR_BLOCKED_BY_CLIENT) that is not our own timeout: offline, or a
    // browser / ad blocker that filters the URL (ADS-044); the debug panel names the URL
    if (!signal.aborted && (e as Error)?.name === 'TypeError') logBlocked(`request ${o.base}${path}: ${String((e as Error).message)}`);
    throw new AdError('network', `${path} ${(e as Error).name ?? ''}`.trim());
  }
  if (!res.ok) throw new AdError('network', `${path} ${res.status}`);
  const len = Number(res.headers.get('content-length') ?? '0');
  if (len > max) throw new AdError('image-size', path);
  const buf = new Uint8Array(await res.arrayBuffer());
  if (buf.length > max) throw new AdError('image-size', path);
  return buf;
}

/** Runs `job(signal)` with a deadline that aborts the requests. */
async function withDeadline<T>(ms: number, job: (signal: AbortSignal) => Promise<T>): Promise<T> {
  const ctl = new AbortController();
  const timer = setTimeout(() => ctl.abort(), ms);
  try {
    return await Promise.race([
      job(ctl.signal),
      new Promise<never>((_, rej) => ctl.signal.addEventListener('abort', () => rej(new AdError('network', 'timeout')))),
    ]);
  } finally {
    clearTimeout(timer);
  }
}

/**
 * Loads and verifies the campaigns. Resolves with the verified content, or `null` when there is
 * nothing to show (no key, offline, timeout, bad signature, rollback, expired …): the boards
 * then keep their placeholders. Never throws. Without a key no request is made.
 */
export async function loadAds(o: LoadOptions): Promise<AdContent | null> {
  if (o.keys.length === 0) return null;
  const timeout = o.timeoutMs ?? FETCH_TIMEOUT_MS;
  const tel = adsTelemetry;
  const t0 = performance.now();
  tel.attempts += 1;
  tel.manifest = { state: 'loading' };
  tel.sig = { ok: null, verifier: null };
  tel.images = {};
  try {
    const { body, sig } = await withDeadline(timeout, async (signal) => {
      const [b, s] = await Promise.all([
        fetchBytes(o, 'index.json', signal, 64 * 1024),
        fetchBytes(o, 'index.sig', signal, 256),
      ]);
      return { body: b, sig: new TextDecoder().decode(s) };
    });
    tel.manifest = { state: 'loading', ms: Math.round(performance.now() - t0), bytes: body.length };
    const tv = performance.now();
    const verified = await verifySignature(body, sig, o.keys);
    tel.sig = { ok: verified, verifier: tel.sig.verifier, tries: tel.sig.tries, ms: Math.round(performance.now() - tv) };
    if (!verified) throw new AdError('signature');
    let last = 0;
    try {
      last = Number(o.store?.getItem(VERSION_KEY) ?? '0') || 0;
    } catch {
      last = 0;
    }
    const manifest = parseManifest(body, o.now, last);
    try {
      o.store?.setItem(VERSION_KEY, String(manifest.version));
    } catch {
      /* no storage: the rollback check then only works within the session */
    }
    tel.manifest = { ...tel.manifest, state: 'ok', version: manifest.version };
    const bySlot = new Map<number, VerifiedCampaign>();
    try {
      await withDeadline(o.timeoutMs ?? IMAGE_TIMEOUT_MS, async (signal) => {
        await Promise.all(
          manifest.campaigns.map(async (c) => {
            try {
              const images: VerifiedImage[] = [];
              for (const im of c.images) {
                const ti = performance.now();
                tel.images[im.path] = { state: 'loading' };
                try {
                  const data = await fetchBytes(o, im.path, signal, MAX_IMAGE_BYTES);
                  await checkImageBytes(im, data);
                  images.push({ ...im, data });
                  tel.images[im.path] = { state: 'ok', bytes: data.length, ms: Math.round(performance.now() - ti) };
                } catch (e) {
                  tel.images[im.path] = { state: 'failed', ms: Math.round(performance.now() - ti), error: String((e as Error).message ?? e) };
                  throw e;
                }
              }
              bySlot.set(c.slot, { ...c, images });
            } catch {
              /* this campaign keeps its placeholder */
            }
          }),
        );
      });
    } catch {
      /* deadline: the campaigns that arrived in time are shown, the others keep the placeholder */
      for (const im of Object.values(tel.images)) if (im.state === 'loading') Object.assign(im, { state: 'failed', error: 'timeout' });
    }
    const incomplete = bySlot.size < manifest.campaigns.length;
    return bySlot.size > 0 ? { version: manifest.version, bySlot, incomplete } : null;
  } catch (e) {
    const msg = String((e as Error).message ?? e);
    tel.manifest = { ...tel.manifest, state: 'failed', error: msg, ms: tel.manifest.ms ?? Math.round(performance.now() - t0) };
    logAdError(`load: ${msg}`);
    return null;
  } finally {
    logAdEvent(`load #${tel.attempts} ${tel.manifest.state}`);
  }
}

// ------------------------------------------------------------------ picking an image

/** The image of a campaign for a language: matching language first, else `*`; `n` alternates. */
export function pickImage(c: VerifiedCampaign, lang: string, n: number): VerifiedImage {
  const exact = c.images.filter((i) => i.lang === lang);
  const any = c.images.filter((i) => i.lang === '*');
  const list = exact.length > 0 ? exact : any.length > 0 ? any : c.images;
  return list[((n % list.length) + list.length) % list.length];
}

// ------------------------------------------------------------------ carousel (ADS-031)

/** Auto-advance interval of the all-done carousel (ms). */
export const CAROUSEL_MS = 4000;

export interface CarouselItem {
  campaign: VerifiedCampaign;
  image: VerifiedImage;
}

/** The verified campaigns in slot order, each with its image for `lang` (never placeholders). */
export function carouselItems(content: AdContent | null, lang: string): CarouselItem[] {
  if (!content) return [];
  return [...content.bySlot.entries()]
    .sort((a, b) => a[0] - b[0])
    .map(([, campaign]) => campaign)
    .filter((c) => c.images.length > 0)
    .map((campaign) => ({ campaign, image: pickImage(campaign, lang, 0) }));
}

/** Position + auto-advance timing of the carousel (pure, no DOM). */
export class Carousel {
  index = 0;
  private since = 0;

  constructor(
    readonly count: number,
    now = 0,
    readonly intervalMs = CAROUSEL_MS,
  ) {
    this.since = now;
  }

  /** Goes to slide `i` (wraps) and restarts the auto-advance timer. */
  go(i: number, now: number): void {
    this.index = this.count > 0 ? ((i % this.count) + this.count) % this.count : 0;
    this.since = now;
  }

  next(now: number): void {
    this.go(this.index + 1, now);
  }

  prev(now: number): void {
    this.go(this.index - 1, now);
  }

  /** Advances when the interval has passed; true if the slide changed. `paused` freezes the timer. */
  tick(now: number, paused = false): boolean {
    if (paused) {
      this.since = now;
      return false;
    }
    if (this.count < 2 || now - this.since < this.intervalMs) return false;
    this.next(now);
    return true;
  }
}

// ------------------------------------------------------------------ parental gate

export const GATE_HOLD_MS = 2000; // user request 2026-10-04: 2 s (was 3 s)

/** A plus or minus task the gate asks (numbers and result 0…20) and 4 answers. */
export interface GateQuestion {
  a: number;
  b: number;
  /** `+` (default) or `-`. */
  op?: '+' | '-';
  /** Language task (ABC Smash gate): the line to show; `options` are then indexes into `labels`. */
  prompt?: string;
  labels?: string[];
  options: number[];
  answer: number;
}

/** `rnd()` returns a float in [0, 1). */
export function makeGateQuestion(rnd: () => number = Math.random): GateQuestion {
  const int = (lo: number, hi: number) => lo + Math.floor(rnd() * (hi - lo + 1));
  const op: '+' | '-' = rnd() < 0.5 ? '+' : '-';
  let a = 0;
  let b = 0;
  if (op === '+') {
    a = int(4, 16);
    b = int(3, 20 - a);
  } else {
    a = int(9, 20);
    b = int(3, a - 2);
  }
  const answer = op === '+' ? a + b : a - b;
  const wrong = new Set<number>();
  while (wrong.size < 3) {
    const w = answer + [-1, 1, -2, 2, -3, 3, -10, 10][int(0, 7)];
    if (w !== answer && w >= 0 && w <= 20) wrong.add(w);
  }
  const options = [answer, ...wrong];
  for (let i = options.length - 1; i > 0; i--) {
    const j = int(0, i);
    [options[i], options[j]] = [options[j], options[i]];
  }
  return { a, b, op, options, answer };
}

/** German nouns with their article (the gate of the reading-game campaign asks it). */
export const GATE_NOUNS_DE: readonly (readonly [string, 'der' | 'die' | 'das'])[] = [
  ['Gabel', 'die'], ['Löffel', 'der'], ['Messer', 'das'], ['Tisch', 'der'], ['Lampe', 'die'], ['Haus', 'das'],
  ['Hund', 'der'], ['Katze', 'die'], ['Buch', 'das'], ['Ball', 'der'], ['Schule', 'die'], ['Auto', 'das'],
  ['Apfel', 'der'], ['Blume', 'die'], ['Fenster', 'das'], ['Stuhl', 'der'],
];

/** English plurals with 4 answers (answer first). */
export const GATE_PLURALS_EN: readonly (readonly [string, string, string, string, string])[] = [
  ['mouse', 'mice', 'mouses', 'mices', 'mouse'], ['child', 'children', 'childs', 'childes', 'child'],
  ['foot', 'feet', 'foots', 'feets', 'foot'], ['man', 'men', 'mans', 'manes', 'man'],
  ['tooth', 'teeth', 'tooths', 'teeths', 'tooth'], ['goose', 'geese', 'gooses', 'geeses', 'goose'],
];

/** The gate of a language campaign (ABC Smash): the right article (de) / plural (en). */
export function makeLanguageGateQuestion(lang: string, rnd: () => number = Math.random): GateQuestion {
  const int = (lo: number, hi: number) => lo + Math.floor(rnd() * (hi - lo + 1));
  if (lang === 'en') {
    const [one, right, w1, w2, w3] = GATE_PLURALS_EN[int(0, GATE_PLURALS_EN.length - 1)];
    const labels = [right, w1, w2, w3];
    const order = [0, 1, 2, 3];
    for (let i = 3; i > 0; i--) {
      const j = int(0, i);
      [order[i], order[j]] = [order[j], order[i]];
    }
    return { a: 0, b: 0, prompt: `one ${one}, two …?`, labels, options: order, answer: 0 };
  }
  const [noun, art] = GATE_NOUNS_DE[int(0, GATE_NOUNS_DE.length - 1)];
  const labels = ['der', 'die', 'das'];
  return { a: 0, b: 0, prompt: `… ${noun}`, labels, options: [0, 1, 2], answer: labels.indexOf(art) };
}

export type GateStage = 'sum' | 'hold' | 'failed' | 'open';

/**
 * The parental gate (ADS-018, Q-242): first the sum (a wrong answer ends the gate), then the
 * hand must stay on the button for {@link GATE_HOLD_MS} (releasing resets). `open` is reached
 * only after both; the host then opens the link once. Time is passed in (`now` in ms).
 */
export class ParentalGate {
  stage: GateStage = 'sum';
  private holdFrom: number | null = null;
  constructor(readonly question: GateQuestion = makeGateQuestion()) {}

  answer(choice: number): GateStage {
    if (this.stage !== 'sum') return this.stage;
    this.stage = choice === this.question.answer ? 'hold' : 'failed';
    return this.stage;
  }

  holdStart(now: number): void {
    if (this.stage === 'hold') this.holdFrom = now;
  }

  holdEnd(): void {
    this.holdFrom = null;
  }

  /** Progress 0…1 of the current hold; reaching 1 opens the gate. */
  progress(now: number): number {
    if (this.stage === 'open') return 1;
    if (this.stage !== 'hold' || this.holdFrom === null) return 0;
    const p = Math.min(1, (now - this.holdFrom) / GATE_HOLD_MS);
    if (p >= 1) this.stage = 'open';
    return p;
  }

  cancel(): void {
    this.stage = 'failed';
    this.holdFrom = null;
  }
}
