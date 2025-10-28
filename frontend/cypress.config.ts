import { defineConfig } from 'cypress';

export default defineConfig({
  e2e: {
    baseUrl: 'http://localhost:4200',
    specPattern: 'cypress/e2e/**/*.cy.{js,jsx,ts,tsx}',
    supportFile: 'cypress/support/e2e.ts',
    videosFolder: 'cypress/videos',
    screenshotsFolder: 'cypress/screenshots',
    fixturesFolder: 'cypress/fixtures',

    // Viewport settings
    viewportWidth: 1280,
    viewportHeight: 720,

    // Test settings
    defaultCommandTimeout: 10000,
    pageLoadTimeout: 30000,
    requestTimeout: 10000,
    responseTimeout: 10000,

    // Retry settings
    retries: {
      runMode: 2,
      openMode: 0,
    },

    // Video and screenshot settings
    video: true,
    videoCompression: 32,
    screenshotOnRunFailure: true,

    // Chrome web security (disable for API calls)
    chromeWebSecurity: false,

    setupNodeEvents(on, config) {
      // implement node event listeners here
      on('task', {
        log(message) {
          console.log(message);
          return null;
        },
      });

      return config;
    },

    env: {
      apiUrl: 'http://localhost:8889',
      coverage: false,
    },
  },

  component: {
    devServer: {
      framework: 'angular',
      bundler: 'webpack',
    },
    specPattern: '**/*.cy.ts',
  },
});
