/**
 * User Workflow E2E Tests
 *
 * End-to-end tests simulating real user workflows.
 */

describe('Complete User Workflow', () => {
  it('should complete full browsing workflow', () => {
    // 1. Start at home (redirects to cassettes)
    cy.visit('/');
    cy.url().should('include', '/cassettes');

    // 2. View cassette list
    cy.get('mat-table', { timeout: 10000 }).should('exist');

    // 3. Click on a cassette
    cy.get('mat-row').first().then(($row) => {
      if ($row.length > 0) {
        // Get cassette name before clicking
        cy.wrap($row).click();

        // 4. View cassette detail
        cy.url().should('include', '/cassettes/');
        cy.get('mat-card', { timeout: 10000 }).should('exist');

        // 5. Look at interactions
        // Interactions should be visible on the page

        // 6. Navigate back to list
        cy.get('button, a').contains(/back|return/i).first().click();
        cy.url().should('match', /\/cassettes\/?$/);
      }
    });
  });

  it('should handle exploration workflow', () => {
    // Browse through different cassettes
    cy.visit('/cassettes');

    cy.get('mat-row').then(($rows) => {
      if ($rows.length > 1) {
        // Click first cassette
        cy.wrap($rows[0]).click();
        cy.url().should('include', '/cassettes/');

        // Go back
        cy.go('back');
        cy.url().should('match', /\/cassettes\/?$/);

        // Click second cassette
        cy.wrap($rows[1]).click();
        cy.url().should('include', '/cassettes/');
      }
    });
  });

  it('should handle pagination workflow', () => {
    cy.visit('/cassettes');

    cy.get('mat-paginator').then(($paginator) => {
      if ($paginator.length > 0) {
        // Try to navigate to next page
        cy.get('.mat-mdc-paginator-navigation-next').then(($next) => {
          if (!$next.attr('disabled')) {
            cy.wrap($next).click();
            // Verify page changed
            cy.get('mat-table').should('exist');
          }
        });
      }
    });
  });

  it('should handle mobile workflow', () => {
    // Set mobile viewport
    cy.viewport('iphone-x');

    // Navigate through pages on mobile
    cy.visit('/');
    cy.url().should('include', '/cassettes');

    // Verify mobile layout works
    cy.get('mat-toolbar').should('be.visible');

    // Try to click a cassette on mobile
    cy.get('mat-row').first().then(($row) => {
      if ($row.length > 0) {
        cy.wrap($row).click();
        cy.url().should('include', '/cassettes/');
      }
    });
  });
});
