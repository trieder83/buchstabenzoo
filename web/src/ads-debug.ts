// Field diagnostics of the ad boards (GAME-ADS ADS-038): `?adsdebug=1` (alias `?boarddebug=1`) shows a small panel with the live
// ad state; `window.__zoo.adsDebug()` returns the same data as JSON. Works in release builds. Never
// makes a request, stores nothing and sends nothing: the data stays on the device (a "copy" button
// puts it on the clipboard so a tester can paste it into a message).
//
// The recording itself (`adsTelemetry`) is always on (a few counters, no cost per frame); only the
// overlay needs the URL parameter. No imports from ads.ts / ads-ui.ts (they import this file).

export type LoadState = 'idle' | 'loading' | 'ok' | 'failed';

export interface ImageTel {
  state: 'loading' | 'ok' | 'failed';
  bytes?: number;
  ms?: number;
  error?: string;
  /** Decoded and drawn into the board texture. */
  decoded?: boolean;
}

export interface Telemetry {
  attempts: number;
  manifest: { state: LoadState; ms?: number; bytes?: number; error?: string; version?: number };
  sig: { ok: boolean | null; verifier: 'subtle' | 'js' | null; ms?: number; error?: string; tries?: string[] };
  images: Record<string, ImageTel>;
  /** `set_ad_texture` results: board id → true (uploaded) / false. */
  uploads: Record<string, boolean>;
  ptr: { down: number; up: number; cancel: number; lostCapture: number; contextmenu: number; leave: number; last: string; holdMs: number };
  open: { calls: number; url?: string; result?: string; visibleAfter?: boolean; at?: number };
  events: string[];
  errors: string[];
  glLost: number;
  /** Suspected interference by the browser / an ad blocker (ADS-044): what was blocked or hidden, with the URL / element. */
  blocked: string[];
  /** The visibility check of each view after it was shown (ADS-044). */
  views: Record<string, { display: string; width: number; height: number; ok: boolean; rescued?: boolean }>;
}

export const adsTelemetry: Telemetry = {
  attempts: 0,
  manifest: { state: 'idle' },
  sig: { ok: null, verifier: null },
  images: {},
  uploads: {},
  ptr: { down: 0, up: 0, cancel: 0, lostCapture: 0, contextmenu: 0, leave: 0, last: '', holdMs: 0 },
  open: { calls: 0 },
  events: [],
  errors: [],
  glLost: 0,
  blocked: [],
  views: {},
};

const MAX_ERRORS = 20;

export function adsDebugEnabled(search: string): boolean {
  const q = new URLSearchParams(search);
  return q.get('adsdebug') === '1' || q.get('boarddebug') === '1';
}

export const BLOCKED_VERDICT = 'blocked by browser/ad blocker (suspected)';

/** Records one suspected block (a request that failed at network level, an element hidden by the browser); keeps the last 20. */
export function logBlocked(what: string): void {
  const line = `${Math.round(performance.now())}ms ${what}`.slice(0, 300);
  if (!adsTelemetry.blocked.includes(line)) adsTelemetry.blocked.push(line);
  if (adsTelemetry.blocked.length > MAX_ERRORS) adsTelemetry.blocked.shift();
  logAdError(`${BLOCKED_VERDICT}: ${what}`);
}

export function logAdError(msg: string): void {
  adsTelemetry.errors.push(`${Math.round(performance.now())}ms ${msg}`.slice(0, 300));
  if (adsTelemetry.errors.length > MAX_ERRORS) adsTelemetry.errors.shift();
}

export function logAdEvent(msg: string): void {
  adsTelemetry.events.push(`${Math.round(performance.now())}ms ${msg}`.slice(0, 200));
  if (adsTelemetry.events.length > 30) adsTelemetry.events.shift();
}

let captured = false;
/** Records uncaught errors and rejected promises (last 20) and WebGL context loss; idempotent. */
export function installErrorCapture(): void {
  if (captured || typeof window === 'undefined') return;
  captured = true;
  window.addEventListener('error', (e) => logAdError(`error: ${e.message} @${(e.filename ?? '').split('/').pop()}:${e.lineno}`));
  window.addEventListener('unhandledrejection', (e) => logAdError(`rejection: ${String((e.reason as Error | undefined)?.message ?? e.reason)}`));
  document.addEventListener(
    'webglcontextlost',
    () => {
      adsTelemetry.glLost += 1;
      logAdError('webglcontextlost');
    },
    true,
  );
}

export interface GlCaps {
  version: string;
  maxTextureSize: number;
  renderer: string;
  vendor: string;
}

let capsCache: GlCaps | null = null;
/** WebGL limits, read once from a throw-away canvas (released at once). */
export function glCaps(): GlCaps {
  if (capsCache) return capsCache;
  const out: GlCaps = { version: 'none', maxTextureSize: 0, renderer: '', vendor: '' };
  try {
    const c = document.createElement('canvas');
    const gl = (c.getContext('webgl2') ?? c.getContext('webgl')) as WebGL2RenderingContext | WebGLRenderingContext | null;
    if (gl) {
      out.version = String(gl.getParameter(gl.VERSION));
      out.maxTextureSize = Number(gl.getParameter(gl.MAX_TEXTURE_SIZE));
      const ext = gl.getExtension('WEBGL_debug_renderer_info');
      if (ext) {
        out.renderer = String(gl.getParameter(ext.UNMASKED_RENDERER_WEBGL));
        out.vendor = String(gl.getParameter(ext.UNMASKED_VENDOR_WEBGL));
      }
      gl.getExtension('WEBGL_lose_context')?.loseContext();
    }
  } catch (e) {
    out.version = `error: ${String(e)}`;
  }
  capsCache = out;
  return out;
}

/** Which newer JS / CSS features this browser has (old Samsung Internet = old Chromium); `false` entries explain a failure. */
export function featureReport(): Record<string, boolean> {
  const css = (p: string, v: string) => typeof CSS !== 'undefined' && !!CSS.supports?.(p, v);
  const t = (f: () => unknown): boolean => {
    try {
      return !!f();
    } catch {
      return false;
    }
  };
  return {
    bigint: typeof BigInt === 'function',
    subtleDigest: t(() => globalThis.crypto?.subtle?.digest),
    createImageBitmap: typeof createImageBitmap === 'function',
    arrayAt: t(() => [1].at?.(0) === 1),
    objectHasOwn: typeof Object.hasOwn === 'function',
    structuredClone: typeof structuredClone === 'function',
    dialogShowModal: t(() => typeof HTMLDialogElement !== 'undefined' && typeof HTMLDialogElement.prototype.showModal === 'function'),
    cssInset: css('inset', '0'),
    cssDvh: css('height', '1dvh'),
    cssAspectRatio: css('aspect-ratio', '1/1'),
    cssHas: t(() => CSS.supports('selector(:has(a))')),
    cssConic: css('background', 'conic-gradient(red, blue)'),
    pointerEvents: typeof PointerEvent === 'function',
  };
}

export function envInfo(): Record<string, unknown> {
  const n = navigator as Navigator & { userAgentData?: { brands?: { brand: string; version: string }[]; mobile?: boolean }; connection?: { effectiveType?: string; saveData?: boolean } };
  return {
    ua: navigator.userAgent,
    brands: n.userAgentData?.brands?.map((b) => `${b.brand} ${b.version}`) ?? null,
    secureContext: window.isSecureContext,
    subtle: !!globalThis.crypto?.subtle,
    origin: location.origin,
    framed: window.self !== window.top,
    online: navigator.onLine,
    connection: n.connection?.effectiveType ?? null,
    saveData: n.connection?.saveData ?? null,
    dpr: window.devicePixelRatio,
    viewport: `${window.innerWidth}x${window.innerHeight}`,
    visualViewport: window.visualViewport ? `${Math.round(window.visualViewport.width)}x${Math.round(window.visualViewport.height)}` : null,
    screen: `${screen.width}x${screen.height}`,
    maxTouchPoints: navigator.maxTouchPoints,
    standalone: window.matchMedia?.('(display-mode: standalone)').matches ?? false,
    lang: navigator.language,
    memoryGB: (navigator as Navigator & { deviceMemory?: number }).deviceMemory ?? null,
    cores: navigator.hardwareConcurrency,
    gl: glCaps(),
    features: featureReport(),
  };
}

/** The overlay: dismissible, big text, selectable, with a copy button. */
export class AdsDebugOverlay {
  private readonly root = document.createElement('div');
  private readonly pre = document.createElement('pre');
  private timer = 0;

  constructor(private readonly snapshot: () => unknown) {
    const r = this.root;
    r.id = 'zb-dbg';
    r.style.cssText =
      'position:fixed;left:0;right:0;top:0;bottom:0;z-index:99;display:flex;flex-direction:column;background:rgba(15,15,25,.94);color:#e8ffe8;font:15px/1.35 monospace;padding:max(8px,env(safe-area-inset-top)) 8px 8px;box-sizing:border-box;touch-action:pan-y';
    const bar = document.createElement('div');
    bar.style.cssText = 'display:flex;gap:10px;margin-bottom:6px;flex:none';
    const btn = (id: string, label: string, fn: () => void) => {
      const b = document.createElement('button');
      b.id = id;
      b.type = 'button';
      b.textContent = label;
      b.style.cssText = 'min-height:56px;min-width:96px;font-size:18px;font-weight:bold;border-radius:10px;border:3px solid #fff;background:#2d5a3a;color:#fff';
      b.addEventListener('click', fn);
      return b;
    };
    bar.append(
      btn('zb-dbg-copy', 'copy', () => void this.copy()),
      btn('zb-dbg-close', 'close', () => this.hide()),
    );
    this.pre.style.cssText = 'flex:1;overflow:auto;margin:0;white-space:pre-wrap;word-break:break-all;user-select:text;-webkit-user-select:text;-webkit-touch-callout:default;touch-action:pan-y';
    r.append(bar, this.pre);
    // never reaches the game input
    for (const t of ['pointerdown', 'pointerup', 'touchstart', 'keydown']) r.addEventListener(t, (e) => e.stopPropagation());
  }

  show(): void {
    if (!this.root.isConnected) document.body.append(this.root);
    this.refresh();
    window.clearInterval(this.timer);
    this.timer = window.setInterval(() => this.refresh(), 700);
  }

  hide(): void {
    window.clearInterval(this.timer);
    this.root.remove();
  }

  private text(): string {
    return JSON.stringify(this.snapshot(), null, 1).replace(/[{}",]/g, '').replace(/\n\s*\n/g, '\n');
  }

  private refresh(): void {
    const sel = window.getSelection();
    if (sel && !sel.isCollapsed) return; // do not disturb a selection in progress
    this.pre.textContent = this.text();
  }

  private async copy(): Promise<void> {
    const json = JSON.stringify(this.snapshot(), null, 1);
    const btn = this.root.querySelector<HTMLButtonElement>('#zb-dbg-copy')!;
    try {
      await navigator.clipboard.writeText(json);
      btn.textContent = 'copied';
    } catch {
      // no Clipboard API (plain http, old WebView): select everything so a long press → Copy works
      const range = document.createRange();
      range.selectNodeContents(this.pre);
      const sel = window.getSelection();
      sel?.removeAllRanges();
      sel?.addRange(range);
      btn.textContent = 'selected - tap Copy';
    }
  }
}
