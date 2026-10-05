// Opt-in, privacy-friendly analytics (TECH-PLATFORMS "Analytics (opt-in)", PLAT-022..027).
//  - OFF until a parent opts in (parental gate in analytics-ui.ts); nothing is loaded before:
//    no script, no `dataLayer`, no cookie, no request. The choice lives in `localStorage` `zoo.analytics`.
//  - An empty measurement id disables everything, even with a stored consent.
//  - Only the events of ALLOWED_EVENTS with their allowed params are sent; values are enum-like ids or
//    small integers (never free text, names, coordinates or save contents).
//  - The environment is injectable like in audio.ts so everything is unit-testable without a browser.

export const CONSENT_KEY = 'zoo.analytics';
export type Consent = 'granted' | 'denied' | 'unset';

/** The script the consent loads (the id is appended). */
export const GTAG_SCRIPT = 'https://www.googletagmanager.com/gtag/js?id=';

/** Active play per `zoo_play_minutes` event, and the cap of the reported minutes. */
export const PLAY_STEP_MIN = 5;
export const PLAY_MAX_MIN = 120;
/** Interval of the active-time timer (ms): one coarse tick, no per-frame code. */
export const TICK_MS = 10_000;

type ParamKind = 'id' | 'lang' | 'reading_level' | 'minutes';
const LANGS = ['de', 'en', 'fr'];
const LEVELS = ['kiga', 'klasse1', 'klasse2', 'klasse3'];
const ID_RE = /^[a-z][a-z0-9_]{0,31}$/;

/** Event name → allowed param keys with their value shape. Everything else is dropped. */
export const ALLOWED_EVENTS: Readonly<Record<string, Readonly<Record<string, ParamKind>>>> = {
  zoo_session: { app_language: 'lang', reading_level: 'reading_level' },
  zoo_play_minutes: { minutes: 'minutes' },
  level_started: { level_id: 'id' },
  level_complete: { level_id: 'id' },
  mission_complete: { animal_id: 'id' },
  night_started: {},
  all_animals_home: {},
  baby_born: { species_id: 'id' },
};

function valid(kind: ParamKind, v: unknown): boolean {
  switch (kind) {
    case 'id':
      return typeof v === 'string' && ID_RE.test(v);
    case 'lang':
      return typeof v === 'string' && LANGS.includes(v);
    case 'reading_level':
      return typeof v === 'string' && LEVELS.includes(v);
    case 'minutes':
      return typeof v === 'number' && Number.isInteger(v) && v >= 1 && v <= PLAY_MAX_MIN;
  }
}

/**
 * The event params after the allowlist: unknown keys removed; null when the event name is unknown
 * or a value has the wrong type / shape (the whole event is then dropped).
 */
export function sanitize(name: string, params: Record<string, unknown> = {}): Record<string, string | number> | null {
  if (!Object.prototype.hasOwnProperty.call(ALLOWED_EVENTS, name)) return null;
  const spec = ALLOWED_EVENTS[name];
  const out: Record<string, string | number> = {};
  for (const [k, kind] of Object.entries(spec)) {
    if (!(k in params)) return null; // every allowed param is required
    if (!valid(kind, params[k])) return null;
    out[k] = params[k] as string | number;
  }
  return out;
}

/** Everything the module needs from the browser (replaceable in tests). */
export interface AnalyticsEnv {
  storage: { getItem(k: string): string | null; setItem(k: string, v: string): void } | null;
  /** Adds the script tag (async). Called at most once, never before consent. */
  loadScript: (src: string) => void;
  /** `gtag(...)` (pushes onto `window.dataLayer`, created on first use). */
  gtag: (...args: unknown[]) => void;
  now: () => number;
  /** Removes every `_ga*` cookie (host and parent domains). */
  clearCookies: () => void;
  /** `window['ga-disable-<id>'] = on`. */
  setDisabled: (id: string, on: boolean) => void;
  /** The tab is visible. */
  visible: () => boolean;
  setInterval: (fn: () => void, ms: number) => number;
  clearInterval: (h: number) => void;
  /** Current language / reading level for `zoo_session`. */
  context: () => { language: string; readingLevel: string };
}

export class Analytics {
  private active = false;
  private loaded = false;
  private timer: number | null = null;
  private activeMs = 0;
  private reportedMin = 0;
  private lastTick = 0;
  private levelsSeen = new Set<string>();
  private sessionSent = false;
  private anonymousSent = false;

  constructor(
    private readonly env: AnalyticsEnv,
    readonly id: string,
  ) {}

  /** Analytics exists at all (non-empty measurement id). */
  get available(): boolean {
    return this.id !== '';
  }

  /** The stored choice. */
  consent(): Consent {
    try {
      const v = this.env.storage?.getItem(CONSENT_KEY);
      return v === 'granted' || v === 'denied' ? v : 'unset';
    } catch {
      return 'unset';
    }
  }

  /** Events are being sent (consent given and started). */
  get on(): boolean {
    return this.active;
  }

  /**
   * At page start: a stored consent is resumed; with NO decision yet only the anonymous, cookie-less
   * page ping runs (consent mode: storage denied, `client_storage: none`, no game events, user
   * request 2026-10-04); an explicit "denied" stays off.
   */
  init(): void {
    if (!this.available) return;
    const c = this.consent();
    if (c === 'granted') this.start();
    else if (c === 'unset') this.startAnonymous();
  }

  /** Loads gtag once with every storage denied (consent mode). */
  private load(): void {
    if (this.loaded) return;
    this.loaded = true;
    const { gtag } = this.env;
    gtag('consent', 'default', {
      analytics_storage: 'denied',
      ad_storage: 'denied',
      ad_user_data: 'denied',
      ad_personalization: 'denied',
    });
    gtag('set', 'ads_data_redaction', true);
    gtag('set', 'restricted_data_processing', true);
    this.env.loadScript(GTAG_SCRIPT + encodeURIComponent(this.id));
    gtag('js', new Date(this.env.now()));
  }

  /** The anonymous ping before any decision: page view only, no cookies, no custom events. */
  private startAnonymous(): void {
    this.load();
    this.env.setDisabled(this.id, false);
    this.env.gtag('config', this.id, {
      allow_google_signals: false,
      allow_ad_personalization_signals: false,
      client_storage: 'none',
      send_page_view: true,
      page_referrer: '',
      page_location: this.pageLocation(),
    });
    this.anonymousSent = true;
  }

  /** The parent opted in (called only after the gate). */
  grant(): void {
    if (!this.available) return;
    this.store('granted');
    this.start();
  }

  /** The parent said no, or switched it off: stop, delete the cookies. */
  deny(): void {
    this.store('denied');
    this.stop();
  }

  private store(v: Consent): void {
    try {
      this.env.storage?.setItem(CONSENT_KEY, v);
    } catch {
      /* storage blocked: the choice only lives for this page */
    }
  }

  private start(): void {
    if (this.active) return;
    this.active = true;
    const { gtag } = this.env;
    this.load();
    this.env.setDisabled(this.id, false);
    gtag('consent', 'update', { analytics_storage: 'granted' });
    gtag('config', this.id, {
      allow_google_signals: false,
      allow_ad_personalization_signals: false,
      cookie_flags: 'SameSite=Lax;Secure',
      client_storage: 'cookie',
      send_page_view: !this.anonymousSent,
      page_referrer: '',
      page_location: this.pageLocation(),
    });
    this.activeMs = 0;
    this.reportedMin = 0;
    this.lastTick = this.env.now();
    this.timer = this.env.setInterval(() => this.tick(), TICK_MS);
    if (!this.sessionSent) {
      this.sessionSent = true;
      const c = this.env.context();
      this.track('zoo_session', { app_language: c.language, reading_level: c.readingLevel });
    }
  }

  private pageLocation(): string {
    return typeof location === 'undefined' ? '' : location.origin + location.pathname;
  }

  private stop(): void {
    if (this.timer !== null) this.env.clearInterval(this.timer);
    this.timer = null;
    if (this.active || this.loaded) {
      this.env.gtag('consent', 'update', { analytics_storage: 'denied' });
      this.env.setDisabled(this.id, true);
    }
    this.active = false;
    this.env.clearCookies();
  }

  /** Counts active play: a tick while the tab is hidden does not count (PLAT-026). */
  tick(): void {
    if (!this.active) return;
    const now = this.env.now();
    const dt = Math.min(now - this.lastTick, TICK_MS * 2);
    this.lastTick = now;
    if (!this.env.visible()) return;
    this.activeMs += dt;
    const min = Math.floor(this.activeMs / 60_000);
    if (min >= this.reportedMin + PLAY_STEP_MIN && this.reportedMin < PLAY_MAX_MIN) {
      this.reportedMin = Math.min(min - (min % PLAY_STEP_MIN), PLAY_MAX_MIN);
      this.track('zoo_play_minutes', { minutes: this.reportedMin });
    }
  }

  /** Sends an allowlisted event (dropped without consent, for unknown names / params). */
  track(name: string, params: Record<string, unknown> = {}): void {
    if (!this.active) return;
    const clean = sanitize(name, params);
    if (clean) this.env.gtag('event', name, clean);
  }

  /** The level part the player stands in: `level_started` once per level and session. */
  observeLevel(levelId: string): void {
    if (!this.active || !levelId || this.levelsSeen.has(levelId)) return;
    if (!ID_RE.test(levelId)) return;
    this.levelsSeen.add(levelId);
    this.track('level_started', { level_id: levelId });
  }

  /** A message of `App.poll_events()` (type + ids only are read). */
  onGameEvent(e: { type: string; level?: string; animal?: string }): void {
    switch (e.type) {
      case 'level_complete':
        this.track('level_complete', { level_id: e.level });
        break;
      case 'mission_complete':
        this.track('mission_complete', { animal_id: e.animal });
        break;
      case 'night':
        this.track('night_started');
        break;
      case 'all_home':
        this.track('all_animals_home');
        break;
      case 'baby_born':
        this.track('baby_born', { species_id: e.animal });
        break;
    }
  }
}

interface AnalyticsWindow {
  dataLayer?: unknown[];
  [k: string]: unknown;
}

/** Removes the `_ga*` cookies on the host and every parent domain (e.g. `.letterzoo.web.app`, `.web.app`). */
export function clearGaCookies(doc: { cookie: string }, hostname: string): void {
  const names = doc.cookie
    .split(';')
    .map((c) => c.split('=')[0].trim())
    .filter((n) => n.startsWith('_ga'));
  if (names.length === 0) return;
  const labels = hostname.split('.');
  const domains = ['']; // host-only cookie
  for (let i = 0; i < labels.length - 1; i++) domains.push('; domain=' + labels.slice(i).join('.'), '; domain=.' + labels.slice(i).join('.'));
  for (const n of names) {
    for (const d of domains) doc.cookie = `${n}=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/${d}`;
  }
}

/** The real browser environment. */
export function browserAnalyticsEnv(context: AnalyticsEnv['context']): AnalyticsEnv {
  const w = window as unknown as AnalyticsWindow;
  let store: Storage | null = null;
  try {
    store = window.localStorage;
  } catch {
    store = null;
  }
  return {
    storage: store,
    loadScript: (src) => {
      const s = document.createElement('script');
      s.async = true;
      s.src = src;
      document.head.append(s);
    },
    gtag: (...args) => {
      // gtag.js needs the `arguments` object itself, not an array
      w.dataLayer = w.dataLayer ?? [];
      (function (..._a: unknown[]) {
        // eslint-disable-next-line prefer-rest-params
        w.dataLayer!.push(arguments);
      })(...args);
    },
    now: () => Date.now(),
    clearCookies: () => clearGaCookies(document, location.hostname),
    setDisabled: (id, on) => {
      w[`ga-disable-${id}`] = on;
    },
    visible: () => document.visibilityState === 'visible',
    setInterval: (fn, ms) => window.setInterval(fn, ms),
    clearInterval: (h) => window.clearInterval(h),
    context,
  };
}
