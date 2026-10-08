// Last-resort card for a browser that hides the normal board panel / parental gate (ADS-044/045): an
// ad blocker's cosmetic filters, a "reader mode" or an overlay remover can set `display:none` on our
// elements. This card is a native `<dialog>` (top layer) with a random id, no class names and only
// inline styles (`display` marked !important, which also beats a filter's stylesheet rule), so
// there is nothing to match by name. It keeps the parental gate: question, then the press-and-hold,
// the link opens at the release of the finger. Texts come from `t()`, outside texts via `textContent`.
import { ParentalGate, type GateQuestion } from './ads';

export interface RescueOptions {
  t: (key: string) => string;
  url: string;
  /** The campaign card (picture + tagline); absent when only the gate was hidden. */
  card?: { imageUrl: string; tagline: string | null };
  question: GateQuestion;
  /** Starts directly at the gate question (the gate view itself was hidden). */
  startAtGate?: boolean;
  /** The link must be opened now (called from the finger release = a user gesture). */
  /** Gate steps for the anonymous counters (counter.ts). */
  onStep?: (s: 'answer_right' | 'answer_wrong' | 'hold_started' | 'hold_complete' | 'hold_cancelled') => void;
  onOpen: (url: string) => void;
  /** `opened`: closed because the link was opened (else ✖ / Esc / wrong answer). */
  onClose: (opened: boolean) => void;
}

export interface Rescue {
  dialog: HTMLDialogElement;
  close(opened?: boolean): void;
}

const BROWN = '#3b2314';
const BTN = `min-height:64px;min-width:64px;border-radius:32px;border:5px solid ${BROWN};background:#7cc46a;color:${BROWN};font:bold 20px sans-serif;padding:6px 16px;max-width:100%;box-sizing:border-box;touch-action:manipulation`;

function make<K extends keyof HTMLElementTagNameMap>(tag: K, css: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  e.style.cssText = css;
  if (text !== undefined) e.textContent = text;
  return e;
}

/** Marks a property important inline (a filter's `display:none !important` stylesheet rule then loses). */
function force(e: HTMLElement, prop: string, value: string): void {
  e.style.setProperty(prop, value, 'important');
}

export function randomName(): string {
  return `q${Math.random().toString(36).slice(2, 8)}${Date.now().toString(36).slice(-3)}`;
}

export function showRescue(o: RescueOptions): Rescue {
  const dialog = document.createElement('dialog');
  dialog.id = randomName();
  dialog.style.cssText = `position:fixed;top:0;left:0;right:0;bottom:0;margin:auto;box-sizing:border-box;width:min(94vw,640px);max-height:94vh;overflow:auto;padding:14px;border-radius:24px;border:6px solid ${BROWN};background:#fff8e7;color:${BROWN};text-align:center;font:bold 20px sans-serif;flex-direction:column;align-items:center;gap:12px;touch-action:manipulation`;
  force(dialog, 'display', 'flex');
  force(dialog, 'visibility', 'visible');
  force(dialog, 'opacity', '1');
  let raf = 0;
  let closed = false;
  const stop = (ev: Event) => ev.stopPropagation();
  for (const t of ['pointerdown', 'pointerup', 'touchstart', 'keydown', 'contextmenu']) dialog.addEventListener(t, stop);
  const close = (opened = false) => {
    if (closed) return;
    closed = true;
    cancelAnimationFrame(raf);
    try {
      dialog.close();
    } catch {
      /* not open */
    }
    dialog.remove();
    o.onClose(opened);
  };
  const xBtn = () => {
    const x = make('button', `align-self:flex-end;width:64px;height:64px;border-radius:50%;border:4px solid ${BROWN};background:#fff;font-size:28px;color:${BROWN}`, '✖');
    x.type = 'button';
    x.setAttribute('aria-label', o.t('ad-close'));
    x.addEventListener('click', () => close());
    return x;
  };
  const host = new URL(o.url).hostname;

  const showCard = () => {
    const card = o.card!;
    const img = make('img', `display:block;max-width:100%;max-height:34vh;border:4px solid ${BROWN};border-radius:12px;background:#fff8e7`);
    img.alt = '';
    img.src = card.imageUrl;
    const link = make('button', `${BTN};display:inline-flex;gap:12px;align-items:center;justify-content:center`);
    link.type = 'button';
    link.append(make('span', '', '🔗'), make('span', '', host));
    link.setAttribute('aria-label', o.t('ad-link-open'));
    link.addEventListener('click', showGate);
    dialog.replaceChildren(xBtn(), img, ...(card.tagline ? [make('p', 'margin:0', card.tagline)] : []), link);
  };

  const showGate = () => {
    const gate = new ParentalGate(o.question);
    const q = o.question;
    const choices = make('div', 'display:grid;grid-template-columns:1fr 1fr;gap:12px;width:100%');
    for (const n of q.options) {
      const b = make('button', `${BTN};background:#ffd65c`, q.labels ? q.labels[n] : String(n));
      b.type = 'button';
      b.addEventListener('click', () => {
        if (gate.answer(n) === 'hold') {
          o.onStep?.('answer_right');
          showHold(gate);
        } else {
          o.onStep?.('answer_wrong');
          close();
        }
      });
      choices.append(b);
    }
    dialog.replaceChildren(
      xBtn(),
      make('h2', 'margin:0;font-size:1.2em', o.t('ad-gate-title')),
      make('div', '', o.t('ad-gate-sum')),
      make('div', 'font-size:1.8em', q.prompt ?? `${q.a} ${q.op === '-' ? '−' : '+'} ${q.b} = ?`),
      choices,
    );
  };

  const showHold = (gate: ParentalGate) => {
    const hold = make('button', `width:150px;height:150px;border-radius:50%;border:5px solid ${BROWN};background:#ffd65c;font-size:64px;touch-action:none;user-select:none;-webkit-user-select:none;-webkit-touch-callout:none`, '✋');
    hold.type = 'button';
    let ready = false;
    const frame = () => {
      const p = gate.progress(performance.now());
      hold.style.background = `conic-gradient(#e8604c ${Math.round(p * 360)}deg,#ffd65c 0)`;
      if (gate.stage === 'open') {
        ready = true;
        o.onStep?.('hold_complete');
        hold.textContent = '✔';
        raf = 0;
        return;
      }
      raf = requestAnimationFrame(frame);
    };
    hold.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      try {
        hold.setPointerCapture?.(e.pointerId);
      } catch {
        /* synthetic pointer */
      }
      o.onStep?.('hold_started');
      gate.holdStart(performance.now());
      cancelAnimationFrame(raf);
      raf = requestAnimationFrame(frame);
    });
    hold.addEventListener('pointerup', () => {
      if (ready) {
        o.onOpen(o.url);
        close(true);
        return;
      }
      o.onStep?.('hold_cancelled');
      gate.holdEnd();
      cancelAnimationFrame(raf);
      hold.style.background = '#ffd65c';
    });
    hold.addEventListener('pointercancel', () => {
      if (ready) {
        // no release gesture: a plain link to tap is always allowed
        const a = make('a', `${BTN};display:inline-flex;gap:12px;align-items:center;text-decoration:none`);
        a.href = o.url;
        a.target = '_blank';
        a.rel = 'noopener noreferrer';
        a.append(make('span', '', '🔗'), make('span', '', host));
        a.addEventListener('click', () => window.setTimeout(() => close(true), 300));
        dialog.append(a);
        ready = false;
        return;
      }
      gate.holdEnd();
      cancelAnimationFrame(raf);
    });
    hold.addEventListener('contextmenu', (e) => e.preventDefault());
    dialog.replaceChildren(xBtn(), make('h2', 'margin:0;font-size:1.2em', o.t('ad-gate-title')), make('div', '', o.t('ad-gate-hold')), hold);
  };

  if (o.startAtGate || !o.card) showGate();
  else showCard();
  dialog.addEventListener('close', () => close());
  document.body.append(dialog);
  try {
    dialog.showModal();
  } catch {
    dialog.setAttribute('open', '');
  }
  return { dialog, close };
}
