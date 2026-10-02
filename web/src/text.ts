// Text textures (ART-ENVIRONMENT behaviour 7): the game asks for sign texts (Fluent, current
// language) and the host draws each into an offscreen 2D canvas — big bold rounded comic
// lettering, dark on a cream board with a dark outline — and hands the RGBA bytes back to
// Rust, which uploads them as decal textures. No font files are bundled: system / CSS generic
// fonts only (works offline). Generic: every later sign reuses this. Food box labels
// (GAME-FEED §1) also carry a vector pictogram (pictograms.ts) above the word.

import { drawPictogram } from './pictograms';

/** One text texture requested by the game (`App.text_textures()`). */
export interface TextTextureSpec {
  id: string;
  key: string;
  text: string;
  width: number;
  height: number;
  /** The shared food label atlas (FEED-030): the cells to draw. */
  cells?: AtlasCell[];
}

/** The subset of the WASM `App` used here. */
export interface TextApp {
  text_textures_dirty(): boolean;
  text_textures(): string;
  set_text_texture(id: string, width: number, height: number, rgba: Uint8Array): boolean;
}

/** Bold rounded sans from the system, falling back to generic families. */
export const SIGN_FONT_FAMILY =
  '"Arial Rounded MT Bold", "Nunito", "Varela Round", "Comic Neue", "Comic Sans MS", "DejaVu Sans", "Verdana", sans-serif';
export const INK = '#3b2314';
export const CREAM = '#fff3d6';

/**
 * Largest font size (px) whose text fits `maxWidth` × `maxHeight`. `measure(px)` returns the
 * text width at a font size; letter height is taken as the font size.
 */
export function fitFontSize(measure: (px: number) => number, maxWidth: number, maxHeight: number): number {
  let px = Math.floor(maxHeight);
  const w = measure(px);
  if (w > maxWidth && w > 0) px = Math.floor((px * maxWidth) / w);
  return Math.max(8, px);
}

/** Layout of a label with a pictogram above the word (FEED-032); all values in px. */
export interface LabelLayout {
  pictoSize: number;
  pictoX: number;
  pictoY: number;
  /** The word's box: horizontal margin, centre line and maximum height. */
  wordCenterY: number;
  wordMaxH: number;
}

/**
 * Pictogram (share `scale` of the inner height, square, centred) above the word (the rest).
 * The pictogram never gets wider than the inner width.
 */
export function labelLayout(w: number, h: number, line: number, scale: number): LabelLayout {
  const inner = h - line * 2;
  const pictoSize = Math.min(inner * scale, w - line * 2);
  const pictoY = line + inner * 0.01;
  const wordTop = line + inner * scale + inner * 0.03;
  const wordMaxH = Math.max(8, h - line - wordTop - inner * 0.04);
  return { pictoSize, pictoX: (w - pictoSize) / 2, pictoY, wordCenterY: wordTop + wordMaxH / 2, wordMaxH };
}

/** One cell of the food label atlas (FEED-030): lid = pictogram above the word, front = row. */
export interface AtlasCell {
  part: 'lid' | 'front';
  x: number;
  y: number;
  w: number;
  h: number;
  text: string;
  pictogram: string;
  pictogram_scale: number;
}

/** The 2D context calls the label drawing needs (a real canvas context). */
type Ctx = CanvasRenderingContext2D;

/** Cream rounded board with a dark outline; returns the outline width. */
function board(ctx: Ctx, w: number, h: number, lineFrac: number): number {
  const line = Math.max(3, Math.round(h * lineFrac));
  ctx.beginPath();
  ctx.roundRect(line / 2, line / 2, w - line, h - line, Math.round(h * 0.16));
  ctx.fillStyle = CREAM;
  ctx.fill();
  ctx.lineWidth = line;
  ctx.strokeStyle = INK;
  ctx.stroke();
  return line;
}

/** Draws `text` as large as fits `maxW` x `maxH`, centred at (`cx`, `cy`). */
function drawWord(ctx: Ctx, text: string, cx: number, cy: number, maxW: number, maxH: number): void {
  const font = (px: number) => `900 ${px}px ${SIGN_FONT_FAMILY}`;
  const px = fitFontSize(
    (size) => {
      ctx.font = font(size);
      return ctx.measureText(text).width;
    },
    maxW,
    maxH,
  );
  ctx.font = font(px);
  ctx.textAlign = 'center';
  ctx.textBaseline = 'alphabetic';
  const m = ctx.measureText(text);
  // centre the ink box vertically (ascent/descent of the actual glyphs)
  const asc = m.actualBoundingBoxAscent || px * 0.72;
  const desc = m.actualBoundingBoxDescent || 0;
  const y = cy + (asc - desc) / 2;
  ctx.fillStyle = INK;
  ctx.lineJoin = 'round';
  ctx.lineWidth = Math.max(1, px * 0.05); // a touch bolder and rounder
  ctx.strokeStyle = INK;
  ctx.strokeText(text, cx, y);
  ctx.fillText(text, cx, y);
}

/** Draws one atlas cell at its place (clipped to the cell). */
function drawCell(ctx: Ctx, c: AtlasCell): void {
  ctx.save();
  ctx.translate(c.x, c.y);
  ctx.beginPath();
  ctx.rect(0, 0, c.w, c.h);
  ctx.clip();
  ctx.clearRect(0, 0, c.w, c.h);
  if (c.part === 'lid') {
    const line = board(ctx, c.w, c.h, 0.05);
    const lay = labelLayout(c.w, c.h, line, Math.min(0.78, c.pictogram_scale * 1.25));
    drawPictogram(ctx, c.pictogram, lay.pictoX, lay.pictoY, lay.pictoSize);
    drawWord(ctx, c.text, c.w / 2, lay.wordCenterY, c.w - line * 3, lay.wordMaxH * 0.9);
  } else {
    // front: small pictogram on the left (its size follows the level), the word beside it
    const line = board(ctx, c.w, c.h, 0.07);
    const inner = c.h - line * 2;
    const pic = Math.min(inner, inner * (c.pictogram_scale / 0.62));
    drawPictogram(ctx, c.pictogram, line * 1.5, line + (inner - pic) / 2, pic);
    const left = line * 1.5 + pic + line;
    drawWord(ctx, c.text, (left + c.w - line) / 2, c.h / 2, c.w - line - left - line, inner * 0.8);
  }
  ctx.restore();
}

/** Draws a sign text texture (straight-alpha RGBA, top row first). */
export function renderTextTexture(spec: TextTextureSpec): Uint8Array {
  const { width: w, height: h } = spec;
  const canvas = document.createElement('canvas');
  canvas.width = w;
  canvas.height = h;
  const ctx = canvas.getContext('2d', { willReadFrequently: true });
  if (!ctx) return new Uint8Array(w * h * 4);
  ctx.clearRect(0, 0, w, h);

  if (spec.cells) {
    for (const c of spec.cells) drawCell(ctx, c);
  } else {
    // cream board with a bold dark outline and rounded corners; text as large as fits
    const line = board(ctx, w, h, 0.06);
    drawWord(ctx, spec.text, w / 2, h / 2, w - line * 2 - h * 0.3, (h - line * 2) * 0.82);
  }

  const data = ctx.getImageData(0, 0, w, h).data;
  return new Uint8Array(data.buffer, data.byteOffset, data.byteLength);
}

/** Renders every requested text texture if the game marked them dirty (start, language). */
export function updateTextTextures(app: TextApp, render = renderTextTexture): number {
  if (!app.text_textures_dirty()) return 0;
  const specs = JSON.parse(app.text_textures()) as TextTextureSpec[];
  let n = 0;
  for (const spec of specs) {
    if (app.set_text_texture(spec.id, spec.width, spec.height, render(spec))) n += 1;
  }
  return n;
}
