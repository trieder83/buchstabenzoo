import { describe, expect, it } from 'vitest';
import { fitFontSize, updateTextTextures, type TextApp, type TextTextureSpec } from './text';

describe('text textures (ART-ENVIRONMENT 7)', () => {
  it('fits the font to the height, shrinking long texts to the width', () => {
    // 0.6 px of width per px of font size per letter
    const word = (letters: number) => (px: number) => letters * 0.6 * px;
    expect(fitFontSize(word(6), 400, 100)).toBe(100); // "Futter": 360 px wide at 100 px
    expect(fitFontSize(word(12), 400, 100)).toBe(55); // shrinks to fit 400 px
    expect(fitFontSize(word(4), 400, 100)).toBe(100); // "Food"
  });

  it('renders only when the game marks the textures dirty (start, language change)', () => {
    let dirty = true;
    const uploaded: string[] = [];
    const specs: TextTextureSpec[] = [{ id: 'text:sign-food-storage', key: 'sign-food-storage', text: 'Futter', width: 8, height: 2 }];
    const app: TextApp = {
      text_textures_dirty: () => dirty,
      text_textures: () => {
        dirty = false;
        return JSON.stringify(specs);
      },
      set_text_texture: (id, w, h, rgba) => {
        uploaded.push(`${id}:${w}x${h}:${rgba.length}`);
        return true;
      },
    };
    const render = (s: TextTextureSpec) => new Uint8Array(s.width * s.height * 4);
    expect(updateTextTextures(app, render)).toBe(1);
    expect(uploaded).toEqual(['text:sign-food-storage:8x2:64']);
    expect(updateTextTextures(app, render)).toBe(0);
    dirty = true;
    specs[0].text = 'Food';
    expect(updateTextTextures(app, render)).toBe(1);
  });
});
