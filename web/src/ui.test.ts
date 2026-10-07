import { describe, expect, it } from 'vitest';
import {
  introEnabled,
  FOOD_ICONS,
  HINT_ICONS,
  TARGET_ICONS,
  loadSettings,
  parseBasket,
  parseHint,
  parseLyingIcons,
  parseProgress,
  badgeIcon,
  missingAnimals,
  stripView,
  progressInfoKey,
  saveSettings,
  targetIcon,
  type KeyValue,
  TREATS,
  VIEW_ICONS,
  STRIP_OPEN_MS,
  MATH_LEVELS,
  DEFAULT_MATH_LEVEL,
} from './ui';
import { codeOf, resultSound, turn } from './lock-panel';
import { countParts } from './math-aid';

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
    expect(loadSettings(new MapStore(), 'de')).toEqual({ language: 'de', readingLevel: 'klasse1', view: 'zoo', sound: true, mathLevel: 'mathe1' });
    expect(loadSettings(null, 'de')).toEqual({ language: 'de', readingLevel: 'klasse1', view: 'zoo', sound: true, mathLevel: 'mathe1' });
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
    expect(loadSettings(s, 'de')).toEqual({ language: 'en', readingLevel: 'kiga', view: 'zoo', sound: true, mathLevel: 'mathe1' });
    s.setItem('zoo.language', 'fr');
    s.setItem('zoo.readingLevel', 'klasse9');
    expect(loadSettings(s, 'de')).toEqual({ language: 'de', readingLevel: 'klasse1', view: 'zoo', sound: true, mathLevel: 'mathe1' });
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

describe('sound setting (ASND-009)', () => {
  it('ASND-009: on by default, off is stored as 0 and restored, other saves keep it', () => {
    const s = new MapStore();
    expect(loadSettings(s, 'de').sound).toBe(true);
    saveSettings(s, { language: 'de', readingLevel: 'klasse1', sound: false });
    expect(s.getItem('zoo.sound')).toBe('0');
    expect(loadSettings(s, 'de').sound).toBe(false);
    saveSettings(s, { language: 'en', readingLevel: 'kiga' }); // no sound field: untouched
    expect(loadSettings(s, 'de').sound).toBe(false);
    saveSettings(s, { language: 'en', readingLevel: 'kiga', sound: true });
    expect(loadSettings(s, 'de').sound).toBe(true);
    expect(loadSettings(null, 'de').sound).toBe(true);
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

// LAYOUT-N2-015: the terrarium garden (night_2) needs no reading either
describe('GAME-LEVEL-NIGHT-2 icons', async () => {
  const { ANIMAL_ICONS, PLACE_ICONS } = await import('./ui');
  it('the new foods, animals, places and the lantern-gate hint have pictures', () => {
    for (const f of ['fish', 'crickets', 'flies', 'eggs', 'frozen_insects', 'bone']) expect(FOOD_ICONS[f], f).toBeTruthy();
    for (const a of ['snake', 'chameleon', 'poison_dart_frog']) expect(ANIMAL_ICONS[a], a).toBeTruthy();
    for (const p of ['loc_stone_wall', 'loc_pumpkins', 'loc_rowing_boat', 'loc_lanterns', 'loc_palm', 'loc_vine_arch', 'loc_stepping_stones', 'loc_ferns', 'loc_rain_barrel'])
      expect(PLACE_ICONS[p], p).toBeTruthy();
    expect(HINT_ICONS.night_gate).toBe('🚪');
  });
});

describe('garden basket HUD (GAME-GARDEN §4)', () => {
  it('parses the basket and falls back to empty', () => {
    expect(parseBasket('{"carrot":2,"potato":3,"capacity":6,"offered":"carrot"}')).toEqual({
      carrot: 2,
      potato: 3,
      apple: 0,
      orange: 0,
      capacity: 6,
      offered: 'carrot',
    });
    expect(parseBasket('')).toEqual({ carrot: 0, potato: 0, apple: 0, orange: 0 });
    expect(parseBasket(undefined)).toEqual({ carrot: 0, potato: 0, apple: 0, orange: 0 });
  });
  // GARD-015: fruit treats of the level-3 fruit garden
  it('parses fruit counts and has a picture per treat kind', () => {
    expect(parseBasket('{"carrot":0,"potato":0,"apple":2,"orange":1,"capacity":6,"offered":"apple"}')).toMatchObject({
      apple: 2,
      orange: 1,
      offered: 'apple',
    });
    expect(TREATS).toEqual(['carrot', 'potato', 'apple', 'orange']);
    for (const t of TREATS) expect(FOOD_ICONS[t], t).toBeTruthy();
    expect(FOOD_ICONS.apple).toBe('🍎');
    expect(FOOD_ICONS.orange).toBe('🍊');
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
    const h = parseHint('{"id":"board:zebra","kind":"board","on":false,"x":40,"y":300,"angle":180,"dots":9,"step":"hint-read"}');
    // HINT-016: the next-step key travels with the hint
    expect(h).toEqual({ id: 'board:zebra', kind: 'board', on: false, x: 40, y: 300, angle: 180, dots: 5, step: 'hint-read', animal: '' });
    expect(parseHint('')).toBeNull();
    expect(parseHint('{broken')).toBeNull();
    expect(parseHint('{"kind":"board"}')).toBeNull();
  });
  it('has an icon for every hint kind of rule 2', () => {
    for (const k of ['board', 'food', 'animal', 'gate', 'garden', 'potato', 'apple', 'orange', 'key_box', 'note', 'keybox', 'bed', 'moon_door', 'event', 'pick_up', 'bamboo', 'water', 'treat']) {
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
    // a pair with one animal home and its partner out is marked (½), see GAME-NIGHT rule 11
    const half = parseProgress('{"state":"missing","level":"level_1","animals":[{"id":"hippo","home":false}],"partial":["hippo","x",3],"badge":"animal","badge_animal":"hippo"}');
    expect(half.partial).toEqual(['hippo', 'x']);
    expect(parseProgress('nope')).toEqual({ state: 'hidden', level: '', animals: [], partial: [], badge: '', badgeAnimal: '', next: '' });
  });
  it('NIGHT-022: shows only the animals still missing; none missing = empty list', () => {
    const p = parseProgress('{"state":"missing","level":"level_1","animals":[{"id":"zebra","home":true},{"id":"hippo","home":false},{"id":"panda","home":false}]}');
    expect(missingAnimals(p).map((a) => a.id)).toEqual(['hippo', 'panda']);
    const done = parseProgress('{"state":"night_coming","level":"level_1","animals":[{"id":"zebra","home":true}]}');
    expect(missingAnimals(done)).toEqual([]);
    expect(stripView(done)).toEqual({ ids: [], more: 0 });
  });
  it('NIGHT-022: at most 6 icons, then +n; the badge kind maps to an icon', () => {
    const many = parseProgress(
      `{"state":"missing","level":"l","animals":[${Array.from({ length: 9 }, (_, i) => `{"id":"a${i}","home":false}`).join(',')}],"badge":"board","badge_animal":"zebra"}`,
    );
    expect(stripView(many)).toEqual({ ids: ['a0', 'a1', 'a2', 'a3', 'a4'], more: 4 });
    expect(many.badge).toBe('board');
    expect(many.badgeAnimal).toBe('zebra');
    expect(badgeIcon('board')).toBe('📋');
    expect(badgeIcon('night_coming')).toBe('🌙');
    expect(badgeIcon('bed')).toBe('🛏️');
    expect(badgeIcon('moon_door')).toBe('🚪🌙');
    expect(badgeIcon('')).toBe('');
    expect(badgeIcon('all_done')).toBe('🎉');
    expect(badgeIcon('explore')).toBe('🔍');
  });
});

describe('NIGHT-023: tapping the compass explains it', () => {
  const mk = (state: string, home: boolean) =>
    parseProgress(`{"state":"${state}","level":"level_1","animals":[{"id":"zebra","home":${home}}]}`);
  it('picks the info key per state and reading level', () => {
    expect(progressInfoKey(mk('missing', false), 'klasse2')).toBe('night-progress-info-klasse2');
    expect(progressInfoKey(mk('missing', true), 'klasse1')).toBe('night-progress-info-done-klasse1');
    expect(progressInfoKey(mk('night_coming', true), 'kiga')).toBe('night-progress-info-done-kiga');
    expect(progressInfoKey(mk('night', false), 'klasse3')).toBe('night-progress-info-night-klasse3');
    expect(progressInfoKey(mk('sleep', true), 'klasse2')).toBe('night-progress-info-sleep-klasse2');
  });
  it('HINT-028: only optional things left -> the what-next line (also while the strip is hidden)', () => {
    const p = parseProgress('{"state":"hidden","level":"","animals":[],"badge":"all_done","next":"all_done"}');
    expect(p.next).toBe('all_done');
    expect(progressInfoKey(p, 'kiga')).toBe('night-progress-info-next-all_done-kiga');
    expect(progressInfoKey(parseProgress('{"state":"hidden","next":"explore"}'), 'klasse1')).toBe(
      'night-progress-info-next-explore-klasse1',
    );
  });
});

describe('entrance intro (RESC-029)', () => {
  it('is shown in a normal new game and skipped by automated tests unless ?intro=1', () => {
    expect(introEnabled('', false)).toBe(true);
    expect(introEnabled('', true)).toBe(false);
    expect(introEnabled('?intro=1', true)).toBe(true);
    expect(introEnabled('?intro=0', false)).toBe(false);
  });
});

describe('one view button (CAMV-025) and the small-screen compass strip (HINT-023)', () => {
  it('has one icon per view and the strip closes after about 6 s', () => {
    expect(VIEW_ICONS).toEqual({ zoo: '🗺️', first_person: '👓', look_around: '👁️' });
    expect(STRIP_OPEN_MS).toBe(6000);
  });
});

describe('interact button icon of a plant (GARD-026)', () => {
  it('shows the fruit that grows at the plant, not always a carrot', () => {
    expect(targetIcon('plant', 'plant:apple_1')).toBe('🍎');
    expect(targetIcon('plant', 'plant:orange_2')).toBe('🍊');
    expect(targetIcon('plant', 'plant:potato_w1')).toBe('🥔');
    expect(targetIcon('plant', 'plant:carrot_w1')).toBe('🥕');
  });
});

// CART-030: the math level setting
describe('math level setting (GAME-CART rule 21, CART-030)', () => {
  it('has five levels, default mathe1', () => {
    expect([...MATH_LEVELS]).toEqual(['mathe1', 'mathe2', 'mathe3', 'mathe4', 'mathe5']);
    expect(DEFAULT_MATH_LEVEL).toBe('mathe1');
    expect(loadSettings(new MapStore(), 'de').mathLevel).toBe('mathe1');
  });
  it('is stored as zoo.mathLevel and unknown values are ignored', () => {
    const s = new MapStore();
    saveSettings(s, { language: 'de', readingLevel: 'klasse1', mathLevel: 'mathe4' });
    expect(s.getItem('zoo.mathLevel')).toBe('mathe4');
    expect(loadSettings(s, 'de').mathLevel).toBe('mathe4');
    s.setItem('zoo.mathLevel', 'mathe9');
    expect(loadSettings(s, 'de').mathLevel).toBe('mathe1');
    saveSettings(s, { language: 'de', readingLevel: 'klasse1', mathLevel: 'mathe2' });
    saveSettings(s, { language: 'de', readingLevel: 'klasse1', mathLevel: 'mathe7' });
    expect(s.getItem('zoo.mathLevel')).toBe('mathe2'); // an unknown level is never stored
  });
});

describe('key box lock panel maths (CART-014)', () => {
  it('turns the wheels with wrap-around and composes the code', () => {
    expect(turn(9, 1)).toBe(0);
    expect(turn(0, -1)).toBe(9);
    expect(turn(4, 1)).toBe(5);
    expect(codeOf([0, 0, 5])).toBe(5);
    expect(codeOf([9, 9, 9])).toBe(999);
    expect(codeOf([1, 2, 0])).toBe(120);
  });
  it('shows numbers above 20 as tens-rods and dots (visual aid)', () => {
    expect(countParts(7)).toEqual({ rods: 0, dots: 7 });
    expect(countParts(20)).toEqual({ rods: 0, dots: 20 });
    expect(countParts(47)).toEqual({ rods: 4, dots: 7 });
  });
});

// CART-010: the golf cart icons of the interact button (🛻 board, 🚶 get out)
describe('golf cart icons', () => {
  it('CART-010: cart and get_out have their icons', () => {
    expect(TARGET_ICONS.cart).toBe('🛻');
    expect(TARGET_ICONS.get_out).toBe('🚶');
    expect(targetIcon('get_out', 'get_out')).toBe('🚶');
  });
});

// ASND-038: the lock panel sound of an enter_code result
describe('lock panel sounds (ASND-038)', () => {
  it('maps the results to wrong / ok, others are silent', () => {
    expect(resultSound('wrong')).toBe('wrong');
    expect(resultSound('right')).toBe('ok');
    expect(resultSound('far')).toBeNull();
    expect(resultSound('none')).toBeNull();
  });
});
