/**
 * Modèles TypeScript pour l'API Hydra/JSON-LD de Magneto-Serge
 */

import { IResource } from 'alcaeus';

/**
 * Opération Hydra (action disponible sur une ressource)
 */
export interface HydraOperation {
  '@type': string;
  method: string;
  expects?: string;
  returns?: string;
  title?: string;
}

/**
 * Lien Hydra
 */
export interface HydraLink {
  '@type': string;
  'hydra:target': string;
  title?: string;
  'hydra:operation'?: HydraOperation[];
}

/**
 * Réponse API de base avec support Hydra
 */
export interface HydraResponse<T = any> {
  '@context': string;
  '@type': string;
  success: boolean;
  data?: T;
  error?: string;
  timestamp: string;
  'hydra:link'?: HydraLink[];
}

/**
 * Statut du proxy
 */
export interface ProxyStatus {
  running: boolean;
  mode: string;
  port: number;
  cassette: string | null;
  interactions_count: number;
  uptime_seconds: number;
}

/**
 * Informations sur une cassette
 */
export interface CassetteInfo {
  name: string;
  size_bytes: number;
  interactions: number;
  created_at: string;
  format: string;
}

/**
 * Statistiques du proxy
 */
export interface ProxyStats {
  total_requests: number;
  total_responses: number;
  requests_per_second: number;
  avg_response_time_ms: number;
  cache_hit_rate: number;
  memory_mb: number;
  metrics: Record<string, number>;
}

/**
 * Requête pour démarrer le proxy
 */
export interface StartProxyRequest {
  mode: 'auto' | 'record' | 'replay' | 'passthrough';
  cassette_name: string;
  port?: number;
  strict?: boolean;
}

/**
 * Requête pour arrêter le proxy
 */
export interface StopProxyRequest {
  force?: boolean;
}

/**
 * Informations sur l'API racine
 */
export interface ApiRoot {
  '@id': string;
  title: string;
  description: string;
  version: string;
  documentation: string;
  openapi: string;
}

/**
 * Collection Hydra générique
 */
export interface HydraCollection<T = any> {
  '@context': string;
  '@id': string;
  '@type': 'hydra:Collection';
  'hydra:member': T[];
  'hydra:totalItems'?: number;
  'hydra:view'?: {
    '@id': string;
    '@type': 'hydra:PartialCollectionView';
    'hydra:first'?: string;
    'hydra:last'?: string;
    'hydra:next'?: string;
    'hydra:previous'?: string;
  };
}

/**
 * Ressource Alcaeus enrichie avec métadonnées Hydra
 */
export interface MagnetoResource extends IResource {
  // Propriétés métier
  id?: string;

  // Navigation Hydra
  operations?: HydraOperation[];
  links?: HydraLink[];

  // Méthodes helper
  getOperation(type: string): HydraOperation | undefined;
  hasOperation(method: string): boolean;
  followLink(title: string): Promise<IResource | null>;
}

/**
 * Configuration du client Hydra
 */
export interface HydraClientConfig {
  baseUrl: string;
  apiKey?: string;
  acceptHeader?: string;
  autoFollowLinks?: boolean;
  cacheEnabled?: boolean;
}

/**
 * Événements de navigation Hydra
 */
export enum HydraNavigationEvent {
  RESOURCE_LOADED = 'resource:loaded',
  LINK_FOLLOWED = 'link:followed',
  OPERATION_EXECUTED = 'operation:executed',
  ERROR = 'error'
}

/**
 * Payload d'événement de navigation
 */
export interface NavigationEventPayload {
  type: HydraNavigationEvent;
  resource?: IResource;
  link?: HydraLink;
  operation?: HydraOperation;
  error?: Error;
  timestamp: Date;
}
