/// <reference types="vitest/config" />
// Vite config for the thin TypeScript host (TECH-ARCH). The game itself is the WASM module
// built by wasm-pack into crates/zoo-web/pkg (imported directly from there).
//
// Game assets are NOT copied into web/: the `zooAssets` plugin serves the repo's
// `assets/{levels,models,textures,i18n,audio}` under `/assets/` in dev and preview, emits them
// into `dist/assets/` on build, and publishes `/assets/index.json` (the list of files) so
// the host only fetches files that exist (no 404s in the console).
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { Connect, Plugin } from 'vite';
import { defineConfig } from 'vite';

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, '..');
const assetsRoot = path.join(repoRoot, 'assets');
const ASSET_DIRS = ['levels', 'models', 'textures', 'i18n', 'audio'];
const TYPES: Record<string, string> = {
  '.glb': 'model/gltf-binary',
  '.png': 'image/png',
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

function zooAssets(): Plugin {
  const middleware: Connect.NextHandleFunction = (req, res, next) => {
    const url = decodeURIComponent((req.url ?? '').split('?')[0]);
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
    },
  };
}

export default defineConfig({
  base: './',
  appType: 'mpa', // no SPA fallback: unknown paths are 404, never index.html
  plugins: [zooAssets()],
  server: { fs: { allow: [repoRoot] } },
  build: { assetsDir: 'bundle', target: 'es2022' },
  test: { include: ['src/**/*.test.ts'] },
});
