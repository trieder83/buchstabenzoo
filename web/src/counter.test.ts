// TECH-PLATFORMS "Anonymous counters": PLAT-044 (queue/format), PLAT-045 (allowlist parity with firestore.rules),
// PLAT-046 (no storage / cookies), PLAT-047 (off switches), PLAT-048 (CSP).
import { describe, expect, it, vi } from 'vitest';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { COUNTER_EVENTS, Counter, countersAllowed, cleanVersion, MAX_BATCH, MAX_PER_MINUTE, paramOk, slotParam, utcDay, type CounterEnv } from './counter';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const T0 = Date.UTC(2026, 9, 8, 12, 0, 0);

function make(over: Partial<CounterEnv> = {}) {
  const calls: { url: string; body: { writes: Record<string, any>[] } }[] = [];
  const timers: (() => void)[] = [];
  let now = T0;
  const env: CounterEnv = {
    allowed: true,
    platform: 'web',
    language: () => 'de',
    version: 'abc123',
    now: () => now,
    fetch: (url, init) => {
      calls.push({ url, body: JSON.parse(init.body) });
      return Promise.resolve({});
    },
    setTimeout: (fn) => {
      timers.push(fn);
      return timers.length;
    },
    ...over,
  };
  const c = new Counter(env);
  const fire = () => timers.splice(0).forEach((f) => f());
  return { c, calls, fire, advance: (ms: number) => (now += ms) };
}

describe('anonymous counters: queue and request format (PLAT-044)', () => {
  it('PLAT-044 one write per count: document id, exactly the allowed fields, increment of n by 1', () => {
    const { c, calls, fire } = make();
    c.count('level_started', 'level_1');
    expect(calls).toHaveLength(0); // queued, not sent at once
    fire();
    expect(calls).toHaveLength(1);
    const { url, body } = calls[0];
    expect(url).toMatch(/^https:\/\/firestore\.googleapis\.com\/v1\/projects\/letterzoo\/databases\/\(default\)\/documents:commit\?key=AIza/);
    const w = body.writes[0];
    expect(w.update.name).toBe('projects/letterzoo/databases/(default)/documents/c/20261008_level_started_level_1_web_de_abc123');
    expect(Object.keys(w.update.fields).sort()).toEqual(['date', 'event', 'lang', 'param', 'platform', 'v']);
    expect(w.update.fields.date).toEqual({ stringValue: '20261008' });
    expect(w.updateMask.fieldPaths.sort()).toEqual(['date', 'event', 'lang', 'param', 'platform', 'v']);
    expect(w.updateTransforms).toEqual([{ fieldPath: 'n', increment: { integerValue: '1' } }]);
  });
  it('PLAT-044 unknown events and wrong param shapes are dropped; the day is UTC', () => {
    const { c, calls, fire } = make();
    c.count('nope');
    c.count('level_started', 'Free Text!');
    c.count('lock_wrong', 'x');
    c.count('board_link_tapped', 'ios');
    expect(c.pending).toBe(0);
    fire();
    expect(calls).toHaveLength(0);
    expect(utcDay(Date.UTC(2026, 0, 2, 23, 59, 59))).toBe('20260102');
  });
  it('PLAT-044 the same document twice goes into two commits (rules allow n + 1 per write), at most one per commit', () => {
    const { c, calls, fire } = make();
    c.count('lock_wrong');
    c.count('lock_wrong');
    c.count('lock_ok');
    fire();
    expect(calls).toHaveLength(1);
    expect(calls[0].body.writes).toHaveLength(2); // lock_wrong + lock_ok
    fire(); // the repeat
    expect(calls).toHaveLength(2);
    expect(calls[1].body.writes).toHaveLength(1);
  });
  it('PLAT-044 per-event rate limit, batch cap, errors swallowed, nothing without permission', async () => {
    const m = make();
    for (let i = 0; i < MAX_PER_MINUTE + 10; i++) m.c.count('lock_wrong');
    expect(m.c.pending).toBe(MAX_PER_MINUTE);
    m.advance(61_000);
    m.c.count('lock_wrong');
    expect(m.c.pending).toBe(MAX_PER_MINUTE + 1);
    const ev = Object.keys(COUNTER_EVENTS).filter((e) => COUNTER_EVENTS[e] === 'none');
    expect(ev.length).toBeGreaterThan(MAX_BATCH / 2);
    const failing = make({ fetch: () => Promise.reject(new Error('offline')) });
    failing.c.count('lock_ok');
    expect(() => failing.fire()).not.toThrow();
    const throwing = make({ fetch: () => { throw new Error('sync'); } });
    throwing.c.count('lock_ok');
    expect(() => throwing.fire()).not.toThrow();
    const off = make({ allowed: false });
    off.c.count('lock_ok');
    off.c.flush();
    expect(off.calls).toHaveLength(0);
    expect(off.c.pending).toBe(0);
  });
  it('PLAT-044 platform, language (de|en) and a clean version are part of the document', () => {
    const { c, calls, fire } = make({ platform: 'ios', language: () => 'en', version: 'v1.2' });
    c.count('session_start');
    fire();
    expect(calls[0].body.writes[0].update.name).toMatch(/c\/20261008_session_start__ios_en_v1\.2$/);
    expect(cleanVersion('A1b2/../x y')).toBe('a1b2..xy');
    expect(cleanVersion('')).toBe('dev');
  });
  it('PLAT-044 board params are slot + link type', () => {
    expect(slotParam(1, 'https://apps.apple.com/app/id1')).toBe('s1_ios');
    expect(slotParam(2, 'https://play.google.com/store/apps/details?id=x')).toBe('s2_android');
    expect(slotParam(3, 'https://mathfighter.rcms.ch/')).toBe('s3_web');
    expect(paramOk('slot', 's3_web')).toBe(true);
    expect(paramOk('slot', 's4_web')).toBe(false);
  });
});

describe('anonymous counters: allowlist parity with firestore.rules (PLAT-045)', () => {
  const rules = fs.readFileSync(path.join(root, 'firestore.rules'), 'utf8');
  it('PLAT-045 the event list of the rules equals COUNTER_EVENTS', () => {
    const block = /d\.event in \[([^\]]*)\]/.exec(rules)![1];
    const inRules = [...block.matchAll(/'([a-z_]+)'/g)].map((m) => m[1]).sort();
    expect(inRules).toEqual(Object.keys(COUNTER_EVENTS).sort());
  });
  it('PLAT-045 the rules allow exactly the fields, n == 1 on create and +1 on update, no read / delete', () => {
    expect(rules).toContain("hasOnly(['date', 'event', 'param', 'platform', 'lang', 'v', 'n'])");
    expect(rules).toContain('request.resource.data.n == 1');
    expect(rules).toContain('request.resource.data.n == resource.data.n + 1');
    expect(rules).toContain('allow get, list, delete: if false');
    expect(rules).toContain("d.param.matches('^[a-z0-9_-]{0,32}$')");
    expect(rules).toContain("d.platform in ['web', 'ios', 'android']");
    expect(rules).toContain("d.lang in ['de', 'en']");
    expect(rules).toContain("id == d.date + '_' + d.event + '_' + d.param + '_' + d.platform + '_' + d.lang + '_' + d.v");
  });
  it('PLAT-045 every shape the client sends passes the rules pattern for param', () => {
    const re = /^[a-z0-9_-]{0,32}$/;
    for (const p of ['', 'level_1', 's1_ios', 'no_key', 'compass', 'gate']) expect(re.test(p)).toBe(true);
  });
  it('PLAT-045 firebase.json wires the rules file', () => {
    const j = JSON.parse(fs.readFileSync(path.join(root, 'firebase.json'), 'utf8'));
    expect(j.firestore.rules).toBe('firestore.rules');
    expect(fs.existsSync(path.join(root, 'firestore.indexes.json'))).toBe(true);
  });
});

describe('anonymous counters: no storage, no identifiers (PLAT-046)', () => {
  it('PLAT-046 counter.ts never touches cookies or web storage and sends no identifier fields', () => {
    const src = fs.readFileSync(path.join(root, 'web/src/counter.ts'), 'utf8').replace(/\/\/.*$/gm, '');
    expect(src).not.toMatch(/localStorage|sessionStorage|indexedDB|document\.cookie|crypto\.|Math\.random|userAgent|credentials/);
    const { c, calls, fire } = make();
    const touched: string[] = [];
    const trap = (name: string) => new Proxy({}, { get: (_t, k) => { touched.push(`${name}.${String(k)}`); return () => null; }, set: () => { touched.push(`${name} set`); return true; } });
    vi.stubGlobal('localStorage', trap('localStorage'));
    vi.stubGlobal('sessionStorage', trap('sessionStorage'));
    vi.stubGlobal('document', trap('document'));
    c.count('session_start');
    fire();
    vi.unstubAllGlobals();
    expect(touched).toEqual([]);
    const json = JSON.stringify(calls[0].body);
    expect(json).not.toMatch(/user|device|session_id|client_id|uuid|timestamp/i);
  });
});

describe('anonymous counters: off switches (PLAT-047)', () => {
  const base = { hostname: 'letterzoo.web.app', search: '' };
  it('PLAT-047 on by default on the production host', () => {
    expect(countersAllowed(base)).toBe(true);
  });
  it('PLAT-047 Do-Not-Track and Global Privacy Control switch it off, even with ?count=1', () => {
    expect(countersAllowed({ ...base, dnt: '1' })).toBe(false);
    expect(countersAllowed({ ...base, gpc: true })).toBe(false);
    expect(countersAllowed({ ...base, dnt: '1', search: '?count=1' })).toBe(false);
    expect(countersAllowed({ ...base, dnt: '0' })).toBe(true);
  });
  it('PLAT-047 ?nocount=1, automation and dev hosts are off unless ?count=1', () => {
    expect(countersAllowed({ ...base, search: '?nocount=1' })).toBe(false);
    expect(countersAllowed({ ...base, webdriver: true })).toBe(false);
    expect(countersAllowed({ ...base, webdriver: true, search: '?count=1' })).toBe(true);
    for (const h of ['localhost', '127.0.0.1', '192.168.1.20', '10.0.0.4', '172.16.3.3', 'box.local']) {
      expect(countersAllowed({ hostname: h, search: '' }), h).toBe(false);
    }
    expect(countersAllowed({ hostname: 'localhost', search: '?count=1' })).toBe(true);
    expect(countersAllowed({ hostname: 'letterzoo.rcms.ch', search: '' })).toBe(true);
  });
});

describe('anonymous counters: CSP and native build (PLAT-048)', () => {
  it('PLAT-048 the CSP allows the Firestore endpoint in connect-src only', () => {
    const j = JSON.parse(fs.readFileSync(path.join(root, 'firebase.json'), 'utf8'));
    const csp: string = j.hosting.headers[0].headers.find((h: { key: string }) => h.key === 'Content-Security-Policy').value;
    const connect = /connect-src ([^;]*)/.exec(csp)![1];
    expect(connect).toContain('https://firestore.googleapis.com');
    expect(csp.replace(connect, '')).not.toContain('firestore.googleapis.com');
  });
  it('PLAT-048 main.ts starts the counters in every build (also native) and the native build still has no gtag id', () => {
    const m = fs.readFileSync(path.join(root, 'web/src/main.ts'), 'utf8');
    expect(m).toMatch(/initCounter\(/);
    expect(m.indexOf('initCounter(')).toBeLessThan(m.indexOf('if (analytics.available)'));
    expect(m).toMatch(/analyticsId\(MEASUREMENT_ID\)/);
  });
});
