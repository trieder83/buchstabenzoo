// Telescope view (GAME-TELESCOPE, TELE-007…009): at night the child looks through the toy
// telescope and sees the eight planets as simple comic discs. Tapping one fills a small info
// box (name, type, one sentence of her reading level). zoo-core owns the planet table
// (`telescope_json`: look + Fluent keys); this file only lays it out and draws it with DOM
// elements (testable, Fluent text). No state, no network, no tracking.

export interface PlanetData {
  id: string;
  kind: 'rocky' | 'gas' | 'ice';
  order: number;
  size: number;
  colors: [string, string];
  ring: boolean;
  bands: number;
  spots: number;
  name: string;
  kind_key: string;
  texts: Record<string, string>;
}

export interface TelescopeApp {
  t(key: string): string;
  reading_level?(): string;
  telescope_json?(): string;
  set_telescope_open?(open: boolean): void;
}

export const KIND_ICONS: Record<string, string> = { rocky: '🪨', gas: '☁️', ice: '🧊' };

export function parsePlanets(json: string | undefined): PlanetData[] | null {
  if (!json) return null;
  try {
    const d = JSON.parse(json) as { planets?: PlanetData[] };
    return Array.isArray(d.planets) && d.planets.length > 0 ? d.planets : null;
  } catch {
    return null;
  }
}

export interface LineLayout {
  /** Planet centres in sky pixels, Mercury first (lower left) to Neptune (upper right). */
  centres: { x: number; y: number }[];
  /** The Sun: centre at the lower-left corner, disc radius r (partly outside the sky). */
  sun: { x: number; y: number; r: number };
  /** Smallest separation of consecutive boxes along x or y (px). */
  step: number;
}

const clamp = (v: number, lo: number, hi: number): number => Math.max(lo, Math.min(hi, v));

/**
 * Planets on one straight line from near the Sun (lower left) to the upper-right corner,
 * evenly spaced. Consecutive 64 px boxes must differ by >= 64 px in x or y, so the line
 * starts as far left as needed when the sky is wide and low (landscape phone) and leaves room
 * for the Sun when the sky is tall.
 */
export function lineLayout(w: number, h: number, n = 8): LineLayout {
  const place = (m: number): LineLayout => {
    const ay = h - m;
    const by = m;
    const dyStep = (ay - by) / (n - 1);
    // a tall sky has room for a big Sun; a low one (landscape phone) keeps it small
    const r = dyStep >= 64 ? clamp(Math.min(w, h) * 0.24, 56, 150) : clamp(Math.min(w, h) * 0.105, 34, 130);
    // Mercury keeps a clear gap to the Sun's rim (user report 2026-10-04: too near the Sun)
    const ax = dyStep >= 64 ? Math.max(m + 8, r + 50) : m + 8;
    const bx = w - m;
    const centres: { x: number; y: number }[] = [];
    for (let i = 0; i < n; i++) {
      const t = i / (n - 1);
      centres.push({ x: ax + (bx - ax) * t, y: ay + (by - ay) * t });
    }
    return { centres, sun: { x: -0.45 * r, y: h + 0.45 * r, r }, step: Math.max((bx - ax) / (n - 1), dyStep) };
  };
  // margin: half a 64 px box plus a little; larger discs on big skies need more so they stay inside
  const first = place(34);
  return place(Math.max(34, 0.42 * discBase(first.step) + 8));
}

/** Base disc diameter (px) for a given box step; planets scale it by their `size`. */
export function discBase(step: number): number {
  return clamp(step * 0.9, 50, 96);
}

/** Tiny deterministic generator (stars and spots look the same on every visit). */
function lcg(seed: number): () => number {
  let s = seed >>> 0;
  return () => {
    s = (Math.imul(s, 1664525) + 1013904223) >>> 0;
    return s / 4294967296;
  };
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

// ---- Planet art: one static inline SVG per planet (disc radius 48 units, comic outline,
// flat colours + one soft shadow tone, rim light, glow halo). No per-frame JS.
const OUTLINE = '#0b0b1c';
const SHADOW = '#0a0a3a';

function wavyBand(y: number, h: number, fill: string, amp = 3): string {
  const a = amp;
  return (
    `<path d="M-52 ${y} q13 ${-a} 26 0 t26 0 t26 0 t26 0 V${y + h} q-13 ${a} -26 0 t-26 0 t-26 0 t-26 0Z" fill="${fill}"/>`
  );
}

function crater(x: number, y: number, r: number, c: string): string {
  return (
    `<circle cx="${x}" cy="${y}" r="${r}" fill="${c}"/>` +
    `<path d="M${x - r * 0.8} ${y + r * 0.2} A${r} ${r} 0 0 1 ${x + r * 0.4} ${y - r * 0.85}" fill="none" stroke="#fff" stroke-opacity=".35" stroke-width="1.6" stroke-linecap="round"/>`
  );
}

function ringSvg(kind: 'saturn' | 'uranus', half: 'back' | 'front', id: string): string {
  type R = [number, number]; // rx, stroke
  const saturn: { rings: R[]; colors: string[]; ratio: number; tilt: number } = {
    rings: [
      [92, 5],
      [80, 9],
      [66, 6],
    ],
    colors: ['#cfa95c', '#f0d890', '#b38f4a'],
    ratio: 0.26,
    tilt: -16,
  };
  const uranus: { rings: R[]; colors: string[]; ratio: number; tilt: number } = {
    rings: [[66, 3]],
    colors: ['#d9f7f9'],
    ratio: 0.2,
    tilt: -72,
  };
  const d = kind === 'saturn' ? saturn : uranus;
  const clip = half === 'front' ? ` clip-path="url(#tp-${id}-front)"` : '';
  let out = `<g transform="rotate(${d.tilt})"><g${clip}${kind === 'uranus' ? ' opacity=".8"' : ''}>`;
  d.rings.forEach(([rx, sw], i) => {
    const ry = rx * d.ratio;
    out += `<ellipse rx="${rx}" ry="${ry}" fill="none" stroke="${OUTLINE}" stroke-width="${sw + 3}"/>`;
    out += `<ellipse rx="${rx}" ry="${ry}" fill="none" stroke="${d.colors[i]}" stroke-width="${sw}"/>`;
  });
  return out + '</g></g>';
}

function details(p: PlanetData): string {
  const [c1, c2] = p.colors;
  switch (p.id) {
    case 'mercury':
      return (
        crater(-18, -14, 10, c2) + crater(15, -22, 7, c2) + crater(21, 10, 12, c2) +
        crater(-22, 18, 8, c2) + crater(0, 3, 5, c2) + crater(-2, 33, 6, c2) + crater(-38, -2, 5, c2)
      );
    case 'venus':
      return (
        '<g fill="none" stroke-linecap="round">' +
        '<path d="M-52 -26 q13 -12 26 0 t26 0 t26 0 t26 0" stroke="#fff6c8" stroke-opacity=".75" stroke-width="7"/>' +
        `<path d="M-52 -6 q13 -12 26 0 t26 0 t26 0 t26 0" stroke="${c2}" stroke-width="8"/>` +
        '<path d="M-52 16 q13 -12 26 0 t26 0 t26 0 t26 0" stroke="#fff6c8" stroke-opacity=".7" stroke-width="7"/>' +
        `<path d="M-52 34 q13 -10 26 0 t26 0 t26 0 t26 0" stroke="${c2}" stroke-width="7"/>` +
        '<path d="M-20 -40 q10 6 0 14" stroke="#fff" stroke-opacity=".6" stroke-width="3"/>' +
        '</g>'
      );
    case 'earth':
      return (
        `<path d="M-30 -22 C-14 -36 6 -26 4 -10 C2 4 -12 6 -20 -2 C-30 -8 -38 -14 -30 -22Z" fill="${c2}"/>` +
        `<path d="M14 8 C30 0 42 14 34 28 C26 40 10 36 8 24 C6 16 8 12 14 8Z" fill="${c2}"/>` +
        `<path d="M-34 20 C-24 14 -14 22 -18 32 C-24 40 -34 34 -34 20Z" fill="${c2}"/>` +
        '<path d="M-6 -30 C6 -34 22 -28 24 -18 C16 -20 6 -16 -6 -30Z" fill="#c7d98a"/>' +
        '<g fill="#fff" fill-opacity=".9">' +
        '<rect x="-44" y="-8" width="32" height="9" rx="4.5"/><rect x="-30" y="-1" width="22" height="7" rx="3.5"/>' +
        '<rect x="6" y="-18" width="30" height="8" rx="4"/><rect x="-14" y="38" width="30" height="8" rx="4"/>' +
        '<rect x="18" y="20" width="26" height="7" rx="3.5"/></g>'
      );
    case 'mars':
      return (
        `<path d="M-30 -4 C-20 -14 -6 -8 -8 4 C-10 14 -26 14 -32 6Z" fill="${c2}"/>` +
        `<path d="M8 10 C18 2 34 8 32 20 C28 30 12 30 6 22Z" fill="${c2}"/>` +
        `<path d="M-2 -22 C6 -26 14 -22 12 -14 C6 -10 -2 -14 -2 -22Z" fill="${c2}"/>` +
        '<ellipse cx="0" cy="-47" rx="22" ry="10" fill="#fff" fill-opacity=".92"/>' +
        '<ellipse cx="4" cy="48" rx="12" ry="5" fill="#fff" fill-opacity=".8"/>'
      );
    case 'jupiter': {
      const cols = [c1, '#b9743f', c2, c1, '#c9854a', c2, '#b9743f', c1];
      let o = '';
      for (let i = 0; i < 8; i++) o += wavyBand(-52 + i * 13, 14, cols[i], i % 2 ? 2.5 : 4);
      o +=
        '<g fill="#fff" fill-opacity=".45"><ellipse cx="-26" cy="-4" rx="8" ry="2.6"/><ellipse cx="30" cy="-18" rx="7" ry="2.4"/><ellipse cx="-10" cy="30" rx="9" ry="2.6"/></g>';
      o +=
        '<ellipse cx="14" cy="14" rx="13" ry="8" fill="#c8452d" stroke="#f2b48a" stroke-width="2"/>' +
        '<ellipse cx="14" cy="14" rx="6" ry="3.4" fill="#a8341f"/>';
      return o;
    }
    case 'saturn': {
      let o = '';
      for (let i = 0; i < 6; i++) o += wavyBand(-44 + i * 16, 8, i % 2 ? c2 : '#f6e3b0', 1.5);
      return o;
    }
    case 'uranus':
      return (
        `<rect x="-52" y="-10" width="104" height="9" fill="${c2}" fill-opacity=".55"/>` +
        '<rect x="-52" y="-26" width="104" height="6" fill="#fff" fill-opacity=".25"/>' +
        '<ellipse cx="-14" cy="-16" rx="14" ry="5" fill="#fff" fill-opacity=".3"/>'
      );
    case 'neptune':
      return (
        `<rect x="-52" y="-30" width="104" height="9" fill="${c2}" fill-opacity=".7"/>` +
        `<rect x="-52" y="18" width="104" height="11" fill="${c2}" fill-opacity=".55"/>` +
        `<ellipse cx="-8" cy="-4" rx="11" ry="7" fill="#14246a"/>` +
        '<path d="M-20 -14 q12 -8 26 -3" fill="none" stroke="#fff" stroke-width="3" stroke-linecap="round" stroke-opacity=".9"/>' +
        '<path d="M14 12 q10 -6 22 -2" fill="none" stroke="#fff" stroke-width="2.4" stroke-linecap="round" stroke-opacity=".8"/>'
      );
    default:
      return '';
  }
}

/** Static SVG markup for one planet (numbers and fixed paths only; colours from zoo-core). */
export function planetSvg(p: PlanetData): string {
  const id = p.id;
  const [c1] = p.colors;
  const ringKind = p.ring ? 'saturn' : id === 'uranus' ? 'uranus' : null;
  const vb = ringKind ? '-110 -80 220 160' : '-60 -60 120 120';
  const halo = id === 'earth' ? '<circle r="52" fill="none" stroke="#8fd8ff" stroke-opacity=".6" stroke-width="4"/>' : '';
  return (
    `<svg class="tele-art" viewBox="${vb}" aria-hidden="true" focusable="false">` +
    `<defs><clipPath id="tp-${id}-clip"><circle r="48"/></clipPath>` +
    `<clipPath id="tp-${id}-front"><rect x="-120" y="0" width="240" height="90"/></clipPath>` +
    `<radialGradient id="tp-${id}-glow"><stop offset=".7" stop-color="${c1}" stop-opacity=".5"/><stop offset="1" stop-color="${c1}" stop-opacity="0"/></radialGradient>` +
    `<radialGradient id="tp-${id}-sh" cx=".34" cy=".28" r=".95"><stop offset="0" stop-color="#fff" stop-opacity=".42"/><stop offset=".34" stop-color="#fff" stop-opacity="0"/><stop offset=".6" stop-color="${SHADOW}" stop-opacity="0"/><stop offset="1" stop-color="${SHADOW}" stop-opacity=".55"/></radialGradient></defs>` +
    `<circle r="64" fill="url(#tp-${id}-glow)"/>` +
    (ringKind ? ringSvg(ringKind, 'back', id) : '') +
    halo +
    `<g clip-path="url(#tp-${id}-clip)"><rect x="-50" y="-50" width="100" height="100" fill="${c1}"/>${details(p)}` +
    `<circle r="48" fill="url(#tp-${id}-sh)"/></g>` +
    `<path d="M-41 -16 A44 44 0 0 1 -16 -41" fill="none" stroke="#fff" stroke-opacity=".6" stroke-width="2.4" stroke-linecap="round"/>` +
    `<circle r="48" fill="none" stroke="${OUTLINE}" stroke-width="3"/>` +
    (ringKind ? ringSvg(ringKind, 'front', id) : '') +
    '</svg>'
  );
}

/** The Sun (decoration): glowing comic disc with short rays; the viewBox is 200 units wide, disc radius 64. */
export function sunSvg(): string {
  let rays = '';
  for (let i = 0; i < 14; i++) {
    const a = (i / 14) * Math.PI * 2;
    const l = i % 2 ? 84 : 96;
    const f = (r: number, da: number) => `${(Math.cos(a + da) * r).toFixed(1)},${(Math.sin(a + da) * r).toFixed(1)}`;
    rays += `<polygon points="${f(66, -0.12)} ${f(l, 0)} ${f(66, 0.12)}" fill="#ffb02e" stroke="${OUTLINE}" stroke-width="2.5" stroke-linejoin="round"/>`;
  }
  return (
    '<svg class="tele-sun-art" viewBox="-100 -100 200 200" aria-hidden="true" focusable="false">' +
    '<defs><radialGradient id="tp-sun-glow"><stop offset=".5" stop-color="#ffc83c" stop-opacity=".55"/><stop offset="1" stop-color="#ffc83c" stop-opacity="0"/></radialGradient>' +
    '<radialGradient id="tp-sun-disc" cx=".4" cy=".35" r=".8"><stop offset="0" stop-color="#fff8c2"/><stop offset=".55" stop-color="#ffd23f"/><stop offset="1" stop-color="#ff9a1f"/></radialGradient></defs>' +
    '<circle r="100" fill="url(#tp-sun-glow)"/>' +
    `<g class="tele-sun-rays">${rays}</g>` +
    `<circle r="64" fill="url(#tp-sun-disc)" stroke="${OUTLINE}" stroke-width="3.5"/>` +
    '<path d="M-48 -22 A52 52 0 0 1 -22 -48" fill="none" stroke="#fff" stroke-opacity=".7" stroke-width="4" stroke-linecap="round"/>' +
    '</svg>'
  );
}


export class TelescopeView {
  private readonly root: HTMLElement;
  private readonly title: HTMLElement;
  private readonly closeBtn: HTMLButtonElement;
  private readonly sky: HTMLElement;
  private readonly info: HTMLElement;
  private planets: PlanetData[] = [];
  private selected: string | null = null;
  private opened = false;

  constructor(
    private readonly app: TelescopeApp,
    private readonly onOpen: () => void = () => {},
    private readonly onClose: () => void = () => {},
  ) {
    this.root = el('div', 'tele');
    this.root.id = 'telescope-view';
    this.root.hidden = true;
    this.title = el('h2', 'tele-title');
    this.closeBtn = el('button', 'tele-close round', '✖');
    this.closeBtn.id = 'telescope-close';
    this.closeBtn.type = 'button';
    this.closeBtn.addEventListener('click', () => this.close());
    const eye = el('div', 'tele-eyepiece');
    this.sky = el('div', 'tele-sky');
    eye.append(this.sky);
    this.info = el('div', 'tele-info');
    this.info.id = 'telescope-info';
    this.info.setAttribute('aria-live', 'polite');
    this.root.append(this.title, eye, this.info, this.closeBtn);
    document.body.append(this.root);
    window.addEventListener('resize', () => this.opened && this.render());
  }

  get isOpen(): boolean {
    return this.opened;
  }

  open(): void {
    if (this.opened) return;
    const planets = parsePlanets(this.app.telescope_json?.());
    if (!planets) return;
    this.planets = planets;
    this.selected = null;
    this.opened = true;
    this.app.set_telescope_open?.(true);
    this.root.hidden = false;
    this.onOpen();
    this.render();
    this.closeBtn.focus({ preventScroll: true });
  }

  close(): void {
    if (!this.opened) return;
    this.opened = false;
    this.root.hidden = true;
    this.app.set_telescope_open?.(false);
    this.onClose();
  }

  /** Language / reading level change: the texts follow at once. */
  relabel(): void {
    if (this.opened) this.render();
  }

  private render(): void {
    this.title.textContent = this.app.t('telescope-title');
    this.closeBtn.setAttribute('aria-label', this.app.t('telescope-close'));
    this.sky.replaceChildren();
    // stars first (behind), then the planets
    const rnd = lcg(7);
    for (let i = 0; i < 34; i++) {
      const s = el('i', 'tele-star');
      s.style.left = `${(rnd() * 96 + 2).toFixed(1)}%`;
      s.style.top = `${(rnd() * 96 + 2).toFixed(1)}%`;
      const size = 2 + Math.floor(rnd() * 3);
      s.style.width = s.style.height = `${size}px`;
      s.style.animationDelay = `${(rnd() * 4).toFixed(2)}s`;
      s.style.animationDuration = `${(2.5 + rnd() * 3).toFixed(2)}s`;
      this.sky.append(s);
    }
    const w = this.sky.clientWidth || window.innerWidth / 2;
    const h = this.sky.clientHeight || window.innerHeight / 2;
    const { centres, sun, step } = lineLayout(w, h, this.planets.length);
    const base = discBase(step);
    // the Sun: only a part of it shows in the lower-left corner (decoration, behind everything)
    const sunEl = el('div', 'tele-sun');
    const sunSize = ((sun.r * 2) / 128) * 200;
    sunEl.style.width = sunEl.style.height = `${sunSize}px`;
    sunEl.style.left = `${sun.x - sunSize / 2}px`;
    sunEl.style.top = `${sun.y - sunSize / 2}px`;
    sunEl.innerHTML = sunSvg();
    this.sky.append(sunEl);
    // a faint dashed orbit hint along the line of planets
    const last = centres[centres.length - 1];
    const orbit = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    orbit.setAttribute('class', 'tele-orbit');
    orbit.setAttribute('viewBox', `0 0 ${w} ${h}`);
    const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
    line.setAttribute('x1', String(sun.x));
    line.setAttribute('y1', String(sun.y));
    line.setAttribute('x2', String(last.x));
    line.setAttribute('y2', String(last.y));
    orbit.append(line);
    this.sky.append(orbit);
    this.planets.forEach((p, i) => this.sky.append(this.planetButton(p, base, centres[i])));
    this.renderInfo();
  }

  private planetButton(p: PlanetData, base: number, c: { x: number; y: number }): HTMLElement {
    const d = base * p.size;
    const btn = el('button', 'tele-planet');
    btn.type = 'button';
    btn.dataset.planet = p.id;
    btn.dataset.kind = p.kind;
    btn.classList.toggle('selected', this.selected === p.id);
    btn.setAttribute('aria-label', this.app.t(p.name));
    // transparent hit area: always 64 px or the disc, never wider (the ring may overhang)
    const box = Math.max(64, d);
    btn.style.width = `${box}px`;
    btn.style.height = `${box}px`;
    btn.style.left = `${c.x}px`;
    btn.style.top = `${c.y}px`;
    btn.style.setProperty('--d', `${d}px`);
    btn.style.animationDelay = `${-(p.order * 0.9)}s`;
    const art = el('span', 'tele-art-wrap');
    const withRing = p.ring || p.id === 'uranus';
    const vbW = withRing ? 220 : 120;
    const vbH = withRing ? 160 : 120;
    art.style.width = `${(d / 96) * vbW}px`;
    art.style.height = `${(d / 96) * vbH}px`;
    art.innerHTML = planetSvg(p);
    btn.append(art);
    btn.addEventListener('click', () => {
      this.selected = p.id;
      for (const b of this.sky.querySelectorAll('.tele-planet')) {
        b.classList.toggle('selected', (b as HTMLElement).dataset.planet === p.id);
      }
      this.renderInfo();
    });
    return btn;
  }

  private renderInfo(): void {
    const p = this.planets.find((q) => q.id === this.selected);
    if (!p) {
      this.info.dataset.planet = '';
      this.info.replaceChildren(el('span', 'tele-hand', '👆'), el('p', 'tele-text', this.app.t('telescope-hint')));
      return;
    }
    const level = this.app.reading_level?.() ?? 'klasse1';
    const text = this.app.t(p.texts[level] ?? p.texts.klasse1);
    this.info.dataset.planet = p.id;
    this.info.dataset.kind = p.kind;
    const kind = el('p', 'tele-kind');
    kind.append(el('span', 'icon', KIND_ICONS[p.kind] ?? ''), document.createTextNode(` ${this.app.t(p.kind_key)}`));
    this.info.replaceChildren(el('h3', 'tele-name', this.app.t(p.name)), kind, el('p', 'tele-text', text));
  }
}
