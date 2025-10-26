/**
 * Browser polyfills for Node.js built-in modules
 * Required by alcaeus -> parse-link-header dependency
 */

// Polyfill for 'querystring' module
(window as any).global = window;
(window as any).process = { env: { DEBUG: undefined } };

import * as querystring from 'querystring-es3';
import * as url from 'url';

// Make modules available globally for CommonJS requires
(window as any).querystring = querystring;
(window as any).url = url;
