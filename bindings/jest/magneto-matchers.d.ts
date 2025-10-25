/**
 * TypeScript definitions for Magnéto-Serge Jest matchers
 */

declare namespace jest {
  interface Matchers<R> {
    /**
     * Assert that response matches a cassette
     * @param cassetteName - Name of cassette to match against
     */
    toMatchCassette(cassetteName: string): R;

    /**
     * Assert that response status matches cassette
     * @param cassetteName - Name of cassette
     * @param expectedStatus - Expected status code
     */
    toMatchCassetteStatus(cassetteName: string, expectedStatus: number): R;

    /**
     * Assert that response body matches cassette
     * @param cassetteName - Name of cassette
     */
    toMatchCassetteBody(cassetteName: string): R;

    /**
     * Assert that cassette has specific number of interactions
     * @param expectedCount - Expected interaction count
     */
    toHaveInteractionCount(expectedCount: number): R;

    /**
     * Assert that cassette contains cookies
     */
    toHaveCookies(): R;

    /**
     * Assert that cassette has specific cookie
     * @param cookieName - Name of cookie to find
     */
    toHaveCookie(cookieName: string): R;

    /**
     * Assert that cassette version matches
     * @param expectedVersion - Expected version (e.g., "2.0")
     */
    toHaveCassetteVersion(expectedVersion: string): R;
  }
}

export {};
