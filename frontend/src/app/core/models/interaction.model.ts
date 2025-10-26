import { Resource } from 'alcaeus';

/**
 * Modèle TypeScript pour une Interaction
 *
 * Correspond à la ressource Hydra `magneto:Interaction`
 */
export interface InteractionResource extends Resource {
  '@id': string;
  '@type': 'magneto:Interaction';
  kind: 'Http' | 'WebSocket';
  request?: HttpRequestResource;
  response?: HttpResponseResource;
  messages?: WebSocketMessageResource[];
  url?: string;
}

/**
 * Requête HTTP
 */
export interface HttpRequestResource {
  method: string;
  url: string;
  headers: Record<string, string>;
  body?: string | null;
}

/**
 * Réponse HTTP
 */
export interface HttpResponseResource {
  status: number;
  headers: Record<string, string>;
  body?: number[] | null; // Bytes array
}

/**
 * Message WebSocket
 */
export interface WebSocketMessageResource {
  direction: 'Sent' | 'Received';
  timestamp_ms: number;
  msg_type: 'Text' | 'Binary';
  data: string | number[];
}

/**
 * Collection d'interactions
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
