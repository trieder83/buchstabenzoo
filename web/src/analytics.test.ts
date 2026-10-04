// Opt-in analytics (PLAT-022..027, PLAT-030): the module with an injected environment.
import { describe, expect, it } from 'vitest';
import { ALLOWED_EVENTS, Analytics, clearGaCookies, CONSENT_KEY, GTAG_SCRIPT, sanitize, TICK_MS, type AnalyticsEnv } from './analytics';

function setup(id = 'G-TEST000000', stored?: string) {
  const mem = new Map<string, string>();
  if (stored) mem.set(CONSENT_KEY, stored);
  const calls: unknown[][] = [];
  const scripts: string[] = [];
  const disabled: Record<string, boolean> = {};
  let t = 1_000_000;
  let visible = true;
  let timer: (() => void) | null = null;
  const st = { cleared: 0, intervals: 0 };
  const env: AnalyticsEnv = {
    storage: { getItem: (k) => mem.get(k) ?? null, setItem: (k, v) => void mem.set(k, v) },
    loadScript: (s) => void scripts.push(s),
    gtag: (...a) => void calls.push(a),
    now: () => t,
    clearCookies: () => void st.cleared++,
    setDisabled: (i, on) => void (disabled[i] = on),
    visible: () => visible,
    setInterval: (fn) => {
      timer = fn;
      st.intervals++;
      return 7;
    },
    clearInterval: () => {
      timer = null;
    },
    context: () => ({ language: 'de', readingLevel: 'klasse1' }),
  };
  const a = new Analytics(env, id);
  const events = () => calls.filter((c) => c[0] === 'event');
  /** Lets `ms` of wall time pass in 10 s timer ticks. */
  const advance = (ms: number) => {
    for (let n = 0; n < ms; n += TICK_MS) {
      t += TICK_MS;
      timer?.();
    }
  };
  return { a, mem, calls, scripts, disabled, st, events, advance, hide: () => (visible = false), show: () => (visible = true), timer: () => timer };
}

describe('default: nothing before consent', () => {
  it('PLAT-022 init without a stored choice loads nothing and sends nothing', () => {
    const s = setup();
    s.a.init();
    s.a.track('level_started', { level_id: 'level_1' });
    s.a.observeLevel('level_1');
    s.a.onGameEvent({ type: 'mission_complete', animal: 'zebra' });
    expect(s.a.consent()).toBe('unset');
    expect(s.a.on).toBe(false);
    expect(s.scripts).toEqual([]);
    expect(s.calls).toEqual([]);
    expect(s.timer()).toBeNull();
  });

  it('PLAT-022 a stored denied stays off', () => {
    const s = setup('G-TEST000000', 'denied');
    s.a.init();
    expect(s.scripts).toEqual([]);
    expect(s.calls).toEqual([]);
  });

  it('PLAT-024 an empty id disables everything even with consent', () => {
    const s = setup('', 'granted');
    expect(s.a.available).toBe(false);
    s.a.init();
    s.a.grant();
    expect(s.scripts).toEqual([]);
    expect(s.calls).toEqual([]);
  });
});

describe('consent flow', () => {
  it('PLAT-024 grant: consent default denied, script, consent update, private config, session event', () => {
    const s = setup();
    s.a.grant();
    expect(s.mem.get(CONSENT_KEY)).toBe('granted');
    expect(s.scripts).toEqual([GTAG_SCRIPT + 'G-TEST000000']);
    const names = s.calls.map((c) => c.slice(0, 2).join(':'));
    expect(names.indexOf('consent:default')).toBe(0);
    expect(s.calls[0][2]).toEqual({ analytics_storage: 'denied', ad_storage: 'denied', ad_user_data: 'denied', ad_personalization: 'denied' });
    expect(names.indexOf('consent:update')).toBeGreaterThan(names.indexOf('consent:default'));
    expect(s.calls.find((c) => c[0] === 'consent' && c[1] === 'update')![2]).toEqual({ analytics_storage: 'granted' });
    expect(s.calls).toContainEqual(['set', 'ads_data_redaction', true]);
    expect(s.calls).toContainEqual(['set', 'restricted_data_processing', true]);
    const cfg = s.calls.find((c) => c[0] === 'config')!;
    expect(cfg[1]).toBe('G-TEST000000');
    expect(cfg[2]).toMatchObject({ allow_google_signals: false, allow_ad_personalization_signals: false, cookie_flags: 'SameSite=Lax;Secure', send_page_view: true });
    expect(s.calls.some((c) => c.includes('user_id') || (c[2] && typeof c[2] === 'object' && 'user_id' in (c[2] as object)))).toBe(false);
    expect(s.calls.some((c) => c[0] === 'set' && c[1] === 'user_properties')).toBe(false);
    expect(s.events()).toEqual([['event', 'zoo_session', { app_language: 'de', reading_level: 'klasse1' }]]);
    expect(s.disabled['G-TEST000000']).toBe(false);
  });

  it('PLAT-024 a stored consent is resumed at page start; the script loads once', () => {
    const s = setup('G-TEST000000', 'granted');
    s.a.init();
    s.a.grant();
    expect(s.scripts.length).toBe(1);
    expect(s.a.on).toBe(true);
  });

  it('PLAT-023 deny stores denied, sends nothing, clears cookies', () => {
    const s = setup();
    s.a.deny();
    expect(s.mem.get(CONSENT_KEY)).toBe('denied');
    expect(s.a.on).toBe(false);
    expect(s.scripts).toEqual([]);
    expect(s.events()).toEqual([]);
  });

  it('PLAT-027 withdrawal: denied, consent update, disable flag, cookies, no more events, no timer', () => {
    const s = setup();
    s.a.grant();
    const before = s.events().length;
    s.a.deny();
    expect(s.mem.get(CONSENT_KEY)).toBe('denied');
    expect(s.calls[s.calls.length - 1]).toEqual(['consent', 'update', { analytics_storage: 'denied' }]);
    expect(s.disabled['G-TEST000000']).toBe(true);
    expect(s.st.cleared).toBe(1);
    expect(s.timer()).toBeNull();
    s.a.track('mission_complete', { animal_id: 'zebra' });
    s.a.observeLevel('level_1');
    s.advance(10 * 60_000);
    expect(s.events().length).toBe(before);
    // switching on again re-enables without loading the script twice; the session event is not repeated
    s.a.grant();
    expect(s.scripts.length).toBe(1);
    expect(s.disabled['G-TEST000000']).toBe(false);
    expect(s.events().filter((c) => c[1] === 'zoo_session').length).toBe(1);
  });

  it('PLAT-027 clearGaCookies removes _ga* cookies on host and parent domains, others stay', () => {
    const writes: string[] = [];
    let jar = '_ga=GA1.1.1; _ga_ABC=GS1; theme=x';
    const doc = {
      get cookie() {
        return jar;
      },
      set cookie(v: string) {
        writes.push(v);
      },
    };
    clearGaCookies(doc, 'letterzoo.web.app');
    expect(writes.some((w) => w.startsWith('_ga=;') && w.includes('expires=Thu, 01 Jan 1970'))).toBe(true);
    expect(writes.some((w) => w.startsWith('_ga_ABC=;') && w.includes('domain=.letterzoo.web.app'))).toBe(true);
    expect(writes.some((w) => w.startsWith('_ga=;') && w.includes('domain=.web.app'))).toBe(true);
    expect(writes.some((w) => w.startsWith('theme'))).toBe(false);
    jar = 'theme=x';
    writes.length = 0;
    clearGaCookies(doc, 'letterzoo.web.app');
    expect(writes).toEqual([]);
  });
});

describe('allowlist', () => {
  it('PLAT-025 unknown events and params are dropped', () => {
    expect(Object.keys(ALLOWED_EVENTS).sort()).toEqual(
      ['all_animals_home', 'baby_born', 'level_complete', 'level_started', 'mission_complete', 'night_started', 'zoo_play_minutes', 'zoo_session'].sort(),
    );
    expect(sanitize('purchase', {})).toBeNull();
    expect(sanitize('toString', {})).toBeNull();
    expect(sanitize('__proto__', {})).toBeNull();
    // extra keys (a name, coordinates) are removed, the event stays
    expect(sanitize('level_complete', { level_id: 'level_1', player_name: 'Anna', x: 3, y: 4 })).toEqual({ level_id: 'level_1' });
    expect(sanitize('night_started', { anything: 'x' })).toEqual({});
  });

  it('PLAT-025 free text, long strings and wrong types drop the event', () => {
    expect(sanitize('level_complete', { level_id: 'Anna spielt' })).toBeNull();
    expect(sanitize('level_complete', { level_id: 'a'.repeat(33) })).toBeNull();
    expect(sanitize('level_complete', { level_id: 3 })).toBeNull();
    expect(sanitize('level_complete', {})).toBeNull();
    expect(sanitize('mission_complete', { animal_id: 'Zebra' })).toBeNull();
    expect(sanitize('zoo_session', { app_language: 'xx', reading_level: 'kiga' })).toBeNull();
    expect(sanitize('zoo_session', { app_language: 'en', reading_level: 'klasse3' })).toEqual({ app_language: 'en', reading_level: 'klasse3' });
    expect(sanitize('zoo_play_minutes', { minutes: 5.5 })).toBeNull();
    expect(sanitize('zoo_play_minutes', { minutes: 0 })).toBeNull();
    expect(sanitize('zoo_play_minutes', { minutes: 500 })).toBeNull();
    expect(sanitize('zoo_play_minutes', { minutes: 10 })).toEqual({ minutes: 10 });
  });

  it('PLAT-025 track sends only sanitised events', () => {
    const s = setup();
    s.a.grant();
    const n = s.events().length;
    s.a.track('level_complete', { level_id: 'level_1', note: 'hi' });
    s.a.track('level_complete', { level_id: 'my name is Bob' });
    s.a.track('nope', {});
    expect(s.events().slice(n)).toEqual([['event', 'level_complete', { level_id: 'level_1' }]]);
  });
});

describe('play minutes', () => {
  it('PLAT-026 sends 5, 10 minutes of active play, capped at 120', () => {
    const s = setup();
    s.a.grant();
    s.advance(4 * 60_000 + 50_000);
    expect(s.events().filter((c) => c[1] === 'zoo_play_minutes')).toEqual([]);
    s.advance(10_000);
    expect(s.events().filter((c) => c[1] === 'zoo_play_minutes')).toEqual([['event', 'zoo_play_minutes', { minutes: 5 }]]);
    s.advance(5 * 60_000);
    expect(s.events().filter((c) => c[1] === 'zoo_play_minutes').map((c) => (c[2] as { minutes: number }).minutes)).toEqual([5, 10]);
    s.advance(300 * 60_000);
    const mins = s.events().filter((c) => c[1] === 'zoo_play_minutes').map((c) => (c[2] as { minutes: number }).minutes);
    expect(Math.max(...mins)).toBe(120);
    expect(mins.length).toBe(24);
  });

  it('PLAT-026 a hidden tab does not count', () => {
    const s = setup();
    s.a.grant();
    s.advance(60_000);
    s.hide();
    s.advance(30 * 60_000);
    expect(s.events().filter((c) => c[1] === 'zoo_play_minutes')).toEqual([]);
    s.show();
    s.advance(3 * 60_000);
    expect(s.events().filter((c) => c[1] === 'zoo_play_minutes')).toEqual([]);
    s.advance(60_000);
    expect(s.events().filter((c) => c[1] === 'zoo_play_minutes').length).toBe(1);
  });

  it('PLAT-026 no timer without consent', () => {
    const s = setup();
    s.a.init();
    expect(s.st.intervals).toBe(0);
    s.advance(10 * 60_000);
    expect(s.calls).toEqual([]);
  });
});

describe('game events', () => {
  it('PLAT-030 maps the host messages to allowlisted events with ids only', () => {
    const s = setup();
    s.a.grant();
    const n = s.events().length;
    s.a.observeLevel('level_1');
    s.a.observeLevel('level_1'); // once per session
    s.a.observeLevel('');
    s.a.observeLevel('level_2');
    s.a.onGameEvent({ type: 'mission_complete', animal: 'zebra' });
    s.a.onGameEvent({ type: 'level_complete', level: 'level_1' });
    s.a.onGameEvent({ type: 'night' });
    s.a.onGameEvent({ type: 'all_home' });
    s.a.onGameEvent({ type: 'baby_born', animal: 'panda' });
    s.a.onGameEvent({ type: 'say', animal: 'zebra' });
    expect(s.events().slice(n)).toEqual([
      ['event', 'level_started', { level_id: 'level_1' }],
      ['event', 'level_started', { level_id: 'level_2' }],
      ['event', 'mission_complete', { animal_id: 'zebra' }],
      ['event', 'level_complete', { level_id: 'level_1' }],
      ['event', 'night_started', {}],
      ['event', 'all_animals_home', {}],
      ['event', 'baby_born', { species_id: 'panda' }],
    ]);
  });
});
