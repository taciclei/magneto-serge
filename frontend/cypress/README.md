# Cypress E2E Testing Guide

This directory contains end-to-end (E2E) tests for the Magnéto-Serge Angular frontend using Cypress.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Test Structure](#test-structure)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [Custom Commands](#custom-commands)
- [CI/CD Integration](#cicd-integration)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

---

## Overview

**Framework:** Cypress 15.5.0
**Test Files:** 8 test suites with 40+ tests
**Coverage:**
- ✅ Application loading and structure
- ✅ Cassette list and detail pages
- ✅ Interaction viewing
- ✅ Navigation flows
- ✅ Error handling
- ✅ Responsive design (mobile, tablet, desktop)
- ✅ Complete user workflows

---

## Quick Start

### Prerequisites

- Node.js 18+ and npm 9+
- Magneto API server running on `http://localhost:8889`
- Angular dev server running on `http://localhost:4200`

### Installation

Dependencies are already installed if you ran `npm install` in the frontend directory.

```bash
cd frontend
npm install  # If not already done
```

### Run Tests

**Interactive Mode** (recommended for development):
```bash
npm run e2e
```

This opens the Cypress Test Runner where you can:
- Select and run individual test files
- Watch tests execute in a real browser
- Time-travel through test steps
- Debug failures interactively

**Headless Mode** (for CI/CD):
```bash
npm run e2e:headless
```

**CI Mode** (with Chrome):
```bash
npm run e2e:ci
```

---

## Test Structure

```
cypress/
├── e2e/                      # Test files
│   ├── 01-smoke.cy.ts        # Basic sanity checks
│   ├── 02-cassette-list.cy.ts      # Cassette list functionality
│   ├── 03-cassette-detail.cy.ts    # Cassette detail viewing
│   ├── 04-interaction-list.cy.ts   # Interaction display
│   ├── 05-interaction-detail.cy.ts # HTTP/WebSocket details
│   ├── 06-error-handling.cy.ts     # Error scenarios
│   ├── 07-navigation.cy.ts         # Browser navigation
│   └── 08-user-workflow.cy.ts      # Complete workflows
├── fixtures/                 # Test data
│   └── example.json          # Sample cassette/interaction data
├── support/                  # Setup and helpers
│   ├── e2e.ts                # Global configuration
│   └── commands.ts           # Custom commands
└── README.md                 # This file
```

### Test Files

#### 1. **01-smoke.cy.ts** - Smoke Tests
Basic sanity checks to ensure the application loads.

**Tests:**
- Application loads without errors
- Page title is correct
- Navigation menu is visible

**Example:**
```typescript
it('should load the application', () => {
  cy.visit('/');
  cy.url().should('include', '/cassettes');
  cy.get('mat-toolbar').should('exist');
});
```

#### 2. **02-cassette-list.cy.ts** - Cassette List
Tests for the cassette list page.

**Tests:**
- Display cassette list table
- Handle pagination
- Navigate to cassette detail
- Handle empty state
- Responsive design

#### 3. **03-cassette-detail.cy.ts** - Cassette Detail
Tests for viewing cassette details.

**Tests:**
- Display cassette metadata
- Display interactions list
- Back button navigation
- Invalid cassette handling

#### 4. **04-interaction-list.cy.ts** - Interaction List
Tests for viewing interactions within cassettes.

**Tests:**
- Display interactions
- Handle empty list
- Show metadata (method, URL, status)
- Click for details

#### 5. **05-interaction-detail.cy.ts** - Interaction Detail
Tests for detailed interaction viewing.

**Tests:**
- Display HTTP request/response
- Formatted code display
- cURL command generation
- Copy functionality
- Back navigation
- Invalid ID handling

#### 6. **06-error-handling.cy.ts** - Error Handling
Tests for error scenarios and resilience.

**Tests:**
- 404 errors
- Network failures
- User-friendly error messages
- Invalid routes
- Malformed data handling

#### 7. **07-navigation.cy.ts** - Navigation
Tests for application navigation.

**Tests:**
- Toolbar navigation
- Browser back/forward buttons
- State preservation
- Direct URL navigation
- Root redirect

#### 8. **08-user-workflow.cy.ts** - User Workflows
Complete end-to-end user workflows.

**Tests:**
- Full browsing workflow
- Exploration workflow
- Pagination workflow
- Mobile workflow

---

## Running Tests

### Local Development

1. **Start the API server:**
```bash
# From project root
./target/release/magneto -c ./cassettes serve
```

2. **Start the Angular dev server:**
```bash
# In a separate terminal
cd frontend
npm start
```

3. **Run Cypress tests:**
```bash
# Interactive mode
npm run e2e

# Headless mode
npm run e2e:headless
```

### Running Specific Tests

**Single test file:**
```bash
npx cypress run --spec "cypress/e2e/01-smoke.cy.ts"
```

**Multiple test files:**
```bash
npx cypress run --spec "cypress/e2e/01-smoke.cy.ts,cypress/e2e/02-cassette-list.cy.ts"
```

**Specific browser:**
```bash
npx cypress run --browser chrome
npx cypress run --browser firefox
npx cypress run --browser edge
```

### Debugging Tests

**Open Cypress with Chrome DevTools:**
```bash
npx cypress open
```

Then:
1. Select a test file
2. Click to run it
3. Use the time-travel feature to inspect each step
4. Open browser DevTools for debugging

**Add debug points in tests:**
```typescript
it('should do something', () => {
  cy.visit('/');
  cy.pause();  // Pauses execution
  cy.get('button').click();
});
```

---

## Writing Tests

### Test Structure

```typescript
describe('Feature Name', () => {
  beforeEach(() => {
    // Runs before each test
    cy.visit('/cassettes');
  });

  it('should do something', () => {
    // Test implementation
    cy.get('button').click();
    cy.url().should('include', '/detail');
  });

  afterEach(() => {
    // Cleanup after each test
  });
});
```

### Common Patterns

**Visiting pages:**
```typescript
cy.visit('/cassettes');  // Relative to baseUrl
cy.visitCassetteList();  // Custom command
cy.visitCassetteDetail('my-cassette');  // Custom command
```

**Finding elements:**
```typescript
// By Material component
cy.get('mat-toolbar').should('be.visible');
cy.get('mat-table').should('exist');

// By test ID (recommended)
cy.get('[data-testid="cassette-row"]').click();

// By text content
cy.contains('Magnéto-Serge').should('be.visible');

// By class
cy.get('.cassette-name').should('have.text', 'test-cassette');
```

**Assertions:**
```typescript
// Existence
cy.get('mat-toolbar').should('exist');
cy.get('mat-toolbar').should('not.exist');

// Visibility
cy.get('mat-table').should('be.visible');
cy.get('.spinner').should('not.be.visible');

// Text content
cy.get('h1').should('contain', 'Cassettes');
cy.get('h1').should('have.text', 'Cassettes');

// URLs
cy.url().should('include', '/cassettes');
cy.url().should('eq', 'http://localhost:4200/cassettes');
```

**Waiting for elements:**
```typescript
// Explicit timeout
cy.get('mat-table', { timeout: 10000 }).should('exist');

// Wait for API call
cy.intercept('GET', '/api/cassettes').as('getCassettes');
cy.wait('@getCassettes');
```

**Conditional testing:**
```typescript
cy.get('mat-row').first().then(($row) => {
  if ($row.length > 0) {
    cy.wrap($row).click();
  }
});
```

---

## Custom Commands

Located in `cypress/support/commands.ts`.

### Available Commands

#### `cy.visitCassetteList()`
Navigate to the cassette list page.

**Usage:**
```typescript
cy.visitCassetteList();
```

#### `cy.visitCassetteDetail(cassetteName)`
Navigate to a cassette detail page.

**Parameters:**
- `cassetteName` (string) - Name of the cassette

**Usage:**
```typescript
cy.visitCassetteDetail('my-cassette');
```

#### `cy.waitForApi(alias)`
Wait for an API call to complete.

**Parameters:**
- `alias` (string) - Request alias (without @)

**Usage:**
```typescript
cy.intercept('GET', '/api/cassettes').as('cassettes');
cy.visitCassetteList();
cy.waitForApi('cassettes');
```

### Creating Custom Commands

Add new commands in `cypress/support/commands.ts`:

```typescript
Cypress.Commands.add('login', (username: string, password: string) => {
  cy.visit('/login');
  cy.get('input[name="username"]').type(username);
  cy.get('input[name="password"]').type(password);
  cy.get('button[type="submit"]').click();
});

// Declare TypeScript types
declare global {
  namespace Cypress {
    interface Chainable {
      login(username: string, password: string): Chainable<void>;
    }
  }
}
```

---

## CI/CD Integration

E2E tests run automatically in GitHub Actions on push and pull requests.

### Workflow: `.github/workflows/e2e.yml`

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`
- Changes to `frontend/**` files

**Matrix Strategy:**
- **Browsers:** Chrome, Firefox, Edge
- **Parallel execution:** 2 containers per browser

**Steps:**
1. Build Magneto API server
2. Install frontend dependencies
3. Build Angular frontend
4. Start API server (port 8889)
5. Start Angular dev server (port 4200)
6. Run Cypress tests
7. Upload screenshots/videos on failure

**Artifacts:**
- Cypress screenshots (on failure, 7 days retention)
- Cypress videos (always, 7 days retention)

### Running Locally Like CI

```bash
# Build API server
cargo build --bin magneto --features cli --release

# Start API server
./target/release/magneto -c ./cassettes serve &

# Install and build frontend
cd frontend
npm ci
npm run build

# Start dev server
npm start &

# Run E2E tests
npm run e2e:ci

# Cleanup
pkill -f magneto
pkill -f "ng serve"
```

---

## Best Practices

### 1. **Use Data Attributes for Selectors**

```typescript
// Good - stable and semantic
cy.get('[data-testid="cassette-row"]').click();

// Bad - fragile and implementation-dependent
cy.get('.mat-row.cassette-item-0').click();
```

### 2. **Avoid Hard-Coded Waits**

```typescript
// Bad
cy.wait(5000);

// Good - wait for specific condition
cy.get('mat-table', { timeout: 10000 }).should('be.visible');
```

### 3. **Use beforeEach for Setup**

```typescript
describe('Cassette List', () => {
  beforeEach(() => {
    cy.visit('/cassettes');  // Run before each test
  });

  it('test 1', () => {
    // Already on /cassettes
  });

  it('test 2', () => {
    // Already on /cassettes
  });
});
```

### 4. **Keep Tests Independent**

Each test should be able to run in isolation.

```typescript
// Good - independent tests
it('should display cassette list', () => {
  cy.visit('/cassettes');
  cy.get('mat-table').should('exist');
});

it('should navigate to detail', () => {
  cy.visit('/cassettes');  // Start fresh
  cy.get('mat-row').first().click();
});

// Bad - dependent tests
it('should display cassette list', () => {
  cy.visit('/cassettes');
});

it('should navigate to detail', () => {
  cy.get('mat-row').first().click();  // Assumes previous test ran
});
```

### 5. **Use Fixtures for Test Data**

```typescript
// Load fixture data
cy.fixture('example.json').then((data) => {
  cy.intercept('GET', '/api/cassettes', data.cassettes);
});
```

### 6. **Handle Flaky Tests**

```typescript
// Retry flaky tests
describe('Flaky Feature', { retries: 2 }, () => {
  it('should work consistently', () => {
    // Test implementation
  });
});
```

---

## Troubleshooting

### Tests Fail Locally But Pass in CI

**Problem:** Environment differences
**Solution:**
- Check Node.js and npm versions match CI
- Run `npm ci` instead of `npm install`
- Verify API server is running on the same port

### Timeout Errors

**Problem:** Elements take too long to appear
**Solution:**
```typescript
// Increase timeout for slow operations
cy.get('mat-table', { timeout: 15000 }).should('exist');

// Or configure globally in cypress.config.ts
defaultCommandTimeout: 10000
```

### Element Not Found

**Problem:** Selector doesn't match any elements
**Solution:**
- Open Cypress Test Runner and inspect the DOM
- Use `cy.pause()` to debug
- Verify the element exists in the current state

### Tests Pass Individually But Fail Together

**Problem:** Test isolation issues or state pollution
**Solution:**
- Clear state in `beforeEach` or `afterEach`
- Ensure each test starts fresh
```typescript
beforeEach(() => {
  cy.clearLocalStorage();
  cy.clearCookies();
  cy.visit('/cassettes');
});
```

### Screenshots/Videos Not Generated

**Problem:** Configuration or permissions
**Solution:**
- Check `cypress.config.ts` settings:
```typescript
video: true,
screenshotOnRunFailure: true,
```
- Ensure `cypress/videos` and `cypress/screenshots` folders are writable

---

## Configuration

### Cypress Config (`cypress.config.ts`)

```typescript
{
  e2e: {
    baseUrl: 'http://localhost:4200',  // Angular dev server
    env: {
      apiUrl: 'http://localhost:8889'  // Magneto API server
    },
    viewportWidth: 1280,
    viewportHeight: 720,
    defaultCommandTimeout: 10000,
    pageLoadTimeout: 30000,
    retries: {
      runMode: 2,       // Retry failed tests in CI
      openMode: 0       // No retries in interactive mode
    },
    video: true,
    screenshotOnRunFailure: true
  }
}
```

### Environment Variables

Set in `cypress.env.json` (gitignored):

```json
{
  "apiUrl": "http://localhost:8889",
  "customVar": "value"
}
```

Access in tests:
```typescript
cy.visit(Cypress.env('apiUrl'));
```

---

## Resources

- **Cypress Docs:** https://docs.cypress.io
- **Best Practices:** https://docs.cypress.io/guides/references/best-practices
- **Cypress API:** https://docs.cypress.io/api/table-of-contents
- **Angular Testing:** https://angular.io/guide/testing

---

**Last Updated:** 2025-10-28
**Cypress Version:** 15.5.0
**Test Coverage:** 40+ tests across 8 suites
