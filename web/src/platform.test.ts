// ADS-047: which store link an ad opens per platform.
import { afterEach, describe, expect, it } from 'vitest';
import { currentPlatform, detectPlatform, platformParam, setPlatformOverride } from './platform';

const UA = {
  iphoneSafari: 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1',
  iphoneChrome: 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/126.0.6478.153 Mobile/15E148 Safari/604.1',
  ipadDesktop: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15',
  macSafari: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15',
  macChrome: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36',
  androidChrome: 'Mozilla/5.0 (Linux; Android 14; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Mobile Safari/537.36',
  samsung: 'Mozilla/5.0 (Linux; Android 13; SAMSUNG SM-S918B) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/24.0 Chrome/117.0.0.0 Mobile Safari/537.36',
  firefoxAndroid: 'Mozilla/5.0 (Android 14; Mobile; rv:127.0) Gecko/127.0 Firefox/127.0',
  windows: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36',
  linux: 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36',
  chromeOs: 'Mozilla/5.0 (X11; CrOS x86_64 14541.0.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36',
  androidDesktopMode: 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36',
};

describe('platform detection (ADS-047)', () => {
  it('ADS-047 Apple devices -> ios (iPhone Safari/Chrome, iPad desktop mode, Mac Safari/Chrome)', () => {
    expect(detectPlatform(UA.iphoneSafari, 'iPhone', 5)).toBe('ios');
    expect(detectPlatform(UA.iphoneChrome, 'iPhone', 5)).toBe('ios');
    expect(detectPlatform(UA.ipadDesktop, 'MacIntel', 5)).toBe('ios'); // iPadOS 13+ says Macintosh, touch points > 1
    expect(detectPlatform(UA.macSafari, 'MacIntel', 0)).toBe('ios');
    expect(detectPlatform(UA.macChrome, 'MacIntel', 0)).toBe('ios');
  });

  it('ADS-047 every Android browser -> android (Chrome, Samsung Internet, Firefox)', () => {
    for (const ua of [UA.androidChrome, UA.samsung, UA.firefoxAndroid]) expect(detectPlatform(ua, 'Linux armv81', 5)).toBe('android');
    expect(detectPlatform(UA.androidDesktopMode, 'Linux armv81', 5)).toBe('android'); // "desktop site" with a touch screen
  });

  it('ADS-047 Windows, Linux, ChromeOS, unknown and empty -> web', () => {
    expect(detectPlatform(UA.windows, 'Win32', 0)).toBe('web');
    expect(detectPlatform(UA.linux, 'Linux x86_64', 0)).toBe('web');
    expect(detectPlatform(UA.chromeOs, 'Linux x86_64', 0)).toBe('web');
    expect(detectPlatform('SomeBot/1.0')).toBe('web');
    expect(detectPlatform('')).toBe('web');
  });

  it('ADS-047 userAgentData.platform wins over the user agent string', () => {
    expect(detectPlatform(UA.windows, '', 0, 'Android')).toBe('android');
    expect(detectPlatform(UA.androidChrome, '', 0, 'Windows')).toBe('web');
    expect(detectPlatform(UA.windows, '', 0, 'macOS')).toBe('ios');
    expect(detectPlatform(UA.windows, '', 0, 'iOS')).toBe('ios');
    expect(detectPlatform(UA.iphoneSafari, '', 0, 'Chrome OS')).toBe('web');
    expect(detectPlatform(UA.androidChrome, '', 0, '')).toBe('android'); // empty -> the UA decides
  });

  afterEach(() => setPlatformOverride(null));

  it('ADS-047 ?platform= and the native override beat the detection; invalid values are ignored', () => {
    expect(platformParam('?platform=ios')).toBe('ios');
    expect(platformParam('?seed=1&platform=android')).toBe('android');
    expect(platformParam('?platform=web')).toBe('web');
    expect(platformParam('?platform=windows')).toBeNull();
    expect(platformParam('')).toBeNull();
    setPlatformOverride('ios');
    expect(currentPlatform()).toEqual({ platform: 'ios', source: 'override' });
    setPlatformOverride(null);
    expect(currentPlatform().source).not.toBe('override');
  });
});
