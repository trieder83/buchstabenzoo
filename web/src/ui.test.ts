import { describe, expect, it } from 'vitest';
import { FOOD_ICONS, loadSettings, saveSettings, type KeyValue } from './ui';

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
