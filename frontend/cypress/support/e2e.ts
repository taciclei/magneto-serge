// ***********************************************************
// This example support/e2e.ts is processed and
// loaded automatically before your test files.
//
// This is a great place to put global configuration and
// behavior that modifies Cypress.
//
// You can change the location of this file or turn off
// automatically serving support files with the
// 'supportFile' configuration option.
//
// You can read more here:
// https://on.cypress.io/configuration
// ***********************************************************

// Import commands.js using ES2015 syntax:
import './commands';

// Alternatively you can use CommonJS syntax:
// require('./commands')

// Hide fetch/XHR in command log to reduce noise
const app = window.top;

if (!app.document.head.querySelector('[data-hide-command-log-request]')) {
  const style = app.document.createElement('style');
  style.innerHTML =
    '.command-name-request, .command-name-xhr { display: none }';
  style.setAttribute('data-hide-command-log-request', '');

  app.document.head.appendChild(style);
}

// Preserve cookies between tests
Cypress.Cookies.defaults({
  preserve: (cookie) => {
    return true;
  },
});

// Global error handler
Cypress.on('uncaught:exception', (err, runnable) => {
  // returning false here prevents Cypress from
  // failing the test on uncaught exceptions
  // Customize this based on your needs
  if (err.message.includes('ResizeObserver')) {
    return false;
  }
  if (err.message.includes('Non-Error promise rejection')) {
    return false;
  }
  return true;
});

// Before each test
beforeEach(() => {
  // Clear local storage
  cy.clearLocalStorage();

  // Clear cookies
  cy.clearCookies();

  // Preserve test isolation
  cy.log('Starting test');
});

// After each test
afterEach(() => {
  cy.log('Test completed');
});
