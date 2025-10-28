/**
 * Interaction Detail E2E Tests
 *
 * Tests for viewing detailed information about individual interactions.
 */

describe('Interaction Detail', () => {
  const cassetteName = 'test-cassette';
  const interactionId = '0'; // First interaction

  it('should display interaction detail page', () => {
    // Navigate to interaction detail
    cy.visit(`/cassettes/${cassetteName}/interactions/${interactionId}`);

    // Verify page loaded
    cy.get('mat-toolbar').should('exist');
  });

  it('should display HTTP request details', () => {
    cy.visit(`/cassettes/${cassetteName}/interactions/${interactionId}`);

    // Look for request information
    // Could include method, URL, headers, body
    cy.get('mat-card', { timeout: 10000 }).should('exist');
  });

  it('should display HTTP response details', () => {
    cy.visit(`/cassettes/${cassetteName}/interactions/${interactionId}`);

    // Look for response information
    // Could include status code, headers, body
    cy.get('mat-card', { timeout: 10000 }).should('exist');
  });

  it('should have formatted code display', () => {
    cy.visit(`/cassettes/${cassetteName}/interactions/${interactionId}`);

    // Check for code formatting (syntax highlighting, pre tags, etc.)
    cy.get('pre, code, [class*="highlight"]').should('exist');
  });

  it('should display cURL command', () => {
    cy.visit(`/cassettes/${cassetteName}/interactions/${interactionId}`);

    // Look for cURL command generation
    cy.contains(/curl/i, { timeout: 10000 }).should('exist');
  });

  it('should have copy functionality for cURL', () => {
    cy.visit(`/cassettes/${cassetteName}/interactions/${interactionId}`);

    // Look for copy button near cURL command
    cy.get('button').contains(/copy/i).should('exist');
  });

  it('should navigate back to cassette detail', () => {
    cy.visit(`/cassettes/${cassetteName}/interactions/${interactionId}`);

    // Find and click back button
    cy.get('button, a').contains(/back|return/i).first().click();

    // Should navigate back to cassette detail
    cy.url().should('include', `/cassettes/${cassetteName}`);
    cy.url().should('not.include', '/interactions/');
  });

  it('should handle invalid interaction ID gracefully', () => {
    const invalidId = '999999';

    cy.visit(`/cassettes/${cassetteName}/interactions/${invalidId}`);

    // Page should still render without crashing
    cy.get('mat-toolbar').should('exist');
  });
});
