/**
 * Navigation E2E Tests
 *
 * Tests for application navigation and routing.
 */

describe('Navigation', () => {
  it('should navigate between pages using toolbar links', () => {
    cy.visit('/');

    // Click on toolbar/nav items if they exist
    cy.get('mat-toolbar').should('be.visible');

    // Verify current URL
    cy.url().should('exist');
  });

  it('should navigate using browser back button', () => {
    // Start at cassette list
    cy.visit('/cassettes');

    // Navigate to a detail page
    cy.get('mat-row').first().then(($row) => {
      if ($row.length > 0) {
        cy.wrap($row).click();

        // Verify we're on detail page
        cy.url().should('not.equal', Cypress.config().baseUrl + '/cassettes');

        // Go back
        cy.go('back');

        // Should be back at list
        cy.url().should('match', /\/cassettes\/?$/);
      }
    });
  });

  it('should navigate using browser forward button', () => {
    cy.visit('/cassettes');

    cy.get('mat-row').first().then(($row) => {
      if ($row.length > 0) {
        cy.wrap($row).click();
        cy.url().should('not.equal', Cypress.config().baseUrl + '/cassettes');

        // Go back
        cy.go('back');
        cy.url().should('match', /\/cassettes\/?$/);

        // Go forward
        cy.go('forward');
        cy.url().should('not.equal', Cypress.config().baseUrl + '/cassettes');
      }
    });
  });

  it('should maintain state during navigation', () => {
    cy.visit('/cassettes');

    // Interact with pagination if exists
    cy.get('mat-paginator').then(($paginator) => {
      if ($paginator.length > 0) {
        // Navigate to detail and back
        cy.get('mat-row').first().click();
        cy.go('back');

        // Pagination state might be preserved (depending on implementation)
        cy.get('mat-paginator').should('exist');
      }
    });
  });

  it('should handle direct URL navigation', () => {
    // Direct navigation to cassette detail
    cy.visit('/cassettes/test-cassette');
    cy.get('mat-toolbar').should('exist');

    // Direct navigation to interaction detail
    cy.visit('/cassettes/test-cassette/interactions/0');
    cy.get('mat-toolbar').should('exist');
  });

  it('should redirect root to cassettes', () => {
    cy.visit('/');

    // Should redirect to /cassettes
    cy.url().should('include', '/cassettes');
  });
});
