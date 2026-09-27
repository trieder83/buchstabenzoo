// HTML overlays of the host shell (GAME-PLAYER §3/§4, GAME-FEED §6, CONT-L10N): interact
// button / key hint, text panel, carried-food HUD, settings menu, feedback bubble and the
// mission celebration. Every text comes from the game (Fluent via `App.t` / `App.interact`);
// icons are placeholders (emoji) until the icon art exists.

import { dragScroll } from './scroll';

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
  /** GAME-NIGHT: time of day and the dream fade of the sleep (0…1). */
  daytime?(): string;
  sleep_fade?(): number;
}

export const LANGUAGES = ['de', 'en'] as const;
export const READING_LEVELS = ['kiga', 'klasse1', 'klasse2', 'klasse3'] as const;

/** Placeholder food pictures (kiga labels, HUD). */
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
  // night zoo (GAME-NIGHT rule 6)
  beetles: '🪲',
  fruit: '🍎',
  worms: '🪱',
  nectar: '🌺',
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
  // GAME-NIGHT rule 3: the two night choices, no reading needed
  bed: '🛏️',
  moon_door: '🌙',
};

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
}

/** Views that are stored (look-around is only held, never stored). */
export const SAVED_VIEWS = ['zoo', 'first_person'] as const;

const KEY_LANG = 'zoo.language';
const KEY_LEVEL = 'zoo.readingLevel';
const KEY_VIEW = 'zoo.view';

/**
 * Stored settings, falling back to `defaultLanguage` (always `de`, CONT-L10N §5 — the stored
 * choice from the settings wins) and `klasse1`. Invalid stored values are ignored.
 */
export function loadSettings(store: KeyValue | null, defaultLanguage: string): Settings {
  let lang: string | null = null;
  let level: string | null = null;
  let view: string | null = null;
  try {
    lang = store?.getItem(KEY_LANG) ?? null;
    level = store?.getItem(KEY_LEVEL) ?? null;
    view = store?.getItem(KEY_VIEW) ?? null;
  } catch {
    // storage blocked (private mode): defaults
  }
  return {
    language: (LANGUAGES as readonly string[]).includes(lang ?? '') ? lang! : defaultLanguage,
    readingLevel: (READING_LEVELS as readonly string[]).includes(level ?? '') ? level! : 'klasse1',
    view: (SAVED_VIEWS as readonly string[]).includes(view ?? '') ? view! : 'zoo',
  };
}

export function saveSettings(store: KeyValue | null, s: Settings): void {
  try {
    store?.setItem(KEY_LANG, s.language);
    store?.setItem(KEY_LEVEL, s.readingLevel);
    if (s.view && (SAVED_VIEWS as readonly string[]).includes(s.view)) store?.setItem(KEY_VIEW, s.view);
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
  food?: string;
  food_text?: string;
  take?: string;
  /** Info board: a container is needed (the goldfish bowl hint). */
  hint?: string | null;
}

interface GameEventMsg {
  type: string;
  panel?: PanelData;
  animal?: string;
  key?: string;
  text?: string;
  food?: string;
  into_night_zoo?: boolean;
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

  readonly act = document.getElementById('act') as HTMLButtonElement;
  readonly hint = document.getElementById('hint') as HTMLButtonElement;
  readonly panel = document.getElementById('panel') as HTMLDivElement;
  readonly hud = document.getElementById('hud-carry') as HTMLDivElement;
  readonly gear = document.getElementById('settings-btn') as HTMLButtonElement;
  readonly settings = document.getElementById('settings') as HTMLDivElement;
  readonly bubble = document.getElementById('bubble') as HTMLDivElement;
  readonly celebrate = document.getElementById('celebrate') as HTMLDivElement;
  /** First-person toggle (GAME-CAMERA-VIEWS 3) and the touch eye button (look-around, 2). */
  readonly viewBtn = document.getElementById('view-btn') as HTMLButtonElement | null;
  readonly lookBtn = document.getElementById('look-btn') as HTMLButtonElement | null;
  /** GAME-NIGHT: dusk cut-in text, dream fade, the night choice icons. */
  readonly nightBanner = document.getElementById('night-banner') as HTMLDivElement | null;
  readonly dream = document.getElementById('dream') as HTMLDivElement | null;
  readonly nightChoices = document.getElementById('night-choices') as HTMLDivElement | null;

  constructor(
    private readonly app: UiApp,
    private readonly store: KeyValue | null,
    private readonly onNewGame: () => void = () => {},
  ) {
    for (const b of [this.act, this.hint]) {
      b.addEventListener('pointerdown', (e) => {
        e.preventDefault();
        e.stopPropagation();
        this.interact();
      });
    }
    this.gear.addEventListener('click', () => this.toggleSettings());
    // pointerdown, not click: a second finger (left thumb on the stick) never gets a click
    // (CAMV-019)
    this.viewBtn?.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      e.stopPropagation();
      this.app.toggle_first_person?.();
      this.update();
    });
    this.buildSettings();
    this.applyLabels();
  }

  /** Touch controls on (first touch, PLAY-014). */
  setTouch(): void {
    this.touch = true;
    document.body.classList.add('touch');
    this.lastTarget = '\u0000';
  }

  /** Interact button / key: take from an open food panel, else interact with the target. */
  interact(): void {
    if (this.panelFood && this.app.panel_key() === this.panelKey) {
      this.take();
      return;
    }
    const json = this.app.interact();
    if (!json) return;
    const data = JSON.parse(json) as PanelData;
    if (data.kind === 'info_board' || data.kind === 'food_box') this.openPanel(data);
    this.pollEvents();
  }

  /** Per-frame update: button visibility, HUD, events, auto-close. */
  update(): void {
    const key = this.app.target_key();
    if (key !== this.lastTarget) {
      this.lastTarget = key;
      const kind = this.app.target_kind();
      const icon = TARGET_ICONS[kind] ?? '';
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
    const carry = `${this.app.carry_food()}|${this.app.carry_bowl?.() ?? ''}`;
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
    const view = this.app.saved_view_mode?.() ?? 'zoo';
    if (view === this.lastView) return;
    const first = this.lastView === '';
    this.lastView = view;
    const fp = view === 'first_person';
    document.body.dataset.view = view;
    if (this.viewBtn) {
      this.viewBtn.classList.toggle('on', fp);
      this.viewBtn.setAttribute('aria-pressed', String(fp));
    }
    if (this.lookBtn) this.lookBtn.hidden = fp; // look-around only from the zoo view
    if (!first) {
      saveSettings(this.store, {
        language: this.app.language(),
        readingLevel: this.app.reading_level(),
        view,
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

  /** HUD: the carried food and — carried with both hands — the fish bowl (RESC-020). */
  private renderCarry(): void {
    const carry = this.app.carry_food();
    const bowl = this.app.carry_bowl?.() ?? '';
    this.hud.hidden = !carry && !bowl;
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
    this.hud.setAttribute('aria-label', `${this.app.t('ui-carrying')} ${this.app.carry_text()}`);
  }

  private pollEvents(): void {
    const events = JSON.parse(this.app.poll_events()) as GameEventMsg[];
    for (const e of events) {
      if (e.type === 'say' && e.text) this.say(e.text, e.key ?? '');
      else if (e.type === 'mission_complete' && e.text) this.celebrateMission(e.text, e.key ?? '');
      else if (e.type === 'panel_open' && e.panel) this.openPanel(e.panel);
      else if (e.type === 'panel_close' && e.key === this.panelKey) this.hidePanel();
      else if (['dusk', 'morning', 'moon_door', 'level_complete'].includes(e.type) && e.text)
        this.showBanner(e.text, e.key ?? '');
      else if (e.type === 'sleep') this.hidePanel();
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
    } else {
      if (data.picture === true) {
        const pic = el('div', 'panel-picture', FOOD_ICONS[data.food ?? ''] ?? '❓');
        pic.id = 'panel-picture';
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
    if (!this.settings.hidden) this.settings.hidden = true;
    else this.closePanel();
  }

  private take(): void {
    if (!this.panelFood) return;
    if (this.app.take_food(this.panelFood)) this.closePanel();
    this.pollEvents();
  }

  private say(text: string, key: string): void {
    this.bubble.textContent = text;
    this.bubble.dataset.key = key;
    this.bubble.hidden = false;
    window.clearTimeout(this.bubbleTimer);
    this.bubbleTimer = window.setTimeout(() => {
      this.bubble.hidden = true;
    }, 2500);
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
    this.settings.replaceChildren(langRow, levelRow, gameRow);
  }

  private change(part: Partial<Settings>): void {
    if (part.language) this.app.set_language(part.language);
    if (part.readingLevel) this.app.set_reading_level(part.readingLevel);
    saveSettings(this.store, {
      language: this.app.language(),
      readingLevel: this.app.reading_level(),
      view: this.app.saved_view_mode?.() ?? 'zoo',
    });
    // All visible texts follow at once (L10N-004): labels, HUD, an open panel.
    const panel = this.panelKey ? this.app.panel_json() : '';
    if (panel) this.openPanel(JSON.parse(panel) as PanelData);
    else this.hidePanel();
    this.applyLabels();
    this.renderCarry();
    this.markSettings();
  }

  private markSettings(): void {
    for (const b of this.settings.querySelectorAll<HTMLButtonElement>('button')) {
      const on = b.dataset.lang === this.app.language() || b.dataset.level === this.app.reading_level();
      b.classList.toggle('on', on);
      b.setAttribute('aria-pressed', String(on));
    }
  }

  private applyLabels(): void {
    document.documentElement.lang = this.app.language();
    this.gear.setAttribute('aria-label', this.app.t('ui-settings'));
    this.viewBtn?.setAttribute('aria-label', this.app.t('ui-first-person'));
    this.lookBtn?.setAttribute('aria-label', this.app.t('ui-look-around'));
    this.act.setAttribute('aria-label', this.app.t('ui-interact'));
    this.hint.setAttribute('aria-label', this.app.t('ui-interact'));
    this.settings.querySelector('#settings-lang')?.setAttribute('aria-label', this.app.t('ui-language'));
    this.settings.querySelector('#settings-level')?.setAttribute('aria-label', this.app.t('ui-reading-level'));
    this.settings.querySelector('#new-game')?.setAttribute('aria-label', this.app.t('ui-new-game'));
    this.settings.querySelector('#new-game-yes')?.setAttribute('aria-label', this.app.t('ui-yes'));
    this.settings.querySelector('#new-game-no')?.setAttribute('aria-label', this.app.t('ui-no'));
    for (const b of this.settings.querySelectorAll<HTMLButtonElement>('button')) {
      if (b.dataset.lang) b.setAttribute('aria-label', this.app.t(`ui-lang-${b.dataset.lang}`));
      if (b.dataset.level) b.setAttribute('aria-label', this.app.t(`ui-level-${b.dataset.level}`));
    }
  }
}
