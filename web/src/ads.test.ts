// External signed ad content (GAME-ADS "External content"): every rule that makes a manifest
// or an image unacceptable leads to the placeholder, never to shown content. No network, no DOM.
import * as ed from '@noble/ed25519';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it, vi } from 'vitest';
import {
  AD_FORMAT,
  b64decode,
  Carousel,
  carouselItems,
  CAROUSEL_MS,
  type AdContent,
  type VerifiedCampaign,
  checkLink,
  checkTagline,
  GATE_HOLD_MS,
  imageInfo,
  KNOWN_CAMPAIGNS,
  loadAds,
  makeGateQuestion,
  ParentalGate,
  GATE_NOUNS_DE,
  GATE_PLURALS_EN,
  makeLanguageGateQuestion,
  parseManifest,
  pickImage,
  resolveKeys,
  testKeyParam,
  sha256Hex,
  verifySignature,
  VERSION_KEY,
  type LoadOptions,
} from './ads';

const here = path.dirname(fileURLToPath(import.meta.url));
const fixtures = path.resolve(here, '../tests/fixtures/ads');
const adsDir = path.join(fixtures, 'ads');
const seed = b64decode(fs.readFileSync(path.join(fixtures, 'TEST-ONLY-private.key'), 'utf8'))!;
const pub = b64decode(fs.readFileSync(path.join(fixtures, 'TEST-ONLY-public.key'), 'utf8'))!;
const DOMAIN = new TextEncoder().encode(`${AD_FORMAT}\n`);
const NOW = Date.parse('2026-10-01T12:00:00Z');

const b64 = (d: Uint8Array) => btoa(String.fromCharCode(...d));
const read = (rel: string) => new Uint8Array(fs.readFileSync(path.join(adsDir, rel)));
const cat = (a: Uint8Array, b: Uint8Array) => Uint8Array.from([...a, ...b]);

async function sign(body: Uint8Array, key = seed, domain = DOMAIN): Promise<string> {
  return b64(await ed.signAsync(cat(domain, body), key));
}

interface Img {
  lang: string;
  path: string;
  mime: string;
  bytes: number;
  width: number;
  height: number;
  sha256: string;
}
type Json = Record<string, unknown>;

/** The fixture manifest (signed by tools/ads/sign.py) as an editable object. */
function fixtureManifest(): { campaigns: { id: string; link: string; tagline: Json; images: Img[]; [k: string]: unknown }[]; [k: string]: unknown } {
  return JSON.parse(fs.readFileSync(path.join(adsDir, 'campaigns.json'), 'utf8'));
}

interface Served {
  files: Map<string, Uint8Array>;
  requests: string[];
}

/** A fake same-origin server: `ads/…` paths → bytes. */
function server(manifest: unknown, opts: { key?: Uint8Array; tamper?: Record<string, Uint8Array>; rawBody?: Uint8Array; sig?: string } = {}): Promise<Served> {
  const body = opts.rawBody ?? new TextEncoder().encode(JSON.stringify(manifest));
  return sign(body, opts.key ?? seed).then((sig) => {
    const files = new Map<string, Uint8Array>();
    files.set('campaigns.json', body);
    files.set('campaigns.sig', new TextEncoder().encode(opts.sig ?? sig));
    for (const f of fs.readdirSync(path.join(adsDir, 'img'))) files.set(`img/${f}`, read(`img/${f}`));
    for (const [k, v] of Object.entries(opts.tamper ?? {})) files.set(k, v);
    return { files, requests: [] };
  });
}

function options(s: Served, extra: Partial<LoadOptions> = {}): LoadOptions {
  const mem = new Map<string, string>();
  return {
    base: 'ads/',
    keys: [pub],
    now: NOW,
    store: { getItem: (k) => mem.get(k) ?? null, setItem: (k, v) => void mem.set(k, v) },
    fetchFn: async (url) => {
      s.requests.push(url);
      const f = s.files.get(url.replace(/^ads\//, ''));
      if (!f) return new Response(null, { status: 404 });
      return new Response(f as BufferSource, { status: 200 });
    },
    ...extra,
  };
}

describe('language gate of the reading campaign (ADS-025)', () => {
  it('ADS-025 German: a noun with its right article (Gabel → die); English: the right plural; always one right answer', () => {
    for (let i = 0; i < 100; i++) {
      const de = makeLanguageGateQuestion('de');
      expect(de.labels).toEqual(['der', 'die', 'das']);
      expect(de.options).toEqual([0, 1, 2]);
      const noun = /… (\S+)/.exec(de.prompt!)![1];
      const art = GATE_NOUNS_DE.find(([n]) => n === noun)![1];
      expect(de.labels![de.answer]).toBe(art);
      const en = makeLanguageGateQuestion('en');
      expect(new Set(en.options)).toEqual(new Set([0, 1, 2, 3]));
      expect(en.labels![en.answer]).toBeTruthy();
      const one = /one (\S+),/.exec(en.prompt!)![1];
      expect(GATE_PLURALS_EN.find(([s]) => s === one)![1]).toBe(en.labels![en.answer]);
    }
    expect(GATE_NOUNS_DE.find(([n]) => n === 'Gabel')![1]).toBe('die');
    const g = new ParentalGate(makeLanguageGateQuestion('de', () => 0)); // first noun
    expect(g.answer(g.question.answer)).toBe('hold');
  });
});

describe('no Web Crypto (plain http on the LAN, ADS-024)', () => {
  it('ADS-024 signature and SHA-256 give the same results with the pure-JS fallback', async () => {
    const body = read('campaigns.json');
    const sig = fs.readFileSync(path.join(adsDir, 'campaigns.sig'), 'utf8');
    const withSubtle = await sha256Hex(body);
    vi.stubGlobal('crypto', {}); // insecure context: no crypto.subtle
    try {
      expect(await verifySignature(body, sig, [pub])).toBe(true);
      const tampered = new Uint8Array(body);
      tampered[10] ^= 1;
      expect(await verifySignature(tampered, sig, [pub])).toBe(false);
      expect(await sha256Hex(body)).toBe(withSubtle);
    } finally {
      vi.unstubAllGlobals();
    }
  });
});

describe('signature (ADS-008, ADS-009)', () => {
  it('ADS-008 the manifest signed by tools/ads/sign.py verifies and parses', async () => {
    const body = read('campaigns.json');
    const sig = fs.readFileSync(path.join(adsDir, 'campaigns.sig'), 'utf8');
    expect(await verifySignature(body, sig, [pub])).toBe(true);
    const m = parseManifest(body, NOW, 0);
    expect(m.campaigns.map((c) => c.id)).toEqual(['mathfighter', 'abcsmash', 'edugamegalaxy']);
    expect(m.dropped).toEqual([]);
    expect(m.campaigns[0].link).toBe('https://mathfighter.rcms.ch/');
  });

  it('ADS-008 loadAds returns all three verified campaigns by slot', async () => {
    const s = await server(fixtureManifest());
    const c = await loadAds(options(s));
    expect([...c!.bySlot.keys()].sort()).toEqual([1, 2, 3]);
    expect(c!.bySlot.get(2)!.images.map((i) => i.lang)).toEqual(['de', 'en']);
  });

  it('ADS-009 a wrong key, a changed byte, a changed signature and a missing domain prefix are rejected', async () => {
    const body = read('campaigns.json');
    const sig = fs.readFileSync(path.join(adsDir, 'campaigns.sig'), 'utf8');
    const other = ed.utils.randomSecretKey();
    const otherPub = await ed.getPublicKeyAsync(other);
    expect(await verifySignature(body, sig, [otherPub])).toBe(false);
    const flipped = body.slice();
    flipped[flipped.length - 5] ^= 1;
    expect(await verifySignature(flipped, sig, [pub])).toBe(false);
    const badSig = b64decode(sig)!;
    badSig[10] ^= 1;
    expect(await verifySignature(body, b64(badSig), [pub])).toBe(false);
    expect(await verifySignature(body, await sign(body, seed, new Uint8Array()), [pub])).toBe(false);
    expect(await verifySignature(body, 'not base64!', [pub])).toBe(false);
    expect(await verifySignature(body, sig, [])).toBe(false);
  });

  it('ADS-009 a manifest signed with the wrong key yields no content', async () => {
    const s = await server(fixtureManifest(), { key: ed.utils.randomSecretKey() });
    expect(await loadAds(options(s))).toBeNull();
    expect(s.requests.some((r) => r.startsWith('ads/img/'))).toBe(false); // no image is even fetched
  });

  it('ADS-009 a rotated second key is accepted, an unknown one is not', async () => {
    const s = await server(fixtureManifest());
    const other = await ed.getPublicKeyAsync(ed.utils.randomSecretKey());
    expect(await loadAds(options(s, { keys: [other, pub] }))).not.toBeNull();
    expect(await loadAds(options(s, { keys: [other] }))).toBeNull();
  });
});

describe('manifest rules (ADS-010, ADS-011)', () => {
  it('ADS-010 a lower version than the last seen is refused, an equal one accepted', async () => {
    const s = await server(fixtureManifest()); // version 5
    const o = options(s);
    o.store!.setItem(VERSION_KEY, '6');
    expect(await loadAds(o)).toBeNull();
    o.store!.setItem(VERSION_KEY, '5');
    expect(await loadAds(o)).not.toBeNull();
  });

  it('ADS-010 the accepted version is remembered, a higher one raises the mark', async () => {
    const m = fixtureManifest();
    m.version = 9;
    const s = await server(m);
    const o = options(s);
    expect(await loadAds(o)).not.toBeNull();
    expect(o.store!.getItem(VERSION_KEY)).toBe('9');
  });

  it('ADS-010 a rejected (badly signed) manifest does not move the version mark', async () => {
    const m = fixtureManifest();
    m.version = 99;
    const s = await server(m, { key: ed.utils.randomSecretKey() });
    const o = options(s);
    await loadAds(o);
    expect(o.store!.getItem(VERSION_KEY)).toBeNull();
  });

  it('ADS-011 an expired manifest and one issued in the future are refused', () => {
    const body = read('campaigns.json');
    expect(() => parseManifest(body, Date.parse('2200-01-01T00:00:00Z'), 0)).toThrow(/expired/);
    expect(() => parseManifest(body, Date.parse('2025-06-01T00:00:00Z'), 0)).toThrow(/not-yet-valid/);
    expect(() => parseManifest(body, NOW, 6)).toThrow(/rollback/);
  });

  it('ADS-011 wrong format, malformed dates, more than 3 campaigns, garbage bodies are refused', () => {
    const enc = (o: unknown) => new TextEncoder().encode(JSON.stringify(o));
    const base = fixtureManifest();
    expect(() => parseManifest(enc({ ...base, format: 'x/2' }), NOW, 0)).toThrow(/format/);
    expect(() => parseManifest(enc({ ...base, issued: '2026-01-01' }), NOW, 0)).toThrow(/format/);
    expect(() => parseManifest(enc({ ...base, version: 0 }), NOW, 0)).toThrow(/format/);
    expect(() => parseManifest(enc({ ...base, campaigns: [1, 2, 3, 4] }), NOW, 0)).toThrow(/format/);
    expect(() => parseManifest(new TextEncoder().encode('{nope'), NOW, 0)).toThrow(/format/);
    expect(() => parseManifest(Uint8Array.from([0xff, 0xfe]), NOW, 0)).toThrow(/format/);
  });

  it('ADS-011 unknown campaigns, slot mismatch, inactive and duplicate slots never show', () => {
    const enc = (o: unknown) => new TextEncoder().encode(JSON.stringify(o));
    const m = fixtureManifest();
    const [mf, abc, edu] = m.campaigns;
    const res = parseManifest(
      enc({ ...m, campaigns: [{ ...mf, id: 'evil' }, { ...abc, slot: 1 }, { ...edu, slot: 2 }] }),
      NOW,
      0,
    );
    expect(res.campaigns).toEqual([]);
    expect(res.dropped.map((d) => d[0])).toEqual(['evil', 'abcsmash', 'edugamegalaxy']);
    const inactive = parseManifest(enc({ ...m, campaigns: [{ ...mf, active: false }] }), NOW, 0);
    expect(inactive.campaigns).toEqual([]);
    const dup = parseManifest(enc({ ...m, campaigns: [mf, { ...mf }] }), NOW, 0);
    expect(dup.campaigns).toHaveLength(1);
    expect(dup.dropped).toEqual([['mathfighter', 'campaign']]);
  });
});

describe('images (ADS-012, ADS-013, ADS-014)', () => {
  it('ADS-012 a tampered image (hash mismatch) drops its campaign only', async () => {
    const evil = read('img/mathfighter-wide.png').slice();
    evil[evil.length - 12] ^= 0xff; // same size, other pixels
    const s = await server(fixtureManifest(), { tamper: { 'img/mathfighter-wide.png': evil } });
    const c = await loadAds(options(s));
    expect([...c!.bySlot.keys()].sort()).toEqual([2, 3]);
  });

  it('ADS-012 every image of every campaign tampered → nothing to show', async () => {
    const tamper: Record<string, Uint8Array> = {};
    for (const f of fs.readdirSync(path.join(adsDir, 'img'))) {
      const d = read(`img/${f}`).slice();
      d[d.length - 12] ^= 1;
      tamper[`img/${f}`] = d;
    }
    const s = await server(fixtureManifest(), { tamper });
    expect(await loadAds(options(s))).toBeNull();
  });

  it('ADS-012 an image swapped for another valid one (other hash, same size class) is refused', async () => {
    const s = await server(fixtureManifest(), { tamper: { 'img/abcsmash-de.png': read('img/abcsmash-en.png') } });
    const c = await loadAds(options(s));
    expect(c!.bySlot.get(2)).toBeUndefined();
  });

  it('ADS-013 an image over 512 KB (declared or served) and out-of-range dimensions are refused', async () => {
    const m = fixtureManifest();
    m.campaigns[0].images[0].bytes = 600 * 1024;
    m.campaigns[1].images[0].width = 4096;
    m.campaigns[2].images[0].width = 4096;
    expect(parseManifest(new TextEncoder().encode(JSON.stringify(m)), NOW, 0).campaigns).toEqual([]);
    const big = new Uint8Array(600 * 1024);
    big.set(read('img/abcsmash-de.png'));
    const s = await server(fixtureManifest(), { tamper: { 'img/abcsmash-de.png': big } });
    const c = await loadAds(options(s));
    expect(c!.bySlot.get(2)).toBeUndefined();
  });

  it('ADS-013 declared dimensions that differ from the file header are refused', async () => {
    const m = fixtureManifest();
    for (const i of m.campaigns[1].images) i.height = 130;
    for (const i of m.campaigns[2].images) i.height = 130;
    const s = await server(m);
    const c = await loadAds(options(s));
    expect([...c!.bySlot.keys()]).toEqual([1]);
  });

  it('ADS-014 magic bytes must match the declared type; unknown types are refused', async () => {
    const m = fixtureManifest();
    for (const i of m.campaigns[0].images) i.mime = 'image/webp'; // the files are PNG
    m.campaigns[1].images[0].mime = 'image/gif';
    m.campaigns[2].images[0].mime = 'image/gif';
    const s = await server(m);
    const c = await loadAds(options(s));
    expect(c).toBeNull();
    expect(imageInfo(new TextEncoder().encode('<svg xmlns="x"></svg>'))).toBeNull();
    expect(imageInfo(new TextEncoder().encode('GIF89a-----------------------------'))).toBeNull();
  });

  it('ADS-014 image paths must stay in img/ (no traversal, no scheme, no absolute URL)', () => {
    const enc = (o: unknown) => new TextEncoder().encode(JSON.stringify(o));
    for (const bad of ['../secret.png', '/img/a.png', 'img/../a.png', 'https://evil.example/a.png', 'img/A.png', 'other/a.png', 'img/a b.png']) {
      const m = fixtureManifest();
      m.campaigns[0].images[0].path = bad;
      const r = parseManifest(enc(m), NOW, 0);
      expect(r.campaigns.map((c) => c.id), bad).toEqual(['abcsmash', 'edugamegalaxy']);
    }
  });

  it('reads the header of the shipped WebP and PNG files (same numbers as tools/ads/sign.py)', () => {
    const adsImg = path.resolve(here, '../../ads/img');
    for (const f of fs.readdirSync(adsImg)) {
      const info = imageInfo(new Uint8Array(fs.readFileSync(path.join(adsImg, f))));
      expect(info, f).not.toBeNull();
      expect(info!.mime).toBe('image/webp');
      expect(info!.width).toBe(1024);
    }
    expect(imageInfo(read('img/abcsmash-de.png'))).toEqual({ mime: 'image/png', width: 256, height: 128 });
  });

  it('sha256Hex matches the known digest of "abc"', async () => {
    expect(await sha256Hex(new TextEncoder().encode('abc'))).toBe('ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad');
  });
});

describe('links (ADS-015)', () => {
  it('ADS-015 only https and exactly the allowlisted host of that campaign pass', () => {
    const ok = (id: string, l: string) => checkLink(id, l);
    expect(ok('mathfighter', 'https://mathfighter.rcms.ch')).toBe('https://mathfighter.rcms.ch/');
    expect(ok('mathfighter', 'https://mathfighter.rcms.ch/')).toBe('https://mathfighter.rcms.ch/');
    expect(ok('abcsmash', 'https://abcsmash.rcms.ch')).toBe('https://abcsmash.rcms.ch/');
    expect(ok('edugamegalaxy', 'https://edugamegalaxy.rcms.ch')).toBe('https://edugamegalaxy.rcms.ch/');
    expect(ok('edugamegalaxy', 'https://edugamegalaxy.rcms.ch.evil.example')).toBeNull();
    expect(ok('edugamegalaxy', 'https://mathfighter.rcms.ch')).toBeNull();
    const bad: [string, string][] = [
      ['mathfighter', 'http://mathfighter.rcms.ch'],
      ['mathfighter', 'https://abcsmash.rcms.ch'], // the other campaign's host
      ['mathfighter', 'https://mathfighter.rcms.ch.evil.example'],
      ['mathfighter', 'https://evil.example/mathfighter.rcms.ch'],
      ['mathfighter', 'https://user@mathfighter.rcms.ch'],
      ['mathfighter', 'https://mathfighter.rcms.ch@evil.example'],
      ['mathfighter', 'https://mathfighter.rcms.ch:8443'],
      ['mathfighter', 'https://mathfighter.rcms.ch/?utm=1'],
      ['mathfighter', 'https://mathfighter.rcms.ch/#x'],
      ['mathfighter', 'https://mathfighter.rcms.ch/app'],
      ['mathfighter', 'https://MATHFIGHTER.rcms.ch.'],
      ['mathfighter', 'javascript:alert(1)'],
      ['mathfighter', 'data:text/html,x'],
      ['mathfighter', '//mathfighter.rcms.ch'],
      ['mathfighter', ''],
      ['evil', 'https://mathfighter.rcms.ch'],
    ];
    for (const [id, l] of bad) expect(ok(id, l), l).toBeNull();
    expect(checkLink('mathfighter', 42)).toBeNull();
    expect(Object.values(KNOWN_CAMPAIGNS).map((k) => k.host)).toEqual(['mathfighter.rcms.ch', 'abcsmash.rcms.ch', 'edugamegalaxy.rcms.ch']);
  });

  it('ADS-015 a manifest with a bad link drops that campaign', () => {
    const m = fixtureManifest();
    m.campaigns[0].link = 'https://mathfighter.rcms.ch/?ref=zoo';
    m.campaigns[1].link = 'https://evil.example';
    m.campaigns[2].link = 'https://edugamegalaxy.rcms.ch/?x=1';
    const r = parseManifest(new TextEncoder().encode(JSON.stringify(m)), NOW, 0);
    expect(r.campaigns).toEqual([]);
    expect(r.dropped.map((d) => d[1])).toEqual(['link', 'link', 'link']);
  });
});

describe('text (ADS-016)', () => {
  it('ADS-016 plain taglines pass, markup / control characters / long text do not', () => {
    expect(checkTagline('Lesen lernen – flüssig und schnell')).toBe('Lesen lernen – flüssig und schnell');
    for (const bad of ['<img src=x onerror=alert(1)>', 'a <b>b</b>', 'x & y', 'a\nb', 'a\u0000b', 'say "hi"', 'x'.repeat(81), '', '   ']) {
      expect(checkTagline(bad), JSON.stringify(bad)).toBeNull();
    }
    expect(checkTagline('x'.repeat(80))).not.toBeNull();
    expect(checkTagline(5)).toBeNull();
  });

  it('ADS-016 a campaign with HTML in a tagline is dropped', () => {
    const m = fixtureManifest();
    m.campaigns[0].tagline = { de: '<script>alert(1)</script>', en: 'ok' };
    const r = parseManifest(new TextEncoder().encode(JSON.stringify(m)), NOW, 0);
    expect(r.campaigns.map((c) => c.id)).toEqual(['abcsmash', 'edugamegalaxy']);
    expect(r.dropped).toEqual([['mathfighter', 'text']]);
  });
});

describe('loader (ADS-004, ADS-017)', () => {
  it('ADS-017 without a compiled key no request is made and nothing is shown', async () => {
    const s = await server(fixtureManifest());
    expect(await loadAds(options(s, { keys: [] }))).toBeNull();
    expect(s.requests).toEqual([]);
    expect(resolveKeys(null, [])).toEqual([]);
  });

  it('ADS-017 only the manifest, its signature and images of the same directory are requested', async () => {
    const s = await server(fixtureManifest());
    await loadAds(options(s));
    expect(s.requests.every((u) => u.startsWith('ads/') && !u.includes('//') && !u.includes('?'))).toBe(true);
    expect(s.requests).toContain('ads/campaigns.json');
    expect(s.requests).toContain('ads/campaigns.sig');
  });

  it('ADS-017 a missing file, an HTTP error or a hanging server ends in placeholders within the timeout', async () => {
    const s = await server(fixtureManifest());
    s.files.delete('campaigns.sig');
    expect(await loadAds(options(s))).toBeNull();
    const s2 = await server(fixtureManifest());
    const hang = options(s2, { timeoutMs: 60, fetchFn: (_u, init) => new Promise((_, rej) => init.signal.addEventListener('abort', () => rej(new Error('abort')))) });
    const t0 = Date.now();
    expect(await loadAds(hang)).toBeNull();
    expect(Date.now() - t0).toBeLessThan(1500);
    const off = options(s2, { fetchFn: () => Promise.reject(new TypeError('offline')) });
    expect(await loadAds(off)).toBeNull();
  });

  it('ADS-017 an image that never arrives drops only after the deadline, the others are kept', async () => {
    const s = await server(fixtureManifest());
    let slow = 0;
    const o = options(s, {
      timeoutMs: 200,
      fetchFn: async (url, init) => {
        if (url.includes('abcsmash-en')) {
          slow += 1;
          return new Promise((_, rej) => init.signal.addEventListener('abort', () => rej(new Error('abort'))));
        }
        return options(s).fetchFn(url, init);
      },
    });
    const c = await loadAds(o);
    expect(slow).toBe(1);
    expect(c === null || c.bySlot.get(2) === undefined).toBe(true);
  });

  it('ADS-017 picks the image of the language and alternates per board', async () => {
    const s = await server(fixtureManifest());
    const c = (await loadAds(options(s)))!;
    const mf = c.bySlot.get(1)!;
    const abc = c.bySlot.get(2)!;
    expect(pickImage(abc, 'de', 0).path).toContain('abcsmash-de');
    expect(pickImage(abc, 'en', 3).path).toContain('abcsmash-en');
    expect(pickImage(mf, 'de', 0).path).not.toBe(pickImage(mf, 'de', 1).path);
  });
});

describe('test key override (ADS-019)', () => {
  const compiled = [b64(pub)];
  it('ADS-019 the compiled keys are used unless a test key is given; bad keys are dropped', () => {
    const other = b64(new Uint8Array(32).fill(7));
    expect(resolveKeys(null, compiled)).toEqual([pub]);
    expect(resolveKeys(other, compiled)).toEqual([new Uint8Array(32).fill(7)]);
    expect(resolveKeys('zzz', compiled)).toEqual([]);
    expect(resolveKeys(null, [])).toEqual([]);
    expect(testKeyParam('?seed=1&adkey=abc')).toBe('abc');
    expect(testKeyParam('?seed=1')).toBeNull();
  });
});

describe('parental gate (ADS-018)', () => {
  it('ADS-018 the question is a plus or minus task up to 20 with 4 distinct answers', () => {
    const ops = new Set<string>();
    for (let i = 0; i < 300; i++) {
      const q = makeGateQuestion();
      ops.add(q.op ?? '+');
      expect(q.op === '-' ? q.a - q.b : q.a + q.b).toBe(q.answer);
      for (const n of [q.a, q.b, q.answer, ...q.options]) {
        expect(n).toBeGreaterThanOrEqual(0);
        expect(n).toBeLessThanOrEqual(20);
      }
      expect(new Set(q.options).size).toBe(4);
      expect(q.options).toContain(q.answer);
    }
    expect([...ops].sort()).toEqual(['+', '-']);
  });

  const gate = () => new ParentalGate({ a: 38, b: 47, options: [85, 75, 84, 95], answer: 85 });

  it('ADS-018 holding alone (without the right answer) never opens', () => {
    const g = gate();
    g.holdStart(0);
    expect(g.progress(10_000)).toBe(0);
    expect(g.stage).toBe('sum');
  });

  it('ADS-018 a wrong answer fails the gate for good', () => {
    const g = gate();
    expect(g.answer(75)).toBe('failed');
    g.holdStart(0);
    expect(g.progress(5000)).toBe(0);
    expect(g.answer(85)).toBe('failed');
  });

  it('ADS-018 the right answer then 2 s of holding opens; releasing early resets', () => {
    const g = gate();
    expect(g.answer(85)).toBe('hold');
    g.holdStart(1000);
    expect(g.progress(1000 + GATE_HOLD_MS - 1)).toBeLessThan(1);
    g.holdEnd();
    expect(g.progress(9000)).toBe(0);
    expect(g.stage).toBe('hold');
    g.holdStart(10_000);
    expect(g.progress(10_000 + GATE_HOLD_MS)).toBe(1);
    expect(g.stage).toBe('open');
  });

  it('ADS-018 cancelling ends the gate', () => {
    const g = gate();
    g.answer(85);
    g.cancel();
    g.holdStart(0);
    expect(g.progress(99_999)).toBe(0);
    expect(g.stage).toBe('failed');
  });
});

// ADS-031
describe('carousel', () => {
  const camp = (id: string, slot: number): VerifiedCampaign => ({
    id,
    slot,
    active: true,
    link: `https://${id}.rcms.ch/`,
    tagline: { de: 'a', en: 'b' },
    images: [
      { lang: 'de', path: `img/${id}-de.png`, mime: 'image/png', bytes: 1, width: 64, height: 64, sha256: '', data: new Uint8Array(1) },
      { lang: 'en', path: `img/${id}-en.png`, mime: 'image/png', bytes: 1, width: 64, height: 64, sha256: '', data: new Uint8Array(1) },
    ],
  });
  const content: AdContent = { version: 1, bySlot: new Map([[3, camp('c', 3)], [1, camp('a', 1)], [2, camp('b', 2)]]) };

  it('lists verified campaigns in slot order with the language image', () => {
    const it1 = carouselItems(content, 'en');
    expect(it1.map((i) => i.campaign.id)).toEqual(['a', 'b', 'c']);
    expect(it1[0].image.path).toBe('img/a-en.png');
    expect(carouselItems(content, 'de')[2].image.path).toBe('img/c-de.png');
    expect(carouselItems(null, 'de')).toEqual([]);
    expect(carouselItems({ version: 1, bySlot: new Map() }, 'de')).toEqual([]);
  });

  it('wraps, auto-advances after 4 s, restarts on manual moves and can be paused', () => {
    const c = new Carousel(3, 0);
    expect(CAROUSEL_MS).toBe(4000);
    expect(c.tick(3999)).toBe(false);
    expect(c.tick(4000)).toBe(true);
    expect(c.index).toBe(1);
    c.prev(4500);
    c.prev(4500);
    expect(c.index).toBe(2);
    expect(c.tick(8499)).toBe(false); // the manual move restarted the timer
    expect(c.tick(8500)).toBe(true);
    expect(c.index).toBe(0);
    c.go(5, 9000);
    expect(c.index).toBe(2);
    expect(c.tick(20000, true)).toBe(false); // paused (gate up)
    expect(c.tick(23999)).toBe(false);
    expect(c.tick(24000)).toBe(true);
    expect(new Carousel(1, 0).tick(99999)).toBe(false);
  });
});
