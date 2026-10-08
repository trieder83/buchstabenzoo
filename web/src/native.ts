// Native app build switches (PLAT-036, specs/40-tech/app-store.md). `VITE_NATIVE=1` is set by
// `npm run build:native` for the Capacitor/iOS app only; the web build never has it.
// The App Store build ships WITHOUT third-party analytics and WITHOUT the external ad boards
// (Apple Kids Category Guidelines 1.3 / 5.1.4, Q-390/Q-392): the web build is unchanged.
/// <reference types="vite/client" />

/** True in the native app build. */
export const NATIVE: boolean = import.meta.env.VITE_NATIVE === '1';

/** The GA4 measurement id to use: always '' (= analytics fully off, no button, no request) in the native build. */
export function analyticsId(id: string, native: boolean = NATIVE): string {
  return native ? '' : id;
}

/**
 * Native in-world posters for the developer's OWN iOS apps (Q-392, PLAT-038): the native build bundles its own
 * signed manifest `boards-native/` (links ONLY to apps.apple.com product pages, opened behind the parental gate).
 * `true` since 2026-10-08: the native campaign ids are in `ads.ts` / `tools/ads/sign.py` and
 * `boards-native/index.json` + `.sig` are signed. With `false` the native build would have NO ad keys, i.e.
 * no ad request and local placeholders only.
 */
export const NATIVE_ADS = true;

/** The ad-signature keys to use: none in the native build unless {@link NATIVE_ADS} (= no ad request, placeholders only). */
export function adKeys<T>(keys: T[], native: boolean = NATIVE, nativeAds: boolean = NATIVE_ADS): T[] {
  return native && !nativeAds ? [] : keys;
}
