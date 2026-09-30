// Buchstabenzoo host shell (TECH-ARCH): loads the WASM game, owns the canvas, fetches the
// asset files, forwards input and shows the HTML overlays. All game logic, what is
// interactable and every text live in Rust (zoo-web / zoo-core, Fluent).
import init, { App, required_assets } from '../../crates/zoo-web/pkg/zoo_web.js';
import { attachInput, attachLookButton, type StickView } from './input';
import { qualityMode } from './quality';
import { newGameSeed, SaveSlot } from './save';
import { updateTextTextures } from './text';
import { introEnabled, loadSettings, Ui } from './ui';

/** The day levels, joined into one zoo (GAME-LAYOUT "Joining levels", proposal Q-088). */
const LEVELS = [
  'levels/level-1.toml',
  'levels/level-2.toml',
  'levels/level-3.toml',
  // the night zoo behind the moon door (GAME-NIGHT rule 4, NIGHT-004)
  'levels/night-1.toml',
];

/** Debug handle for e2e tests. */
export interface ZooDebug {
  app: App;
  ui: Ui;
  slot: SaveSlot;
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

function storage(): Storage | null {
  try {
    return window.localStorage;
  } catch {
    return null;
  }
}

function stickView(): StickView {
  const stick = document.getElementById('stick')!;
  const knob = document.getElementById('knob')!;
  return {
    show(ox, oy, kx, ky) {
      stick.classList.add('active');
      stick.style.left = `${ox}px`;
      stick.style.top = `${oy}px`;
      knob.style.transform = `translate(${kx - ox}px, ${ky - oy}px)`;
    },
    hide() {
      stick.classList.remove('active');
      stick.style.left = '';
      stick.style.top = '';
      knob.style.transform = '';
    },
  };
}

async function main(): Promise<void> {
  await init();
  const index: string[] = await (await fetch('assets/index.json')).json();
  const available = new Set(index);

  const files = new Map<string, Uint8Array>();
  const levels = LEVELS.filter((p) => available.has(p));
  for (const p of levels) {
    const bytes = await fetchBytes(p);
    if (!bytes) throw new Error(`missing ${p}`);
    files.set(p, bytes);
  }
  const wanted = new Set<string>([
    ...required_assets(levels.map((p) => new TextDecoder().decode(files.get(p)!))),
    ...index.filter((p) => p.startsWith('i18n/') && p.endsWith('.ftl')),
  ]);
  await Promise.all(
    [...wanted]
      .filter((p) => available.has(p))
      .map(async (p) => {
        const bytes = await fetchBytes(p);
        if (bytes) files.set(p, bytes);
      }),
  );

  const canvas = document.getElementById('game') as HTMLCanvasElement;
  const app = new App(canvas, levels, files);
  const store = storage();
  // GAME-SAVE: continue where the child stopped (restored before the first frame); else a
  // new game with a random seed (`?seed=N` for tests) that avoids the last hiding places
  const slot = new SaveSlot(app, store);
  slot.start(newGameSeed(window.location.search));
  document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'hidden') slot.flush();
  });
  window.addEventListener('pagehide', () => slot.flush());
  const settings = loadSettings(store, App.default_language(navigator.language || 'de'));
  app.set_language(settings.language);
  app.set_reading_level(settings.readingLevel);
  app.set_view_mode(settings.view ?? 'zoo'); // GAME-CAMERA-VIEWS 9

  updateTextTextures(app); // sign texts (re-rendered on language change, in the loop)

  const resize = () => app.resize(canvas.clientWidth, canvas.clientHeight, window.devicePixelRatio || 1);
  resize();
  // automatic quality tier for weak phones (PERF-BUDGETS rule 5); `?quality=` overrides
  app.set_quality(qualityMode(window.location.search, navigator.webdriver === true));
  new ResizeObserver(resize).observe(canvas);
  window.addEventListener('orientationchange', resize);

  const ui = new Ui(app, store, () => {
    // new game (confirmed in the settings, GAME-SAVE §6): delete the save, restart the level
    slot.reset();
    window.location.reload();
  }, introEnabled(window.location.search, navigator.webdriver === true));
  attachInput(app, {
    canvas,
    stickView: stickView(),
    onFirstTouch: () => ui.setTouch(),
    onInteract: () => ui.interact(),
    onEscape: () => ui.escape(),
  });
  attachLookButton(document.getElementById('look-btn')!, app);
  canvas.focus();

  const debug: ZooDebug = { app, ui, slot, frames: 0, frameMs: 0, intervalMs: 0 };
  window.__zoo = debug;
  let last = performance.now();
  const loop = (now: number) => {
    const dt = (now - last) / 1000;
    last = now;
    const t0 = performance.now();
    updateTextTextures(app);
    app.frame(dt);
    ui.update();
    slot.tick();
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
