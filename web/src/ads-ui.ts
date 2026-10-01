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
}

const STYLE = `
#ad-panel{position:fixed;left:0;right:0;top:0;z-index:5;display:flex;justify-content:center;pointer-events:none;padding:calc(10px + env(safe-area-inset-top)) env(safe-area-inset-right) 0 env(safe-area-inset-left)}
#ad-panel[hidden],#ad-gate[hidden]{display:none}
.ad-body{position:relative;box-sizing:border-box;width:min(96vw,900px);pointer-events:auto;padding:10px 12px 12px;border-radius:22px;border:6px solid #3b2314;background:#fff8e7;box-shadow:0 8px 0 rgba(59,35,20,.5);text-align:center;display:flex;flex-direction:column;align-items:center;gap:8px}
.ad-x{position:absolute;right:8px;top:8px;width:64px;height:64px;border-radius:50%;border:4px solid #3b2314;background:#fff;font-size:28px;z-index:1}
#ad-image{display:block;max-width:calc(100% - 80px);max-height:16dvh;border:4px solid #3b2314;border-radius:12px;background:${CREAM}}
#ad-tagline{margin:0;font-size:max(2.6vh,18px);line-height:1.2;padding:0 70px;font-weight:bold;color:#3b2314}
#ad-link,.ad-choice,#ad-hold{min-height:64px;min-width:64px;border-radius:32px;border:5px solid #3b2314;background:#7cc46a;color:#3b2314;font-size:max(2.2vh,17px);font-weight:bold;padding:6px 14px;max-width:100%;box-sizing:border-box;display:inline-flex;gap:12px;align-items:center;justify-content:center;box-shadow:0 5px 0 rgba(59,35,20,.55)}
#ad-gate{position:fixed;inset:0;z-index:8;display:flex;align-items:center;justify-content:center;background:rgba(40,25,15,.6);pointer-events:auto}
.ad-gate-card{position:relative;box-sizing:border-box;width:min(92vw,620px);padding:18px;border-radius:26px;border:6px solid #3b2314;background:#fff8e7;text-align:center;display:flex;flex-direction:column;gap:12px;align-items:center;font-weight:bold;color:#3b2314;font-size:max(3.2vh,20px)}
.ad-gate-card h2{margin:0;font-size:1.2em}
#ad-gate-question{font-size:1.8em}
.ad-choices{display:grid;grid-template-columns:1fr 1fr;gap:12px;width:100%}
.ad-choice{background:#ffd65c}
#ad-hold{--p:0;width:150px;height:150px;border-radius:50%;padding:0;font-size:64px;background:conic-gradient(#e8604c calc(var(--p) * 360deg),#fff 0);touch-action:none;user-select:none}
#ad-hold span{display:flex;width:112px;height:112px;border-radius:50%;background:#ffd65c;align-items:center;justify-content:center}
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
    document.body.append(this.panel, this.gateView);
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
    void loadAds({
      base: this.o.base ?? 'ads/',
      keys: this.o.keys,
      fetchFn: this.o.fetchFn ?? ((url, init) => fetch(url, init)),
      now: (this.o.now ?? Date.now)(),
      store: this.o.store,
    }).then((c) => {
      this.content = c;
      this.settled = true;
      void this.upload();
    });
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
      return;
    }
    if (id === this.shown || id === this.dismissed) return;
    if (this.shown) this.hide();
    const board = this.boards.get(id);
    const campaign = board ? this.content?.bySlot.get(board.slot) : undefined;
    if (!board || !campaign) return; // placeholders stay passive (ADS-004)
    if (this.app.panel_key() !== '') return; // a riddle / food panel is open
    this.shown = id;
    this.render();
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
    close.addEventListener('click', () => this.dismiss());
    const body = el('div');
    body.className = 'ad-body';
    body.dataset.campaign = campaign.id;
    const img = el('img', 'ad-image') as HTMLImageElement;
    img.alt = '';
    const old = this.panel.querySelector<HTMLImageElement>('#ad-image');
    if (old?.dataset.path === im.path && old.src) img.src = old.src;
    else img.src = URL.createObjectURL(new Blob([im.data as BlobPart], { type: im.mime }));
    img.dataset.path = im.path;
    body.append(close, img);
    if (this.app.reading_level() !== 'kiga') body.append(el('p', 'ad-tagline', campaign.tagline[lang]));
    const link = el('button', 'ad-link');
    link.type = 'button';
    link.append(el('span', undefined, '🔗'), el('span', 'ad-link-text', new URL(campaign.link).hostname));
    link.addEventListener('click', () => this.startGate(campaign.link, campaign.id === 'abcsmash'));
    body.append(link);
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
        this.closeGate();
        this.openLink(url);
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
      hold.setPointerCapture?.(e.pointerId);
      gate.holdStart(now());
      cancelAnimationFrame(this.raf);
      this.raf = requestAnimationFrame(frame);
    });
    for (const t of ['pointerup', 'pointercancel', 'lostpointercapture', 'pointerleave']) hold.addEventListener(t, end);
    const hint = el('div', 'ad-gate-hint', this.app.t('ad-gate-hold'));
    card.replaceChildren(card.querySelector('.ad-x')!, el('h2', 'ad-gate-title', this.app.t('ad-gate-title')), hint, hold);
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
