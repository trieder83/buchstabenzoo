// Overview map of the whole zoo (GAME-MAP, MAP-005…021, Q-363). zoo-core builds the data
// (`overview_json`: level parts with progress, shapes, enclosures, the player, the next mark);
// this file only lays it out and draws it: shapes on a 2D canvas, icons and labels as DOM
// elements on top (testable, Fluent text). Never shows escaped animals or hiding places.

export type Rect4 = [number, number, number, number]; // x, z, w, d (level metres, z = north)

export interface OverviewData {
  bounds: Rect4;
  parts: { id: string; rect: Rect4; state: 'locked' | 'open' | 'solved'; night: boolean; total: number; home: number }[];
  enclosures: { id: string; animal: string; rect: Rect4; part: number; home: boolean }[];
  shapes: { class: string; kind: string; rect: Rect4; part: number }[];
  player: { x: number; z: number; fx: number; fz: number; part: number | null };
  next: { part: number; kind: string; x: number | null; z: number | null } | null;
}

export interface OverviewApp {
  t(key: string): string;
  overview_json?(): string;
  set_map_open?(open: boolean): void;
}

/** Scale (px per metre) and offset that fit the `bounds` into a `w` x `h` box, aspect kept. */
export function fitMap(bounds: Rect4, w: number, h: number): { scale: number; ox: number; oz: number; width: number; height: number } {
  const [bx, bz, bw, bd] = bounds;
  const scale = Math.max(0.1, Math.min(w / bw, h / bd));
  const width = bw * scale;
  const height = bd * scale;
  return { scale, ox: bx, oz: bz + bd, width, height }; // z up: screen y = (oz - z) * scale
}

/** Level point -> pixel in the map box. */
export function toPx(f: ReturnType<typeof fitMap>, x: number, z: number): [number, number] {
  return [(x - f.ox) * f.scale, (f.oz - z) * f.scale];
}

export function parseOverview(json: string | undefined): OverviewData | null {
  if (!json) return null;
  try {
    const d = JSON.parse(json) as OverviewData;
    return Array.isArray(d.parts) ? d : null;
  } catch {
    return null;
  }
}

const COLORS: Record<string, string> = {
  path: '#ecd9a0',
  water: '#6cb8e8',
  building: '#c98763',
  barrier: '#8a5a34',
  wall: '#5a3a22',
  trees: '#5aa04a',
};

const GROUND = { open: '#c3e59a', solved: '#a8dc8f', locked: '#d4d4d4', night: '#7f93bd', nightLocked: '#a9adb8' };

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

export class OverviewMap {
  private readonly root: HTMLElement;
  private readonly stage: HTMLElement;
  private readonly canvas: HTMLCanvasElement;
  private readonly title: HTMLElement;
  private readonly closeBtn: HTMLButtonElement;
  private data: OverviewData | null = null;
  private opened = false;

  constructor(
    private readonly app: OverviewApp,
    private readonly animalIcons: Record<string, string>,
    private readonly nextIcons: Record<string, string>,
    private readonly onOpen: () => void = () => {},
    private readonly onClose: () => void = () => {},
  ) {
    this.root = el('div', 'overview');
    this.root.id = 'overview-map';
    this.root.hidden = true;
    this.title = el('h2', 'map-title');
    this.closeBtn = el('button', 'map-close round', '✖');
    this.closeBtn.id = 'map-close';
    this.closeBtn.type = 'button';
    this.closeBtn.addEventListener('click', () => this.close());
    this.stage = el('div', 'map-stage');
    this.canvas = el('canvas', 'map-canvas');
    this.stage.append(this.canvas);
    this.root.append(this.title, this.closeBtn, this.stage);
    document.body.append(this.root);
    window.addEventListener('resize', () => this.opened && this.render());
    window.addEventListener('keydown', (e) => {
      if (e.code === 'KeyM' && !e.repeat && !e.ctrlKey && !e.metaKey) {
        e.preventDefault();
        this.toggle();
      }
    });
  }

  get isOpen(): boolean {
    return this.opened;
  }

  toggle(): void {
    if (this.opened) this.close();
    else this.open();
  }

  open(): void {
    if (this.opened || !this.app.overview_json) return;
    this.data = parseOverview(this.app.overview_json());
    if (!this.data) return;
    this.opened = true;
    this.app.set_map_open?.(true);
    this.root.hidden = false;
    this.onOpen();
    this.render();
    this.closeBtn.focus({ preventScroll: true });
  }

  close(): void {
    if (!this.opened) return;
    this.opened = false;
    this.root.hidden = true;
    this.app.set_map_open?.(false);
    this.onClose();
  }

  /** Language change: texts follow at once. */
  relabel(): void {
    if (this.opened) this.render();
  }

  private render(): void {
    const d = this.data;
    if (!d) return;
    this.title.textContent = this.app.t('map-title');
    this.closeBtn.setAttribute('aria-label', this.app.t('map-close'));
    const rw = this.root.clientWidth || window.innerWidth;
    const rh = this.root.clientHeight || window.innerHeight;
    const top = 76; // title row / close button
    const pad = 8;
    const f = fitMap(d.bounds, rw - 2 * pad, rh - top - pad);
    const st = this.stage.style;
    st.width = `${f.width}px`;
    st.height = `${f.height}px`;
    st.left = `${(rw - f.width) / 2}px`;
    st.top = `${top + (rh - top - pad - f.height) / 2}px`;
    const dpr = window.devicePixelRatio || 1;
    this.canvas.width = Math.ceil(f.width * dpr);
    this.canvas.height = Math.ceil(f.height * dpr);
    this.canvas.style.width = `${f.width}px`;
    this.canvas.style.height = `${f.height}px`;
    const ctx = this.canvas.getContext('2d');
    if (ctx) this.draw(ctx, d, f, dpr);
    // DOM overlay: levels, enclosure icons, player, next mark
    for (const old of [...this.stage.querySelectorAll('.map-level, .map-enclosure, .map-player, .map-next')]) old.remove();
    for (const p of d.parts) this.addLevel(p, f);
    for (const e of d.enclosures) this.addEnclosure(d, e, f);
    this.addPlayer(d, f);
    this.addNext(d, f);
  }

  private draw(ctx: CanvasRenderingContext2D, d: OverviewData, f: ReturnType<typeof fitMap>, dpr: number): void {
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, f.width, f.height);
    const rect = (r: Rect4): [number, number, number, number] => {
      const [x, y] = toPx(f, r[0], r[1] + r[3]);
      return [x, y, r[2] * f.scale, r[3] * f.scale];
    };
    d.parts.forEach((p) => {
      const [x, y, w, h] = rect(p.rect);
      ctx.fillStyle = p.night ? (p.state === 'locked' ? GROUND.nightLocked : GROUND.night) : GROUND[p.state];
      ctx.fillRect(x, y, w, h);
    });
    ctx.lineJoin = 'round';
    for (const s of d.shapes) {
      const locked = d.parts[s.part]?.state === 'locked';
      const [x, y, w, h] = rect(s.rect);
      ctx.globalAlpha = locked ? 0.45 : 1;
      ctx.fillStyle = locked ? '#b9b9b9' : (COLORS[s.class] ?? '#999');
      ctx.fillRect(x, y, w, h);
      if (s.class === 'building' || s.class === 'barrier') {
        ctx.lineWidth = 1.5;
        ctx.strokeStyle = '#3b2314';
        ctx.strokeRect(x, y, w, h);
      }
    }
    ctx.globalAlpha = 1;
    for (const e of d.enclosures) {
      const locked = d.parts[e.part]?.state === 'locked';
      const [x, y, w, h] = rect(e.rect);
      ctx.fillStyle = locked ? '#dcdcdc' : e.home ? '#9fdc86' : '#e8f4cf';
      ctx.fillRect(x, y, w, h);
      ctx.lineWidth = 2;
      ctx.strokeStyle = locked ? '#8a8a8a' : '#3b2314';
      ctx.strokeRect(x, y, w, h);
    }
    // level borders
    ctx.lineWidth = 2.5;
    ctx.strokeStyle = '#3b2314';
    d.parts.forEach((p) => {
      const [x, y, w, h] = rect(p.rect);
      ctx.setLineDash(p.state === 'locked' ? [6, 4] : []);
      ctx.strokeRect(x, y, w, h);
    });
    ctx.setLineDash([]);
  }

  private place(node: HTMLElement, f: ReturnType<typeof fitMap>, x: number, z: number): void {
    const [px, py] = toPx(f, x, z);
    node.style.left = `${px}px`;
    node.style.top = `${py}px`;
  }

  private addLevel(p: OverviewData['parts'][number], f: ReturnType<typeof fitMap>): void {
    const locked = p.state === 'locked';
    const text = [locked ? '🔒' : '', p.night ? '🌙' : '', this.app.t(`map-level-${p.id}`)];
    if (!locked) text.push(`${p.home}/${p.total}`, p.state === 'solved' ? '✔' : '');
    const n = el('div', 'map-level', text.filter(Boolean).join(' '));
    n.dataset.level = p.id;
    n.dataset.state = p.state;
    n.dataset.home = String(p.home);
    n.dataset.total = String(p.total);
    n.dataset.night = String(p.night);
    if (locked) n.title = this.app.t('map-locked');
    this.place(n, f, p.rect[0] + p.rect[2] / 2, p.rect[1] + p.rect[3] - 1);
    this.stage.append(n);
  }

  private addEnclosure(d: OverviewData, e: OverviewData['enclosures'][number], f: ReturnType<typeof fitMap>): void {
    const locked = d.parts[e.part]?.state === 'locked';
    const size = Math.max(14, Math.min(e.rect[2], e.rect[3]) * f.scale * 0.8);
    const n = el('div', 'map-enclosure');
    n.dataset.animal = e.animal;
    n.dataset.home = String(e.home);
    n.classList.toggle('locked', locked);
    n.style.fontSize = `${Math.min(size, 40)}px`;
    n.append(el('span', 'icon', this.animalIcons[e.animal] ?? '🐾'));
    if (e.home) n.append(el('span', 'check', '✔'));
    this.place(n, f, e.rect[0] + e.rect[2] / 2, e.rect[1] + e.rect[3] / 2);
    this.stage.append(n);
  }

  private addPlayer(d: OverviewData, f: ReturnType<typeof fitMap>): void {
    const n = el('div', 'map-player');
    n.setAttribute('aria-label', this.app.t('map-you'));
    n.dataset.x = d.player.x.toFixed(1);
    n.dataset.z = d.player.z.toFixed(1);
    // the arrow points to the facing direction (level z up = screen up)
    const deg = (Math.atan2(d.player.fx, d.player.fz) * 180) / Math.PI;
    const arrow = el('span', 'arrow', '➤');
    arrow.style.transform = `translate(-50%, -50%) rotate(${deg - 90}deg) translateX(22px)`;
    n.append(el('span', 'dot', '🧒'), arrow);
    this.place(n, f, d.player.x, d.player.z);
    this.stage.append(n);
  }

  private addNext(d: OverviewData, f: ReturnType<typeof fitMap>): void {
    const nx = d.next;
    if (!nx) return;
    const part = d.parts[nx.part];
    if (!part || part.state === 'locked' && nx.x === null) return;
    const n = el('div', 'map-next');
    n.dataset.kind = nx.kind;
    n.dataset.level = part.id;
    n.dataset.point = String(nx.x !== null);
    n.setAttribute('aria-label', this.app.t('map-next'));
    n.append(el('span', 'icon', this.nextIcons[nx.kind] ?? ''), el('span', 'arrow', '▼'));
    if (nx.x !== null && nx.z !== null) this.place(n, f, nx.x, nx.z);
    else this.place(n, f, part.rect[0] + part.rect[2] / 2, part.rect[1] + part.rect[3] - 5); // level only
    this.stage.append(n);
  }
}
