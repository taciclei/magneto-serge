/**
 * Cassette Detail E2E Tests
 *
 * Tests for the cassette detail page functionality.
 */

describe('Cassette Detail Page', () => {
  // Use a mock cassette name for testing
  const cassetteName = 'test-cassette';

  it('should display cassette detail page', () => {
    // Navigate to a specific cassette detail page
    cy.visit(`/cassettes/${cassetteName}`);

    // Verify URL contains cassette name
    cy.url().should('include', `/cassettes/${cassetteName}`);

    // Page should render without crashing
    cy.get('mat-toolbar').should('exist');
  });

  it('should display cassette metadata', () => {
    cy.visit(`/cassettes/${cassetteName}`);

    // Wait for content to load
    cy.get('mat-card', { timeout: 10000 }).should('exist');
  });

  it('should display interactions list', () => {
    cy.visit(`/cassettes/${cassetteName}`);

    // Check for interactions table or list
    cy.get('mat-card', { timeout: 10000 }).should('exist');
  });

  it('should have a back button', () => {
    cy.visit(`/cassettes/${cassetteName}`);

    // Look for navigation back to list
    cy.get('button, a').contains(/back|return/i).should('exist');
  });

  it('should handle invalid cassette name gracefully', () => {
    const invalidName = 'non-existent-cassette-12345';

    cy.visit(`/cassettes/${invalidName}`);

    // Page should render without crashing
    // Even if data doesn't exist, the page structure should be there
    cy.get('mat-toolbar').should('exist');
  });

  it('should navigate back to cassette list', () => {
    cy.visit(`/cassettes/${cassetteName}`);

    // Find and click back button
    cy.get('button, a').contains(/back|return/i).first().click();

    // Should navigate to cassettes list
    cy.url().should('match', /\/cassettes\/?$/);
  });
});
