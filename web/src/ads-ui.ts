// Host side of the ad billboards (GAME-ADS): fills the board pictures (placeholder text or a
// verified campaign image), opens the reading panel in front of a readable board and runs the
// parental gate before the link. All content checks live in ads.ts; this file only draws.
// Every text from outside is set with `textContent` (never HTML).
import {
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
import { CREAM, renderTextTexture } from './text';

/** The subset of the WASM `App` the ads need. */
export interface AdsApp {
  ad_boards_json(): string;
  ad_near(): string;
  set_ad_texture(id: string, width: number, height: number, rgba: Uint8Array): boolean;
  language(): string;
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
#ad-image{display:block;max-width:calc(100% - 80px);max-height:16dvh;border:4px solid #3b2314;border-radius:12px;background:${CREAM}}
#ad-tagline{margin:0;font-size:max(2.6vh,18px);line-height:1.2;padding:0 70px;font-weight:bold;color:#3b2314}
#ad-link,.ad-choice,#ad-hold{touch-action:manipulation;min-height:64px;min-width:64px;border-radius:32px;border:5px solid #3b2314;background:#7cc46a;color:#3b2314;font-size:max(2.2vh,17px);font-weight:bold;padding:6px 14px;max-width:100%;box-sizing:border-box;display:inline-flex;gap:12px;align-items:center;justify-content:center;box-shadow:0 5px 0 rgba(59,35,20,.55)}
#ad-gate{position:fixed;inset:0;z-index:8;display:flex;align-items:center;justify-content:center;background:rgba(40,25,15,.6);pointer-events:auto}
.ad-gate-card{position:relative;box-sizing:border-box;width:min(92vw,620px);padding:18px;border-radius:26px;border:6px solid #3b2314;background:#fff8e7;text-align:center;display:flex;flex-direction:column;gap:12px;align-items:center;font-weight:bold;color:#3b2314;font-size:max(3.2vh,20px)}
.ad-gate-card h2{margin:0;font-size:1.2em}
#ad-gate-question{font-size:1.8em}
.ad-choices{display:grid;grid-template-columns:1fr 1fr;gap:12px;width:100%}
.ad-choice{background:#ffd65c}
#ad-hold{--p:0;width:150px;height:150px;border-radius:50%;padding:0;font-size:64px;background:conic-gradient(#e8604c calc(var(--p) * 360deg),#fff 0);touch-action:none;user-select:none}
#ad-hold span{display:flex;width:112px;height:112px;border-radius:50%;background:#ffd65c;align-items:center;justify-content:center;pointer-events:none}
#ad-hold,#ad-gate{-webkit-touch-callout:none;-webkit-user-select:none;user-select:none;touch-action:none}
#ad-open{min-height:72px;max-width:100%;box-sizing:border-box;border-radius:36px;border:5px solid #3b2314;background:#7cc46a;color:#3b2314;font-size:max(2.6vh,19px);font-weight:bold;padding:8px 22px;display:inline-flex;gap:12px;align-items:center;justify-content:center;text-decoration:none;box-shadow:0 5px 0 rgba(59,35,20,.55)}
/* small screens (GAME-PLAYER 3): a compact card that leaves the right-hand control column free */
@media (orientation:landscape) and (max-height:460px){
#ad-panel{justify-content:flex-start;padding:calc(8px + env(safe-area-inset-top)) calc(96px + env(safe-area-inset-right)) 0 calc(8px + env(safe-area-inset-left))}
.ad-body{width:auto;max-width:640px;flex-direction:row;gap:12px;padding:8px 10px;border-width:5px;border-radius:18px;text-align:left}
.ad-text{flex:1;align-items:center;margin-right:68px;gap:6px}
#ad-image{max-width:40vw;max-height:min(30dvh,110px)}
#ad-tagline{padding:0;font-size:16px}
#ad-link{min-height:64px;font-size:17px}
#ad-act{right:calc(8px + env(safe-area-inset-right));bottom:calc(8px + env(safe-area-inset-bottom));width:80px;height:80px;font-size:42px}
.ad-gate-card{padding:10px 14px;gap:6px;font-size:16px}
#ad-gate-question{font-size:1.5em}
.ad-choices{grid-template-columns:repeat(4,1fr);gap:8px}
#ad-hold{width:110px;height:110px;font-size:44px}
#ad-hold span{width:80px;height:80px}
}
@media (orientation:portrait) and (max-width:480px){
#ad-panel{padding-top:calc(152px + env(safe-area-inset-top))}
#ad-act{right:calc(8px + env(safe-area-inset-right));bottom:calc(8px + env(safe-area-inset-bottom));width:80px;height:80px;font-size:42px}
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
  private retried = false;
  readonly actBtn = el('button', 'ad-act', '🔗');
  readonly panel = el('div', 'ad-panel');
  readonly gateView = el('div', 'ad-gate');
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
    this.actBtn.className = 'round';
    this.actBtn.type = 'button';
    this.actBtn.hidden = true;
    // pointerdown like the other touch buttons: a second finger never gets a click (CAMV-019)
    this.actBtn.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      e.stopPropagation();
      this.interact();
    });
    for (const e of [this.panel, this.gateView]) {
      e.addEventListener('pointerdown', (ev) => ev.stopPropagation());
      e.addEventListener('contextmenu', (ev) => ev.preventDefault());
    }
    document.body.append(this.panel, this.gateView, this.actBtn);
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

  /** Loads the campaigns; a failed load is retried once later (slow mobile data, ADS-029). */
  private async load(): Promise<void> {
    const c = await loadAds({
      base: this.o.base ?? 'ads/',
      keys: this.o.keys,
      fetchFn: this.o.fetchFn ?? ((url, init) => fetch(url, init)),
      now: (this.o.now ?? Date.now)(),
      store: this.o.store,
    });
    if (c || this.o.keys.length === 0 || this.retried) {
      this.content = c;
      this.settled = true;
      void this.upload();
      return;
    }
    this.retried = true;
    this.settled = true; // the placeholders stay until the retry has a result
    window.setTimeout(() => void this.load(), this.o.retryMs ?? 20_000);
  }

  /** Slot → campaign id that is shown now (debug / tests). */
  state(): { settled: boolean; loaded: boolean; campaigns: Record<number, string>; panel: string | null; gate: string | null; opened: string[] } {
    const campaigns: Record<number, string> = {};
    for (const [slot, c] of this.content?.bySlot ?? []) campaigns[slot] = c.id;
    return { settled: this.settled, loaded: this.content !== null, campaigns, panel: this.shown, gate: this.gate?.stage ?? null, opened: this.opened };
  }

  get panelOpen(): boolean {
    return !this.panel.hidden;
  }

  /** Esc / ✖: closes the gate, else the panel. True if something was closed. */
  closeIfOpen(): boolean {
    if (this.gate) {
      this.closeGate();
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
        (b) => (b.width === w && b.height === h ? b : null),
        () => null,
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
          this.app.set_ad_texture(b.id, b.w, b.h, rgba);
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
    return !!board && !!campaign && this.app.panel_key() === '';
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

  // ------------------------------------------------------------ parental gate

  private startGate(url: string, language = false): void {
    // the reading-game campaign asks a language question (article / plural), the maths game a sum
    const gate = new ParentalGate(language ? makeLanguageGateQuestion(this.app.language()) : undefined);
    this.gate = gate;
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
    const frame = () => {
      const p = gate.progress(now());
      hold.style.setProperty('--p', String(p));
      if (gate.stage === 'open') {
        // the end of the hold is a timer, not a tap: pop-up blockers (iOS Safari, Samsung Internet)
        // would stop a window.open here. The child taps the open button instead (ADS-029).
        this.raf = 0;
        this.showOpen(card, url);
        return;
      }
      this.raf = requestAnimationFrame(frame);
    };
    const end = () => {
      gate.holdEnd();
      cancelAnimationFrame(this.raf);
      this.raf = 0;
      hold.style.setProperty('--p', '0');
    };
    hold.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      try {
        hold.setPointerCapture?.(e.pointerId);
      } catch {
        /* synthetic pointers cannot be captured */
      }
      gate.holdStart(now());
      cancelAnimationFrame(this.raf);
      this.raf = requestAnimationFrame(frame);
    });
    // a touch is captured by the button, so a small finger movement never leaves it; only a mouse
    // leaving the button counts as releasing
    for (const t of ['pointerup', 'pointercancel', 'lostpointercapture']) hold.addEventListener(t, end);
    hold.addEventListener('pointerleave', (e) => {
      if (e.pointerType !== 'touch') end();
    });
    hold.addEventListener('contextmenu', (e) => e.preventDefault());
    const hint = el('div', 'ad-gate-hint', this.app.t('ad-gate-hold'));
    card.replaceChildren(card.querySelector('.ad-x')!, el('h2', 'ad-gate-title', this.app.t('ad-gate-title')), hint, hold);
  }

  /** The gate is passed: a big anchor the child TAPS; its click opens the link once (a real user gesture). */
  private showOpen(card: HTMLElement, url: string): void {
    const a = el('a', 'ad-open');
    a.href = url;
    a.target = '_blank';
    a.rel = 'noopener noreferrer';
    a.append(el('span', undefined, '🔗'), el('span', undefined, new URL(url).hostname));
    a.setAttribute('aria-label', this.app.t('ad-link-open'));
    let done = false;
    // The finger that held the ✋ is still down when this button appears right under it: the click
    // that its release may synthesise must not count. Armed 300 ms after that release (ADS-029).
    let armed = false;
    const arm = () => {
      window.removeEventListener('pointerup', arm, true);
      window.removeEventListener('pointercancel', arm, true);
      window.setTimeout(() => (armed = true), 300);
    };
    window.addEventListener('pointerup', arm, true);
    window.addEventListener('pointercancel', arm, true);
    a.addEventListener('click', (e) => {
      e.preventDefault();
      if (done || !armed) return;
      done = true;
      this.closeGate();
      this.openLink(url);
    });
    card.replaceChildren(card.querySelector('.ad-x')!, el('h2', 'ad-gate-title', this.app.t('ad-gate-title')), a);
  }

  private closeGate(): void {
    cancelAnimationFrame(this.raf);
    this.raf = 0;
    this.gate = null;
    this.gateView.hidden = true;
    this.gateView.replaceChildren();
  }

  /** Opens the link once (only reached after the gate). */
  private openLink(url: string): void {
    this.opened.push(url);
    if (this.o.open) this.o.open(url);
    else window.open(url, '_blank', 'noopener,noreferrer');
    this.dismiss();
  }
}

export { GATE_HOLD_MS };
