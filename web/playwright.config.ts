// Playwright e2e config. `npm run test:e2e` builds the release bundle (wasm-pack --release +
// vite build) and serves it with `vite preview` (POC-001: "release build served locally").
import { defineConfig } from '@playwright/test';

const PORT = Number(process.env.E2E_PORT ?? 4173);

export default defineConfig({
  testDir: './tests/e2e',
  // Hard limits so a stuck browser or wait can never hang a run (per test / whole run).
  timeout: 120_000,
  globalTimeout: 5_400_000, // the full suite needs ≈ 65 min with software WebGL
  fullyParallel: false,
  workers: 1,
  reporter: [['list']],
  use: {
    baseURL: `http://localhost:${PORT}/`,
    browserName: 'chromium',
    viewport: { width: 1080, height: 2340 },
    deviceScaleFactor: 1,
    launchOptions: {
      // Software WebGL2 in headless Chromium (no GPU on CI).
      args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist'],
    },
  },
  webServer: {
    command: `npx vite preview --port ${PORT} --strictPort --outDir ${process.env.E2E_DIST ?? 'dist'}`,
    url: `http://localhost:${PORT}/`,
    reuseExistingServer: false,
    timeout: 60_000,
  },
});
