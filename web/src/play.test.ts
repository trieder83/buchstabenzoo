// Google Play packaging tests (specs/40-tech/play-store.md): PLAY-001..006. Reads the repo files only.
import { execSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { describe, expect, it } from 'vitest';

const root = path.resolve(__dirname, '../..');
const read = (p: string) => fs.readFileSync(path.join(root, p), 'utf8');
const exists = (p: string) => fs.existsSync(path.join(root, p));
const LOCALES = ['de-DE', 'en-US'];
const LIMITS: Record<string, number> = { 'title.txt': 30, 'short_description.txt': 80, 'full_description.txt': 4000 };
const field = (loc: string, f: string) => read(`store/play/${loc}/${f}`).replace(/\n$/, '');

describe('Google Play packaging (TECH-PLAY)', () => {
  it('PLAY-001 every Play text exists within the limit, the title has no emoji', () => {
    for (const loc of LOCALES) {
      for (const [f, max] of Object.entries(LIMITS)) {
        const t = field(loc, f);
        expect(t.length, `${loc}/${f}`).toBeGreaterThan(0);
        expect([...t].length, `${loc}/${f}`).toBeLessThanOrEqual(max);
      }
      expect(field(loc, 'title.txt')).not.toMatch(/\p{Extended_Pictographic}/u);
    }
  });
  it('PLAY-002 the Play texts name no other platform or store and no donation wording', () => {
    for (const loc of LOCALES) {
      for (const f of Object.keys(LIMITS)) {
        expect(field(loc, f), `${loc}/${f}`).not.toMatch(/app store|apple|ios|iphone|ipad|itch\.io|spenden|donat/i);
      }
    }
  });
  it('PLAY-003 the Android manifest asks for INTERNET only and has no AD_ID', () => {
    const p = 'web/android/app/src/main/AndroidManifest.xml';
    expect(exists(p), 'run `npx cap add android` in web/').toBe(true);
    const m = read(p);
    const perms = [...m.matchAll(/<uses-permission[^>]*android:name="([^"]+)"/g)].map((x) => x[1]);
    expect(perms).toEqual(['android.permission.INTERNET']);
    expect(m).not.toMatch(/AD_ID/);
    expect(read('web/android/app/build.gradle')).toMatch(/applicationId "ch\.rcms\.letterzoo"/);
  });
  it('PLAY-004 capacitor config: stable https origin and the shared app id', () => {
    const c = read('web/capacitor.config.ts');
    expect(c).toMatch(/appId: 'ch\.rcms\.letterzoo'/);
    expect(c).toMatch(/androidScheme: 'https'/);
  });
  it('PLAY-005 the Android workflow is manual, builds the bundle, takes the keystore from secrets and checks the links', () => {
    const w = read('.github/workflows/android.yml');
    expect(w).toMatch(/workflow_dispatch/);
    expect(w).not.toMatch(/\n\s+(push|pull_request):/);
    expect(w).toMatch(/bundleRelease/);
    expect(w).toMatch(/secrets\.ANDROID_KEYSTORE_B64/);
    expect(w).toMatch(/play\.google\.com\/store\/apps\/details/);
  });
  it('PLAY-006 no keystore is tracked and the guide names secrets only by name', () => {
    const tracked = execSync('git ls-files', { cwd: root, encoding: 'utf8' });
    expect(tracked).not.toMatch(/\.(jks|keystore)$/m);
    const g = read('BUILD_ANDROID.md');
    expect(g).toMatch(/ANDROID_KEYSTORE_B64/);
    expect(g).not.toMatch(/storePassword\s*=\s*\S+/);
  });
});
