// Buchstabenzoo host shell (TECH-ARCH): loads the WASM game, owns the canvas, fetches the
// asset files and forwards input. All game logic and rendering live in Rust (zoo-web).
import init, { App, required_assets } from '../../crates/zoo-web/pkg/zoo_web.js';
import { attachInput } from './input';

const LEVEL = 'levels/level-1.toml';

/** Debug handle for e2e tests (read-only use). */
export interface ZooDebug {
  app: App;
  frames: number;
  /** Average CPU time of `app.frame()` in ms (exponential moving average). */
  frameMs: number;
  /** Average time between animation frames in ms. */
  intervalMs: number;
}

declare global {
  interface Window {
    __zoo?: ZooDebug;
    __zooError?: string;
  }
}

async function fetchBytes(path: string): Promise<Uint8Array | null> {
  const res = await fetch(`assets/${path}`);
  if (!res.ok) return null;
  return new Uint8Array(await res.arrayBuffer());
}

async function main(): Promise<void> {
  await init();
  const index: string[] = await (await fetch('assets/index.json')).json();
  const available = new Set(index);

  const level = await fetchBytes(LEVEL);
  if (!level) throw new Error(`missing ${LEVEL}`);
  const wanted = new Set<string>([
    ...required_assets(new TextDecoder().decode(level)),
    ...index.filter((p) => p.startsWith('i18n/') && p.endsWith('.ftl')),
  ]);
  const files = new Map<string, Uint8Array>([[LEVEL, level]]);
  await Promise.all(
    [...wanted]
      .filter((p) => available.has(p))
      .map(async (p) => {
        const bytes = await fetchBytes(p);
        if (bytes) files.set(p, bytes);
      }),
  );

  const canvas = document.getElementById('game') as HTMLCanvasElement;
  const app = new App(canvas, LEVEL, files);
  const resize = () => app.resize(canvas.clientWidth, canvas.clientHeight, window.devicePixelRatio || 1);
  resize();
  new ResizeObserver(resize).observe(canvas);
  window.addEventListener('orientationchange', resize);

  attachInput(app, canvas, document.getElementById('stick')!, document.getElementById('knob')!);
  canvas.focus();

  const debug: ZooDebug = { app, frames: 0, frameMs: 0, intervalMs: 0 };
  window.__zoo = debug;
  let last = performance.now();
  const loop = (now: number) => {
    const dt = (now - last) / 1000;
    last = now;
    const t0 = performance.now();
    app.frame(dt);
    const ms = performance.now() - t0;
    debug.frames += 1;
    debug.frameMs = debug.frames === 1 ? ms : debug.frameMs * 0.95 + ms * 0.05;
    debug.intervalMs = debug.frames === 1 ? dt * 1000 : debug.intervalMs * 0.95 + dt * 1000 * 0.05;
    requestAnimationFrame(loop);
  };
  requestAnimationFrame(loop);
}

main().catch((e: unknown) => {
  window.__zooError = String(e);
  console.error(e);
});
