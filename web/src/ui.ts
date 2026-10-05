// HTML overlays of the host shell (GAME-PLAYER §3/§4, GAME-FEED §6, CONT-L10N): interact
// button / key hint, text panel, carried-food HUD, settings menu, feedback bubble and the
// mission celebration. Every text comes from the game (Fluent via `App.t` / `App.interact`);
// icons are placeholders (emoji) until the icon art exists.

import { OverviewMap } from './overview-map';
import { TelescopeView } from './telescope';
import { SOUND_EVENT } from './audio';
import { browserEnv, isFullscreen, showFullscreenButton, showIosInstallHint, toggleFullscreen } from './fullscreen';
import { dragScroll } from './scroll';
import { pictogramCanvas } from './pictograms';

/** The gate intro (RESC-029) is shown in a new game; automated tests skip it unless `?intro=1`. */
export function introEnabled(search: string, webdriver: boolean): boolean {
  const q = new URLSearchParams(search).get('intro');
  if (q !== null) return q !== '0';
  return !webdriver;
}

/** The subset of the WASM `App` the UI needs. */
export interface UiApp {
  t(key: string): string;
  target_kind(): string;
  target_key(): string;
  interact(): string;
  close_panel(): void;
  panel_key(): string;
  panel_json(): string;
  take_food(food: string): boolean;
  carry_food(): string;
  carry_text(): string;
  /** Carried fish bowl: '' (none), 'empty', 'water', 'fish' (RESC-020). */
  carry_bowl?(): string;
  poll_events(): string;
  set_language(id: string): boolean;
  set_reading_level(id: string): boolean;
  language(): string;
  reading_level(): string;
  /** Camera views (GAME-CAMERA-VIEWS): the view to store, the first-person toggle. */
  saved_view_mode?(): string;
  view_mode?(): string;
  toggle_first_person?(): string;
  cycle_view?(): string;
  /** GAME-NIGHT: time of day and the dream fade of the sleep (0…1). */
  daytime?(): string;
  sleep_fade?(): number;
  /** GAME-GARDEN: the treat basket as JSON `{"carrot", "potato", "apple", "orange", "capacity", "offered"}`. */
  basket_json?(): string;
  /** GAME-FEED §8: something droppable is in the hands; put it down (false: nothing dropped). */
  can_put_down?(): boolean;
  put_down?(): boolean;
  /** GAME-FEED §10: lying foods near the player with their screen position (CSS px). */
  lying_icons_json?(): string;
  /** GAME-HINT: the 🧭 button / `H`; the shown hint as JSON (empty = none); idle pulses. */
  hint_press?(): string;
  hint_json?(): string;
  hint_pulses?(): number;
  /** GAME-RESCUE: the entrance intro (RESC-029): still to show / shown or skipped. */
  intro_pending?(): boolean;
  intro_done?(): void;
  /** GAME-NIGHT rule 11: the 🌙 night progress as JSON. */
  compass_json?(): string;
  /** GAME-MAP: the overview map data (JSON) and the pause while it is open. */
  overview_json?(): string;
  set_map_open?(open: boolean): void;
  /** GAME-TELESCOPE: the planet table (JSON) and the pause while the view is open. */
  telescope_json?(): string;
  set_telescope_open?(open: boolean): void;
}

/** Welcome board pictures (RESC-028, as the intro): empty enclosure, paw prints, riddle. */
export const WELCOME_PICTURES = '🏚️🐾❓ → 🔍 → 🥕 → 🏠';
/** Icons of the four steps: riddle board, food, animal, home. */
export const WELCOME_STEP_ICONS = ['📋', '🥕', '🐾', '🏠'];

/** Icons of the hint targets (GAME-HINT rule 2). */
export const HINT_ICONS: Record<string, string> = {
  board: '📋',
  food: '📦',
  animal: '🐾',
  gate: '🚪',
  pick_up: '📦',
  bamboo: '🎋',
  water: '💧',
  garden: '🥕',
  potato: '🥔',
  apple: '🍎',
  orange: '🍊',
  treat: '🧺',
  bed: '🛏️',
  help: '👀',
  moon_door: '🌙',
  // the lantern gate to the terrarium garden (night_2)
  night_gate: '🚪',
  key_box: '🔑',
  event: '❗',
};

/** The shown hint (from `hint_json`). */
export interface HintView {
  id: string;
  kind: string;
  on: boolean;
  x: number;
  y: number;
  angle: number;
  dots: number;
  /** Fluent key of the next-step line (`hint-read`, …; GAME-HINT rule 4a). */
  step: string;
  /** The animal the hint is about ('' = none); its strip icon pulses. */
  animal: string;
}

/** Parses the hint JSON; null when no hint is shown or the JSON is broken. */
export function parseHint(json: string | undefined): HintView | null {
  if (!json) return null;
  try {
    const h = JSON.parse(json) as Partial<HintView>;
    if (typeof h.kind !== 'string' || typeof h.x !== 'number' || typeof h.y !== 'number') return null;
    return {
      id: String(h.id ?? ''),
      kind: h.kind,
      on: Boolean(h.on),
      x: h.x,
      y: h.y,
      angle: typeof h.angle === 'number' ? h.angle : 90,
      dots: Math.max(1, Math.min(5, Math.round(h.dots ?? 1))),
      step: typeof h.step === 'string' ? h.step : '',
      animal: typeof h.animal === 'string' ? h.animal : '',
    };
  } catch {
    return null;
  }
}

/** Icon of the CURRENT view on the one view button (GAME-CAMERA-VIEWS 3a). */
export const VIEW_ICONS: Record<string, string> = { zoo: '🗺️', first_person: '👓', look_around: '👁️' };
/** How long the compass strip stays open on small screens after a tap (HINT-023). */
export const STRIP_OPEN_MS = 6000;

/** The compass strip and task badge (GAME-NIGHT rule 11). */
export interface NightProgress {
  state: string;
  level: string;
  animals: { id: string; home: boolean }[];
  /** Species with one member home and its partner still out (shown as ½). */
  partial: string[];
  /** Kind of the next task (the best hint candidate), `night_coming`, or '' for none. */
  badge: string;
  badgeAnimal: string;
  /** Only optional things left (HINT-028): '' | 'explore' | 'all_done'. */
  next: string;
}

/** Strip icons shown at most; the rest is a "+n" chip. */
export const STRIP_MAX = 6;

/** Only the animals still missing: a finished animal disappears from the strip (GAME-NIGHT rule 11). */
export function missingAnimals(p: NightProgress): NightProgress['animals'] {
  return p.animals.filter((a) => !a.home);
}

/** The strip: the first icons of the missing animals and how many more there are (NIGHT-022). */
export function stripView(p: NightProgress): { ids: string[]; more: number } {
  const ids = missingAnimals(p).map((a) => a.id);
  return ids.length > STRIP_MAX ? { ids: ids.slice(0, STRIP_MAX - 1), more: ids.length - (STRIP_MAX - 1) } : { ids, more: 0 };
}

/** Icon of the task badge: the hint icons, the moon door as 🚪🌙, night coming as 🌙. */
export function badgeIcon(kind: string): string {
  if (kind === 'moon_door') return '🚪🌙';
  if (kind === 'night_coming') return '🌙';
  if (kind === 'all_done') return '🎉';
  if (kind === 'explore') return '🔍';
  return kind ? (HINT_ICONS[kind] ?? '⭐') : '';
}

/** Fluent key of the info bubble shown when the compass is tapped (NIGHT-023). */
export function progressInfoKey(p: NightProgress, readingLevel: string): string {
  if (p.next !== '') return `night-progress-info-next-${p.next}-${readingLevel}`;
  const mid = p.state === 'night_coming' || (p.state === 'missing' && missingAnimals(p).length === 0)
    ? 'done-'
    : p.state === 'night'
      ? 'night-'
      : p.state === 'sleep'
        ? 'sleep-'
        : '';
  return `night-progress-info-${mid}${readingLevel}`;
}

/** Parses the compass JSON; hidden on errors. */
export function parseProgress(json: string | undefined): NightProgress {
  try {
    const p = JSON.parse(json ?? '') as Partial<NightProgress> & { badge_animal?: string };
    const animals = Array.isArray(p.animals)
      ? p.animals
          .filter((a) => a && typeof a.id === 'string')
          .map((a) => ({ id: a.id, home: Boolean(a.home) }))
      : [];
    return {
      state: typeof p.state === 'string' ? p.state : 'hidden',
      level: String(p.level ?? ''),
      animals,
      partial: Array.isArray(p.partial) ? p.partial.filter((x): x is string => typeof x === 'string') : [],
      badge: typeof p.badge === 'string' ? p.badge : '',
      badgeAnimal: typeof p.badge_animal === 'string' ? p.badge_animal : '',
      next: typeof p.next === 'string' ? p.next : '',
    };
  } catch {
    return { state: 'hidden', level: '', animals: [], partial: [], badge: '', badgeAnimal: '', next: '' };
  }
}

/** Treat kinds of the basket, in HUD order (GAME-GARDEN §5: vegetables, fruit). */
export const TREATS = ['carrot', 'potato', 'apple', 'orange'] as const;

/** Treat basket contents (GAME-GARDEN §4). */
export interface Basket {
  carrot: number;
  potato: number;
  apple: number;
  orange: number;
  capacity?: number;
  offered?: string;
}

/** Parses the basket JSON; empty basket on errors. */
export function parseBasket(json: string | undefined): Basket {
  try {
    const b = JSON.parse(json ?? '') as Partial<Basket>;
    return {
      carrot: Math.max(0, b.carrot ?? 0),
      potato: Math.max(0, b.potato ?? 0),
      apple: Math.max(0, b.apple ?? 0),
      orange: Math.max(0, b.orange ?? 0),
      capacity: b.capacity,
      offered: b.offered,
    };
  } catch {
    return { carrot: 0, potato: 0, apple: 0, orange: 0 };
  }
}

export const LANGUAGES = ['de', 'en'] as const;
export const READING_LEVELS = ['kiga', 'klasse1', 'klasse2', 'klasse3'] as const;

/** Emoji food pictures for the HUD / lists (food box labels use the vector pictograms.ts). */
export const FOOD_ICONS: Record<string, string> = {
  grass: '🌿',
  melons: '🍉',
  bamboo: '🎋',
  eucalyptus: '🍃',
  hay: '🌾',
  fish_food: '🐟',
  bananas: '🍌',
  leaves: '🍂',
  meat: '🥩',
  berries: '🫐',
  // garden treats (GAME-GARDEN)
  carrot: '🥕',
  potato: '🥔',
  apple: '🍎',
  orange: '🍊',
  // night zoo (GAME-NIGHT rule 6)
  beetles: '🪲',
  fruit: '🍎',
  worms: '🪱',
  nectar: '🌺',
  // night_2 (terrarium garden)
  fish: '🐟',
  crickets: '🦗',
  flies: '🪰',
  eggs: '🥚',
  frozen_insects: '🧊',
  bone: '🦴',
};

/** Placeholder pictures of hiding places (kiga riddle). */
export const PLACE_ICONS: Record<string, string> = {
  loc_river: '🏞️',
  loc_meadow: '🌼',
  loc_sand: '🏖️',
  loc_pond: '🪷',
  loc_mud: '🟤',
  loc_shade: '🌳',
  loc_cave: '🕳️',
  loc_bamboo: '🎋',
  loc_leaves: '🍂',
  loc_treehouse: '🛖',
  loc_tallest_tree: '🌲',
  loc_blossom_tree: '🌸',
  loc_mud_pool: '🟤',
  loc_fountain: '⛲',
  loc_log_pile: '🪵',
  loc_big_ball: '🔴',
  loc_lookout_tower: '🗼',
  loc_train: '🚂',
  loc_playground: '🛝',
  loc_sun_rocks: '🪨',
  loc_stage: '🥁',
  loc_deckchairs: '⛱️',
  loc_pirate_ship: '🏴‍☠️',
  loc_carousel: '🎠',
  loc_trampoline: '🤸',
  loc_waterfall: '💦',
  loc_water_wheel: '⚙️',
  loc_willow: '🌿',
  loc_ice_cream_kiosk: '🍦',
  loc_sprinkler: '🌈',
  loc_laundry: '👕',
  // night zoo `night_1`
  loc_brush_pile: '🪵',
  loc_flowerpots: '🪴',
  loc_mushrooms: '🍄',
  loc_windmill: '🌬️',
  loc_fireflies: '✨',
  loc_hollow_tree: '🌳',
  loc_moon_pond: '🌕',
  loc_hilltop: '⛰️',
  loc_fir: '🎄',
  // night zoo `night_2` (terrarium garden)
  loc_stone_wall: '🧱',
  loc_pumpkins: '🎃',
  loc_rowing_boat: '🚣',
  loc_lanterns: '🏮',
  loc_palm: '🌴',
  loc_vine_arch: '🌸',
  loc_stepping_stones: '👣',
  loc_ferns: '🌿',
  loc_rain_barrel: '🛢️',
};

/** Placeholder pictures of the carried fish bowl (HUD, RESC-020). */
export const BOWL_ICONS: Record<string, string> = {
  empty: '🫙',
  water: '💧',
  fish: '🐠',
};

/** Placeholder animal pictures (info board heading, kiga facts picture). */
export const ANIMAL_ICONS: Record<string, string> = {
  zebra: '🦓',
  hippo: '🦛',
  panda: '🐼',
  koala: '🐨',
  elephant: '🐘',
  goldfish: '🐠',
  monkey: '🐒',
  giraffe: '🦒',
  lion: '🦁',
  snow_fox: '🦊',
  hedgehog: '🦔',
  bat: '🦇',
  owl: '🦉',
  snake: '🐍',
  chameleon: '🦎',
  poison_dart_frog: '🐸',
};

/** Icon of the interact button per target kind. */
export const TARGET_ICONS: Record<string, string> = {
  info_board: '👀',
  food_box: '✋',
  animal: '🤲',
  gate: '🏠',
  item: '🫙',
  water: '💧',
  put_down: '⬇️',
  // GAME-FEED §11/§14: pick up a lying food (its own icon, see targetIcon), cut bamboo
  lying_food: '📦',
  bamboo: '🎋',
  // GAME-NIGHT rule 3: the two night choices, no reading needed
  bed: '🛏️',
  moon_door: '🌙',
  // GAME-GARDEN: pull a plant, read a garden sign, give a treat at the fence
  plant: '🥕',
  garden_sign: '👀',
  // RESC-028: the welcome board at the level entry
  welcome_board: '🗺️',
  // GAME-TELESCOPE: look at the planets (night only)
  telescope: '🔭',
  treat: '🧺',
};

/**
 * Icon of the interact button for a target: a lying food shows its food (key
 * `lying_food:<food>:<uid>`), everything else the kind's icon.
 */
export function targetIcon(kind: string, key: string): string {
  if (kind === 'plant') {
    // the interact button shows WHAT grows at the plant (user report 2026-10-04: always a carrot)
    const spot = key.split(':')[1] ?? '';
    return spot.includes('apple') ? '🍎' : spot.includes('orange') ? '🍊' : spot.includes('potato') ? '🥔' : TARGET_ICONS.plant;
  }
  if (kind === 'lying_food') {
    const food = key.split(':')[1] ?? '';
    return FOOD_ICONS[food] ?? TARGET_ICONS.lying_food;
  }
  return TARGET_ICONS[kind] ?? '';
}

/** Lying-food icon positions (GAME-FEED §10) from the JSON of `lying_icons_json`. */
export interface LyingIcon {
  food: string;
  x: number;
  y: number;
}

export function parseLyingIcons(json: string | undefined): LyingIcon[] {
  if (!json) return [];
  try {
    const v = JSON.parse(json) as unknown;
    if (!Array.isArray(v)) return [];
    return v.filter(
      (i): i is LyingIcon =>
        typeof i === 'object' &&
        i !== null &&
        typeof (i as LyingIcon).food === 'string' &&
        typeof (i as LyingIcon).x === 'number' &&
        typeof (i as LyingIcon).y === 'number',
    );
  } catch {
    return [];
  }
}

const LEVEL_ICONS: Record<string, string> = { kiga: '🧸', klasse1: '1', klasse2: '2', klasse3: '3' };
const LANG_ICONS: Record<string, string> = { de: '🇩🇪', en: '🇬🇧' };

/** Minimal storage interface (localStorage in the browser, a Map in tests). */
export interface KeyValue {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
}

export interface Settings {
  language: string;
  readingLevel: string;
  /** Camera view (GAME-CAMERA-VIEWS 9): `zoo` (default) or `first_person`. */
  view?: string;
  /** Sound effects on (ASND-009); default on, stored as `zoo.sound` = `0` when off. */
  sound?: boolean;
}

/** Views that are stored (look-around is only held, never stored). */
export const SAVED_VIEWS = ['zoo', 'first_person'] as const;

const KEY_LANG = 'zoo.language';
const KEY_LEVEL = 'zoo.readingLevel';
const KEY_VIEW = 'zoo.view';
const KEY_SOUND = 'zoo.sound';

/**
 * Stored settings, falling back to `defaultLanguage` (always `de`, CONT-L10N §5 — the stored
 * choice from the settings wins) and `klasse1`. Invalid stored values are ignored.
 */
export function loadSettings(store: KeyValue | null, defaultLanguage: string): Settings {
  let lang: string | null = null;
  let level: string | null = null;
  let view: string | null = null;
  let sound: string | null = null;
  try {
    lang = store?.getItem(KEY_LANG) ?? null;
    level = store?.getItem(KEY_LEVEL) ?? null;
    view = store?.getItem(KEY_VIEW) ?? null;
    sound = store?.getItem(KEY_SOUND) ?? null;
  } catch {
    // storage blocked (private mode): defaults
  }
  return {
    language: (LANGUAGES as readonly string[]).includes(lang ?? '') ? lang! : defaultLanguage,
    readingLevel: (READING_LEVELS as readonly string[]).includes(level ?? '') ? level! : 'klasse1',
    view: (SAVED_VIEWS as readonly string[]).includes(view ?? '') ? view! : 'zoo',
    sound: sound !== '0',
  };
}

export function saveSettings(store: KeyValue | null, s: Settings): void {
  try {
    store?.setItem(KEY_LANG, s.language);
    store?.setItem(KEY_LEVEL, s.readingLevel);
    if (s.view && (SAVED_VIEWS as readonly string[]).includes(s.view)) store?.setItem(KEY_VIEW, s.view);
    if (s.sound !== undefined) store?.setItem(KEY_SOUND, s.sound ? '1' : '0');
  } catch {
    // storage blocked: settings live for this session only
  }
}

interface PanelData {
  kind: string;
  key?: string;
  animal?: string;
  /** Info board: animal name, "more about" heading and facts (GAME-ANIMALS "Info board" 4). */
  title?: string;
  more?: string;
  facts?: string;
  text?: string;
  picture?: string | boolean | null;
  /** Food box panel (FEED-033): pictogram id and its share of the label height. */
  pictogram?: string;
  pictogram_scale?: number;
  food?: string;
  food_text?: string;
  take?: string;
  /** Info board: a container is needed (the goldfish bowl hint). */
  hint?: string | null;
  /** Welcome board (RESC-028): level goal, animals, steps, start hint, level note. */
  goal?: string;
  animals?: string[];
  steps?: string[];
  start?: string;
  level_text?: string;
  level?: string;
}

export interface GameEventMsg {
  level?: string;
  type: string;
  panel?: PanelData;
  animal?: string;
  key?: string;
  text?: string;
  food?: string;
  into_night_zoo?: boolean;
}

/** Corner-bracket icon (outward = enter, inward = leave); an SVG because the glyph ⛶ is missing in many fonts. */
function fullscreenIcon(on: boolean): SVGSVGElement {
  const ns = 'http://www.w3.org/2000/svg';
  const svg = document.createElementNS(ns, 'svg');
  svg.setAttribute('viewBox', '0 0 24 24');
  svg.setAttribute('width', '40');
  svg.setAttribute('height', '40');
  const path = document.createElementNS(ns, 'path');
  path.setAttribute(
    'd',
    on
      ? 'M9 3v6H3M15 3v6h6M9 21v-6H3M15 21v-6h6'
      : 'M3 9V3h6M21 9V3h-6M3 15v6h6M21 15v6h-6',
  );
  path.setAttribute('fill', 'none');
  path.setAttribute('stroke', '#3b2314');
  path.setAttribute('stroke-width', '3');
  path.setAttribute('stroke-linecap', 'round');
  path.setAttribute('stroke-linejoin', 'round');
  svg.append(path);
  return svg;
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

/** Smallest letter size the riddle may shrink to so it fits without scrolling (Q-070). */
export const MIN_READ_PX = 18;

/**
 * Shrinks the reading size (`--read`) until the riddle + food word (`.panel-main`) fit in
 * the panel without scrolling, never below {@link MIN_READ_PX}; the facts part scrolls.
 * Returns the final size in px.
 */
export function fitReadingText(body: HTMLElement): number {
  const main = body.querySelector<HTMLElement>('.panel-main');
  const cs = getComputedStyle(body);
  let px = parseFloat(getComputedStyle(body.querySelector('#panel-text') ?? body).fontSize) || 24;
  if (!main) return px;
  const available = () => {
    const max = parseFloat(cs.maxHeight);
    const pad = parseFloat(cs.paddingTop) + parseFloat(cs.paddingBottom) + parseFloat(cs.borderTopWidth) * 2;
    return (Number.isFinite(max) ? max : body.clientHeight) - pad;
  };
  while (main.offsetHeight > available() && px > MIN_READ_PX) {
    px = Math.max(MIN_READ_PX, px - 2);
    body.style.setProperty('--read', `${px}px`);
  }
  return px;
}

export class Ui {
  private panelKey: string | null = null;
  private panelFood: string | null = null;
  private lastCarry = '\u0000';
  private lastTarget = '\u0000';
  private bubbleTimer = 0;
  private touch = false;
  private lastView = '';
  private lastDaytime = '';
  private bannerTimer = 0;
  /** Sound effects switch (ASND-009). */
  private sound = true;

  readonly act = document.getElementById('act') as HTMLButtonElement;
  readonly hint = document.getElementById('hint') as HTMLButtonElement;
  readonly panel = document.getElementById('panel') as HTMLDivElement;
  readonly hud = document.getElementById('hud-carry') as HTMLDivElement;
  /** GAME-FEED §8: the put-down button next to the carried item (✋⬇, no text). */
  readonly dropBtn = document.getElementById('drop-btn') as HTMLButtonElement | null;
  /** GAME-FEED §10: readable icons above lying foods near the player. */
  readonly lyingIcons = document.getElementById('lying-icons') as HTMLDivElement | null;
  private lastLying = '';
  /** GAME-HINT: the 🧭 button, the indicator above the target and the edge arrow. */
  readonly compass = document.getElementById('compass-btn') as HTMLButtonElement | null;
  readonly hintMarker = document.getElementById('hint-marker') as HTMLDivElement | null;
  readonly hintEdge = document.getElementById('hint-edge') as HTMLDivElement | null;
  /** GAME-NIGHT rule 11: the task badge and the strip of missing animals on the compass. */
  private readonly compassBadge = document.querySelector('#compass-btn .badge') as HTMLElement | null;
  private readonly compassStrip = document.querySelector('#compass-btn .strip') as HTMLElement | null;
  private lastCompassPoll = 0;
  private hintAnimal = '';
  private lastHintKind = '';
  private lastHintStep = '';
  private lastHintDots = -1;
  private lastPulses = 0;
  private lastProgress = '';
  readonly gear = document.getElementById('settings-btn') as HTMLButtonElement;
  readonly settings = document.getElementById('settings') as HTMLDivElement;
  readonly bubble = document.getElementById('bubble') as HTMLDivElement;
  readonly celebrate = document.getElementById('celebrate') as HTMLDivElement;
  /** The ONE view button (GAME-CAMERA-VIEWS 3a): zoo -> first person -> look-around -> zoo. */
  readonly viewBtn = document.getElementById('view-btn') as HTMLButtonElement | null;
  /** Compass strip expanded on small screens (HINT-023): collapse timer. */
  private stripTimer: ReturnType<typeof setTimeout> | null = null;
  private safeProbe: HTMLElement | null = null;
  private safe = { left: 0, top: 0, right: 0, bottom: 0 };
  private safeW = -1;
  private safeH = -1;
  /** GAME-NIGHT: dusk cut-in text, dream fade, the night choice icons. */
  readonly nightBanner = document.getElementById('night-banner') as HTMLDivElement | null;
  readonly dream = document.getElementById('dream') as HTMLDivElement | null;
  readonly nightChoices = document.getElementById('night-choices') as HTMLDivElement | null;

  constructor(
    private readonly app: UiApp,
    private readonly store: KeyValue | null,
    private readonly onNewGame: () => void = () => {},
    introEnabled = true,
  ) {
    for (const b of [this.act, this.hint]) {
      b.addEventListener('pointerdown', (e) => {
        e.preventDefault();
        e.stopPropagation();
        this.interact();
      });
    }
    this.dropBtn?.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      e.stopPropagation();
      this.putDown();
    });
    // the badge and the strip are children of the button: a tap on them is this tap
    this.compass?.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      e.stopPropagation();
      // everything done: the compass offers the campaign carousel (ADS-031, HINT-031)
      if (this.allDoneTap()) return;
      this.expandStrip();
      this.pressHint();
      this.sayProgressInfo();
    });
    // any other tap collapses the strip again (small screens, HINT-023)
    window.addEventListener(
      'pointerdown',
      (e) => {
        if (!(e.target instanceof Node) || !this.compass?.contains(e.target)) this.collapseStrip();
      },
      true,
    );
    this.map = new OverviewMap(this.app, ANIMAL_ICONS, HINT_ICONS, () => {
      this.settings.hidden = true;
    });
    this.telescope = new TelescopeView(this.app, () => {
      this.settings.hidden = true;
    });
    this.gear.addEventListener('click', () => this.toggleSettings());
    // pointerdown, not click: a second finger (left thumb on the stick) never gets a click
    // (CAMV-019)
    this.viewBtn?.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      e.stopPropagation();
      this.app.cycle_view?.();
      this.update();
    });
    this.sound = loadSettings(this.store, 'de').sound !== false;
    this.buildSettings();
    this.applyLabels();
    if (introEnabled) this.showIntro();
  }

  /** Touch controls on (first touch, PLAY-014). */
  setTouch(): void {
    this.touch = true;
    document.body.classList.add('touch');
    this.lastTarget = '\u0000';
  }

  /** Interact button / key: take from an open food panel, else interact with the target. */
  interact(): void {
    if (this.map.isOpen) return; // the game is paused behind the overview map (MAP-006)
    if (this.telescope.isOpen) {
      this.telescope.close(); // the interact key again closes the telescope view (TELE-007)
      return;
    }
    if (this.panelFood && this.app.panel_key() === this.panelKey) {
      this.take();
      return;
    }
    const json = this.app.interact();
    if (!json) return;
    const data = JSON.parse(json) as PanelData;
    if (data.kind === 'telescope') {
      this.telescope.open();
      return;
    }
    if (data.kind === 'info_board' || data.kind === 'food_box' || data.kind === 'garden_sign' || data.kind === 'welcome_board')
      this.openPanel(data);
    this.pollEvents();
  }

  /** Small screens: the compass strip opens for ~6 s (CSS shows it only on small screens). */
  private expandStrip(): void {
    this.compass?.classList.add('expanded');
    if (this.stripTimer) clearTimeout(this.stripTimer);
    this.stripTimer = setTimeout(() => this.collapseStrip(), STRIP_OPEN_MS);
  }

  private collapseStrip(): void {
    if (this.stripTimer) clearTimeout(this.stripTimer);
    this.stripTimer = null;
    this.compass?.classList.remove('expanded');
  }

  /** Safe-area insets in CSS px (a probe element with `env()` padding; cached per window size). */
  private safeInsets(): { left: number; top: number; right: number; bottom: number } {
    if (this.safeW === window.innerWidth && this.safeH === window.innerHeight) return this.safe;
    this.safeW = window.innerWidth;
    this.safeH = window.innerHeight;
    if (!this.safeProbe) {
      const e = document.createElement('div');
      e.style.cssText =
        'position:fixed;left:0;top:0;width:0;height:0;visibility:hidden;pointer-events:none;' +
        'padding:env(safe-area-inset-top) env(safe-area-inset-right) env(safe-area-inset-bottom) env(safe-area-inset-left)';
      document.body.appendChild(e);
      this.safeProbe = e;
    }
    const cs = getComputedStyle(this.safeProbe);
    this.safe = {
      left: parseFloat(cs.paddingLeft) || 0,
      top: parseFloat(cs.paddingTop) || 0,
      right: parseFloat(cs.paddingRight) || 0,
      bottom: parseFloat(cs.paddingBottom) || 0,
    };
    return this.safe;
  }

  /** The 🧭 button (GAME-HINT rule 1/8): show the next target. */
  pressHint(): void {
    // works while a reading panel is open: the panel closes first (GAME-HINT rule 4a)
    if (this.panelKey) this.closePanel();
    this.app.hint_press?.();
    this.updateHint();
  }

  /**
   * The shown hint (GAME-HINT rule 2): a bouncing indicator above the target when it is on
   * screen, else an edge arrow at the border with its icon and the distance dots.
   */
  private updateHint(): void {
    if (!this.hintMarker || !this.hintEdge || !this.app.hint_json) return;
    const h = parseHint(this.app.hint_json());
    this.compass?.classList.toggle('on', h !== null);
    this.setHintAnimal(h?.animal ?? '');
    if (!h) {
      if (!this.hintMarker.hidden) this.hintMarker.hidden = true;
      if (!this.hintEdge.hidden) this.hintEdge.hidden = true;
      this.lastHintKind = '';
      return;
    }
    const icon = HINT_ICONS[h.kind] ?? '⭐';
    if (h.kind !== this.lastHintKind) {
      this.lastHintKind = h.kind;
      for (const e of [this.hintMarker, this.hintEdge]) {
        e.dataset.kind = h.kind;
        (e.querySelector('.icon') as HTMLElement).textContent = icon;
      }
    }
    this.hintMarker.dataset.id = h.id;
    if (h.step !== this.lastHintStep) {
      this.lastHintStep = h.step;
      const line = h.step ? this.app.t(h.step) : '';
      for (const e of [this.hintMarker, this.hintEdge]) {
        const l = e.querySelector('.line') as HTMLElement | null;
        if (l) l.textContent = line;
      }
    }
    this.hintEdge.dataset.id = h.id;
    this.hintMarker.hidden = !h.on;
    this.hintEdge.hidden = h.on;
    if (h.on) {
      this.hintMarker.style.transform = `translate(${h.x.toFixed(0)}px, ${h.y.toFixed(0)}px) translate(-50%, -100%)`;
    } else {
      // keep the whole arrow (icon + step line) on screen: the edge point is clamped by the
      // size of the element, so the text is never cut off at the border
      const halfW = this.hintEdge.offsetWidth / 2;
      const halfH = this.hintEdge.offsetHeight / 2;
      // inside the safe area (notch, rounded corners, system bars) plus a 16 px margin
      const sa = this.safeInsets();
      const loX = sa.left + 16 + halfW;
      const hiX = window.innerWidth - sa.right - 16 - halfW;
      const loY = sa.top + 16 + halfH;
      const hiY = window.innerHeight - sa.bottom - 16 - halfH;
      const ex = Math.min(Math.max(h.x, loX), Math.max(loX, hiX));
      const ey = Math.min(Math.max(h.y, loY), Math.max(loY, hiY));
      this.hintEdge.style.transform = `translate(${ex.toFixed(0)}px, ${ey.toFixed(0)}px) translate(-50%, -50%)`;
      (this.hintEdge.querySelector('.spin') as HTMLElement).style.transform = `rotate(${h.angle.toFixed(0)}deg)`;
      if (h.dots !== this.lastHintDots) {
        this.lastHintDots = h.dots;
        const dots = this.hintEdge.querySelector('.dots') as HTMLElement;
        dots.replaceChildren(...Array.from({ length: h.dots }, () => el('i')));
        this.hintEdge.dataset.dots = String(h.dots);
      }
    }
  }

  /**
   * The intro at the entrance gate (GAME-RESCUE, RESC-029): 3 pages — the animals broke out,
   * find them and bring them back to the right enclosure, find the food they like. One
   * picture and one short text per page, a big next arrow, skippable; afterwards the 🧭 hint
   * shows the first step (`intro_done`).
   */
  showIntro(force = false): void {
    const box = document.getElementById('intro');
    if (!box || (!force && !this.app.intro_pending?.())) return;
    const pics = ['🏚️🐾❓', '🐘➡️🏠', '🥕➡️🐘🚶'];
    let page = 0;
    const text = box.querySelector('.intro-text') as HTMLElement;
    const pic = box.querySelector('.intro-pic') as HTMLElement;
    const next = box.querySelector('.intro-next') as HTMLButtonElement;
    const skip = box.querySelector('.intro-skip') as HTMLButtonElement;
    const dots = box.querySelector('.intro-dots') as HTMLElement;
    const render = (): void => {
      pic.textContent = pics[page];
      text.textContent = this.app.t(`intro-${page + 1}-${this.app.reading_level()}`);
      next.textContent = page === pics.length - 1 ? '✔' : '➤';
      next.setAttribute('aria-label', this.app.t(page === pics.length - 1 ? 'intro-go' : 'intro-next'));
      skip.setAttribute('aria-label', this.app.t('intro-skip'));
      dots.replaceChildren(...pics.map((_, i) => el('i', i === page ? 'on' : '')));
      box.dataset.page = String(page + 1);
    };
    const finish = (): void => {
      box.hidden = true;
      document.removeEventListener('keydown', onKey);
      this.app.intro_done?.();
      this.updateHint();
    };
    const advance = (): void => {
      if (page >= pics.length - 1) finish();
      else {
        page += 1;
        render();
      }
    };
    const onKey = (e: KeyboardEvent): void => {
      if (e.key === 'Enter' || e.key === ' ') advance();
      else if (e.key === 'Escape') finish();
      else return;
      e.preventDefault();
      e.stopPropagation();
    };
    next.onclick = advance;
    skip.onclick = finish;
    document.addEventListener('keydown', onKey, true);
    box.hidden = false;
    render();
  }

  /** Idle nudge (GAME-HINT rule 6): the 🧭 button pulses gently once; no popup. */
  private updatePulse(): void {
    const n = this.app.hint_pulses?.() ?? 0;
    if (n === this.lastPulses || !this.compass) return;
    this.lastPulses = n;
    this.compass.classList.remove('pulse');
    void this.compass.offsetWidth; // restart the animation
    this.compass.classList.add('pulse');
    this.compass.dataset.pulses = String(n);
  }

  /** The strip icon of the hinted animal pulses (NIGHT-023). */
  private setHintAnimal(id: string): void {
    if (id === this.hintAnimal) return;
    this.hintAnimal = id;
    this.compassStrip?.querySelectorAll<HTMLElement>('.pa[data-animal]').forEach((e) => {
      e.classList.toggle('hl', id !== '' && e.dataset.animal === id);
    });
  }

  /**
   * The compass (GAME-NIGHT rule 11): the task badge (kind of the next task) and the strip of
   * the missing animals. Polled at <= 4 Hz; the DOM is only touched when the JSON changes.
   */
  private updateProgress(): void {
    if (!this.compass || !this.app.compass_json) return;
    const now = performance.now();
    if (now - this.lastCompassPoll < 250) return;
    this.lastCompassPoll = now;
    const json = this.app.compass_json();
    if (json === this.lastProgress) return;
    this.lastProgress = json;
    const p = parseProgress(json);
    const c = this.compass;
    c.dataset.state = p.state;
    c.dataset.level = p.level;
    c.dataset.home = String(p.animals.filter((a) => a.home).length);
    c.dataset.missing = String(missingAnimals(p).length);
    if (this.compassBadge) {
      this.compassBadge.hidden = p.badge === '';
      this.compassBadge.dataset.badge = p.badge;
      this.compassBadge.textContent = badgeIcon(p.badge);
    }
    if (this.compassStrip) {
      const v = stripView(p);
      const icons = v.ids.map((id) => {
        const e = el('span', 'pa', ANIMAL_ICONS[id] ?? '🐾');
        e.dataset.animal = id;
        e.classList.toggle('hl', id === this.hintAnimal);
        if (p.partial.includes(id)) {
          e.classList.add('half'); // one of the pair is home, the partner is still out
          e.append(el('span', 'half-tag', '½'));
        }
        return e;
      });
      if (v.more > 0) icons.push(el('span', 'pa more', `+${v.more}`));
      this.compassStrip.replaceChildren(...icons);
    }
  }

  /** Put-down button (GAME-FEED §8): drop the item in the hands; a gentle shake if not. */
  putDown(): void {
    if (!this.app.put_down) return;
    if (!this.app.put_down()) this.shakeDrop();
    this.pollEvents();
  }

  private shakeDrop(): void {
    if (!this.dropBtn) return;
    this.dropBtn.classList.remove('shake');
    void this.dropBtn.offsetWidth; // restart the animation
    this.dropBtn.classList.add('shake');
  }

  /** Readable icons above the lying foods near the player (GAME-FEED §10). */
  private updateLying(): void {
    if (!this.lyingIcons || !this.app.lying_icons_json) return;
    const json = this.app.lying_icons_json();
    if (json === this.lastLying) return;
    this.lastLying = json;
    const icons = parseLyingIcons(json);
    this.lyingIcons.replaceChildren(
      ...icons.map((i) => {
        const e = el('span', 'lying-icon', FOOD_ICONS[i.food] ?? '📦');
        e.dataset.food = i.food;
        e.style.transform = `translate(${i.x.toFixed(0)}px, ${i.y.toFixed(0)}px) translate(-50%, -100%)`;
        return e;
      }),
    );
  }

  /** Per-frame update: button visibility, HUD, events, auto-close. */
  update(): void {
    const key = this.app.target_key();
    if (key !== this.lastTarget) {
      this.lastTarget = key;
      const kind = this.app.target_kind();
      const icon = targetIcon(kind, key);
      this.act.textContent = icon;
      this.act.hidden = !(this.touch && kind);
      this.act.dataset.kind = kind;
      this.hint.hidden = this.touch || !kind;
      this.hint.dataset.kind = kind;
      (this.hint.querySelector('.icon') as HTMLElement).textContent = icon;
      // reading panels open/close by themselves: decided in Rust (panel_open/panel_close)
    }
    this.updateView();
    this.updateNight();
    if (this.dropBtn) {
      const can = this.app.can_put_down?.() ?? false;
      if (this.dropBtn.hidden === can) this.dropBtn.hidden = !can;
    }
    this.updateLying();
    this.updateHint();
    this.updatePulse();
    this.updateProgress();
    const carry = `${this.app.carry_food()}|${this.app.carry_bowl?.() ?? ''}|${this.app.basket_json?.() ?? ''}`;
    if (carry !== this.lastCarry) {
      this.lastCarry = carry;
      this.renderCarry();
    }
    this.pollEvents();
  }

  /**
   * View buttons follow the game's view (also switched with `V`), and the chosen view is
   * stored with the settings (GAME-CAMERA-VIEWS 9).
   */
  private updateView(): void {
    // the button shows the CURRENT view (look-around included); only the saved one is stored
    const cur = this.app.view_mode?.() ?? 'zoo';
    if (this.viewBtn && this.viewBtn.dataset.view !== cur) {
      this.viewBtn.dataset.view = cur;
      this.viewBtn.textContent = VIEW_ICONS[cur] ?? VIEW_ICONS.zoo;
      this.viewBtn.setAttribute('aria-pressed', String(cur !== 'zoo'));
      this.viewBtn.setAttribute('aria-label', this.app.t(`ui-view-cycle-${cur}`));
    }
    const view = this.app.saved_view_mode?.() ?? 'zoo';
    if (view === this.lastView) return;
    const first = this.lastView === '';
    this.lastView = view;
    document.body.dataset.view = view;
    if (!first) {
      saveSettings(this.store, {
        language: this.app.language(),
        readingLevel: this.app.reading_level(),
        view,
        sound: this.sound,
      });
    }
  }

  /**
   * Night overlays (GAME-NIGHT rule 3): the 🛏/🌙 choice icons while it is night, the dream
   * fade while sleeping.
   */
  private updateNight(): void {
    const phase = this.app.daytime?.() ?? 'day';
    if (this.dream) {
      const fade = this.app.sleep_fade?.() ?? 0;
      const o = fade.toFixed(3);
      if (this.dream.style.opacity !== o) this.dream.style.opacity = o;
    }
    if (phase === this.lastDaytime) return;
    this.lastDaytime = phase;
    document.body.dataset.daytime = phase;
    if (this.nightChoices) {
      this.nightChoices.hidden = phase !== 'night';
      // spoken / screen-reader labels of the two night choices (Fluent)
      const level = this.app.reading_level();
      document.getElementById('choice-bed')?.setAttribute('aria-label', this.app.t(`night-bed-${level}`));
      document.getElementById('choice-moon')?.setAttribute('aria-label', this.app.t(`night-moon-door-${level}`));
    }
  }

  /** A gentle text cut-in (dusk, morning) for a few seconds. */
  private showBanner(text: string, key: string): void {
    if (!this.nightBanner) return;
    this.nightBanner.textContent = text;
    this.nightBanner.dataset.key = key;
    this.nightBanner.hidden = false;
    window.clearTimeout(this.bannerTimer);
    this.bannerTimer = window.setTimeout(() => {
      if (this.nightBanner) this.nightBanner.hidden = true;
    }, 6000);
  }

  /**
   * HUD: the carried food, the fish bowl carried with both hands (RESC-020) and the treat
   * basket with its counts (GAME-GARDEN §4: icons + numbers, no reading needed).
   */
  private renderCarry(): void {
    const carry = this.app.carry_food();
    const bowl = this.app.carry_bowl?.() ?? '';
    const basket = parseBasket(this.app.basket_json?.());
    const treats = basket.carrot + basket.potato + basket.apple + basket.orange;
    this.hud.hidden = !carry && !bowl && treats === 0;
    this.hud.dataset.food = carry;
    this.hud.dataset.bowl = bowl;
    this.hud.replaceChildren();
    if (bowl) {
      const b = el('span', 'bowl', BOWL_ICONS[bowl] ?? '🫙');
      b.id = 'hud-bowl';
      b.dataset.bowl = bowl;
      this.hud.append(b);
    }
    if (carry) {
      this.hud.append(el('span', 'icon', FOOD_ICONS[carry] ?? '📦'), el('span', 'word', this.app.carry_text()));
    }
    if (treats > 0) {
      const b = el('span', 'basket');
      b.id = 'hud-basket';
      b.append(el('span', 'icon', '🧺'));
      for (const t of TREATS) {
        if (basket[t] > 0) b.append(el('span', `treat ${t}`, `${FOOD_ICONS[t]}${basket[t]}`));
      }
      b.dataset.carrot = String(basket.carrot);
      b.dataset.potato = String(basket.potato);
      b.dataset.apple = String(basket.apple);
      b.dataset.orange = String(basket.orange);
      this.hud.append(b);
    }
    this.hud.setAttribute('aria-label', `${this.app.t('ui-carrying')} ${this.app.carry_text()}`);
  }

  /** Optional consumer of the game events (opt-in analytics, PLAT-030): type + ids only. */
  onGameEvent: ((e: GameEventMsg) => void) | null = null;
  private settingsRelabel: (() => void) | null = null;
  /** The big overview map (GAME-MAP), opened from the settings menu or with `M`. */
  readonly map: OverviewMap;
  readonly telescope: TelescopeView;

  /** Adds a row to the settings menu (analytics consent, PLAT-031); `relabel` follows language changes. */
  addSettingsRow(row: HTMLElement, relabel: () => void): void {
    this.settings.append(row);
    this.settingsRelabel = relabel;
    relabel();
  }

  private pollEvents(): void {
    const events = JSON.parse(this.app.poll_events()) as GameEventMsg[];
    for (const e of events) {
      this.onGameEvent?.(e);
      if (e.type === 'say' && e.text) this.say(e.text, e.key ?? '');
      else if (e.type === 'mission_complete' && e.text) this.celebrateMission(e.text, e.key ?? '');
      else if (e.type === 'panel_open' && e.panel) this.openPanel(e.panel);
      else if (e.type === 'panel_close' && e.key === this.panelKey) this.hidePanel();
      else if (['dusk', 'morning', 'moon_door', 'level_complete'].includes(e.type) && e.text)
        this.showBanner(e.text, e.key ?? '');
      else if (e.type === 'sleep') this.hidePanel();
      else if (e.type === 'put_down_refused') this.shakeDrop();
    }
  }

  openPanel(data: PanelData): void {
    this.panelKey = data.key ?? null;
    this.panelFood = data.kind === 'food_box' ? (data.food ?? null) : null;
    const body = el('div', 'panel-body');
    const close = el('button', 'panel-close', '✖');
    close.id = 'panel-close';
    close.setAttribute('aria-label', this.app.t('ui-close'));
    close.addEventListener('click', () => this.closePanel());
    const text = el('p', 'panel-text', data.text ?? '');
    text.id = 'panel-text';
    const kids: HTMLElement[] = [];
    if (data.kind === 'info_board') {
      // riddle → food word first (always visible, never scrolls), then "more about" + facts
      // in their own scrolling part (GAME-ANIMALS "Info board" 4, QA F2, Q-070)
      const main = el('div', 'panel-main');
      main.id = 'panel-main';
      main.append(close);
      if (typeof data.picture === 'string') {
        const pic = el('div', 'panel-picture', PLACE_ICONS[data.picture] ?? '❓');
        pic.id = 'panel-picture';
        pic.dataset.place = data.picture;
        main.append(pic);
      }
      main.append(text);
      const food = el('p', 'panel-food');
      food.id = 'panel-food';
      if (data.picture) food.append(el('span', 'icon', FOOD_ICONS[data.food ?? ''] ?? ''));
      food.append(el('span', 'word', data.food_text ?? ''));
      food.dataset.food = data.food ?? '';
      main.append(food);
      if (data.hint) {
        // the goldfish needs a bowl with water (GAME-RESCUE "goldfish bowl" 1)
        const hint = el('p', 'panel-hint', data.hint);
        hint.id = 'panel-hint';
        main.append(hint);
      }
      kids.push(main);
      const more = el('div', 'panel-more');
      more.id = 'panel-more';
      const title = el('h2', 'panel-title');
      title.id = 'panel-title';
      title.append(el('span', 'icon', ANIMAL_ICONS[data.animal ?? ''] ?? ''), el('span', 'word', data.more ?? data.title ?? ''));
      more.append(title);
      if (data.facts) {
        const facts = el('p', 'panel-facts', data.facts);
        facts.id = 'panel-facts';
        more.append(facts);
      }
      kids.push(more);
    } else if (data.kind === 'welcome_board') {
      // RESC-028: pictures, game description, goal of this level, four steps, start hint
      const pic = el('div', 'panel-picture', WELCOME_PICTURES);
      pic.id = 'panel-picture';
      const title = el('h2', 'panel-title', data.title ?? '');
      title.id = 'panel-title';
      const goal = el('p', 'welcome-goal');
      goal.id = 'welcome-goal';
      goal.dataset.level = data.level ?? '';
      goal.append(el('span', 'icons', (data.animals ?? []).map((a) => ANIMAL_ICONS[a] ?? '🐾').join('')), el('span', 'word', data.goal ?? ''));
      const steps = el('ol', 'welcome-steps');
      steps.id = 'welcome-steps';
      (data.steps ?? []).forEach((t, i) => {
        const li = el('li', 'welcome-step');
        li.append(el('span', 'icon', WELCOME_STEP_ICONS[i] ?? ''), el('span', 'word', t));
        steps.append(li);
      });
      const kidsW: HTMLElement[] = [close, pic, title, text, goal, steps];
      if (data.level_text) {
        const note = el('p', 'welcome-level', data.level_text);
        note.id = 'welcome-level';
        kidsW.push(note);
      }
      const start = el('p', 'welcome-start', data.start ?? '');
      start.id = 'welcome-start';
      kidsW.push(start);
      kids.push(...kidsW);
    } else if (data.kind === 'garden_sign') {
      // garden sign (GARD-009): the vegetable picture, its word, a sentence from klasse1 on
      const pic = el('div', 'panel-picture', FOOD_ICONS[data.food ?? ''] ?? '🌱');
      pic.id = 'panel-picture';
      const title = el('h2', 'panel-title', data.title ?? '');
      title.id = 'panel-title';
      kids.push(close, pic, title);
      if (data.text) kids.push(text);
    } else {
      if (data.pictogram) {
        // food box: vector pictogram, bigger on kiga than on klasse3 (FEED-033)
        const pic = el('div', 'panel-picture');
        pic.id = 'panel-picture';
        pic.dataset.pictogram = data.pictogram;
        pic.dataset.scale = String(data.pictogram_scale ?? 0.5);
        pic.append(pictogramCanvas(data.pictogram, data.pictogram_scale ?? 0.5));
        kids.push(pic);
      }
      kids.push(close, text);
      const take = el('button', 'take', '✋');
      take.id = 'take';
      take.setAttribute('aria-label', data.take ?? this.app.t('ui-take'));
      take.addEventListener('click', () => this.take());
      kids.push(take);
    }
    body.append(...kids);
    body.dataset.kind = data.kind;
    this.panel.replaceChildren(body);
    this.panel.dataset.kind = data.kind;
    this.panel.hidden = false;
    if (data.kind === 'welcome_board') {
      dragScroll(body, body);
      // the ▼ cue shows until the end of the long panel is reached (or when it all fits)
      const markEnd = (): void => {
        body.dataset.end = body.scrollTop + body.clientHeight >= body.scrollHeight - 8 ? '1' : '0';
      };
      body.addEventListener('scroll', markEnd);
      requestAnimationFrame(markEnd);
    }
    if (data.kind === 'info_board') {
      fitReadingText(body);
      // long facts scroll by dragging anywhere on the panel box (PLAY-032/033)
      const more = body.querySelector<HTMLElement>('.panel-more');
      if (more) dragScroll(body, more);
    }
  }

  /** Closed by hand (✖, Esc): stays closed until the player leaves and returns (PLAY-026). */
  closePanel(): void {
    if (this.panelKey) this.app.close_panel();
    this.hidePanel();
  }

  /** Hides the panel without telling the game (it closed it itself). */
  private hidePanel(): void {
    this.panel.hidden = true;
    this.panel.replaceChildren();
    this.panelKey = null;
    this.panelFood = null;
  }

  get panelOpen(): boolean {
    return !this.panel.hidden;
  }

  escape(): void {
    if (this.telescope.isOpen) this.telescope.close();
    else if (this.map.isOpen) this.map.close();
    else if (!this.settings.hidden) this.settings.hidden = true;
    else this.closePanel();
  }

  private take(): void {
    if (!this.panelFood) return;
    if (this.app.take_food(this.panelFood)) this.closePanel();
    this.pollEvents();
  }

  /** Set by main.ts: opens the ad carousel, true if it was shown (needs a verified campaign). */
  onAllDone: (() => boolean) | null = null;

  private allDoneTap(): boolean {
    if (!this.onAllDone || parseProgress(this.app.compass_json?.() ?? this.lastProgress).next !== 'all_done') return false;
    return this.onAllDone();
  }

  /** Tapping the compass explains it in a bubble next to it for ~4 s (NIGHT-023). */
  private sayProgressInfo(): void {
    const p = parseProgress(this.app.compass_json?.() ?? this.lastProgress);
    if (p.state === 'hidden' && p.next === '') return;
    const key = progressInfoKey(p, this.app.reading_level());
    this.say(this.app.t(key), key, 4000, 'compass');
  }

  private say(text: string, key: string, ms = 2500, near = ''): void {
    this.bubble.textContent = text;
    this.bubble.dataset.key = key;
    if (near) this.bubble.dataset.near = near;
    else delete this.bubble.dataset.near;
    this.bubble.hidden = false;
    window.clearTimeout(this.bubbleTimer);
    this.bubbleTimer = window.setTimeout(() => {
      this.bubble.hidden = true;
    }, ms);
  }

  private celebrateMission(text: string, key: string): void {
    this.celebrate.replaceChildren();
    const msg = el('div', 'celebrate-text', text);
    msg.id = 'celebrate-text';
    msg.dataset.key = key;
    const colors = ['#f2c14e', '#e8604c', '#5fb3e8', '#7cc46a', '#c38de0'];
    for (let i = 0; i < 60; i++) {
      const c = el('i', 'confetti');
      c.style.left = `${(i * 37) % 100}%`;
      c.style.background = colors[i % colors.length];
      c.style.animationDelay = `${(i % 12) * 0.12}s`;
      c.style.transform = `rotate(${(i * 47) % 360}deg)`;
      this.celebrate.append(c);
    }
    this.celebrate.append(msg);
    this.celebrate.hidden = false;
    window.setTimeout(() => {
      this.celebrate.hidden = true;
    }, 7000);
  }

  // -------------------------------------------------------------- settings

  private toggleSettings(): void {
    this.settings.hidden = !this.settings.hidden;
    this.markSettings();
  }

  private buildSettings(): void {
    const langRow = el('div', 'row');
    langRow.id = 'settings-lang';
    for (const l of LANGUAGES) {
      const b = el('button', 'choice', LANG_ICONS[l]);
      b.dataset.lang = l;
      b.addEventListener('click', () => this.change({ language: l }));
      langRow.append(b);
    }
    const levelRow = el('div', 'row');
    levelRow.id = 'settings-level';
    for (const r of READING_LEVELS) {
      const b = el('button', 'choice', LEVEL_ICONS[r]);
      b.dataset.level = r;
      b.addEventListener('click', () => this.change({ readingLevel: r }));
      levelRow.append(b);
    }
    // New game (GAME-SAVE §6): icon button, then a big yes/no icon pair — no reading needed.
    const gameRow = el('div', 'row');
    gameRow.id = 'settings-game';
    const newGame = el('button', 'choice', '🔄');
    newGame.id = 'new-game';
    const confirm = el('div', 'row confirm');
    confirm.id = 'new-game-confirm';
    confirm.hidden = true;
    const yes = el('button', 'choice yes', '✔');
    yes.id = 'new-game-yes';
    const no = el('button', 'choice no', '✖');
    no.id = 'new-game-no';
    confirm.append(yes, no);
    newGame.addEventListener('click', () => {
      confirm.hidden = !confirm.hidden;
    });
    no.addEventListener('click', () => {
      confirm.hidden = true;
    });
    yes.addEventListener('click', () => {
      confirm.hidden = true;
      this.settings.hidden = true;
      this.onNewGame();
    });
    gameRow.append(newGame, confirm);
    // Sound on / off (ASND-009): icon button 🔊 / 🔇, saved with the other settings.
    const soundRow = el('div', 'row');
    soundRow.id = 'settings-sound';
    const soundBtn = el('button', 'choice', '');
    soundBtn.id = 'sound-toggle';
    soundBtn.addEventListener('click', () => this.setSound(!this.sound));
    soundRow.append(soundBtn);
    // Explain the game again (RESC-029 replay): ❓ shows the 3 intro pages
    const introBtn = el('button', 'choice', '❓');
    introBtn.id = 'intro-replay';
    introBtn.addEventListener('click', () => {
      this.settings.hidden = true;
      this.showIntro(true);
    });
    soundRow.append(introBtn);
    // Full screen (PLAT-019/020): only where the Fullscreen API exists and the app is not installed.
    const fsEnv = browserEnv();
    const extra: HTMLElement[] = [];
    if (showFullscreenButton(fsEnv)) {
      const fsBtn = el('button', 'choice', '');
      fsBtn.id = 'fullscreen-toggle';
      fsBtn.addEventListener('click', () => {
        this.settings.hidden = true;
        void toggleFullscreen(fsEnv);
      });
      const mark = () => {
        const on = isFullscreen(fsEnv);
        fsBtn.replaceChildren(fullscreenIcon(on));
        fsBtn.classList.toggle('on', on);
        fsBtn.setAttribute('aria-pressed', String(on));
      };
      document.addEventListener('fullscreenchange', mark);
      document.addEventListener('webkitfullscreenchange', mark);
      mark();
      soundRow.append(fsBtn);
    }
    // iPhone Safari cannot install by itself: one hint line, settings menu only (no popup).
    if (showIosInstallHint(fsEnv)) {
      const hint = el('p', 'install-hint', '');
      hint.id = 'install-hint';
      extra.push(hint);
    }
    // Overview map (GAME-MAP): 🗺️ opens the big map of the whole zoo.
    const mapRow = el('div', 'row');
    mapRow.id = 'settings-map';
    const mapBtn = el('button', 'choice', '🗺️');
    mapBtn.id = 'map-btn';
    mapBtn.addEventListener('click', () => this.map.open());
    mapRow.append(mapBtn);
    this.settings.replaceChildren(mapRow, langRow, levelRow, soundRow, gameRow, ...extra);
    this.markSound();
  }

  private change(part: Partial<Settings>): void {
    if (part.language) this.app.set_language(part.language);
    if (part.readingLevel) this.app.set_reading_level(part.readingLevel);
    saveSettings(this.store, {
      language: this.app.language(),
      readingLevel: this.app.reading_level(),
      view: this.app.saved_view_mode?.() ?? 'zoo',
      sound: this.sound,
    });
    // All visible texts follow at once (L10N-004): labels, HUD, an open panel.
    const panel = this.panelKey ? this.app.panel_json() : '';
    if (panel) this.openPanel(JSON.parse(panel) as PanelData);
    else this.hidePanel();
    this.applyLabels();
    this.renderCarry();
    this.markSettings();
  }

  /** Sound switch: saved, the host audio is told (ASND-009). */
  private setSound(on: boolean): void {
    this.sound = on;
    saveSettings(this.store, {
      language: this.app.language(),
      readingLevel: this.app.reading_level(),
      view: this.app.saved_view_mode?.() ?? 'zoo',
      sound: on,
    });
    window.dispatchEvent(new CustomEvent(SOUND_EVENT, { detail: { on } }));
    this.markSound();
  }

  private markSound(): void {
    const b = this.settings.querySelector<HTMLButtonElement>('#sound-toggle');
    if (!b) return;
    b.textContent = this.sound ? '🔊' : '🔇';
    b.classList.toggle('on', this.sound);
    b.setAttribute('aria-pressed', String(this.sound));
  }

  private markSettings(): void {
    for (const b of this.settings.querySelectorAll<HTMLButtonElement>('button')) {
      if (b.id === 'sound-toggle' || b.id === 'intro-replay' || b.id === 'map-btn' || b.id === 'fullscreen-toggle' || b.id === 'analytics-toggle') continue;
      const on = b.dataset.lang === this.app.language() || b.dataset.level === this.app.reading_level();
      b.classList.toggle('on', on);
      b.setAttribute('aria-pressed', String(on));
    }
  }

  private applyLabels(): void {
    document.documentElement.lang = this.app.language();
    this.gear.setAttribute('aria-label', this.app.t('ui-settings'));
    this.viewBtn?.setAttribute('aria-label', this.app.t(`ui-view-cycle-${this.viewBtn.dataset.view ?? 'zoo'}`));
    this.act.setAttribute('aria-label', this.app.t('ui-interact'));
    this.hint.setAttribute('aria-label', this.app.t('ui-interact'));
    this.dropBtn?.setAttribute('aria-label', this.app.t('ui-put-down'));
    this.compass?.setAttribute('aria-label', this.app.t('ui-hint'));
    this.settings.querySelector('#settings-lang')?.setAttribute('aria-label', this.app.t('ui-language'));
    this.settings.querySelector('#settings-level')?.setAttribute('aria-label', this.app.t('ui-reading-level'));
    this.settings.querySelector('#sound-toggle')?.setAttribute('aria-label', this.app.t('ui-sound'));
    this.settings.querySelector('#map-btn')?.setAttribute('aria-label', this.app.t('map-button'));
    this.map?.relabel();
    this.telescope?.relabel();
    this.settings.querySelector('#intro-replay')?.setAttribute('aria-label', this.app.t('ui-replay-intro'));
    this.settings.querySelector('#fullscreen-toggle')?.setAttribute('aria-label', this.app.t('ui-fullscreen'));
    const installHint = this.settings.querySelector('#install-hint');
    if (installHint) installHint.textContent = this.app.t('ui-install-hint-ios');
    this.settingsRelabel?.();
    this.settings.querySelector('#new-game')?.setAttribute('aria-label', this.app.t('ui-new-game'));
    this.settings.querySelector('#new-game-yes')?.setAttribute('aria-label', this.app.t('ui-yes'));
    this.settings.querySelector('#new-game-no')?.setAttribute('aria-label', this.app.t('ui-no'));
    for (const b of this.settings.querySelectorAll<HTMLButtonElement>('button')) {
      if (b.dataset.lang) b.setAttribute('aria-label', this.app.t(`ui-lang-${b.dataset.lang}`));
      if (b.dataset.level) b.setAttribute('aria-label', this.app.t(`ui-level-${b.dataset.level}`));
    }
  }
}
