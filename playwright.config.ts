import { defineConfig, devices } from '@playwright/test';

/**
 * See https://playwright.dev/docs/test-configuration
 */
export default defineConfig({
  testDir: './tests/e2e',
  /* Maximum time one test can run for */
  timeout: 60 * 1000, // 60 seconds
  expect: {
    /**
     * Maximum time expect() should wait for the condition to be met
     */
    timeout: 10000 // 10 seconds
  },
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [
    ['html', { open: 'never' }],
    ['json', { outputFile: 'test-results.json' }]
  ],
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')` */
    baseURL: 'http://localhost:1420',

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'on-first-retry',
    /* Take screenshot on failure */
    screenshot: 'only-on-failure',
  },

  /* Configure projects for major browsers */
  projects: [
    {
      name: 'chromium',
      use: { 
        ...devices['Desktop Chrome'],
        // Add a tag to easily identify this project
        launchOptions: {
          args: ['--user-agent=playwright-chromium']
        }
      },
      // You can add a tag to the project name
      // name: 'chromium:stable',
    },

    {
      name: 'firefox',
      use: { 
        ...devices['Desktop Firefox'],
        launchOptions: {
          args: ['--user-agent=playwright-firefox']
        }
      },
    },

    {
      name: 'webkit',
      use: { 
        ...devices['Desktop Safari'],
        launchOptions: {
          args: ['--user-agent=playwright-webkit']
        }
      },
    },
  ],

  /* Folder for test artifacts such as screenshots, videos, traces, etc. */
  outputDir: 'test-results/',

  /* Run your local dev server before starting the tests */
  webServer: {
    command: 'npm run tauri dev',
    url: 'http://localhost:1420',
    reuseExistingServer: !process.env.CI,
    stdout: 'pipe',
    stderr: 'pipe',
    timeout: 2 * 60 * 1000, // 2 minutes
  },
});
