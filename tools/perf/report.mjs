// Markdown summary of a perf run for specs/50-performance/measurements.md.
// Usage: node tools/perf/report.mjs <out dir> [<previous results.json>]
// Merges meta.json, sizes.json, probe.json and browser.json into results.json and prints
// the tables. With a previous results.json, values > 10 % worse are printed in bold.
import fs from 'node:fs';
import path from 'node:path';

const outDir = process.argv[2];
const read = (f) => {
  const p = path.join(outDir, f);
  return fs.existsSync(p) ? JSON.parse(fs.readFileSync(p, 'utf8')) : null;
};
const r = { meta: read('meta.json'), sizes: read('sizes.json'), probe: read('probe.json'), browser: read('browser.json') };
fs.writeFileSync(path.join(outDir, 'results.json'), JSON.stringify(r, null, 2));
const prevPath = process.argv[3];
const prev = prevPath && fs.existsSync(prevPath) ? JSON.parse(fs.readFileSync(prevPath, 'utf8')) : null;

const kb = (b) => (b == null ? '—' : `${(b / 1024).toFixed(0)} KB`);
const mb = (b) => (b == null ? '—' : `${(b / 1048576).toFixed(2)} MB`);
const f1 = (x) => (x == null ? '—' : Number(x).toFixed(1));
const f2 = (x) => (x == null ? '—' : Number(x).toFixed(2));
const get = (o, p) => p.split('.').reduce((a, k) => (a == null ? a : a[k]), o);
/** Formats `cur` and flags it when > 10 % worse (higher) than the previous run's value. */
const cmp = (fmt, p, root = r, prevRoot = prev) => {
  const cur = get(root, p);
  const old = prevRoot ? get(prevRoot, p) : null;
  const s = fmt(cur);
  if (cur != null && old != null && old > 0 && cur > old * 1.1) return `**${s}** (was ${fmt(old)})`;
  return s;
};

const lines = [];
const m = r.meta ?? {};
lines.push(`- Commit \`${m.commit}\`, ${m.dirty_files ? `**dirty tree: ${m.dirty_files} changed/untracked files** (diff sha1 ${m.diff_sha1})` : 'clean tree'}; ${m.date}`);
lines.push(`- Machine: ${m.cpu}, ${m.host}; ${m.rustc}; ${m.wasm_pack}`);
const anyVp = r.browser && Object.values(r.browser.viewports)[0];
if (anyVp) lines.push(`- Browser: ${anyVp.load.renderer} — ${anyVp.load.user_agent.replace(/.*(HeadlessChrome\/[\d.]+|Chrome\/[\d.]+).*/, '$1')}; ${r.browser.frames_per_sample} frames per sample`);
lines.push('');

if (r.sizes) {
  const w = r.sizes.wasm;
  lines.push('**Sizes**');
  lines.push('');
  lines.push('| Artefact | raw | gzip | brotli |');
  lines.push('|---|---|---|---|');
  const row = (name, key) => {
    const x = w[key];
    if (!x) return;
    lines.push(`| ${name} | ${cmp(kb, `sizes.wasm.${key}.bytes`)} | ${kb(x.gzip)} | ${kb(x.brotli)} |`);
  };
  row('WASM dev (`--dev`)', 'dev');
  row('WASM release, rustc output (opt-level "s", LTO)', 'release_rustc');
  row('WASM release after wasm-bindgen + wasm-opt -Os (shipped)', 'release');
  row('… + extra wasm-opt -Oz (estimate)', 'release_plus-Oz');
  row('… + extra wasm-opt -O3 (estimate)', 'release_plus-O3');
  row('JS glue (zoo_web.js)', 'release_js_glue');
  const d = r.sizes.dist;
  lines.push(`| **web/dist total** (${d.total.files} files) | ${cmp(mb, 'sizes.dist.total.bytes')} | ${mb(d.total.gzip)} | ${mb(d.total.brotli)} |`);
  for (const [k, g] of Object.entries(d.groups).sort((a, b) => b[1].bytes - a[1].bytes)) {
    lines.push(`| · ${k} (${g.files}) | ${mb(g.bytes)} | ${mb(g.gzip)} | ${mb(g.brotli)} |`);
  }
  lines.push('');
}

if (r.browser) {
  lines.push('**Load** (first load, empty cache, local `vite preview`, no HTTP compression)');
  lines.push('');
  lines.push('| Viewport | canvas px | first frame (wall) | resources | transferred | WASM heap | JS heap |');
  lines.push('|---|---|---|---|---|---|---|');
  for (const [id, v] of Object.entries(r.browser.viewports)) {
    const s1 = Object.values(v.scenarios)[0];
    lines.push(`| ${id} | ${s1 ? s1.canvas.join('×') : '—'} | ${cmp((x) => (x == null ? '—' : `${(x / 1000).toFixed(1)} s`), `browser.viewports.${id}.load.first_frame_wall_ms`)} | ${v.load.resources} | ${mb(v.load.transfer_bytes)} | ${mb(v.load_after?.wasm_memory_bytes ?? v.load.wasm_memory_bytes)} | ${mb(v.load_after?.js_heap_used_bytes)} |`);
  }
  lines.push('');
  for (const [id, v] of Object.entries(r.browser.viewports)) {
    const la = (a) => (a ? a.map((x) => x.toFixed(1)).join(' / ') : '—');
    lines.push(`**Scenarios — ${id}** (${v.viewport.width}×${v.viewport.height} CSS @ DPR ${v.viewport.dpr}; load average ${la(v.loadavg_start)} → ${la(v.loadavg_end)} on ${v.cpus} CPUs${v.loadavg_start && v.loadavg_start[0] > v.cpus / 2 ? ' — **machine busy: timings unreliable, counts valid**' : ''})`);
    lines.push('');
    lines.push('| Scenario | draw calls | inst. | tris | CPU `frame()` p50/p95 ms | CPU+GPU fenced p50/p95 ms | rAF interval p50 ms | scene / full-screen pass ms | sim step ms | GL calls | programs | tex binds | uniforms | upload KB | lights/pools |');
    lines.push('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|');
    for (const [sid, s] of Object.entries(v.scenarios)) {
      const c = s.census ?? {};
      const b = `browser.viewports.${id}.scenarios.${sid}`;
      const rowOf = (label, x, base) =>
        `| ${label} | ${cmp(String, `${base}.draw_calls`)} | ${x.instances} | ${(x.triangles / 1000).toFixed(0)} k | ${cmp(f1, `${base}.cpu_ms.p50`)} / ${f1(x.cpu_ms.p95)} | ${cmp(f1, `${base}.frame_ms.p50`)} / ${f1(x.frame_ms?.p95)} | ${f1(x.interval_ms.p50)} |`;
      lines.push(
        `${rowOf(`${sid} ${s.desc}`, s, b)} ${f1(c.scene_pass_ms)} / ${f1(c.fullscreen_pass_ms)} | ${f2(c.sim_step_ms)} | ${f1(c.gl_calls)} | ${f1(c.use_program)} | ${f1(c.bind_texture)} | ${f1(c.uniform_calls)} | ${f1((c.upload_bytes ?? 0) / 1024)} | ${s.lights?.[0] ?? '—'}/${s.lights?.[1] ?? '—'} |`,
      );
      for (const k of Object.keys(s)) {
        if (s[k] && typeof s[k] === 'object' && s[k].cpu_ms && k !== 'census') {
          lines.push(`${rowOf(`${sid} · ${k}`, s[k], `${b}.${k}`)} | | | | | | | |`);
        }
      }
    }
    if (v.errors?.length) lines.push(`\nPage errors: ${v.errors.slice(0, 5).join(' / ')}`);
    lines.push('');
  }
}

if (r.probe) {
  const g = r.probe.game;
  lines.push('**Native probe** (x86-64 release, lower bound for WASM; allocation counts are exact)');
  lines.push('');
  lines.push('| System | p50 µs | p95 µs | max µs | heap allocs / frame | bytes / frame |');
  lines.push('|---|---|---|---|---|---|');
  lines.push(`| zoo-core \`Game::update\` (joined zoo, ${g.frames} frames) | ${cmp(f1, 'probe.game.game_update_us.p50')} | ${f1(g.game_update_us.p95)} | ${f1(g.game_update_us.max)} | ${cmp(f1, 'probe.game.game_allocs_per_frame')} | ${f1(g.game_alloc_bytes_per_frame)} |`);
  lines.push(`| \`Ambient\` update + poses (${g.ambient_poses} crowd poses, ${g.butterflies} butterflies) | ${f1(g.ambient_us.p50)} | ${f1(g.ambient_us.p95)} | ${f1(g.ambient_us.max)} | ${f1(g.ambient_allocs_per_frame)} | |`);
  lines.push(`| \`LevelScene::build\` (once at load) | ${f1(g.scene_build_ms * 1000)} | | | | |`);
  lines.push('');
  const an = r.probe.anim;
  const tot = an.reduce((s, a) => s + a.pose_us, 0);
  const worst = [...an].sort((a, b) => b.pose_us - a.pose_us).slice(0, 5);
  lines.push(`Animation pose (sample idle + walk, blend, joint matrices) — ${an.length} skinned models, sum of one pose each ${f1(tot)} µs; slowest: ${worst.map((a) => `${path.basename(a.path, '.glb')} ${f1(a.pose_us)} µs (${a.joints} joints)`).join(', ')}.`);
  lines.push('');
  const models = r.probe.models;
  const budget = (x) => (x.kind === 'animals' || x.kind === 'characters' ? 3000 : x.kind === 'props' ? 500 : null);
  const over = models.filter((x) => budget(x) && x.triangles > budget(x));
  const tex = models.filter((x) => x.texture && Number(x.texture.split('x')[0]) > 256);
  lines.push(`Models: ${models.length} \`.glb\`, ${models.reduce((s, x) => s + x.triangles, 0)} triangles in total. Over the APIPE-005 triangle budget (animals/characters 3 000, props 500): ${over.length ? over.map((x) => `${x.path} ${x.triangles}`).join(', ') : 'none'}. Buildings (no budget yet): ${models.filter((x) => x.kind === 'buildings').map((x) => `${path.basename(x.path, '.glb')} ${x.triangles}`).join(', ')}. Textures > 256²: ${tex.length ? tex.map((x) => `${x.path} ${x.texture}`).join(', ') : 'none'}.`);
  lines.push('');
}
console.log(lines.join('\n'));
