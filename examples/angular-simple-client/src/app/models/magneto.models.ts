/**
 * Modèles TypeScript simples pour l'API Magneto (via backend Node.js)
 *
 * Aucune dépendance RDF/Hydra - JSON simple uniquement
 */

/**
 * Réponse API de base
 */
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

/**
 * Statut du proxy
 */
export interface ProxyStatus {
  running: boolean;
  mode: 'auto' | 'record' | 'replay' | 'passthrough';
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
 * Requête pour démarrer le proxy
 */
export interface StartProxyRequest {
  mode: 'auto' | 'record' | 'replay' | 'passthrough';
  cassette_name: string;
  port?: number;
  strict?: boolean;
}

/**
 * Action Hydra découverte (simplifié)
 */
export interface NextAction {
  title: string;
  target: string;
}

/**
 * Réponse avec statut du proxy
 */
export interface ProxyStatusResponse extends ApiResponse<ProxyStatus> {
  nextActions?: NextAction[];
}

/**
 * Réponse avec liste de cassettes
 */
export interface CassettesResponse extends ApiResponse {
  cassettes: CassetteInfo[];
  pagination?: {
    totalItems: number;
  };
}
