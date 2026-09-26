// HTML overlays of the host shell (GAME-PLAYER §3/§4, GAME-FEED §6, CONT-L10N): interact
// button / key hint, text panel, carried-food HUD, settings menu, feedback bubble and the
// mission celebration. Every text comes from the game (Fluent via `App.t` / `App.interact`);
// icons are placeholders (emoji) until the icon art exists.

/** The subset of the WASM `App` the UI needs. */
export interface UiApp {
  t(key: string): string;
  target_kind(): string;
  target_key(): string;
  interact(): string;
  take_food(food: string): boolean;
  carry_food(): string;
  carry_text(): string;
  poll_events(): string;
  set_language(id: string): boolean;
  set_reading_level(id: string): boolean;
  language(): string;
  reading_level(): string;
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
};

/** Placeholder pictures of hiding places (kiga riddle). */
export const PLACE_ICONS: Record<string, string> = {
  loc_river: '🏞️',
  loc_pond: '🪷',
  loc_cave: '🕳️',
  loc_tallest_tree: '🌳',
  loc_mud_pool: '🟤',
  loc_fountain: '⛲',
  loc_pirate_ship: '🏴‍☠️',
  loc_playground: '🛝',
  loc_sun_rocks: '🪨',
  loc_ice_cream_kiosk: '🍦',
};

/** Icon of the interact button per target kind. */
export const TARGET_ICONS: Record<string, string> = {
  info_board: '👀',
  food_box: '✋',
  animal: '🤲',
  gate: '🏠',
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
}

const KEY_LANG = 'zoo.language';
const KEY_LEVEL = 'zoo.readingLevel';

/**
 * Stored settings, falling back to the device language (CONT-L10N §5, via `defaultLanguage`)
 * and `klasse1`. Invalid stored values are ignored.
 */
export function loadSettings(store: KeyValue | null, defaultLanguage: string): Settings {
  let lang: string | null = null;
  let level: string | null = null;
  try {
    lang = store?.getItem(KEY_LANG) ?? null;
    level = store?.getItem(KEY_LEVEL) ?? null;
  } catch {
    // storage blocked (private mode): defaults
  }
  return {
    language: (LANGUAGES as readonly string[]).includes(lang ?? '') ? lang! : defaultLanguage,
    readingLevel: (READING_LEVELS as readonly string[]).includes(level ?? '') ? level! : 'klasse1',
  };
}

export function saveSettings(store: KeyValue | null, s: Settings): void {
  try {
    store?.setItem(KEY_LANG, s.language);
    store?.setItem(KEY_LEVEL, s.readingLevel);
  } catch {
    // storage blocked: settings live for this session only
  }
}

interface PanelData {
  kind: string;
  key?: string;
  text?: string;
  picture?: string | boolean | null;
  food?: string;
  food_text?: string;
  take?: string;
}

interface GameEventMsg {
  type: string;
  animal?: string;
  key?: string;
  text?: string;
  food?: string;
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

export class Ui {
  private panelKey: string | null = null;
  private panelFood: string | null = null;
  private lastCarry = '\u0000';
  private lastTarget = '\u0000';
  private bubbleTimer = 0;
  private touch = false;

  readonly act = document.getElementById('act') as HTMLButtonElement;
  readonly hint = document.getElementById('hint') as HTMLButtonElement;
  readonly panel = document.getElementById('panel') as HTMLDivElement;
  readonly hud = document.getElementById('hud-carry') as HTMLDivElement;
  readonly gear = document.getElementById('settings-btn') as HTMLButtonElement;
  readonly settings = document.getElementById('settings') as HTMLDivElement;
  readonly bubble = document.getElementById('bubble') as HTMLDivElement;
  readonly celebrate = document.getElementById('celebrate') as HTMLDivElement;

  constructor(
    private readonly app: UiApp,
    private readonly store: KeyValue | null,
  ) {
    for (const b of [this.act, this.hint]) {
      b.addEventListener('pointerdown', (e) => {
        e.preventDefault();
        e.stopPropagation();
        this.interact();
      });
    }
    this.gear.addEventListener('click', () => this.toggleSettings());
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
    if (this.panelFood && this.app.target_key() === this.panelKey) {
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
      if (this.panelKey && key !== this.panelKey) this.closePanel();
    }
    const carry = this.app.carry_food();
    if (carry !== this.lastCarry) {
      this.lastCarry = carry;
      this.renderCarry();
    }
    this.pollEvents();
  }

  private renderCarry(): void {
    const carry = this.app.carry_food();
    this.hud.hidden = !carry;
    this.hud.dataset.food = carry;
    this.hud.replaceChildren();
    if (!carry) return;
    this.hud.append(el('span', 'icon', FOOD_ICONS[carry] ?? '📦'), el('span', 'word', this.app.carry_text()));
    this.hud.setAttribute('aria-label', `${this.app.t('ui-carrying')} ${this.app.carry_text()}`);
  }

  private pollEvents(): void {
    const events = JSON.parse(this.app.poll_events()) as GameEventMsg[];
    for (const e of events) {
      if (e.type === 'say' && e.text) this.say(e.text, e.key ?? '');
      else if (e.type === 'mission_complete' && e.text) this.celebrateMission(e.text, e.key ?? '');
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
    const kids: HTMLElement[] = [close];
    if (data.kind === 'info_board') {
      if (typeof data.picture === 'string') {
        const pic = el('div', 'panel-picture', PLACE_ICONS[data.picture] ?? '❓');
        pic.id = 'panel-picture';
        pic.dataset.place = data.picture;
        kids.push(pic);
      }
      kids.push(text);
      const food = el('p', 'panel-food');
      food.id = 'panel-food';
      if (data.picture) food.append(el('span', 'icon', FOOD_ICONS[data.food ?? ''] ?? ''));
      food.append(el('span', 'word', data.food_text ?? ''));
      food.dataset.food = data.food ?? '';
      kids.push(food);
    } else {
      if (data.picture === true) {
        const pic = el('div', 'panel-picture', FOOD_ICONS[data.food ?? ''] ?? '❓');
        pic.id = 'panel-picture';
        kids.push(pic);
      }
      kids.push(text);
      const take = el('button', 'take', '✋');
      take.id = 'take';
      take.setAttribute('aria-label', data.take ?? this.app.t('ui-take'));
      take.addEventListener('click', () => this.take());
      kids.push(take);
    }
    body.append(...kids);
    this.panel.replaceChildren(body);
    this.panel.dataset.kind = data.kind;
    this.panel.hidden = false;
  }

  closePanel(): void {
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
    this.settings.replaceChildren(langRow, levelRow);
  }

  private change(part: Partial<Settings>): void {
    if (part.language) this.app.set_language(part.language);
    if (part.readingLevel) this.app.set_reading_level(part.readingLevel);
    saveSettings(this.store, { language: this.app.language(), readingLevel: this.app.reading_level() });
    // All visible texts follow at once (L10N-004): labels, HUD; open panels close.
    this.closePanel();
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
    this.act.setAttribute('aria-label', this.app.t('ui-interact'));
    this.hint.setAttribute('aria-label', this.app.t('ui-interact'));
    this.settings.querySelector('#settings-lang')?.setAttribute('aria-label', this.app.t('ui-language'));
    this.settings.querySelector('#settings-level')?.setAttribute('aria-label', this.app.t('ui-reading-level'));
    for (const b of this.settings.querySelectorAll<HTMLButtonElement>('button')) {
      if (b.dataset.lang) b.setAttribute('aria-label', this.app.t(`ui-lang-${b.dataset.lang}`));
      if (b.dataset.level) b.setAttribute('aria-label', this.app.t(`ui-level-${b.dataset.level}`));
    }
  }
}
