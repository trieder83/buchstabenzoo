import { describe, expect, it } from 'vitest';
import {
  FOOD_ICONS,
  HINT_ICONS,
  TARGET_ICONS,
  loadSettings,
  parseBasket,
  parseHint,
  parseLyingIcons,
  parseProgress,
  saveSettings,
  targetIcon,
  type KeyValue,
} from './ui';

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

describe('put down and lying items (GAME-FEED §8–11)', () => {
  it('a lying food shows its own icon on the interact button, bamboo spots the bamboo', () => {
    expect(targetIcon('lying_food', 'lying_food:hay:3')).toBe(FOOD_ICONS.hay);
    expect(targetIcon('lying_food', 'lying_food:unknown:3')).toBe(TARGET_ICONS.lying_food);
    expect(targetIcon('bamboo', 'bamboo:cut_bamboo_n1')).toBe('🎋');
    expect(targetIcon('food_box', 'food_box:hay')).toBe(TARGET_ICONS.food_box);
    expect(targetIcon('', '')).toBe('');
  });
  it('parses lying-food icon positions and ignores broken data', () => {
    expect(parseLyingIcons('[{"food":"hay","x":10.5,"y":20}]')).toEqual([{ food: 'hay', x: 10.5, y: 20 }]);
    expect(parseLyingIcons('[{"food":"hay"}, 3, null]')).toEqual([]);
    expect(parseLyingIcons('nope')).toEqual([]);
    expect(parseLyingIcons(undefined)).toEqual([]);
  });
});

describe('GAME-HINT overlay data (HINT-007)', () => {
  it('parses a shown hint and clamps the distance dots to 1…5', () => {
    const h = parseHint('{"id":"board:zebra","kind":"board","on":false,"x":40,"y":300,"angle":180,"dots":9}');
    expect(h).toEqual({ id: 'board:zebra', kind: 'board', on: false, x: 40, y: 300, angle: 180, dots: 5 });
    expect(parseHint('')).toBeNull();
    expect(parseHint('{broken')).toBeNull();
    expect(parseHint('{"kind":"board"}')).toBeNull();
  });
  it('has an icon for every hint kind of rule 2', () => {
    for (const k of ['board', 'food', 'animal', 'gate', 'garden', 'key_box', 'bed', 'moon_door', 'event', 'pick_up', 'bamboo', 'water', 'treat']) {
      expect(HINT_ICONS[k], k).toBeTruthy();
    }
  });
});

describe('GAME-NIGHT rule 11: night progress (NIGHT-019)', () => {
  it('parses the animals and their state; broken JSON is hidden', () => {
    const p = parseProgress('{"state":"missing","level":"level_1","animals":[{"id":"zebra","home":true},{"id":"hippo","home":false}]}');
    expect(p.state).toBe('missing');
    expect(p.animals).toEqual([
      { id: 'zebra', home: true },
      { id: 'hippo', home: false },
    ]);
    expect(parseProgress('nope')).toEqual({ state: 'hidden', level: '', animals: [] });
  });
});
