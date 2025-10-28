/**
 * Cassette List E2E Tests
 *
 * Tests for the cassette list page functionality.
 */

describe('Cassette List Page', () => {
  beforeEach(() => {
    // Visit cassette list page before each test
    cy.visit('/cassettes');
  });

  it('should display the cassette list page', () => {
    // Verify URL
    cy.url().should('include', '/cassettes');

    // Verify page elements are visible
    cy.get('mat-toolbar').should('be.visible');
    cy.get('mat-table').should('exist');
  });

  it('should display cassettes in a table', () => {
    // Wait for data to load
    cy.get('mat-table', { timeout: 10000 }).should('be.visible');

    // Check for table headers
    cy.get('mat-header-row').should('exist');

    // Verify column headers
    cy.get('mat-header-cell').should('have.length.at.least', 1);
  });

  it('should handle pagination', () => {
    // Check if paginator exists
    cy.get('mat-paginator').should('exist');

    // Verify pagination controls are visible
    cy.get('mat-paginator').within(() => {
      cy.get('.mat-mdc-paginator-range-label').should('exist');
    });
  });

  it('should navigate to cassette detail on row click', () => {
    // Wait for table to load
    cy.get('mat-table', { timeout: 10000 }).should('be.visible');

    // Get the first row (if exists)
    cy.get('mat-row').first().then(($row) => {
      if ($row.length > 0) {
        // Click the first row
        cy.wrap($row).click();

        // Verify navigation to detail page
        cy.url().should('include', '/cassettes/');
        cy.url().should('not.equal', Cypress.config().baseUrl + '/cassettes');
      }
    });
  });

  it('should handle empty state gracefully', () => {
    // This test verifies that the page doesn't crash with no data
    cy.visit('/cassettes');

    // Page should still render without errors
    cy.get('mat-toolbar').should('be.visible');
  });

  it('should be responsive', () => {
    // Test mobile viewport
    cy.viewport('iphone-x');
    cy.visit('/cassettes');
    cy.get('mat-toolbar').should('be.visible');

    // Test tablet viewport
    cy.viewport('ipad-2');
    cy.visit('/cassettes');
    cy.get('mat-toolbar').should('be.visible');

    // Test desktop viewport
    cy.viewport(1280, 720);
    cy.visit('/cassettes');
    cy.get('mat-toolbar').should('be.visible');
  });
});
