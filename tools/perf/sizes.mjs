// Size report of a perf run (PERF-BUDGETS: WASM size, total download PLAT-001).
// Usage: node tools/perf/sizes.mjs <PERF_WORK>   → JSON on stdout.
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import zlib from 'node:zlib';
import { execFileSync } from 'node:child_process';

const work = process.argv[2];
const tree = path.join(work, 'tree');
const gz = (b) => zlib.gzipSync(b, { level: 9 }).length;
const br = (b) =>
  zlib.brotliCompressSync(b, { params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 11 } }).length;
const file = (p) => {
  if (!fs.existsSync(p)) return null;
  const b = fs.readFileSync(p);
  return { bytes: b.length, gzip: gz(b), brotli: br(b) };
};

function walk(dir, out = []) {
  if (!fs.existsSync(dir)) return out;
  for (const e of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, e.name);
    if (e.isDirectory()) walk(p, out);
    else out.push(p);
  }
  return out;
}

// wasm-opt of the wasm-pack cache (estimate of an extra -Oz / -O3 pass on the release module)
function wasmOpt() {
  const cache = path.join(os.homedir(), '.cache/.wasm-pack');
  if (!fs.existsSync(cache)) return null;
  for (const d of fs.readdirSync(cache)) {
    const p = path.join(cache, d, 'bin/wasm-opt');
    if (fs.existsSync(p)) return p;
  }
  return null;
}

const releaseWasm = path.join(tree, 'crates/zoo-web/pkg/zoo_web_bg.wasm');
const out = {
  wasm: {
    dev: file(path.join(work, 'pkg-dev/zoo_web_bg.wasm')),
    release_rustc: file(path.join(work, 'target/wasm32-unknown-unknown/release/zoo_web.wasm')),
    release: file(releaseWasm),
    release_js_glue: file(path.join(tree, 'crates/zoo-web/pkg/zoo_web.js')),
  },
  dist: {},
};

const opt = wasmOpt();
if (opt && fs.existsSync(releaseWasm)) {
  for (const level of ['-Oz', '-O3']) {
    const tmp = path.join(work, `opt${level}.wasm`);
    try {
      execFileSync(opt, [level, '--enable-bulk-memory', '--enable-nontrapping-float-to-int', '--enable-sign-ext', '--enable-mutable-globals', releaseWasm, '-o', tmp], { stdio: 'ignore' });
      out.wasm[`release_plus${level}`] = file(tmp);
    } catch {
      out.wasm[`release_plus${level}`] = null;
    }
  }
}

// dist groups: bundle (JS/CSS/WASM), assets/<dir>
const dist = path.join(tree, 'web/dist');
const groups = {};
let total = { files: 0, bytes: 0, gzip: 0, brotli: 0 };
const biggest = [];
for (const p of walk(dist)) {
  const rel = path.relative(dist, p).split(path.sep).join('/');
  const parts = rel.split('/');
  const key = parts[0] === 'assets' && parts.length > 2 ? `assets/${parts[1]}` : parts.length > 1 ? parts[0] : 'root';
  const f = file(p);
  const g = (groups[key] ??= { files: 0, bytes: 0, gzip: 0, brotli: 0 });
  for (const t of [g, total]) {
    t.files += 1;
    t.bytes += f.bytes;
    t.gzip += f.gzip;
    t.brotli += f.brotli;
  }
  biggest.push({ path: rel, ...f });
}
biggest.sort((a, b) => b.bytes - a.bytes);
out.dist = { total, groups, biggest: biggest.slice(0, 15) };
console.log(JSON.stringify(out, null, 2));
