import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { Store } from '@ngrx/store';
import { of } from 'rxjs';
import { map, catchError, switchMap, withLatestFrom } from 'rxjs/operators';

import { AlcaeusService } from '../../../core/services/alcaeus.service';
import { CassetteActions } from './cassette.actions';
import { selectPage, selectLimit, selectSearch, selectFilterType, selectSortBy, selectSortOrder } from './cassette.selectors';
import { CassetteCollection, CassetteResource } from '../../../core/models/cassette.model';

/**
 * Effets NgRx pour les cassettes
 *
 * Gère les side-effects comme les appels HTTP via Alcaeus
 */
@Injectable()
export class CassetteEffects {
  constructor(
    private actions$: Actions,
    private alcaeusService: AlcaeusService,
    private store: Store
  ) {}

  /**
   * Effet: Charger les cassettes
   *
   * Déclenché par: loadCassettes, navigateToPage, navigateToNextPage, etc.
   */
  loadCassettes$ = createEffect(() =>
    this.actions$.pipe(
      ofType(
        CassetteActions.loadCassettes,
        CassetteActions.navigateToPage,
        CassetteActions.navigateToNextPage,
        CassetteActions.navigateToPreviousPage,
        CassetteActions.navigateToFirstPage,
        CassetteActions.navigateToLastPage,
        CassetteActions.updateSearch,
        CassetteActions.updateFilterType,
        CassetteActions.updateSort,
        CassetteActions.clearFilters
      ),
      withLatestFrom(
        this.store.select(selectPage),
        this.store.select(selectLimit),
        this.store.select(selectSearch),
        this.store.select(selectFilterType),
        this.store.select(selectSortBy),
        this.store.select(selectSortOrder)
      ),
      switchMap(([action, page, limit, search, filterType, sortBy, sortOrder]) => {
        // Construire l'URL avec pagination et paramètres de recherche/tri
        const params = new URLSearchParams();
        params.set('page', page.toString());
        params.set('limit', limit.toString());

        if (search) {
          params.set('search', search);
        }
        if (filterType) {
          params.set('filter_type', filterType);
        }
        if (sortBy) {
          params.set('sort_by', sortBy);
        }
        if (sortOrder) {
          params.set('sort_order', sortOrder);
        }

        const url = `/cassettes?${params.toString()}`;

        return this.alcaeusService.loadResource<CassetteCollection>(url).pipe(
          map(response => {
            const collection = response.root as CassetteCollection;
            const cassettes = collection['hydra:member'] || [];
            const totalItems = collection['hydra:totalItems'] || 0;

            return CassetteActions.loadCassettesSuccess({
              cassettes,
              totalItems,
              page,
              limit
            });
          }),
          catchError(error =>
            of(CassetteActions.loadCassettesFailure({
              error: error.message || 'Failed to load cassettes'
            }))
          )
        );
      })
    )
  );

  /**
   * Effet: Charger une cassette individuelle
   */
  loadCassetteDetail$ = createEffect(() =>
    this.actions$.pipe(
      ofType(CassetteActions.loadCassetteDetail),
      switchMap(({ name }) => {
        const url = `/cassettes/${name}`;

        return this.alcaeusService.loadResource<CassetteResource>(url).pipe(
          map(response => {
            const cassette = response.root as CassetteResource;
            return CassetteActions.loadCassetteDetailSuccess({ cassette });
          }),
          catchError(error =>
            of(CassetteActions.loadCassetteDetailFailure({
              error: error.message || `Failed to load cassette: ${name}`
            }))
          )
        );
      })
    )
  );
}
