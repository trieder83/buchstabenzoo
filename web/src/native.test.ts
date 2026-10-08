import { describe, expect, it } from 'vitest';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { AD_PUBLIC_KEYS } from './ad-keys';
import { parseManifest, resolveKeys, verifySignature } from './ads';
import { adKeys, analyticsId, NATIVE, NATIVE_ADS } from './native';

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
