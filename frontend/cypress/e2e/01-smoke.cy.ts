/**
 * Smoke Test
 *
 * Basic sanity check to ensure the application loads correctly.
 */

describe('Smoke Test', () => {
  it('should load the application', () => {
    cy.visit('/');

    // Verify the page loads without errors
    cy.url().should('include', '/cassettes');

    // Verify Material Design components are loaded
    cy.get('mat-toolbar').should('exist');
  });

  it('should have correct page title', () => {
    cy.visit('/');
    cy.title().should('contain', 'Magnéto-Serge');
  });

  it('should display navigation menu', () => {
    cy.visit('/');

    // Check for navigation elements
    cy.get('mat-toolbar').should('be.visible');
    cy.get('mat-toolbar').should('contain', 'Magnéto-Serge');
  });
});
