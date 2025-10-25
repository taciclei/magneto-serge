/**
 * Service Angular simplifié pour Magneto-Serge
 *
 * Utilise le backend Node.js (pas d'Alcaeus côté client)
 * Architecture production recommandée
 */

import { Injectable } from '@angular/core';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError, map } from 'rxjs/operators';
import {
  ApiResponse,
  ProxyStatus,
  ProxyStatusResponse,
  StartProxyRequest,
  CassetteInfo,
  CassettesResponse
} from '../models/magneto.models';

@Injectable({
  providedIn: 'root'
})
export class MagnetoService {
  private backendUrl = 'http://localhost:3000';

  constructor(private http: HttpClient) {}

  /**
   * Configure l'URL du backend
   */
  setBackendUrl(url: string): void {
    this.backendUrl = url.replace(/\/$/, '');
  }

  /**
   * Découvre l'API
   */
  discoverApi(): Observable<any> {
    return this.http.get(`${this.backendUrl}/`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Obtient le statut du proxy
   */
  getProxyStatus(): Observable<ProxyStatusResponse> {
    return this.http.get<ProxyStatusResponse>(`${this.backendUrl}/proxy/status`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Démarre le proxy
   */
  startProxy(request: StartProxyRequest): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(`${this.backendUrl}/proxy/start`, request)
      .pipe(catchError(this.handleError));
  }

  /**
   * Arrête le proxy
   */
  stopProxy(): Observable<ApiResponse> {
    return this.http.post<ApiResponse>(`${this.backendUrl}/proxy/stop`, {})
      .pipe(catchError(this.handleError));
  }

  /**
   * Liste les cassettes
   */
  listCassettes(): Observable<CassettesResponse> {
    return this.http.get<CassettesResponse>(`${this.backendUrl}/cassettes`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Obtient une cassette spécifique
   */
  getCassette(name: string): Observable<ApiResponse> {
    return this.http.get<ApiResponse>(`${this.backendUrl}/cassettes/${name}`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Supprime une cassette
   */
  deleteCassette(name: string): Observable<ApiResponse> {
    return this.http.delete<ApiResponse>(`${this.backendUrl}/cassettes/${name}`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Vérifie la santé de l'API
   */
  checkHealth(): Observable<ApiResponse> {
    return this.http.get<ApiResponse>(`${this.backendUrl}/health`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Obtient les statistiques du cache
   */
  getCacheStats(): Observable<any> {
    return this.http.get(`${this.backendUrl}/cache/stats`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Vide le cache du backend
   */
  clearCache(): Observable<ApiResponse> {
    return this.http.delete<ApiResponse>(`${this.backendUrl}/cache`)
      .pipe(catchError(this.handleError));
  }

  /**
   * Gestion des erreurs HTTP
   */
  private handleError(error: HttpErrorResponse): Observable<never> {
    let errorMessage = 'Une erreur est survenue';

    if (error.error instanceof ErrorEvent) {
      // Erreur côté client
      errorMessage = `Erreur: ${error.error.message}`;
    } else {
      // Erreur côté serveur
      errorMessage = `Code: ${error.status}, Message: ${error.message}`;

      if (error.error?.error) {
        errorMessage = error.error.error;
      }
    }

    console.error(errorMessage);
    return throwError(() => new Error(errorMessage));
  }
}
