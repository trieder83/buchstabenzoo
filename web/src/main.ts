// Buchstabenzoo host shell (TECH-ARCH): loads the WASM game, owns the canvas, fetches the
// asset files, forwards input and shows the HTML overlays. All game logic, what is
// interactable and every text live in Rust (zoo-web / zoo-core, Fluent).
import init, { App, required_assets } from '../../crates/zoo-web/pkg/zoo_web.js';
import { installErrorCapture } from './ads-debug';
import { AdsHost } from './ads-ui';
import { Analytics, browserAnalyticsEnv } from './analytics';
import { MEASUREMENT_ID } from './analytics-config';
import { createAnalyticsRow } from './analytics-ui';
import { AD_TEST_BUILD, resolveKeys, testKeyParam } from './ads';
import { attachUiTaps, GameAudio, SOUND_EVENT } from './audio';
import { attachInput, type StickView } from './input';
import { adKeys, analyticsId } from './native';
import { qualityMode } from './quality';
import { newGameSeed, SaveSlot } from './save';
import { updateTextTextures } from './text';
import { introEnabled, loadSettings, Ui } from './ui';

installErrorCapture(); // field diagnostics of the ad boards (ADS-038): last errors, context loss

/** The day levels, joined into one zoo (GAME-LAYOUT "Joining levels", proposal Q-088). */
const LEVELS = [
  'levels/level-1.toml',
  'levels/level-2.toml',
  'levels/level-3.toml',
  // the night zoo behind the moon door (GAME-NIGHT rule 4, NIGHT-004)
  'levels/night-1.toml',
  // the terrarium garden behind the lantern gate of night_1 (GAME-LEVEL-NIGHT-2)
  'levels/night-2.toml',
];

/** Debug handle for e2e tests. */
export interface ZooDebug {
  app: App;
  ui: Ui;
  /** Sound playback with the decision log (ART-SOUND "Playback"). */
  audio: GameAudio;
  slot: SaveSlot;
  /** Ad billboards (GAME-ADS): content state for tests. */
  ads: AdsHost;
  /** Field diagnostics of the ad boards as JSON data (ADS-038; the same as `?adsdebug=1` shows). */
  adsDebug: () => Record<string, unknown>;
  /** Opt-in analytics (PLAT-022). */
  analytics: Analytics;
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
  app.set_math_level(settings.mathLevel ?? 'mathe1'); // GAME-CART rule 21
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
  // opt-in analytics (PLAT-022..032): nothing happens until a parent allowed it; an empty id = off entirely
  const analytics = new Analytics(
    browserAnalyticsEnv(() => ({ language: app.language(), readingLevel: app.reading_level() })),
    analyticsId(MEASUREMENT_ID), // '' in the native app build (PLAT-036)
  );
  if (analytics.available) {
    const { row, relabel, showWelcome } = createAnalyticsRow(app, analytics);
    ui.addSettingsRow(row, relabel);
    ui.onGameEvent = (e) => analytics.onGameEvent(e);
    // the level part the player stands in, once a second and only while analytics is on (no per-frame code)
    window.setInterval(() => {
      if (analytics.on) analytics.observeLevel(app.player_level());
    }, 1000);
    analytics.init();
    // first start: the welcome dialog (animals escaped, data note, Yes / No) before the intro
    if (introEnabled(window.location.search, navigator.webdriver === true)) showWelcome();
  }
  // ad billboards (GAME-ADS): signed external campaigns load after the first frame
  const ads = new AdsHost(app, { keys: adKeys(resolveKeys(AD_TEST_BUILD ? testKeyParam(window.location.search) : null)), store });
  ui.onAllDone = () => ads.openCarousel();
  attachInput(app, {
    canvas,
    stickView: stickView(),
    onFirstTouch: () => ui.setTouch(),
    onInteract: () => {
      if (!ads.interact()) ui.interact();
    },
    onEscape: () => {
      if (!ads.closeIfOpen()) ui.escape();
    },
  });
  // sound: Web Audio starts after the first gesture, files load lazily (ASND-007)
  const audio = new GameAudio(index.filter((p) => p.startsWith('audio/')));
  audio.setEnabled(settings.sound !== false);
  audio.attach();
  attachUiTaps(app);
  window.addEventListener(SOUND_EVENT, (e) => audio.setEnabled((e as CustomEvent<{ on: boolean }>).detail.on));
  canvas.focus();

  const debug: ZooDebug = { app, ui, audio, slot, ads, adsDebug: () => ads.debug(), analytics, frames: 0, frameMs: 0, intervalMs: 0 };
  window.__zoo = debug;
  let last = performance.now();
  const loop = (now: number) => {
    const dt = (now - last) / 1000;
    last = now;
    const t0 = performance.now();
    updateTextTextures(app);
    app.frame(dt);
    ui.update();
    ads.tick();
    audio.tick(app);
    slot.tick();
    const ms = performance.now() - t0;
    debug.frames += 1;
    if (debug.frames === 1) ads.start();
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
