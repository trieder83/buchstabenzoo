import { describe, expect, it } from 'vitest';
import { FOOD_ICONS, TARGET_ICONS, loadSettings, parseBasket, saveSettings, type KeyValue } from './ui';

class MapStore implements KeyValue {
  m = new Map<string, string>();
  getItem(k: string): string | null {
    return this.m.get(k) ?? null;
  }
  setItem(k: string, v: string): void {
    this.m.set(k, v);
  }
}

describe('settings persistence (CONT-L10N §6)', () => {
  it('defaults to the given default language (always de, CONT-L10N §5) and klasse1', () => {
    expect(loadSettings(new MapStore(), 'de')).toEqual({ language: 'de', readingLevel: 'klasse1', view: 'zoo' });
    expect(loadSettings(null, 'de')).toEqual({ language: 'de', readingLevel: 'klasse1', view: 'zoo' });
  });
  it('L10N-005: browser en without stored choice starts in de; a chosen en is kept', () => {
    const s = new MapStore();
    // main.ts passes App.default_language(navigator.language), which is always 'de'
    expect(loadSettings(s, 'de').language).toBe('de');
    saveSettings(s, { language: 'en', readingLevel: 'klasse1' });
    expect(loadSettings(s, 'de').language).toBe('en');
  });
  it('round-trips and ignores invalid values', () => {
    const s = new MapStore();
    saveSettings(s, { language: 'en', readingLevel: 'kiga' });
    expect(loadSettings(s, 'de')).toEqual({ language: 'en', readingLevel: 'kiga', view: 'zoo' });
    s.setItem('zoo.language', 'fr');
    s.setItem('zoo.readingLevel', 'klasse9');
    expect(loadSettings(s, 'de')).toEqual({ language: 'de', readingLevel: 'klasse1', view: 'zoo' });
  });
  it('survives a throwing storage', () => {
    const bad: KeyValue = {
      getItem: () => {
        throw new Error('blocked');
      },
      setItem: () => {
        throw new Error('blocked');
      },
    };
    expect(loadSettings(bad, 'de').language).toBe('de');
    expect(() => saveSettings(bad, { language: 'de', readingLevel: 'kiga' })).not.toThrow();
  });
});

describe('camera view setting (GAME-CAMERA-VIEWS 9)', () => {
  it('CAMV-011: first_person is stored and restored; look_around never; invalid → zoo', () => {
    const s = new MapStore();
    expect(loadSettings(s, 'de').view).toBe('zoo');
    saveSettings(s, { language: 'de', readingLevel: 'klasse1', view: 'first_person' });
    expect(loadSettings(s, 'de').view).toBe('first_person');
    saveSettings(s, { language: 'de', readingLevel: 'klasse1', view: 'look_around' });
    expect(loadSettings(s, 'de').view).toBe('first_person'); // not overwritten by look-around
    s.setItem('zoo.view', 'look_around');
    expect(loadSettings(s, 'de').view).toBe('zoo');
    s.setItem('zoo.view', 'drone');
    expect(loadSettings(s, 'de').view).toBe('zoo');
    saveSettings(s, { language: 'de', readingLevel: 'klasse1', view: 'zoo' });
    expect(loadSettings(s, 'de').view).toBe('zoo');
  });
});

describe('icons', () => {
  it('every food has a picture (FEED-001 kiga labels)', () => {
    for (const f of ['grass', 'melons', 'bamboo', 'eucalyptus', 'hay', 'fish_food', 'bananas', 'leaves', 'meat', 'berries']) {
      expect(FOOD_ICONS[f]).toBeTruthy();
    }
  });
});

describe('GAME-NIGHT icons (no reading needed)', async () => {
  const { ANIMAL_ICONS, PLACE_ICONS, TARGET_ICONS } = await import('./ui');
  it('the two night choices, the night foods, animals and places have pictures', () => {
    expect(TARGET_ICONS.bed).toBe('🛏️');
    expect(TARGET_ICONS.moon_door).toBe('🌙');
    for (const f of ['beetles', 'fruit', 'worms', 'nectar']) expect(FOOD_ICONS[f], f).toBeTruthy();
    for (const a of ['hedgehog', 'bat', 'owl']) expect(ANIMAL_ICONS[a], a).toBeTruthy();
    for (const p of [
      'loc_brush_pile',
      'loc_flowerpots',
      'loc_mushrooms',
      'loc_windmill',
      'loc_fireflies',
      'loc_hollow_tree',
      'loc_moon_pond',
      'loc_hilltop',
      'loc_fir',
    ])
      expect(PLACE_ICONS[p], p).toBeTruthy();
  });
});

describe('garden basket HUD (GAME-GARDEN §4)', () => {
  it('parses the basket and falls back to empty', () => {
    expect(parseBasket('{"carrot":2,"potato":3,"capacity":6,"offered":"carrot"}')).toEqual({
      carrot: 2,
      potato: 3,
      capacity: 6,
      offered: 'carrot',
    });
    expect(parseBasket('')).toEqual({ carrot: 0, potato: 0 });
    expect(parseBasket(undefined)).toEqual({ carrot: 0, potato: 0 });
  });
  it('has pictures for treats and the garden targets (no reading needed)', () => {
    expect(FOOD_ICONS.carrot).toBeTruthy();
    expect(FOOD_ICONS.potato).toBeTruthy();
    for (const k of ['plant', 'garden_sign', 'treat']) expect(TARGET_ICONS[k]).toBeTruthy();
  });
});
