// Key box lock panel (GAME-CART rule 15, CART-014): three big number wheels (▲ / ▼, digits
// 0-9, start 000) and a big ✔ (🔓). No reading needed: icons only, the texts are aria labels.
// ✖ / Esc closes. Never a lockout: any number of tries. The game is paused while it is open
// (`set_lock_open`). Touch and keyboard (arrows, digits, Enter).

export interface LockApp {
  t(key: string): string;
  /** A code `0..=999`: `right` | `wrong` | `far` | `none`. */
  enter_code(code: number): string;
  cart_key_json?(): string;
  set_lock_open?(open: boolean): void;
}

export interface LockOpenData {
  title?: string;
  /** 3 wrong codes in a row: the 📝 pulses. */
  help?: boolean;
}

/** The code of three wheel digits. */
export function codeOf(digits: readonly number[]): number {
  return digits[0] * 100 + digits[1] * 10 + digits[2];
}

/** A wheel one step up / down (wraps 9 -> 0 and 0 -> 9). */
export function turn(d: number, delta: 1 | -1): number {
  return (d + delta + 10) % 10;
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

export class LockPanel {
  private readonly root: HTMLElement;
  private readonly card: HTMLElement;
  private readonly title: HTMLElement;
  private readonly closeBtn: HTMLButtonElement;
  private readonly okBtn: HTMLButtonElement;
  private readonly noteHint: HTMLElement;
  private readonly digitEls: HTMLElement[] = [];
  private readonly ups: HTMLButtonElement[] = [];
  private readonly downs: HTMLButtonElement[] = [];
  private digits = [0, 0, 0];
  private selected = 0;
  private opened = false;
  private titleText = '';

  constructor(
    private readonly app: LockApp,
    private readonly onOpen: () => void = () => {},
    private readonly onClose: () => void = () => {},
    /** Called with the result of every code (`right`, `wrong`, …). */
    private readonly onResult: (r: string) => void = () => {},
  ) {
    this.root = el('div', 'lock');
    this.root.id = 'lock-panel';
    this.root.hidden = true;
    this.card = el('div', 'lock-card');
    this.title = el('div', 'lock-title');
    this.closeBtn = el('button', 'lock-close round', '✖');
    this.closeBtn.id = 'lock-close';
    this.closeBtn.type = 'button';
    this.closeBtn.addEventListener('click', () => this.close());
    const head = el('div', 'lock-head');
    head.append(this.title, this.closeBtn);
    const wheels = el('div', 'lock-wheels');
    for (let i = 0; i < 3; i++) {
      const w = el('div', 'lock-wheel');
      w.dataset.i = String(i);
      const up = el('button', 'lock-up', '▲');
      up.id = `lock-up-${i}`;
      up.type = 'button';
      const digit = el('div', 'lock-digit', '0');
      digit.id = `lock-digit-${i}`;
      const down = el('button', 'lock-down', '▼');
      down.id = `lock-down-${i}`;
      down.type = 'button';
      // pointerdown, not click: a second finger never gets a click (CAMV-019)
      up.addEventListener('pointerdown', (e) => this.tap(e, i, 1));
      down.addEventListener('pointerdown', (e) => this.tap(e, i, -1));
      up.addEventListener('click', (e) => {
        if (e.detail === 0) this.step(i, 1); // keyboard activation
      });
      down.addEventListener('click', (e) => {
        if (e.detail === 0) this.step(i, -1);
      });
      digit.addEventListener('pointerdown', () => this.select(i));
      w.append(up, digit, down);
      wheels.append(w);
      this.digitEls.push(digit);
      this.ups.push(up);
      this.downs.push(down);
    }
    this.okBtn = el('button', 'lock-ok', '✔');
    this.okBtn.id = 'lock-ok';
    this.okBtn.type = 'button';
    this.okBtn.addEventListener('click', () => this.submit());
    this.noteHint = el('div', 'lock-note', '📝');
    this.noteHint.id = 'lock-note';
    this.noteHint.hidden = true;
    const side = el('div', 'lock-side');
    side.append(this.noteHint, this.okBtn);
    const row = el('div', 'lock-row');
    row.append(wheels, side);
    this.card.append(head, row);
    this.root.append(this.card);
    document.body.append(this.root);
    // keyboard while open: stop the game's own key handling (capture phase on the window)
    window.addEventListener('keydown', (e) => this.key(e), true);
    window.addEventListener('keyup', (e) => this.keyUp(e), true);
  }

  get isOpen(): boolean {
    return this.opened;
  }

  /** The three digits (tests). */
  get code(): number {
    return codeOf(this.digits);
  }

  open(data: LockOpenData = {}): void {
    if (this.opened) return;
    this.opened = true;
    this.digits = [0, 0, 0];
    this.selected = 0;
    this.titleText = data.title ?? '';
    this.app.set_lock_open?.(true);
    this.root.hidden = false;
    this.onOpen();
    this.setHelp(Boolean(data.help));
    this.render();
    this.okBtn.focus({ preventScroll: true });
  }

  close(): void {
    if (!this.opened) return;
    this.opened = false;
    this.root.hidden = true;
    this.app.set_lock_open?.(false);
    this.onClose();
  }

  /** Language change: labels follow at once. */
  relabel(): void {
    if (this.opened) this.titleText = this.app.t('cart-lock-title');
    this.labels();
  }

  private labels(): void {
    this.title.textContent = `🔑 ${this.titleText}`;
    this.closeBtn.setAttribute('aria-label', this.app.t('ui-lock-close'));
    this.okBtn.setAttribute('aria-label', this.app.t('ui-lock-open'));
    this.noteHint.setAttribute('aria-label', this.app.t('hint-cart-note'));
    for (let i = 0; i < 3; i++) {
      this.ups[i].setAttribute('aria-label', this.app.t('ui-lock-up'));
      this.downs[i].setAttribute('aria-label', this.app.t('ui-lock-down'));
    }
  }

  private setHelp(on: boolean): void {
    this.noteHint.hidden = !on;
  }

  private render(): void {
    this.labels();
    this.digits.forEach((d, i) => {
      this.digitEls[i].textContent = String(d);
      this.digitEls[i].parentElement?.classList.toggle('selected', i === this.selected);
    });
  }

  private tap(e: PointerEvent, i: number, delta: 1 | -1): void {
    e.preventDefault();
    e.stopPropagation();
    this.step(i, delta);
  }

  private select(i: number): void {
    this.selected = i;
    this.render();
  }

  private step(i: number, delta: 1 | -1): void {
    this.selected = i;
    this.digits[i] = turn(this.digits[i], delta);
    this.render();
  }

  /** ✔: try the code. Right closes; wrong shakes and keeps the digits (never a lockout). */
  submit(): void {
    if (!this.opened) return;
    const r = this.app.enter_code(this.code);
    this.onResult(r);
    if (r === 'wrong') {
      this.card.classList.remove('shake');
      void this.card.offsetWidth; // restart the animation
      this.card.classList.add('shake');
      let help = false;
      try {
        help = Boolean((JSON.parse(this.app.cart_key_json?.() ?? '{}') as { help?: boolean }).help);
      } catch {
        // unreadable: no pulse
      }
      this.setHelp(help);
      return;
    }
    this.close();
  }

  private keyUp(e: KeyboardEvent): void {
    if (this.opened && e.code !== 'Escape') e.stopPropagation();
  }

  private key(e: KeyboardEvent): void {
    if (!this.opened) return;
    if (e.code === 'Escape') return; // closes through the shell's Esc handler
    e.stopPropagation(); // the game is paused: no movement keys behind the panel
    switch (e.code) {
      case 'ArrowUp':
        e.preventDefault();
        this.step(this.selected, 1);
        break;
      case 'ArrowDown':
        e.preventDefault();
        this.step(this.selected, -1);
        break;
      case 'ArrowLeft':
        e.preventDefault();
        this.select((this.selected + 2) % 3);
        break;
      case 'ArrowRight':
        e.preventDefault();
        this.select((this.selected + 1) % 3);
        break;
      case 'Enter':
      case 'NumpadEnter':
        e.preventDefault();
        this.submit();
        break;
      default: {
        const m = /^(?:Digit|Numpad)(\d)$/.exec(e.code);
        if (m) {
          e.preventDefault();
          this.digits[this.selected] = Number(m[1]);
          this.select(Math.min(2, this.selected + 1));
        }
      }
    }
  }
}
