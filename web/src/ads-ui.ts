// Host side of the ad billboards (GAME-ADS): fills the board pictures (placeholder text or a
// verified campaign image), opens the reading panel in front of a readable board and runs the
// parental gate before the link. All content checks live in ads.ts; this file only draws.
// Every text from outside is set with `textContent` (never HTML).
import {
  Carousel,
  carouselItems,
  GATE_HOLD_MS,
  loadAds,
  ParentalGate,
  makeLanguageGateQuestion,
  pickImage,
  type AdContent,
  type KeyValue,
  type LoadOptions,
  type VerifiedCampaign,
} from './ads';
import { AdsDebugOverlay, adsDebugEnabled, adsTelemetry, envInfo, logAdError, logAdEvent } from './ads-debug';
import { CREAM, renderTextTexture } from './text';

/** The subset of the WASM `App` the ads need. */
export interface AdsApp {
  ad_boards_json(): string;
  ad_near(): string;
  set_ad_texture(id: string, width: number, height: number, rgba: Uint8Array): boolean;
  language(): string;
  player_x?(): number;
  player_z?(): number;
  reading_level(): string;
  t(key: string): string;
  panel_key(): string;
  target_kind?(): string;
}

interface Board {
  id: string;
  slot: number;
  n: number;
  w: number;
  h: number;
  x?: number;
  z?: number;
}

export interface AdsHostOptions {
  keys: readonly Uint8Array[];
  /** URL prefix of the ads directory (same origin). */
  base?: string;
  store: KeyValue | null;
  fetchFn?: LoadOptions['fetchFn'];
  /** Opens the link (default `window.open` with `noopener,noreferrer`), called once per passed gate. */
  open?: (url: string) => void;
  now?: () => number;
  /** Delay before the single retry of a failed load (ms, default 20 s; ADS-029). */
  retryMs?: number;
}

/** The 3 retries after a failed / incomplete load come from `online`, coming back to the tab, or the timer (ADS-039). */
export const MAX_LOADS = 4;
/** After `window.open` the page must have been hidden within this time (a new tab took over), else the fallback link shows (ADS-040). */
export const OPEN_CHECK_MS = 1500;

/** A ✖ tap this soon after the panel opened is an accidental touch (ADS-027). */
export const CLOSE_GUARD_MS = 400;

const STYLE = `
#ad-panel{position:fixed;left:0;right:0;top:0;z-index:5;display:flex;justify-content:center;pointer-events:none;padding:calc(10px + env(safe-area-inset-top)) env(safe-area-inset-right) 0 env(safe-area-inset-left)}
#ad-panel[hidden],#ad-gate[hidden]{display:none}
.ad-body{position:relative;box-sizing:border-box;width:min(96vw,900px);pointer-events:auto;padding:10px 12px 12px;border-radius:22px;border:6px solid #3b2314;background:#fff8e7;box-shadow:0 8px 0 rgba(59,35,20,.5);text-align:center;display:flex;flex-direction:column;align-items:center;gap:8px}
.ad-text{display:flex;flex-direction:column;align-items:center;gap:8px;min-width:0}
#ad-act{position:fixed;z-index:4;right:calc(24px + env(safe-area-inset-right));bottom:calc(28px + env(safe-area-inset-bottom));width:96px;height:96px;font-size:50px;background:#ffd65c}
#ad-act[hidden]{display:none}
.ad-x{position:absolute;right:8px;top:8px;width:64px;height:64px;border-radius:50%;border:4px solid #3b2314;background:#fff;font-size:28px;z-index:1}
#ad-image{display:block;max-width:calc(100% - 80px);max-height:16vh;max-height:16dvh;border:4px solid #3b2314;border-radius:12px;background:${CREAM}}
#ad-tagline{margin:0;font-size:max(2.6vh,18px);line-height:1.2;padding:0 70px;font-weight:bold;color:#3b2314}
#ad-link,.ad-choice,#ad-hold{touch-action:manipulation;min-height:64px;min-width:64px;border-radius:32px;border:5px solid #3b2314;background:#7cc46a;color:#3b2314;font-size:max(2.2vh,17px);font-weight:bold;padding:6px 14px;max-width:100%;box-sizing:border-box;display:inline-flex;gap:12px;align-items:center;justify-content:center;box-shadow:0 5px 0 rgba(59,35,20,.55)}
#ad-gate{position:fixed;top:0;left:0;right:0;bottom:0;z-index:8;display:flex;align-items:center;justify-content:center;background:rgba(40,25,15,.6);pointer-events:auto}
.ad-gate-card{position:relative;box-sizing:border-box;width:min(92vw,620px);padding:18px;border-radius:26px;border:6px solid #3b2314;background:#fff8e7;text-align:center;display:flex;flex-direction:column;gap:12px;align-items:center;font-weight:bold;color:#3b2314;font-size:max(3.2vh,20px)}
.ad-gate-card h2{margin:0;font-size:1.2em}
#ad-gate-question{font-size:1.8em}
.ad-choices{display:grid;grid-template-columns:1fr 1fr;gap:12px;width:100%}
.ad-choice{background:#ffd65c}
#ad-hold{--p:0;width:150px;height:150px;border-radius:50%;padding:0;font-size:64px;background:conic-gradient(#e8604c calc(var(--p) * 360deg),#fff 0);touch-action:none;user-select:none}
#ad-hold span{display:flex;width:112px;height:112px;border-radius:50%;background:#ffd65c;align-items:center;justify-content:center;pointer-events:none}
#ad-hold,#ad-gate{-webkit-touch-callout:none;-webkit-user-select:none;user-select:none;touch-action:none}
#ad-open{min-height:72px;max-width:100%;box-sizing:border-box;border-radius:36px;border:5px solid #3b2314;background:#7cc46a;color:#3b2314;font-size:max(2.6vh,19px);font-weight:bold;padding:8px 22px;display:inline-flex;gap:12px;align-items:center;justify-content:center;text-decoration:none;box-shadow:0 5px 0 rgba(59,35,20,.55)}
#ad-fallback{position:fixed;z-index:9;left:50%;bottom:calc(16px + env(safe-area-inset-bottom));transform:translateX(-50%);box-sizing:border-box;width:min(92vw,520px);padding:12px;border-radius:22px;border:6px solid #3b2314;background:#fff8e7;color:#3b2314;font-weight:bold;font-size:max(2.2vh,17px);text-align:center;display:flex;flex-direction:column;gap:10px;align-items:center;pointer-events:auto;touch-action:manipulation}
#ad-fallback[hidden]{display:none}
.ad-open-row{display:flex;gap:10px;align-items:center;justify-content:center;flex-wrap:wrap}
#ad-dbg-chip{position:fixed;z-index:98;left:6px;bottom:6px;width:56px;height:56px;border-radius:50%;border:3px solid #fff;background:#2d5a3a;color:#fff;font:bold 16px monospace;opacity:.85}
#ad-carousel{position:fixed;left:0;right:0;top:0;z-index:5;display:flex;justify-content:center;pointer-events:none;padding:calc(10px + env(safe-area-inset-top)) env(safe-area-inset-right) 0 env(safe-area-inset-left)}
#ad-carousel[hidden]{display:none}
#ad-car-title{margin:0;padding:0 70px;font-size:max(2.8vh,18px);line-height:1.2;color:#3b2314}
#ad-car-image{display:block;max-width:100%;max-height:30vh;max-height:30dvh;border:4px solid #3b2314;border-radius:12px;background:${CREAM};cursor:pointer;touch-action:pan-y;-webkit-user-drag:none}
#ad-car-tagline{margin:0;font-weight:bold;color:#3b2314;font-size:max(2.4vh,16px)}
.ad-car-nav{display:flex;align-items:center;justify-content:center;gap:14px}
.ad-car-arrow{width:64px;height:64px;border-radius:50%;border:5px solid #3b2314;background:#ffd65c;font-size:30px;color:#3b2314;touch-action:manipulation;box-shadow:0 5px 0 rgba(59,35,20,.55)}
.ad-car-dots{display:flex;gap:10px;align-items:center}
.ad-car-dot{width:16px;height:16px;border-radius:50%;border:3px solid #3b2314;background:#fff;padding:0}
.ad-car-dot.on{background:#e8604c}
/* small screens (GAME-PLAYER 3): a compact card that leaves the right-hand control column free */
@media (orientation:landscape) and (max-height:460px){
#ad-panel{justify-content:flex-start;padding:calc(8px + env(safe-area-inset-top)) calc(96px + env(safe-area-inset-right)) 0 calc(8px + env(safe-area-inset-left))}
.ad-body{width:auto;max-width:640px;flex-direction:row;gap:12px;padding:8px 10px;border-width:5px;border-radius:18px;text-align:left}
.ad-text{flex:1;align-items:center;margin-right:68px;gap:6px}
#ad-image{max-width:40vw;max-height:min(30vh,110px);max-height:min(30dvh,110px)}
#ad-tagline{padding:0;font-size:16px}
#ad-link{min-height:64px;font-size:17px}
#ad-act{right:calc(8px + env(safe-area-inset-right));bottom:calc(8px + env(safe-area-inset-bottom));width:80px;height:80px;font-size:42px}
#ad-carousel{justify-content:flex-start;padding:calc(6px + env(safe-area-inset-top)) calc(96px + env(safe-area-inset-right)) 0 calc(8px + env(safe-area-inset-left))}
#ad-carousel .ad-body{width:auto;max-width:560px;flex-direction:column;gap:4px;padding:6px 10px;text-align:center}
#ad-car-title{font-size:15px;padding:0 70px 0 0}
#ad-car-image{max-height:min(24vh,92px);max-height:min(24dvh,92px);max-width:60vw}
#ad-car-tagline{display:none}
.ad-gate-card{padding:10px 14px;gap:6px;font-size:16px}
#ad-gate-question{font-size:1.5em}
.ad-choices{grid-template-columns:repeat(4,1fr);gap:8px}
#ad-hold{width:110px;height:110px;font-size:44px}
#ad-hold span{width:80px;height:80px}
}
@media (orientation:portrait) and (max-width:480px){
#ad-panel{padding-top:calc(152px + env(safe-area-inset-top))}
#ad-act{right:calc(8px + env(safe-area-inset-right));bottom:calc(8px + env(safe-area-inset-bottom));width:80px;height:80px;font-size:42px}
#ad-carousel{padding-top:calc(152px + env(safe-area-inset-top))}
#ad-car-image{max-height:24vh;max-height:24dvh}
}
`;

function el<K extends keyof HTMLElementTagNameMap>(tag: K, id?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (id) e.id = id;
  if (text !== undefined) e.textContent = text;
  return e;
}

export class AdsHost {
  private boards = new Map<string, Board>();
  private content: AdContent | null = null;
  private bitmaps = new Map<string, Promise<ImageBitmap | null>>();
  private lang = '';
  private level = '';
  private shown: string | null = null;
  private dismissed: string | null = null;
  private gate: ParentalGate | null = null;
  private raf = 0;
  private started = false;
  private settled = false;
  private uploading = false;
  private again = false;
  private openedAt = 0;
  private loads = 0;
  private loading = false;
  private retryTimer = 0;
  private histPushed = false;
  private ignorePop = false;
  private dbg: AdsDebugOverlay | null = null;
  private fallbackTimer = 0;
  readonly actBtn = el('button', 'ad-act', '🔗');
  readonly panel = el('div', 'ad-panel');
  readonly gateView = el('div', 'ad-gate');
  /** Shown when the browser did not open the link after the gate (pop-up blocked): a real link to tap (ADS-040). */
  readonly fallbackView = el('div', 'ad-fallback');
  /** The all-done carousel of the verified campaigns (ADS-031). */
  readonly carouselView = el('div', 'ad-carousel');
  private carousel: Carousel | null = null;
  private carouselOpenedAt = 0;
  /** Links opened so far (debug / tests). */
  opened: string[] = [];

  constructor(
    private readonly app: AdsApp,
    private readonly o: AdsHostOptions,
  ) {
    const style = el('style');
    style.textContent = STYLE;
    document.head.appendChild(style);
    this.panel.hidden = true;
    this.gateView.hidden = true;
    this.carouselView.hidden = true;
    this.fallbackView.hidden = true;
    this.actBtn.className = 'round';
    this.actBtn.type = 'button';
    this.actBtn.hidden = true;
    // pointerdown like the other touch buttons: a second finger never gets a click (CAMV-019)
    this.actBtn.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      e.stopPropagation();
      this.interact();
    });
    for (const e of [this.panel, this.gateView, this.carouselView]) {
      e.addEventListener('pointerdown', (ev) => ev.stopPropagation());
      e.addEventListener('contextmenu', (ev) => ev.preventDefault());
    }
    this.fallbackView.addEventListener('pointerdown', (ev) => ev.stopPropagation());
    document.body.append(this.panel, this.carouselView, this.gateView, this.fallbackView, this.actBtn);
    // Android Back / swipe-back closes the gate or the carousel instead of leaving the game (ADS-041)
    window.addEventListener('popstate', () => this.onPop());
    // a failed load is tried again when the phone comes online or the player returns to the tab (ADS-039)
    window.addEventListener('online', () => this.retryNow('online'));
    document.addEventListener('visibilitychange', () => {
      if (document.visibilityState === 'visible') this.retryNow('visible');
    });
    if (adsDebugEnabled(window.location.search)) {
      this.dbg = new AdsDebugOverlay(() => this.debug());
      const chip = el('button', 'ad-dbg-chip', 'AD');
      chip.type = 'button';
      chip.addEventListener('click', () => this.dbg?.show());
      document.body.append(chip);
      this.dbg.show();
    }
  }

  /** Uploads the placeholders at once, then loads the external campaigns (never blocks play). */
  start(): void {
    if (this.started) return;
    this.started = true;
    let list: Board[] = [];
    try {
      list = JSON.parse(this.app.ad_boards_json()) as Board[];
    } catch {
      list = [];
    }
    this.boards = new Map(list.map((b) => [b.id, b]));
    this.lang = this.app.language();
    void this.upload();
    void this.load();
  }

  /**
   * Loads the campaigns. A failed or incomplete load is tried again (slow mobile data, ADS-029/039): once
   * after 20 s, and at once when the phone comes back online or the tab becomes visible; at most
   * {@link MAX_LOADS} loads in total.
   */
  private async load(): Promise<void> {
    if (this.loading) return;
    this.loading = true;
    this.loads += 1;
    window.clearTimeout(this.retryTimer);
    let c: AdContent | null = null;
    try {
      c = await loadAds({
        base: this.o.base ?? 'ads/',
        keys: this.o.keys,
        fetchFn: this.o.fetchFn ?? ((url, init) => fetch(url, init)),
        now: (this.o.now ?? Date.now)(),
        store: this.o.store,
      });
    } finally {
      this.loading = false;
    }
    if (c) this.content = c;
    this.settled = true;
    if (c) void this.upload();
    const done = !!c && !c.incomplete;
    if (!done && this.o.keys.length > 0 && this.loads < MAX_LOADS) {
      this.retryTimer = window.setTimeout(() => void this.load(), this.o.retryMs ?? 20_000);
    }
  }

  private retryNow(why: string): void {
    if (!this.started || this.loading || this.o.keys.length === 0 || this.loads >= MAX_LOADS) return;
    if (this.content && !this.content.incomplete) return;
    logAdEvent(`retry (${why})`);
    void this.load();
  }

  /** Slot → campaign id that is shown now (debug / tests). */
  state(): { settled: boolean; loaded: boolean; campaigns: Record<number, string>; panel: string | null; gate: string | null; opened: string[] } {
    const campaigns: Record<number, string> = {};
    for (const [slot, c] of this.content?.bySlot ?? []) campaigns[slot] = c.id;
    return { settled: this.settled, loaded: this.content !== null, campaigns, panel: this.shown, gate: this.gate?.stage ?? null, opened: this.opened };
  }

  /** Everything the field diagnostics show (`?adsdebug=1`, `window.__zoo.adsDebug()`), plain JSON data (ADS-038). */
  debug(): Record<string, unknown> {
    let near: Record<string, unknown> = { id: this.safe(() => this.app.ad_near()), inReach: false };
    near.inReach = near.id !== '' && near.id !== null;
    const px = this.safe(() => this.app.player_x?.());
    const pz = this.safe(() => this.app.player_z?.());
    if (typeof px === 'number' && typeof pz === 'number') {
      let best: { id: string; d: number } | null = null;
      for (const b of this.boards.values()) {
        if (b.x === undefined || b.z === undefined) continue;
        const d = Math.hypot(b.x - px, b.z - pz);
        if (!best || d < best.d) best = { id: b.id, d };
      }
      near = { ...near, player: [Math.round(px * 10) / 10, Math.round(pz * 10) / 10], nearestBoard: best ? best.id : null, distM: best ? Math.round(best.d * 10) / 10 : null };
    }
    const st = this.state();
    return {
      app: 'letterzoo ads debug 1',
      uptimeS: Math.round(performance.now() / 100) / 10,
      env: envInfo(),
      keys: this.o.keys.length,
      loads: this.loads,
      settled: st.settled,
      loaded: st.loaded,
      incomplete: this.content?.incomplete ?? null,
      campaigns: st.campaigns,
      boards: this.boards.size,
      near,
      touchClass: document.body.classList.contains('touch'),
      targetKind: this.safe(() => this.app.target_kind?.() ?? ''),
      panelKey: this.safe(() => this.app.panel_key()),
      panel: st.panel,
      gate: st.gate,
      carousel: this.carousel !== null,
      actVisible: !this.actBtn.hidden,
      opened: st.opened,
      manifest: adsTelemetry.manifest,
      signature: adsTelemetry.sig,
      images: adsTelemetry.images,
      uploads: adsTelemetry.uploads,
      pointer: adsTelemetry.ptr,
      windowOpen: adsTelemetry.open,
      glContextLost: adsTelemetry.glLost,
      events: adsTelemetry.events,
      errors: adsTelemetry.errors,
    };
  }

  private safe<T>(f: () => T): T | null {
    try {
      return f();
    } catch {
      return null;
    }
  }

  get panelOpen(): boolean {
    return !this.panel.hidden || this.carousel !== null;
  }

  /** Esc / ✖: closes the gate, else the panel. True if something was closed. */
  closeIfOpen(): boolean {
    if (this.gate) {
      this.closeGate();
      return true;
    }
    if (this.carousel) {
      this.closeCarousel();
      return true;
    }
    if (this.shown) {
      this.dismiss();
      return true;
    }
    return false;
  }

  // ------------------------------------------------------------ pictures

  private bitmap(path: string, data: Uint8Array, mime: string, w: number, h: number): Promise<ImageBitmap | null> {
    let p = this.bitmaps.get(path);
    if (!p) {
      p = createImageBitmap(new Blob([data as BlobPart], { type: mime })).then(
        (b) => {
          const ok = b.width === w && b.height === h;
          const t = adsTelemetry.images[path];
          if (t) t.decoded = ok;
          if (!ok) logAdError(`decode ${path}: ${b.width}x${b.height}`);
          return ok ? b : null;
        },
        (e) => {
          const t = adsTelemetry.images[path];
          if (t) t.decoded = false;
          logAdError(`decode ${path}: ${String((e as Error)?.message ?? e)}`);
          return null;
        },
      );
      this.bitmaps.set(path, p);
    }
    return p;
  }

  private async picture(b: Board, c: VerifiedCampaign | undefined): Promise<Uint8Array> {
    const lang = this.lang;
    if (c) {
      const im = pickImage(c, lang, b.n);
      const bmp = await this.bitmap(im.path, im.data, im.mime, im.width, im.height);
      if (bmp) {
        const canvas = document.createElement('canvas');
        canvas.width = b.w;
        canvas.height = b.h;
        const ctx = canvas.getContext('2d', { willReadFrequently: true });
        if (ctx) {
          ctx.fillStyle = CREAM;
          ctx.fillRect(0, 0, b.w, b.h);
          const k = Math.min(b.w / bmp.width, b.h / bmp.height);
          const dw = bmp.width * k;
          const dh = bmp.height * k;
          ctx.drawImage(bmp, (b.w - dw) / 2, (b.h - dh) / 2, dw, dh);
          return new Uint8Array(ctx.getImageData(0, 0, b.w, b.h).data.buffer);
        }
      }
    }
    return renderTextTexture({ id: b.id, key: `ad-placeholder-${b.slot}`, text: this.app.t(`ad-placeholder-${b.slot}`), width: b.w, height: b.h });
  }

  /** (Re-)draws every board picture; coalesces overlapping requests. */
  private async upload(): Promise<void> {
    if (this.uploading) {
      this.again = true;
      return;
    }
    this.uploading = true;
    try {
      do {
        this.again = false;
        for (const b of this.boards.values()) {
          const rgba = await this.picture(b, this.content?.bySlot.get(b.slot));
          adsTelemetry.uploads[b.id] = this.app.set_ad_texture(b.id, b.w, b.h, rgba);
        }
      } while (this.again);
    } finally {
      this.uploading = false;
    }
  }

  // ------------------------------------------------------------ per frame

  tick(): void {
    if (!this.started) return;
    const lang = this.app.language();
    const level = this.app.reading_level();
    if (lang !== this.lang) {
      this.lang = lang;
      void this.upload();
      if (this.shown) this.render();
    }
    if (level !== this.level) {
      this.level = level;
      if (this.shown) this.render();
    }
    if (this.carousel) {
      // auto-advance, frozen while the gate is up (ADS-031)
      if (this.carousel.tick(performance.now(), this.gate !== null)) this.renderCarousel();
      return;
    }
    const id = this.app.ad_near();
    if (!id) {
      this.dismissed = null;
      if (this.shown) this.hide();
      this.syncButton(null);
      return;
    }
    if (id !== this.shown) {
      if (this.shown) this.hide();
      if (id !== this.dismissed && this.canOpen(id)) this.open(id);
    }
    this.syncButton(id);
  }

  private canOpen(id: string): boolean {
    const board = this.boards.get(id);
    const campaign = board ? this.content?.bySlot.get(board.slot) : undefined;
    // placeholders stay passive (ADS-004); a riddle / food panel is open
    return !!board && !!campaign && this.app.panel_key() === '' && !this.carousel;
  }

  private open(id: string): void {
    this.shown = id;
    this.openedAt = performance.now();
    this.render();
  }

  /** The 🔗 button: touch only, while a verified board is near, its panel is closed and nothing else is interactable. */
  private syncButton(id: string | null): void {
    const want =
      !!id &&
      this.shown === null &&
      document.body.classList.contains('touch') &&
      this.canOpen(id) &&
      (this.app.target_kind?.() ?? '') === '';
    if (this.actBtn.hidden === want) {
      this.actBtn.hidden = !want;
      if (want) this.actBtn.setAttribute('aria-label', this.app.t('ad-link-open'));
    }
  }

  /**
   * Interact button / key: (re)opens the panel of the verified board the player stands at.
   * True if it took the press (ADS-027).
   */
  interact(): boolean {
    const id = this.app.ad_near();
    if (!this.started || !id || !this.canOpen(id) || (this.app.target_kind?.() ?? '') !== '') return false;
    if (this.shown !== id) {
      this.dismissed = null;
      if (this.shown) this.hide();
      this.open(id);
      this.syncButton(id);
    }
    return true;
  }

  private current(): { board: Board; campaign: VerifiedCampaign } | null {
    const board = this.shown ? this.boards.get(this.shown) : undefined;
    const campaign = board ? this.content?.bySlot.get(board.slot) : undefined;
    return board && campaign ? { board, campaign } : null;
  }

  // ------------------------------------------------------------ reading panel

  private render(): void {
    const cur = this.current();
    if (!cur) return;
    const { board, campaign } = cur;
    const lang = this.app.language() === 'en' ? 'en' : 'de';
    const im = pickImage(campaign, lang, board.n);
    const close = el('button', 'ad-close', '✖');
    close.className = 'ad-x';
    close.type = 'button';
    close.setAttribute('aria-label', this.app.t('ad-close'));
    close.addEventListener('click', () => {
      // an accidental touch right after the panel opened is ignored (ADS-027)
      if (performance.now() - this.openedAt < CLOSE_GUARD_MS) return;
      this.dismiss();
    });
    const body = el('div');
    body.className = 'ad-body';
    body.dataset.campaign = campaign.id;
    const img = el('img', 'ad-image') as HTMLImageElement;
    img.alt = '';
    const old = this.panel.querySelector<HTMLImageElement>('#ad-image');
    if (old?.dataset.path === im.path && old.src) img.src = old.src;
    else img.src = URL.createObjectURL(new Blob([im.data as BlobPart], { type: im.mime }));
    img.dataset.path = im.path;
    const text = el('div');
    text.className = 'ad-text';
    body.append(close, img, text);
    if (this.app.reading_level() !== 'kiga') text.append(el('p', 'ad-tagline', campaign.tagline[lang]));
    const link = el('button', 'ad-link');
    link.type = 'button';
    link.append(el('span', undefined, '🔗'), el('span', 'ad-link-text', new URL(campaign.link).hostname));
    link.addEventListener('click', () => this.startGate(campaign.link, campaign.id === 'abcsmash'));
    link.setAttribute('aria-label', this.app.t('ad-link-open'));
    text.append(link);
    this.panel.replaceChildren(body);
    this.panel.hidden = false;
  }

  private hide(): void {
    this.closeGate();
    this.panel.hidden = true;
    this.panel.replaceChildren();
    this.shown = null;
  }

  private dismiss(): void {
    this.dismissed = this.shown;
    this.hide();
    this.syncButton(this.dismissed);
  }

  // ------------------------------------------------------------ all-done carousel (ADS-031..)

  /** Whether a carousel can be offered: at least one verified campaign (never placeholders). */
  canCarousel(): boolean {
    return carouselItems(this.content, this.app.language() === 'en' ? 'en' : 'de').length > 0;
  }

  /**
   * Opens the carousel of the verified campaigns (compass tap when everything is done).
   * False (nothing shown) when no campaign is verified.
   */
  openCarousel(): boolean {
    if (!this.started || !this.canCarousel()) return false;
    if (this.carousel) return true;
    if (this.shown) this.hide();
    this.carousel = new Carousel(carouselItems(this.content, this.lang).length, performance.now());
    this.pushModal();
    this.carouselOpenedAt = performance.now();
    this.renderCarousel();
    return true;
  }

  private closeCarousel(): void {
    if (!this.carousel) return;
    this.closeGate();
    this.carousel = null;
    this.carouselView.hidden = true;
    this.carouselView.replaceChildren();
    this.dismissed = this.app.ad_near() || null;
    this.popModalIfIdle();
  }

  private renderCarousel(): void {
    const car = this.carousel;
    if (!car) return;
    const lang = this.app.language() === 'en' ? 'en' : 'de';
    const items = carouselItems(this.content, lang);
    if (items.length === 0) {
      this.closeCarousel();
      return;
    }
    if (items.length !== car.count) {
      this.carousel = new Carousel(items.length, performance.now());
      return this.renderCarousel();
    }
    const { campaign, image } = items[car.index];
    const now = () => performance.now();
    const body = el('div');
    body.className = 'ad-body';
    body.dataset.campaign = campaign.id;
    body.dataset.index = String(car.index);
    const close = el('button', 'ad-car-close', '✖');
    close.className = 'ad-x';
    close.type = 'button';
    close.setAttribute('aria-label', this.app.t('ad-close'));
    close.addEventListener('click', () => {
      if (now() - this.carouselOpenedAt < CLOSE_GUARD_MS) return;
      this.closeCarousel();
    });
    const title = el('h2', 'ad-car-title', this.app.t('ad-carousel-title'));
    const img = el('img', 'ad-car-image') as HTMLImageElement;
    img.alt = '';
    img.draggable = false;
    img.src = URL.createObjectURL(new Blob([image.data as BlobPart], { type: image.mime }));
    img.dataset.campaign = campaign.id;
    img.addEventListener('load', () => URL.revokeObjectURL(img.src), { once: true });
    // a tap opens the parental gate; a horizontal swipe turns the page (no link then)
    let down: { x: number; y: number } | null = null;
    img.addEventListener('pointerdown', (e) => {
      down = { x: e.clientX, y: e.clientY };
    });
    img.addEventListener('pointerup', (e) => {
      if (!down) return;
      const dx = e.clientX - down.x;
      const dy = e.clientY - down.y;
      down = null;
      if (Math.abs(dx) >= 40 && Math.abs(dx) > Math.abs(dy)) {
        if (dx < 0) car.next(now());
        else car.prev(now());
        this.renderCarousel();
      } else if (Math.hypot(dx, dy) < 12) {
        car.go(car.index, now());
        this.startGate(campaign.link, campaign.id === 'abcsmash');
      }
    });
    const mk = (id: string, text: string, key: string, step: () => void): HTMLButtonElement => {
      const b = el('button', id, text);
      b.className = 'ad-car-arrow';
      b.type = 'button';
      b.setAttribute('aria-label', this.app.t(key));
      b.addEventListener('click', () => {
        step();
        this.renderCarousel();
      });
      return b;
    };
    const dots = el('div', 'ad-car-dots');
    dots.className = 'ad-car-dots';
    items.forEach((_, i) => {
      const d = el('button');
      d.className = i === car.index ? 'ad-car-dot on' : 'ad-car-dot';
      d.type = 'button';
      d.setAttribute('aria-label', String(i + 1));
      d.addEventListener('click', () => {
        car.go(i, now());
        this.renderCarousel();
      });
      dots.append(d);
    });
    const nav = el('div');
    nav.className = 'ad-car-nav';
    nav.append(
      mk('ad-car-prev', '◀', 'ad-carousel-prev', () => car.prev(now())),
      dots,
      mk('ad-car-next', '▶', 'ad-carousel-next', () => car.next(now())),
    );
    body.append(close, title, img);
    if (this.app.reading_level() !== 'kiga') body.append(el('p', 'ad-car-tagline', campaign.tagline[lang]));
    body.append(nav);
    this.carouselView.replaceChildren(body);
    this.carouselView.hidden = false;
  }

  // ------------------------------------------------------------ parental gate

  private startGate(url: string, language = false): void {
    // the reading-game campaign asks a language question (article / plural), the maths game a sum
    const gate = new ParentalGate(language ? makeLanguageGateQuestion(this.app.language()) : undefined);
    this.gate = gate;
    this.pushModal();
    const card = el('div');
    card.className = 'ad-gate-card';
    const close = el('button', 'ad-gate-close', '✖');
    close.className = 'ad-x';
    close.type = 'button';
    close.setAttribute('aria-label', this.app.t('ad-close'));
    close.addEventListener('click', () => this.closeGate());
    card.append(close, el('h2', 'ad-gate-title', this.app.t('ad-gate-title')), el('div', undefined, this.app.t('ad-gate-sum')));
    card.append(el('div', 'ad-gate-question', gate.question.prompt ?? `${gate.question.a} ${gate.question.op === '-' ? '−' : '+'} ${gate.question.b} = ?`));
    const choices = el('div');
    choices.className = 'ad-choices';
    for (const n of gate.question.options) {
      const b = el('button', undefined, gate.question.labels ? gate.question.labels[n] : String(n));
      b.className = 'ad-choice';
      b.type = 'button';
      b.addEventListener('click', () => {
        if (gate.answer(n) === 'hold') this.showHold(card, gate, url);
        else this.closeGate();
      });
      choices.append(b);
    }
    card.append(choices);
    this.gateView.replaceChildren(card);
    this.gateView.hidden = false;
  }

  private showHold(card: HTMLElement, gate: ParentalGate, url: string): void {
    const hold = el('button', 'ad-hold');
    hold.type = 'button';
    hold.append(el('span', undefined, '✋'));
    const now = () => performance.now();
    let ready = false;
    let down = false;
    let heldFrom = 0;
    const ptr = adsTelemetry.ptr;
    const frame = () => {
      const p = gate.progress(now());
      hold.style.setProperty('--p', String(p));
      ptr.holdMs = Math.round(now() - heldFrom);
      if (gate.stage === 'open') {
        // the end of the hold is a timer, not a tap: pop-up blockers (iOS Safari, Samsung Internet)
        // would stop a window.open here. So the link opens at the RELEASE of the finger (a real
        // user gesture, ADS-029/030); the ✔ shows that the hold is complete.
        this.raf = 0;
        ready = true;
        hold.classList.add('ready');
        hold.replaceChildren(el('span', undefined, '✔'));
        return;
      }
      this.raf = requestAnimationFrame(frame);
    };
    /** `up`: the finger was lifted. `cancel`: the system took the touch away (Android gesture, mouse left). */
    const end = (kind: 'up' | 'cancel') => {
      down = false;
      if (ready) {
        ready = false;
        if (kind === 'up') {
          // released after the full hold: the link opens directly (user gesture, ADS-030)
          this.openLink(url);
        } else {
          // no release gesture (pointercancel): a window.open would be blocked, so offer a real link to tap (ADS-040)
          logAdEvent('hold complete but the touch was cancelled: link button shown');
          hold.classList.remove('ready');
          card.append(this.linkAnchor(url, () => this.openedByAnchor()));
        }
        return;
      }
      gate.holdEnd();
      cancelAnimationFrame(this.raf);
      this.raf = 0;
      hold.style.setProperty('--p', '0');
    };
    hold.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      ptr.down += 1;
      ptr.last = `down ${e.pointerType}`;
      down = true;
      heldFrom = now();
      try {
        hold.setPointerCapture?.(e.pointerId);
      } catch {
        /* synthetic pointers cannot be captured */
      }
      gate.holdStart(now());
      cancelAnimationFrame(this.raf);
      this.raf = requestAnimationFrame(frame);
    });
    hold.addEventListener('pointerup', (e) => {
      ptr.up += 1;
      ptr.last = `up ${e.pointerType} after ${Math.round(now() - heldFrom)}ms`;
      end('up');
    });
    hold.addEventListener('pointercancel', (e) => {
      ptr.cancel += 1;
      ptr.last = `CANCEL ${e.pointerType} after ${Math.round(now() - heldFrom)}ms`;
      end('cancel');
    });
    // Losing the capture while the finger is still down (Android Chrome can do that) does NOT reset the
    // hold; the release (pointerup) or the cancel decides.
    hold.addEventListener('lostpointercapture', (e) => {
      ptr.lostCapture += 1;
      ptr.last = `lostcapture${down ? ' (still down)' : ''}`;
      if (down) {
        try {
          hold.setPointerCapture?.(e.pointerId);
        } catch {
          /* gone */
        }
      }
    });
    hold.addEventListener('pointerleave', (e) => {
      ptr.leave += 1;
      if (e.pointerType !== 'touch') end('cancel');
    });
    hold.addEventListener('contextmenu', (e) => {
      ptr.contextmenu += 1;
      e.preventDefault();
    });
    const hint = el('div', 'ad-gate-hint', this.app.t('ad-gate-hold'));
    card.replaceChildren(card.querySelector('.ad-x')!, el('h2', 'ad-gate-title', this.app.t('ad-gate-title')), hint, hold);
  }

  private closeGate(): void {
    cancelAnimationFrame(this.raf);
    this.raf = 0;
    this.gate = null;
    this.gateView.hidden = true;
    this.gateView.replaceChildren();
    this.popModalIfIdle();
  }

  /** Opens the link once (only reached after the gate and the full hold). */
  private openLink(url: string): void {
    this.opened.push(url);
    this.closeGate();
    let result = 'custom';
    let returned: Window | null | undefined;
    if (this.o.open) this.o.open(url);
    else {
      try {
        returned = window.open(url, '_blank', 'noopener,noreferrer');
        // with `noopener` the result is always null, so success is judged by the page being hidden by the new tab
        result = returned ? 'window' : 'null';
      } catch (e) {
        result = `threw ${String((e as Error)?.name ?? e)}`;
        logAdError(`window.open: ${result}`);
      }
    }
    const open = adsTelemetry.open;
    open.calls += 1;
    open.url = url;
    open.result = result;
    open.at = Math.round(performance.now());
    this.watchOpened(url, !!returned || result === 'custom');
    this.dismiss();
    this.closeCarousel();
  }

  /** After `window.open`: if the page was not hidden by a new tab within {@link OPEN_CHECK_MS}, the open was blocked: show a real link. */
  private watchOpened(url: string, trusted: boolean): void {
    let hidden = document.visibilityState === 'hidden';
    const onHide = () => {
      if (document.visibilityState === 'hidden') hidden = true;
    };
    document.addEventListener('visibilitychange', onHide);
    window.addEventListener('pagehide', onHide);
    window.setTimeout(() => {
      document.removeEventListener('visibilitychange', onHide);
      window.removeEventListener('pagehide', onHide);
      adsTelemetry.open.visibleAfter = !hidden;
      if (!hidden && !trusted) {
        logAdEvent('window.open did not take over the page: link button shown');
        this.showFallback(url);
      }
    }, OPEN_CHECK_MS);
  }

  /** The big anchor: a real tap on a link is always allowed (no pop-up blocker). */
  private linkAnchor(url: string, onTap: () => void): HTMLElement {
    const row = el('div');
    row.className = 'ad-open-row';
    const a = el('a', 'ad-open');
    a.href = url;
    a.target = '_blank';
    a.rel = 'noopener noreferrer';
    a.append(el('span', undefined, '🔗'), el('span', undefined, new URL(url).hostname));
    a.setAttribute('aria-label', this.app.t('ad-link-open'));
    a.addEventListener('click', () => {
      logAdEvent('link button tapped');
      adsTelemetry.open.calls += 1;
      adsTelemetry.open.result = 'anchor';
      this.opened.push(url);
      onTap();
    });
    row.append(a);
    return row;
  }

  private showFallback(url: string): void {
    const note = el('div', undefined, this.app.t('ad-link-blocked'));
    const close = el('button', 'ad-fallback-close', '✖');
    close.className = 'ad-x';
    close.type = 'button';
    close.setAttribute('aria-label', this.app.t('ad-close'));
    close.addEventListener('click', () => this.hideFallback());
    this.fallbackView.replaceChildren(note, this.linkAnchor(url, () => window.setTimeout(() => this.hideFallback(), 300)));
    this.fallbackView.style.position = 'fixed';
    this.fallbackView.append(close);
    this.fallbackView.hidden = false;
    window.clearTimeout(this.fallbackTimer);
    this.fallbackTimer = window.setTimeout(() => this.hideFallback(), 15_000);
  }

  private hideFallback(): void {
    window.clearTimeout(this.fallbackTimer);
    this.fallbackView.hidden = true;
    this.fallbackView.replaceChildren();
  }

  /** The anchor inside the gate card was tapped after a cancelled hold: the browser opens it; close the gate. */
  private openedByAnchor(): void {
    window.setTimeout(() => {
      this.closeGate();
      this.dismiss();
      this.closeCarousel();
    }, 300);
  }

  // ------------------------------------------------------------ Android back button (ADS-041)

  /** One history entry while the gate or the carousel is open, so Back closes it instead of leaving the game. */
  private pushModal(): void {
    if (this.histPushed) return;
    try {
      history.pushState({ zooAd: 1 }, '');
      this.histPushed = true;
    } catch {
      /* no history (sandboxed frame) */
    }
  }

  private popModalIfIdle(): void {
    if (!this.histPushed || this.gate || this.carousel) return;
    this.histPushed = false;
    this.ignorePop = true;
    window.setTimeout(() => (this.ignorePop = false), 800);
    try {
      history.back();
    } catch {
      this.ignorePop = false;
    }
  }

  private onPop(): void {
    if (this.ignorePop) {
      this.ignorePop = false;
      return;
    }
    if (!this.histPushed) return;
    this.histPushed = false;
    logAdEvent('back button: closed an ad layer');
    this.closeIfOpen();
    if (this.gate || this.carousel) this.pushModal();
  }
}

export { GATE_HOLD_MS };
