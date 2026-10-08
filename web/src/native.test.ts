import { describe, expect, it } from 'vitest';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { AD_PUBLIC_KEYS } from './ad-keys';
import { parseManifest, resolveKeys, verifySignature } from './ads';
import { adKeys, analyticsId, NATIVE, NATIVE_ADS } from './native';
import { nativeBoardsDir } from '../vite.config';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

describe('native app build switches (PLAT-036)', () => {
  it('PLAT-036 the web build keeps the analytics id and the ad keys', () => {
    expect(NATIVE).toBe(false); // vitest runs without VITE_NATIVE
    expect(analyticsId('G-ABC', false)).toBe('G-ABC');
    const keys = [new Uint8Array(32)];
    expect(adKeys(keys, false)).toBe(keys);
  });
  it('PLAT-036 the native build has no analytics id and no ad keys', () => {
    expect(analyticsId('G-ABC', true)).toBe('');
    expect(adKeys([new Uint8Array(32)], true, false)).toEqual([]); // native ads switched off: no keys
    expect(NATIVE_ADS).toBe(true); // boards-native/ is signed (Q-392)
    const keys = [new Uint8Array(32)];
    expect(adKeys(keys, true, true)).toBe(keys); // with the own-apps manifest enabled
  });
});

describe('native posters (PLAT-038)', () => {
  const dir = path.join(root, 'boards-native');
  const body = new Uint8Array(fs.readFileSync(path.join(dir, 'index.json')));
  const sig = fs.readFileSync(path.join(dir, 'index.sig'), 'utf8');

  it('PLAT-038 the native manifest verifies with the production key and holds only the own App Store pages', async () => {
    expect(await verifySignature(body, sig, resolveKeys(null, AD_PUBLIC_KEYS))).toBe(true);
    const issued = Date.parse(JSON.parse(new TextDecoder().decode(body)).issued);
    const m = parseManifest(body, issued + 86_400_000, 0);
    expect(m.dropped).toEqual([]);
    expect(m.campaigns.map((c) => [c.slot, c.id, c.link])).toEqual([
      [1, 'mathfighter-ios', 'https://apps.apple.com/app/id6760628828'],
      [2, 'abcsmash-ios', 'https://apps.apple.com/app/id6790508038'],
      [3, 'mathfighter-ios-b', 'https://apps.apple.com/app/id6760628828'],
    ]);
    for (const c of m.campaigns) expect(c.links).toEqual({});
  });

  it('PLAT-038 a native campaign id with any other link (web, itch, Play) is dropped', () => {
    const j = JSON.parse(new TextDecoder().decode(body));
    const issued = Date.parse(j.issued) + 86_400_000;
    for (const bad of ['https://mathfighter.rcms.ch/', 'https://itch.io/x', 'https://play.google.com/store/apps/details?id=com.mathfighter.app', 'https://apps.apple.com/app/id1234567890', 'https://apps.apple.com/']) {
      const x = JSON.parse(JSON.stringify(j));
      x.campaigns[0].link = bad;
      expect(parseManifest(new TextEncoder().encode(JSON.stringify(x)), issued, 0).dropped, bad).toEqual([['mathfighter-ios', 'link']]);
    }
  });

  const dist = path.join(root, 'web', 'dist-native');
  it.skipIf(!fs.existsSync(path.join(dist, 'boards', 'index.json')))('PLAT-038 the native bundle ships the native manifest as boards/ and no web, itch or Play link', () => {
    expect(fs.readFileSync(path.join(dist, 'boards', 'index.json'), 'utf8')).toBe(fs.readFileSync(path.join(dir, 'index.json'), 'utf8'));
    const walk = (d: string): string[] => fs.readdirSync(d, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? walk(path.join(d, e.name)) : [path.join(d, e.name)]));
    for (const f of walk(path.join(dist, 'boards'))) {
      if (/\.(json|sig)$/.test(f)) expect(fs.readFileSync(f, 'utf8'), f).not.toMatch(/rcms\.ch|itch\.io|play\.google/);
    }
  });
});

describe('Android native posters (PLAT-054..057)', () => {
  const dir = path.join(root, 'boards-native-android');
  const body = new Uint8Array(fs.readFileSync(path.join(dir, 'index.json')));
  const sig = fs.readFileSync(path.join(dir, 'index.sig'), 'utf8');
  const PLAY = 'https://play.google.com/store/apps/details?id=';

  it('PLAT-054 the Android manifest verifies and holds only the own Google Play pages', async () => {
    expect(await verifySignature(body, sig, resolveKeys(null, AD_PUBLIC_KEYS))).toBe(true);
    const issued = Date.parse(JSON.parse(new TextDecoder().decode(body)).issued);
    const m = parseManifest(body, issued + 86_400_000, 0);
    expect(m.dropped).toEqual([]);
    expect(m.campaigns.map((c) => [c.slot, c.id, c.link])).toEqual([
      [1, 'mathfighter-android', `${PLAY}com.mathfighter.app`],
      [2, 'abcsmash-android', `${PLAY}app.abcshooter.twa`],
      [3, 'credit-android', ''], // credit poster: no link at all
    ]);
    for (const c of m.campaigns) expect(c.links).toEqual({});
    expect(m.campaigns[2].tagline).toEqual({
      de: 'Buchstabenzoo basiert auf den Ideen von Elena Rieder, entwickelt von Thomas Rieder',
      en: 'Letter Zoo is based on the ideas of Elena Rieder, engineered by Thomas Rieder',
    });
    // the manifest text has no "link" key for the credit poster (the CI grep over '"link"' lines sees Play links only)
    const raw = JSON.parse(new TextDecoder().decode(body)).campaigns;
    expect('link' in raw[2] || 'links' in raw[2]).toBe(false);
    expect(raw.filter((c: { link?: string }) => c.link).every((c: { link: string }) => c.link.startsWith(PLAY))).toBe(true);
  });

  it('PLAT-055 an Android campaign id with any other link (web, itch, App Store, other package, extra query) is dropped', () => {
    const j = JSON.parse(new TextDecoder().decode(body));
    const issued = Date.parse(j.issued) + 86_400_000;
    for (const bad of [
      'https://mathfighter.rcms.ch/',
      'https://itch.io/x',
      'https://apps.apple.com/app/id6760628828',
      `${PLAY}com.evil.app`,
      `${PLAY}com.mathfighter.app&hl=de`,
      'https://play.google.com/store/apps/details',
      'https://play.google.com/store/apps/details?id=com.mathfighter.app#x',
      'https://play.google.com/store/apps/dev?id=com.mathfighter.app',
    ]) {
      const x = JSON.parse(JSON.stringify(j));
      x.campaigns[0].link = bad;
      expect(parseManifest(new TextEncoder().encode(JSON.stringify(x)), issued, 0).dropped, bad).toEqual([['mathfighter-android', 'link']]);
    }
  });

  it('PLAT-055 the credit poster is link-less only: any link or links on it drops it, a missing link drops every other campaign', () => {
    const j = JSON.parse(new TextDecoder().decode(body));
    const issued = Date.parse(j.issued) + 86_400_000;
    const run = (x: unknown) => parseManifest(new TextEncoder().encode(JSON.stringify(x)), issued, 0).dropped;
    for (const extra of [{ link: `${PLAY}com.mathfighter.app` }, { link: 'https://itch.io/x' }, { link: '' }, { links: { android: `${PLAY}com.mathfighter.app` } }]) {
      const x = JSON.parse(JSON.stringify(j));
      Object.assign(x.campaigns[2], extra);
      expect(run(x), JSON.stringify(extra)).toEqual([['credit-android', 'link']]);
    }
    const y = JSON.parse(JSON.stringify(j));
    delete y.campaigns[0].link;
    expect(run(y)).toEqual([['mathfighter-android', 'link']]);
  });

  it('PLAT-056 the Android manifest has no itch, website, App Store or other-store wording; the iOS manifest has no Play link', () => {
    const text = new TextDecoder().decode(body);
    expect(text).not.toMatch(/rcms\.ch|itch\.io|apps\.apple|itunes|app store|\bios\b|iphone|ipad/i);
    expect(fs.readFileSync(path.join(root, 'boards-native', 'index.json'), 'utf8')).not.toMatch(/play\.google|android|itch\.io|rcms\.ch/i);
  });

  it('PLAT-057 the native build picks the manifest dir by VITE_NATIVE_PLATFORM (ios default, android, web none)', () => {
    expect(nativeBoardsDir({})).toBeNull();
    expect(nativeBoardsDir({ VITE_NATIVE: '1' })).toBe(path.join(root, 'boards-native'));
    expect(nativeBoardsDir({ VITE_NATIVE: '1', VITE_NATIVE_PLATFORM: 'ios' })).toBe(path.join(root, 'boards-native'));
    expect(nativeBoardsDir({ VITE_NATIVE: '1', VITE_NATIVE_PLATFORM: 'android' })).toBe(dir);
    expect(() => nativeBoardsDir({ VITE_NATIVE: '1', VITE_NATIVE_PLATFORM: 'xyz' })).toThrow();
    const pkg = JSON.parse(fs.readFileSync(path.join(root, 'web', 'package.json'), 'utf8')).scripts;
    expect(pkg['build:native']).toBe('VITE_NATIVE=1 npm run build');
    expect(pkg['build:native:android']).toMatch(/VITE_NATIVE=1 VITE_NATIVE_PLATFORM=android/);
  });
});
