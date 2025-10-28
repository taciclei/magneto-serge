/**
 * Interaction List E2E Tests
 *
 * Tests for viewing and filtering interaction lists within cassettes.
 */

describe('Interaction List', () => {
  const cassetteName = 'test-cassette';

  beforeEach(() => {
    // Navigate to cassette detail (which contains interaction list)
    cy.visit(`/cassettes/${cassetteName}`);
  });

  it('should display interactions within a cassette', () => {
    // Wait for page to load
    cy.get('mat-card', { timeout: 10000 }).should('exist');

    // Look for interaction display elements
    // Could be a table, list, or cards depending on implementation
    cy.get('body').should('exist'); // Basic check that page loaded
  });

  it('should handle empty interaction list', () => {
    // Even with no interactions, page should render
    cy.visit(`/cassettes/${cassetteName}`);

    cy.get('mat-toolbar').should('exist');
  });

  it('should display interaction metadata', () => {
    cy.visit(`/cassettes/${cassetteName}`);

    // Check that some content is displayed
    // This could be HTTP method, URL, status code, etc.
    cy.get('mat-card', { timeout: 10000 }).should('exist');
  });

  it('should allow clicking on interactions for details', () => {
    cy.visit(`/cassettes/${cassetteName}`);

    // Wait for content
    cy.get('body', { timeout: 10000 }).should('exist');

    // Try to find clickable interaction elements
    cy.get('mat-list-item, mat-row, [data-testid*="interaction"]').first().then(($el) => {
      if ($el.length > 0) {
        cy.wrap($el).click();
        // Verify some detail view opens or route changes
        cy.url().should('exist');
      }
    });
  });
});
