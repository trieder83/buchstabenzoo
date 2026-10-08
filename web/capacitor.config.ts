// Capacitor wrapper of the same web build for the iOS app (PLAT-034, specs/40-tech/app-store.md) and the Android app (specs/40-tech/play-store.md).
// The game is bundled into the app (webDir) - nothing is fetched from the web at start (Guideline 4.2).
// Build the native variant first: `npm run build:native` (VITE_NATIVE=1, PLAT-036), then `cap sync ios`.
import type { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'ch.rcms.letterzoo',
  appName: 'Letter Zoo',
  webDir: 'dist',
  android: {
    // https://localhost origin: stable, so localStorage (save game, settings) survives app updates - never change it after release (PLAY-004).
    // The WebView plays sound without a tap (the game starts audio after the first touch anyway).
    backgroundColor: '#fff3d6',
  },
  server: {
    androidScheme: 'https',
  },
  ios: {
    // the default capacitor:// origin is kept: a stable origin, so localStorage (save game, settings)
    // survives app updates. Never change the origin after release - it would reset the saves.
    // content under the notch / home bar: the game uses viewport-fit=cover + env(safe-area-inset-*) (PLAT-021)
    contentInset: 'never',
    scrollEnabled: false,
    backgroundColor: '#fff3d6',
  },
};

export default config;
