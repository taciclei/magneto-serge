import { Injectable } from '@angular/core';
import { Observable, from } from 'rxjs';
import { create } from 'alcaeus';
import type { HydraResponse, Resource } from 'alcaeus';
import { environment } from '../../../environments/environment';

/**
 * Service Alcaeus pour interagir avec l'API Hydra
 *
 * Ce service utilise la bibliothèque Alcaeus pour consommer l'API hypermedia
 * et naviguer dans les ressources liées via les liens Hydra.
 */
@Injectable({
  providedIn: 'root'
})
export class AlcaeusService {
  private readonly client = create({});

  constructor() {
    // Configuration du client Alcaeus
    this.client.baseUri = environment.apiUrl;
    this.client.headers = {
      'Accept': 'application/ld+json',
      'Content-Type': 'application/ld+json'
    };
  }

  /**
   * Charge une ressource Hydra depuis une URL
   *
   * @param url URL de la ressource (relative ou absolue)
   * @returns Observable de HydraResponse contenant la ressource
   */
  loadResource<T extends Resource = Resource>(url: string): Observable<HydraResponse<T>> {
    const fullUrl = url.startsWith('http') ? url : `${environment.apiUrl}${url}`;
    return from(this.client.loadResource(fullUrl)) as Observable<HydraResponse<T>>;
  }

  /**
   * Suit un lien Hydra depuis une ressource
   *
   * @param resource Ressource source contenant les liens
   * @param rel Relation du lien à suivre (ex: 'next', 'prev', 'interactions')
   * @returns Observable de HydraResponse ou null si le lien n'existe pas
   */
  followLink<T extends Resource = Resource>(
    resource: Resource,
    rel: string
  ): Observable<HydraResponse<T>> | null {
    const link = this.findLink(resource, rel);
    if (!link) {
      console.warn(`Link relation "${rel}" not found in resource`);
      return null;
    }
    return this.loadResource<T>(link.href);
  }

  /**
   * Trouve un lien dans une ressource par sa relation
   *
   * @param resource Ressource contenant les liens
   * @param rel Relation recherchée
   * @returns Lien trouvé ou undefined
   */
  private findLink(resource: Resource, rel: string) {
    // Alcaeus expose les liens via resource.links
    const links = (resource as any).links || [];
    return links.find((link: any) => link.supportedProperty?.title === rel || link.rel === rel);
  }

  /**
   * Navigue vers la page suivante d'une collection
   *
   * @param collection Collection Hydra avec pagination
   * @returns Observable de la page suivante ou null
   */
  nextPage<T extends Resource = Resource>(collection: Resource): Observable<HydraResponse<T>> | null {
    return this.followLink<T>(collection, 'next');
  }

  /**
   * Navigue vers la page précédente d'une collection
   *
   * @param collection Collection Hydra avec pagination
   * @returns Observable de la page précédente ou null
   */
  previousPage<T extends Resource = Resource>(collection: Resource): Observable<HydraResponse<T>> | null {
    return this.followLink<T>(collection, 'previous');
  }

  /**
   * Navigue vers la première page d'une collection
   *
   * @param collection Collection Hydra avec pagination
   * @returns Observable de la première page ou null
   */
  firstPage<T extends Resource = Resource>(collection: Resource): Observable<HydraResponse<T>> | null {
    return this.followLink<T>(collection, 'first');
  }

  /**
   * Navigue vers la dernière page d'une collection
   *
   * @param collection Collection Hydra avec pagination
   * @returns Observable de la dernière page ou null
   */
  lastPage<T extends Resource = Resource>(collection: Resource): Observable<HydraResponse<T>> | null {
    return this.followLink<T>(collection, 'last');
  }
}
