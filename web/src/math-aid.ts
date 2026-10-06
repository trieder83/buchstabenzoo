// Visual aid of the golf-cart note task (CONT-MATH "Cart note" rule 4): drawn with DOM
// elements from the data of `App.interact` — dots, tens-rods, block grids, equal groups and a
// bar. No text, so it works on every reading level (and `kiga`).

export type MathAidData =
  | { kind: 'dots'; a: number; b: number; minus: boolean }
  | { kind: 'blocks'; rows: number; cols: number }
  | { kind: 'groups'; groups: number; per: number }
  | { kind: 'bar'; parts: number; shaded: number };

function el(tag: string, cls: string, text?: string): HTMLElement {
  const e = document.createElement(tag);
  e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

/** `n` as tens-rods and single dots (a rod = 10; below 21 only dots). */
export function countParts(n: number): { rods: number; dots: number } {
  return n > 20 ? { rods: Math.floor(n / 10), dots: n % 10 } : { rods: 0, dots: n };
}

/** A number as a row of rods and dots. */
function amount(n: number, cls: string): HTMLElement {
  const g = el('span', `aid-amount ${cls}`);
  const { rods, dots } = countParts(n);
  for (let i = 0; i < rods; i++) g.append(el('i', 'aid-rod'));
  for (let i = 0; i < dots; i++) g.append(el('i', 'aid-dot'));
  g.dataset.count = String(n);
  return g;
}

/** The aid element (class `math-aid`, `data-aid` = the kind). */
export function renderAid(aid: MathAidData): HTMLElement {
  const root = el('div', 'math-aid');
  root.dataset.aid = aid.kind;
  switch (aid.kind) {
    case 'dots':
      root.append(
        amount(aid.a, 'first'),
        el('span', 'aid-op', aid.minus ? '−' : '+'),
        amount(aid.b, aid.minus ? 'second taken' : 'second'),
      );
      break;
    case 'blocks': {
      if (aid.cols <= 10) {
        const grid = el('div', 'aid-grid');
        grid.style.setProperty('--cols', String(aid.cols));
        for (let i = 0; i < aid.rows * aid.cols; i++) grid.append(el('i', 'aid-cell'));
        grid.dataset.cells = String(aid.rows * aid.cols);
        root.append(grid);
      } else {
        // wide rows: every row is a number of tens-rods and ones
        const rows = el('div', 'aid-rows');
        for (let r = 0; r < aid.rows; r++) rows.append(amount(aid.cols, 'row'));
        root.append(rows);
      }
      break;
    }
    case 'groups': {
      const groups = el('div', 'aid-groups');
      for (let g = 0; g < aid.groups; g++) groups.append(amount(aid.per, 'group'));
      groups.dataset.groups = String(aid.groups);
      root.append(groups);
      break;
    }
    case 'bar': {
      const bar = el('div', 'aid-bar');
      for (let p = 0; p < aid.parts; p++) {
        const seg = el('i', p < aid.shaded ? 'aid-part shaded' : 'aid-part');
        bar.append(seg);
      }
      bar.dataset.parts = String(aid.parts);
      bar.dataset.shaded = String(aid.shaded);
      root.append(bar);
      break;
    }
  }
  return root;
}
