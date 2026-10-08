// Anonymous first-party counters (TECH-PLATFORMS "Anonymous counters", PLAT-044..053).
//  - Counts HOW OFTEN something happens, nothing else: one Firestore document per
//    day + event + param + platform + language + version, with the single number `n`.
//  - NO identifier of any kind: no cookie, no localStorage / sessionStorage, no user / device / session id,
//    no timestamp finer than the UTC day, no free text. The IP address is seen by Google's servers like in
//    every request but is not stored by us.
//  - Off with Do-Not-Track / Global Privacy Control, `?nocount=1`, automation (`navigator.webdriver`) unless
//    `?count=1`, and on dev hosts (localhost, private networks, *.local) unless `?count=1`.
//  - Never blocks the game: a small in-memory queue, flushed every ~10 s and when the page is hidden; errors
//    are swallowed, a failed batch is dropped (no retries).
//  - The allowlist below is the SAME list as `firestore.rules` (checked by a unit test).

import { COUNTER_API_KEY, COUNTER_PROJECT } from './counter-config';

export type ParamKind = 'none' | 'id' | 'slot';

/** Event → shape of its `param` (the table in specs/40-tech/platforms-and-testing.md). */
export const COUNTER_EVENTS: Readonly<Record<string, ParamKind>> = {
  // app / progress / difficulty
  session_start: 'none',
  level_started: 'id',
  level_completed: 'id',
  mission_complete: 'id',
  all_animals_home: 'none',
  baby_born: 'id',
  night_started: 'none',
  sleep_started: 'none',
  hint_used: 'id',
  key_note_read: 'none',
  key_box_opened: 'none',
  lock_wrong: 'none',
  lock_ok: 'none',
  cart_boarded: 'none',
  cart_locked_tap: 'id',
  cart_park_refused: 'none',
  // ad billboards (param = slot + link type, e.g. s1_ios)
  board_panel_opened: 'slot',
  board_link_tapped: 'slot',
  gate_answer_right: 'slot',
  gate_answer_wrong: 'slot',
  gate_hold_started: 'slot',
  gate_hold_complete: 'slot',
  gate_hold_cancelled: 'slot',
  board_link_opened: 'slot',
  board_link_blocked_fallback: 'slot',
  board_rescue_dialog: 'slot',
  board_blocked_detected: 'id',
  carousel_opened: 'none',
  carousel_link_opened: 'slot',
};

export const FLUSH_MS = 10_000;
/** Per event and minute at most this many counts are queued (spam guard). */
export const MAX_PER_MINUTE = 30;
export const MAX_QUEUE = 200;
/** Writes per commit (each document at most once per commit, see {@link Counter.flush}). */
export const MAX_BATCH = 20;

const ID_RE = /^[a-z0-9][a-z0-9_-]{0,31}$/;
const SLOT_RE = /^s[1-3]_(ios|android|web)$/;
const VERSION_RE = /^[a-z0-9][a-z0-9.-]{0,15}$/;

/** Whether `param` has the shape the event allows ('' = no param). */
export function paramOk(kind: ParamKind, param: string): boolean {
  switch (kind) {
    case 'none':
      return param === '';
    case 'id':
      return ID_RE.test(param);
    case 'slot':
      return SLOT_RE.test(param);
  }
}

/** The board param: `s<slot>_<ios|android|web>` from the slot and the link actually used. */
export function slotParam(slot: number, url: string): string {
  let kind = 'web';
  try {
    const h = new URL(url).hostname;
    if (h === 'apps.apple.com') kind = 'ios';
    else if (h === 'play.google.com') kind = 'android';
  } catch {
    /* keep web */
  }
  return `s${slot}_${kind}`;
}

export interface CounterEnv {
  /** Counting allowed at all (see {@link countersAllowed}). */
  allowed: boolean;
  platform: 'web' | 'ios' | 'android';
  language: () => string;
  version: string;
  now: () => number;
  fetch: (url: string, init: { method: 'POST'; keepalive: true; headers: Record<string, string>; body: string }) => Promise<unknown>;
  setTimeout: (fn: () => void, ms: number) => unknown;
}

export interface CounterSwitches {
  dnt?: string | null;
  gpc?: boolean;
  webdriver?: boolean;
  hostname: string;
  search: string;
}

const DEV_HOST = /^(localhost|127\.|10\.|192\.168\.|172\.(1[6-9]|2\d|3[01])\.|0\.0\.0\.0$|\[?::1\]?$)|\.local$|\.localhost$/;

/** The off switches (PLAT-047): DNT / GPC, `?nocount=1`, automation and dev hosts unless `?count=1`. */
export function countersAllowed(s: CounterSwitches): boolean {
  if (s.dnt === '1' || s.dnt === 'yes' || s.gpc === true) return false;
  const q = new URLSearchParams(s.search);
  if (q.get('nocount') === '1') return false;
  if (q.get('count') === '1') return true;
  if (s.webdriver === true) return false;
  return !DEV_HOST.test(s.hostname);
}

interface Pending {
  event: string;
  param: string;
  day: string;
  lang: string;
}

/** `yyyymmdd` of the UTC day. */
export function utcDay(ms: number): string {
  const d = new Date(ms);
  const p = (n: number) => String(n).padStart(2, '0');
  return `${d.getUTCFullYear()}${p(d.getUTCMonth() + 1)}${p(d.getUTCDate())}`;
}

export class Counter {
  private queue: Pending[] = [];
  private timer = false;
  private minute = 0;
  private perMinute = new Map<string, number>();

  constructor(private readonly env: CounterEnv) {}

  /** Counts one occurrence. Unknown events / wrong param shapes are dropped. Never throws. */
  count(event: string, param = ''): void {
    try {
      if (!this.env.allowed) return;
      if (!Object.prototype.hasOwnProperty.call(COUNTER_EVENTS, event)) return;
      if (!paramOk(COUNTER_EVENTS[event], param)) return;
      const now = this.env.now();
      const m = Math.floor(now / 60_000);
      if (m !== this.minute) {
        this.minute = m;
        this.perMinute.clear();
      }
      const c = (this.perMinute.get(event) ?? 0) + 1;
      this.perMinute.set(event, c);
      if (c > MAX_PER_MINUTE || this.queue.length >= MAX_QUEUE) return;
      this.queue.push({ event, param, day: utcDay(now), lang: this.env.language() === 'en' ? 'en' : 'de' });
      this.schedule(FLUSH_MS);
    } catch {
      /* counting never disturbs the game */
    }
  }

  get pending(): number {
    return this.queue.length;
  }

  private schedule(ms: number): void {
    if (this.timer) return;
    this.timer = true;
    this.env.setTimeout(() => {
      this.timer = false;
      this.flush();
    }, ms);
  }

  /** The id of the document (also checked by `firestore.rules`). */
  docId(p: Pending): string {
    return `${p.day}_${p.event}_${p.param}_${this.env.platform}_${p.lang}_${this.env.version}`;
  }

  /**
   * Sends one commit. Each document appears at most once per commit (the rules allow `n + 1` only), repeats
   * wait for the next flush (1 s later). A failed commit is dropped.
   */
  flush(): void {
    try {
      if (!this.env.allowed || this.queue.length === 0) return;
      const seen = new Set<string>();
      const take: Pending[] = [];
      const rest: Pending[] = [];
      for (const p of this.queue) {
        const id = this.docId(p);
        if (!seen.has(id) && take.length < MAX_BATCH) {
          seen.add(id);
          take.push(p);
        } else rest.push(p);
      }
      this.queue = rest;
      if (rest.length > 0) this.schedule(1000);
      const base = `projects/${COUNTER_PROJECT}/databases/(default)/documents`;
      const writes = take.map((p) => ({
        update: {
          name: `${base}/c/${this.docId(p)}`,
          fields: {
            date: { stringValue: p.day },
            event: { stringValue: p.event },
            param: { stringValue: p.param },
            platform: { stringValue: this.env.platform },
            lang: { stringValue: p.lang },
            v: { stringValue: this.env.version },
          },
        },
        updateMask: { fieldPaths: ['date', 'event', 'param', 'platform', 'lang', 'v'] },
        updateTransforms: [{ fieldPath: 'n', increment: { integerValue: '1' } }],
      }));
      const url = `https://firestore.googleapis.com/v1/${base}:commit?key=${COUNTER_API_KEY}`;
      void Promise.resolve(
        this.env.fetch(url, { method: 'POST', keepalive: true, headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ writes }) }),
      ).catch(() => {});
    } catch {
      /* dropped */
    }
  }
}

// ---------------------------------------------------------------- the page singleton

let instance: Counter | null = null;

/** Short build id: lower-case `[a-z0-9.-]`, at most 16 characters; 'dev' when unusable. */
export function cleanVersion(v: string): string {
  const s = v.toLowerCase().replace(/[^a-z0-9.-]/g, '').slice(0, 16);
  return VERSION_RE.test(s) ? s : 'dev';
}

/** Starts the counters of this page (once, from main.ts). Safe in the native build and in tests. */
export function initCounter(language: () => string, platform: CounterEnv['platform'], version: string): Counter {
  const nav = navigator as Navigator & { globalPrivacyControl?: boolean };
  const allowed = countersAllowed({
    dnt: nav.doNotTrack ?? (window as unknown as { doNotTrack?: string }).doNotTrack ?? null,
    gpc: nav.globalPrivacyControl === true,
    webdriver: nav.webdriver === true,
    hostname: location.hostname,
    search: location.search,
  });
  instance = new Counter({
    allowed,
    platform,
    language,
    version: cleanVersion(version),
    now: () => Date.now(),
    fetch: (url, init) => fetch(url, init),
    setTimeout: (fn, ms) => window.setTimeout(fn, ms),
  });
  const flush = () => instance?.flush();
  document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'hidden') flush();
  });
  window.addEventListener('pagehide', flush);
  instance.count('session_start');
  return instance;
}

/** Counts an event (no-op before {@link initCounter}). */
export function count(event: string, param = ''): void {
  instance?.count(event, param);
}

/** The counters for the game event messages of `App.poll_events()` (type + ids only). */
export function countGameEvent(e: { type: string; level?: string; animal?: string; panel?: { kind?: string } }): void {
  switch (e.type) {
    case 'level_complete':
      count('level_completed', e.level ?? '');
      break;
    case 'mission_complete':
      count('mission_complete', e.animal ?? '');
      break;
    case 'night':
      count('night_started');
      break;
    case 'sleep':
      count('sleep_started');
      break;
    case 'all_home':
      count('all_animals_home');
      break;
    case 'baby_born':
      count('baby_born', e.animal ?? '');
      break;
    case 'key_box_opened':
      count('key_box_opened');
      break;
    case 'panel_open':
      if (e.panel?.kind === 'note') count('key_note_read');
      break;
  }
}

const levelsSeen = new Set<string>();
/** `level_started` once per level and session. */
export function countLevel(levelId: string): void {
  if (!levelId || levelsSeen.has(levelId)) return;
  levelsSeen.add(levelId);
  count('level_started', levelId);
}

/** The result of a lock panel code (`enter_code`). */
export function countLockResult(r: string): void {
  if (r === 'wrong') count('lock_wrong');
  else if (r === 'right') count('lock_ok');
}
