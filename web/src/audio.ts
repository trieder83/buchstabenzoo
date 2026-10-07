// Sound playback of the host shell (ART-SOUND "Playback"). zoo-core decides which cue plays,
// where and how loud (`App.poll_sounds()`: cue id, final gain, pitch rate, variation number);
// this module only loads, decodes and plays the named cue with the Web Audio API.
//  - The AudioContext is created after the first user gesture (autoplay rule, ASND-007).
//  - Files are fetched lazily per group (`steps`, `ui`, `doors`, `pickups`, `animals/<species>`)
//    the first time a cue of the group is needed, or prefetched when idle after the gesture.
//  - `.ogg` is preferred, `.m4a` is the fallback (Q-212).
//  - Night ambience (ART-SOUND "Ambient loops", ASND-020..027): a looping cricket bed on its own
//    gain node; zoo-core gives the target (`App.ambient_target()`), the gain follows it linearly in 3 s.
//  - Golf cart engine (ASND-034..037): two looping sources (`path`, `grass`) on their own gain nodes,
//    created lazily on the first boarding after a gesture; zoo-core gives the target level / pitch
//    / grass blend of the seated cart (`engine_*`), the host smooths them (0.2 s / 0.15 s) and fades
//    in 0.5 s / out 0.4 s. One pair of sources at most, released once faded out.
//  - Sound events may carry a delay `d` (key box cues): scheduled at `currentTime + d`.
//  - Every failure is silent: the game works without audio (ASND-019).

/** Fired on `window` by the settings menu: `detail.on` = sound switched on / off (ASND-009). */
export const SOUND_EVENT = 'zoo-sound';

/** Groups prefetched when idle after the first gesture. */
export const PREFETCH_GROUPS = ['steps', 'ui', 'doors', 'pickups'] as const;

/** A cue played this long after it was decided is stale (the file was still loading). */
export const STALE_MS = 1500;

/** Safety clamp of the ambient gain; equals `zoo_core::sound::AMBIENT_GAIN` (ASND-022). */
export const AMBIENT_GAIN_MAX = 0.05;
/** Fade in / out time of the ambient bed in seconds (full swing, ASND-023). */
export const AMBIENT_FADE_S = 3;
/** Cue id and group of the night cricket loop. */
export const AMBIENT_CUE = 'ambient_crickets';

/** Engine channel (ASND-034/036): safety clamp = `zoo_core::sound::ENGINE_GAIN_MAX`. */
export const ENGINE_GAIN_MAX = 0.28;
/** Layer factor of the grass loop = `zoo_core::sound::GRASS_GAIN_FACTOR`. */
export const GRASS_GAIN_FACTOR = 0.7;
/** Smoothing time constants (s) of the engine gain and pitch. */
export const ENGINE_GAIN_TAU_S = 0.2;
export const ENGINE_RATE_TAU_S = 0.15;
/** Fade in on boarding / out on leaving (s). */
export const ENGINE_FADE_IN_S = 0.5;
export const ENGINE_FADE_OUT_S = 0.4;
/** Cue ids of the two engine loops (group `engine`). */
export const ENGINE_PATH_CUE = 'cart_engine_path';
export const ENGINE_GRASS_CUE = 'cart_engine_grass';

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
  /** Start delay in seconds (default 0): the key box cues follow the lock sound. */
  d?: number;
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
  /** Start delay in seconds (0 = at once). */
  delay: number;
  /** `played`, `muted`, `locked` (no user gesture yet), `missing` (no files), `loading`, `failed`. */
  result: string;
}

export interface AudioApp {
  poll_sounds(): string;
  audio_animals?(): string;
  /** Target gain of the ambient bed (0 by day, 0.05 at dusk / night). */
  ambient_target?(): number;
  ui_tap?(): void;
  /** Seated golf cart: speed in m/s (abs), -1 when not seated (or paused); then `engine_gain` /
   *  `engine_rate` (the law of ASND-034) and `engine_grass` (blend 0 path .. 1 grass). */
  engine_speed?(): number;
  engine_gain?(): number;
  engine_rate?(): number;
  engine_grass?(): number;
}

/** Everything the module needs from the browser (replaceable in tests). */
export interface AudioEnv {
  fetch: (url: string) => Promise<{ ok: boolean; arrayBuffer(): Promise<ArrayBuffer> }>;
  createContext: () => AudioContextLike | null;
  canPlayType: (type: string) => string;
  now: () => number;
  /** The tab is in the background (the bed is silenced and the context suspended). */
  hidden?: () => boolean;
  /** A title / intro overlay covers the game (the bed stays silent). */
  blocked?: () => boolean;
}

/** The subset of `AudioContext` used here. */
export interface AudioContextLike {
  state: string;
  currentTime?: number;
  destination: unknown;
  resume(): Promise<void>;
  suspend?(): Promise<void>;
  decodeAudioData(
    data: ArrayBuffer,
    ok: (b: AudioBufferLike) => void,
    fail: (e: unknown) => void,
  ): unknown;
  createBufferSource(): {
    buffer: AudioBufferLike | null;
    playbackRate: { value: number };
    loop?: boolean;
    connect(n: unknown): void;
    start(when?: number): void;
    stop?(): void;
    disconnect?(): void;
  };
  createGain(): { gain: { value: number }; connect(n: unknown): void; disconnect?(): void };
}
export type AudioBufferLike = object;

interface EngineLayer {
  src: ReturnType<AudioContextLike['createBufferSource']>;
  node: ReturnType<AudioContextLike['createGain']>;
}

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
    hidden: () => document.visibilityState === 'hidden',
    blocked: () => {
      const intro = document.getElementById('intro');
      return !!intro && !intro.hidden;
    },
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
  // ambient bed
  private ambGain = 0;
  private ambTarget = 0;
  private ambLast = -1;
  private ambSrc: { stop?(): void; disconnect?(): void } | null = null;
  private ambNode: { gain: { value: number }; disconnect?(): void } | null = null;
  private ambLoading = false;
  private ambFailed = false;
  private ambSuspended = false;
  // golf cart engine channel
  private engLast = -1;
  /** Smoothed level (before the layer split and the fade) and pitch. */
  private engLevel = 0;
  private engRate = 0.75;
  private engFade = 0;
  private engGrass = 0;
  private engSeated = false;
  private engSrc: { path: EngineLayer; grass: EngineLayer } | null = null;
  private engLoading = false;
  private engFailed = false;
  /** Gain nodes of delayed cues that have not started yet (zeroed when muted meanwhile). */
  private pending: { gain: { value: number }; at: number }[] = [];

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

  /** Debug / e2e state of the ambient bed (`__zoo.audio.ambient`). */
  get ambient(): { playing: boolean; gain: number; target: number; fetched: boolean } {
    return {
      playing: this.ambSrc !== null,
      gain: this.ambGain,
      target: this.ambTarget,
      fetched: this.fetched.some((f) => f.includes(`/${AMBIENT_CUE}_`)),
    };
  }

  /** Debug / e2e state of the golf cart engine (`__zoo.audio.engine`, ASND-040). */
  get engine(): {
    playing: boolean;
    gain: number;
    rate: number;
    grass: number;
    fetched: boolean;
    /** Final gains of the two loops (path, grass). */
    layers: [number, number];
  } {
    return {
      layers: this.engSrc ? [this.engSrc.path.node.gain.value, this.engSrc.grass.node.gain.value] : [0, 0],
      playing: this.engSrc !== null,
      gain: this.engSrc ? this.engLevel * this.engFade : 0,
      rate: this.engRate,
      grass: this.engGrass,
      fetched: this.fetched.some((f) => f.includes('/engine/')),
    };
  }

  setEnabled(on: boolean): void {
    this.enabled = on;
    if (!on) this.silencePending();
  }

  /** Delayed cues that have not started yet do not play after muting. */
  private silencePending(): void {
    const t = this.ctx?.currentTime ?? 0;
    for (const p of this.pending) if (p.at > t) p.gain.value = 0;
    this.pending = [];
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
    this.tickAmbient(app, now);
    this.tickEngine(app, now);
    if (this.unlocked && now - this.lastPrefetch > 2000) {
      this.lastPrefetch = now;
      this.prefetch(app);
    }
  }

  /**
   * The night cricket bed (ASND-023..026): moves the gain linearly towards the target (0.05 at
   * dusk / night, from zoo-core) in `AMBIENT_FADE_S`, silences it at once in a hidden tab,
   * fetches the file lazily (first target > 0 after the first gesture) and keeps one source.
   */
  private tickAmbient(app: AudioApp, now: number): void {
    const dt = this.ambLast < 0 ? 0 : Math.min(0.25, Math.max(0, (now - this.ambLast) / 1000));
    this.ambLast = now;
    let raw = 0;
    try {
      raw = Number(app.ambient_target?.() ?? 0) || 0;
    } catch {
      raw = 0;
    }
    const hidden = this.env.hidden?.() ?? false;
    const blocked = this.env.blocked?.() ?? false;
    const target = this.enabled && !blocked && !hidden ? Math.min(AMBIENT_GAIN_MAX, Math.max(0, raw)) : 0;
    this.ambTarget = target;
    if (hidden) this.ambGain = 0; // silent at once
    else {
      const step = (AMBIENT_GAIN_MAX / AMBIENT_FADE_S) * dt;
      this.ambGain =
        this.ambGain < target ? Math.min(target, this.ambGain + step) : Math.max(target, this.ambGain - step);
    }
    if (!this.unlocked || !this.ctx) return;
    try {
      if (hidden && (this.ambSrc || this.engSrc) && !this.ambSuspended && this.ctx.state === 'running') {
        this.ambSuspended = true;
        void this.ctx.suspend?.()?.catch(() => undefined);
      } else if (!hidden && this.ambSuspended) {
        this.ambSuspended = false;
        void this.ctx.resume().catch(() => undefined);
      }
    } catch {
      // silent
    }
    if (target > 0 && !this.ambSrc && !this.ambLoading && !this.ambFailed) this.startAmbient();
    if (this.ambSrc && this.ambGain === 0 && target === 0) this.stopAmbient();
    if (this.ambNode) this.ambNode.gain.value = this.ambGain;
  }

  /**
   * The golf cart engine (ASND-034..037). While seated (`engine_speed() >= 0`) the level, pitch
   * and grass blend come from zoo-core; this only smooths them, fades and keeps one pair of
   * looping sources. Muted / hidden tab / title overlay: fades to 0 (hidden: at once).
   */
  private tickEngine(app: AudioApp, now: number): void {
    const dt = this.engLast < 0 ? 0 : Math.min(0.25, Math.max(0, (now - this.engLast) / 1000));
    this.engLast = now;
    if (!app.engine_speed) return;
    let speed = -1;
    let level = 0;
    let rate = 0.75;
    let grass = 0;
    try {
      speed = Number(app.engine_speed()) || 0;
      if (speed >= 0) {
        level = Math.min(ENGINE_GAIN_MAX, Math.max(0, Number(app.engine_gain?.() ?? 0) || 0));
        rate = Math.min(2, Math.max(0.25, Number(app.engine_rate?.() ?? 1) || 1));
        grass = Math.min(1, Math.max(0, Number(app.engine_grass?.() ?? 0) || 0));
      }
    } catch {
      speed = -1;
    }
    const seated = speed >= 0;
    const hidden = this.env.hidden?.() ?? false;
    const blocked = this.env.blocked?.() ?? false;
    const want = seated && this.enabled && !blocked && !hidden;
    if (seated) {
      if (!this.engSeated || this.engLevel === 0) {
        // a new ride starts at the law value (the fade-in covers the start)
        this.engLevel = level;
        this.engRate = rate;
      } else {
        this.engLevel += (level - this.engLevel) * (1 - Math.exp(-dt / ENGINE_GAIN_TAU_S));
        this.engRate += (rate - this.engRate) * (1 - Math.exp(-dt / ENGINE_RATE_TAU_S));
      }
      this.engGrass = grass;
    }
    this.engSeated = seated;
    if (hidden) this.engFade = 0;
    else if (want) this.engFade = Math.min(1, this.engFade + dt / ENGINE_FADE_IN_S);
    else this.engFade = Math.max(0, this.engFade - dt / ENGINE_FADE_OUT_S);
    if (!this.unlocked || !this.ctx) return;
    if (want && !this.engSrc && !this.engLoading && !this.engFailed) this.startEngine();
    if (this.engSrc && this.engFade === 0 && !want) {
      this.stopEngine();
      return;
    }
    const e = this.engSrc;
    if (!e) return;
    const g = Math.min(ENGINE_GAIN_MAX, this.engLevel * this.engFade);
    e.path.node.gain.value = g * (1 - this.engGrass);
    e.grass.node.gain.value = g * GRASS_GAIN_FACTOR * this.engGrass;
    e.path.src.playbackRate.value = this.engRate;
    e.grass.src.playbackRate.value = this.engRate;
  }

  private startEngine(): void {
    const ctx = this.ctx;
    const pf = this.cues.get(ENGINE_PATH_CUE);
    const gf = this.cues.get(ENGINE_GRASS_CUE);
    if (!ctx || !pf || !gf) {
      this.engFailed = true; // no files: silent, never retried
      return;
    }
    const pb = this.buffers.get(ENGINE_PATH_CUE)?.[0];
    const gb = this.buffers.get(ENGINE_GRASS_CUE)?.[0];
    if (!pb || !gb) {
      this.engLoading = true;
      void this.load(pf.group).finally(() => {
        this.engLoading = false; // loaded: the next tick starts it; failed: never retried
        this.engFailed = !this.buffers.has(ENGINE_PATH_CUE) || !this.buffers.has(ENGINE_GRASS_CUE);
      });
      return;
    }
    try {
      const mk = (buffer: AudioBufferLike): EngineLayer => {
        const src = ctx.createBufferSource();
        const node = ctx.createGain();
        src.buffer = buffer;
        src.loop = true;
        src.playbackRate.value = this.engRate;
        node.gain.value = 0;
        src.connect(node);
        node.connect(ctx.destination);
        src.start();
        return { src, node };
      };
      this.engSrc = { path: mk(pb), grass: mk(gb) };
    } catch {
      this.engSrc = null; // playback failed: silent
      this.engFailed = true;
    }
  }

  private stopEngine(): void {
    const e = this.engSrc;
    this.engSrc = null;
    if (!e) return;
    for (const l of [e.path, e.grass]) {
      try {
        l.src.stop?.();
        l.src.disconnect?.();
        l.node.disconnect?.();
      } catch {
        // silent
      }
    }
  }

  private startAmbient(): void {
    const files = this.cues.get(AMBIENT_CUE);
    const ctx = this.ctx;
    if (!files || !ctx) return;
    const buffer = this.buffers.get(AMBIENT_CUE)?.[0];
    if (!buffer) {
      this.ambLoading = true;
      void this.load(files.group).finally(() => {
        this.ambLoading = false; // loaded: the next tick starts it; failed: never retried
        this.ambFailed = !this.buffers.has(AMBIENT_CUE);
      });
      return;
    }
    try {
      const src = ctx.createBufferSource();
      const node = ctx.createGain();
      src.buffer = buffer;
      src.loop = true;
      node.gain.value = this.ambGain;
      src.connect(node);
      node.connect(ctx.destination);
      src.start();
      this.ambSrc = src;
      this.ambNode = node;
    } catch {
      // playback failed: silent
    }
  }

  private stopAmbient(): void {
    try {
      this.ambSrc?.stop?.();
      this.ambSrc?.disconnect?.();
      this.ambNode?.disconnect?.();
    } catch {
      // silent
    }
    this.ambSrc = null;
    this.ambNode = null;
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
      for (const g of todo) {
        if (!this.enabled) break; // muted meanwhile: stop downloading (ASND-009)
        await this.load(g);
      }
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
      delay: Math.max(0, Number(e.d) || 0),
      result: 'played',
    };
    if (!files || !list.length) entry.result = 'missing';
    else if (!this.enabled) entry.result = 'muted';
    else if (!this.unlocked || !this.ctx) entry.result = 'locked';
    else this.lastVariant.set(e.cue, k);
    if (this.log.push(entry) > LOG_LIMIT) this.log.shift();
    if (entry.result !== 'played' || !files) return entry;

    const delay = Math.max(0, Number(e.d) || 0);
    const at = (this.ctx?.currentTime ?? 0) + delay;
    const buffers = this.buffers.get(e.cue);
    if (buffers?.length) {
      this.start(buffers[k % buffers.length], e, at);
    } else {
      entry.result = 'loading';
      const t0 = this.env.now();
      void this.load(files.group).then((all) => {
        const b = all.get(e.cue);
        if (b?.length && this.env.now() - t0 <= STALE_MS + delay * 1000 && this.enabled) {
          this.start(b[k % b.length], e, Math.max(at, this.ctx?.currentTime ?? 0));
        }
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

  private start(buffer: AudioBufferLike, e: SoundEvent, at = 0): void {
    const ctx = this.ctx;
    if (!ctx) return;
    try {
      const now = ctx.currentTime ?? 0;
      const delayed = at > now + 0.001;
      const src = ctx.createBufferSource();
      const gain = ctx.createGain();
      src.buffer = buffer;
      src.playbackRate.value = e.r;
      gain.gain.value = e.g;
      src.connect(gain);
      gain.connect(ctx.destination);
      if (delayed) {
        this.pending = this.pending.filter((p) => p.at > now);
        this.pending.push({ gain: gain.gain, at });
        src.start(at);
      } else {
        src.start();
      }
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
