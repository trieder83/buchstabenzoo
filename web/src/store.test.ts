// App Store packaging tests (specs/40-tech/app-store.md): STORE-* and PLAT-034..041. Reads the repo files only.
import fs from 'node:fs';
import path from 'node:path';
import { describe, expect, it } from 'vitest';

const root = path.resolve(__dirname, '../..');
const read = (p: string) => fs.readFileSync(path.join(root, p), 'utf8');
const LOCALES = ['de-DE', 'en-GB', 'en-US'];
const LIMITS: Record<string, number> = {
  'name.txt': 30,
  'subtitle.txt': 30,
  'promotional_text.txt': 170,
  'description.txt': 4000,
  'keywords.txt': 100,
  'whats_new.txt': 4000,
};
const field = (loc: string, f: string) => read(`store/appstore/${loc}/${f}`).replace(/\n$/, '');

describe('App Store texts (STORE)', () => {
  it('STORE-001 every locale has every field within the App Store Connect limit', () => {
    for (const loc of LOCALES) {
      for (const [f, max] of Object.entries(LIMITS)) {
        const t = field(loc, f);
        expect(t.length, `${loc}/${f}`).toBeGreaterThan(0);
        expect(t.length, `${loc}/${f}`).toBeLessThanOrEqual(max);
      }
      expect(field(loc, 'keywords.txt')).not.toMatch(/, |^,|,$|,,/); // no wasted spaces / empty entries
      expect(field(loc, 'privacy_url.txt')).toBe('https://letterzoo.web.app/privacy.html');
    }
  });
  it('STORE-002 exactly the three published localisations exist (en-US, en-GB, de-DE)', () => {
    const dirs = fs.readdirSync(path.join(root, 'store/appstore'), { withFileTypes: true }).filter((e) => e.isDirectory() && e.name !== 'screenshots');
    expect(dirs.map((d) => d.name).sort()).toEqual(LOCALES);
  });
  it('STORE-003 no names of other platforms or stores, no donation or external-purchase wording (2.3.10, 3.1.1)', () => {
    const bad = /android|google play|itch\.io|windows|steam|spende|donat|paypal|ko-fi/i;
    for (const loc of LOCALES) {
      for (const f of Object.keys(LIMITS)) expect(field(loc, f), `${loc}/${f}`).not.toMatch(bad);
    }
    expect(read('store/appstore/review_notes.txt')).not.toMatch(bad);
  });
  it('STORE-004 the keywords do not repeat the name or subtitle words', () => {
    for (const loc of LOCALES) {
      const own = new Set(`${field(loc, 'name.txt')} ${field(loc, 'subtitle.txt')}`.toLowerCase().split(/[^a-zäöüß]+/).filter((w) => w.length > 2));
      for (const k of field(loc, 'keywords.txt').toLowerCase().split(',')) expect(own.has(k), `${loc}: ${k}`).toBe(false);
    }
  });
  it('STORE-005 review notes explain the posters and the parental gate (2.3.1, 1.3)', () => {
    const n = read('store/appstore/review_notes.txt');
    expect(n).toMatch(/parental gate/);
    expect(n).toMatch(/offline/);
    expect(n.length).toBeLessThanOrEqual(4000);
  });
});

describe('iOS packaging (PLAT-034..041)', () => {
  it('PLAT-034 Capacitor config: bundle id, name, bundled web dir, default scheme (saves survive updates)', () => {
    const c = read('web/capacitor.config.ts');
    expect(c).toMatch(/appId: 'ch\.rcms\.letterzoo'/);
    expect(c).toMatch(/appName: 'Letter Zoo'/);
    expect(c).toMatch(/webDir: 'dist'/);
    expect(c).not.toMatch(/server:\s*{[^}]*url/); // never load the game from a web URL (4.2)
    expect(c).not.toMatch(/iosScheme|hostname/);
  });
  it('PLAT-035 Info.plist: encryption exempt, no privacy permission keys, de/en, portrait + landscape, arm64', () => {
    const p = read('web/ios/App/App/Info.plist');
    expect(p).toMatch(/<key>ITSAppUsesNonExemptEncryption<\/key>\s*<false\/>/);
    expect(p).not.toMatch(/NS[A-Za-z]+UsageDescription/);
    expect(p).not.toMatch(/UIBackgroundModes/);
    expect(p).toMatch(/<key>CFBundleLocalizations<\/key>\s*<array>\s*<string>de<\/string>\s*<string>en<\/string>/);
    expect(p).toMatch(/UIInterfaceOrientationPortrait/);
    expect(p).toMatch(/UIInterfaceOrientationLandscapeLeft/);
    const pbx = read('web/ios/App/App.xcodeproj/project.pbxproj');
    expect(pbx).toMatch(/IPHONEOS_DEPLOYMENT_TARGET = 15\.0/); // WebGL2 in WKWebView needs iOS 15
    expect(pbx).toMatch(/PRODUCT_BUNDLE_IDENTIFIER = ch\.rcms\.letterzoo/);
  });
  it('PLAT-036 the native build has analytics off and no web-link ads: wired in main.ts', () => {
    const m = read('web/src/main.ts');
    expect(m).toMatch(/analyticsId\(MEASUREMENT_ID\)/);
    expect(m).toMatch(/adKeys\(/);
    expect(read('web/package.json')).toMatch(/"build:native": "VITE_NATIVE=1 npm run build"/);
  });
  it('PLAT-037 audio plays with the ringer switch on silent (playback session)', () => {
    expect(read('web/ios/App/App/AppDelegate.swift')).toMatch(/setCategory\(\.playback/);
  });
  it('PLAT-038 native poster template: only App Store product pages of the own apps, all three slots', () => {
    const t = JSON.parse(read('tools/ads/campaigns-native.template.json')) as { campaigns: { slot: number; link: string; images: { path: string }[] }[] };
    expect(t.campaigns.map((c) => c.slot).sort()).toEqual([1, 2, 3]);
    for (const c of t.campaigns) {
      expect(c.link).toMatch(/^https:\/\/apps\.apple\.com\/app\/id\d{9,10}$/);
      for (const i of c.images) expect(fs.existsSync(path.join(root, 'boards-native', i.path)), i.path).toBe(true);
    }
    // the signed manifest (once made) must contain no other link
    const man = path.join(root, 'boards-native/index.json');
    if (fs.existsSync(man)) for (const c of JSON.parse(fs.readFileSync(man, 'utf8')).campaigns) expect(c.link).toMatch(/^https:\/\/apps\.apple\.com\//);
  });
  it('PLAT-039 workflow: manual start, macOS, native build, signing + upload secrets by name only', () => {
    const w = read('.github/workflows/ios.yml');
    expect(w).toMatch(/workflow_dispatch:/);
    expect(w).not.toMatch(/^\s*push:/m);
    expect(w).toMatch(/runs-on: macos-/);
    expect(w).toMatch(/npm run build:native/);
    for (const s of ['IOS_DIST_CERT_P12', 'IOS_DIST_CERT_PASSWORD', 'IOS_PROVISIONING_PROFILE', 'ASC_KEY_ID', 'ASC_ISSUER_ID', 'ASC_KEY_P8']) {
      expect(w).toContain(`secrets.${s}`);
    }
    expect(w).not.toMatch(/-----BEGIN/);
  });
  it('PLAT-040 screenshot script knows the exact App Store sizes', () => {
    const s = read('tools/store/make_appstore_screenshots.py');
    for (const size of ['1320, 2868', '1284, 2778', '2064, 2752']) expect(s).toContain(size);
  });
  it('PLAT-041 App Store icon is 1024x1024 without alpha, splash exists', () => {
    const b = fs.readFileSync(path.join(root, 'web/ios/App/App/Assets.xcassets/AppIcon.appiconset/AppIcon-512@2x.png'));
    expect(b.readUInt32BE(16)).toBe(1024);
    expect(b.readUInt32BE(20)).toBe(1024);
    expect(b[25]).toBe(2); // PNG colour type 2 = RGB (6 = RGBA)
    expect(fs.existsSync(path.join(root, 'web/ios/App/App/Assets.xcassets/Splash.imageset/splash-2732x2732.png'))).toBe(true);
  });
  it('PLAT-042 the privacy page covers the iOS app and the rcms.ch alias', () => {
    const h = read('web/public/privacy.html');
    expect(h).toMatch(/iOS-App/);
    expect(h).toMatch(/iOS app/);
    expect(h).toContain('https://letterzoo.rcms.ch');
  });
});
