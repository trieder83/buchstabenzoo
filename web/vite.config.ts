/// <reference types="vitest/config" />
// Vite config for the thin TypeScript host (TECH-ARCH). The game itself is the WASM module
// built by wasm-pack into crates/zoo-web/pkg (imported directly from there).
//
// Game assets are NOT copied into web/: the `zooAssets` plugin serves the repo's
// `assets/{levels,models,textures,i18n,audio}` under `/assets/` in dev and preview, emits them
// into `dist/assets/` on build, and publishes `/assets/index.json` (the list of files) so
// the host only fetches files that exist (no 404s in the console).
import { execSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { Connect, Plugin } from 'vite';
import { defineConfig } from 'vite';

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, '..');
const assetsRoot = path.join(repoRoot, 'assets');
const ASSET_DIRS = ['levels', 'models', 'textures', 'i18n', 'audio'];
// Ad content (GAME-ADS "External content"): the signed manifest + its images live in the repo's
// `boards/` and are served at `/boards/` (same origin, swapped without a game update, PLAT-010).
// The URL, file and DOM names avoid the words ad/advert/banner/sponsor/promo: browser ad blockers
// filter by name (ADS-043).
const adsRoot = path.join(repoRoot, 'boards');
const TYPES: Record<string, string> = {
  '.glb': 'model/gltf-binary',
  '.png': 'image/png',
  '.webp': 'image/webp',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.sig': 'text/plain; charset=utf-8',
  '.toml': 'text/plain; charset=utf-8',
  '.ftl': 'text/plain; charset=utf-8',
  '.json': 'application/json',
  '.ogg': 'audio/ogg',
  '.m4a': 'audio/mp4',
};

/** Paths (relative to assets/, forward slashes) of every servable asset file. */
export function listAssets(root = assetsRoot): string[] {
  const out: string[] = [];
  const walk = (dir: string) => {
    for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, e.name);
      if (e.isDirectory()) walk(p);
      else if (TYPES[path.extname(e.name)]) out.push(path.relative(root, p).split(path.sep).join('/'));
    }
  };
  for (const d of ASSET_DIRS) {
    const p = path.join(root, d);
    if (fs.existsSync(p)) walk(p);
  }
  return out.sort();
}

/** Files of `boards/` that are served: manifest, signature, images (not templates or keys). */
export function listAds(root = adsRoot): string[] {
  const out: string[] = [];
  for (const f of ['index.json', 'index.sig']) {
    if (fs.existsSync(path.join(root, f))) out.push(f);
  }
  const img = path.join(root, 'img');
  if (fs.existsSync(img)) {
    for (const f of fs.readdirSync(img).sort()) if (/\.(png|webp|jpe?g)$/.test(f)) out.push(`img/${f}`);
  }
  return out;
}

/**
 * The signed poster manifest dir of the native app build, or null for the web build (PLAT-057). `VITE_NATIVE_PLATFORM`
 * is `ios` (default, `boards-native/`) or `android` (`boards-native-android/`); each bundle carries ONLY its own manifest.
 */
export function nativeBoardsDir(env: Record<string, string | undefined> = process.env): string | null {
  if (env.VITE_NATIVE !== '1') return null;
  const platform = env.VITE_NATIVE_PLATFORM ?? 'ios';
  if (platform === 'ios') return path.join(repoRoot, 'boards-native');
  if (platform === 'android') return path.join(repoRoot, 'boards-native-android');
  throw new Error(`VITE_NATIVE_PLATFORM must be ios or android, got ${platform}`);
}

function zooAssets(): Plugin {
  const middleware: Connect.NextHandleFunction = (req, res, next) => {
    const url = decodeURIComponent((req.url ?? '').split('?')[0]);
    if (url.startsWith('/boards/')) {
      const rel = url.slice('/boards/'.length);
      if (!listAds().includes(rel)) {
        res.statusCode = 404;
        res.end();
        return;
      }
      res.setHeader('Content-Type', TYPES[path.extname(rel)] ?? 'application/octet-stream');
      res.setHeader('Cache-Control', 'no-store');
      fs.createReadStream(path.join(adsRoot, rel)).pipe(res);
      return;
    }
    if (!url.startsWith('/assets/')) return next();
    const rel = url.slice('/assets/'.length);
    if (rel === 'index.json') {
      res.setHeader('Content-Type', TYPES['.json']);
      res.setHeader('Cache-Control', 'no-store');
      res.end(JSON.stringify(listAssets()));
      return;
    }
    const file = path.resolve(assetsRoot, rel);
    const top = rel.split('/')[0];
    if (!file.startsWith(assetsRoot + path.sep) || !ASSET_DIRS.includes(top) || !fs.existsSync(file)) {
      if (ASSET_DIRS.includes(top)) {
        res.statusCode = 404;
        res.end();
        return;
      }
      return next();
    }
    res.setHeader('Content-Type', TYPES[path.extname(file)] ?? 'application/octet-stream');
    res.setHeader('Cache-Control', 'no-store');
    fs.createReadStream(file).pipe(res);
  };
  return {
    name: 'zoo-assets',
    configureServer(server) {
      server.middlewares.use(middleware);
    },
    configurePreviewServer(server) {
      // dist/assets holds the build copy; serving from the repo keeps preview fresh too.
      server.middlewares.use(middleware);
    },
    generateBundle() {
      const files = listAssets();
      for (const rel of files) {
        this.emitFile({ type: 'asset', fileName: `assets/${rel}`, source: fs.readFileSync(path.join(assetsRoot, rel)) });
      }
      this.emitFile({ type: 'asset', fileName: 'assets/index.json', source: JSON.stringify(files) });
      // the native app build (VITE_NATIVE=1, PLAT-036/038) carries NOT the web campaigns but its own signed
      // manifest `boards-native/` (links only to the developer's App Store pages), emitted under the same `boards/` path
      const native = process.env.VITE_NATIVE === '1';
      const root = nativeBoardsDir() ?? adsRoot;
      for (const rel of native && !fs.existsSync(root) ? [] : listAds(root)) {
        this.emitFile({ type: 'asset', fileName: `boards/${rel}`, source: fs.readFileSync(path.join(root, rel)) });
      }
    },
  };
}

/** Short build id for the anonymous counters (counter.ts): git hash, else the package version. */
function appVersion(): string {
  try {
    return execSync('git rev-parse --short=8 HEAD', { cwd: repoRoot, stdio: ['ignore', 'pipe', 'ignore'] }).toString().trim();
  } catch {
    return String((JSON.parse(fs.readFileSync(path.join(here, 'package.json'), 'utf8')) as { version?: string }).version ?? 'dev');
  }
}

export default defineConfig(({ command }) => ({
  base: './',
  // `?adkey=` (test public key for the ad signature) exists only in the dev server and in the
  // e2e test build (`VITE_AD_TEST=1`), never in the release bundle (PLAT-012, Q-245).
  define: {
    __AD_TEST__: JSON.stringify(command === 'serve' || process.env.VITE_AD_TEST === '1'),
    // fake GA4 id for the e2e test build only (PLAT-029); '' in the release build
    __APP_VERSION__: JSON.stringify(appVersion()),
    __ANALYTICS_TEST_ID__: JSON.stringify(process.env.VITE_ANALYTICS_TEST_ID ?? ''),
  },
  appType: 'mpa', // no SPA fallback: unknown paths are 404, never index.html
  plugins: [zooAssets()],
  server: { fs: { allow: [repoRoot] } },
  build: { assetsDir: 'bundle', target: ['chrome87', 'es2020', 'safari14'] },
  test: { include: ['src/**/*.test.ts'] },
}));
