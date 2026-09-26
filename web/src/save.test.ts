import { describe, expect, it } from 'vitest';
import { newGameSeed, PICKS_KEY, SAVE_KEY, SaveSlot, type Removable, type SaveApp } from './save';

class MapStore implements Removable {
  m = new Map<string, string>();
  getItem(k: string): string | null {
    return this.m.get(k) ?? null;
  }
  setItem(k: string, v: string): void {
    this.m.set(k, v);
  }
  removeItem(k: string): void {
    this.m.delete(k);
  }
}

function fakeApp(valid = true) {
  let due = false;
  let n = 0;
  const restored: string[] = [];
  const games: [number, string][] = [];
  const app: SaveApp & { setDue(d: boolean): void } = {
    new_game: (seed, avoid) => {
      games.push([seed, avoid]);
      return true;
    },
    picks_json: () => `{"zebra":"loc_sand","n":${games.length}}`,
    save: () => `{"version":1,"n":${++n}}`,
    take_save: () => (due ? `{"version":1,"n":${++n}}` : ''),
    restore: (json) => {
      restored.push(json);
      return valid;
    },
    setDue: (d) => {
      due = d;
    },
  };
  return { app, restored, games };
}

describe('save slot (GAME-SAVE)', () => {
  it('restores a stored save before the first frame', () => {
    const store = new MapStore();
    store.setItem(SAVE_KEY, '{"version":1}');
    const { app, restored } = fakeApp();
    expect(new SaveSlot(app, store).restore()).toBe(true);
    expect(restored).toEqual(['{"version":1}']);
  });

  it('SAVE-005: an invalid save is dropped silently (new game)', () => {
    const store = new MapStore();
    store.setItem(SAVE_KEY, 'broken');
    const { app } = fakeApp(false);
    expect(new SaveSlot(app, store).restore()).toBe(false);
    expect(store.getItem(SAVE_KEY)).toBeNull();
  });

  it('stores due saves, flushes on hide, and stops after a new game', () => {
    const store = new MapStore();
    const { app } = fakeApp();
    const slot = new SaveSlot(app, store);
    slot.tick();
    expect(store.getItem(SAVE_KEY)).toBeNull();
    app.setDue(true);
    slot.tick();
    expect(store.getItem(SAVE_KEY)).toContain('"n":1');
    slot.flush();
    expect(store.getItem(SAVE_KEY)).toContain('"n":2');
    slot.reset();
    expect(store.getItem(SAVE_KEY)).toBeNull();
    slot.tick();
    slot.flush();
    expect(store.getItem(SAVE_KEY)).toBeNull();
  });

  it('survives a throwing storage', () => {
    const bad: Removable = {
      getItem: () => {
        throw new Error('blocked');
      },
      setItem: () => {
        throw new Error('blocked');
      },
      removeItem: () => {
        throw new Error('blocked');
      },
    };
    const { app } = fakeApp();
    const slot = new SaveSlot(app, bad);
    expect(slot.restore()).toBe(false);
    expect(() => {
      slot.tick();
      slot.flush();
      slot.reset();
    }).not.toThrow();
  });

  it('RESC-014 / Q-082: without a save a new game starts with the seed, avoids the last picks and stores its own', () => {
    const store = new MapStore();
    const { app, games } = fakeApp();
    expect(new SaveSlot(app, store).start(42)).toBe(false);
    expect(games).toEqual([[42, '']]);
    expect(store.getItem(PICKS_KEY)).toBe('{"zebra":"loc_sand","n":1}');
    // the next new game (after "new game" deleted the save) avoids those places
    expect(new SaveSlot(app, store).start(7)).toBe(false);
    expect(games[1]).toEqual([7, '{"zebra":"loc_sand","n":1}']);
    // a stored save wins
    store.setItem(SAVE_KEY, '{"version":1}');
    expect(new SaveSlot(app, store).start(9)).toBe(true);
    expect(games.length).toBe(2);
  });

  it('seed from ?seed=N, else random', () => {
    expect(newGameSeed('?seed=123')).toBe(123);
    expect(newGameSeed('?lang=de&seed=5')).toBe(5);
    expect(newGameSeed('', () => 0.5)).toBe(Math.floor(0.5 * 0xffffffff));
  });
});
