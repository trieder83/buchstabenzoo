// ART-SOUND "Ambient loops": ASND-023..026 (the night cricket bed; fake context, fake clock).
import { describe, expect, it, vi } from 'vitest';
import { AMBIENT_FADE_S, AMBIENT_GAIN_MAX, type AudioContextLike, type AudioEnv, GameAudio } from './audio';

const INDEX = ['audio/ambient/ambient_crickets_1.ogg', 'audio/ambient/ambient_crickets_1.m4a', 'audio/ui/ui_tap_1.ogg'];

class Ctx implements AudioContextLike {
  state = 'running';
  destination = {};
  sources: { loop: boolean; started: boolean; stopped: boolean }[] = [];
  nodes: { gain: { value: number } }[] = [];
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
  decodeAudioData(_d: ArrayBuffer, ok: (b: object) => void) {
    ok({ decoded: true });
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
    const g = { gain: { value: 0 }, connect() {} };
    this.nodes.push(g);
    return g;
  }
}

function setup(over: Partial<AudioEnv> = {}) {
  const ctx = new Ctx();
  const urls: string[] = [];
  const clock = { t: 0, hidden: false, blocked: false };
  const env: AudioEnv = {
    fetch: (url) => {
      urls.push(url);
      return Promise.resolve({ ok: true, arrayBuffer: () => Promise.resolve(new ArrayBuffer(4)) });
    },
    createContext: () => ctx,
    canPlayType: () => 'probably',
    now: () => clock.t,
    hidden: () => clock.hidden,
    blocked: () => clock.blocked,
    ...over,
  };
  const audio = new GameAudio(INDEX, env);
  const phase = { target: 0 };
  const app = { poll_sounds: () => '', ambient_target: () => phase.target };
  /** Advances `s` seconds in 100 ms frames (flushing the loads between frames). */
  const run = async (s: number) => {
    for (let i = 0; i < Math.round(s * 10); i++) {
      clock.t += 100;
      audio.tick(app);
      await Promise.resolve();
      await Promise.resolve();
    }
  };
  return { audio, ctx, urls, clock, phase, run };
}
const live = (ctx: Ctx) => ctx.sources.filter((s) => s.started && !s.stopped).length;

describe('night cricket bed', () => {
  it('ASND-023 fades in linearly to 0.12 in 3 s, never above, and out again, releasing the source', async () => {
    expect(AMBIENT_GAIN_MAX).toBe(0.12);
    expect(AMBIENT_FADE_S).toBe(3);
    const { audio, ctx, phase, run } = setup();
    audio.unlock();
    phase.target = 0.12;
    await run(0.5); // load
    await run(1.5);
    const mid = audio.ambient.gain;
    expect(mid).toBeGreaterThan(0.02);
    expect(mid).toBeLessThan(0.12);
    expect(ctx.sources[0].loop).toBe(true);
    await run(3.2);
    expect(audio.ambient).toMatchObject({ playing: true, fetched: true });
    expect(audio.ambient.gain).toBeCloseTo(0.12, 6);
    phase.target = 10; // a wrong host value is clamped
    await run(1);
    expect(audio.ambient.gain).toBeLessThanOrEqual(0.12);
    phase.target = 0;
    await run(1.5);
    expect(audio.ambient.gain).toBeGreaterThan(0);
    expect(audio.ambient.gain).toBeLessThan(0.12);
    await run(2);
    expect(audio.ambient).toMatchObject({ playing: false, gain: 0 });
    expect(ctx.sources[0].stopped).toBe(true);
  });

  it('ASND-024 repeated night / day flips: one source at a time, one fetch, no jumps', async () => {
    const { audio, ctx, urls, phase, run } = setup();
    audio.unlock();
    let last = 0;
    for (let i = 0; i < 8; i++) {
      phase.target = i % 2 === 0 ? 0.12 : 0;
      for (let f = 0; f < 12; f++) {
        await run(0.1);
        const g = audio.ambient.gain;
        expect(Math.abs(g - last)).toBeLessThanOrEqual(0.12 / 30 + 1e-9); // <= 1 frame of the fade
        expect(live(ctx)).toBeLessThanOrEqual(1);
        last = g;
      }
    }
    expect(urls.filter((u) => u.includes('ambient_crickets')).length).toBe(1);
    await run(4);
    await run(4);
    expect(live(ctx)).toBe(0);
    phase.target = 0.12;
    await run(4);
    expect(live(ctx)).toBe(1);
    expect(urls.filter((u) => u.includes('ambient_crickets')).length).toBe(1);
  });

  it('ASND-025 mute fades out (setting untouched), unmute fades in; a hidden tab is silent and suspends', async () => {
    const { audio, ctx, clock, phase, run } = setup();
    audio.unlock();
    phase.target = 0.12;
    await run(4.5);
    expect(audio.ambient.gain).toBeCloseTo(0.12, 6);
    audio.setEnabled(false);
    await run(1);
    expect(audio.ambient.gain).toBeLessThan(0.12);
    expect(audio.ambient.gain).toBeGreaterThan(0);
    await run(3);
    expect(audio.ambient.gain).toBe(0);
    expect(audio.ambient.playing).toBe(false);
    audio.setEnabled(true);
    await run(4);
    expect(audio.ambient.gain).toBeCloseTo(0.12, 6);
    clock.hidden = true;
    await run(0.2);
    expect(audio.ambient.gain).toBe(0);
    expect(ctx.suspended).toBe(1);
    clock.hidden = false;
    await run(0.5);
    expect(ctx.resumed).toBeGreaterThan(0);
    expect(audio.ambient.gain).toBeGreaterThan(0);
    expect(audio.ambient.gain).toBeLessThan(0.12);
    // an overlay (title / intro) keeps it silent
    clock.blocked = true;
    await run(4);
    expect(audio.ambient.gain).toBe(0);
  });

  it('ASND-026 no gesture, no context, failing fetch / decode: nothing fetched or logged, never throws', async () => {
    const spy = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => undefined);
    const a = setup();
    a.phase.target = 0.12; // night, but no gesture yet
    await a.run(1);
    expect(a.urls).toEqual([]);
    expect(a.audio.ambient.fetched).toBe(false);
    expect(a.audio.ambient.gain).toBeGreaterThan(0); // the target still moves; nothing plays
    expect(a.audio.ambient.playing).toBe(false);
    a.audio.unlock();
    await a.run(0.3);
    expect(a.urls.length).toBe(1);
    expect(a.urls[0]).toBe('assets/audio/ambient/ambient_crickets_1.ogg');
    // day: never fetched
    const d = setup();
    d.audio.unlock();
    await d.run(2);
    expect(d.urls).toEqual([]);
    // no Web Audio / failing fetch
    const n = setup({ createContext: () => null });
    n.audio.unlock();
    n.phase.target = 0.12;
    await expect(n.run(1)).resolves.toBeUndefined();
    const f = setup({ fetch: () => Promise.reject(new Error('offline')) });
    f.audio.unlock();
    f.phase.target = 0.12;
    await f.run(2);
    expect(f.audio.ambient.playing).toBe(false);
    expect(() => f.audio.tick({ poll_sounds: () => '', ambient_target: () => { throw new Error('wasm'); } })).not.toThrow();
    expect(spy).not.toHaveBeenCalled();
    expect(warn).not.toHaveBeenCalled();
    spy.mockRestore();
    warn.mockRestore();
  });
});
