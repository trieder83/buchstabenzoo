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
  it('defaults to the device language and klasse1', () => {
    expect(loadSettings(new MapStore(), 'en')).toEqual({ language: 'en', readingLevel: 'klasse1' });
    expect(loadSettings(null, 'de')).toEqual({ language: 'de', readingLevel: 'klasse1' });
  });
  it('round-trips and ignores invalid values', () => {
    const s = new MapStore();
    saveSettings(s, { language: 'en', readingLevel: 'kiga' });
    expect(loadSettings(s, 'de')).toEqual({ language: 'en', readingLevel: 'kiga' });
    s.setItem('zoo.language', 'fr');
    s.setItem('zoo.readingLevel', 'klasse9');
    expect(loadSettings(s, 'de')).toEqual({ language: 'de', readingLevel: 'klasse1' });
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

describe('icons', () => {
  it('every food has a picture (FEED-001 kiga labels)', () => {
    for (const f of ['grass', 'melons', 'bamboo', 'eucalyptus', 'hay', 'fish_food', 'bananas', 'leaves', 'meat', 'berries']) {
      expect(FOOD_ICONS[f]).toBeTruthy();
    }
  });
});
