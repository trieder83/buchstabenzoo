// Opt-in analytics configuration (TECH-PLATFORMS "Analytics (opt-in)", PLAT-029).
// Paste the GA4 measurement id of the Firebase web data stream (`G-XXXXXXXXXX`) here.
// EMPTY = analytics is disabled entirely: no 📊 button, no script, no request, even with a stored consent.
export const ANALYTICS_MEASUREMENT_ID = '';

/** Build define (vite.config.ts): the fake test id, set only by `VITE_ANALYTICS_TEST_ID` in the e2e test build. */
declare const __ANALYTICS_TEST_ID__: string;

/** The id in use: the real one, else the test-build id (always '' in the release build). */
export const MEASUREMENT_ID: string =
  ANALYTICS_MEASUREMENT_ID || (typeof __ANALYTICS_TEST_ID__ !== 'undefined' ? __ANALYTICS_TEST_ID__ : '');
