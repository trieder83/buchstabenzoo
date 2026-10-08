// Settings row + welcome dialog of the opt-in analytics (TECH-PLATFORMS "Analytics (opt-in)").
// A new player first sees the WELCOME dialog (PLAT-033): the animals escaped, a small data note,
// then "No, I don't want to play" (light) / "Yes, I agree, let's play" (bold green). Yes grants
// consent, No stores nothing and shows a goodbye card (back = ask again). Without a decision the
// settings 📊 button grants consent with ONE tap (no question, no hold: user request 2026-10-04);
// once consent is given the button is hidden.
import type { Analytics } from './analytics';

export interface AnalyticsUiApp {
  t(key: string): string;
}

const STYLE = `
#analytics-welcome{position:fixed;inset:0;z-index:10;display:flex;align-items:center;justify-content:center;background:rgba(40,25,15,.7);pointer-events:auto;touch-action:none;user-select:none;-webkit-user-select:none}
#analytics-welcome[hidden]{display:none}
.an-story{font-size:1.3em}
.an-small{font-size:.75em;font-weight:normal;opacity:.85}
.an-btn.light{background:#fff8e7;font-weight:normal;box-shadow:none;border-width:3px}
.an-btn.go{font-size:clamp(13px,4.4vw,1.25em)}
.an-card{position:relative;box-sizing:border-box;width:min(92vw,640px);max-height:94vh;max-height:94dvh;overflow-y:auto;padding:clamp(10px,3vw,18px);border-radius:26px;border:6px solid #3b2314;background:#fff8e7;text-align:center;display:flex;flex-direction:column;gap:12px;align-items:center;font-weight:bold;color:#3b2314;font-size:clamp(14px,min(2.6vh,4.6vw),22px)}
.an-card p{margin:0}
.an-choices{display:grid;grid-template-columns:1fr 1fr;gap:12px;width:100%}
.an-btn{touch-action:manipulation;min-height:64px;min-width:64px;border-radius:32px;border:5px solid #3b2314;background:#ffd65c;color:#3b2314;font-size:clamp(12px,3.8vw,1.1em);font-weight:bold;line-height:1.15;padding:6px 12px;box-sizing:border-box;box-shadow:0 5px 0 rgba(59,35,20,.55);min-width:0;max-width:100%;white-space:normal;overflow-wrap:anywhere;hyphens:auto;overflow:hidden}
@media (max-width:340px){.an-choices{grid-template-columns:1fr}}
.an-btn.allow{background:#7cc46a}
`;

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

/** Builds the settings row (`#analytics-toggle`) and the dialog; `relabel` follows language changes. */
export function createAnalyticsRow(app: AnalyticsUiApp, analytics: Analytics, now: () => number = () => performance.now()): { row: HTMLElement; relabel: () => void; showWelcome: () => boolean } {
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

  const mark = () => {
    btn.classList.toggle('on', analytics.on);
    btn.setAttribute('aria-pressed', String(analytics.on));
    btn.dataset.state = analytics.on ? 'on' : 'off';
    row.style.display = analytics.consent() === 'granted' ? 'none' : ''; // consent given once: nothing more to show
  };
  const relabel = () => btn.setAttribute('aria-label', app.t('ui-analytics'));
  btn.addEventListener('click', () => {
    if (analytics.on) analytics.deny();
    else analytics.grant(); // one tap, no question, no hold
    mark();
  });
  mark();
  relabel();
  /**
   * The welcome dialog (PLAT-033), only while no decision was made: story line, small data note,
   * two buttons. Yes = consent + play, No = nothing stored, goodbye card with a way back.
   * Returns whether it was shown.
   */
  const showWelcome = (): boolean => {
    if (analytics.consent() !== 'unset' || document.getElementById('analytics-welcome')) return false;
    const w = el('div');
    w.id = 'analytics-welcome';
    w.setAttribute('role', 'dialog');
    document.body.append(w);
    const ask = () => {
      const card = el('div', 'an-card');
      const yes = el('button', 'an-btn allow go', app.t('welcome-yes'));
      yes.id = 'welcome-yes';
      const no = el('button', 'an-btn light', app.t('welcome-no'));
      no.id = 'welcome-no';
      yes.type = no.type = 'button';
      yes.addEventListener('click', () => {
        analytics.grant();
        mark();
        w.remove();
      });
      no.addEventListener('click', bye);
      const choices = el('div', 'an-choices');
      choices.append(no, yes);
      card.append(el('p', 'an-story', app.t('welcome-story')), el('p', 'an-small', app.t('welcome-data')), choices);
      w.replaceChildren(card);
    };
    const bye = () => {
      const card = el('div', 'an-card');
      const back = el('button', 'an-btn allow', '↩');
      back.id = 'welcome-back';
      back.type = 'button';
      back.setAttribute('aria-label', app.t('welcome-back'));
      back.addEventListener('click', ask);
      card.append(el('p', 'an-story', app.t('welcome-bye')), back);
      w.replaceChildren(card);
    };
    ask();
    return true;
  };
  return { row, relabel, showWelcome };
}
