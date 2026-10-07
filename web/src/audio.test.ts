// ART-SOUND "Playback" host tests: ASND-007/009/017/019 (no real audio device: a fake context).
import { describe, expect, it, vi } from 'vitest';
import {
  type AudioContextLike,
  type AudioEnv,
  GameAudio,
  parseAudioIndex,
  pickFormat,
  pickVariation,
  type SoundEvent,
} from './audio';

const INDEX = [
  'levels/level-1.toml',
  'audio/steps/step_grass_1.ogg',
  'audio/steps/step_grass_2.ogg',
  'audio/steps/step_grass_1.m4a',
  'audio/steps/step_grass_2.m4a',
  'audio/ui/ui_refuse_1.ogg',
  'audio/ui/ui_refuse_1.m4a',
  'audio/animals/animal_zebra_call_1.ogg',
  'audio/animals/animal_zebra_call_1.m4a',
  'audio/pickups/only_m4a_1.m4a',
];

const ev = (cue: string, v = 0): SoundEvent => ({ cue, x: 1, z: 2, g: 0.175, r: 1, v });

class FakeCtx implements AudioContextLike {
  state = 'suspended';
  destination = {};
  started: { gain: number; rate: number }[] = [];
  resume() {
    this.state = 'running';
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
      playbackRate: { value: 1 },
      connect() {},
      start: () => this.started.push({ gain: this.lastGain, rate: s.playbackRate.value }),
    };
    return s;
  }
  lastGain = 0;
  createGain() {
    const g = { gain: { value: 0 }, connect: () => (this.lastGain = g.gain.value) };
    return g;
  }
}

function env(over: Partial<AudioEnv> = {}, ctx: AudioContextLike | null = new FakeCtx()) {
  const urls: string[] = [];
  const e: AudioEnv = {
    fetch: (url) => {
      urls.push(url);
      return Promise.resolve({ ok: true, arrayBuffer: () => Promise.resolve(new ArrayBuffer(4)) });
    },
    createContext: () => ctx,
    canPlayType: () => 'probably',
    now: () => 0,
    ...over,
  };
  return { e, urls, ctx };
}

const flush = () => new Promise((r) => setTimeout(r, 0));

describe('host playback', () => {
  it('ASND-017 maps cues to groups and files, ogg first, m4a fallback', () => {
    const cues = parseAudioIndex(INDEX);
    expect(cues.get('step_grass')).toEqual({
      group: 'steps',
      ogg: ['audio/steps/step_grass_1.ogg', 'audio/steps/step_grass_2.ogg'],
      m4a: ['audio/steps/step_grass_1.m4a', 'audio/steps/step_grass_2.m4a'],
    });
    expect(cues.get('animal_zebra_call')?.group).toBe('animals/zebra');
    expect(cues.get('ui_refuse')?.group).toBe('ui');
    expect(pickFormat(() => 'probably')).toBe('ogg');
    expect(pickFormat(() => '')).toBe('m4a');
    expect(pickFormat(() => {
      throw new Error('no audio');
    })).toBe('m4a');
    const a = new GameAudio(INDEX, env({ canPlayType: () => '' }).e);
    a.unlock();
    expect(a.play(ev('step_grass')).file).toBe('audio/steps/step_grass_1.m4a');
    const b = new GameAudio(INDEX, env().e);
    b.unlock();
    expect(b.play(ev('only_m4a')).file).toBe('audio/pickups/only_m4a_1.m4a'); // other format
    expect(b.play(ev('step_grass', 5)).file).toBe('audio/steps/step_grass_2.ogg');
  });

  it('ASND-017 variation v mod n, never the same as the last', () => {
    expect(pickVariation(7, 4, -1)).toBe(3);
    expect(pickVariation(7, 4, 3)).toBe(0);
    expect(pickVariation(9, 1, 0)).toBe(0);
    expect(pickVariation(3, 0, -1)).toBe(0);
  });

  it('ASND-017 a cue without files is skipped silently', async () => {
    const { e, urls } = env();
    const a = new GameAudio(INDEX, e);
    a.unlock();
    expect(a.play(ev('animal_koala_call')).result).toBe('missing');
    await flush();
    expect(urls).toEqual([]);
  });

  it('ASND-007 nothing is fetched before the first gesture; the cue is logged as locked', async () => {
    const { e, urls } = env();
    const a = new GameAudio(INDEX, e);
    const app = { poll_sounds: () => JSON.stringify([ev('step_grass')]) };
    a.tick(app);
    await flush();
    expect(urls).toEqual([]);
    expect(a.fetched).toEqual([]);
    expect(a.log.map((l) => l.result)).toEqual(['locked']);
    expect(a.state).toBe('none');
  });

  it('loads a group lazily on first use, caches it and plays with gain and rate', async () => {
    const { e, urls, ctx } = env();
    const a = new GameAudio(INDEX, e);
    a.unlock();
    a.play({ ...ev('step_grass'), r: 1.03 });
    await flush();
    expect(urls).toEqual(['assets/audio/steps/step_grass_1.ogg', 'assets/audio/steps/step_grass_2.ogg']);
    const c = ctx as FakeCtx;
    expect(c.started).toEqual([{ gain: 0.175, rate: 1.03 }]);
    a.play(ev('step_grass', 1));
    await flush();
    expect(urls.length).toBe(2); // cached: no new fetch
    expect(c.started.length).toBe(2);
  });

  it('prefetches the common groups and the animals of the unlocked levels one by one', async () => {
    const { e, urls } = env({ now: (() => { let t = 0; return () => (t += 3000); })() });
    const a = new GameAudio(INDEX, e);
    a.unlock();
    a.tick({ poll_sounds: () => '', audio_animals: () => '["zebra","koala"]' });
    await flush();
    await flush();
    expect(urls.some((u) => u.includes('animal_zebra_call'))).toBe(true);
    expect(urls.some((u) => u.includes('koala'))).toBe(false); // no files: skipped
    expect(urls.some((u) => u.includes('step_grass_1.ogg'))).toBe(true);
  });

  it('ASND-009 a muted game decides and logs, but plays and fetches nothing', async () => {
    const { e, urls, ctx } = env();
    const a = new GameAudio(INDEX, e);
    a.unlock();
    a.setEnabled(false);
    const l = a.play(ev('ui_refuse'));
    a.prefetch();
    await flush();
    expect(l).toMatchObject({ muted: true, result: 'muted' });
    expect(urls).toEqual([]);
    expect((ctx as FakeCtx).started).toEqual([]);
    a.setEnabled(true);
    a.play(ev('ui_refuse'));
    await flush();
    expect((ctx as FakeCtx).started.length).toBe(1);
  });

  it('ASND-019 no Web Audio, failing fetch or decode: never throws, never logs to the console', async () => {
    const spy = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => undefined);
    const none = new GameAudio(INDEX, env({}, null).e);
    none.unlock();
    expect(() => none.play(ev('step_grass'))).not.toThrow();
    const bad = env({ fetch: () => Promise.reject(new Error('offline')) });
    const a = new GameAudio(INDEX, bad.e);
    a.unlock();
    expect(() => a.play(ev('step_grass'))).not.toThrow();
    const notOk = env({ fetch: () => Promise.resolve({ ok: false, arrayBuffer: () => Promise.reject(new Error('x')) }) });
    const b = new GameAudio(INDEX, notOk.e);
    b.unlock();
    b.play(ev('ui_refuse'));
    const ctx = new FakeCtx();
    ctx.failDecode = true;
    const c = new GameAudio(INDEX, env({}, ctx).e);
    c.unlock();
    c.play(ev('ui_refuse'));
    const broken = { poll_sounds: () => { throw new Error('wasm'); } };
    expect(() => c.tick(broken)).not.toThrow();
    expect(() => c.tick({ poll_sounds: () => 'not json' })).not.toThrow();
    await flush();
    await flush();
    expect(spy).not.toHaveBeenCalled();
    expect(warn).not.toHaveBeenCalled();
    spy.mockRestore();
    warn.mockRestore();
  });
});

// ASND-038: delayed events (key box cues) are scheduled at currentTime + d, not when muted
describe('delayed sound events', () => {
  class TimedCtx extends FakeCtx {
    currentTime = 10;
    whens: (number | undefined)[] = [];
    gains: { value: number }[] = [];
    createBufferSource() {
      const s = {
        buffer: null as object | null,
        playbackRate: { value: 1 },
        connect() {},
        start: (when?: number) => this.whens.push(when),
      };
      return s;
    }
    createGain() {
      const g = { gain: { value: 0 }, connect: () => 0 };
      this.gains.push(g.gain);
      return g;
    }
  }
  const delayed = async (mute = false) => {
    const ctx = new TimedCtx();
    const audio = new GameAudio(INDEX, env({}, ctx).e);
    audio.unlock();
    audio.prefetch();
    await new Promise((r) => setTimeout(r, 20));
    audio.play({ ...ev('ui_refuse'), d: 0.6 });
    audio.play({ ...ev('ui_refuse'), d: 1.2 });
    audio.play(ev('ui_refuse'));
    if (mute) audio.setEnabled(false);
    return ctx;
  };
  it('ASND-038 plays at currentTime + d', async () => {
    const ctx = await delayed();
    expect(ctx.whens[0]).toBeCloseTo(10.6, 6);
    expect(ctx.whens[1]).toBeCloseTo(11.2, 6);
    expect(ctx.whens[2]).toBeUndefined(); // no d: at once
  });
  it('ASND-038 a cue muted before it fires does not play', async () => {
    const ctx = await delayed(true);
    expect(ctx.gains[0].value).toBe(0);
    expect(ctx.gains[1].value).toBe(0);
    expect(ctx.gains[2].value).toBeGreaterThan(0); // the immediate one already started
  });
});
