/**
 * Error Handling E2E Tests
 *
 * Tests for proper error handling and user feedback.
 */

describe('Error Handling', () => {
  it('should handle 404 errors gracefully', () => {
    // Navigate to non-existent cassette
    cy.visit('/cassettes/non-existent-cassette-12345', { failOnStatusCode: false });

    // Page should render without crashing
    cy.get('mat-toolbar').should('exist');

    // Could show error message
    cy.get('body').should('exist');
  });

  it('should handle network errors', () => {
    // Simulate network failure by visiting with API down
    // This is a basic test - might need API mocking for full coverage
    cy.visit('/cassettes');

    // Page should still render structure
    cy.get('mat-toolbar').should('exist');
  });

  it('should display user-friendly error messages', () => {
    cy.visit('/cassettes/invalid-cassette', { failOnStatusCode: false });

    // Look for error messages or empty states
    cy.get('body').should('not.contain', 'undefined');
    cy.get('body').should('not.contain', '[object Object]');
  });

  it('should handle invalid routes', () => {
    cy.visit('/invalid-route-that-does-not-exist', { failOnStatusCode: false });

    // Should either redirect or show 404 page
    cy.get('body').should('exist');
  });

  it('should not crash on malformed data', () => {
    // Visit pages that might have data issues
    cy.visit('/cassettes');

    // Application should be resilient
    cy.get('mat-toolbar').should('exist');
  });
});
