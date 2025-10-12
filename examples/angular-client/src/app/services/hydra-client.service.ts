/**
 * Service Angular pour l'API Hydra/JSON-LD avec Alcaeus
 *
 * Ce service exploite pleinement les capacités Hydra de l'API Magneto-Serge:
 * - Navigation automatique via les liens Hydra
 * - Découverte des opérations disponibles
 * - Support JSON-LD complet avec Alcaeus
 * - Cache de ressources
 * - Typage fort TypeScript
 */

import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, from, BehaviorSubject, Subject } from 'rxjs';
import { map, catchError, tap, shareReplay } from 'rxjs/operators';
import Alcaeus from 'alcaeus';
import { IHydraResponse, IResource, IOperation } from 'alcaeus';
import {
  HydraClientConfig,
  HydraLink,
  HydraOperation,
  NavigationEventPayload,
  HydraNavigationEvent
} from '../models/hydra.models';

@Injectable({
  providedIn: 'root'
})
export class HydraClientService {
  private config: HydraClientConfig = {
    baseUrl: 'http://localhost:8889',
    acceptHeader: 'application/ld+json',
    autoFollowLinks: true,
    cacheEnabled: true
  };

  private alcaeusClient: typeof Alcaeus;
  private resourceCache = new Map<string, IResource>();
  private navigationEvents$ = new Subject<NavigationEventPayload>();

  // État observable de la configuration
  private configSubject = new BehaviorSubject<HydraClientConfig>(this.config);
  public config$ = this.configSubject.asObservable();

  constructor(private http: HttpClient) {
    this.initializeAlcaeus();
  }

  /**
   * Initialise le client Alcaeus
   */
  private initializeAlcaeus(): void {
    this.alcaeusClient = Alcaeus;

    // Configuration des headers par défaut
    this.alcaeusClient.defaultHeaders = {
      'Accept': this.config.acceptHeader!
    };

    if (this.config.apiKey) {
      this.alcaeusClient.defaultHeaders['Authorization'] = `Bearer ${this.config.apiKey}`;
    }
  }

  /**
   * Configure le client Hydra
   */
  configure(config: Partial<HydraClientConfig>): void {
    this.config = { ...this.config, ...config };
    this.configSubject.next(this.config);
    this.initializeAlcaeus();
  }

  /**
   * Définit la clé API pour l'authentification
   */
  setApiKey(apiKey: string): void {
    this.configure({ apiKey });
  }

  /**
   * Charge une ressource Hydra depuis une URL
   */
  loadResource(url: string): Observable<IResource> {
    const fullUrl = url.startsWith('http') ? url : `${this.config.baseUrl}${url}`;

    // Vérifier le cache
    if (this.config.cacheEnabled && this.resourceCache.has(fullUrl)) {
      return from(Promise.resolve(this.resourceCache.get(fullUrl)!));
    }

    return from(
      this.alcaeusClient.loadResource(fullUrl).then((response: IHydraResponse) => {
        const resource = response.root;

        // Mise en cache
        if (this.config.cacheEnabled && resource) {
          this.resourceCache.set(fullUrl, resource);
        }

        // Émettre l'événement de chargement
        this.emitNavigationEvent({
          type: HydraNavigationEvent.RESOURCE_LOADED,
          resource,
          timestamp: new Date()
        });

        return resource;
      })
    ).pipe(
      catchError(error => {
        this.emitNavigationEvent({
          type: HydraNavigationEvent.ERROR,
          error,
          timestamp: new Date()
        });
        throw error;
      })
    );
  }

  /**
   * Suit un lien Hydra depuis une ressource
   */
  followLink(resource: IResource, linkTitle: string): Observable<IResource | null> {
    // Chercher le lien dans les propriétés de la ressource
    const links = this.extractHydraLinks(resource);
    const link = links.find(l => l.title === linkTitle);

    if (!link) {
      console.warn(`Link "${linkTitle}" not found in resource`);
      return from(Promise.resolve(null));
    }

    this.emitNavigationEvent({
      type: HydraNavigationEvent.LINK_FOLLOWED,
      link,
      timestamp: new Date()
    });

    return this.loadResource(link['hydra:target']);
  }

  /**
   * Extrait les liens Hydra d'une ressource
   */
  extractHydraLinks(resource: IResource): HydraLink[] {
    const links: HydraLink[] = [];

    // Parcourir les propriétés de la ressource
    for (const [predicate, values] of Object.entries(resource)) {
      if (predicate === 'hydra:link' || predicate.endsWith('/link')) {
        const linkArray = Array.isArray(values) ? values : [values];
        links.push(...linkArray.map(v => v as HydraLink));
      }
    }

    return links;
  }

  /**
   * Extrait les opérations Hydra disponibles sur une ressource
   */
  extractOperations(resource: IResource): IOperation[] {
    if (!resource || !resource.operations) {
      return [];
    }

    return resource.operations;
  }

  /**
   * Exécute une opération Hydra
   */
  executeOperation(
    operation: IOperation,
    body?: any
  ): Observable<IResource> {
    const method = operation.method.toUpperCase();
    const target = operation.target?.['@id'] || operation['@id'];

    this.emitNavigationEvent({
      type: HydraNavigationEvent.OPERATION_EXECUTED,
      operation: operation as any,
      timestamp: new Date()
    });

    // Utiliser l'invoke d'Alcaeus pour exécuter l'opération
    return from(
      operation.invoke(body).then(response => {
        if (response.root) {
          return response.root;
        }
        throw new Error('No resource returned from operation');
      })
    );
  }

  /**
   * Découvre l'API racine et ses liens
   */
  discoverApi(): Observable<{
    root: IResource;
    links: HydraLink[];
    operations: IOperation[];
  }> {
    return this.loadResource('/').pipe(
      map(root => ({
        root,
        links: this.extractHydraLinks(root),
        operations: this.extractOperations(root)
      })),
      shareReplay(1)
    );
  }

  /**
   * Vide le cache de ressources
   */
  clearCache(): void {
    this.resourceCache.clear();
  }

  /**
   * Observable des événements de navigation
   */
  get navigationEvents(): Observable<NavigationEventPayload> {
    return this.navigationEvents$.asObservable();
  }

  /**
   * Émet un événement de navigation
   */
  private emitNavigationEvent(payload: NavigationEventPayload): void {
    this.navigationEvents$.next(payload);
  }

  /**
   * Obtient une ressource depuis le cache
   */
  getCachedResource(url: string): IResource | undefined {
    return this.resourceCache.get(url);
  }

  /**
   * Vérifie si une ressource est en cache
   */
  isResourceCached(url: string): boolean {
    return this.resourceCache.has(url);
  }
}
