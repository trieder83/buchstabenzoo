// Sound playback of the host shell (ART-SOUND "Playback"). zoo-core decides which cue plays,
// where and how loud (`App.poll_sounds()`: cue id, final gain, pitch rate, variation number);
// this module only loads, decodes and plays the named cue with the Web Audio API.
//  - The AudioContext is created after the first user gesture (autoplay rule, ASND-007).
//  - Files are fetched lazily per group (`steps`, `ui`, `doors`, `pickups`, `animals/<species>`)
//    the first time a cue of the group is needed, or prefetched when idle after the gesture.
//  - `.ogg` is preferred, `.m4a` is the fallback (Q-212).
//  - Every failure is silent: the game works without audio (ASND-019).

/** Fired on `window` by the settings menu: `detail.on` = sound switched on / off (ASND-009). */
export const SOUND_EVENT = 'zoo-sound';

/** Groups prefetched when idle after the first gesture. */
export const PREFETCH_GROUPS = ['steps', 'ui', 'doors', 'pickups'] as const;

/** A cue played this long after it was decided is stale (the file was still loading). */
export const STALE_MS = 1500;

const LOG_LIMIT = 500;

/** One sound event of `App.poll_sounds()`. */
export interface SoundEvent {
  cue: string;
  x: number;
  z: number;
  /** Final gain (master × group × distance), decided in zoo-core. */
  g: number;
  /** Pitch rate 0.95…1.05. */
  r: number;
  /** Variation number (`v mod n` picks the file). */
  v: number;
}

export type Format = 'ogg' | 'm4a';

/** The files of one cue. */
export interface CueFiles {
  group: string;
  ogg: string[];
  m4a: string[];
}

/** One decision of the debug log (written before the cue is played). */
export interface LogEntry {
  cue: string;
  /** File chosen (`audio/…`), or null when the cue has no files. */
  file: string | null;
  gain: number;
  rate: number;
  x: number;
  z: number;
  muted: boolean;
  /** `played`, `muted`, `locked` (no user gesture yet), `missing` (no files), `loading`, `failed`. */
  result: string;
}

export interface AudioApp {
  poll_sounds(): string;
  audio_animals?(): string;
  ui_tap?(): void;
}

/** Everything the module needs from the browser (replaceable in tests). */
export interface AudioEnv {
  fetch: (url: string) => Promise<{ ok: boolean; arrayBuffer(): Promise<ArrayBuffer> }>;
  createContext: () => AudioContextLike | null;
  canPlayType: (type: string) => string;
  now: () => number;
}

/** The subset of `AudioContext` used here. */
export interface AudioContextLike {
  state: string;
  destination: unknown;
  resume(): Promise<void>;
  decodeAudioData(
    data: ArrayBuffer,
    ok: (b: AudioBufferLike) => void,
    fail: (e: unknown) => void,
  ): unknown;
  createBufferSource(): {
    buffer: AudioBufferLike | null;
    playbackRate: { value: number };
    connect(n: unknown): void;
    start(): void;
  };
  createGain(): { gain: { value: number }; connect(n: unknown): void };
}
export type AudioBufferLike = object;

const FILE_RE = /^audio\/([^/]+)\/(.+)_(\d+)\.(ogg|m4a)$/;

/** Cue → files from the asset index (`assets/index.json`); group = folder, animals by species. */
export function parseAudioIndex(index: readonly string[]): Map<string, CueFiles> {
  const found = new Map<string, { group: string; n: number; ext: Format; path: string }[]>();
  for (const path of index) {
    const m = FILE_RE.exec(path);
    if (!m) continue;
    const [, dir, cue, n, ext] = m;
    const group = dir === 'animals' ? `animals/${cue.split('_')[1] ?? ''}` : dir;
    const list = found.get(cue) ?? [];
    list.push({ group, n: Number(n), ext: ext as Format, path });
    found.set(cue, list);
  }
  const out = new Map<string, CueFiles>();
  for (const [cue, list] of found) {
    list.sort((a, b) => a.n - b.n);
    out.set(cue, {
      group: list[0].group,
      ogg: list.filter((f) => f.ext === 'ogg').map((f) => f.path),
      m4a: list.filter((f) => f.ext === 'm4a').map((f) => f.path),
    });
  }
  return out;
}

/** `.ogg` when the browser can play Vorbis, else `.m4a` (Safari before 17.4, Q-212). */
export function pickFormat(canPlayType: (type: string) => string): Format {
  try {
    return canPlayType('audio/ogg; codecs="vorbis"') !== '' ? 'ogg' : 'm4a';
  } catch {
    return 'm4a';
  }
}

/** Variation `v mod n`, never the same as the last one when there is a choice. */
export function pickVariation(v: number, n: number, last: number): number {
  if (n <= 0) return 0;
  const k = v % n;
  return n > 1 && k === last ? (k + 1) % n : k;
}

function browserEnv(): AudioEnv {
  return {
    fetch: (url) => fetch(url),
    createContext: () => {
      const w = window as unknown as Record<string, unknown>;
      const Ctor = (w.AudioContext ?? w.webkitAudioContext) as (new () => AudioContextLike) | undefined;
      return Ctor ? new Ctor() : null;
    },
    canPlayType: (t) => new Audio().canPlayType(t),
    now: () => performance.now(),
  };
}

export class GameAudio {
  /** Sound switch (settings, ASND-009). */
  enabled = true;
  /** Every cue decision, before playing (debug / e2e). */
  readonly log: LogEntry[] = [];
  /** Every audio URL requested so far (debug / e2e, ASND-007). */
  readonly fetched: string[] = [];

  private readonly cues: Map<string, CueFiles>;
  private readonly env: AudioEnv;
  private format: Format;
  private ctx: AudioContextLike | null = null;
  private unlocked = false;
  private readonly groups = new Map<string, Promise<Map<string, AudioBufferLike[]>>>();
  private readonly buffers = new Map<string, AudioBufferLike[]>();
  private readonly lastVariant = new Map<string, number>();
  private lastPrefetch = 0;
  private prefetching = false;

  constructor(index: readonly string[], env: AudioEnv = browserEnv()) {
    this.env = env;
    this.cues = parseAudioIndex(index);
    this.format = pickFormat(env.canPlayType);
  }

  /** Whether a user gesture has unlocked the audio. */
  get isUnlocked(): boolean {
    return this.unlocked;
  }

  /** `running`, `suspended`, … of the AudioContext; `none` before the gesture / without Web Audio. */
  get state(): string {
    return this.ctx?.state ?? 'none';
  }

  setEnabled(on: boolean): void {
    this.enabled = on;
  }

  /** Listens for the first gesture (`pointerdown` / `keydown`) on `target`. */
  attach(target: EventTarget = window): void {
    const gesture = () => {
      this.unlock();
      if (this.unlocked && this.state === 'running') {
        for (const ev of ['pointerdown', 'pointerup', 'keydown']) target.removeEventListener(ev, gesture, true);
      }
    };
    for (const ev of ['pointerdown', 'pointerup', 'keydown']) target.addEventListener(ev, gesture, true);
  }

  /** Creates / resumes the AudioContext (only ever called from a user gesture). */
  unlock(): void {
    try {
      if (!this.ctx) this.ctx = this.env.createContext();
      if (!this.ctx) return;
      if (!this.unlocked) this.lastPrefetch = this.env.now(); // idle prefetch starts 2 s later
      this.unlocked = true;
      if (this.ctx.state !== 'running') void this.ctx.resume().catch(() => undefined);
    } catch {
      // no Web Audio: silent
    }
  }

  /** Plays the events of one frame (`App.poll_sounds()`), then prefetches when idle. */
  tick(app: AudioApp): void {
    let raw = '';
    try {
      raw = app.poll_sounds();
    } catch {
      return;
    }
    if (raw) {
      let events: SoundEvent[] = [];
      try {
        events = JSON.parse(raw) as SoundEvent[];
      } catch {
        // ignore a malformed batch
      }
      for (const e of events) this.play(e);
    }
    const now = this.env.now();
    if (this.unlocked && now - this.lastPrefetch > 2000) {
      this.lastPrefetch = now;
      this.prefetch(app);
    }
  }

  /** Prefetches the common groups and the animal calls of the unlocked levels, one at a time. */
  prefetch(app?: AudioApp): void {
    if (!this.unlocked || !this.enabled) return;
    const groups: string[] = [...PREFETCH_GROUPS];
    try {
      for (const id of JSON.parse(app?.audio_animals?.() ?? '[]') as string[]) groups.push(`animals/${id}`);
    } catch {
      // no animals list: only the common groups
    }
    const todo = groups.filter((g) => !this.groups.has(g) && this.groupCues(g).length > 0);
    // one group after the other (never a burst of downloads)
    if (this.prefetching || !todo.length) return;
    this.prefetching = true;
    void (async () => {
      for (const g of todo) await this.load(g);
    })().finally(() => {
      this.prefetching = false;
    });
  }

  /** Decides, logs and (when possible) plays one sound event. */
  play(e: SoundEvent): LogEntry {
    const files = this.cues.get(e.cue);
    const list = files ? this.filesOf(files) : [];
    const k = list.length ? pickVariation(e.v, list.length, this.lastVariant.get(e.cue) ?? -1) : 0;
    const entry: LogEntry = {
      cue: e.cue,
      file: list[k] ?? null,
      gain: e.g,
      rate: e.r,
      x: e.x,
      z: e.z,
      muted: !this.enabled,
      result: 'played',
    };
    if (!files || !list.length) entry.result = 'missing';
    else if (!this.enabled) entry.result = 'muted';
    else if (!this.unlocked || !this.ctx) entry.result = 'locked';
    else this.lastVariant.set(e.cue, k);
    if (this.log.push(entry) > LOG_LIMIT) this.log.shift();
    if (entry.result !== 'played' || !files) return entry;

    const buffers = this.buffers.get(e.cue);
    if (buffers?.length) {
      this.start(buffers[k % buffers.length], e);
    } else {
      entry.result = 'loading';
      const t0 = this.env.now();
      void this.load(files.group).then((all) => {
        const b = all.get(e.cue);
        if (b?.length && this.env.now() - t0 <= STALE_MS && this.enabled) this.start(b[k % b.length], e);
      });
    }
    return entry;
  }

  private filesOf(f: CueFiles): string[] {
    const preferred = this.format === 'ogg' ? f.ogg : f.m4a;
    return preferred.length ? preferred : this.format === 'ogg' ? f.m4a : f.ogg;
  }

  private groupCues(group: string): [string, CueFiles][] {
    return [...this.cues].filter(([, f]) => f.group === group);
  }

  /** Fetches and decodes every file of a group once; failures end up as an empty group. */
  private load(group: string): Promise<Map<string, AudioBufferLike[]>> {
    const cached = this.groups.get(group);
    if (cached) return cached;
    const p = this.loadGroup(group).catch(() => new Map<string, AudioBufferLike[]>());
    this.groups.set(group, p);
    return p;
  }

  private async loadGroup(group: string): Promise<Map<string, AudioBufferLike[]>> {
    const ctx = this.ctx;
    const out = new Map<string, AudioBufferLike[]>();
    if (!ctx) return out;
    for (const [cue, f] of this.groupCues(group)) {
      const buffers: AudioBufferLike[] = [];
      for (const path of this.filesOf(f)) {
        const b = (await this.decode(ctx, path)) ?? (await this.decode(ctx, this.otherFormat(path, f)));
        if (b) buffers.push(b);
      }
      if (buffers.length) {
        out.set(cue, buffers);
        this.buffers.set(cue, buffers);
      }
    }
    return out;
  }

  /** The same file in the other format (only when it exists). */
  private otherFormat(path: string, f: CueFiles): string {
    const i = this.filesOf(f).indexOf(path);
    const other = this.filesOf(f) === f.ogg ? f.m4a : f.ogg;
    return other[i] ?? '';
  }

  private async decode(ctx: AudioContextLike, path: string): Promise<AudioBufferLike | null> {
    if (!path) return null;
    try {
      this.fetched.push(path);
      const res = await this.env.fetch(`assets/${path}`);
      if (!res.ok) return null;
      const data = await res.arrayBuffer();
      return await new Promise<AudioBufferLike>((ok, fail) => {
        const r = ctx.decodeAudioData(data, ok, fail) as Promise<AudioBufferLike> | undefined;
        r?.then?.(ok, fail); // promise form (modern browsers)
      });
    } catch {
      return null;
    }
  }

  private start(buffer: AudioBufferLike, e: SoundEvent): void {
    const ctx = this.ctx;
    if (!ctx) return;
    try {
      const src = ctx.createBufferSource();
      const gain = ctx.createGain();
      src.buffer = buffer;
      src.playbackRate.value = e.r;
      gain.gain.value = e.g;
      src.connect(gain);
      gain.connect(ctx.destination);
      src.start();
    } catch {
      // playback failed: silent
    }
  }
}

/** Plays `ui_tap` on a tap of the settings menu (event delegation; the cue comes from the core). */
export function attachUiTaps(app: AudioApp, root: Document = document): void {
  root.addEventListener(
    'pointerdown',
    (ev) => {
      const t = ev.target as Element | null;
      if (t?.closest?.('#settings-btn, #settings button')) app.ui_tap?.();
    },
    true,
  );
}
