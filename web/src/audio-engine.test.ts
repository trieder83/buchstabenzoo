// ART-SOUND "Golf cart sounds": ASND-036/037 (the engine channel; fake context, fake clock).
import { describe, expect, it, vi } from 'vitest';
import {
  type AudioContextLike,
  type AudioEnv,
  ENGINE_FADE_IN_S,
  ENGINE_FADE_OUT_S,
  ENGINE_GAIN_MAX,
  GameAudio,
  GRASS_GAIN_FACTOR,
} from './audio';

const INDEX = [
  'audio/engine/cart_engine_path_1.ogg',
  'audio/engine/cart_engine_path_1.m4a',
  'audio/engine/cart_engine_grass_1.ogg',
  'audio/engine/cart_engine_grass_1.m4a',
  'audio/cart/cart_board_1.ogg',
];

interface Src {
  loop: boolean;
  started: boolean;
  stopped: boolean;
  playbackRate: { value: number };
}
class Ctx implements AudioContextLike {
  state = 'running';
  currentTime = 0;
  destination = {};
  sources: Src[] = [];
  nodes: { gain: { value: number }; disconnected: boolean }[] = [];
  suspended = 0;
  resumed = 0;
  resume() {
    this.resumed++;
    this.state = 'running';
    return Promise.resolve();
  }
  suspend() {
    this.suspended++;
    this.state = 'suspended';
    return Promise.resolve();
  }
  failDecode = false;
  decodeAudioData(_d: ArrayBuffer, ok: (b: object) => void, fail: (e: unknown) => void) {
    if (this.failDecode) fail(new Error('decode'));
    else ok({ decoded: true });
  }
  createBufferSource() {
    const s = {
      buffer: null as object | null,
      loop: false,
      started: false,
      stopped: false,
      playbackRate: { value: 1 },
      connect() {},
      start: () => (s.started = true),
      stop: () => (s.stopped = true),
    };
    this.sources.push(s);
    return s;
  }
  createGain() {
    const g = { gain: { value: 0 }, disconnected: false, connect() {}, disconnect: () => (g.disconnected = true) };
    this.nodes.push(g);
    return g;
  }
}

/** The state of the cart as zoo-core reports it (speed -1 = not seated). */
const law = (v: number) => {
  const s = Math.min(1, Math.abs(v) / 4.5);
  return { gain: 0.28 * (0.25 + 0.75 * s), rate: 0.75 + 0.55 * s };
};

function setup(over: Partial<AudioEnv> = {}, withCtx = true) {
  const ctx = new Ctx();
  const urls: string[] = [];
  const clock = { t: 0, hidden: false, blocked: false };
  const env: AudioEnv = {
    fetch: (url) => {
      urls.push(url);
      return Promise.resolve({ ok: true, arrayBuffer: () => Promise.resolve(new ArrayBuffer(4)) });
    },
    createContext: () => (withCtx ? ctx : null),
    canPlayType: () => 'probably',
    now: () => clock.t,
    hidden: () => clock.hidden,
    blocked: () => clock.blocked,
    ...over,
  };
  const audio = new GameAudio(INDEX, env);
  const cart = { speed: -1, grass: 0 };
  const app = {
    poll_sounds: () => '',
    engine_speed: () => cart.speed,
    engine_gain: () => law(cart.speed).gain,
    engine_rate: () => law(cart.speed).rate,
    engine_grass: () => cart.grass,
  };
  /** Advances `s` seconds in 50 ms frames. */
  const run = async (s: number) => {
    for (let i = 0; i < Math.round(s * 20); i++) {
      clock.t += 50;
      audio.tick(app);
      await Promise.resolve();
      await Promise.resolve();
    }
  };
  return { audio, ctx, urls, clock, cart, run };
}
const live = (ctx: Ctx) => ctx.sources.filter((s) => s.started && !s.stopped).length;
const engineUrls = (urls: string[]) => urls.filter((u) => u.includes('/engine/'));

describe('golf cart engine channel', () => {
  it('ASND-036 nothing is fetched before the first gesture and the first boarding', async () => {
    const { audio, urls, cart, run } = setup();
    await run(1); // not seated, no gesture
    expect(engineUrls(urls)).toEqual([]);
    cart.speed = 0; // seated, but no gesture yet
    await run(1);
    expect(engineUrls(urls)).toEqual([]);
    expect(audio.engine.playing).toBe(false);
    audio.unlock();
    await run(1);
    expect(audio.engine).toMatchObject({ playing: true, fetched: true });
    expect(engineUrls(urls).length).toBe(2); // path + grass, .ogg only
    cart.speed = -1;
    audio.unlock();
    await run(2);
    cart.speed = 0;
    await run(1);
    expect(engineUrls(urls).length).toBe(2); // fetched once
  });

  it('ASND-036 boarding ramps the gain up in 0.5 s, leaving ramps to 0 in 0.4 s, then releases both nodes', async () => {
    expect(ENGINE_FADE_IN_S).toBe(0.5);
    expect(ENGINE_FADE_OUT_S).toBe(0.4);
    const { audio, ctx, cart, run } = setup();
    audio.unlock();
    cart.speed = 0;
    await run(0.2); // load + start
    await run(0.1);
    const early = audio.engine.gain;
    expect(early).toBeGreaterThan(0);
    expect(early).toBeLessThan(0.07);
    await run(0.5);
    expect(audio.engine.gain).toBeCloseTo(0.07, 6); // standing: 0.28 * 0.25
    expect(ctx.sources.slice(0, 2).every((s) => s.loop)).toBe(true);
    expect(live(ctx)).toBe(2);
    cart.speed = -1; // got out
    await run(0.2);
    expect(audio.engine.gain).toBeGreaterThan(0);
    expect(audio.engine.gain).toBeLessThan(0.07);
    await run(0.4);
    expect(audio.engine.playing).toBe(false);
    expect(live(ctx)).toBe(0);
    expect(ctx.nodes.every((n) => n.disconnected)).toBe(true);
  });

  it('ASND-036 board / leave / board in quick succession keeps one pair of sources', async () => {
    const { audio, ctx, urls, cart, run } = setup();
    audio.unlock();
    for (let i = 0; i < 6; i++) {
      cart.speed = i % 2 === 0 ? 0 : -1;
      await run(0.15);
      expect(live(ctx)).toBeLessThanOrEqual(2);
    }
    cart.speed = 2;
    await run(1);
    expect(live(ctx)).toBe(2);
    expect(engineUrls(urls).length).toBe(2);
    expect(audio.engine.playing).toBe(true);
  });

  it('ASND-036 gain and pitch follow the speed with the smoothing, never above 0.28', async () => {
    const { audio, ctx, cart, run } = setup();
    audio.unlock();
    cart.speed = 0;
    await run(1);
    cart.speed = 4.5;
    await run(0.1);
    const mid = audio.engine;
    expect(mid.gain).toBeGreaterThan(0.07);
    expect(mid.gain).toBeLessThan(0.28); // smoothed, not a jump
    expect(mid.rate).toBeGreaterThan(0.75);
    expect(mid.rate).toBeLessThan(1.3);
    await run(2);
    expect(audio.engine.gain).toBeCloseTo(0.28, 2);
    expect(audio.engine.rate).toBeCloseTo(1.3, 2);
    expect(ctx.sources[0].playbackRate.value).toBeCloseTo(1.3, 2);
    cart.speed = 100; // a wrong host value is clamped
    await run(2);
    expect(audio.engine.gain).toBeLessThanOrEqual(ENGINE_GAIN_MAX + 1e-9);
    for (const n of ctx.nodes) expect(n.gain.value).toBeLessThanOrEqual(ENGINE_GAIN_MAX + 1e-9);
  });

  it('ASND-036 the layers follow the grass blend: path (1-b), grass 0.7 * b', async () => {
    expect(GRASS_GAIN_FACTOR).toBe(0.7);
    const { audio, ctx, cart, run } = setup();
    audio.unlock();
    cart.speed = 4.5;
    await run(2);
    const [path, grass] = ctx.nodes;
    expect(path.gain.value).toBeCloseTo(0.28, 2);
    expect(grass.gain.value).toBe(0);
    cart.grass = 1;
    await run(0.1);
    expect(path.gain.value).toBeCloseTo(0, 6);
    expect(grass.gain.value).toBeCloseTo(0.28 * 0.7, 2);
    cart.grass = 0.5;
    await run(0.1);
    expect(path.gain.value).toBeCloseTo(0.14, 2);
    expect(grass.gain.value).toBeCloseTo(0.098, 2);
    void audio;
  });

  it('ASND-037 the sound switch off fades to 0 in 0.4 s and releases; on again brings it back', async () => {
    const { audio, ctx, cart, run } = setup();
    audio.unlock();
    cart.speed = 3;
    await run(1.5);
    const g0 = audio.engine.gain;
    expect(g0).toBeGreaterThan(0.1);
    audio.setEnabled(false);
    await run(0.2);
    expect(audio.engine.gain).toBeLessThan(g0);
    expect(audio.engine.gain).toBeGreaterThan(0);
    await run(0.4);
    expect(audio.engine.playing).toBe(false);
    expect(live(ctx)).toBe(0);
    audio.setEnabled(true);
    await run(1);
    expect(audio.engine.playing).toBe(true);
    expect(live(ctx)).toBe(2);
  });

  it('ASND-037 a hidden tab silences at once and suspends the context; visible resumes', async () => {
    const { audio, ctx, clock, cart, run } = setup();
    audio.unlock();
    cart.speed = 3;
    await run(1.5);
    clock.hidden = true;
    await run(0.1);
    expect(audio.engine.gain).toBe(0);
    expect(ctx.suspended).toBeGreaterThanOrEqual(1);
    clock.hidden = false;
    await run(1);
    expect(ctx.resumed).toBeGreaterThanOrEqual(1);
    expect(audio.engine.gain).toBeGreaterThan(0);
    expect(live(ctx)).toBe(2);
  });

  it('ASND-037 no AudioContext / a failing fetch / a failing decode: no throw, no console output', async () => {
    const spies = ['log', 'warn', 'error'].map((m) => vi.spyOn(console, m as 'log').mockImplementation(() => undefined));
    try {
      const a = setup({}, false);
      a.audio.unlock();
      a.cart.speed = 3;
      await a.run(1);
      expect(a.audio.engine.playing).toBe(false);
      const b = setup({ fetch: () => Promise.reject(new Error('offline')) });
      b.audio.unlock();
      b.cart.speed = 3;
      await expect(b.run(1)).resolves.toBeUndefined();
      expect(b.audio.engine.playing).toBe(false);
      const c = setup();
      c.ctx.failDecode = true;
      c.audio.unlock();
      c.cart.speed = 3;
      await expect(c.run(1)).resolves.toBeUndefined();
      expect(c.audio.engine.playing).toBe(false);
      for (const s of spies) expect(s).not.toHaveBeenCalled();
    } finally {
      for (const s of spies) s.mockRestore();
    }
  });
});
