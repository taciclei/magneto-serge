import { Resource } from 'alcaeus';

/**
 * Type d'interaction
 */
export type InteractionKind = 'Http' | 'WebSocket';

/**
 * Ressource HTTP Request
 */
export interface HttpRequestResource {
  method: string;
  url: string;
  headers: { [key: string]: string };
  body?: string;
}

/**
 * Ressource HTTP Response
 */
export interface HttpResponseResource {
  status: number;
  headers: { [key: string]: string };
  body?: string;
  hasTemplates?: boolean;
}

/**
 * Message WebSocket
 */
export interface WebSocketMessageResource {
  direction: 'Sent' | 'Received';
  timestampMs: number; // Aligned with backend camelCase
  msgType: 'Text' | 'Binary'; // Aligned with backend camelCase
  data: string;
}

/**
 * Interaction Resource - union type pour HTTP et WebSocket
 */
export type InteractionResource = HttpInteractionResource | WebSocketInteractionResource;

/**
 * Interaction HTTP
 */
export interface HttpInteractionResource extends Resource {
  '@id': string;
  kind: 'Http';
  request: HttpRequestResource;
  response: HttpResponseResource;
  _links?: {
    self: { href: string };
    cassette: { href: string };
  };
}

/**
 * Interaction WebSocket
 */
export interface WebSocketInteractionResource extends Resource {
  '@id': string;
  kind: 'WebSocket';
  url: string;
  messages: WebSocketMessageResource[];
  _links?: {
    self: { href: string };
    cassette: { href: string };
  };
}

/**
 * Collection Hydra d'interactions avec pagination
 */
export interface InteractionCollection extends Resource {
  '@id': string;
  '@type': 'hydra:Collection';
  'hydra:member': InteractionResource[];
  'hydra:totalItems': number;
  'hydra:view'?: {
    '@id': string;
    '@type': 'hydra:PartialCollectionView';
    'hydra:first': string;
    'hydra:previous'?: string;
    'hydra:next'?: string;
    'hydra:last': string;
  };
}

/**
 * Helper functions
 */
export function isHttpInteraction(interaction: InteractionResource): interaction is HttpInteractionResource {
  return interaction.kind === 'Http';
}

export function isWebSocketInteraction(interaction: InteractionResource): interaction is WebSocketInteractionResource {
  return interaction.kind === 'WebSocket';
}

/**
 * Get HTTP method color for display
 */
export function getMethodColor(method: string): string {
  const colors: { [key: string]: string } = {
    GET: 'primary',
    POST: 'accent',
    PUT: 'warn',
    DELETE: 'warn',
    PATCH: 'accent',
    HEAD: 'primary',
    OPTIONS: 'primary'
  };
  return colors[method.toUpperCase()] || 'primary';
}

/**
 * Get status code color
 */
export function getStatusColor(status: number): string {
  if (status >= 200 && status < 300) return 'success';
  if (status >= 300 && status < 400) return 'info';
  if (status >= 400 && status < 500) return 'warn';
  if (status >= 500) return 'error';
  return 'default';
}
