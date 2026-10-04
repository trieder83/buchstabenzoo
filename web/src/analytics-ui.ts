// Settings row + consent dialog of the opt-in analytics (TECH-PLATFORMS "Analytics (opt-in)").
// The 📊 button turns analytics on only through the parental gate (sum, 3 s hold; the same
// `ParentalGate` as the ad links), then a card with the privacy note and Erlauben / Nein danke.
// Switching off needs no gate. At start only a small NON-BLOCKING notice shows for 3 s (PLAT-033):
// it records nothing, a tap on it starts the same parental gate; consent is only given in the dialog.
// The text is shown here, never as a link.
import type { Analytics } from './analytics';
import { makeGateQuestion, ParentalGate } from './ads';

export interface AnalyticsUiApp {
  t(key: string): string;
}

const STYLE = `
#analytics-dialog{position:fixed;inset:0;z-index:9;display:flex;align-items:center;justify-content:center;background:rgba(40,25,15,.6);pointer-events:auto;touch-action:none;user-select:none;-webkit-user-select:none}
#analytics-dialog[hidden]{display:none}
#analytics-notice{position:fixed;left:50%;bottom:calc(10px + env(safe-area-inset-bottom));transform:translateX(-50%);z-index:4;max-width:min(70vw,460px);padding:8px 16px;border-radius:22px;border:4px solid #3b2314;background:#fff8e7;color:#3b2314;font:bold max(2.2vh,14px)/1.2 sans-serif;text-align:center;box-shadow:0 4px 0 rgba(59,35,20,.45);touch-action:manipulation;animation:an-notice 3s ease both}
@keyframes an-notice{0%{opacity:0;transform:translateX(-50%) translateY(10px)}10%,85%{opacity:1;transform:translateX(-50%)}100%{opacity:0}}
.an-card{position:relative;box-sizing:border-box;width:min(92vw,640px);max-height:94dvh;overflow-y:auto;padding:18px;border-radius:26px;border:6px solid #3b2314;background:#fff8e7;text-align:center;display:flex;flex-direction:column;gap:12px;align-items:center;font-weight:bold;color:#3b2314;font-size:max(2.6vh,17px)}
.an-card h2{margin:0;font-size:1.2em}
.an-card p{margin:0}
.an-detail{font-size:.85em;font-weight:normal}
.an-choices{display:grid;grid-template-columns:1fr 1fr;gap:12px;width:100%}
.an-btn{touch-action:manipulation;min-height:64px;min-width:64px;border-radius:32px;border:5px solid #3b2314;background:#ffd65c;color:#3b2314;font-size:1.1em;font-weight:bold;padding:6px 14px;box-sizing:border-box;box-shadow:0 5px 0 rgba(59,35,20,.55)}
.an-btn.allow{background:#7cc46a}
.an-x{position:absolute;right:8px;top:8px;width:64px;height:64px;min-height:0;border-radius:50%;padding:0;font-size:28px;background:#fff}
#analytics-hold{--p:0;width:130px;height:130px;border-radius:50%;border:5px solid #3b2314;padding:0;font-size:56px;background:conic-gradient(#e8604c calc(var(--p) * 360deg),#fff 0);touch-action:none;user-select:none;-webkit-touch-callout:none}
#analytics-hold span{display:flex;width:96px;height:96px;margin:auto;border-radius:50%;background:#ffd65c;align-items:center;justify-content:center;pointer-events:none}
@media (max-height:480px){.an-card{padding:10px 14px;gap:6px;font-size:15px}.an-btn{min-height:64px}#analytics-hold{width:100px;height:100px;font-size:40px}#analytics-hold span{width:72px;height:72px}.an-choices{grid-template-columns:repeat(4,1fr);gap:8px}}
`;

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

/** Builds the settings row (`#analytics-toggle`) and the dialog; `relabel` follows language changes. */
export const NOTICE_MS = 3000;

export function createAnalyticsRow(app: AnalyticsUiApp, analytics: Analytics, now: () => number = () => performance.now()): { row: HTMLElement; relabel: () => void; showNotice: () => boolean } {
  if (!document.getElementById('analytics-style')) {
    const st = document.createElement('style');
    st.id = 'analytics-style';
    st.textContent = STYLE;
    document.head.append(st);
  }
  const row = el('div', 'row');
  row.id = 'settings-analytics';
  const btn = el('button', 'choice', '📊');
  btn.id = 'analytics-toggle';
  btn.type = 'button';
  row.append(btn);
  const dialog = el('div');
  dialog.id = 'analytics-dialog';
  dialog.hidden = true;
  document.body.append(dialog);
  let raf = 0;

  const mark = () => {
    btn.classList.toggle('on', analytics.on);
    btn.setAttribute('aria-pressed', String(analytics.on));
    btn.dataset.state = analytics.on ? 'on' : 'off';
  };
  const relabel = () => btn.setAttribute('aria-label', app.t('ui-analytics'));
  const close = () => {
    cancelAnimationFrame(raf);
    raf = 0;
    dialog.hidden = true;
    dialog.replaceChildren();
  };
  const closeBtn = () => {
    const x = el('button', 'an-btn an-x', '✖');
    x.type = 'button';
    x.setAttribute('aria-label', app.t('ui-close'));
    x.addEventListener('click', close);
    return x;
  };

  const consentCard = () => {
    const card = el('div', 'an-card');
    const allow = el('button', 'an-btn allow', app.t('analytics-allow'));
    allow.id = 'analytics-allow';
    const deny = el('button', 'an-btn', app.t('analytics-deny'));
    deny.id = 'analytics-deny';
    allow.type = deny.type = 'button';
    allow.addEventListener('click', () => {
      analytics.grant();
      mark();
      close();
    });
    deny.addEventListener('click', () => {
      analytics.deny();
      mark();
      close();
    });
    const choices = el('div', 'an-choices');
    choices.append(allow, deny);
    card.append(closeBtn(), el('h2', undefined, app.t('analytics-title')), el('p', undefined, app.t('analytics-note')), el('p', 'an-detail', app.t('analytics-detail')), choices);
    dialog.replaceChildren(card);
  };

  const holdCard = (gate: ParentalGate) => {
    const card = el('div', 'an-card');
    const hold = el('button');
    hold.id = 'analytics-hold';
    hold.type = 'button';
    hold.append(el('span', undefined, '✋'));
    const frame = () => {
      const p = gate.progress(now());
      hold.style.setProperty('--p', String(p));
      if (gate.stage === 'open') {
        raf = 0;
        consentCard();
        return;
      }
      raf = requestAnimationFrame(frame);
    };
    const end = () => {
      gate.holdEnd();
      cancelAnimationFrame(raf);
      raf = 0;
      hold.style.setProperty('--p', '0');
    };
    hold.addEventListener('pointerdown', (e) => {
      e.preventDefault();
      try {
        hold.setPointerCapture?.(e.pointerId);
      } catch {
        /* synthetic pointers cannot be captured */
      }
      gate.holdStart(now());
      cancelAnimationFrame(raf);
      raf = requestAnimationFrame(frame);
    });
    for (const t of ['pointerup', 'pointercancel', 'lostpointercapture']) hold.addEventListener(t, end);
    hold.addEventListener('pointerleave', (e) => {
      if (e.pointerType !== 'touch') end();
    });
    hold.addEventListener('contextmenu', (e) => e.preventDefault());
    card.append(closeBtn(), el('h2', undefined, app.t('analytics-title')), el('p', undefined, app.t('ad-gate-hold')), hold);
    dialog.replaceChildren(card);
  };

  const startGate = () => {
    const gate = new ParentalGate(makeGateQuestion());
    const q = gate.question;
    const card = el('div', 'an-card');
    const choices = el('div', 'an-choices');
    for (const n of q.options) {
      const b = el('button', 'an-btn', String(n));
      b.type = 'button';
      b.addEventListener('click', () => {
        if (gate.answer(n) === 'hold') holdCard(gate);
        else close(); // a wrong answer ends the gate: nothing is granted
      });
      choices.append(b);
    }
    card.append(
      closeBtn(),
      el('h2', undefined, app.t('analytics-title')),
      el('p', undefined, app.t('ad-gate-sum')),
      el('p', 'an-question', `${q.a} ${q.op === '-' ? '−' : '+'} ${q.b} = ?`),
      choices,
    );
    dialog.replaceChildren(card);
    dialog.hidden = false;
  };

  btn.addEventListener('click', () => {
    if (analytics.on) {
      analytics.deny(); // switching off needs no gate
      mark();
    } else {
      startGate();
    }
  });
  mark();
  relabel();
  /**
   * The small start notice (PLAT-033): only while no decision was made, once per session, at the
   * bottom centre, 3 s, never blocking (the canvas keeps its input; only the notice itself takes a
   * tap, which opens the parental gate). Returns whether it was shown.
   */
  const showNotice = (): boolean => {
    if (analytics.consent() !== 'unset' || document.getElementById('analytics-notice')) return false;
    try {
      if (window.sessionStorage.getItem('zoo.analytics.notice') === '1') return false;
      window.sessionStorage.setItem('zoo.analytics.notice', '1');
    } catch {
      /* no session storage: show it anyway */
    }
    const n = el('div', undefined, app.t('analytics-notice'));
    n.id = 'analytics-notice';
    n.setAttribute('role', 'status');
    const remove = () => n.remove();
    n.addEventListener('pointerdown', (e) => {
      e.stopPropagation();
      remove();
      btn.click(); // the same parental gate as the settings button
    });
    document.body.append(n);
    window.setTimeout(remove, NOTICE_MS);
    return true;
  };
  return { row, relabel, showNotice };
}
