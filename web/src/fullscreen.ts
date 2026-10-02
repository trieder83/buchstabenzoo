// Full screen + install hint of the host shell (TECH-PLATFORMS "Installable and full screen",
// PLAT-017/018). Logic only; the environment is injectable like in audio.ts so it is unit-testable.
//  - The settings button toggles `requestFullscreen({ navigationUI: 'hide' })` / `exitFullscreen()`
//    (webkit-prefixed fallbacks for old Android WebViews). Errors are silent.
//  - The orientation is never locked (portrait and landscape are both supported).
//  - The button is hidden without the Fullscreen API (iPhone Safari) or when the app already runs
//    standalone / fullscreen as an installed web app.
//  - The iPhone install hint shows only on iOS Safari that is not standalone (settings menu only).

/** The parts of `document` / `window` the helper uses. */
export interface FullscreenEnv {
  doc: {
    fullscreenEnabled?: boolean;
    webkitFullscreenEnabled?: boolean;
    fullscreenElement?: Element | null;
    webkitFullscreenElement?: Element | null;
    exitFullscreen?: () => Promise<void> | void;
    webkitExitFullscreen?: () => Promise<void> | void;
    documentElement: {
      requestFullscreen?: (o?: { navigationUI?: 'hide' | 'show' | 'auto' }) => Promise<void> | void;
      webkitRequestFullscreen?: () => Promise<void> | void;
    };
  };
  userAgent: string;
  maxTouchPoints: number;
  /** `navigator.standalone` (iOS home-screen app). */
  navigatorStandalone?: boolean;
  /** `matchMedia(query).matches`. */
  matches: (query: string) => boolean;
}

/** The real browser environment. */
export function browserEnv(): FullscreenEnv {
  return {
    doc: document as unknown as FullscreenEnv['doc'],
    userAgent: navigator.userAgent,
    maxTouchPoints: navigator.maxTouchPoints,
    navigatorStandalone: (navigator as { standalone?: boolean }).standalone,
    matches: (q) => typeof matchMedia === 'function' && matchMedia(q).matches,
  };
}

/** Running as an installed web app (standalone or fullscreen display mode). */
export function isInstalled(env: FullscreenEnv): boolean {
  return (
    env.navigatorStandalone === true ||
    env.matches('(display-mode: standalone)') ||
    env.matches('(display-mode: fullscreen)')
  );
}

/** The browser offers the Fullscreen API (standard or webkit-prefixed). */
export function fullscreenSupported(env: FullscreenEnv): boolean {
  const d = env.doc;
  return Boolean(d.fullscreenEnabled && d.documentElement.requestFullscreen) ||
    Boolean(d.webkitFullscreenEnabled && d.documentElement.webkitRequestFullscreen);
}

/** Show the full-screen button: API available and not already installed. */
export function showFullscreenButton(env: FullscreenEnv): boolean {
  return fullscreenSupported(env) && !isInstalled(env);
}

export function isFullscreen(env: FullscreenEnv): boolean {
  return Boolean(env.doc.fullscreenElement ?? env.doc.webkitFullscreenElement);
}

/** iPhone / iPad (iPadOS 13+ reports as a touch Mac). */
export function isIos(env: FullscreenEnv): boolean {
  return /iPhone|iPad|iPod/.test(env.userAgent) || (/Macintosh/.test(env.userAgent) && env.maxTouchPoints > 1);
}

/** The "Add to Home Screen" hint: iOS, not yet a home-screen app. */
export function showIosInstallHint(env: FullscreenEnv): boolean {
  return isIos(env) && !isInstalled(env);
}

/** Enter or leave full screen; never throws, never rejects (PLAT-018). */
export async function toggleFullscreen(env: FullscreenEnv): Promise<void> {
  try {
    const d = env.doc;
    if (isFullscreen(env)) {
      await (d.exitFullscreen ?? d.webkitExitFullscreen)?.call(d);
    } else if (d.documentElement.requestFullscreen) {
      await d.documentElement.requestFullscreen({ navigationUI: 'hide' });
    } else {
      await d.documentElement.webkitRequestFullscreen?.();
    }
  } catch {
    // unsupported or refused: the game just stays as it is
  }
}
