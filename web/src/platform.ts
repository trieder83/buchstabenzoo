// Which store link an ad opens (GAME-ADS rule 16): Apple devices -> App Store, Android -> Play Store,
// everything else -> the web link. Pure functions; the only browser access is in `currentPlatform`.

export type Platform = 'ios' | 'android' | 'web';

/**
 * Detects the platform. `uaDataPlatform` is `navigator.userAgentData.platform` (preferred when it is a
 * known value); otherwise the user agent string decides. Samsung Internet, Chrome and Firefox on Android
 * all say "Android". iPadOS 13+ in desktop mode and macOS say "Macintosh" (Apple -> `ios`). Windows,
 * Linux, ChromeOS and anything unknown -> `web`.
 */
export function detectPlatform(ua: string, platform = '', maxTouchPoints = 0, uaDataPlatform?: string): Platform {
  const d = (uaDataPlatform ?? '').toLowerCase();
  if (d === 'android') return 'android';
  if (d === 'ios' || d === 'macos') return 'ios';
  if (d === 'windows' || d === 'linux' || d === 'chrome os' || d === 'chromeos' || d === 'chromium os') {
    // a desktop-mode Android browser can report "Linux": the touch screen + ARM platform string tells
    if (d === 'linux' && /^linux (arm|aarch)/i.test(platform) && maxTouchPoints > 1) return 'android';
    return 'web';
  }
  if (/android/i.test(ua)) return 'android';
  if (/iphone|ipad|ipod/i.test(ua)) return 'ios';
  if (/macintosh|mac os x/i.test(ua) || platform === 'MacIntel') return 'ios'; // incl. iPadOS in desktop mode
  if (/^linux (arm|aarch)/i.test(platform) && maxTouchPoints > 1) return 'android';
  return 'web';
}

let override: Platform | null = null;

/** Forces the platform (the native wrappers call this; tests too). `null` removes the override. */
export function setPlatformOverride(p: Platform | null): void {
  override = p;
}

/** `?platform=ios|android|web` (documented, harmless: it only chooses which public store page a link opens). */
export function platformParam(search: string): Platform | null {
  const v = new URLSearchParams(search).get('platform');
  return v === 'ios' || v === 'android' || v === 'web' ? v : null;
}

/** Why `currentPlatform` chose: for the diagnostics. */
export function currentPlatform(): { platform: Platform; source: 'override' | 'param' | 'native' | 'detected' } {
  if (override) return { platform: override, source: 'override' };
  if (typeof window !== 'undefined') {
    const q = platformParam(window.location.search);
    if (q) return { platform: q, source: 'param' };
  }
  const g = (globalThis as { __ZOO_PLATFORM__?: unknown }).__ZOO_PLATFORM__;
  const env = (import.meta as unknown as { env?: Record<string, string | undefined> }).env?.VITE_NATIVE;
  for (const n of [g, env]) if (n === 'ios' || n === 'android') return { platform: n, source: 'native' };
  if (typeof navigator === 'undefined') return { platform: 'web', source: 'detected' };
  const n = navigator as Navigator & { userAgentData?: { platform?: string } };
  return { platform: detectPlatform(n.userAgent ?? '', n.platform ?? '', n.maxTouchPoints ?? 0, n.userAgentData?.platform), source: 'detected' };
}
