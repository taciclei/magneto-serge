/// <reference types="cypress" />

/**
 * Custom Cypress commands for Magnéto-Serge E2E tests
 */

declare global {
  namespace Cypress {
    interface Chainable {
      /**
       * Navigate to cassette list page
       * @example cy.visitCassetteList()
       */
      visitCassetteList(): Chainable<void>;

      /**
       * Navigate to cassette detail page
       * @param cassetteName - Name of the cassette
       * @example cy.visitCassetteDetail('my-cassette')
       */
      visitCassetteDetail(cassetteName: string): Chainable<void>;

      /**
       * Wait for API call to complete
       * @param alias - Request alias (without @)
       * @example cy.waitForApi('cassettes')
       */
      waitForApi(alias: string): Chainable<void>;
    }
  }
}

// Navigate to cassette list
Cypress.Commands.add('visitCassetteList', () => {
  cy.visit('/cassettes');
  cy.url().should('include', '/cassettes');
});

// Navigate to cassette detail
Cypress.Commands.add('visitCassetteDetail', (cassetteName: string) => {
  cy.visit(`/cassettes/${cassetteName}`);
  cy.url().should('include', `/cassettes/${cassetteName}`);
});

// Wait for API call
Cypress.Commands.add('waitForApi', (alias: string) => {
  cy.wait(`@${alias}`);
});

export {};
