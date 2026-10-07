// Firebase Hosting preview deployment (TECH-PLATFORMS "Preview deployment for playtests").
// Static checks only: reads firebase.json, the deploy script and the host sources — no network.
import { execFileSync } from 'node:child_process';
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import { listAds, listAssets } from '../vite.config';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const read = (rel: string) => fs.readFileSync(path.join(repoRoot, rel), 'utf8');

interface HeaderRule {
  source: string;
  headers: { key: string; value: string }[];
}
interface Hosting {
  public: string;
  headers: HeaderRule[];
  rewrites?: unknown[];
  redirects?: unknown[];
}
// PLAT-028: the CSP allowlist of the opt-in analytics (exactly this and nothing else)
const CONNECT_SRC = "connect-src 'self' https://*.google-analytics.com https://*.analytics.google.com https://www.googletagmanager.com";
const SCRIPT_SRC = "script-src 'self' 'wasm-unsafe-eval' https://www.googletagmanager.com";
const IMG_SRC = "img-src 'self' data: blob: https://*.google-analytics.com https://*.googletagmanager.com";
const hosting: Hosting = JSON.parse(read('firebase.json')).hosting;

/** Value of `key` set by the rule whose source is exactly `source`. */
function header(source: string, key: string): string | undefined {
  const rule = hosting.headers.find((r) => r.source === source);
  return rule?.headers.find((h) => h.key.toLowerCase() === key.toLowerCase())?.value;
}

function maxAge(value: string | undefined): number {
  const m = /max-age=(\d+)/.exec(value ?? '');
  return m ? Number(m[1]) : NaN;
}

describe('firebase.json', () => {
  it('PLAT-003 serves web/dist without rewrites or redirects', () => {
    expect(hosting.public).toBe('web/dist');
    expect(hosting.rewrites ?? []).toEqual([]);
    expect(hosting.redirects ?? []).toEqual([]);
  });

  it('PLAT-004 sets MIME types for .wasm and .glb', () => {
    expect(header('**/*.wasm', 'Content-Type')).toBe('application/wasm');
    expect(header('**/*.glb', 'Content-Type')).toBe('model/gltf-binary');
  });

  it('ASND-018 sets MIME types for .ogg and .m4a, CSP allows own audio only', () => {
    expect(header('**/*.ogg', 'Content-Type')).toBe('audio/ogg');
    expect(header('**/*.m4a', 'Content-Type')).toBe('audio/mp4');
    const csp = header('**', 'Content-Security-Policy') ?? '';
    const directive = (name: string) =>
      csp
        .split(';')
        .map((d) => d.trim())
        .find((d) => d.startsWith(name + ' '));
    expect(directive('media-src')).toMatch(/^media-src 'self'( data: blob:)?$/);
    expect(directive('connect-src')).toBe(CONNECT_SRC);
  });

  it('ASND-018 the build lists and serves the audio files', () => {
    const files = listAssets();
    const audio = files.filter((f) => f.startsWith('audio/'));
    expect(audio.some((f) => f.endsWith('.ogg'))).toBe(true);
    expect(audio.some((f) => f.endsWith('.m4a'))).toBe(true);
    expect(audio.every((f) => /\.(ogg|m4a)$/.test(f))).toBe(true); // CREDITS.md is not served
  });

  it('ASND-039 the cart and engine sounds are in the asset index, with a .ogg and a .m4a twin', () => {
    const audio = listAssets().filter((f) => f.startsWith('audio/'));
    for (const cue of ['cart/cart_board', 'cart/cart_bump', 'cart/key_pickup', 'cart/lock_ok', 'engine/cart_engine_path', 'engine/cart_engine_grass']) {
      expect(audio.some((f) => f.startsWith(`audio/${cue}_`) && f.endsWith('.ogg')), `${cue} .ogg`).toBe(true);
      expect(audio.some((f) => f.startsWith(`audio/${cue}_`) && f.endsWith('.m4a')), `${cue} .m4a`).toBe(true);
    }
  });

  it('PLAT-005 caches hashed files long, index.html never, assets briefly', () => {
    const bundle = header('bundle/**', 'Cache-Control');
    expect(bundle).toContain('immutable');
    expect(maxAge(bundle)).toBe(31536000);
    expect(header('/', 'Cache-Control')).toBe('no-cache');
    expect(header('**/*.html', 'Cache-Control')).toBe('no-cache');
    const assets = maxAge(header('assets/**', 'Cache-Control'));
    expect(assets).toBeGreaterThan(0);
    expect(assets).toBeLessThanOrEqual(3600);
  });
});

describe('no external network except the opt-in analytics', () => {
  it('PLAT-028 the CSP allowlist is exactly the analytics hosts', () => {
    const csp = header('**', 'Content-Security-Policy') ?? '';
    const directive = (name: string) =>
      csp
        .split(';')
        .map((d) => d.trim())
        .find((d) => d.startsWith(name + ' '));
    expect(directive('script-src')).toBe(SCRIPT_SRC);
    expect(directive('connect-src')).toBe(CONNECT_SRC);
    expect(directive('img-src')).toBe(IMG_SRC);
    for (const strict of ["default-src 'self'", "style-src 'self' 'unsafe-inline'", "media-src 'self' data: blob:", "font-src 'self'", "object-src 'none'", "base-uri 'self'", "form-action 'none'", "frame-ancestors 'none'"]) {
      expect(csp.split(';').map((d) => d.trim())).toContain(strict);
    }
    // every http(s) source of the whole policy is one of the analytics hosts
    const hosts = csp.match(/https:\/\/[^\s;]+/g) ?? [];
    expect(new Set(hosts)).toEqual(new Set(['https://www.googletagmanager.com', 'https://*.google-analytics.com', 'https://*.analytics.google.com', 'https://*.googletagmanager.com']));
  });

  it('PLAT-006 default-src is own files only', () => {
    const csp = header('**', 'Content-Security-Policy') ?? '';
    const directive = (name: string) =>
      csp
        .split(';')
        .map((d) => d.trim())
        .find((d) => d.startsWith(name + ' '));
    expect(directive('default-src')).toBe("default-src 'self'");
    expect(csp).not.toMatch(/http:/);
  });

  it('PLAT-006 host sources load no external URL or analytics SDK', () => {
    const srcDir = path.join(repoRoot, 'web', 'src');
    const files = ['web/index.html', 'web/package.json'].concat(
      fs
        .readdirSync(srcDir)
        .filter((f) => f.endsWith('.ts') && !f.endsWith('.test.ts'))
        .map((f) => `web/src/${f}`),
    );
    // the opt-in analytics module is the one place that names the Google script URL (PLAT-006/028)
    const analyticsFiles = ['web/src/analytics.ts', 'web/src/analytics-config.ts'];
    for (const f of files.filter((f) => !analyticsFiles.includes(f))) {
      const text = read(f);
      expect(text, f).not.toMatch(/https?:\/\/(?!www\.w3\.org\/)/);
      expect(text, f).not.toMatch(/firebase|gtag|googletagmanager|google-analytics/i);
    }
  });
});

describe('release build', () => {
  const dist = path.join(repoRoot, 'web', 'dist');
  const built = fs.existsSync(path.join(dist, 'index.html'));

  it.skipIf(!built)('PLAT-007 has the entry files and stays under 30 MB', () => {
    let bytes = 0;
    const wasm: string[] = [];
    const walk = (dir: string) => {
      for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
        const p = path.join(dir, e.name);
        if (e.isDirectory()) walk(p);
        else {
          bytes += fs.statSync(p).size;
          if (e.name.endsWith('.wasm')) wasm.push(p);
        }
      }
    };
    walk(dist);
    expect(wasm.length).toBeGreaterThan(0);
    expect(fs.existsSync(path.join(dist, 'assets', 'index.json'))).toBe(true);
    expect(bytes).toBeLessThanOrEqual(30 * 1024 * 1024);
  });
});

describe('deploy script', () => {
  it('PLAT-008 deploys to an expiring preview channel, never live', () => {
    const script = read('scripts/deploy-preview.sh');
    expect(script).toMatch(/firebase hosting:channel:deploy "\$channel" --expires "\$expires"/);
    const days = /expires="\$\{2:-(\d+)d\}"/.exec(script);
    expect(days).not.toBeNull();
    expect(Number(days![1])).toBeLessThanOrEqual(30);
    const commands = script
      .split('\n')
      .filter((l) => !l.trim().startsWith('#'))
      .join('\n');
    expect(commands).not.toMatch(/firebase deploy/);
  });
});

describe('ad content hosting (GAME-ADS "External content")', () => {
  const adsDir = path.join(repoRoot, 'ads');

  it('PLAT-010 ads/** is cached briefly so campaigns can change without an app update', () => {
    const age = maxAge(header('ads/**', 'Cache-Control'));
    expect(age).toBeGreaterThan(0);
    expect(age).toBeLessThanOrEqual(600);
    expect(header('ads/**', 'Cache-Control')).toContain('must-revalidate');
    expect(header('ads/**', 'Cache-Control')).not.toContain('immutable');
  });

  it('PLAT-010 the CSP stays strict: ads are same-origin (connect-src and img-src self)', () => {
    const csp = header('**', 'Content-Security-Policy') ?? '';
    const directive = (name: string) =>
      csp
        .split(';')
        .map((d) => d.trim())
        .find((d) => d.startsWith(name + ' '));
    // ads load only same-origin content; the CSP widening is the analytics allowlist only (PLAT-028)
    expect(directive('connect-src')).toBe(CONNECT_SRC);
    expect(directive('img-src')).toBe(IMG_SRC);
    expect(directive('form-action')).toBe("form-action 'none'");
    expect(csp).not.toMatch(/http:/);
    const ads = read('web/src/ads.ts') + read('web/src/ads-ui.ts');
    expect(ads).not.toMatch(/https?:\/\/(?!www\.w3\.org\/)/);
  });

  it('PLAT-011 the served ads are the manifest, its signature and images of at most 512 KB', () => {
    const files = listAds();
    expect(files.every((f) => f === 'campaigns.json' || f === 'campaigns.sig' || /^img\/[a-z0-9._-]+\.(png|webp|jpe?g)$/.test(f))).toBe(true);
    const images = files.filter((f) => f.startsWith('img/'));
    expect(images.length).toBeGreaterThanOrEqual(4); // ADC1-001, ADC2-001
    for (const f of images) expect(fs.statSync(path.join(adsDir, f)).size, f).toBeLessThanOrEqual(512 * 1024);
    // no template, key or note is published
    expect(files.some((f) => /template|\.key|\.pem|README/i.test(f))).toBe(false);
  });

  it('PLAT-011 a shipped manifest has its signature and matches its images (size, SHA-256)', () => {
    const manifest = path.join(adsDir, 'campaigns.json');
    if (!fs.existsSync(manifest)) return; // no production manifest until the owner signed one
    expect(fs.existsSync(path.join(adsDir, 'campaigns.sig'))).toBe(true);
    const m = JSON.parse(fs.readFileSync(manifest, 'utf8')) as { campaigns: { images: { path: string; bytes: number; sha256: string }[] }[] };
    for (const c of m.campaigns) {
      for (const im of c.images) {
        const data = fs.readFileSync(path.join(adsDir, im.path));
        expect(data.length, im.path).toBe(im.bytes);
        expect(crypto.createHash('sha256').update(data).digest('hex'), im.path).toBe(im.sha256);
      }
    }
  });

  it('PLAT-011 no private key is tracked except the TEST-ONLY fixture key', () => {
    let out = '';
    try {
      out = execFileSync('git', ['ls-files', '*.key', '*.pem'], { cwd: repoRoot, encoding: 'utf8' });
    } catch {
      return; // not a git checkout
    }
    const tracked = out.split('\n').filter(Boolean);
    expect(tracked.filter((f) => f !== 'web/tests/fixtures/ads/TEST-ONLY-private.key' && f !== 'web/tests/fixtures/ads/TEST-ONLY-public.key')).toEqual([]);
  });

  it('PLAT-012 the release has no test-key override: the compiled keys never contain the test key', () => {
    const keys = read('web/src/ad-keys.ts');
    const testPub = read('web/tests/fixtures/ads/TEST-ONLY-public.key').trim();
    expect(keys).not.toContain(testPub);
    const pkg = JSON.parse(read('web/package.json')) as { scripts: Record<string, string> };
    for (const name of ['build', 'wasm', 'preview', 'deploy:preview']) expect(pkg.scripts[name] ?? '').not.toMatch(/VITE_AD_TEST/);
    const cfg = read('web/vite.config.ts');
    expect(cfg).toMatch(/__AD_TEST__: JSON\.stringify\(command === 'serve' \|\| process\.env\.VITE_AD_TEST === '1'\)/);
    // the only place that reads ?adkey= sits behind the build-time switch
    const ads = read('web/src/ads.ts');
    expect(ads.match(/adkey/g)?.length).toBe(1);
    expect(read('web/src/main.ts')).toMatch(/AD_TEST_BUILD \? testKeyParam/);
  });

  it.skipIf(!fs.existsSync(path.join(repoRoot, 'web', 'dist', 'index.html')))('PLAT-012 a release bundle in web/dist has no ?adkey= code path', () => {
    const dir = path.join(repoRoot, 'web', 'dist', 'bundle');
    for (const f of fs.readdirSync(dir).filter((n) => n.endsWith('.js'))) {
      expect(fs.readFileSync(path.join(dir, f), 'utf8'), f).not.toContain('adkey');
    }
  });
});

// TECH-PLATFORMS "Installable and full screen": manifest, icons, meta tags (PLAT-013..016, 019).
describe('installable web app', () => {
  const manifest = JSON.parse(read('web/public/manifest.webmanifest'));
  const pngSize = (rel: string) => {
    const b = fs.readFileSync(path.join(repoRoot, 'web/public', rel));
    expect(b.subarray(1, 4).toString()).toBe('PNG');
    return { w: b.readUInt32BE(16), h: b.readUInt32BE(20), bytes: b.length };
  };

  it('PLAT-013 manifest fields', () => {
    expect(manifest.name).toBe('Buchstabenzoo');
    expect(manifest.short_name).toBe('Zoo');
    expect(manifest.lang).toBe('de');
    expect(manifest.start_url).toBe('./');
    expect(manifest.scope).toBe('./');
    expect(manifest.display_override).toEqual(['fullscreen', 'standalone']);
    expect(manifest.display).toBe('fullscreen');
    expect(manifest.orientation).toBe('any');
    expect(manifest.background_color).toBe('#fff3d6');
    expect(manifest.theme_color).toBe('#3b2314');
  });

  it('PLAT-014 icons exist with the declared sizes and stay small', () => {
    const icons = manifest.icons as { src: string; sizes: string; purpose?: string }[];
    expect(icons.some((i) => i.sizes === '192x192')).toBe(true);
    expect(icons.some((i) => i.sizes === '512x512' && i.purpose === 'any')).toBe(true);
    expect(icons.some((i) => i.sizes === '512x512' && i.purpose === 'maskable')).toBe(true);
    for (const i of icons) {
      const [w, h] = i.sizes.split('x').map(Number);
      const p = pngSize(i.src);
      expect([p.w, p.h], i.src).toEqual([w, h]);
      expect(p.bytes, i.src).toBeLessThanOrEqual(60 * 1024);
    }
    expect(pngSize('icons/apple-touch-icon.png')).toMatchObject({ w: 180, h: 180 });
    expect(pngSize('icons/favicon-32.png')).toMatchObject({ w: 32, h: 32 });
  });

  it('PLAT-015 firebase.json serves the manifest with its MIME type, no-cache, strict CSP', () => {
    expect(header('manifest.webmanifest', 'Content-Type')).toBe('application/manifest+json');
    expect(header('manifest.webmanifest', 'Cache-Control')).toBe('no-cache');
    const csp = header('**', 'Content-Security-Policy') ?? '';
    expect(csp).toContain("default-src 'self'");
    expect(csp).not.toContain('manifest-src');
    expect(csp).toContain("connect-src 'self'");
  });

  it('PLAT-016 index.html links the manifest and icons, sets the web-app meta tags, no service worker', () => {
    const html = read('web/index.html');
    expect(html).toContain('<link rel="manifest" href="manifest.webmanifest"');
    expect(html).toMatch(/<link rel="apple-touch-icon"[^>]*href="icons\/apple-touch-icon\.png"/);
    expect(html).toMatch(/name="apple-mobile-web-app-capable" content="yes"/);
    expect(html).toMatch(/name="mobile-web-app-capable" content="yes"/);
    expect(html).toMatch(/name="apple-mobile-web-app-status-bar-style" content="black-translucent"/);
    expect(html).toMatch(/name="apple-mobile-web-app-title"/);
    expect(html).toMatch(/name="theme-color"/);
    expect(html).toContain('viewport-fit=cover');
    for (const f of fs.readdirSync(path.join(repoRoot, 'web/src')).filter((n) => n.endsWith('.ts') && !n.endsWith('.test.ts'))) {
      expect(read(`web/src/${f}`), f).not.toMatch(/serviceWorker/);
    }
  });

  it('PLAT-019 the full-screen texts exist in de and en', () => {
    for (const lang of ['de', 'en']) {
      const ui = read(`assets/i18n/${lang}/ui.ftl`);
      expect(ui, lang).toMatch(/^ui-fullscreen = .+/m);
      expect(ui, lang).toMatch(/^ui-install-hint-ios = .+/m);
    }
  });
});

// Opt-in analytics (TECH-PLATFORMS "Analytics (opt-in)", PLAT-029, PLAT-032).
describe('analytics configuration', () => {
  it('PLAT-029 the id is empty or a G- id; the test id only comes from the build define', () => {
    const cfg = read('web/src/analytics-config.ts');
    const m = /export const ANALYTICS_MEASUREMENT_ID = '([^']*)';/.exec(cfg);
    expect(m).not.toBeNull();
    expect(m![1]).toMatch(/^(G-[A-Z0-9]{6,12})?$/);
    expect(m![1]).not.toMatch(/TEST/);
    const pkg = JSON.parse(read('web/package.json')) as { scripts: Record<string, string> };
    for (const [name, cmd] of Object.entries(pkg.scripts)) expect(cmd, name).not.toMatch(/VITE_ANALYTICS_TEST_ID/);
    expect(read('web/vite.config.ts')).toContain('process.env.VITE_ANALYTICS_TEST_ID');
    for (const f of ['scripts/deploy-preview.sh']) expect(read(f)).not.toMatch(/VITE_ANALYTICS_TEST_ID/);
  });

  it.skipIf(!fs.existsSync(path.join(repoRoot, 'web', 'dist', 'index.html')))('PLAT-029 a release bundle in web/dist has no test id', () => {
    const dir = path.join(repoRoot, 'web', 'dist', 'bundle');
    for (const f of fs.readdirSync(dir).filter((n) => n.endsWith('.js'))) {
      expect(fs.readFileSync(path.join(dir, f), 'utf8'), f).not.toMatch(/G-TEST/);
    }
  });

  it('PLAT-032 the consent texts exist in de and en, the privacy page has both languages', () => {
    for (const lang of ['de', 'en']) {
      const ui = read(`assets/i18n/${lang}/ui.ftl`);
      for (const k of ['ui-analytics', 'analytics-title', 'analytics-note', 'analytics-detail', 'analytics-allow', 'analytics-deny']) {
        expect(ui, `${lang} ${k}`).toMatch(new RegExp(`^${k} = .+`, 'm'));
      }
    }
    expect(read('assets/i18n/de/ui.ftl')).toContain('Es werden keine Namen, keine Texte und keine persönlichen Daten gespeichert.');
    const page = read('web/public/privacy.html');
    expect(page).toMatch(/lang="de"/);
    expect(page).toMatch(/lang="en"/);
    for (const w of ['Eltern', 'parent', '📊', '2 Monate', '2 months', 'Verantwortliche', 'controller']) expect(page, w).toContain(w);
    expect(page).not.toMatch(/<script/i);
  });
});
