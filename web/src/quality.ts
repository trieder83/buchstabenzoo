// Quality tier of the renderer (PERF-BUDGETS rule 5, PERF-R-005). The tier logic lives in
// Rust (`zoo_core::quality`); the host only picks the start mode: `?quality=auto|high|low1|low`
// (debug override), else `auto` — except in automated browsers (Playwright sets
// `navigator.webdriver`), which pin `high` so e2e tests and the perf / look tools stay
// deterministic under software rendering (they pass `?quality=auto` to test the governor).

const MODES = new Set(['auto', 'high', 'low1', 'low']);

/** The quality mode for `App.set_quality` from the page's query string. */
export function qualityMode(search: string, webdriver: boolean): string {
  const q = new URLSearchParams(search).get('quality');
  if (q && MODES.has(q)) return q;
  return webdriver ? 'high' : 'auto';
}
