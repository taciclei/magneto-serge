/**
 * Service spécifique à l'API Magneto-Serge
 *
 * Construit au-dessus de HydraClientService pour fournir
 * des méthodes métier typées pour l'API Magneto
 */

import { Injectable } from '@angular/core';
import { Observable, map, from } from 'rxjs';
import { IResource } from 'alcaeus';
import { HydraClientService } from './hydra-client.service';
import {
  ProxyStatus,
  CassetteInfo,
  ProxyStats,
  StartProxyRequest,
  StopProxyRequest,
  ApiRoot,
  HydraResponse
} from '../models/hydra.models';

@Injectable({
  providedIn: 'root'
})
export class MagnetoApiService {
  constructor(private hydraClient: HydraClientService) {}

  /**
   * Configure l'URL de base de l'API
   */
  setBaseUrl(url: string): void {
    this.hydraClient.configure({ baseUrl: url });
  }

  /**
   * Configure la clé API
   */
  setApiKey(key: string): void {
    this.hydraClient.setApiKey(key);
  }

  /**
   * Découvre l'API et retourne les informations racine
   */
  discoverApi(): Observable<{
    info: ApiRoot;
    availableActions: string[];
    resource: IResource;
  }> {
    return this.hydraClient.discoverApi().pipe(
      map(({ root, links, operations }) => ({
        info: this.extractApiRoot(root),
        availableActions: links.map(l => l.title || l['hydra:target']),
        resource: root
      }))
    );
  }

  /**
   * Vérifie la santé de l'API
   */
  checkHealth(): Observable<{
    status: string;
    uptime: number;
    resource: IResource;
  }> {
    return this.hydraClient.loadResource('/health').pipe(
      map(resource => ({
        status: this.getProperty(resource, 'status') || 'unknown',
        uptime: this.getProperty(resource, 'uptime_seconds') || 0,
        resource
      }))
    );
  }

  /**
   * Obtient le statut actuel du proxy avec navigation Hydra
   */
  getProxyStatus(): Observable<{
    status: ProxyStatus;
    availableActions: string[];
    resource: IResource;
  }> {
    return this.hydraClient.loadResource('/proxy/status').pipe(
      map(resource => {
        const data = this.getProperty(resource, 'data');
        const links = this.hydraClient.extractHydraLinks(resource);

        return {
          status: data as ProxyStatus,
          availableActions: links.map(l => l.title!).filter(Boolean),
          resource
        };
      })
    );
  }

  /**
   * Démarre le proxy via navigation Hydra
   */
  startProxy(request: StartProxyRequest): Observable<{
    message: string;
    nextActions: string[];
    statusResource: IResource | null;
  }> {
    return this.hydraClient.loadResource('/proxy/start').pipe(
      map(resource => {
        const operations = this.hydraClient.extractOperations(resource);
        const startOp = operations.find(op =>
          op.method.toLowerCase() === 'post'
        );

        if (!startOp) {
          throw new Error('Start operation not found');
        }

        // Exécuter l'opération
        return from(
          this.hydraClient.executeOperation(startOp, request).toPromise()
        ).pipe(
          map(result => {
            const links = this.hydraClient.extractHydraLinks(result!);
            const message = this.getProperty(result!, 'message') || 'Proxy started';

            // Chercher le lien vers le statut
            const statusLink = links.find(l =>
              l.title === 'Check Proxy Status' ||
              l['hydra:target'].includes('/proxy/status')
            );

            return {
              message,
              nextActions: links.map(l => l.title!).filter(Boolean),
              statusLink: statusLink ? statusLink['hydra:target'] : null
            };
          })
        );
      })
    ).pipe(
      // Aplatir l'Observable imbriqué
      map(obs => obs)
    ) as any;
  }

  /**
   * Arrête le proxy
   */
  stopProxy(request: StopProxyRequest = {}): Observable<{
    message: string;
    resource: IResource;
  }> {
    return this.hydraClient.loadResource('/proxy/stop').pipe(
      map(resource => {
        const operations = this.hydraClient.extractOperations(resource);
        const stopOp = operations.find(op =>
          op.method.toLowerCase() === 'post'
        );

        if (!stopOp) {
          throw new Error('Stop operation not found');
        }

        return from(
          this.hydraClient.executeOperation(stopOp, request).toPromise()
        ).pipe(
          map(result => ({
            message: this.getProperty(result!, 'message') || 'Proxy stopped',
            resource: result!
          }))
        );
      })
    ) as any;
  }

  /**
   * Liste toutes les cassettes avec pagination Hydra
   */
  listCassettes(): Observable<{
    cassettes: CassetteInfo[];
    totalItems: number;
    hasNext: boolean;
    hasPrevious: boolean;
    resource: IResource;
  }> {
    return this.hydraClient.loadResource('/cassettes').pipe(
      map(resource => {
        const data = this.getProperty(resource, 'data') || [];
        const cassettes = Array.isArray(data) ? data : [data];

        // Extraire les informations de pagination si disponibles
        const totalItems = this.getProperty(resource, 'hydra:totalItems') || cassettes.length;
        const view = this.getProperty(resource, 'hydra:view');

        return {
          cassettes,
          totalItems,
          hasNext: !!(view && view['hydra:next']),
          hasPrevious: !!(view && view['hydra:previous']),
          resource
        };
      })
    );
  }

  /**
   * Obtient une cassette spécifique
   */
  getCassette(name: string): Observable<{
    cassette: any;
    resource: IResource;
  }> {
    return this.hydraClient.loadResource(`/cassettes/${name}`).pipe(
      map(resource => ({
        cassette: this.getProperty(resource, 'data'),
        resource
      }))
    );
  }

  /**
   * Supprime une cassette via opération Hydra
   */
  deleteCassette(name: string): Observable<{
    message: string;
    resource: IResource;
  }> {
    return this.hydraClient.loadResource(`/cassettes/${name}`).pipe(
      map(resource => {
        const operations = this.hydraClient.extractOperations(resource);
        const deleteOp = operations.find(op =>
          op.method.toLowerCase() === 'delete'
        );

        if (!deleteOp) {
          throw new Error('Delete operation not found');
        }

        return from(
          this.hydraClient.executeOperation(deleteOp).toPromise()
        ).pipe(
          map(result => ({
            message: this.getProperty(result!, 'message') || 'Cassette deleted',
            resource: result!
          }))
        );
      })
    ) as any;
  }

  /**
   * Obtient les statistiques du proxy
   */
  getProxyStats(): Observable<{
    stats: ProxyStats;
    resource: IResource;
  }> {
    return this.hydraClient.loadResource('/proxy/stats').pipe(
      map(resource => ({
        stats: this.getProperty(resource, 'data') as ProxyStats,
        resource
      }))
    );
  }

  /**
   * Obtient la spécification OpenAPI
   */
  getOpenApiSpec(): Observable<any> {
    return this.hydraClient.loadResource('/openapi.json').pipe(
      map(resource => resource)
    );
  }

  /**
   * Navigation Hydra: suit un lien depuis une ressource
   */
  followLink(resource: IResource, linkTitle: string): Observable<IResource | null> {
    return this.hydraClient.followLink(resource, linkTitle);
  }

  /**
   * Helper: extrait une propriété d'une ressource Alcaeus
   */
  private getProperty(resource: IResource, property: string): any {
    if (!resource) return undefined;

    // Essayer d'accéder directement à la propriété
    if (resource[property as keyof IResource]) {
      return resource[property as keyof IResource];
    }

    // Essayer avec les préfixes JSON-LD courants
    const prefixes = ['', 'data.', 'http://www.w3.org/ns/hydra/core#'];

    for (const prefix of prefixes) {
      const key = prefix + property;
      if (resource[key as keyof IResource]) {
        return resource[key as keyof IResource];
      }
    }

    return undefined;
  }

  /**
   * Helper: extrait les informations de l'API racine
   */
  private extractApiRoot(resource: IResource): ApiRoot {
    return {
      '@id': (resource as any)['@id'] || '/',
      title: this.getProperty(resource, 'title') || 'Magneto-Serge API',
      description: this.getProperty(resource, 'description') || '',
      version: this.getProperty(resource, 'version') || '0.1.0',
      documentation: this.getProperty(resource, 'documentation') || '',
      openapi: this.getProperty(resource, 'openapi') || '/openapi.json'
    };
  }
}
