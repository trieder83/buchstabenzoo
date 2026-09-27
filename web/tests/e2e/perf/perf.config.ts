// Playwright config of the performance scenarios (PERF-BUDGETS, specs/50-performance/).
// Not part of `npm run test:e2e` (files are `*.perf.ts`); run through tools/perf/run.sh,
// which builds a snapshot of the working tree and serves its `web/dist` on PERF_PORT.
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from '@playwright/test';

const webDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../..');
const PORT = Number(process.env.PERF_PORT ?? 4190);
const ANGLE = process.env.PERF_ANGLE ?? 'swiftshader';

export default defineConfig({
  testDir: '.',
  testMatch: '**/*.perf.ts',
  timeout: 3_600_000, // one test per viewport (all scenarios; ≈ 2 s per frame under SwiftShader)
  globalTimeout: 10_800_000,
  fullyParallel: false,
  workers: 1,
  reporter: [['list']],
  use: {
    baseURL: `http://localhost:${PORT}/`,
    browserName: 'chromium',
    launchOptions: {
      args:
        ANGLE === 'swiftshader'
          ? ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist']
          : [`--use-angle=${ANGLE}`, '--enable-gpu', '--ignore-gpu-blocklist'],
    },
  },
  webServer: {
    command: `npx vite preview --port ${PORT} --strictPort`,
    cwd: webDir,
    url: `http://localhost:${PORT}/`,
    reuseExistingServer: false,
    timeout: 60_000,
  },
});
