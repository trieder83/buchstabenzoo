// Text textures (ART-ENVIRONMENT behaviour 7): the game asks for sign texts (Fluent, current
// language) and the host draws each into an offscreen 2D canvas — big bold rounded comic
// lettering, dark on a cream board with a dark outline — and hands the RGBA bytes back to
// Rust, which uploads them as decal textures. No font files are bundled: system / CSS generic
// fonts only (works offline). Generic: every later sign reuses this.

/** One text texture requested by the game (`App.text_textures()`). */
export interface TextTextureSpec {
  id: string;
  key: string;
  text: string;
  width: number;
  height: number;
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

/** Draws a sign text texture (straight-alpha RGBA, top row first). */
export function renderTextTexture(spec: TextTextureSpec): Uint8Array {
  const { width: w, height: h } = spec;
  const canvas = document.createElement('canvas');
  canvas.width = w;
  canvas.height = h;
  const ctx = canvas.getContext('2d', { willReadFrequently: true });
  if (!ctx) return new Uint8Array(w * h * 4);
  ctx.clearRect(0, 0, w, h);

  // cream board with a bold dark outline and rounded corners
  const line = Math.max(4, Math.round(h * 0.06));
  const r = Math.round(h * 0.16);
  ctx.beginPath();
  ctx.roundRect(line / 2, line / 2, w - line, h - line, r);
  ctx.fillStyle = CREAM;
  ctx.fill();
  ctx.lineWidth = line;
  ctx.strokeStyle = INK;
  ctx.stroke();

  // text: as large as fits inside the border
  const font = (px: number) => `900 ${px}px ${SIGN_FONT_FAMILY}`;
  const px = fitFontSize(
    (size) => {
      ctx.font = font(size);
      return ctx.measureText(spec.text).width;
    },
    w - line * 2 - h * 0.3,
    (h - line * 2) * 0.82,
  );
  ctx.font = font(px);
  ctx.textAlign = 'center';
  ctx.textBaseline = 'alphabetic';
  const m = ctx.measureText(spec.text);
  // centre the ink box vertically (ascent/descent of the actual glyphs)
  const asc = m.actualBoundingBoxAscent || px * 0.72;
  const desc = m.actualBoundingBoxDescent || 0;
  const y = h / 2 + (asc - desc) / 2;
  ctx.fillStyle = INK;
  ctx.lineJoin = 'round';
  ctx.lineWidth = Math.max(1, px * 0.05); // a touch bolder and rounder
  ctx.strokeStyle = INK;
  ctx.strokeText(spec.text, w / 2, y);
  ctx.fillText(spec.text, w / 2, y);

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
