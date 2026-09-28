// Firebase Hosting preview deployment (TECH-PLATFORMS "Preview deployment for playtests").
// Static checks only: reads firebase.json, the deploy script and the host sources — no network.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

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
