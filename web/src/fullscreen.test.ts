// TECH-PLATFORMS "Installable and full screen": PLAT-017 / PLAT-018 (fake environment).
import { describe, expect, it, vi } from 'vitest';
import {
  type FullscreenEnv,
  isFullscreen,
  showFullscreenButton,
  showIosInstallHint,
  toggleFullscreen,
} from './fullscreen';

const IPHONE = 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 Safari/604.1';
const ANDROID = 'Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 Chrome/120 Mobile Safari/537.36';

function env(over: Partial<Omit<FullscreenEnv, 'doc'>> & { doc?: Partial<FullscreenEnv['doc']> } = {}): FullscreenEnv {
  const { doc, ...rest } = over;
  return {
    doc: {
      fullscreenEnabled: true,
      fullscreenElement: null,
      exitFullscreen: vi.fn(async () => {}),
      documentElement: { requestFullscreen: vi.fn(async () => {}) },
      ...doc,
    },
    userAgent: ANDROID,
    maxTouchPoints: 5,
    matches: () => false,
    ...rest,
  };
}

describe('fullscreen helper', () => {
  it('PLAT-017 shows the button only with the API and not installed', () => {
    expect(showFullscreenButton(env())).toBe(true);
    // iPhone Safari: no Fullscreen API
    expect(
      showFullscreenButton(env({ userAgent: IPHONE, doc: { fullscreenEnabled: undefined, documentElement: {} } })),
    ).toBe(false);
    // webkit prefix is enough
    expect(
      showFullscreenButton(
        env({ doc: { fullscreenEnabled: undefined, webkitFullscreenEnabled: true, documentElement: { webkitRequestFullscreen: () => {} } } }),
      ),
    ).toBe(true);
    // installed: standalone / fullscreen display mode, iOS navigator.standalone
    expect(showFullscreenButton(env({ matches: (q) => q.includes('standalone') }))).toBe(false);
    expect(showFullscreenButton(env({ matches: (q) => q.includes('display-mode: fullscreen') }))).toBe(false);
    expect(showFullscreenButton(env({ navigatorStandalone: true }))).toBe(false);
  });

  it('PLAT-017 the iOS install hint shows only on iPhone/iPad Safari, not standalone', () => {
    expect(showIosInstallHint(env({ userAgent: IPHONE }))).toBe(true);
    expect(showIosInstallHint(env({ userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)', maxTouchPoints: 5 }))).toBe(true);
    expect(showIosInstallHint(env({ userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X)', maxTouchPoints: 0 }))).toBe(false);
    expect(showIosInstallHint(env())).toBe(false);
    expect(showIosInstallHint(env({ userAgent: IPHONE, navigatorStandalone: true }))).toBe(false);
    expect(showIosInstallHint(env({ userAgent: IPHONE, matches: (q) => q.includes('standalone') }))).toBe(false);
  });

  it('PLAT-018 toggle enters with navigationUI hide, leaves with exitFullscreen', async () => {
    const e = env();
    await toggleFullscreen(e);
    expect(e.doc.documentElement.requestFullscreen).toHaveBeenCalledWith({ navigationUI: 'hide' });
    const inFs = env({ doc: { fullscreenElement: {} as Element } });
    expect(isFullscreen(inFs)).toBe(true);
    await toggleFullscreen(inFs);
    expect(inFs.doc.exitFullscreen).toHaveBeenCalled();
    expect(inFs.doc.documentElement.requestFullscreen).not.toHaveBeenCalled();
  });

  it('PLAT-018 uses the webkit prefix when the standard API is missing', async () => {
    const req = vi.fn();
    await toggleFullscreen(env({ doc: { documentElement: { webkitRequestFullscreen: req } } }));
    expect(req).toHaveBeenCalled();
    const exit = vi.fn();
    await toggleFullscreen(
      env({ doc: { exitFullscreen: undefined, webkitExitFullscreen: exit, fullscreenElement: undefined, webkitFullscreenElement: {} as Element } }),
    );
    expect(exit).toHaveBeenCalled();
  });

  it('PLAT-018 errors (rejection or throw) are silent', async () => {
    const rejecting = env({ doc: { documentElement: { requestFullscreen: vi.fn(() => Promise.reject(new Error('denied'))) } } });
    await expect(toggleFullscreen(rejecting)).resolves.toBeUndefined();
    const throwing = env({
      doc: {
        fullscreenElement: {} as Element,
        exitFullscreen: () => {
          throw new Error('boom');
        },
      },
    });
    await expect(toggleFullscreen(throwing)).resolves.toBeUndefined();
    await expect(toggleFullscreen(env({ doc: { documentElement: {} } }))).resolves.toBeUndefined();
  });

  it('PLAT-018 never locks the orientation', async () => {
    const lock = vi.fn();
    vi.stubGlobal('screen', { orientation: { lock } });
    await toggleFullscreen(env());
    vi.unstubAllGlobals();
    expect(lock).not.toHaveBeenCalled();
  });
});
