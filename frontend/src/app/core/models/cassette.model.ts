import { Resource } from 'alcaeus';

/**
 * Modèle TypeScript pour une Cassette Magnéto
 *
 * Correspond à la ressource Hydra `magneto:Cassette`
 */
export interface CassetteResource extends Resource {
  '@id': string;
  '@type': 'magneto:Cassette';
  name: string;
  version: string;
  recordedAt: string; // ISO 8601 date
  interactionCount: number;
  sizeBytes: number;
  hasTemplates?: boolean;
  description?: string;

  // Liens Hydra
  interactions?: string; // URL vers la collection d'interactions
}

/**
 * Collection Hydra de cassettes avec pagination
 */
export interface CassetteCollection extends Resource {
  '@id': string;
  '@type': 'hydra:Collection';
  'hydra:member': CassetteResource[];
  'hydra:totalItems': number;
  'hydra:view'?: HydraView;
}

/**
 * Vue de pagination Hydra
 */
export interface HydraView {
  '@id': string;
  '@type': 'hydra:PartialCollectionView';
  'hydra:first': string;
  'hydra:previous'?: string;
  'hydra:next'?: string;
  'hydra:last': string;
}

/**
 * Paramètres de pagination
 */
export interface PaginationParams {
  page: number;
  limit: number;
}
