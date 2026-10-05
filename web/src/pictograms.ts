// Food pictograms (GAME-FEED §1, FEED-031): one flat-colour vector drawing with a dark outline
// per food on a 128 x 128 design grid. They are drawn on canvases (food box label textures in
// text.ts, the box panel in ui.ts), not emoji: emoji fonts differ between headless Linux,
// Android and iOS. Deterministic: no randomness, no time. Pure canvas paths, no fonts.

/** The canvas 2D calls the drawers use (a recording mock implements it in the tests). */
export interface PictoCtx {
  save(): void;
  restore(): void;
  translate(x: number, y: number): void;
  scale(x: number, y: number): void;
  beginPath(): void;
  closePath(): void;
  moveTo(x: number, y: number): void;
  lineTo(x: number, y: number): void;
  quadraticCurveTo(cx: number, cy: number, x: number, y: number): void;
  bezierCurveTo(c1x: number, c1y: number, c2x: number, c2y: number, x: number, y: number): void;
  arc(x: number, y: number, r: number, a0: number, a1: number): void;
  ellipse(x: number, y: number, rx: number, ry: number, rot: number, a0: number, a1: number): void;
  fill(): void;
  stroke(): void;
  fillStyle: string | CanvasGradient | CanvasPattern;
  strokeStyle: string | CanvasGradient | CanvasPattern;
  lineWidth: number;
  lineJoin: CanvasLineJoin;
  lineCap: CanvasLineCap;
}

export const PICTO_GRID = 128;
const OUT = '#3b2314';
const LINE = 5;

/** Fills the current path, then outlines it. */
function paint(c: PictoCtx, fill: string, line = LINE): void {
  c.fillStyle = fill;
  c.fill();
  c.lineWidth = line;
  c.strokeStyle = OUT;
  c.stroke();
}

function blob(c: PictoCtx, x: number, y: number, rx: number, ry: number, fill: string, rot = 0): void {
  c.beginPath();
  c.ellipse(x, y, rx, ry, rot, 0, Math.PI * 2);
  paint(c, fill);
}

function poly(c: PictoCtx, pts: [number, number][], fill: string, line = LINE): void {
  c.beginPath();
  c.moveTo(pts[0][0], pts[0][1]);
  for (const p of pts.slice(1)) c.lineTo(p[0], p[1]);
  c.closePath();
  paint(c, fill, line);
}

/** A leaf from base to tip with a bulge to both sides. */
function leaf(c: PictoCtx, bx: number, by: number, tx: number, ty: number, w: number, fill: string): void {
  const mx = (bx + tx) / 2;
  const my = (by + ty) / 2;
  const nx = -(ty - by);
  const ny = tx - bx;
  const len = Math.hypot(nx, ny) || 1;
  const ox = (nx / len) * w;
  const oy = (ny / len) * w;
  c.beginPath();
  c.moveTo(bx, by);
  c.quadraticCurveTo(mx + ox, my + oy, tx, ty);
  c.quadraticCurveTo(mx - ox, my - oy, bx, by);
  c.closePath();
  paint(c, fill);
}

function stroke(c: PictoCtx, pts: [number, number][], width = LINE, color = OUT): void {
  c.beginPath();
  c.moveTo(pts[0][0], pts[0][1]);
  for (const p of pts.slice(1)) c.lineTo(p[0], p[1]);
  c.lineWidth = width;
  c.strokeStyle = color;
  c.stroke();
}

const GREEN = '#6cc04a';
const GREEN_D = '#3f9a3a';

const DRAWERS: Record<string, (c: PictoCtx) => void> = {
  grass(c) {
    leaf(c, 40, 112, 22, 32, 11, GREEN_D);
    leaf(c, 88, 112, 106, 36, 11, GREEN_D);
    leaf(c, 56, 112, 44, 16, 14, GREEN);
    leaf(c, 72, 112, 84, 14, 14, GREEN);
    leaf(c, 64, 112, 64, 8, 13, '#8bd45a');
  },
  melons(c) {
    // half melon: green rind, red flesh, seeds
    c.beginPath();
    c.arc(64, 52, 52, 0, Math.PI);
    c.closePath();
    paint(c, '#2f8a3c');
    c.beginPath();
    c.arc(64, 52, 40, 0, Math.PI);
    c.closePath();
    paint(c, '#ff5a5f', 4);
    for (const [x, y] of [
      [40, 56],
      [64, 66],
      [88, 56],
      [52, 38],
      [76, 38],
    ] as [number, number][])
      blob(c, x, y, 3.5, 6, OUT, 0.3);
  },
  bamboo(c) {
    for (const [x, w] of [
      [44, 18],
      [84, 18],
    ] as [number, number][]) {
      poly(c, [[x - w / 2, 120], [x - w / 2, 8], [x + w / 2, 8], [x + w / 2, 120]], '#8bd45a');
      for (const y of [34, 66, 98]) poly(c, [[x - w / 2 - 3, y], [x - w / 2 - 3, y + 7], [x + w / 2 + 3, y + 7], [x + w / 2 + 3, y]], GREEN_D, 4);
    }
    leaf(c, 53, 66, 112, 40, 9, GREEN);
    leaf(c, 75, 98, 20, 74, 9, GREEN);
  },
  eucalyptus(c) {
    stroke(c, [[64, 120], [64, 14]], 6);
    for (const [y, dx] of [
      [92, 40],
      [62, 36],
      [34, 28],
    ] as [number, number][]) {
      leaf(c, 64, y + 6, 64 - dx, y - 16, 10, '#7fb8a4');
      leaf(c, 64, y + 6, 64 + dx, y - 16, 10, '#7fb8a4');
    }
    leaf(c, 64, 20, 64, 4, 8, '#9fd0bc');
  },
  hay(c) {
    // bale of straw tied with a red band
    for (const [x0, y0, x1, y1] of [
      [20, 40, 8, 16],
      [44, 36, 40, 8],
      [68, 36, 76, 8],
      [92, 40, 108, 14],
    ]) stroke(c, [[x0, y0], [x1, y1]], 6, '#d9a31a');
    poly(c, [[16, 40], [112, 40], [118, 112], [10, 112]], '#f2c94c');
    for (const x of [34, 56, 78, 98]) stroke(c, [[x, 50], [x - 3, 104]], 3, '#c78d12');
    poly(c, [[48, 38], [76, 38], [80, 114], [44, 114]], '#e5483e');
  },
  fish_food(c) {
    // a fish
    c.beginPath();
    c.moveTo(20, 64);
    c.quadraticCurveTo(52, 20, 92, 52);
    c.lineTo(116, 28);
    c.lineTo(116, 100);
    c.lineTo(92, 76);
    c.quadraticCurveTo(52, 108, 20, 64);
    c.closePath();
    paint(c, '#ff9f2e');
    stroke(c, [[56, 38], [62, 90]], 4, '#d9701a');
    stroke(c, [[72, 42], [76, 86]], 4, '#d9701a');
    c.beginPath();
    c.arc(40, 58, 7, 0, Math.PI * 2);
    paint(c, '#ffffff', 4);
    c.beginPath();
    c.arc(38, 58, 3, 0, Math.PI * 2);
    c.fillStyle = OUT;
    c.fill();
  },
  bananas(c) {
    // two bananas
    for (const dx of [0, 18]) {
      c.beginPath();
      c.moveTo(30 + dx, 22);
      c.bezierCurveTo(30 + dx, 90, 70 + dx, 118, 112, 96);
      c.bezierCurveTo(80 + dx, 98, 52 + dx, 74, 46 + dx, 18);
      c.closePath();
      paint(c, '#ffd93b');
    }
    poly(c, [[28, 22], [48, 22], [46, 8], [30, 8]], '#8a5a35');
    c.beginPath();
    c.arc(111, 96, 4, 0, Math.PI * 2);
    c.fillStyle = OUT;
    c.fill();
  },
  leaves(c) {
    leaf(c, 30, 104, 104, 24, 30, '#e8872e');
    stroke(c, [[30, 104], [104, 24]], 4);
    stroke(c, [[56, 78], [58, 50]], 3);
    stroke(c, [[56, 78], [86, 82]], 3);
    stroke(c, [[78, 54], [80, 34]], 3);
    stroke(c, [[78, 54], [100, 56]], 3);
    stroke(c, [[30, 104], [16, 118]], 6);
  },
  meat(c) {
    // drumstick: bone with two knobs, big brown-red meat
    stroke(c, [[70, 70], [106, 106]], 12);
    stroke(c, [[70, 70], [106, 106]], 6, '#fff3d6');
    blob(c, 108, 96, 8, 8, '#fff3d6');
    blob(c, 98, 108, 8, 8, '#fff3d6');
    c.beginPath();
    c.moveTo(14, 54);
    c.bezierCurveTo(10, 14, 62, 6, 84, 30);
    c.bezierCurveTo(100, 50, 78, 88, 52, 84);
    c.bezierCurveTo(34, 82, 16, 74, 14, 54);
    c.closePath();
    paint(c, '#c0453a');
    stroke(c, [[32, 40], [56, 34]], 4, '#f08a7a');
  },
  berries(c) {
    leaf(c, 64, 44, 98, 8, 10, GREEN);
    stroke(c, [[64, 46], [64, 30]], 5);
    for (const [x, y] of [
      [40, 84],
      [88, 84],
      [64, 62],
    ] as [number, number][]) {
      blob(c, x, y, 25, 25, '#4a5bd4');
      blob(c, x - 8, y - 8, 5, 5, '#a9b4ff');
    }
  },
  beetles(c) {
    for (const s of [-1, 1]) {
      stroke(c, [[64 + s * 24, 52], [64 + s * 52, 36]], 5);
      stroke(c, [[64 + s * 28, 72], [64 + s * 58, 76]], 5);
      stroke(c, [[64 + s * 24, 92], [64 + s * 50, 112]], 5);
    }
    blob(c, 64, 78, 30, 38, '#2f4a8f');
    blob(c, 64, 30, 16, 14, '#3b2314');
    stroke(c, [[64, 44], [64, 114]], 4);
    blob(c, 52, 66, 5, 7, '#6d8fe0');
  },
  fruit(c) {
    // a red apple with stem and leaf
    c.beginPath();
    c.moveTo(64, 36);
    c.bezierCurveTo(100, 14, 124, 52, 108, 88);
    c.bezierCurveTo(98, 112, 80, 118, 64, 108);
    c.bezierCurveTo(48, 118, 30, 112, 20, 88);
    c.bezierCurveTo(4, 52, 28, 14, 64, 36);
    c.closePath();
    paint(c, '#e8453c');
    stroke(c, [[64, 36], [68, 12]], 6);
    leaf(c, 68, 22, 104, 8, 10, GREEN);
    blob(c, 38, 56, 6, 10, '#ff9a8a', -0.4);
  },
  worms(c) {
    // two wavy pink worms
    for (const [y, tone] of [
      [46, '#f08aa0'],
      [86, '#e4708a'],
    ] as [number, string][]) {
      c.beginPath();
      c.moveTo(16, y);
      c.bezierCurveTo(32, y - 36, 52, y - 36, 64, y);
      c.bezierCurveTo(76, y + 36, 96, y + 36, 112, y);
      c.lineCap = 'round';
      c.lineWidth = 22;
      c.strokeStyle = OUT;
      c.stroke();
      c.lineWidth = 12;
      c.strokeStyle = tone;
      c.stroke();
    }
  },
  nectar(c) {
    // a flower with a golden nectar drop
    for (let k = 0; k < 6; k++) {
      const a = (k * Math.PI) / 3;
      blob(c, 64 + Math.cos(a) * 30, 56 + Math.sin(a) * 30, 18, 18, '#ff8fb8');
    }
    blob(c, 64, 56, 18, 18, '#ffd93b');
    c.beginPath();
    c.moveTo(64, 94);
    c.quadraticCurveTo(84, 112, 64, 124);
    c.quadraticCurveTo(44, 112, 64, 94);
    c.closePath();
    paint(c, '#f2a81d', 4);
  },
  // night_2 foods (GAME-FEED "Basic food and treats"): friendly cartoon drawings, nothing scary
  fish(c) {
    // plain side-on cartoon fish, orange, tail on the left
    poly(c, [[20, 64], [8, 44], [8, 84]], '#f29a2e');
    blob(c, 66, 64, 44, 26, '#ff9f3a');
    blob(c, 70, 74, 28, 10, '#ffd9a0');
    poly(c, [[56, 40], [74, 28], [84, 42]], '#f27a1d');
    blob(c, 94, 56, 6, 6, '#ffffff');
    blob(c, 95, 56, 2.5, 2.5, OUT);
  },
  crickets(c) {
    // green cricket: long hind leg, antennae
    stroke(c, [[100, 52], [118, 28]]);
    stroke(c, [[104, 56], [124, 44]]);
    stroke(c, [[46, 74], [30, 52], [18, 92]], 6, '#3b8f2a');
    blob(c, 58, 76, 36, 20, '#6cc24a', -0.15);
    blob(c, 98, 64, 15, 14, '#6cc24a');
    blob(c, 104, 60, 4, 4, '#ffffff');
    stroke(c, [[56, 94], [60, 112]]);
    stroke(c, [[78, 90], [86, 110]]);
  },
  flies(c) {
    // round dark fly, two clear wings, big eyes
    blob(c, 46, 40, 24, 12, 'rgba(210,235,255,0.9)', -0.5);
    blob(c, 82, 40, 24, 12, 'rgba(210,235,255,0.9)', 0.5);
    blob(c, 64, 78, 26, 26, '#3a3a46');
    blob(c, 64, 52, 17, 15, '#4a4a58');
    blob(c, 56, 48, 8, 8, '#ffffff');
    blob(c, 72, 48, 8, 8, '#ffffff');
    blob(c, 57, 49, 3.5, 3.5, OUT);
    blob(c, 71, 49, 3.5, 3.5, OUT);
  },
  eggs(c) {
    // two cream eggs, one speckled
    blob(c, 48, 76, 26, 34, '#fff1c9', -0.15);
    blob(c, 86, 78, 24, 32, '#f6e2a8', 0.2);
    blob(c, 80, 70, 3.5, 3.5, '#b58a4a');
    blob(c, 94, 82, 3, 3, '#b58a4a');
    blob(c, 84, 90, 3.5, 3.5, '#b58a4a');
    blob(c, 92, 66, 2.5, 2.5, '#b58a4a');
  },
  frozen_insects(c) {
    // a cricket silhouette inside a pale-blue ice cube with a snowflake
    poly(c, [[20, 28], [108, 28], [108, 108], [20, 108]], '#bfe6ff');
    blob(c, 62, 76, 28, 14, '#3f9a2a', -0.15);
    blob(c, 88, 68, 10, 9, '#3f9a2a');
    stroke(c, [[40, 80], [32, 62], [26, 94]], 4, '#3f9a2a');
    stroke(c, [[34, 44], [34, 56]], 3, '#ffffff');
    stroke(c, [[28, 50], [40, 50]], 3, '#ffffff');
    stroke(c, [[30, 46], [38, 54]], 2, '#ffffff');
    stroke(c, [[38, 46], [30, 54]], 2, '#ffffff');
  },
  bone(c) {
    // cartoon bone, cream, knobs at both ends
    poly(c, [[34, 52], [94, 76], [90, 88], [30, 64]], '#fff1d0');
    blob(c, 28, 46, 13, 12, '#fff1d0');
    blob(c, 38, 36, 13, 12, '#fff1d0');
    blob(c, 98, 82, 13, 12, '#fff1d0');
    blob(c, 88, 92, 13, 12, '#fff1d0');
    poly(c, [[34, 54], [92, 76], [89, 85], [33, 63]], '#fff1d0', 1);
  },
};

/** Ids that have a pictogram (= the food ids of `Food::ALL`). */
export const PICTOGRAM_IDS: string[] = Object.keys(DRAWERS);

/** Draws pictogram `id` into the square `size` px at (`x`, `y`); false for an unknown id. */
export function drawPictogram(c: PictoCtx, id: string, x: number, y: number, size: number): boolean {
  const draw = DRAWERS[id];
  if (!draw) return false;
  c.save();
  c.translate(x, y);
  c.scale(size / PICTO_GRID, size / PICTO_GRID);
  c.lineJoin = 'round';
  c.lineCap = 'round';
  draw(c);
  c.restore();
  return true;
}

/** A canvas with pictogram `id` for the reading panel; `scale` sets its CSS size (FEED-033). */
export function pictogramCanvas(id: string, scale: number): HTMLCanvasElement {
  const px = 192;
  const canvas = document.createElement('canvas');
  canvas.width = px;
  canvas.height = px;
  canvas.style.height = `max(${Math.round(scale * 28)}vh, 44px)`;
  canvas.style.width = canvas.style.height;
  const ctx = canvas.getContext('2d');
  if (ctx) drawPictogram(ctx, id, 0, 0, px);
  return canvas;
}
