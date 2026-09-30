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
    expect(directive('connect-src')).toBe("connect-src 'self'");
  });

  it('ASND-018 the build lists and serves the audio files', () => {
    const files = listAssets();
    const audio = files.filter((f) => f.startsWith('audio/'));
    expect(audio.some((f) => f.endsWith('.ogg'))).toBe(true);
    expect(audio.some((f) => f.endsWith('.m4a'))).toBe(true);
    expect(audio.every((f) => /\.(ogg|m4a)$/.test(f))).toBe(true); // CREDITS.md is not served
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

describe('no tracking, no external network', () => {
  it('PLAT-006 CSP allows only own files', () => {
    const csp = header('**', 'Content-Security-Policy') ?? '';
    const directive = (name: string) =>
      csp
        .split(';')
        .map((d) => d.trim())
        .find((d) => d.startsWith(name + ' '));
    expect(directive('default-src')).toBe("default-src 'self'");
    expect(directive('connect-src')).toBe("connect-src 'self'");
    expect(csp).not.toMatch(/https?:/);
  });

  it('PLAT-006 host sources load no external URL or analytics SDK', () => {
    const srcDir = path.join(repoRoot, 'web', 'src');
    const files = ['web/index.html', 'web/package.json'].concat(
      fs
        .readdirSync(srcDir)
        .filter((f) => f.endsWith('.ts') && !f.endsWith('.test.ts'))
        .map((f) => `web/src/${f}`),
    );
    for (const f of files) {
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
    expect(directive('connect-src')).toBe("connect-src 'self'");
    expect(directive('img-src')).toBe("img-src 'self' data: blob:");
    expect(directive('form-action')).toBe("form-action 'none'");
    expect(csp).not.toMatch(/https?:/);
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
